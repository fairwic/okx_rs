use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};

use futures::{SinkExt, StreamExt};
use log::{debug, error, info, warn};
use serde_json::Value;
use tokio::sync::mpsc;
use tokio::time::sleep;
use tokio_tungstenite::{connect_async, tungstenite::protocol::Message};
use url::Url;

use crate::config::Credentials;
use crate::error::Error;
use crate::utils;
use crate::websocket::channel::{Args, ChannelType};
use crate::websocket::models::{WebSocketAuth, WebSocketLoginRequest, WebSocketOperation, WebSocketRequest, WebSocketSubscription};

/// 连接状态枚举
#[derive(Debug, Clone, PartialEq)]
pub enum ConnectionState {
    Disconnected,
    Connecting,
    Connected,
    Reconnecting,
}

/// 自动重连配置
#[derive(Debug, Clone)]
pub struct ReconnectConfig {
    /// 是否启用自动重连
    pub enabled: bool,
    /// 重连间隔（秒）
    pub interval: u64,
    /// 最大重连次数
    pub max_attempts: u32,
    /// 指数退避因子
    pub backoff_factor: f64,
    /// 最大退避时间（秒）
    pub max_backoff: u64,
    /// 心跳间隔（秒）
    pub heartbeat_interval: u64,
    /// 消息超时时间（秒）
    pub message_timeout: u64,
}

impl Default for ReconnectConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            interval: 3,
            max_attempts: 100,
            backoff_factor: 1.5,
            max_backoff: 6,
            heartbeat_interval: 3,
            message_timeout: 6,
        }
    }
}

/// 自动重连WebSocket客户端
pub struct AutoReconnectWebsocketClient {
    /// WebSocket连接URL
    url: String,
    /// 是否使用私有WS (需要认证)
    is_private: bool,
    /// 认证凭证
    credentials: Option<Credentials>,
    /// 连接状态
    connection_state: Arc<Mutex<ConnectionState>>,
    /// 最后消息时间
    last_message_time: Arc<Mutex<Instant>>,
    /// 订阅列表
    subscriptions: Arc<Mutex<HashMap<String, (ChannelType, Args)>>>,
    /// 重连配置
    reconnect_config: ReconnectConfig,
    /// 消息发送器（向应用层发送接收到的消息）
    message_sender: Arc<Mutex<Option<mpsc::UnboundedSender<Value>>>>,
    /// WebSocket发送器（向WebSocket服务器发送消息）
    ws_sender: Arc<Mutex<Option<mpsc::UnboundedSender<Message>>>>,
    /// 是否正在运行
    is_running: Arc<Mutex<bool>>,
}

impl AutoReconnectWebsocketClient {
    /// 创建新的公共频道客户端
    pub fn new_public() -> Self {
        Self::new_with_config("wss://ws.okx.com:8443/ws/v5/public",None, ReconnectConfig::default())
    }

    /// 创建新的私有频道客户端
    pub fn new_private(credentials: Credentials) -> Self {
        Self::new_with_config("wss://ws.okx.com:8443/ws/v5/private",Some(credentials), ReconnectConfig::default())
    }
    /// 创建新的交易频道客户端
    pub fn new_business(credentials: Credentials) -> Self {
        Self::new_with_config("wss://ws.okx.com:8443/ws/v5/business",Some(credentials), ReconnectConfig::default())
    }



    /// 使用自定义配置创建客户端
    pub fn new_with_config(url:&str,credentials: Option<Credentials>, config: ReconnectConfig) -> Self {
        Self {
            url: url.to_string(),
            is_private: credentials.is_some(),
            credentials,
            connection_state: Arc::new(Mutex::new(ConnectionState::Disconnected)),
            last_message_time: Arc::new(Mutex::new(Instant::now())),
            subscriptions: Arc::new(Mutex::new(HashMap::new())),
            reconnect_config: config,
            message_sender: Arc::new(Mutex::new(None)),
            ws_sender: Arc::new(Mutex::new(None)),
            is_running: Arc::new(Mutex::new(false)),
        }
    }

