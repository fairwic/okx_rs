// OKX SDK - Rust Client Library
// 提供与OKX交易所API的通信能力

pub mod client;
pub mod error;
pub mod dto;
pub mod api;
pub mod utils;
pub mod config;
pub mod websocket;

/// OKX SDK的版本
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

/// Re-export commonly used modules and functions
pub use client::OkxClient;
pub use api::{
    account::OkxAccount,
    trade::OkxTrade,
    market::OkxMarket,
    public_data::OkxPublicData,
    asset::OkxAsset,
    big_data::OkxBigData,
    websocket::OkxWebsocketApi,
    examples,
};
pub use error::Error;
pub use websocket::OkxWebsocketClient;
use crate::dto::common;

/// 验证系统时间，检查本地时间与OKX服务器时间的差异
pub async fn validate_system_time() -> Result<i64, error::Error> {
    let time_str = api::public_data::OkxPublicData::get_time().await
        .map_err(|e| error::Error::ApiRequestError(format!("获取OKX系统时间失败: {}", e)))?;
    
    let time = time_str.parse::<i64>()
        .map_err(|_| error::Error::ParseError("解析时间字符串失败".to_string()))?;
    
    let time = chrono::DateTime::<chrono::Utc>::from_utc(
        chrono::NaiveDateTime::from_timestamp_opt(time / 1000, ((time % 1000) * 1_000_000) as u32)
            .ok_or_else(|| error::Error::ParseError("创建时间戳失败".to_string()))?,
        chrono::Utc,
    );

    let now = chrono::Utc::now().timestamp_millis();
    let okx_time = time.timestamp_millis();
    let time_diff = (now - okx_time).abs();
    
    if time_diff < 20000 {
        log::info!("时间间隔相差值: {} 毫秒", time_diff);
    } else {
        log::warn!("时间未同步，时间间隔相差值: {} 毫秒", time_diff);
    }
    
    Ok(time_diff)
}

/// 使用环境变量配置初始化OKX客户端
pub fn create_client() -> Result<client::OkxClient, error::Error> {
    client::OkxClient::from_env()
} 