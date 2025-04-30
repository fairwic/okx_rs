use hmac::{Hmac, Mac};
use sha2::Sha256;
use base64::{engine::general_purpose, Engine as _};
use reqwest::Method;
use crate::error::Error;

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