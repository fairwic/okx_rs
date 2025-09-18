use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::{mpsc, Mutex, RwLock};
use serde_json::Value;
use log::{debug, info, warn};

use crate::config::Credentials;
use crate::error::Error;
use super::auto_reconnect_client::{AutoReconnectWebsocketClient, ConnectionState, ReconnectConfig};
use super::channel::{Args, ChannelType};

/// WebSocket连接管理器配置
#[derive(Debug, Clone)]
pub struct ManagerConfig {
    /// 重连间隔（秒）
    pub reconnect_interval: u64,
    /// 最大重连次数
    pub max_reconnect_attempts: u32,
    /// 心跳间隔（秒）
    pub heartbeat_interval: u64,
    /// 消息超时时间（秒）
    pub message_timeout: u64,
    /// 是否启用自动重连
    pub auto_reconnect: bool,
}

impl Default for ManagerConfig {
    fn default() -> Self {
        Self {
            reconnect_interval: 5,
            max_reconnect_attempts: 10,
            heartbeat_interval: 30,
            message_timeout: 60,
            auto_reconnect: true,
        }
    }
}

/// 订阅信息
#[derive(Debug, Clone)]
pub struct SubscriptionInfo {
    pub channel: ChannelType,
    pub args: Args,
    pub is_active: bool,
}

/// WebSocket连接管理器
/// 
/// 提供统一的WebSocket连接管理，包括：
/// - 自动重连机制
/// - 订阅管理
/// - 连接状态监控
/// - 错误恢复
pub struct OkxWebsocketManager {
    config: ManagerConfig,
    public_client: Arc<Mutex<AutoReconnectWebsocketClient>>,
    private_client: Option<Arc<Mutex<AutoReconnectWebsocketClient>>>,
    business_client: Option<Arc<Mutex<AutoReconnectWebsocketClient>>>,
    subscriptions: Arc<RwLock<HashMap<String, SubscriptionInfo>>>,
    message_sender: Arc<Mutex<Option<mpsc::UnboundedSender<Value>>>>,
    is_running: Arc<Mutex<bool>>,
}

impl OkxWebsocketManager {
    /// 创建新的WebSocket管理器（仅公共频道）
    pub fn new_public() -> Self {
        Self::new_with_config(None, ManagerConfig::default())
    }

    /// 创建新的WebSocket管理器（包含私有频道）
    pub fn new_with_credentials(credentials: Credentials) -> Self {
        Self::new_with_config(Some(credentials), ManagerConfig::default())
    }

    /// 使用自定义配置创建WebSocket管理器
    pub fn new_with_config(credentials: Option<Credentials>, config: ManagerConfig) -> Self {
        // 转换配置
        let reconnect_config = ReconnectConfig {
            enabled: config.auto_reconnect,
            interval: config.reconnect_interval,
            max_attempts: config.max_reconnect_attempts,
            backoff_factor: 1.5,
            max_backoff: 60,
            heartbeat_interval: config.heartbeat_interval,
            message_timeout: config.message_timeout,
        };

        let public_client = Arc::new(Mutex::new(
            AutoReconnectWebsocketClient::new_public()
        ));

        let private_client = credentials.clone().map(|creds| {
            Arc::new(Mutex::new(
                AutoReconnectWebsocketClient::new_private(creds)
            ))
        });
        let business_client = credentials.clone().map(|creds| {
            Arc::new(Mutex::new(
                AutoReconnectWebsocketClient::new_business(creds)
            ))
        });

        Self {
            config,
            public_client,
            private_client,
            business_client,
            subscriptions: Arc::new(RwLock::new(HashMap::new())),
            message_sender: Arc::new(Mutex::new(None)),
            is_running: Arc::new(Mutex::new(false)),
        }
    }

    /// 启动WebSocket管理器
    pub async fn start(&self) -> Result<mpsc::UnboundedReceiver<Value>, Error> {
        let mut is_running = self.is_running.lock().await;
        if *is_running {
            return Err(Error::WebSocketError("Manager is already running".to_string()));
        }

        let (tx, rx) = mpsc::unbounded_channel();
        *self.message_sender.lock().await = Some(tx.clone());
        *is_running = true;

        // 启动公共客户端
        let public_rx = self.public_client.lock().await.start().await?;
        self.start_message_forwarder(public_rx, tx.clone()).await;

        // 启动私有客户端（如果存在）
        if let Some(private_client) = &self.private_client {
            let private_rx = private_client.lock().await.start().await?;
            self.start_message_forwarder(private_rx, tx.clone()).await;
        }

        info!("WebSocket管理器已启动");
        Ok(rx)
    }

    /// 停止WebSocket管理器
    pub async fn stop(&self) {
        *self.is_running.lock().await = false;
        *self.message_sender.lock().await = None;

        // 关闭所有连接
        self.public_client.lock().await.stop().await;
        if let Some(private_client) = &self.private_client {
            private_client.lock().await.stop().await;
        }

        info!("WebSocket管理器已停止");
    }

