use crate::error::Error;
use once_cell::sync::Lazy;
use serde::de::value;
use std::env;
use std::sync::Once;

/// OKX API的默认基础URL
pub const DEFAULT_API_URL: &str = "https://www.okx.com";

/// OKX WebSocket的默认URL
pub const DEFAULT_WEBSOCKET_URL: &str = "wss://ws.okx.com:8443/ws/v5/public";
pub const DEFAULT_PRIVATE_WEBSOCKET_URL: &str = "wss://ws.okx.com:8443/ws/v5/private";

/// OKX API超时配置（毫秒）
pub const DEFAULT_API_TIMEOUT_MS: u64 = 5000;

/// 默认请求有效时间（毫秒）
pub const DEFAULT_REQUEST_EXPIRATION_MS: i64 = 1000;

/// 环境初始化状态
static INIT_ENV: Once = Once::new();

/// 全局配置
pub static CONFIG: Lazy<Config> = Lazy::new(|| {
    let mut config = Config::default();

    if let Ok(api_url) = env::var("OKX_API_URL") {
        config.api_url = api_url;
    }
    if let Ok(ws_url) = env::var("OKX_WEBSOCKET_URL") {
        config.websocket_url = ws_url;
    }
    if let Ok(private_ws_url) = env::var("OKX_PRIVATE_WEBSOCKET_URL") {
        config.private_websocket_url = private_ws_url;
    }
    if let Ok(timeout) = env::var("OKX_API_TIMEOUT_MS").map(|v| v.parse::<u64>()) {
        if let Ok(timeout) = timeout {
            config.api_timeout_ms = timeout;
        }
    }
    if let Ok(expiration) = env::var("OKX_REQUEST_EXPIRATION_MS").map(|v| v.parse::<i64>()) {
        if let Ok(expiration) = expiration {
            config.request_expiration_ms = expiration;
        }
    }
    if let Ok(value) = env::var("OKX_SIMULATED_TRADING") {
        config.is_simulated_trading = value;
    }

    println!("config{:?}", config);
    config
});

/// OKX SDK配置
#[derive(Debug, Clone)]
pub struct Config {
    /// API基础URL
    pub api_url: String,
    /// WebSocket URL
    pub websocket_url: String,
    /// 私有WebSocket URL
    pub private_websocket_url: String,
    /// API超时时间（毫秒）
    pub api_timeout_ms: u64,
    /// 请求有效时间（毫秒）
    pub request_expiration_ms: i64,
    /// 是否为模拟交易
    pub is_simulated_trading: String,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            api_url: DEFAULT_API_URL.to_string(),
            websocket_url: DEFAULT_WEBSOCKET_URL.to_string(),
            private_websocket_url: DEFAULT_PRIVATE_WEBSOCKET_URL.to_string(),
            api_timeout_ms: DEFAULT_API_TIMEOUT_MS,
            request_expiration_ms: DEFAULT_REQUEST_EXPIRATION_MS,
            is_simulated_trading: "1".into(),
        }
    }
}

impl Config {
    /// 创建一个新的配置实例
    pub fn new() -> Self {
        Self::default()
    }

    /// 设置API基础URL
    pub fn with_api_url(mut self, api_url: impl Into<String>) -> Self {
        self.api_url = api_url.into();
        self
    }

    /// 设置WebSocket URL
    pub fn with_websocket_url(mut self, websocket_url: impl Into<String>) -> Self {
        self.websocket_url = websocket_url.into();
        self
    }

    /// 设置私有WebSocket URL
    pub fn with_private_websocket_url(mut self, private_websocket_url: impl Into<String>) -> Self {
        self.private_websocket_url = private_websocket_url.into();
        self
    }

    /// 设置API超时时间
    pub fn with_api_timeout_ms(mut self, timeout_ms: u64) -> Self {
        self.api_timeout_ms = timeout_ms;
        self
    }

    /// 设置请求有效时间
    pub fn with_request_expiration_ms(mut self, expiration_ms: i64) -> Self {
        self.request_expiration_ms = expiration_ms;
        self
    }

    /// 设置是否为模拟交易
    pub fn with_simulated_trading(mut self, is_simulated: String) -> Self {
        self.is_simulated_trading = is_simulated;
        self
    }
}

/// API凭证
#[derive(Debug, Clone)]
pub struct Credentials {
    /// API密钥
    pub api_key: String,
    /// API密钥
    pub api_secret: String,
    /// API密码
    pub passphrase: String,
}

impl Credentials {
    /// 创建新的凭证实例
    pub fn new(
        api_key: impl Into<String>,
        api_secret: impl Into<String>,
        passphrase: impl Into<String>,
    ) -> Self {
        Self {
            api_key: api_key.into(),
            api_secret: api_secret.into(),
            passphrase: passphrase.into(),
        }
    }

    /// 从环境变量读取凭证
    pub fn from_env() -> Result<Self, Error> {
        // 确保环境变量已加载
        init_env();

        let api_key = env::var("OKX_API_KEY")
            .map_err(|_| Error::ConfigError("缺少环境变量: OKX_API_KEY".to_string()))?;

        let api_secret = env::var("OKX_API_SECRET")
            .map_err(|_| Error::ConfigError("缺少环境变量: OKX_API_SECRET".to_string()))?;

        let passphrase = env::var("OKX_PASSPHRASE")
            .map_err(|_| Error::ConfigError("缺少环境变量: OKX_PASSPHRASE".to_string()))?;

        Ok(Self::new(
            api_key,
            api_secret,
            passphrase,
        ))
    }
}

/// 初始化环境变量
pub fn init_env() {
    INIT_ENV.call_once(|| {
        // 尝试加载.env文件，忽略失败
        dotenv::dotenv().ok();
    });
}