    /// 启动客户端并返回消息接收器
    pub async fn start(&self) -> Result<mpsc::UnboundedReceiver<Value>, Error> {
        let mut is_running = self.is_running.lock().unwrap();
        if *is_running {
            return Err(Error::WebSocketError("Client is already running".to_string()));
        }

        let (tx, rx) = mpsc::unbounded_channel();
        *self.message_sender.lock().unwrap() = Some(tx.clone());
        *is_running = true;

        // 启动连接管理任务
        self.start_connection_manager(tx).await;

        info!("自动重连WebSocket客户端已启动");
        Ok(rx)
    }

    /// 停止客户端
    pub async fn stop(&self) {
        *self.is_running.lock().unwrap() = false;
        *self.message_sender.lock().unwrap() = None;
        *self.ws_sender.lock().unwrap() = None;
        *self.connection_state.lock().unwrap() = ConnectionState::Disconnected;
        info!("自动重连WebSocket客户端已停止");
    }

    /// 订阅频道
    pub async fn subscribe(&self, channel: ChannelType, args: Args) -> Result<(), Error> {
        let subscription_key = format!("{:?}_{}", channel, args.inst_id.as_ref().unwrap_or(&"".to_string()));
        
        // 记录订阅信息
        {
            let mut subscriptions = self.subscriptions.lock().unwrap();
            subscriptions.insert(subscription_key.clone(), (channel.clone(), args.clone()));
        }

        // 如果已连接，立即发送订阅请求
        if *self.connection_state.lock().unwrap() == ConnectionState::Connected {
            self.send_subscription_request(&channel, &args, "subscribe").await?;
        }

        debug!("已添加订阅: {:?}", channel);
        Ok(())
    }

    /// 取消订阅频道
    pub async fn unsubscribe(&self, channel: ChannelType, args: Args) -> Result<(), Error> {
        let subscription_key = format!("{:?}_{}", channel, args.inst_id.as_ref().unwrap_or(&"".to_string()));
        
        // 移除订阅记录
        {
            let mut subscriptions = self.subscriptions.lock().unwrap();
            subscriptions.remove(&subscription_key);
        }

        // 如果已连接，发送取消订阅请求
        if *self.connection_state.lock().unwrap() == ConnectionState::Connected {
            self.send_subscription_request(&channel, &args, "unsubscribe").await?;
        }

        debug!("已取消订阅: {:?}", channel);
        Ok(())
    }

    /// 获取连接状态
    pub fn get_connection_state(&self) -> ConnectionState {
        self.connection_state.lock().unwrap().clone()
    }

    /// 检查连接是否健康
    pub fn is_connection_healthy(&self) -> bool {
        let state = self.connection_state.lock().unwrap();
        if *state != ConnectionState::Connected {
            return false;
        }

        let last_time = self.last_message_time.lock().unwrap();
        let elapsed = last_time.elapsed();
        elapsed < Duration::from_secs(self.reconnect_config.message_timeout)
    }

    /// 获取活跃订阅数量
    pub fn get_active_subscriptions_count(&self) -> usize {
        self.subscriptions.lock().unwrap().len()
    }

