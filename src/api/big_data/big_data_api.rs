use reqwest::Method;

use crate::client::{OkxClient, OkxApiResponse};
use crate::Error;
use crate::dto::big_data::*;
use log::debug;
use crate::api::api_trait::OkxApi;

pub struct OkxBigData {
    client: OkxClient,
}

impl OkxApi for OkxBigData {
    fn new(client: OkxClient) -> Self {
        OkxBigData { client }
    }
    fn from_env() -> Result<Self,Error> {
        let client = OkxClient::from_env()?;
        Ok(OkxBigData::new(client))
    }
    fn client(&self) -> &OkxClient {
        &self.client
    }
}

impl OkxBigData {
    //获取交易大数据支持币种
    pub async fn get_support_coin(&self) -> Result<OkxApiResponse<SupportCoin>,Error> {
        // 币种，如 BTC
        // 支持多币种查询（不超过20个），币种之间半角逗号分隔
        let path = "/api/v5/rubik/stat/trading-data/support-coin?".to_string();
        debug!("path:{:#?}", path);
        self.client
            .send_request(Method::GET, &path, "")
            .await
    }

    // 获取主动买入/卖出情况
    //限速：5次/2s
    // 限速规则：IP
    // 权限：读取
    // 时间粒度，默认值5m。支持[5m/1H/1D]
    // 5m粒度最多只能查询两天之内的数据
    // 1H粒度最多只能查询30天之内的数据
    // 1D粒度最多只能查询180天之内的数据
    pub async fn get_taker_volume(
        &self,
        ccy: &str,
        inst_type: &str,
        begin: Option<&str>,
        end: Option<&str>,
        period: Option<&str>,
    ) -> Result<Vec<Vec<String>>,Error> {
        let mut path = format!(
            "/api/v5/rubik/stat/taker-volume?ccy={}&instType={}",
            ccy, inst_type
        );

        if let Some(begin_time) = begin {
            path.push_str(&format!("&begin={}", begin_time));
        }
        if let Some(end_time) = end {
            path.push_str(&format!("&end={}", end_time));
        }
        if let Some(period_value) = period {
            path.push_str(&format!("&period={}", period_value));
        }

        debug!("path: {:#?}", path);

        self.client
            .send_request(Method::GET, &path, "")
            .await
    }

    // 获取合约主动买入/卖出情况
    // 限速： 5次/2s
    // 限速规则： IP + instrumentID
    // 权限：读取
    // 分页返回的结果集数量，最大为100，不填默认返回100条
    //对于时间粒度period=1D，数据时间范围最早至2024年1月1日；对于其他时间粒度period，最早至2024年2月初。
    pub async fn get_taker_volume_contract(
        &self,
        inst_id: &str,
        period: Option<&str>,
        unit: Option<&str>,
        begin: Option<&str>,
        end: Option<&str>,
        limit: Option<&str>,
    ) -> Result<Vec<Vec<String>>,Error> {
        let mut path = format!(
            "/api/v5/rubik/stat/taker-volume-contract?instId={}",
            inst_id
        );
        if let Some(period_value) = period {
            path.push_str(&format!("&period={}", period_value));
        }
        if let Some(unit_value) = unit {
            path.push_str(&format!("&unit={}", unit_value));
        }
        if let Some(begin_time) = begin {
            path.push_str(&format!("&begin={}", begin_time));
        }
        if let Some(end_time) = end {
            path.push_str(&format!("&end={}", end_time));
        }
        if let Some(limit_value) = limit {
            path.push_str(&format!("&limit={}", limit_value));
        }

        debug!("path: {:#?}", path);

        self.client
            .send_request(Method::GET, &path, "")
            .await
    }

    //获取精英交易员合约多空持仓人数比
    // 获取精英交易员交割永续净开多持仓用户数与净开空持仓用户数的比值。精英交易员指持仓价值前5%的用户。每个粒度最多可获取最近1,440条数据。数据时间范围最早至2024年3月22日。
    // 限速： 5次/2s
    // 限速规则： IP + instrumentID
    // 权限：读取
    pub async fn get_long_short_account_ratio_contract_top_trader(
        &self,
        inst_id: &str,
        period: Option<&str>,
        begin: Option<&str>,
        end: Option<&str>,
        limit: Option<&str>,
    ) -> Result<Vec<Vec<String>>,Error> {
        let mut path = format!(
            "/api/v5/rubik/stat/contracts/long-short-account-ratio-contract-top-trader?instId={}",
            inst_id
        );

        if let Some(period_value) = period {
            path.push_str(&format!("&period={}", period_value));
        }
        if let Some(begin_time) = begin {
            path.push_str(&format!("&begin={}", begin_time));
        }
        if let Some(end_time) = end {
            path.push_str(&format!("&end={}", end_time));
        }
        if let Some(limit_value) = limit {
            path.push_str(&format!("&limit={}", limit_value));
        }

        debug!("path: {:#?}", path);

        self.client
            .send_request(Method::GET, &path, "")
            .await
    }

    //获取精英交易员合约多空持仓仓位比
    // 获取交割永续开多、开空仓位占总持仓的比值。精英交易员指持仓价值前5%的用户。每个粒度最多可获取最近1,440条数据。数据时间范围最早至2024年3月22日。
    //
    // 限速： 5次/2s
    // 限速规则： IP + instrumentID
    // 权限：读取
    pub async fn get_long_short_postion_ratio_contract_top_trader(
        &self,
        inst_id: &str,
        period: Option<&str>,
        begin: Option<&str>,
        end: Option<&str>,
        limit: Option<&str>,
    ) -> Result<Vec<Vec<String>>,Error> {
        let mut path = format!(
            "/api/v5/rubik/stat/contracts/long-short-account-ratio-contract-top-trader?instId={}",
            inst_id
        );

        if let Some(period_value) = period {
            path.push_str(&format!("&period={}", period_value));
        }
        if let Some(begin_time) = begin {
            path.push_str(&format!("&begin={}", begin_time));
        }
        if let Some(end_time) = end {
            path.push_str(&format!("&end={}", end_time));
        }
        if let Some(limit_value) = limit {
            path.push_str(&format!("&limit={}", limit_value));
        }

        debug!("path: {:#?}", path);

        self.client
            .send_request(Method::GET, &path, "")
            .await
    }
} 