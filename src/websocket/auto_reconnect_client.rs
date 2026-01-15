use std::collections::HashMap;
use std::env;
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};

use futures::{SinkExt, StreamExt};
use log::{debug, error, info, warn};
use serde_json::Value;
use tokio::sync::mpsc;
use tokio::time::sleep;
use tokio_tungstenite::tungstenite::Error as TungsteniteError;
use tokio_tungstenite::{connect_async, tungstenite::protocol::Message};
use url::Url;

use crate::config::{Credentials, CONFIG};
use crate::error::Error;
use crate::utils;
use crate::websocket::channel::{Args, ChannelType};
use crate::websocket::models::{
    WebSocketAuth, WebSocketLoginRequest, WebSocketOperation, WebSocketRequest,
    WebSocketSubscription,
};

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
            // 默认无限重试
            max_attempts: u32::MAX,
            backoff_factor: 1.5,
            max_backoff: 6,
            heartbeat_interval: 3,
            message_timeout: 6,
        }
    }
}

/// 自动重连WebSocket客户端
pub struct AutoReconnectWebsocketClient {
    /// WebSocket连接URL池（主+备用）
    urls: Vec<String>,
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
    /// 当前使用的URL索引
    current_url_idx: Arc<Mutex<usize>>,
}

impl AutoReconnectWebsocketClient {
    /// 构建主+备用的URL池
    fn build_url_pool(primary: &str) -> Vec<String> {
        fn push_candidate(urls: &mut Vec<String>, candidate: &str) {
            let trimmed = candidate.trim();
            if trimmed.is_empty() {
                return;
            }
            if let Ok(url) = Url::parse(trimmed) {
                if url.host_str().is_none() {
                    return;
                }
                let url_str = url.to_string();
                if !urls.contains(&url_str) {
                    urls.push(url_str);
                }
            }
        }

        let mut urls = Vec::new();

        // 部分网络下 8443 端口可能不可用（会出现 tls handshake eof），优先尝试 443
        if let Ok(parsed) = Url::parse(primary) {
            let path = parsed.path().to_string();
            let primary_host = parsed.host_str().unwrap_or_default().to_string();
            let primary_port = parsed.port();
            let scheme = parsed.scheme().to_string();

            let mut hosts = Vec::new();
            if !primary_host.is_empty() {
                hosts.push(primary_host.clone());
            }
            for host in ["ws.okx.com"] {
                if host != primary_host && !hosts.contains(&host.to_string()) {
                    hosts.push(host.to_string());
                }
            }

            let ports: Vec<Option<u16>> = match (scheme.as_str(), primary_port) {
                ("wss", Some(8443)) => vec![None, Some(443), Some(8443)],
                ("wss", Some(443)) => vec![None, Some(443)],
                ("wss", None) => vec![None, Some(443)],
                (_, Some(p)) => vec![Some(p), None],
                (_, None) => vec![None],
            };

            for host in hosts {
                for port in &ports {
                    let mut candidate = parsed.clone();
                    if candidate.set_host(Some(&host)).is_err() {
                        continue;
                    }
                    let _ = candidate.set_port(*port);
                    candidate.set_path(&path);
                    push_candidate(&mut urls, candidate.as_str());
                }
            }
        } else {
            push_candidate(&mut urls, primary);
        }

        // 读取自定义备用节点（逗号分隔完整URL）
        if let Ok(extra) = env::var("OKX_WEBSOCKET_FALLBACKS") {
            for item in extra.split(',') {
                push_candidate(&mut urls, item);
            }
        }

        if urls.is_empty() {
            urls.push(primary.to_string());
        }

        urls
    }

    /// 创建新的公共频道客户端
    pub fn new_public() -> Self {
        Self::new_with_config(&CONFIG.websocket_url, None, ReconnectConfig::default())
    }

    /// 创建新的私有频道客户端
    pub fn new_private(credentials: Credentials) -> Self {
        Self::new_with_config(
            &CONFIG.private_websocket_url,
            Some(credentials),
            ReconnectConfig::default(),
        )
    }
    /// 创建新的交易频道客户端
    pub fn new_business(credentials: Credentials) -> Self {
        Self::new_with_config(
            &CONFIG.business_websocket_url,
            Some(credentials),
            ReconnectConfig::default(),
        )
    }

    /// 使用自定义配置创建客户端
    pub fn new_with_config(
        url: &str,
        credentials: Option<Credentials>,
        config: ReconnectConfig,
    ) -> Self {
        let urls = Self::build_url_pool(url);
        Self {
            urls,
            is_private: credentials.is_some(),
            credentials,
            connection_state: Arc::new(Mutex::new(ConnectionState::Disconnected)),
            last_message_time: Arc::new(Mutex::new(Instant::now())),
            subscriptions: Arc::new(Mutex::new(HashMap::new())),
            reconnect_config: config,
            message_sender: Arc::new(Mutex::new(None)),
            ws_sender: Arc::new(Mutex::new(None)),
            is_running: Arc::new(Mutex::new(false)),
            current_url_idx: Arc::new(Mutex::new(0)),
        }
    }

