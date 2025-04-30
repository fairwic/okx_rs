use serde::{Deserialize, Serialize};
use crate::dto::common::MarginMode;

/// 账户余额信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Balance {
    /// 币种
    pub ccy: String,
    /// 币种总额
    #[serde(rename = "bal")]
    pub balance: String,
    /// 可用余额
    #[serde(rename = "availBal")]
    pub available_balance: String,
    /// 冻结余额
    #[serde(rename = "frozenBal")]
    pub frozen_balance: String,
    /// 币种负债额
    #[serde(rename = "liab", skip_serializing_if = "Option::is_none")]
    pub liability: Option<String>,
    /// 币种当前可用保证金
    #[serde(rename = "availEq", skip_serializing_if = "Option::is_none")]
    pub available_equity: Option<String>,
    /// 币种风险价值
    #[serde(rename = "upl", skip_serializing_if = "Option::is_none")]
    pub unrealized_pl: Option<String>,
}

/// 账户配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AccountConfig {
    /// 账户ID
    #[serde(rename = "acctId")]
    pub account_id: String,
    /// 持仓类型
    #[serde(rename = "posMode")]
    pub position_mode: String,
    /// 是否自动借币
    #[serde(rename = "autoLoan")]
    pub auto_loan: bool,
    /// 账户级别
    pub level: String,
    /// 杠杆模式
    #[serde(rename = "mgnMode")]
    pub margin_mode: MarginMode,
}

/// 账户风险数据
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AccountRisk {
    /// 当前风险数据
    pub risk: String,
    /// 风险等级
    #[serde(rename = "riskLvl")]
    pub risk_level: String,
    /// 总权益
    #[serde(rename = "totalEq")]
    pub total_equity: String,
} 