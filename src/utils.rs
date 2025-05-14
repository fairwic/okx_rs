use hmac::{Hmac, Mac};
use sha2::Sha256;
use base64::{engine::general_purpose, Engine as _};
use reqwest::Method;
use crate::error::{self, Error};
use crate::{api, client};

/// 生成API请求签名
pub fn generate_signature(
    api_secret: &str,
    timestamp: &str,
    method: &Method,
    path: &str,
    body: &str,
) -> Result<String, Error> {
    let sign_payload = format!("{}{}{}{}", timestamp, method.as_str(), path, body);
    
    let mut hmac = Hmac::<Sha256>::new_from_slice(api_secret.as_bytes())
        .map_err(|e| Error::AuthenticationError(format!("创建HMAC失败: {}", e)))?;
        
    hmac.update(sign_payload.as_bytes());
    let signature = general_purpose::STANDARD.encode(hmac.finalize().into_bytes());
    
    Ok(signature)
}

/// 生成当前ISO 8601格式的时间戳
pub fn generate_timestamp() -> String {
    chrono::Utc::now()
        .format("%Y-%m-%dT%H:%M:%S.%3fZ")
        .to_string()
}

/// 秒级时间戳
pub fn generate_timestamp_websocket() -> String {
    chrono::Utc::now()
        .timestamp()
        .to_string()
}

/// 生成请求的截止时间戳（毫秒）
pub fn generate_expiration_timestamp(expiration_ms: i64) -> i64 {
    chrono::Utc::now().timestamp_millis() + expiration_ms
}

/// 从字符串解析毫秒时间戳
pub fn parse_timestamp_ms(timestamp_str: &str) -> Result<i64, Error> {
    timestamp_str.parse::<i64>()
        .map_err(|_| Error::ParseError(format!("无法解析时间戳: {}", timestamp_str)))
}

/// 检查服务器时间与本地时间的差异是否在允许范围内
pub fn is_time_synchronized(server_time_ms: i64, allowed_diff_ms: i64) -> bool {
    let local_time_ms = chrono::Utc::now().timestamp_millis();
    let diff_ms = (local_time_ms - server_time_ms).abs();
    diff_ms <= allowed_diff_ms
}

/// 时间戳转为DateTime对象
pub fn timestamp_to_datetime(timestamp_ms: i64) -> Result<chrono::DateTime<chrono::Utc>, Error> {
    let seconds = timestamp_ms / 1000;
    let nanos = ((timestamp_ms % 1000) * 1_000_000) as u32;
    
    chrono::NaiveDateTime::from_timestamp_opt(seconds, nanos)
        .map(|dt| chrono::DateTime::from_utc(dt, chrono::Utc))
        .ok_or_else(|| Error::ParseError(format!("无法转换时间戳: {}", timestamp_ms)))
} 


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