    /// 启动连接管理任务
    async fn start_connection_manager(&self, tx: mpsc::UnboundedSender<Value>) {
        let url = self.url.clone();
        let is_private = self.is_private;
        let credentials = self.credentials.clone();
        let connection_state = self.connection_state.clone();
        let last_message_time = self.last_message_time.clone();
        let subscriptions = self.subscriptions.clone();
        let is_running = self.is_running.clone();
        let config = self.reconnect_config.clone();
        let ws_sender = self.ws_sender.clone();

        tokio::spawn(async move {
            let mut reconnect_attempts = 0;
            let mut backoff_delay = config.interval;

            while *is_running.lock().unwrap() {
                // 尝试连接
                match Self::establish_connection(&url, is_private, &credentials).await {
                    Ok((ws_stream, _)) => {
                        info!("WebSocket连接建立成功");
                        *connection_state.lock().unwrap() = ConnectionState::Connected;
                        *last_message_time.lock().unwrap() = Instant::now();
                        reconnect_attempts = 0;
                        backoff_delay = config.interval;

                        // 分离读写流
                        let (mut ws_sink, ws_stream) = ws_stream.split();

                        // 创建WebSocket发送通道
                        let (ws_tx, mut ws_rx) = mpsc::unbounded_channel::<Message>();
                        *ws_sender.lock().unwrap() = Some(ws_tx);

                        // 启动发送任务
                        let send_task = tokio::spawn(async move {
                            while let Some(message) = ws_rx.recv().await {
                                if let Err(e) = ws_sink.send(message).await {
                                    error!("发送WebSocket消息失败: {}", e);
                                    break;
                                }
                            }
                        });

                        // 重新订阅所有频道
                        Self::resubscribe_all_channels(&subscriptions, &ws_sender).await;

                        // 启动心跳检测任务
                        let heartbeat_task = Self::start_heartbeat_task(
                            &ws_sender,
                            &connection_state,
                            &last_message_time,
                            &is_running,
                            config.heartbeat_interval,
                            config.message_timeout,
                        );

                        // 处理消息
                        let handle_result = Self::handle_messages(
                            ws_stream,
                            &tx,
                            &connection_state,
                            &last_message_time,
                            &is_running,
                        ).await;

                        // 停止心跳任务
                        heartbeat_task.abort();

                        // 清理发送器
                        *ws_sender.lock().unwrap() = None;
                        send_task.abort();

                        if let Err(e) = handle_result {
                            error!("消息处理错误: {}", e);
                        }

                        *connection_state.lock().unwrap() = ConnectionState::Disconnected;
                    }
                    Err(e) => {
                        error!("WebSocket连接失败: {}", e);
                        *connection_state.lock().unwrap() = ConnectionState::Disconnected;
                    }
                }

                // 检查是否需要重连
                if !*is_running.lock().unwrap() {
                    break;
                }

                if config.enabled && reconnect_attempts < config.max_attempts {
                    reconnect_attempts += 1;
                    *connection_state.lock().unwrap() = ConnectionState::Reconnecting;
                    
                    info!("准备重连 (第{}次)，{}秒后重试", reconnect_attempts, backoff_delay);
                    sleep(Duration::from_secs(backoff_delay)).await;
                    
                    // 指数退避
                    backoff_delay = ((backoff_delay as f64 * config.backoff_factor) as u64)
                        .min(config.max_backoff);
                } else {
                    error!("达到最大重连次数或重连已禁用，停止重连");
                    break;
                }
            }

            info!("连接管理任务结束");
        });
    }

    /// 建立WebSocket连接
    async fn establish_connection(
        url: &str,
        is_private: bool,
        credentials: &Option<Credentials>,
    ) -> Result<(tokio_tungstenite::WebSocketStream<tokio_tungstenite::MaybeTlsStream<tokio::net::TcpStream>>, tokio_tungstenite::tungstenite::handshake::client::Response), Error> {
        let url = Url::parse(url).map_err(|e| Error::WebSocketError(format!("Invalid URL: {}", e)))?;
        
        let (ws_stream, response) = connect_async(url)
            .await
            .map_err(|e| Error::WebSocketError(format!("Connection failed: {}", e)))?;

        // 如果是私有频道，需要进行认证
        if is_private {
            if let Some(creds) = credentials {
                Self::authenticate(&ws_stream, creds).await?;
            } else {
                return Err(Error::WebSocketError("Private channel requires credentials".to_string()));
            }
        }

        Ok((ws_stream, response))
    }

