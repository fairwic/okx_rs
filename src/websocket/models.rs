use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// WebSocket消息操作类型
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum WebSocketOperation {
    /// 订阅频道
    Subscribe,
    /// 取消订阅
    Unsubscribe,
    /// 登录
    Login,
}

/// WebSocket认证信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WebSocketAuth {
    /// API密钥
    pub api_key: String,
    /// 签名
    pub sign: String,
    /// 时间戳
    pub timestamp: String,
    /// 密码
    pub passphrase: String,
}

/// WebSocket通道
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WebSocketChannel {
    /// 通道名称
    pub channel: String,
    /// 产品ID
    #[serde(rename = "instId", skip_serializing_if = "Option::is_none")]
    pub instrument_id: Option<String>,
    /// 额外参数
    #[serde(flatten, skip_serializing_if = "HashMap::is_empty")]
    pub args: HashMap<String, String>,
}

/// WebSocket订阅请求
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WebSocketSubscription {
    /// 通道名称
    pub channel: String,
    /// 产品ID
    #[serde(rename = "instId", skip_serializing_if = "Option::is_none")]
    pub instrument_id: Option<String>,
    /// 额外参数
    #[serde(flatten, skip_serializing_if = "HashMap::is_empty")]
    pub args: HashMap<String, String>,
}

/// WebSocket请求
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WebSocketRequest {
    /// 操作类型
    pub op: WebSocketOperation,
    /// 参数
    pub args: Vec<WebSocketSubscription>,
}

/// WebSocket登录请求
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WebSocketLoginRequest {
    /// 操作类型 (login)
    pub op: String,
    /// 认证参数
    pub args: Vec<WebSocketAuth>,
}

/// WebSocket响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WebSocketResponse<T> {
    /// 事件
    pub event: Option<String>,
    /// 操作成功或失败
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    /// 错误消息
    #[serde(skip_serializing_if = "Option::is_none")]
    pub msg: Option<String>,
    /// 通道名称
    #[serde(skip_serializing_if = "Option::is_none")]
    pub channel: Option<String>,
    /// 产品ID
    #[serde(rename = "instId", skip_serializing_if = "Option::is_none")]
    pub instrument_id: Option<String>,
    /// 数据
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<T>,
}

/// WebSocket消息
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum WebSocketMessage<T> {
    /// 响应
    Response(WebSocketResponse<T>),
    /// 原始数据
    Data {
        /// 通道名称
        channel: String,
        /// 产品ID
        #[serde(rename = "instId")]
        instrument_id: String,
        /// 数据
        data: T,
    },
} 