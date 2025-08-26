use crate::enums::account_enums::AccountType;
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

// 请求参数
// 参数名	类型	是否必须	描述
// type	String	否	划转类型
//   0：账户内划转
//   1：母账户转子账户(仅适用于母账户APIKey)
//   2：子账户转母账户(仅适用于母账户APIKey)
//   3：子账户转母账户(仅适用于子账户APIKey)
//   4：子账户转子账户(仅适用于子账户APIKey，且目标账户需要是同一母账户下的其他子账户。子账户主动转出权限默认是关闭的，权限调整参考 设置子账户主动转出权限。)
// 默认是0
// 如果您希望通过母账户API Key控制子账户之间的划转，参考接口 子账户间资金划转

// ccy	String	是	划转币种，如 USDT
// amt	String	是	划转数量
// from	String	是	转出账户
//    6：资金账户
//    18：交易账户
// to	String	是	转入账户
//     6：资金账户
//     18：交易账户
// subAcct	String	可选	子账户名称
// 当type为1/2/4时，该字段必填
// loanTrans	Boolean	否	是否支持现货模式/跨币种保证金模式/组合保证金模式下的借币转出
// true：支持借币转出
// false：不支持借币转出
// 默认为false
// omitPosRisk	String	否	是否忽略仓位风险
// 默认为false
// 仅适用于组合保证金模式
// clientId	String	否	客户自定义ID
// 字母（区分大小写）与数字的组合，可以是纯字母、纯数字且长度要在1-32位之间。
///资金划转
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransferOkxReqDto {
    /// 划转类型
    #[serde(rename = "type")]
    pub transfer_type: Option<String>,
    /// 币种
    pub ccy: String,
    /// 划转数量
    pub amt: String,
    /// 转出账户
    pub from: AccountType,
    /// 转入账户
    pub to: AccountType,
    /// 子账户名称
    #[serde(rename = "subAcct")]
    pub sub_acct: Option<String>,
    // /// 是否支持借币转出
    // #[serde(rename = "loanTrans")]
    // pub loan_trans: bool,
    // /// 是否忽略仓位风险
    // #[serde(rename = "omitPosRisk")]
    // pub omit_pos_risk: bool,
    // /// 客户自定义ID
    // #[serde(rename = "clientId")]
    // pub client_id: String,
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
