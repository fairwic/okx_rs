use serde::{Deserialize, Serialize};
use crate::dto::common::{OrderType, Side, PositionSide, MarginMode};

/// 订单信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Order {
    /// 产品类型
    #[serde(rename = "instType")]
    pub inst_type: String,
    /// 产品ID
    #[serde(rename = "instId")]
    pub inst_id: String,
    /// 杠杆倍数
    #[serde(rename = "lever")]
    pub leverage: String,
    /// 委托价格
    pub px: String,
    /// 委托数量
    pub sz: String,
    /// 订单ID
    #[serde(rename = "ordId")]
    pub order_id: String,
    /// 客户自定义订单ID
    #[serde(rename = "clOrdId", skip_serializing_if = "Option::is_none")]
    pub client_order_id: Option<String>,
    /// 成交数量
    #[serde(rename = "fillSz", skip_serializing_if = "Option::is_none")]
    pub filled_size: Option<String>,
    /// 最新成交价格
    #[serde(rename = "fillPx", skip_serializing_if = "Option::is_none")]
    pub filled_price: Option<String>,
    /// 成交时间
    #[serde(rename = "fillTime", skip_serializing_if = "Option::is_none")]
    pub filled_time: Option<String>,
    /// 订单类型
    #[serde(rename = "ordType")]
    pub order_type: OrderType,
    /// 订单方向
    pub side: Side,
    /// 持仓方向
    #[serde(rename = "posSide", skip_serializing_if = "Option::is_none")]
    pub position_side: Option<PositionSide>,
    /// 订单状态
    pub state: String,
    /// 订单创建时间
    #[serde(rename = "cTime")]
    pub creation_time: String,
    /// 订单更新时间
    #[serde(rename = "uTime", skip_serializing_if = "Option::is_none")]
    pub update_time: Option<String>,
}

/// 仓位信息响应DTO
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PositionRespDto {
    /// 产品类型
    #[serde(rename = "instType")]
    pub inst_type: String,
    /// 产品ID
    #[serde(rename = "instId")]
    pub inst_id: String,
    /// 杠杆倍数
    #[serde(rename = "lever")]
    pub leverage: String,
    /// 持仓数量
    pub pos: String,
    /// 持仓方向
    #[serde(rename = "posSide")]
    pub position_side: PositionSide,
    /// 开仓平均价
    #[serde(rename = "avgPx")]
    pub average_price: String,
    /// 未实现收益
    pub upl: String,
    /// 仓位占用保证金
    #[serde(rename = "margin")]
    pub margin: String,
    /// 杠杆模式
    #[serde(rename = "mgnMode")]
    pub margin_mode: MarginMode,
    /// 预估强平价
    #[serde(rename = "liqPx", skip_serializing_if = "Option::is_none")]
    pub liquidation_price: Option<String>,
}

/// 获取持仓信息请求DTO
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetPositionReqDto {
    /// 产品类型
    #[serde(rename = "instType", skip_serializing_if = "Option::is_none")]
    pub inst_type: Option<String>,
    /// 产品ID
    #[serde(rename = "instId", skip_serializing_if = "Option::is_none")]
    pub inst_id: Option<String>,
    /// 持仓ID
    #[serde(rename = "posId", skip_serializing_if = "Option::is_none")]
    pub pos_id: Option<String>,
}

/// 设置杠杆倍数请求DTO
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SetLeverageReqDto {
    /// 产品ID
    #[serde(rename = "instId")]
    pub inst_id: String,
    /// 杠杆倍数
    #[serde(rename = "lever")]
    pub leverage: String,
    /// 保证金模式 cross: 全仓, isolated: 逐仓
    #[serde(rename = "mgnMode")]
    pub margin_mode: String,
    /// 持仓方向，仅适用于币币杠杆逐仓和交割/永续逐仓
    #[serde(rename = "posSide", skip_serializing_if = "Option::is_none")]
    pub position_side: Option<String>,
}

/// 设置杠杆倍数响应DTO
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SetLeverageRespDto {
    /// 杠杆倍数
    #[serde(rename = "lever")]
    pub leverage: String,
    /// 保证金模式
    #[serde(rename = "mgnMode")]
    pub margin_mode: String,
    /// 产品ID
    #[serde(rename = "instId")]
    pub inst_id: String,
    /// 持仓方向
    #[serde(rename = "posSide")]
    pub position_side: String,
}

/// 交易手续费率
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FeeRate {
    /// 产品类型
    #[serde(rename = "instType")]
    pub inst_type: String,
    /// 币对/合约
    #[serde(rename = "instId", skip_serializing_if = "Option::is_none")]
    pub inst_id: Option<String>,
    /// 标的指数
    #[serde(rename = "uly", skip_serializing_if = "Option::is_none")]
    pub underlying: Option<String>,
    /// 币种
    #[serde(rename = "ccy", skip_serializing_if = "Option::is_none")]
    pub currency: Option<String>,
    /// maker手续费率
    #[serde(rename = "makerFeeRate")]
    pub maker_fee_rate: String,
    /// taker手续费率
    #[serde(rename = "takerFeeRate")]
    pub taker_fee_rate: String,
} 