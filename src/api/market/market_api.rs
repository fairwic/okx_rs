use crate::api::api_trait::OkxApiTrait;
use crate::api::API_MARKET_PATH;
use crate::client::OkxClient;
use crate::dto::market::market_dto::{
    CandleOkxRespDto, Depth, InstrumentOkxResDto, TickerOkxResDto,
};
use crate::error::Error;
use log::debug;
use reqwest::Method;

/// OKX市场数据API
/// 提供市场行情相关的API访问
#[derive(Debug)]
pub struct OkxMarket {
    /// API客户端
    client: OkxClient,
}

impl OkxApiTrait for OkxMarket {
    /// 创建一个新的OkxMarket实例
    fn new(client: OkxClient) -> Self {
        Self { client }
    }
    /// 从环境变量创建一个新的OkxMarket实例
    fn from_env() -> Result<Self, Error> {
        let client = OkxClient::from_env()?;
        Ok(Self { client })
    }
    /// 获取内部客户端引用
    fn client(&self) -> &OkxClient {
        &self.client
    }
}

impl OkxMarket {
    /// 获取单个产品行情信息
    pub async fn get_ticker(&self, inst_id: &str) -> Result<Vec<TickerOkxResDto>, Error> {
        let path = format!("{}/ticker?instId={}", API_MARKET_PATH, inst_id);
        let tickers = self
            .client
            .send_request::<Vec<TickerOkxResDto>>(Method::GET, &path, "")
            .await?;
        Ok(tickers)
        // tickers.into_iter().next()
        //     .ok_or_else(|| Error::ParseError("获取行情数据失败: 空响应".to_string()))
    }

    /// 获取多个产品行情信息
    pub async fn get_tickers(&self, inst_type: &str) -> Result<Vec<TickerOkxResDto>, Error> {
        let path = format!("{}/tickers?instType={}", API_MARKET_PATH, inst_type);
        self.client
            .send_request::<Vec<TickerOkxResDto>>(Method::GET, &path, "")
            .await
    }

    /// 获取指数行情
    pub async fn get_index_tickers(
        &self,
        quot_ccy: Option<&str>,
        inst_id: Option<&str>,
    ) -> Result<Vec<TickerOkxResDto>, Error> {
        let mut path = format!("{}/index-tickers", API_MARKET_PATH);
        let mut query_params = vec![];

        if let Some(ccy) = quot_ccy {
            query_params.push(format!("quotCcy={}", ccy));
        }

        if let Some(id) = inst_id {
            query_params.push(format!("instId={}", id));
        }

        if !query_params.is_empty() {
            path.push_str(&format!("?{}", query_params.join("&")));
        }

        self.client
            .send_request::<Vec<TickerOkxResDto>>(Method::GET, &path, "")
            .await
    }

    /// 获取K线数据。K线数据按请求的粒度分组返回，K线数据每个粒度最多可获取最近1,440条
    /// 限速：40次/2s
    // 限速规则：IP
    pub async fn get_candles(
        &self,
        inst_id: &str,
        bar: &str,
        after: Option<&str>,
        before: Option<&str>,
        limit: Option<&str>,
    ) -> Result<Vec<CandleOkxRespDto>, Error> {
        let mut path = format!("{}/candles?instId={}", API_MARKET_PATH, inst_id);

        path.push_str(&format!("&bar={}", bar));
        // }

        if let Some(a) = after {
            path.push_str(&format!("&after={}", a));
        }

        if let Some(b) = before {
            path.push_str(&format!("&before={}", b));
        }

        if let Some(l) = limit {
            path.push_str(&format!("&limit={}", l));
        }

        self.client
            .send_request::<Vec<CandleOkxRespDto>>(Method::GET, &path, "")
            .await
    }

    // 获取最近几年的历史k线数据(1s k线支持查询最近3个月的数据)
    // 限速：20次/2s
    // 限速规则：IP
    pub async fn get_history_candles(
        &self,
        inst_id: &str,
        bar: &str,
        after: Option<&str>,
        before: Option<&str>,
        limit: Option<&str>,
    ) -> Result<Vec<CandleOkxRespDto>, Error> {
        let mut path = format!("{}/history-candles?instId={}", API_MARKET_PATH, inst_id);

        path.push_str(&format!("&bar={}", bar));

        if let Some(a) = after {
            path.push_str(&format!("&after={}", a));
        }

        if let Some(b) = before {
            path.push_str(&format!("&before={}", b));
        }

        if let Some(l) = limit {
            path.push_str(&format!("&limit={}", l));
        }
        debug!("OKX path: {}", path);
        let res: Vec<Vec<String>> = self
            .client
            .send_request::<Vec<Vec<String>>>(Method::GET, &path, "")
            .await?;
        let candles = res
            .into_iter()
            .map(|v| CandleOkxRespDto::from_vec(v))
            .collect();
        Ok(candles)
    }

    /// 获取交易产品深度
    pub async fn get_books(&self, inst_id: &str, sz: Option<u32>) -> Result<Depth, Error> {
        let mut path = format!("{}/books?instId={}", API_MARKET_PATH, inst_id);

        if let Some(s) = sz {
            path.push_str(&format!("&sz={}", s));
        }

        let depths = self
            .client
            .send_request::<Vec<Depth>>(Method::GET, &path, "")
            .await?;

        depths
            .into_iter()
            .next()
            .ok_or_else(|| Error::ParseError("获取深度数据失败: 空响应".to_string()))
    }

    /// 获取产品列表
    pub async fn get_instruments(
        &self,
        inst_type: &str,
        uly: Option<&str>,
        inst_id: Option<&str>,
    ) -> Result<Vec<InstrumentOkxResDto>, Error> {
        let mut path = format!("{}/instruments?instType={}", API_MARKET_PATH, inst_type);

        if let Some(u) = uly {
            path.push_str(&format!("&uly={}", u));
        }

        if let Some(id) = inst_id {
            path.push_str(&format!("&instId={}", id));
        }

        self.client
            .send_request::<Vec<InstrumentOkxResDto>>(Method::GET, &path, "")
            .await
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_get_ticker() {
        let market = OkxMarket::from_env().expect("无法从环境变量创建市场API");
        let ticker = market.get_ticker("BTC-USDT").await;

        println!("Ticker result: {:?}", ticker);
    }

    #[tokio::test]
    async fn test_get_candles() {
        let market = OkxMarket::from_env().expect("无法从环境变量创建市场API");
        let candles = market
            .get_candles("BTC-USDT", "1D", None, None, Some("10"))
            .await;

        println!("Candles result: {:?}", candles);
    }
}
