use crate::client::OkxClient;
use crate::error::Error;
use crate::dto::asset_model::{AssetBalance, TransferRecord, WithdrawalRecord, DepositRecord};
use reqwest::Method;
use serde_json::json;
use crate::api::API_ASSET_PATH;

/// OKX资产API
/// 提供资产相关的API访问
#[derive(Debug)]
pub struct OkxAsset {
    /// API客户端
    client: OkxClient,
}

impl OkxAsset {
    /// 创建一个新的OkxAsset实例
    pub fn new(client: OkxClient) -> Self {
        Self { client }
    }
    
    /// 从环境变量创建一个新的OkxAsset实例
    pub fn from_env() -> Result<Self, Error> {
        let client = OkxClient::from_env()?;
        Ok(Self { client })
    }
    
    /// 获取内部客户端引用
    pub fn client(&self) -> &OkxClient {
        &self.client
    }

    /// 获取资产余额
    pub async fn get_balances(
        &self,
        ccy: Option<&str>,
    ) -> Result<Vec<AssetBalance>, Error> {
        let mut path = format!("{}/balances", API_ASSET_PATH);
        
        if let Some(currency) = ccy {
            path.push_str(&format!("?ccy={}", currency));
        }
        
        self.client.send_request::<Vec<AssetBalance>>(Method::GET, &path, "").await
    }
    
    /// 获取资金划转状态
    pub async fn get_transfer_state(
        &self,
        trans_id: &str,
        type_param: Option<&str>,
    ) -> Result<Vec<TransferRecord>, Error> {
        let mut path = format!("{}/transfer-state?transId={}", API_ASSET_PATH, trans_id);
        
        if let Some(t) = type_param {
            path.push_str(&format!("&type={}", t));
        }
        
        self.client.send_request::<Vec<TransferRecord>>(Method::GET, &path, "").await
    }
    
    /// 资金划转
    pub async fn transfer(
        &self,
        ccy: &str,
        amt: &str,
        from: &str,
        to: &str,
        sub_acct: Option<&str>,
        inst_id: Option<&str>,
        to_inst_id: Option<&str>,
    ) -> Result<serde_json::Value, Error> {
        let path = format!("{}/transfer", API_ASSET_PATH);
        
        let mut body = json!({
            "ccy": ccy,
            "amt": amt,
            "from": from,
            "to": to,
        });
        
        if let Some(account) = sub_acct {
            body["subAcct"] = json!(account);
        }
        
        if let Some(id) = inst_id {
            body["instId"] = json!(id);
        }
        
        if let Some(to_id) = to_inst_id {
            body["toInstId"] = json!(to_id);
        }
        
        let body_str = serde_json::to_string(&body).map_err(Error::JsonError)?;
        self.client.send_request::<serde_json::Value>(Method::POST, &path, &body_str).await
    }
    
    /// 提币
    pub async fn withdrawal(
        &self,
        ccy: &str,
        amt: &str,
        dest: &str,
        to_addr: &str,
        fee: &str,
        chain: Option<&str>,
        areal_name: Option<&str>,
    ) -> Result<serde_json::Value, Error> {
        let path = format!("{}/withdrawal", API_ASSET_PATH);
        
        let mut body = json!({
            "ccy": ccy,
            "amt": amt,
            "dest": dest,
            "toAddr": to_addr,
            "fee": fee,
        });
        
        if let Some(c) = chain {
            body["chain"] = json!(c);
        }
        
        if let Some(name) = areal_name {
            body["arealName"] = json!(name);
        }
        
        let body_str = serde_json::to_string(&body).map_err(Error::JsonError)?;
        self.client.send_request::<serde_json::Value>(Method::POST, &path, &body_str).await
    }
    
    /// 获取提币历史
    pub async fn get_withdrawal_history(
        &self,
        ccy: Option<&str>,
        tx_id: Option<&str>,
        state: Option<&str>,
        after: Option<&str>,
        before: Option<&str>,
        limit: Option<u32>,
    ) -> Result<Vec<WithdrawalRecord>, Error> {
        let mut path = format!("{}/withdrawal-history", API_ASSET_PATH);
        let mut query_params = vec![];
        
        if let Some(currency) = ccy {
            query_params.push(format!("ccy={}", currency));
        }
        
        if let Some(id) = tx_id {
            query_params.push(format!("txId={}", id));
        }
        
        if let Some(s) = state {
            query_params.push(format!("state={}", s));
        }
        
        if let Some(a) = after {
            query_params.push(format!("after={}", a));
        }
        
        if let Some(b) = before {
            query_params.push(format!("before={}", b));
        }
        
        if let Some(l) = limit {
            query_params.push(format!("limit={}", l));
        }
        
        if !query_params.is_empty() {
            path.push_str(&format!("?{}", query_params.join("&")));
        }
        
        self.client.send_request::<Vec<WithdrawalRecord>>(Method::GET, &path, "").await
    }
    
    /// 获取充值历史
    pub async fn get_deposit_history(
        &self,
        ccy: Option<&str>,
        tx_id: Option<&str>,
        state: Option<&str>,
        after: Option<&str>,
        before: Option<&str>,
        limit: Option<u32>,
    ) -> Result<Vec<DepositRecord>, Error> {
        let mut path = format!("{}/deposit-history", API_ASSET_PATH);
        let mut query_params = vec![];
        
        if let Some(currency) = ccy {
            query_params.push(format!("ccy={}", currency));
        }
        
        if let Some(id) = tx_id {
            query_params.push(format!("txId={}", id));
        }
        
        if let Some(s) = state {
            query_params.push(format!("state={}", s));
        }
        
        if let Some(a) = after {
            query_params.push(format!("after={}", a));
        }
        
        if let Some(b) = before {
            query_params.push(format!("before={}", b));
        }
        
        if let Some(l) = limit {
            query_params.push(format!("limit={}", l));
        }
        
        if !query_params.is_empty() {
            path.push_str(&format!("?{}", query_params.join("&")));
        }
        
        self.client.send_request::<Vec<DepositRecord>>(Method::GET, &path, "").await
    }
    
    /// 获取充值地址
    pub async fn get_deposit_address(
        &self,
        ccy: &str,
    ) -> Result<serde_json::Value, Error> {
        let path = format!("{}/deposit-address?ccy={}", API_ASSET_PATH, ccy);
        self.client.send_request::<serde_json::Value>(Method::GET, &path, "").await
    }
    
    /// 获取币种列表
    pub async fn get_currencies(&self) -> Result<serde_json::Value, Error> {
        let path = format!("{}/currencies", API_ASSET_PATH);
        self.client.send_request::<serde_json::Value>(Method::GET, &path, "").await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[tokio::test]
    async fn test_get_balances() {
        let asset = OkxAsset::from_env().expect("无法从环境变量创建资产API");
        let balances = asset.get_balances(None).await;
        
        println!("资产余额: {:?}", balances);
    }
} 