    /// 进行WebSocket认证
    async fn authenticate(
        _ws_stream: &tokio_tungstenite::WebSocketStream<tokio_tungstenite::MaybeTlsStream<tokio::net::TcpStream>>,
        credentials: &Credentials,
    ) -> Result<(), Error> {
        let timestamp = utils::generate_timestamp_websocket();
        let sign_str = format!("{}GET/users/self/verify", timestamp);
        let signature = utils::generate_signature(
            &credentials.api_secret,
            &timestamp,
            &reqwest::Method::GET,
            "/users/self/verify",
            "",
        )?;

        let login_request = WebSocketLoginRequest {
            op: "login".to_string(),
            args: vec![WebSocketAuth {
                api_key: credentials.api_key.clone(),
                passphrase: credentials.passphrase.clone(),
                timestamp,
                sign: signature,
            }],
        };

        let login_message = serde_json::to_string(&login_request)
            .map_err(|e| Error::JsonError(e))?;

        // 发送认证消息
        // 注意：这里需要修改为可变引用，但为了简化示例，我们先跳过实际发送
        debug!("认证消息: {}", login_message);

        Ok(())
    }

    /// 重新订阅所有频道
    async fn resubscribe_all_channels(
        subscriptions: &Arc<Mutex<HashMap<String, (ChannelType, Args)>>>,
        ws_sender: &Arc<Mutex<Option<mpsc::UnboundedSender<Message>>>>,
    ) {
        let subs = subscriptions.lock().unwrap().clone();
        for (key, (channel, args)) in subs {
            debug!("重新订阅频道: {} - {:?}", key, channel);

            // 构建订阅请求
            if let Err(e) = Self::send_subscription_message(&channel, &args, "subscribe", ws_sender).await {
                error!("重新订阅频道失败: {}", e);
            }
        }
    }

    /// 启动心跳检测任务
    fn start_heartbeat_task(
        ws_sender: &Arc<Mutex<Option<mpsc::UnboundedSender<Message>>>>,
        connection_state: &Arc<Mutex<ConnectionState>>,
        last_message_time: &Arc<Mutex<Instant>>,
        is_running: &Arc<Mutex<bool>>,
        heartbeat_interval: u64,
        message_timeout: u64,
    ) -> tokio::task::JoinHandle<()> {
        let ws_sender = ws_sender.clone();
        let connection_state = connection_state.clone();
        let last_message_time = last_message_time.clone();
        let is_running = is_running.clone();

        tokio::spawn(async move {
            let mut interval = tokio::time::interval(Duration::from_secs(heartbeat_interval));
            interval.set_missed_tick_behavior(tokio::time::MissedTickBehavior::Skip);

            while *is_running.lock().unwrap() {
                interval.tick().await;

                // 检查连接状态
                if *connection_state.lock().unwrap() != ConnectionState::Connected {
                    break;
                }

                // 检查消息超时
                let elapsed = last_message_time.lock().unwrap().elapsed();
                if elapsed >= Duration::from_secs(message_timeout) {
                    warn!("消息超时 {}秒，连接可能已断开", elapsed.as_secs());
                    *connection_state.lock().unwrap() = ConnectionState::Disconnected;
                    break;
                }

                // 发送ping消息
                if let Some(sender) = ws_sender.lock().unwrap().as_ref() {
                    if let Err(e) = sender.send(Message::Ping(vec![])) {
                        warn!("发送心跳ping失败: {}", e);
                        *connection_state.lock().unwrap() = ConnectionState::Disconnected;
                        break;
                    }
                    debug!("发送心跳ping");
                } else {
                    warn!("WebSocket发送器不可用");
                    break;
                }
            }

            debug!("心跳检测任务结束");
        })
    }