    /// 订阅频道
    pub async fn subscribe(&self, channel: ChannelType, args: Args) -> Result<(), Error> {
        let subscription_key = format!("{:?}_{}", channel, args.inst_id.as_ref().unwrap_or(&"".to_string()));
        
        // 记录订阅信息
        {
            let mut subscriptions = self.subscriptions.write().await;
            subscriptions.insert(subscription_key.clone(), SubscriptionInfo {
                channel: channel.clone(),
                args: args.clone(),
                is_active: false,
            });
        }

        // 执行订阅
        self.execute_subscription(&channel, &args).await?;
        
        // 标记为活跃
        {
            let mut subscriptions = self.subscriptions.write().await;
            if let Some(sub) = subscriptions.get_mut(&subscription_key) {
                sub.is_active = true;
            }
        }

        debug!("已订阅频道: {:?}", channel);
        Ok(())
    }

    /// 取消订阅频道
    pub async fn unsubscribe(&self, channel: ChannelType, args: Args) -> Result<(), Error> {
        let subscription_key = format!("{:?}_{}", channel, args.inst_id.as_ref().unwrap_or(&"".to_string()));
        
        // 执行取消订阅
        self.execute_unsubscription(&channel, &args).await?;
        
        // 移除订阅记录
        {
            let mut subscriptions = self.subscriptions.write().await;
            subscriptions.remove(&subscription_key);
        }

        debug!("已取消订阅频道: {:?}", channel);
        Ok(())
    }

    /// 获取连接状态
    pub async fn get_connection_status(&self) -> (ConnectionState, Option<ConnectionState>) {
        let public_state = self.public_client.lock().await.get_connection_state();
        let private_state = if let Some(private_client) = &self.private_client {
            Some(private_client.lock().await.get_connection_state())
        } else {
            None
        };
        (public_state, private_state)
    }

    /// 获取活跃订阅数量
    pub async fn get_active_subscriptions_count(&self) -> usize {
        let subscriptions = self.subscriptions.read().await;
        subscriptions.values().filter(|sub| sub.is_active).count()
    }

    /// 执行订阅
    async fn execute_subscription(&self, channel: &ChannelType, args: &Args) -> Result<(), Error> {
        match channel {
            // 公共频道使用公共客户端
            ChannelType::Tickers | ChannelType::Books | ChannelType::Trades |
            ChannelType::BooksLite | ChannelType::Books50L | ChannelType::BlockTickers |
            ChannelType::IndexTickers | ChannelType::MarkPrice | ChannelType::PriceLimit |
            ChannelType::EstimatedPrice | ChannelType::FundingRate | ChannelType::Status |
            ChannelType::IndexCandle(_) | ChannelType::MarkPriceCandle(_) => {
                self.public_client.lock().await.subscribe(channel.clone(), args.clone()).await
            }
            // 私有频道使用私有客户端
            ChannelType::Candle(_) | ChannelType::Account | ChannelType::Orders |
            ChannelType::Positions | ChannelType::AlgoOrders | ChannelType::AdvancedAlgoOrders |
            ChannelType::OrdersAlgo | ChannelType::Balance | ChannelType::PositionRisk |
            ChannelType::BalanceAndPosition | ChannelType::Greeks | ChannelType::DepositInfo => {
                if let Some(private_client) = &self.private_client {
                    private_client.lock().await.subscribe(channel.clone(), args.clone()).await
                } else {
                    Err(Error::WebSocketError("Private client not available".to_string()))
                }
            }
            // 自定义频道默认使用公共客户端
            ChannelType::Custom(_) => {
                self.public_client.lock().await.subscribe(channel.clone(), args.clone()).await
            }
        }
    }

    /// 执行取消订阅
    async fn execute_unsubscription(&self, channel: &ChannelType, args: &Args) -> Result<(), Error> {
        match channel {
            // 公共频道使用公共客户端
            ChannelType::Tickers | ChannelType::Books | ChannelType::Trades |
            ChannelType::BooksLite | ChannelType::Books50L | ChannelType::BlockTickers |
            ChannelType::IndexTickers | ChannelType::MarkPrice | ChannelType::PriceLimit |
            ChannelType::EstimatedPrice | ChannelType::FundingRate | ChannelType::Status |
            ChannelType::IndexCandle(_) | ChannelType::MarkPriceCandle(_) => {
                self.public_client.lock().await.unsubscribe(channel.clone(), args.clone()).await
            }
            // 私有频道使用私有客户端
            ChannelType::Candle(_) | ChannelType::Account | ChannelType::Orders |
            ChannelType::Positions | ChannelType::AlgoOrders | ChannelType::AdvancedAlgoOrders |
            ChannelType::OrdersAlgo | ChannelType::Balance | ChannelType::PositionRisk |
            ChannelType::BalanceAndPosition | ChannelType::Greeks | ChannelType::DepositInfo => {
                if let Some(private_client) = &self.private_client {
                    private_client.lock().await.unsubscribe(channel.clone(), args.clone()).await
                } else {
                    Err(Error::WebSocketError("Private client not available".to_string()))
                }
            }
            // 自定义频道默认使用公共客户端
            ChannelType::Custom(_) => {
                self.public_client.lock().await.unsubscribe(channel.clone(), args.clone()).await
            }
        }
    }

    /// 启动消息转发器
    async fn start_message_forwarder(
        &self,
        mut rx: mpsc::UnboundedReceiver<Value>,
        tx: mpsc::UnboundedSender<Value>,
    ) {
        tokio::spawn(async move {
            while let Some(message) = rx.recv().await {
                if tx.send(message).is_err() {
                    warn!("消息转发失败，接收器可能已关闭");
                    break;
                }
            }
            debug!("消息转发器结束");
        });
    }


}
