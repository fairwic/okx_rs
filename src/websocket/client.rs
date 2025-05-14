use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};

use futures::Stream;
use futures::{SinkExt, StreamExt};
use log::{debug, error, info, warn};
use reqwest::Method;
use serde::{Deserialize, Serialize};
use serde_json::json;
use tokio::net::TcpStream;
use tokio::sync::mpsc::{self, Receiver, Sender};
use tokio::task::JoinHandle;
use tokio::time::sleep;
use tokio_tungstenite::tungstenite::Error as WsError;
use tokio_tungstenite::{
    connect_async, tungstenite::protocol::Message, MaybeTlsStream, WebSocketStream,
};
use url::Url;

use crate::config::{Credentials, CONFIG};
use crate::error::Error;
use crate::utils;
use crate::websocket::channel::{Args, ChannelType};
use crate::websocket::models::{
    WebSocketAuth, WebSocketLoginRequest, WebSocketMessage, WebSocketOperation, WebSocketRequest,
    WebSocketSubscription,
};

type WebSocketSender = Sender<serde_json::Value>;
type WebSocketConn = WebSocketStream<MaybeTlsStream<TcpStream>>;
type WsMessage = Message;

/// OKX WebSocket客户端
pub struct OkxWebsocketClient {
    /// WebSocket连接URL
    url: String,
    /// 是否使用私有WS (需要认证)
    is_private: bool,
    /// 认证凭证
    credentials: Option<Credentials>,
    /// 是否使用模拟交易
    is_simulated: String,
    /// 已订阅的频道
    subscriptions: Arc<Mutex<HashMap<String, WebSocketSubscription>>>,
    /// 消息发送通道
    tx: Option<Sender<Message>>,
    /// 数据接收通道
    rx: Option<Receiver<serde_json::Value>>,
    /// 连接任务句柄
    connection_task: Option<JoinHandle<()>>,
    /// 重连任务句柄
    reconnect_task: Option<JoinHandle<()>>,
    /// 最后一次ping时间
    last_ping_time: Arc<Mutex<Instant>>,
}

impl OkxWebsocketClient {
    /// 创建新的公共WebSocket客户端
    pub fn new_public() -> Self {
        Self {
            url: CONFIG.websocket_url.clone(),
            is_private: false,
            credentials: None,
            is_simulated: CONFIG.is_simulated_trading.clone(),
            subscriptions: Arc::new(Mutex::new(HashMap::new())),
            tx: None,
            rx: None,
            connection_task: None,
            reconnect_task: None,
            last_ping_time: Arc::new(Mutex::new(Instant::now())),
        }
    }

    /// 创建新的私有WebSocket客户端
    pub fn new_private(credentials: Credentials) -> Self {
        Self {
            url: CONFIG.private_websocket_url.clone(),
            is_private: true,
            credentials: Some(credentials),
            is_simulated: CONFIG.is_simulated_trading.clone(),
            subscriptions: Arc::new(Mutex::new(HashMap::new())),
            tx: None,
            rx: None,
            connection_task: None,
            reconnect_task: None,
            last_ping_time: Arc::new(Mutex::new(Instant::now())),
        }
    }

    /// 设置是否使用模拟交易
    pub fn set_simulated_trading(&mut self, is_simulated: String) {
        self.is_simulated = is_simulated;
    }

    /// 设置WebSocket URL
    pub fn set_url(&mut self, url: impl Into<String>) {
        self.url = url.into();
    }

