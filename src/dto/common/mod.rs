use serde::{Deserialize, Serialize};

/// 枚举转字符串
pub trait EnumToStrTrait {
    fn as_str(&self) -> &'static str;
}

/// 交易对信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InstrumentInfo {
    /// 产品类型
    #[serde(rename = "instType")]
    pub inst_type: String,
    /// 交易品种ID
    #[serde(rename = "instId")]
    pub inst_id: String,
    /// 标的指数
    #[serde(rename = "uly", skip_serializing_if = "Option::is_none")]
    pub underlying: Option<String>,
    /// 盈亏结算和保证金币种
    #[serde(rename = "settleCcy", skip_serializing_if = "Option::is_none")]
    pub settle_currency: Option<String>,
    /// 合约面值
    #[serde(rename = "ctVal", skip_serializing_if = "Option::is_none")]
    pub contract_value: Option<String>,
    /// 合约乘数
    #[serde(rename = "ctMult", skip_serializing_if = "Option::is_none")]
    pub contract_multiplier: Option<String>,
    /// 报价货币
    #[serde(rename = "quoteCcy", skip_serializing_if = "Option::is_none")]
    pub quote_currency: Option<String>,
    /// 交易货币币种
    #[serde(rename = "baseCcy", skip_serializing_if = "Option::is_none")]
    pub base_currency: Option<String>,
}

/// 方向枚举
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum Side {
    #[serde(rename = "buy")]
    Buy,
    #[serde(rename = "sell")]
    Sell,
}
/// 方向枚举转字符串
impl EnumToStrTrait for Side {
    fn as_str(&self) -> &'static str {
        match self {
            Side::Buy => "buy",
            Side::Sell => "sell",
        }
    }
}

/// 持仓方向枚举
///
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum PositionSide {
    #[serde(rename = "long")]
    Long,
    #[serde(rename = "short")]
    Short,
    #[serde(rename = "net")]
    Net, //买卖模式（交割/永续/期权：pos为正代表开多，pos为负代表开空。币币杠杆时，pos均为正，posCcy为交易货币时，代表开多；posCcy为计价货币时，代表开空。
}

/// 持仓方向枚举转字符串
impl EnumToStrTrait for PositionSide {
    fn as_str(&self) -> &'static str {
        match self {
            PositionSide::Long => "long",
            PositionSide::Short => "short",
            PositionSide::Net => "net",
        }
    }
}

/// 订单类型枚举
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum OrderType {
    #[serde(rename = "market")]
    Market,
    #[serde(rename = "limit")]
    Limit,
    #[serde(rename = "post_only")]
    PostOnly,
    #[serde(rename = "fok")]
    FillOrKill,
    #[serde(rename = "ioc")]
    ImmediateOrCancel,
    #[serde(rename = "optimal_limit_ioc")]
    OptimalLimitIoc,
}

/// 订单状态枚举
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum OrderState {
    #[serde(rename = "canceled")]
    Canceled,
    #[serde(rename = "live")]
    Live,
    #[serde(rename = "partially_filled")]
    PartiallyFilled,
    #[serde(rename = "filled")]
    Filled,
}

/// 杠杆方式
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum MarginMode {
    #[serde(rename = "isolated")]
    Isolated,
    #[serde(rename = "cross")]
    Cross,
}

/// 产品类型
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum InstrumentType {
    #[serde(rename = "SPOT")]
    Spot,
    #[serde(rename = "MARGIN")]
    Margin,
    #[serde(rename = "SWAP")]
    Swap,
    #[serde(rename = "FUTURES")]
    Futures,
    #[serde(rename = "OPTION")]
    Option,
}

/// 分页信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PaginationInfo {
    /// 此次分页的最后一条数据的ID
    #[serde(rename = "after", skip_serializing_if = "Option::is_none")]
    pub after: Option<String>,
    /// 此次分页的第一条数据的ID
    #[serde(rename = "before", skip_serializing_if = "Option::is_none")]
    pub before: Option<String>,
    /// 请求的数据条数
    #[serde(rename = "limit", skip_serializing_if = "Option::is_none")]
    pub limit: Option<String>,
}