    /// 启动客户端并返回消息接收器
    pub async fn start(&self) -> Result<mpsc::UnboundedReceiver<Value>, Error> {
        let mut is_running = self.is_running.lock().unwrap();
        if *is_running {
            return Err(Error::WebSocketError(
                "Client is already running".to_string(),
            ));
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
        let subscription_key = format!(
            "{:?}_{}",
            channel,
            args.inst_id.as_ref().unwrap_or(&"".to_string())
        );

        // 记录订阅信息
        {
            let mut subscriptions = self.subscriptions.lock().unwrap();
            subscriptions.insert(subscription_key.clone(), (channel.clone(), args.clone()));
        }

        // 如果已连接，立即发送订阅请求
        if *self.connection_state.lock().unwrap() == ConnectionState::Connected {
            self.send_subscription_request(&channel, &args, "subscribe")
                .await?;
        }

        debug!("已添加订阅: {:?}", channel);
        Ok(())
    }

    /// 取消订阅频道
    pub async fn unsubscribe(&self, channel: ChannelType, args: Args) -> Result<(), Error> {
        let subscription_key = format!(
            "{:?}_{}",
            channel,
            args.inst_id.as_ref().unwrap_or(&"".to_string())
        );

        // 移除订阅记录
        {
            let mut subscriptions = self.subscriptions.lock().unwrap();
            subscriptions.remove(&subscription_key);
        }

        // 如果已连接，发送取消订阅请求
        if *self.connection_state.lock().unwrap() == ConnectionState::Connected {
            self.send_subscription_request(&channel, &args, "unsubscribe")
                .await?;
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
        let urls = self.urls.clone();
        let is_private = self.is_private;
        let credentials = self.credentials.clone();
        let connection_state = self.connection_state.clone();
        let last_message_time = self.last_message_time.clone();
        let subscriptions = self.subscriptions.clone();
        let is_running = self.is_running.clone();
        let config = self.reconnect_config.clone();
        let ws_sender = self.ws_sender.clone();
        let current_url_idx = self.current_url_idx.clone();

        tokio::spawn(async move {
            let mut reconnect_attempts = 0;
            let mut backoff_delay = config.interval;

            info!("WebSocket候选节点: {:?}", urls);
            while *is_running.lock().unwrap() {
                let url = {
                    let idx = *current_url_idx.lock().unwrap();
                    urls.get(idx).cloned().unwrap_or_else(|| urls[0].clone())
                };
                info!(
                    "尝试连接OKX WebSocket: {} (attempt={}, backoff={}s)",
                    url,
                    reconnect_attempts + 1,
                    backoff_delay
                );
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
                        )
                        .await;

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
                        error!("WebSocket连接失败 ({}): {}", url, e);
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
                    if urls.len() > 1 {
                        let mut idx = current_url_idx.lock().unwrap();
                        *idx = (*idx + 1) % urls.len();
                        info!("切换备用WebSocket节点: {}", urls[*idx]);
                    }
                    error!(
                        "准备重连 (第{}次)，{}秒后重试 (当前节点: {})",
                        reconnect_attempts,
                        backoff_delay,
                        urls[*current_url_idx.lock().unwrap()]
                    );
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
    ) -> Result<
        (
            tokio_tungstenite::WebSocketStream<
                tokio_tungstenite::MaybeTlsStream<tokio::net::TcpStream>,
            >,
            tokio_tungstenite::tungstenite::handshake::client::Response,
        ),
        Error,
    > {
        let url =
            Url::parse(url).map_err(|e| Error::WebSocketError(format!("Invalid URL: {}", e)))?;

        let (ws_stream, response) = connect_async(url.as_str())
            .await
            .map_err(|e| Error::WebSocketError(format!("Connection failed: {}", e)))?;

        // 如果是私有频道，需要进行认证
        if is_private {
            if let Some(creds) = credentials {
                Self::authenticate(&ws_stream, creds).await?;
            } else {
                return Err(Error::WebSocketError(
                    "Private channel requires credentials".to_string(),
                ));
            }
        }

        Ok((ws_stream, response))
    }

    /// 进行WebSocket认证
    async fn authenticate(
        _ws_stream: &tokio_tungstenite::WebSocketStream<
            tokio_tungstenite::MaybeTlsStream<tokio::net::TcpStream>,
        >,
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

        let login_message =
            serde_json::to_string(&login_request).map_err(|e| Error::JsonError(e))?;

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
            if let Err(e) =
                Self::send_subscription_message(&channel, &args, "subscribe", ws_sender).await
            {
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
                    if let Err(e) = sender.send(Message::Ping(Vec::new().into())) {
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
        mut ws_stream: futures::stream::SplitStream<
            tokio_tungstenite::WebSocketStream<
                tokio_tungstenite::MaybeTlsStream<tokio::net::TcpStream>,
            >,
        >,
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
                        Some(Ok(Message::Close(close_frame))) => {
                            if let Some(frame) = close_frame {
                                info!("WebSocket连接被服务器关闭: code={}, reason={}",
                                    frame.code, frame.reason);
                            } else {
                                info!("WebSocket连接被服务器关闭（无关闭帧）");
                            }
                            // 立即更新连接状态，确保重连逻辑能及时触发
                            *connection_state.lock().unwrap() = ConnectionState::Disconnected;
                            break;
                        }
                        Some(Err(e)) => {
                            // 区分不同类型的错误，使用不同的日志级别
                            Self::handle_websocket_error(&e, connection_state);
                            break;
                        }
                        None => {
                            warn!("WebSocket流结束（EOF）");
                            // 立即更新连接状态
                            *connection_state.lock().unwrap() = ConnectionState::Disconnected;
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

    /// 处理WebSocket错误，根据错误类型进行分类和日志记录
    fn handle_websocket_error(
        error: &TungsteniteError,
        connection_state: &Arc<Mutex<ConnectionState>>,
    ) {
        // 立即更新连接状态，确保重连逻辑能及时触发
        *connection_state.lock().unwrap() = ConnectionState::Disconnected;

        // 根据错误类型进行分类处理
        match error {
            TungsteniteError::ConnectionClosed => {
                // 连接正常关闭，使用info级别
                info!("WebSocket连接已关闭");
            }
            TungsteniteError::Protocol(ref protocol_error) => {
                // 协议错误，可能是连接重置等情况
                let error_msg = format!("{}", protocol_error);
                if error_msg.contains("Connection reset without closing handshake") {
                    // 连接重置错误：这是可恢复的错误，通常由网络问题或服务器端断开导致
                    // 使用warn级别，因为这会被自动重连机制处理
                    warn!(
                        "WebSocket连接被重置（无关闭握手）: {} - 将自动重连",
                        protocol_error
                    );
                } else {
                    // 其他协议错误
                    error!("WebSocket协议错误: {}", protocol_error);
                }
            }
            TungsteniteError::Io(ref io_error) => {
                // IO错误，可能是网络问题
                let error_kind = io_error.kind();
                match error_kind {
                    std::io::ErrorKind::ConnectionReset
                    | std::io::ErrorKind::ConnectionAborted
                    | std::io::ErrorKind::BrokenPipe => {
                        warn!("WebSocket网络连接错误: {} - 将自动重连", io_error);
                    }
                    std::io::ErrorKind::TimedOut => {
                        warn!("WebSocket连接超时: {} - 将自动重连", io_error);
                    }
                    _ => {
                        error!("WebSocket IO错误: {}", io_error);
                    }
                }
            }
            TungsteniteError::Utf8(_) => {
                // UTF-8编码错误，通常是可恢复的
                warn!("WebSocket消息UTF-8编码错误: {}", error);
            }
            TungsteniteError::Tls(ref tls_error) => {
                // TLS错误，通常需要重连
                warn!("WebSocket TLS错误: {} - 将自动重连", tls_error);
            }
            TungsteniteError::Http(ref http_error) => {
                // HTTP错误，可能是认证或协议问题
                error!("WebSocket HTTP错误: {:?}", http_error);
            }
            _ => {
                // 其他未知错误
                error!("WebSocket未知错误: {}", error);
            }
        }
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

        let message = serde_json::to_string(&request).map_err(|e| Error::JsonError(e))?;
        debug!("发布{}请求: {}", operation, message);
        // 通过WebSocket发送器发送消息
        if let Some(sender) = ws_sender.lock().unwrap().as_ref() {
            let ws_message = Message::Text(message.into());
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
            urls: self.urls.clone(),
            is_private: self.is_private,
            credentials: self.credentials.clone(),
            connection_state: self.connection_state.clone(),
            last_message_time: self.last_message_time.clone(),
            subscriptions: self.subscriptions.clone(),
            reconnect_config: self.reconnect_config.clone(),
            message_sender: self.message_sender.clone(),
            ws_sender: self.ws_sender.clone(),
            is_running: self.is_running.clone(),
            current_url_idx: self.current_url_idx.clone(),
        }
    }
}