    /// 连接到WebSocket服务器
    pub async fn connect(&mut self) -> Result<Receiver<serde_json::Value>, Error> {
        let url_string = self.url.clone();
        let url = Url::parse(&url_string)
            .map_err(|e| Error::WebSocketError(format!("无效的WebSocket URL: {}", e)))?;

        let (ws_stream, _) = connect_async(url)
            .await
            .map_err(|e| Error::WebSocketError(format!("连接WebSocket失败: {}", e)))?;

        info!("已连接到OKX WebSocket服务器");

        let (write, read) = ws_stream.split();
        let (tx_in, rx_in) = mpsc::channel::<WsMessage>(100);
        let (tx_out, rx_out) = mpsc::channel::<serde_json::Value>(100);

        // 消息发送任务
        let tx_forward = tokio::spawn(async move {
            let mut rx_in = rx_in;
            let mut write = write;
            while let Some(msg) = rx_in.recv().await {
                if let Err(e) = write.send(msg).await {
                    error!("发送WebSocket消息错误: {}", e);
                    break;
                }
            }
            debug!("WebSocket发送任务结束");
        });

        // 消息接收+心跳任务
        let rx_task = tokio::spawn(Self::run_ws_with_heartbeat(
            read,
            tx_out.clone(),
            tx_in.clone(),
            Duration::from_secs(15),
        ));

        // 合并任务
        self.connection_task = Some(tokio::spawn(async move {
            let _ = tokio::join!(tx_forward, rx_task);
            debug!("WebSocket连接任务已结束");
        }));

        self.tx = Some(tx_in);
        self.rx = Some(rx_out);

        // 如果是私有连接，进行认证
        if self.is_private {
            if let Some(ref credentials) = self.credentials {
                self.login(credentials).await?;
            } else {
                return Err(Error::AuthenticationError(
                    "私有WebSocket连接需要凭证".to_string(),
                ));
            }
        }

        // 启动重连任务
        self.start_reconnect_task();

        // 重新订阅现有通道
        let subscriptions_clone = self
            .subscriptions
            .lock()
            .map_err(|_| Error::WebSocketError("获取订阅锁失败".to_string()))?
            .clone();

        for subscription in subscriptions_clone.values() {
            self.subscribe_with_subscription(subscription.clone())
                .await?;
        }

        // 直接返回 self.rx.take()，不再转发
        let rx = self
            .rx
            .take()
            .ok_or_else(|| Error::WebSocketError("rx 不存在".to_string()))?;
        Ok(rx)
    }

    /// 关闭连接
    pub async fn close(&mut self) {
        // 发送关闭消息
        if let Some(tx) = &self.tx {
            let _ = tx.send(Message::Close(None)).await;
        }

        // 取消任务
        if let Some(handle) = self.connection_task.take() {
            handle.abort();
        }
        if let Some(handle) = self.reconnect_task.take() {
            handle.abort();
        }

        // 清理资源
        self.tx = None;
        self.rx = None;

        info!("已关闭WebSocket连接");
    }

    /// 订阅通道
    pub async fn subscribe(&self, channel: ChannelType, args: Args) -> Result<(), Error> {
        let channel_name = channel.as_str().to_string();
        let instrument_id = args.inst_id.clone();

        let channel_name = match channel {
            ChannelType::Candle(period) => format!("candle{}", period),
            _ => channel_name,
        };
        let subscription = WebSocketSubscription {
            channel: channel_name.clone(),
            instrument_id,
            args: args.params,
        };
        let key = if let Some(ref inst_id) = subscription.instrument_id {
            format!("{}:{}", subscription.channel, inst_id)
        } else {
            subscription.channel.clone()
        };
        if let Ok(mut subscriptions) = self.subscriptions.lock() {
            subscriptions.insert(key, subscription.clone());
        } else {
            return Err(Error::WebSocketError("获取订阅锁失败".to_string()));
        }
        self.subscribe_with_subscription(subscription).await
    }

    /// 使用订阅对象进行订阅
    async fn subscribe_with_subscription(
        &self,
        subscription: WebSocketSubscription,
    ) -> Result<(), Error> {
        let request = WebSocketRequest {
            op: WebSocketOperation::Subscribe,
            args: vec![subscription],
        };
        self.send_message(&request).await
    }

    /// 取消订阅
    pub async fn unsubscribe(&self, channel: ChannelType, args: Args) -> Result<(), Error> {
        let channel_name = channel.as_str().to_string();
        let subscription = WebSocketSubscription {
            channel: channel_name.clone(),
            instrument_id: args.inst_id.clone(),
            args: args.params,
        };
        let key = if let Some(ref id) = args.inst_id {
            format!("{}:{}", channel_name, id)
        } else {
            channel_name.clone()
        };
        if let Ok(mut subscriptions) = self.subscriptions.lock() {
            subscriptions.remove(&key);
        } else {
            return Err(Error::WebSocketError("获取订阅锁失败".to_string()));
        }
        let request = WebSocketRequest {
            op: WebSocketOperation::Unsubscribe,
            args: vec![subscription],
        };
        self.send_message(&request).await
    }

