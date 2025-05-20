use serde::{Deserialize, Serialize};

/// 账户类型
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AccountType {
    /// 资金账户
    /// 
    #[serde(rename = "6")]
    FOUND,
    /// 交易账户
    #[serde(rename = "18")]
    TRADE,
}

impl AccountType {
    pub fn to_string(&self) -> &str {
        match self {
            AccountType::FOUND => "6",
            AccountType::TRADE => "18",
        }
    }
}
