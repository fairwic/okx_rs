use crate::dto::common::MarginMode;
use serde::{Deserialize, Serialize};

/// 平仓策略委托订单结构体
#[derive(Serialize, Deserialize, Debug, Clone)]
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
/// uTime	String	账户信息的更新时间，Unix时间戳的毫秒数格式，如 1597026383085
/// totalEq	String	美金层面权益
/// isoEq	String	美金层面逐仓仓位权益
/// 适用于合约模式/跨币种保证金模式/组合保证金模式
/// adjEq	String	美金层面有效保证金
/// 适用于现货模式/跨币种保证金模式/组合保证金模式
/// availEq	String	账户美金层面可用保证金，排除因总质押借币上限而被限制的币种
/// 适用于跨币种保证金模式/组合保证金模式
/// ordFroz	String	美金层面全仓挂单占用保证金
/// 仅适用于现货模式/跨币种保证金模式/组合保证金模式
/// imr	String	美金层面占用保证金
/// 适用于现货模式/跨币种保证金模式/组合保证金模式
/// mmr	String	美金层面维持保证金
/// 适用于现货模式/跨币种保证金模式/组合保证金模式
/// borrowFroz	String	账户美金层面潜在借币占用保证金
/// 仅适用于现货模式/跨币种保证金模式/组合保证金模式。在其他账户模式下为""。
/// mgnRatio	String	美金层面维持保证金率
/// 适用于现货模式/跨币种保证金模式/组合保证金模式
/// notionalUsd	String	以美金价值为单位的持仓数量，即仓位美金价值
/// 适用于现货模式/跨币种保证金模式/组合保证金模式
/// notionalUsdForBorrow	String	借币金额（美元价值）
/// 适用于现货模式/跨币种保证金模式/组合保证金模式
/// notionalUsdForSwap	String	永续合约持仓美元价值
/// 适用于跨币种保证金模式/组合保证金模式
/// notionalUsdForFutures	String	交割合约持仓美元价值
/// 适用于跨币种保证金模式/组合保证金模式
/// notionalUsdForOption	String	期权持仓美元价值
/// 适用于现货模式/跨币种保证金模式/组合保证金模式
/// upl	String	账户层面全仓未实现盈亏（美元单位）
/// 适用于跨币种保证金模式/组合保证金模式
/// details	Array of objects	各币种资产详细信息
/// ccy	String	币种
/// eq	String	币种总权益
/// cashBal	String	币种余额
/// uTime	String	币种余额信息的更新时间，Unix时间戳的毫秒数格式，如 1597026383085
/// isoEq	String	币种逐仓仓位权益
/// 适用于合约模式/跨币种保证金模式/组合保证金模式
/// availEq	String	可用保证金
/// 适用于合约模式/跨币种保证金模式/组合保证金模式
/// disEq	String	美金层面币种折算权益
/// 适用于现货模式(开通了借币功能)/跨币种保证金模式/组合保证金模式
/// fixedBal	String	抄底宝、逃顶宝功能的币种冻结金额
/// availBal	String	可用余额
/// frozenBal	String	币种占用金额
/// ordFrozen	String	挂单冻结数量
/// 适用于现货模式/合约模式/跨币种保证金模式
/// liab	String	币种负债额
/// 值为正数，如 "21625.64"
/// 适用于现货模式/跨币种保证金模式/组合保证金模式
/// upl	String	未实现盈亏
/// 适用于合约模式/跨币种保证金模式/组合保证金模式
/// uplLiab	String	由于仓位未实现亏损导致的负债
/// 适用于跨币种保证金模式/组合保证金模式
/// crossLiab	String	币种全仓负债额
/// 适用于现货模式/跨币种保证金模式/组合保证金模式
/// isoLiab	String	币种逐仓负债额
/// 适用于跨币种保证金模式/组合保证金模式
/// rewardBal	String	体验金余额
/// mgnRatio	String	币种全仓维持保证金率，衡量账户内某项资产风险的指标
/// 适用于合约模式且有全仓仓位时
/// imr	String	币种维度全仓占用保证金
/// 适用于合约模式且有全仓仓位时
/// mmr	String	币种维度全仓维持保证金
/// 适用于合约模式且有全仓仓位时
/// interest	String	计息，应扣未扣利息
/// 值为正数，如 9.01
/// 适用于现货模式/跨币种保证金模式/组合保证金模式
/// twap	String	当前负债币种触发系统自动换币的风险
/// 0、1、2、3、4、5其中之一，数字越大代表您的负债币种触发自动换币概率越高
/// 适用于现货模式/跨币种保证金模式/组合保证金模式
/// maxLoan	String	币种最大可借
/// 适用于现货模式/跨币种保证金模式/组合保证金模式 的全仓
/// eqUsd	String	币种权益美金价值
/// borrowFroz	String	币种美金层面潜在借币占用保证金
/// 仅适用于现货模式/跨币种保证金模式/组合保证金模式。在其他账户模式下为""。
/// notionalLever	String	币种杠杆倍数
/// 适用于合约模式
/// stgyEq	String	策略权益
/// isoUpl	String	逐仓未实现盈亏
/// 适用于合约模式/跨币种保证金模式/组合保证金模式
/// spotInUseAmt	String	现货对冲占用数量
/// 适用于组合保证金模式
/// clSpotInUseAmt	String	用户自定义现货占用数量
/// 适用于组合保证金模式
/// maxSpotInUse	String	系统计算得到的最大可能现货占用数量
/// 适用于组合保证金模式
/// spotIsoBal	String	现货逐仓余额
/// 仅适用于现货带单/跟单
/// 适用于现货模式/合约模式
/// smtSyncEq	String	合约智能跟单权益
/// 默认为0，仅适用于跟单人。
/// spotCopyTradingEq	String	现货智能跟单权益
/// 默认为0，仅适用于跟单人。
/// spotBal	String	现货余额 ，单位为 币种，比如 BTC。详情
/// openAvgPx	String	现货开仓成本价 单位 USD。 详情
/// accAvgPx	String	现货累计成本价 单位 USD。 详情
/// spotUpl	String	现货未实现收益，单位 USD。 详情
/// spotUplRatio	String	现货未实现收益率。详情
/// totalPnl	String	现货累计收益，单位 USD。 详情
/// totalPnlRatio	String	现货累计收益率。详情
/// collateralEnabled	Boolean	true：质押币
/// false：非质押币
/// 适用于跨币种保证金模式
/// collateralRestrict	Boolean	平台维度的质押借币限制
/// true
/// false
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Balance {
    /// 账户信息的更新时间，Unix时间戳的毫秒数格式，如 1597026383085
    #[serde(rename = "uTime")]
    pub u_time: String,
    /// 美金层面权益
    #[serde(rename = "totalEq")]
    pub total_eq: String,
    /// 美金层面逐仓仓位权益
    #[serde(rename = "isoEq")]
    pub iso_eq: String,
    /// 美金层面有效保证金
    #[serde(rename = "adjEq")]
    pub adj_eq: String,
    /// 账户美金层面可用保证金，排除因总质押借币上限而被限制的币种
    #[serde(rename = "availEq")]
    pub avail_eq: String,
    /// 美金层面全仓挂单占用保证金
    #[serde(rename = "ordFroz")]
    pub ord_froz: String,
    /// 美金层面占用保证金
    #[serde(rename = "imr")]
    pub imr: String,
    /// 美金层面维持保证金
    #[serde(rename = "mmr")]
    pub mmr: String,
    /// 美金层面潜在借币占用保证金
    #[serde(rename = "borrowFroz")]
    pub borrow_froz: String,
    /// 美金层面维持保证金率
    #[serde(rename = "mgnRatio")]
    pub mgn_ratio: String,
    /// 以美金价值为单位的持仓数量，即仓位美金价值
    #[serde(rename = "notionalUsd")]
    pub notional_usd: String,
    /// 借币金额（美元价值）
    #[serde(rename = "notionalUsdForBorrow")]
    pub notional_usd_for_borrow: String,
    /// 永续合约持仓美元价值
    #[serde(rename = "notionalUsdForSwap")]
    pub notional_usd_for_swap: String,
    /// 交割合约持仓美元价值
    #[serde(rename = "notionalUsdForFutures")]
    pub notional_usd_for_futures: String,
    /// 期权持仓美元价值
    #[serde(rename = "notionalUsdForOption")]
    pub notional_usd_for_option: String,
    /// 账户层面全仓未实现盈亏（美元单位）
    #[serde(rename = "upl")]
    pub upl: String,
    /// 各币种资产详细信息
    pub details: Vec<BalanceDetail>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BalanceDetail {
    /// 币种
    pub ccy: String,
    /// 币种总权益
    pub eq: String,
    /// 币种余额
    #[serde(rename = "cashBal")]
    pub cash_bal: String,
    /// 币种逐仓仓位权益
    #[serde(rename = "isoEq")]
    pub iso_eq: String,
    /// 可用保证金
    #[serde(rename = "availEq")]
    pub avail_eq: String,
    /// 美金层面币种折算权益
    #[serde(rename = "disEq")]
    pub dis_eq: String,
    /// 抄底宝、逃顶宝功能的币种冻结金额
    #[serde(rename = "fixedBal")]
    pub fixed_bal: String,
    /// 可用余额
    #[serde(rename = "availBal")]
    pub avail_bal: String,
    /// 币种占用金额
    #[serde(rename = "frozenBal")]
    pub frozen_bal: String,
    /// 挂单冻结数量
    #[serde(rename = "ordFrozen")]
    pub ord_frozen: String,
    /// 币种负债额
    #[serde(rename = "liab")]
    pub liab: String,
    /// 未实现盈亏
    #[serde(rename = "upl")]
    pub upl: String,
    /// 由于仓位未实现亏损导致的负债
    #[serde(rename = "uplLiab")]
    pub upl_liab: String,
    /// 币种全仓负债额  
    #[serde(rename = "crossLiab")]
    pub cross_liab: String,
    /// 币种逐仓负债额
    #[serde(rename = "isoLiab")]
    pub iso_liab: String,
    /// 体验金余额
    #[serde(rename = "rewardBal")]
    pub reward_bal: String,
    /// 币种全仓维持保证金率
    #[serde(rename = "mgnRatio")]
    pub mgn_ratio: String,
    /// 币种维度全仓占用保证金
    #[serde(rename = "imr")]
    pub imr: String,
    /// 币种维度全仓维持保证金
    #[serde(rename = "mmr")]
    pub mmr: String,
    /// 计息，应扣未扣利息
    #[serde(rename = "interest")]
    pub interest: String,
    /// 当前负债币种触发系统自动换币的风险
    #[serde(rename = "twap")]
    pub twap: String,
    /// 币种最大可借
    #[serde(rename = "maxLoan")]
    pub max_loan: String,
    /// 币种权益美金价值
    #[serde(rename = "eqUsd")]
    pub eq_usd: String,
    /// 币种美金层面潜在借币占用保证金
    #[serde(rename = "borrowFroz")]
    pub borrow_froz: String,
    /// 币种杠杆倍数
    #[serde(rename = "notionalLever")]
    pub notional_lever: String,
    /// 策略权益
    #[serde(rename = "stgyEq")]
    pub stgy_eq: String,
    /// 逐仓未实现盈亏
    #[serde(rename = "isoUpl")]
    pub iso_upl: String,
    /// 现货对冲占用数量
    #[serde(rename = "spotInUseAmt")]
    pub spot_in_use_amt: String,
    /// 用户自定义现货占用数量
    #[serde(rename = "clSpotInUseAmt")]
    pub cl_spot_in_use_amt: String,
    /// 系统计算得到的最大可能现货占用数量
    #[serde(rename = "maxSpotInUse")]
    pub max_spot_in_use: String,
    /// 现货逐仓余额
    #[serde(rename = "spotIsoBal")]
    pub spot_iso_bal: String,
    /// 合约智能跟单权益
    #[serde(rename = "smtSyncEq")]
    pub smt_sync_eq: String,
    /// 现货智能跟单权益
    #[serde(rename = "spotCopyTradingEq")]
    pub spot_copy_trading_eq: String,
    /// 现货余额
    #[serde(rename = "spotBal")]
    pub spot_bal: String,
    /// 现货开仓成本价
    #[serde(rename = "openAvgPx")]
    pub open_avg_px: String,
    /// 现货累计成本价
    #[serde(rename = "accAvgPx")]
    pub acc_avg_px: String,
    /// 现货未实现收益
    #[serde(rename = "spotUpl")]
    pub spot_upl: String,
    /// 现货未实现收益率
    #[serde(rename = "spotUplRatio")]
    pub spot_upl_ratio: String,
    /// 现货累计收益
    #[serde(rename = "totalPnl")]
    pub total_pnl: String,
    /// 现货累计收益率
    #[serde(rename = "totalPnlRatio")]
    pub total_pnl_ratio: String,
    /// 质押币
    #[serde(rename = "collateralEnabled")]
    pub collateral_enabled: bool,
    /// 质押风险
    #[serde(rename = "collateralRisk")]
    pub collateral_risk: String,
    /// 质押风险率
    #[serde(rename = "collateralRestrict")]
    pub collateral_restrict: String,
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
#[derive(Serialize, Deserialize, Debug, Clone)]
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