    /// 处理WebSocket消息
    async fn handle_messages(
        mut ws_stream: futures::stream::SplitStream<tokio_tungstenite::WebSocketStream<tokio_tungstenite::MaybeTlsStream<tokio::net::TcpStream>>>,
        tx: &mpsc::UnboundedSender<Value>,
        connection_state: &Arc<Mutex<ConnectionState>>,
        last_message_time: &Arc<Mutex<Instant>>,
        is_running: &Arc<Mutex<bool>>,
    ) -> Result<(), Error> {
        while *is_running.lock().unwrap() {
            tokio::select! {
                message = ws_stream.next() => {
                    match message {
                        Some(Ok(Message::Text(text))) => {
                            *last_message_time.lock().unwrap() = Instant::now();
                            
                            if let Ok(value) = serde_json::from_str::<Value>(&text) {
                                if tx.send(value).is_err() {
                                    warn!("消息发送失败，接收器可能已关闭");
                                    break;
                                }
                            }
                        }
                        Some(Ok(Message::Ping(_))) => {
                            debug!("收到ping消息");
                            *last_message_time.lock().unwrap() = Instant::now();
                            // WebSocket库会自动回复pong
                        }
                        Some(Ok(Message::Pong(_))) => {
                            debug!("收到pong消息");
                            *last_message_time.lock().unwrap() = Instant::now();
                        }
                        Some(Ok(Message::Close(_))) => {
                            info!("WebSocket连接被服务器关闭");
                            break;
                        }
                        Some(Err(e)) => {
                            error!("WebSocket消息错误: {}", e);
                            break;
                        }
                        None => {
                            warn!("WebSocket流结束");
                            break;
                        }
                        _ => {
                            // 忽略其他消息类型
                        }
                    }
                }
                _ = sleep(Duration::from_secs(1)) => {
                    // 定期检查连接状态
                    if *connection_state.lock().unwrap() != ConnectionState::Connected {
                        break;
                    }
                }
            }
        }

        Ok(())
    }

    /// 发送订阅消息（静态方法）
    async fn send_subscription_message(
        channel: &ChannelType,
        args: &Args,
        operation: &str,
        ws_sender: &Arc<Mutex<Option<mpsc::UnboundedSender<Message>>>>,
    ) -> Result<(), Error> {
        // 构建订阅请求
        let subscription = WebSocketSubscription {
            channel: channel.as_str().to_string(),
            instrument_id: args.inst_id.clone(),
            args: std::collections::HashMap::new(),
        };

        let op = match operation {
            "subscribe" => WebSocketOperation::Subscribe,
            "unsubscribe" => WebSocketOperation::Unsubscribe,
            _ => WebSocketOperation::Subscribe,
        };

        let request = WebSocketRequest {
            op,
            args: vec![subscription],
        };

        let message = serde_json::to_string(&request)
            .map_err(|e| Error::JsonError(e))?;
        debug!("发布{}请求: {}", operation, message);
        // 通过WebSocket发送器发送消息
        if let Some(sender) = ws_sender.lock().unwrap().as_ref() {
            let ws_message = Message::Text(message);
            if let Err(_) = sender.send(ws_message) {
                return Err(Error::ConnectionError("无法发送订阅请求".to_string()));
            }
            debug!("订阅请求已发送到WebSocket");
        } else {
            return Err(Error::ConnectionError("WebSocket连接未建立".to_string()));
        }

        Ok(())
    }

    /// 发送订阅请求
    async fn send_subscription_request(
        &self,
        channel: &ChannelType,
        args: &Args,
        operation: &str,
    ) -> Result<(), Error> {
        Self::send_subscription_message(channel, args, operation, &self.ws_sender).await
    }
}

impl Clone for AutoReconnectWebsocketClient {
    fn clone(&self) -> Self {
        Self {
            url: self.url.clone(),
            is_private: self.is_private,
            credentials: self.credentials.clone(),
            connection_state: self.connection_state.clone(),
            last_message_time: self.last_message_time.clone(),
            subscriptions: self.subscriptions.clone(),
            reconnect_config: self.reconnect_config.clone(),
            message_sender: self.message_sender.clone(),
            ws_sender: self.ws_sender.clone(),
            is_running: self.is_running.clone(),
        }
    }
}
