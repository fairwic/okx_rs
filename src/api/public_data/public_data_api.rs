use crate::client::OkxClient;
use crate::error::Error;
use crate::dto::public_data::public_data_dto::{SystemTime, SystemStatus, EconomicEvent, RateLimit};
use crate::dto::market::market_dto::InstrumentOkxResDto;
use crate::api::API_PUBLIC_PATH;
use reqwest::Method;

/// OKX公共数据API
/// 提供公共数据相关的API访问
#[derive(Debug)]
pub struct OkxPublicData {
    /// API客户端
    client: OkxClient,
}

impl OkxPublicData {
    /// 创建一个新的OkxPublicData实例
    pub fn new(client: OkxClient) -> Self {
        Self { client }
    }
    
    /// 从环境变量创建一个新的OkxPublicData实例
    pub fn from_env() -> Result<Self, Error> {
        let client = OkxClient::from_env()?;
        Ok(Self { client })
    }
    
    /// 获取内部客户端引用
    pub fn client(&self) -> &OkxClient {
        &self.client
    }

    /// 获取系统时间
    pub async fn get_time() -> Result<String, Error> {
        let url = format!("{}/time", API_PUBLIC_PATH);
        
        // 不需要认证的请求，可以使用临时客户端
        let client = reqwest::Client::new();
        let response = client
            .get(format!("{}{}", crate::config::DEFAULT_API_URL, url))
            .send()
            .await
            .map_err(Error::HttpError)?;
            
        let text = response.text().await.map_err(Error::HttpError)?;
        let system_time: crate::client::OkxApiResponse<Vec<SystemTime>> = 
            serde_json::from_str(&text).map_err(Error::JsonError)?;
            
        if system_time.code != "0" {
            return Err(Error::OkxApiError {
                code: system_time.code,
                message: system_time.msg,
            });
        }
        
        if let Some(time) = system_time.data.first() {
            Ok(time.ts.clone())
        } else {
            Err(Error::ParseError("获取系统时间失败: 空响应".to_string()))
        }
    }
    
    /// 获取系统状态
    pub async fn get_status(&self) -> Result<Vec<SystemStatus>, Error> {
        let path = format!("{}/status", API_PUBLIC_PATH);
        self.client.send_request::<Vec<SystemStatus>>(Method::GET, &path, "").await
    }
    
    /// 获取已有交易产品的规格信息
    pub async fn get_instruments(
        &self, 
        inst_type: &str,
        underlying: Option<&str>,
        inst_id: Option<&str>,
        inst_family: Option<&str>,
    ) -> Result<Vec<InstrumentOkxResDto>, Error> {
        let mut path = format!("{}/instruments?instType={}", API_PUBLIC_PATH, inst_type);
        
        if let Some(uly) = underlying {
            path.push_str(&format!("&uly={}", uly));
        }
        
        if let Some(id) = inst_id {
            path.push_str(&format!("&instId={}", id));
        }
        
        if let Some(family) = inst_family {
            path.push_str(&format!("&instFamily={}", family));
        }
        
        self.client.send_request::<Vec<InstrumentOkxResDto>>(Method::GET, &path, "").await
    }
    
    /// 获取经济日历数据
    pub async fn get_economic_calendar(
        &self,
        region: Option<&str>,
        importance: Option<&str>,
        before: Option<i64>,
        after: Option<i64>,
        limit: Option<i64>,
    ) -> Result<Vec<EconomicEvent>, Error> {
        let mut path = format!("{}/economic-calendar", API_PUBLIC_PATH);
        let mut query_params = vec![];
        
        if let Some(r) = region {
            query_params.push(format!("region={}", r));
        }
        
        if let Some(i) = importance {
            query_params.push(format!("importance={}", i));
        }
        
        if let Some(b) = before {
            query_params.push(format!("before={}", b));
        }
        
        if let Some(a) = after {
            query_params.push(format!("after={}", a));
        }
        
        if let Some(l) = limit {
            query_params.push(format!("limit={}", l));
        }
        
        if !query_params.is_empty() {
            path.push_str(&format!("?{}", query_params.join("&")));
        }
        
        self.client.send_request::<Vec<EconomicEvent>>(Method::GET, &path, "").await
    }
    
    ///, 获取API速率限制
    pub async fn get_rate_limit(
        &self,
        api_key: Option<&str>,
    ) -> Result<Vec<RateLimit>, Error> {
        let mut path = format!("{}/rate-limit", API_PUBLIC_PATH);
        
        if let Some(key) = api_key {
            path.push_str(&format!("?apiKey={}", key));
        }
        
        self.client.send_request::<Vec<RateLimit>>(Method::GET, &path, "").await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[tokio::test]
    async fn test_get_time() {
        let time = OkxPublicData::get_time().await;
        println!("系统时间: {:?}", time);
    }
    
    #[tokio::test]
    async fn test_get_instruments() {
        let public_data = OkxPublicData::from_env().expect("无法从环境变量创建公共数据API");
        let instruments = public_data.get_instruments("SPOT", None, None, None).await;
        println!("交易产品列表: {:?}", instruments);
    }
} 