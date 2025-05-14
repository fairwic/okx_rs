use reqwest::Method;
use serde::{Deserialize, Serialize};
use crate::{Error, OkxClient};
use crate::config::Credentials;


#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct VolumeData {
    pub ts: String,
    pub oi: String,  //持仓总量（USD
    pub vol: String, //交易总量（USD）
}


pub struct OkxContracts {
    client: OkxClient,
}

impl OkxContracts {
    /// 创建一个新的OkxAccount实例，使用给定的客户端
    pub fn new() -> Self {
        Self {
            // 创建客户端
            client: OkxClient::new(Credentials::new("api_key", "api_secret", "passphrase", "0"))
                .unwrap(),
        }
    }

    /// 从环境变量创建一个新的OkxAccount实例
    pub fn from_env() -> Result<Self, Error> {
        let client = OkxClient::from_env()?;
        Ok(Self { client })
    }

    /// 获取内部客户端引用
    pub fn client(&self) -> &OkxClient {
        &self.client
    }
    //获取未平仓合约的持仓量和交易总量
    pub async fn get_open_interest_volume(
        &self,
        ccy: Option<&str>,
        begin: Option<i64>,
        end: Option<i64>,
        period: Option<&str>,
    ) -> anyhow::Result<Vec<VolumeData>, anyhow::Error> {
        let mut path = "/api/v5/rubik/stat/contracts/open-interest-volume?".to_string();
        if let Some(ccy) = ccy {
            path.push_str(&format!("&ccy={}", ccy));
        }

        if let Some(begin) = begin {
            path.push_str(&format!("&begin={}", begin));
        }
        if let Some(end) = end {
            path.push_str(&format!("&end={}", end));
        }

        if let Some(period) = period {
            path.push_str(&format!("&period={}", period));
        }

        let res=self.client
            .send_request(Method::GET, &path, "")
            .await?;
        Ok(res)
    }
}
