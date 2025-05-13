use serde::{Deserialize, Serialize};

/// 资产余额信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AssetBalance {
    /// 币种
    pub ccy: String,
    /// 币种余额
    pub bal: String,
    /// 冻结余额
    #[serde(rename = "frozenBal")]
    pub frozen_bal: String,
    /// 可用余额
    #[serde(rename = "availBal")]
    pub avail_bal: String,
}

/// 资金划转记录
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransferRecord {
    /// 划转ID
    #[serde(rename = "transId")]
    pub transfer_id: String,
    /// 币种
    pub ccy: String,
    /// 划转数量
    pub amt: String,
    /// 转入账户
    pub from: String,
    /// 转出账户
    pub to: String,
    /// 划转状态
    pub state: String,
    /// 划转时间
    pub ts: String,
}

/// 提币记录
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WithdrawalRecord {
    /// 提币申请ID
    pub wdId: String,
    /// 币种
    pub ccy: String,
    /// 链信息
    pub chain: String,
    /// 提币数量
    pub amt: String,
    /// 提币地址
    pub to: String,
    /// 提币申请状态
    pub state: String,
    /// 提币时间
    pub ts: String,
    /// 提币手续费
    pub fee: String,
}

/// 充值记录
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DepositRecord {
    /// 充值记录ID
    pub depId: String,
    /// 币种
    pub ccy: String,
    /// 链信息
    pub chain: String,
    /// 充值数量
    pub amt: String,
    /// 充值地址
    pub to: String,
    /// 充值状态
    pub state: String,
    /// 充值时间
    pub ts: String,
} 