use crate::dto::common::{MarginMode, OrderType, PositionSide, Side};
use core::fmt::{Display, Formatter};
use serde::{Deserialize, Serialize};

pub enum TdModeEnum {
    /// 保证金模式：isolated：逐仓
    ISOLATED,
    //保证金模式 ；cross：全仓
    CROSS,
    ///非保证模式，现货
    CASH,
}
impl Display for TpOrdKindEnum {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            TpOrdKindEnum::CONDITION => write!(f, "condition"),
            TpOrdKindEnum::LIMIT => write!(f, "limit"),
        }
    }
}

impl Display for TdModeEnum {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            TdModeEnum::ISOLATED => write!(f, "isolated"),
            TdModeEnum::CROSS => write!(f, "cross"),
            TdModeEnum::CASH => write!(f, "cash"),
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AttachAlgoOrdReqDto {
    /// 下单附带止盈止损时，客户自定义的策略订单ID
    /// 字母（区分大小写）与数字的组合，可以是纯字母、纯数字且长度要在1-32位之间。
    /// 订单完全成交，下止盈止损委托单时，该值会传给algoClOrdId
    pub attach_algo_cl_ord_id: Option<String>,
    /// 止盈触发价
    /// 对于条件止盈单，如果填写此参数，必须填写 止盈委托价
    pub tp_trigger_px: Option<String>,
    /// 止盈委托价
    /// 对于条件止盈单，如果填写此参数，必须填写 止盈触发价
    /// 对于限价止盈单，需填写此参数，不需要填写止盈触发价
    /// 委托价格为-1时，执行市价止盈
    pub tp_ord_px: Option<String>,
    /// 止盈订单类型
    /// condition: 条件单
    /// limit: 限价单
    /// 默认为condition
    pub tp_ord_kind: Option<String>,
    /// 止盈触发价类型
    /// last：最新价格
    /// index：指数价格
    /// mark：标记价格
    /// 默认为last
    pub tp_trigger_px_type: Option<String>,

    /// 止损触发价，如果填写此参数，必须填写 止损委托价
    pub sl_trigger_px: Option<String>,
    /// 止损委托价，如果填写此参数，必须填写 止损触发价
    /// 委托价格为-1时，执行市价止损
    pub sl_ord_px: Option<String>,
    /// 止损触发价类型
    /// last：最新价格
    /// index：指数价格
    /// mark：标记价格
    /// 默认为last
    pub sl_trigger_px_type: Option<String>,
    /// 数量。仅适用于“多笔止盈”的止盈订单，且对于“多笔止盈”的止盈订单必填
    pub sz: Option<String>,
    /// 是否启用开仓价止损，仅适用于分批止盈的止损订单，第一笔止盈触发时，止损触发价格是否移动到开仓均价止损
    /// 0：不开启，默认值
    /// 1：开启，且止损触发价不能为空
    pub amend_px_on_trigger_type: Option<i32>,
}
impl AttachAlgoOrdReqDto {
    /// 创建止盈止损订单
    /// tp_trigger_px: 止盈触发价
    /// tp_ord_px: 止盈委托价
    /// sl_trigger_px: 止损触发价
    /// sl_ord_px: 止损委托价
    /// sz: 数量
    pub fn new(
        tp_trigger_px: Option<String>,
        tp_ord_px: Option<String>,
        sl_trigger_px: Option<String>,
        sl_ord_px: Option<String>,
        sz: String,
    ) -> Self {
        Self {
            attach_algo_cl_ord_id: None,
            tp_trigger_px: tp_trigger_px,
            tp_ord_px: tp_ord_px,
            tp_ord_kind: Some(TpOrdKindEnum::CONDITION.to_string()),
            sl_trigger_px: sl_trigger_px,
            sl_ord_px: sl_ord_px,
            tp_trigger_px_type: Some("last".to_string()),
            sl_trigger_px_type: Some("last".to_string()),
            sz: Some(sz),
            amend_px_on_trigger_type: Some(0),
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct OrderReqDto {
    /// 产品ID，如 BTC-USDT
    pub inst_id: String,
    /// 交易模式
    /// 保证金模式：isolated：逐仓 ；cross：全仓
    /// 非保证金模式：cash：非保证金
    /// spot_isolated：现货逐仓(仅适用于现货带单) ，现货带单时，tdMode 的值需要指定为spot_isolated
    pub td_mode: String,
    /// 保证金币种，仅适用于单币种保证金模式下的全仓杠杆订单
    pub ccy: Option<String>,
    /// 客户自定义订单ID
    /// 字母（区分大小写）与数字的组合，可以是纯字母、纯数字且长度要在1-32位之间。
    pub cl_ord_id: Option<String>,
    /// 订单标签
    /// 字母（区分大小写）与数字的组合，可以是纯字母、纯数字，且长度在1-16位之间。
    pub tag: Option<String>,
    /// 订单方向
    /// buy：买， sell：卖
    pub side: String,
    /// 持仓方向
    /// 在开平仓模式下必填，且仅可选择 long 或 short。 仅适用交割、永续。
    pub pos_side: Option<String>,
    /// 订单类型
    /// market：市价单
    /// limit：限价单
    /// post_only：只做maker单
    /// fok：全部成交或立即取消
    /// Ioc：立即成交并取消剩余
    /// optimal_limit_ioc：市价委托立即成交并取消剩余（仅适用交割、永续）
    /// mmp：做市商保护(仅适用于组合保证金账户模式下的期权订单)
    /// mmp_and_post_only：做市商保护且只做maker单(仅适用于组合保证金账户模式下的期权订单)
    pub ord_type: String,
    /// 委托数量
    pub sz: String,
    /// 委托价格，仅适用于limit、post_only、fok、Ioc、mmp、mmp_and_post_only类型的订单
    /// 期权下单时，px/pxUsd/pxVol 只能填一个
    pub px: Option<String>,
    /// 以USD价格进行期权下单，仅适用于期权
    /// 期权下单时 px/pxUsd/pxVol 必填一个，且只能填一个
    pub px_usd: Option<String>,
    /// 以隐含波动率进行期权下单，例如 1 代表 100%，仅适用于期权
    /// 期权下单时 px/pxUsd/pxVol 必填一个，且只能填一个
    pub px_vol: Option<String>,
    /// 是否只减仓，true 或 false，默认false
    /// 仅适用于币币杠杆，以及买卖模式下的交割/永续
    /// 仅适用于单币种保证金模式和跨币种保证金模式
    pub reduce_only: Option<bool>,
    /// 市价单委托数量sz的单位，仅适用于币币市价订单
    /// base_ccy: 交易货币 ；quote_ccy：计价货币
    /// 买单默认quote_ccy， 卖单默认base_ccy
    pub tgt_ccy: Option<String>,
    /// 是否禁止币币市价改单，true 或 false，默认false
    /// 为true时，余额不足时，系统不会改单，下单会失败，仅适用于币币市价单
    pub ban_amend: Option<bool>,
    /// 一键借币类型，仅适用于杠杆逐仓的一键借币模式：
    /// manual：手动，auto_borrow：自动借币，auto_repay：自动还币
    /// 默认是manual：手动（已弃用）
    pub quick_mgn_type: Option<String>,
    /// 自成交保护ID。来自同一个母账户配着同一个ID的订单不能自成交
    /// 用户自定义1<=x<=999999999的整数（已弃用）
    pub stp_id: Option<String>,
    /// 自成交保护模式
    /// 默认为 cancel maker
    /// cancel_maker,cancel_taker, cancel_both
    /// Cancel both不支持FOK
    pub stp_mode: Option<String>,
    /// 下单附带止盈止损信息
    pub attach_algo_ords: Option<Vec<AttachAlgoOrdReqDto>>,
}

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

// 止盈订单类型
// 默认为condition
pub enum TpOrdKindEnum {
    // : 条件单
    CONDITION,
    // : 限价单
    LIMIT,
}

/// 订单响应数据
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct OrderResDto {
    /// 订单ID
    pub ord_id: String,
    /// 客户自定义订单ID
    pub cl_ord_id: Option<String>,
    /// 订单标签
    pub tag: Option<String>,
    /// 系统完成订单请求处理的时间戳，Unix时间戳的毫秒数格式，如 1597026383085
    pub ts: String,
    /// 事件执行结果的code，0代表成功
    pub s_code: String,
    /// 事件执行失败或成功时的msg
    pub s_msg: Option<String>,
}

/// 市价平仓请求参数结构体
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct CloseOrderReqDto {
    /// 产品ID
    pub inst_id: String,
    /// 持仓方向（可选）
    /// 买卖模式下：可不填写此参数，默认值net，如果填写，仅可以填写net
    /// 开平仓模式下：必须填写此参数，且仅可以填写 long：平多，short：平空
    pub pos_side: Option<String>,
    /// 保证金模式
    /// cross：全仓；isolated：逐仓
    pub mgn_mode: String,
    /// 保证金币种（可选）
    /// 单币种保证金模式的全仓币币杠杆平仓必填
    pub ccy: Option<String>,
    /// 当市价全平时，平仓单是否需要自动撤销，默认为false
    /// false：不自动撤单；true：自动撤单
    pub auto_cxl: Option<bool>,
    /// 客户自定义ID（可选）
    /// 字母（区分大小写）与数字的组合，可以是纯字母、纯数字且长度要在1-32位之间
    pub cl_ord_id: Option<String>,
    /// 订单标签（可选）
    /// 字母（区分大小写）与数字的组合，可以是纯字母、纯数字，且长度在1-16位之间
    pub tag: Option<String>,
}

/// 市价平仓响应参数结构体
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct CloseOrderResDto {
    /// 产品ID
    pub inst_id: String,
    /// 持仓方向（可选）
    /// 买卖模式下：可不填写此参数，默认值net，如果填写，仅可以填写net
    /// 开平仓模式下：必须填写此参数，且仅可以填写 long：平多，short：平空
    pub pos_side: Option<String>,
    /// 客户自定义ID（可选）
    /// 字母（区分大小写）与数字的组合，可以是纯字母、纯数字且长度要在1-32位之间
    pub cl_ord_id: Option<String>,
    /// 订单标签（可选）
    /// 字母（区分大小写）与数字的组合，可以是纯字母、纯数字，且长度在1-16位之间
    pub tag: Option<String>,
}

pub enum OrdTypeEnum {
    /// 限价单
    LIMIT,
    /// 市价单
    MARKET,
    /// 只做make单
    PostOnly,
    /// 全部成交或立即取消
    FOK,
    /// 立即成交并取消全部
    Ioc,
    // 市价委托立即成交并取消剩余（仅适用交割、永续）
    OptimalLimitIoc,
}

impl Display for OrdTypeEnum {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            OrdTypeEnum::LIMIT => write!(f, "limit"),
            OrdTypeEnum::MARKET => write!(f, "market"),
            OrdTypeEnum::PostOnly => write!(f, "post_only"),
            OrdTypeEnum::FOK => write!(f, "fok"),
            OrdTypeEnum::Ioc => write!(f, "ioc"),
            OrdTypeEnum::OptimalLimitIoc => write!(f, "optimal_limit_ioc"),
        }
    }
}