    /// 封装心跳与消息接收的 select! 逻辑
    async fn run_ws_with_heartbeat(
        mut read: impl Stream<Item = Result<WsMessage, WsError>> + Unpin,
        tx_out: Sender<serde_json::Value>,
        tx_in: Sender<WsMessage>,
        heartbeat_interval: Duration,
    ) {
        let mut last_msg_time = Instant::now();
        let mut waiting_pong = false;
        let mut ping_sent_time: Option<Instant> = None;
        loop {
            tokio::select! {
                msg_result = read.next() => {
                    if let Some(res) = msg_result {
                        if let Err(_) = Self::handle_ws_message(
                            res, &tx_out, &tx_in, &mut last_msg_time, &mut waiting_pong, &mut ping_sent_time
                        ).await {
                            break;
                        }
                    } else {
                        break;
                    }
                }
                _ = sleep(heartbeat_interval) => {
                    if !waiting_pong {
                        if let Err(e) = tx_in.send(WsMessage::Text("ping".to_string())).await {
                            error!("发送Ping消息失败: {}", e);
                            break;
                        }
                        debug!("已发送Ping消息");
                        waiting_pong = true;
                        ping_sent_time = Some(Instant::now());
                    } else {
                        error!("心跳超时，未收到pong，准备重连...");
                        break;
                    }
                }
            }
        }
    }

    /// 处理单条 WebSocket 消息
    async fn handle_ws_message(
        res: Result<WsMessage, WsError>,
        tx_out: &Sender<serde_json::Value>,
        tx_in: &Sender<WsMessage>,
        last_msg_time: &mut Instant,
        waiting_pong: &mut bool,
        ping_sent_time: &mut Option<Instant>,
    ) -> Result<(), ()> {
        match res {
            Ok(msg) => {
                *last_msg_time = Instant::now();
                match &msg {
                    WsMessage::Text(text) => {
                        debug!("收到WebSocket消息: {}", text);
                        match serde_json::from_str::<serde_json::Value>(text) {
                            Ok(json_value) => {
                                if let Err(e) = tx_out.send(json_value).await {
                                    error!("发送接收的消息到通道错误: {}", e);
                                    return Err(());
                                }
                            }
                            Err(e) => {
                                error!("解析WebSocket消息错误: {}", e);
                            }
                        }
                    }
                    WsMessage::Ping(data) => {
                        debug!("收到Ping消息");
                        if let Err(e) = tx_in.send(WsMessage::Pong(data.clone())).await {
                            error!("发送Pong响应错误: {}", e);
                        }
                    }
                    WsMessage::Pong(_) => {
                        debug!("收到Pong响应");
                        *waiting_pong = false;
                        *ping_sent_time = None;
                    }
                    _ => {}
                }
            }
            Err(e) => {
                error!("WebSocket接收错误: {}", e);
                return Err(());
            }
        }
        Ok(())
    }

    /// 登录私有WebSocket
    async fn login(&self, credentials: &Credentials) -> Result<(), Error> {
        let timestamp = utils::generate_timestamp_websocket();
        let signature = utils::generate_signature(
            &credentials.api_secret,
            &timestamp,
            &Method::GET,
            "/users/self/verify",
            "",
        )?;
        let auth = WebSocketAuth {
            api_key: credentials.api_key.clone(),
            sign: signature,
            timestamp,
            passphrase: credentials.passphrase.clone(),
        };
        let login_request = WebSocketLoginRequest {
            op: "login".to_string(),
            args: vec![auth],
        };
        self.send_message(&login_request).await?;
        info!("已发送WebSocket登录请求");
        sleep(Duration::from_millis(500)).await;
        Ok(())
    }

