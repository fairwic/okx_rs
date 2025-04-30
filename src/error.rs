use std::fmt;
use thiserror::Error;

/// OKX SDK的统一错误类型
#[derive(Error, Debug)]
pub enum Error {
    /// API请求错误
    #[error("API请求错误: {0}")]
    ApiRequestError(String),

    /// HTTP客户端错误
    #[error("HTTP错误: {0}")]
    HttpError(#[from] reqwest::Error),

    /// JSON序列化/反序列化错误
    #[error("JSON错误: {0}")]
    JsonError(#[from] serde_json::Error),

    /// IO错误
    #[error("IO错误: {0}")]
    IoError(#[from] std::io::Error),

    /// WebSocket错误
    #[error("WebSocket错误: {0}")]
    WebSocketError(String),

    /// 参数错误
    #[error("参数错误: {0}")]
    ParameterError(String),

    /// 解析错误
    #[error("解析错误: {0}")]
    ParseError(String),

    /// 配置错误
    #[error("配置错误: {0}")]
    ConfigError(String),

    /// 认证错误
    #[error("认证错误: {0}")]
    AuthenticationError(String),

    /// OKX API错误
    #[error("OKX API错误 (代码: {code}): {message}")]
    OkxApiError { code: String, message: String },

    /// 未知错误
    #[error("未知错误: {0}")]
    Unknown(String),
}

/// OKX API特定错误码
#[derive(Error, Debug, Clone, Copy, PartialEq, Eq)]
pub enum ApiErrorCode {
    /// 操作成功
    Ok = 0,
    /// 操作全部失败
    OperationFailed = 1,
    /// 批量操作部分成功
    PartialSuccess = 2,
    
    // 通用错误码 (50000-50999)
    /// POST请求的body不能为空
    EmptyBody = 50000,
    /// 服务暂时不可用，请稍后重试
    ServiceUnavailable = 50001,
    /// JSON 语法错误
    JsonSyntaxError = 50002,
    /// 接口请求超时
    RequestTimeout = 50004,
    /// 接口已下线或无法使用
    InterfaceDeprecated = 50005,
    /// 无效的Content-Type
    InvalidContentType = 50006,
    /// 用户被冻结
    UserFrozen = 50007,
    /// 用户不存在
    UserNotFound = 50008,
    /// 用户处于爆仓冻结
    UserMarginFrozen = 50009,
    /// 用户ID为空
    UserIdEmpty = 50010,
    /// 请求频率太高
    TooManyRequests = 50011,
    /// 账户状态无效
    InvalidAccountStatus = 50012,
    /// 当前系统繁忙
    SystemBusy = 50013,

    // API 类错误码
    /// Api 已被冻结
    ApiFrozen = 50100,
    /// APIKey 与当前环境不匹配
    ApiKeyEnvironmentMismatch = 50101,
    /// 请求时间戳过期
    RequestTimestampExpired = 50102,
    /// 请求头"OK-ACCESS-KEY"不能为空
    MissingOkAccessKey = 50103,
    /// 请求头"OK-ACCESS-PASSPHRASE"不能为空
    MissingOkAccessPassphrase = 50104,
    /// 请求头"OK-ACCESS-PASSPHRASE"错误
    InvalidOkAccessPassphrase = 50105,
    /// 请求头"OK-ACCESS-SIGN"不能为空
    MissingOkAccessSign = 50106,
    /// 请求头"OK-ACCESS-TIMESTAMP"不能为空
    MissingOkAccessTimestamp = 50107,
    
    // 未知错误
    /// 未知错误
    Unknown = 99999,
}

impl fmt::Display for ApiErrorCode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?} ({})", self, *self as i32)
    }
}

impl ApiErrorCode {
    /// 从错误码获取ApiErrorCode枚举
    pub fn from_code(code: u32) -> Self {
        match code {
            0 => Self::Ok,
            1 => Self::OperationFailed,
            2 => Self::PartialSuccess,
            50000 => Self::EmptyBody,
            50001 => Self::ServiceUnavailable,
            50002 => Self::JsonSyntaxError,
            50004 => Self::RequestTimeout,
            50005 => Self::InterfaceDeprecated,
            50006 => Self::InvalidContentType,
            50007 => Self::UserFrozen,
            50008 => Self::UserNotFound,
            50009 => Self::UserMarginFrozen,
            50010 => Self::UserIdEmpty,
            50011 => Self::TooManyRequests,
            50012 => Self::InvalidAccountStatus,
            50013 => Self::SystemBusy,
            50100 => Self::ApiFrozen,
            50101 => Self::ApiKeyEnvironmentMismatch,
            50102 => Self::RequestTimestampExpired,
            50103 => Self::MissingOkAccessKey,
            50104 => Self::MissingOkAccessPassphrase,
            50105 => Self::InvalidOkAccessPassphrase,
            50106 => Self::MissingOkAccessSign,
            50107 => Self::MissingOkAccessTimestamp,
            _ => Self::Unknown,
        }
    }
}

/// 把任何错误转换为Error类型的结果
pub fn to_err<E: std::error::Error + Send + Sync + 'static>(err: E) -> Error {
    Error::Unknown(err.to_string())
} 