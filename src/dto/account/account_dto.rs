use serde::{Deserialize, Serialize};
use crate::dto::common::MarginMode;

/// 平仓策略委托订单结构体
#[derive(Serialize, Deserialize, Debug)]
pub struct CloseOrderAlgo {
    /// 策略委托单ID
    pub algo_id: String,
    /// 止损触发价
    pub sl_trigger_px: Option<String>,
    /// 止损触发价类型
    pub sl_trigger_px_type: Option<String>,
    /// 止盈委托价
    pub tp_trigger_px: Option<String>,
    /// 止盈触发价类型
    pub tp_trigger_px_type: Option<String>,
    /// 策略委托触发时，平仓的百分比。1 代表100%
    pub close_fraction: Option<String>,
}

/// 持仓信息结构体
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct TradingNumRequestParams {
    pub inst_id: String,              // 产品ID，如 BTC-USDT
    pub td_mode: String,              // 交易模式: cross, isolated, cash, spot_isolated
    pub ccy: Option<String>,          // 保证金币种，仅适用于单币种保证金模式下的全仓杠杆订单
    pub reduce_only: Option<bool>,    // 是否为只减仓模式，仅适用于币币杠杆
    pub px: Option<String>,           // 对应平仓价格下的可用数量，默认为市价，仅适用于杠杆只减仓
    pub un_spot_offset: Option<bool>, // true：禁止现货对冲，false：允许现货对冲，默认为false，仅适用于组合保证金模式
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct TradingNumResponseData {
    pub inst_id: String,    // 产品ID，如 BTC-USDT
    pub avail_buy: String,  //最大买入可用数量
    pub avail_sell: String, //最大卖出可用数量
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct TradingSwapNumRequestParams {
    pub inst_id: String,              // 产品ID，如 BTC-USDT
    pub td_mode: String,              // 交易模式: cross, isolated, cash, spot_isolated
    pub ccy: Option<String>,          // 保证金币种，仅适用于单币种保证金模式下的全仓杠杆订单
    pub px: Option<String>, // 委托价格当不填委托价时，交割和永续会取当前限价计算，其他业务线会按当前最新成交价计算当指定多个产品ID查询时，忽略该参数，当未填写处理
    pub leverage: Option<String>, // 开仓杠杆倍数默认为当前杠杆倍数仅适用于币币杠杆/交割/永续
    pub un_spot_offset: Option<bool>, // true：禁止现货对冲，false：允许现货对冲，默认为false，仅适用于组合保证金模式
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct TradingSwapNumResponseData {
    pub inst_id: String,  // 产品ID，如 BTC-USDT
    pub ccy: String,      //保证金币种
    pub max_buy: String,  //最大买入可用数量
    pub max_sell: String, //最大卖出可用数量
}
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


#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct SetLeverageRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inst_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ccy: Option<String>,
    pub lever: String,
    pub mgn_mode: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pos_side: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SetLeverageData {
    pub lever: String,
    pub mgn_mode: String,
    pub inst_id: String,
    pub pos_side: String,
}

/// 持仓信息结构体
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Position {
    /// 产品类型
    pub inst_type: String,
    /// 保证金模式 (cross: 全仓, isolated: 逐仓)
    pub mgn_mode: String,
    /// 持仓ID
    pub pos_id: String,
    /// 持仓方向 (long: 开平仓模式开多, short: 开平仓模式开空, net: 买卖模式)
    pub pos_side: String,
    /// 持仓数量
    pub pos: String,
    /// 仓位资产币种，仅适用于币币杠杆仓位
    pub pos_ccy: Option<String>,
    /// 可平仓数量，适用于币币杠杆, 交割/永续（开平仓模式），期权
    pub avail_pos: Option<String>,
    /// 开仓平均价
    pub avg_px: Option<String>,
    /// 未实现收益（以标记价格计算）
    pub upl: Option<String>,
    /// 未实现收益率（以标记价格计算）
    pub upl_ratio: Option<String>,
    /// 以最新成交价格计算的未实现收益
    pub upl_last_px: Option<String>,
    /// 以最新成交价格计算的未实现收益率
    pub upl_ratio_last_px: Option<String>,
    /// 产品ID，如 BTC-USD-180216
    pub inst_id: String,
    /// 杠杆倍数，不适用于期权以及组合保证金模式下的全仓仓位
    pub lever: Option<String>,
    /// 预估强平价，不适用于期权
    pub liq_px: Option<String>,
    /// 最新标记价格
    pub mark_px: Option<String>,
    /// 初始保证金，仅适用于全仓
    pub imr: Option<String>,
    /// 保证金余额，可增减，仅适用于逐仓
    pub margin: Option<String>,
    /// 保证金率
    pub mgn_ratio: Option<String>,
    /// 维持保证金
    pub mmr: Option<String>,
    /// 负债额，仅适用于币币杠杆
    pub liab: Option<String>,
    /// 负债币种，仅适用于币币杠杆
    pub liab_ccy: Option<String>,
    /// 利息，已经生成的未扣利息
    pub interest: Option<String>,
    /// 最新成交ID
    pub trade_id: Option<String>,
    /// 期权市值，仅适用于期权
    pub opt_val: Option<String>,
    /// 逐仓杠杆负债对应平仓挂单的数量
    pub pending_close_ord_liab_val: Option<String>,
    /// 以美金价值为单位的持仓数量
    pub notional_usd: Option<String>,
    /// 信号区，分为5档，从1到5，数字越小代表adl强度越弱
    pub adl: Option<String>,
    /// 占用保证金的币种
    pub ccy: Option<String>,
    /// 最新成交价
    pub last: Option<String>,
    /// 最新指数价格
    pub idx_px: Option<String>,
    /// 美金价格
    pub usd_px: Option<String>,
    /// 盈亏平衡价
    pub be_px: Option<String>,
    /// 美金本位持仓仓位delta，仅适用于期权
    pub delta_bs: Option<String>,
    /// 币本位持仓仓位delta，仅适用于期权
    pub delta_pa: Option<String>,
    /// 美金本位持仓仓位gamma，仅适用于期权
    pub gamma_bs: Option<String>,
    /// 币本位持仓仓位gamma，仅适用于期权
    pub gamma_pa: Option<String>,
    /// 美金本位持仓仓位theta，仅适用于期权
    pub theta_bs: Option<String>,
    /// 币本位持仓仓位theta，仅适用于期权
    pub theta_pa: Option<String>,
    /// 美金本位持仓仓位vega，仅适用于期权
    pub vega_bs: Option<String>,
    /// 币本位持仓仓位vega，仅适用于期权
    pub vega_pa: Option<String>,
    /// 现货对冲占用数量，适用于组合保证金模式
    pub spot_in_use_amt: Option<String>,
    /// 现货对冲占用币种，适用于组合保证金模式
    pub spot_in_use_ccy: Option<String>,
    /// 用户自定义现货占用数量，适用于组合保证金模式
    pub cl_spot_in_use_amt: Option<String>,
    /// 系统计算得到的最大可能现货占用数量，适用于组合保证金模式
    pub max_spot_in_use_amt: Option<String>,
    /// 已实现收益
    pub realized_pnl: Option<String>,
    /// 平仓订单累计收益额
    pub pnl: Option<String>,
    /// 累计手续费金额
    pub fee: Option<String>,
    /// 累计资金费用
    pub funding_fee: Option<String>,
    /// 累计爆仓罚金
    pub liq_penalty: Option<String>,
    /// 平仓策略委托订单
    pub close_order_algo: Option<Vec<CloseOrderAlgo>>,
    /// 持仓创建时间，Unix时间戳的毫秒数格式
    pub c_time: Option<String>,
    /// 最近一次持仓更新时间，Unix时间戳的毫秒数格式
    pub u_time: Option<String>,
    /// 外部业务id，e.g. 体验券id
    pub biz_ref_id: Option<String>,
    /// 外部业务类型
    pub biz_ref_type: Option<String>,
}