    /// 发送WebSocket消息
    async fn send_message<T: Serialize>(&self, message: &T) -> Result<(), Error> {
        if let Some(tx) = &self.tx {
            let message_str = serde_json::to_string(message).map_err(|e| Error::JsonError(e))?;
            debug!("发送WebSocket消息: {}", message_str);
            tx.send(Message::Text(message_str))
                .await
                .map_err(|e| Error::WebSocketError(format!("发送WebSocket消息失败: {}", e)))?;
            Ok(())
        } else {
            Err(Error::WebSocketError("WebSocket未连接".to_string()))
        }
    }

    /// 启动重连任务
    fn start_reconnect_task(&mut self) {
        if self.reconnect_task.is_some() {
            return;
        }
        let tx = self.tx.clone();
        let last_ping_time = self.last_ping_time.clone();
        let mut client = self.clone();
        self.reconnect_task = Some(tokio::spawn(async move {
            let mut interval = tokio::time::interval(Duration::from_secs(5));
            loop {
                interval.tick().await;
                let should_reconnect = {
                    if let Ok(time) = last_ping_time.lock() {
                        let elapsed = time.elapsed();
                        elapsed > Duration::from_secs(30)
                    } else {
                        false
                    }
                };
                if should_reconnect {
                    warn!("WebSocket连接已超过30秒未活动，尝试重连");
                    if let Some(tx) = &tx {
                        let _ = tx.send(Message::Close(None)).await;
                    }
                    match client.connect().await {
                        Ok(_) => {
                            info!("WebSocket重连成功");
                        }
                        Err(e) => {
                            error!("WebSocket重连失败: {}", e);
                            sleep(Duration::from_secs(5)).await;
                        }
                    }
                }
            }
        }));
    }
}

impl Clone for OkxWebsocketClient {
    fn clone(&self) -> Self {
        Self {
            url: self.url.clone(),
            is_private: self.is_private,
            credentials: self.credentials.clone(),
            is_simulated: self.is_simulated.clone(),
            subscriptions: self.subscriptions.clone(),
            tx: self.tx.clone(),
            rx: None,
            connection_task: None,
            reconnect_task: None,
            last_ping_time: self.last_ping_time.clone(),
        }
    }
}

impl Drop for OkxWebsocketClient {
    fn drop(&mut self) {
        if let Some(handle) = self.connection_task.take() {
            handle.abort();
        }
        if let Some(handle) = self.reconnect_task.take() {
            handle.abort();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::env;
    use tokio::time::sleep;
    #[tokio::test]
    async fn test_subscribe() {
        let args = Args::new().with_inst_id("BTC-USDT".to_string());
        let mut client = OkxWebsocketClient::new_public();
        let mut rx = client.connect().await.unwrap();
        client.subscribe(ChannelType::Tickers, args).await.unwrap();
        tokio::spawn(async move {
            while let Some(msg) = rx.recv().await {
                println!("收到公共频道消息: {:?}", msg);
            }
        });
        sleep(Duration::from_secs(100)).await;
    }
    #[tokio::test]
    async fn test_unsubscribe() {
        dotenv::dotenv().ok();
        let api_key = env::var("OKX_API_KEY").expect("OKX_API_KEY 未设置");
        let api_secret = env::var("OKX_API_SECRET").expect("OKX_API_SECRET 未设置");
        let passphrase = env::var("OKX_PASSPHRASE").expect("OKX_PASSPHRASE 未设置");
        let mut client = OkxWebsocketClient::new_private(Credentials::new(api_key, api_secret, passphrase));
        let mut rx_private = client.connect().await.unwrap();
        let args = Args::new()
            .with_inst_id("BTC-USDT".to_string())
            .with_param("period".to_string(), "1D".to_string());
        client
            .subscribe(ChannelType::Candle("1D".to_string()), args)
            .await
            .unwrap();
        tokio::spawn(async move {
            while let Some(msg) = rx_private.recv().await {
                println!("收到私有频道消息: {:?}", msg);
            }
        });
        sleep(Duration::from_secs(100)).await;
    }
}
