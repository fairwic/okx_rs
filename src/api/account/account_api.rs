use crate::api::API_ACCOUNT_PATH;
use crate::client::OkxClient;
use crate::config::Credentials;
use crate::error::Error;
use crate::dto::account::account_dto::{AccountConfig, AccountRisk, Balance, Position, SetLeverageRequest, TradingSwapNumResponseData};
use reqwest::Method;
use serde_json::json;
use crate::dto::trade::trade_dto::PositionRespDto;
use tokio_tungstenite::tungstenite::client;
use crate::api::api_trait::OkxApiTrait;

/// OKX账户API
/// 提供账户相关的API访问
#[derive(Debug)]
pub struct OkxAccount {
    /// API客户端
    client: OkxClient,
}

impl OkxApiTrait for OkxAccount {
    fn new(client: OkxClient) -> Self {
        OkxAccount { client }
    }
    fn from_env() -> Result<Self,Error> {
        let client = OkxClient::from_env()?;
        Ok(OkxAccount::new(client))
    }
    fn client(&self) -> &OkxClient {
        &self.client
    }
}

impl OkxAccount {
    /// 查询账户余额
    pub async fn get_balance(&self, ccy: Option<&str>) -> Result<Vec<Balance>, Error> {
        let mut path = format!("{}/balance", API_ACCOUNT_PATH);

        if let Some(currency) = ccy {
            path.push_str(&format!("?ccy={}", currency));
        }

        self.client
            .send_request::<Vec<Balance>>(Method::GET, &path, "")
            .await
    }

    /// 查询持仓信息
    pub async fn get_positions(
        &self,
        inst_type: Option<&str>,
        inst_id: Option<&str>,
        pos_id: Option<&str>,
    ) -> Result<Vec<PositionRespDto>, Error> {
        let mut path = format!("{}/positions", API_ACCOUNT_PATH);
        let mut query_params = vec![];

        if let Some(it) = inst_type {
            query_params.push(format!("instType={}", it));
        }

        if let Some(id) = inst_id {
            query_params.push(format!("instId={}", id));
        }

        if let Some(pid) = pos_id {
            query_params.push(format!("posId={}", pid));
        }

        if !query_params.is_empty() {
            path.push_str(&format!("?{}", query_params.join("&")));
        }

        self.client
            .send_request::<Vec<PositionRespDto>>(Method::GET, &path, "")
            .await
    }

    /// 查询账户配置
    pub async fn get_config(&self) -> Result<Vec<AccountConfig>, Error> {
        let path = format!("{}/config", API_ACCOUNT_PATH);
        self.client
            .send_request::<Vec<AccountConfig>>(Method::GET, &path, "")
            .await
    }

    /// 设置杠杆倍数
    pub async fn set_leverage(
        &self,
        params: SetLeverageRequest
    ) -> Result<serde_json::Value, Error> {
        let path = format!("{}/set-leverage", API_ACCOUNT_PATH);

        let body_str = serde_json::to_string(&params).map_err(Error::JsonError)?;
        self.client
            .send_request::<serde_json::Value>(Method::POST, &path, &body_str)
            .await
    }

    /// 获取最大可交易数量
    pub async fn get_max_size(
        &self,
        inst_id: &str,
        td_mode: &str,
        ccy: Option<&str>,
        px: Option<&str>,
        leverage: Option<&str>,
    ) -> Result<TradingSwapNumResponseData, Error> {
        let mut path = format!(
            "{}/max-size?instId={}&tdMode={}",
            API_ACCOUNT_PATH, inst_id, td_mode
        );

        if let Some(currency) = ccy {
            path.push_str(&format!("&ccy={}", currency));
        }

        if let Some(price) = px {
            path.push_str(&format!("&px={}", price));
        }

        if let Some(lev) = leverage {
            path.push_str(&format!("&leverage={}", lev));
        }

        self.client
            .send_request::<TradingSwapNumResponseData>(Method::GET, &path, "")
            .await
    }

    /// 获取账户风险状态
    pub async fn get_account_risk(&self) -> Result<Vec<AccountRisk>, Error> {
        let path = format!("{}/account-risk", API_ACCOUNT_PATH);
        self.client
            .send_request::<Vec<AccountRisk>>(Method::GET, &path, "")
            .await
    }

    /// 获取账户账单
    pub async fn get_bills(
        &self,
        inst_type: Option<&str>,
        ccy: Option<&str>,
        margin_mode: Option<&str>,
        typ: Option<&str>,
        start_time: Option<&str>,
        end_time: Option<&str>,
        limit: Option<u32>,
    ) -> Result<serde_json::Value, Error> {
        let mut path = format!("{}/bills", API_ACCOUNT_PATH);
        let mut query_params = vec![];

        if let Some(it) = inst_type {
            query_params.push(format!("instType={}", it));
        }

        if let Some(currency) = ccy {
            query_params.push(format!("ccy={}", currency));
        }

        if let Some(mode) = margin_mode {
            query_params.push(format!("mgnMode={}", mode));
        }

        if let Some(t) = typ {
            query_params.push(format!("type={}", t));
        }

        if let Some(st) = start_time {
            query_params.push(format!("begin={}", st));
        }

        if let Some(et) = end_time {
            query_params.push(format!("end={}", et));
        }

        if let Some(l) = limit {
            query_params.push(format!("limit={}", l));
        }

        if !query_params.is_empty() {
            path.push_str(&format!("?{}", query_params.join("&")));
        }

        self.client
            .send_request::<serde_json::Value>(Method::GET, &path, "")
            .await
    }
    
    /// 获取账户持仓信息
    pub async fn get_account_positions(
        &self,
        inst_type: Option<&str>,
        inst_id: Option<&str>,
        pos_id: Option<&str>,
    ) -> Result<Vec<Position>, Error> {
        let mut path = format!("{}/positions", API_ACCOUNT_PATH);
        let mut query_params = vec![];

        if let Some(it) = inst_type {
            query_params.push(format!("instType={}", it));
        }

        if let Some(id) = inst_id {
            query_params.push(format!("instId={}", id));
        }

        if let Some(pid) = pos_id {
            query_params.push(format!("posId={}", pid));
        }

        if !query_params.is_empty() {
            path.push_str(&format!("?{}", query_params.join("&")));
        }

        self.client
            .send_request::<Vec<Position>>(Method::GET, &path, "")
            .await
    }
} 
#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::init_env;


    #[tokio::test]
    async fn test_get_balance() {
        init_env();
        let client = OkxAccount::from_env().unwrap();
        let balance = client.get_balance(None).await.unwrap();
        println!("{:?}", balance);
    }
    #[tokio::test]
    async fn test_get_positions() {
        init_env();
        let client = OkxAccount::from_env().unwrap();
        let positions = client.get_positions(None, None, None).await.unwrap();
        println!("{:?}", positions);
    }
}
