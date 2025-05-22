use reqwest::{Client, Method, StatusCode};
use serde::{de, Deserialize, Serialize};
use std::time::Duration;

use crate::config::{Credentials, CONFIG};
use crate::error::Error;
use crate::utils;
use log::{debug, error, info};
/// 通用的OKX API响应结构
#[derive(Serialize, Deserialize, Debug)]
pub struct OkxApiResponse<T> {
    pub code: String,
    pub msg: String,
    pub data: T,
}

/// OKX API错误响应
#[derive(Serialize, Deserialize, Debug)]
struct OkxApiErrorResponse {
    msg: String,
    code: String,
}

/// OKX HTTP API客户端
#[derive(Debug, Clone)]
pub struct OkxClient {
    /// HTTP客户端
    client: Client,
    /// API凭证
    credentials: Credentials,
    /// 是否使用模拟交易
    is_simulated_trading: String,
    /// API基础URL
    base_url: String,
    /// 请求有效期（毫秒）
    request_expiration_ms: i64,
}

impl OkxClient {
    /// 创建一个新的OKX客户端
    pub fn new(credentials: Credentials) -> Result<Self, Error> {
        let client = Client::builder()
            .timeout(Duration::from_millis(CONFIG.api_timeout_ms))
            .build()
            .map_err(Error::HttpError)?;

        Ok(Self {
            client,
            is_simulated_trading: credentials.is_simulated_trading.clone(),
            credentials,
            base_url: CONFIG.api_url.clone(),
            request_expiration_ms: CONFIG.request_expiration_ms,
        })
    }

    /// 从环境变量创建OKX客户端
    pub fn from_env() -> Result<Self, Error> {
        let credentials = Credentials::from_env()?;
        info!("OKX credentials: {:?}", credentials);
        Self::new(credentials)
    }

    /// 设置是否使用模拟交易
    pub fn set_simulated_trading(&mut self, is_simulated: String) {
        self.is_simulated_trading = is_simulated;
    }

    /// 设置API基础URL
    pub fn set_base_url(&mut self, base_url: impl Into<String>) {
        self.base_url = base_url.into();
    }

    /// 设置请求有效期
    pub fn set_request_expiration(&mut self, expiration_ms: i64) {
        self.request_expiration_ms = expiration_ms;
    }

    /// 发送API请求并返回反序列化的响应
    pub async fn send_request<T: for<'a> Deserialize<'a>>(
        &self,
        method: Method,
        path: &str,
        body: &str,
    ) -> Result<T, Error> {
        let timestamp = utils::generate_timestamp();
        let signature = utils::generate_signature(
            &self.credentials.api_secret,
            &timestamp,
            &method,
            path,
            body,
        )?;
        debug!("OKX signature: {}", signature);
        let exp_time = utils::generate_expiration_timestamp(self.request_expiration_ms);

        let url = format!("{}{}", self.base_url, path);
        debug!("请求OKX API: {}", url);

        let mut request_builder = self
            .client
            .request(method, &url)
            .header("OK-ACCESS-KEY", &self.credentials.api_key)
            .header("OK-ACCESS-SIGN", signature)
            .header("OK-ACCESS-TIMESTAMP", timestamp)
            .header("OK-ACCESS-PASSPHRASE", &self.credentials.passphrase)
            .header("Content-Type", "application/json")
            .header("expTime", exp_time.to_string());
        if self.is_simulated_trading == "1" {
            request_builder = request_builder.header("x-simulated-trading", "1");
        }

        debug!("OKX body_string: {}", body.to_string());
        let request_builder = request_builder.body(body.to_string());
        let response = request_builder.send().await.map_err(Error::HttpError)?;
        let status_code = response.status();
        let response_body = response.text().await.map_err(Error::HttpError)?;
        debug!("OKX API响应状态码: {}", status_code);

        match status_code {
            StatusCode::OK => {
                println!("OKX API响应: {}", response_body);
                let result: OkxApiResponse<T> =
                    serde_json::from_str(&response_body).map_err(|e| Error::JsonError(e))?;
                if result.code != "0" {
                    return Err(Error::OkxApiError {
                        code: result.code,
                        message: result.msg,
                    });
                }
                Ok(result.data)
            }
            StatusCode::NOT_FOUND => {
                error!("OKX API错误响应: {}", response_body);
                Err(Error::OkxApiError {
                    code: "404".to_string(),
                    message: "API not found".to_string(),
                })
            }
            _ => {
                error!("OKX API错误响应: {}", response_body);
                Err(Error::OkxApiError {
                    code: status_code.to_string(),
                    message: response_body,
                })
            }
        }
    }
}
