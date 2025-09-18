use std::fmt;
use thiserror::Error;

/// OKX SDK的统一错误类型
#[derive(Error, Debug)]
pub enum Error {
    /// API请求错误
    #[error("API请求错误: {0}")]
    ApiRequestError(String),

    /// HTTP客户端错误
    #[error("HTTP错误: {0}")]
    HttpError(#[from] reqwest::Error),

    /// JSON序列化/反序列化错误
    #[error("JSON错误: {0}")]
    JsonError(#[from] serde_json::Error),

    /// IO错误
    #[error("IO错误: {0}")]
    IoError(#[from] std::io::Error),

    /// WebSocket错误
    #[error("WebSocket错误: {0}")]
    WebSocketError(String),

    /// 参数错误
    #[error("参数错误: {0}")]
    ParameterError(String),

    /// 解析错误
    #[error("解析错误: {0}")]
    ParseError(String),

    /// 配置错误
    #[error("配置错误: {0}")]
    ConfigError(String),

    /// 认证错误
    #[error("认证错误: {0}")]
    AuthenticationError(String),

    /// OKX API错误
    #[error("OKX API错误 (代码: {code}): {message},{smg}")]
    OkxApiError {
        code: String,
        message: String,
        smg: String,
    },

    /// 连接错误
    #[error("连接错误: {0}")]
    ConnectionError(String),

    /// 订阅错误
    #[error("订阅错误: {0}")]
    SubscriptionError(String),

    /// 管理器错误
    #[error("管理器错误: {0}")]
    ManagerError(String),

    /// 网络错误
    #[error("网络错误: {0}")]
    NetworkError(String),

    /// 超时错误
    #[error("超时错误: {0}")]
    TimeoutError(String),

    /// 限流错误
    #[error("限流错误: {0}")]
    RateLimitError(String),

    /// 未知错误
    #[error("未知错误: {0}")]
    Unknown(String),
}

/// 错误严重程度
#[derive(Debug, Clone, PartialEq)]
pub enum ErrorSeverity {
    /// 致命错误 - 需要立即停止系统
    Critical,
    /// 严重错误 - 需要人工介入
    High,
    /// 中等错误 - 系统可继续运行但需关注
    Medium,
    /// 轻微错误 - 仅记录日志
    Low,
}

impl Error {
    /// 获取错误的严重程度
    pub fn severity(&self) -> ErrorSeverity {
        match self {
            Error::AuthenticationError(_) | Error::ConfigError(_) => ErrorSeverity::Critical,
            Error::NetworkError(_) | Error::ConnectionError(_) => ErrorSeverity::High,
            Error::WebSocketError(_) | Error::SubscriptionError(_) => ErrorSeverity::Medium,
            Error::TimeoutError(_) | Error::RateLimitError(_) => ErrorSeverity::Medium,
            Error::JsonError(_) | Error::ParameterError(_) => ErrorSeverity::Low,
            _ => ErrorSeverity::Medium,
        }
    }

    /// 判断错误是否可恢复
    pub fn is_recoverable(&self) -> bool {
        match self {
            Error::NetworkError(_) | Error::ConnectionError(_) | Error::TimeoutError(_) => true,
            Error::WebSocketError(_) | Error::SubscriptionError(_) => true,
            Error::RateLimitError(_) => true,
            Error::AuthenticationError(_) | Error::ConfigError(_) => false,
            _ => false,
        }
    }

    /// 获取建议的重试延迟（秒）
    pub fn retry_delay(&self) -> Option<u64> {
        match self {
            Error::NetworkError(_) | Error::ConnectionError(_) => Some(5),
            Error::TimeoutError(_) => Some(3),
            Error::RateLimitError(_) => Some(60),
            Error::WebSocketError(_) => Some(10),
            _ => None,
        }
    }
}

/// OKX API特定错误码
#[derive(Error, Debug, Clone, Copy, PartialEq, Eq)]
pub enum ApiErrorCode {
    /// 操作成功
    Ok = 0,
    /// 操作全部失败
    OperationFailed = 1,
    /// 批量操作部分成功
    PartialSuccess = 2,

    // 通用错误码 (50000-50999)
    /// POST请求的body不能为空
    EmptyBody = 50000,
    /// 服务暂时不可用，请稍后重试
    ServiceUnavailable = 50001,
    /// JSON 语法错误
    JsonSyntaxError = 50002,
    /// 接口请求超时
    RequestTimeout = 50004,
    /// 接口已下线或无法使用
    InterfaceDeprecated = 50005,
    /// 无效的Content-Type
    InvalidContentType = 50006,
    /// 用户被冻结
    UserFrozen = 50007,
    /// 用户不存在
    UserNotFound = 50008,
    /// 用户处于爆仓冻结
    UserMarginFrozen = 50009,
    /// 用户ID为空
    UserIdEmpty = 50010,
    /// 请求频率太高
    TooManyRequests = 50011,
    /// 账户状态无效
    InvalidAccountStatus = 50012,
    /// 当前系统繁忙
    SystemBusy = 50013,

    // API 类错误码
    /// Api 已被冻结
    ApiFrozen = 50100,
    /// APIKey 与当前环境不匹配
    ApiKeyEnvironmentMismatch = 50101,
    /// 请求时间戳过期
    RequestTimestampExpired = 50102,
    /// 请求头"OK-ACCESS-KEY"不能为空
    MissingOkAccessKey = 50103,
    /// 请求头"OK-ACCESS-PASSPHRASE"不能为空
    MissingOkAccessPassphrase = 50104,
    /// 请求头"OK-ACCESS-PASSPHRASE"错误
    InvalidOkAccessPassphrase = 50105,
    /// 请求头"OK-ACCESS-SIGN"不能为空
    MissingOkAccessSign = 50106,
    /// 请求头"OK-ACCESS-TIMESTAMP"不能为空
    MissingOkAccessTimestamp = 50107,

    /// 券商ID不存在
    BrokerIdNotFound = 50108,
    /// 券商域名不存在
    BrokerDomainNotFound = 50109,
    /// 您的IP{param0}不在APIKey绑定IP名单中 (您可以将您的IP加入到APIKey绑定白名单中)
    IpNotInApiKeyBindingIpList = 50110,
    /// 无效的OK-ACCESS-KEY
    InvalidOkAccessKey = 50111,
    /// 无效的OK-ACCESS-TIMESTAMP
    InvalidOkAccessTimestamp = 50112,
    /// 无效的签名
    InvalidOkAccessSign = 50113,
    /// 无效的授权
    InvalidOkAccessAuthorization = 50114,
    /// 无效的请求类型
    InvalidRequestType = 50115,
    /// Fast API 只能创建一个 API key
    FastApiCanOnlyCreateOneApiKey = 50116,
    /// 如需将 API key 绑定 App，经纪商需要提供 IP 才能加入白名单
    BrokerNeedToProvideIpToJoinWhiteList = 50118,
    /// API key 不存在
    ApiKeyNotFound = 50119,
    /// API key 权限不足
    ApiKeyPermissionInsufficient = 50120,
    /// 您无权通过该 IP 地址 ({param0}) 访问
    IpAccessDenied = 50121,
    /// 下单金额必须超过最低金额限制
    OrderAmountMustBeGreaterThanMinimum = 50122,

    //     公共
    // 错误码从 50000 到 53999

    // 通用类
    // 错误码	HTTP 状态码	错误提示
    // 0	200
    // 1	200	操作全部失败
    // 2	200	批量操作部分成功
    /// 必填参数{param0}不能为空
    RequiredParameterEmpty = 50014,
    /// 参数{param0}和{param1}不能同时为空
    ParameterBothEmpty = 50015,
    /// 参数{param0}和{param1}不匹配
    ParameterMismatch = 50016,
    /// 当前仓位处于自动减仓 (ADL) 冻结中，无法进行相关操作，请稍后重试
    PositionAdlFrozen = 50017,
    /// {param0} 处于自动减仓 (ADL) 冻结中，无法进行相关操作，请稍后重试
    PositionAdlFrozenForParam = 50018,
    /// 当前账户处于自动减仓 (ADL) 冻结中，无法进行相关操作，请稍后重试
    AccountAdlFrozenForAccount = 50019,
    /// 当前仓位处于强平冻结中，无法进行相关操作，请稍后重试
    PositionMarginFrozen = 50020,
    /// {param0} 处于强平冻结中，无法进行相关操作，请稍后重试
    PositionMarginFrozenForParam = 50021,
    /// 当前账户处于强平冻结中，无法进行相关操作，请稍后重试
    AccountMarginFrozen = 50022,
    /// 资金费冻结，无法进行相关操作，请稍后重试
    FundingFeeFrozen = 50023,
    /// 参数{param0}和{param1}不能同时存在
    ParameterBothExist = 50024,
    /// 参数{param0}传值个数超过最大限制{param1}
    ParameterValueExceedLimit = 50025,
    /// 系统错误，请稍后重试
    SystemError = 50026,
    /// 当前账户已被限制交易，请联系客服处理
    AccountTradingRestricted = 50027,
    /// 账户异常无法下单
    AccountException = 50028,
    /// 您的账户已经触发风控体系，禁止交易该标的，请检查您在欧易注册的电子邮件以便我们的客服联系
    AccountRiskControl = 50029,
    /// 您没有使用此 API 接口的权限
    AccountPermissionDenied = 50030,
    /// 您的账户已设置禁止该币种交易，请确认后重试
    AccountCurrencyTradingRestricted = 50032,
    /// 您的账户已设置禁止该业务线交易，请确认后重试
    AccountBusinessLineTradingRestricted = 50033,
    /// 该接口要求APIKey必须绑定IP
    ApiKeyIpBindingRequired = 50035,
    /// expTime 不能早于当前系统时间，请调整 expTime 后重试
    ExpTimeTooEarly = 50036,
    /// 订单已过期
    OrderExpired = 50037,
    /// 模拟交易不支持该功能
    SimulatedTradingNotSupported = 50038,
    /// 时间戳分页时，不支持使用before参数
    TimestampBeforeNotSupported = 50039,
    /// 操作频繁，请稍后重试
    OperationTooFrequent = 50040,
    /// 用户 ID 未被列入白名单列表，请联系客服
    UserIdNotInWhiteList = 50041,
    /// 请求重复
    RequestDuplicate = 50042,
    /// 必须指定一种broker类型
    BrokerTypeRequired = 50044,
    /// simPos 应为空。投资组合计算器纳入真实现货仓位时，暂不支持添加模拟仓位。
    SimPosShouldBeEmpty = 50045,
    /// 该功能暂时无法使用，我们正在进行维护，请稍后重试
    FunctionMaintenance = 50046,
    /// {param0} 已经交割，对应的K线请使用{param1}查询
    KlineAlreadyDelivered = 50047,
    /// 切换对冲单元可能导致仓位风险水平升高，引起强制平仓。请调整仓位，使保证金处于安全状态。
    HedgingUnitRiskLevelIncrease = 50048,
    /// 无仓位档位信息，该币种不支持杠杆交易
    NoPositionLevelInfo = 50049,
    /// 您已开通期权交易服务，请勿重复开通
    OptionTradingServiceAlreadyActivated = 50050,
    /// 由于您所在国家或地区的合规限制，您无法使用该功能
    ComplianceRestrictionForCountry = 50051,
    /// 根据当地的法律法规，您无法交易您选择的币种
    ComplianceRestrictionForCurrency = 50052,
    /// 该功能只支持模拟盘
    SimulatedTradingOnly = 50053,
    /// 资产重置失败，超过每日设置5次资产上限
    AssetResetFailedTooManyTimes = 50055,
    /// 当前账户有交易挂单或持仓，请完成全部撤单/平仓后进行重置
    OrderOrPositionExists = 50056,
    /// 资产重置失败，请稍后重试
    AssetResetFailed = 50057,
    /// 该币种不支持资产重置
    AssetResetNotSupported = 50058,
    /// 继续下一步之前，请按照当地监管机构的要求完成额外步骤。您可以前往欧易网页端或 App 端了解详情。
    ComplianceRestrictionNotMet = 50059,
    /// 根据当地法律法规，您需要完成身份认证方可继续使用我们的服务。
    IdentityVerificationRequired = 50060,
    /// 订单请求频率过快，超过账户允许的最高限额
    OrderRequestRateExceeded = 50061,
    /// 该功能暂不可用
    FunctionUnavailable = 50062,
    /// 激活失败，您的体验金可能已过期或已激活
    ActivationFailed = 50063,
    /// 借币系统暂不可用，请稍后再试
    BorrowingSystemUnavailable = 50064,
    /// 当前接口不支持跨站交易功能
    CrossSiteTradingNotSupported = 50067,
    /// 风险单元保证金率校验失败
    RiskUnitMarginRateCheckFailed = 50069,
    /// {param} 已存在  // e.g. clOrdId 已存在
    AlreadyExists = 50071,

    // 交易类
    /// {param0}参数错误
    InvalidParameter = 51000,
    /// Instrument ID 或 Spread ID 不存在
    InstrumentIdOrSpreadIdNotFound = 51001,
    /// 交易产品ID不匹配指数
    TradeProductIdNotMatchIndex = 51002,
    /// ordId或clOrdId至少填一个
    OrdIdOrClOrdIdAtLeastOne = 51003,
    /// 下单失败，您在{instId} 逐仓的开平仓模式下，当前下单张数、同方向持有仓位以及同方向挂单张数之和，不能超过当前杠杆倍数允许的持仓上限{tierLimitQuantity}(张)，请调低杠杆或者使用新的子账户重新下单(当前杠杆：{leverage}×，当前下单张数：{size}张，同方向持有仓位：{posNumber}张，同方向挂单张数：{pendingNumber}张)。
    OrderFailedInMarginMode = 51004,
    /// 下单失败，您在{businessType}和交易品种{instFamily}的全仓买卖模式下，当前买入张数、当前合约持有仓位、当前合约买入挂单张数以及其他合约占用额度之和，不能超过当前杠杆倍数允许的持仓上限{tierLimitQuantity}(张)，请调低杠杆或者使用新的子账户重新下单(当前杠杆：{leverage}×，当前买入张数：{size}张，当前合约持有仓位：{posNumber}张，当前合约买入挂单张数：{pendingNumber}张，其他合约占用额度：{otherQuota}张)。
    OrderFailedInMarginModeForBusinessType = 510041,
    /// 下单失败，您在{businessType}和交易品种{instFamily}的全仓买卖模式下，当前卖出张数、当前合约持有仓位、当前合约卖出挂单张数以及其他合约占用额度之和，不能超过当前杠杆倍数允许的

    /// 修改订单失败，您在{instId}全仓的开平仓模式下，当前改单新增张数、多空持有仓位以及多空挂单张数之和，不能超过当前杠杆倍数允许的持仓上限{tierLimitQuantity}(张)，请调低杠杆或者使用新的子账户重新下单(当前杠杆：{leverage}×，当前改单新增张数：{size}张，多空持有仓位{posLongShortNumber}张，多空挂单张数：{pendingLongShortNumber}张)。
    ModifyOrderFailedInMarginMode = 510042,
    /// 修改订单失败，您在{businessType}和交易品种{instFamily}的全仓买卖模式下，修改当前买单新增张数、当前合约持有仓位、当前合约买入挂单张数以及其他合约占用额度之和，不能超过当前杠杆倍数允许的持仓上限{tierLimitQuantity}(张)，请调低杠杆或者使用新的子账户重新下单(当前杠杆：{leverage}×，修改当前买单新增张数：{size}张，当前合约持有仓位：{posNumber}张，当前合约买入挂单张数：{pendingNumber}张，其他合约占用额度：{otherQuota}张)。
    ModifyOrderFailedInMarginModeForBusinessTypeBuy = 510043,
    /// 修改订单失败，您在{businessType}和交易品种{instFamily}的全仓买卖模式下，修改当前卖单新增张数、当前合约持有仓位、当前合约卖出挂单张数以及其他合约占用额度之和，不能超过当前杠杆倍数允许的持仓上限{tierLimitQuantity}(张)，请调低杠杆或者使用新的子账户重新下单(当前杠杆：{leverage}×，修改当前卖单新增张数：{size}张，当前合约持有仓位：{posNumber}张，当前合约卖出挂单张数：{pendingNumber}张，其他合约占用额度：{otherQuota}张)。
    ModifyOrderFailedInMarginModeForBusinessTypeSell = 510044,
    /// 委托数量大于单笔上限
    OrderQuantityGreaterThanSingleLimit = 51005,
    /// 委托价格不在限价范围内（最高买入价：{param0}，最低卖出价：{param1}）
    OrderPriceNotInLimitRange = 51006,
    /// 委托失败，委托数量不可小于 1 张
    OrderQuantityLessThanOne = 51007,
    /// 委托失败，账户 {param0} 可用余额不足
    OrderFailedAccountBalanceNotEnough = 51008,

    /// 委托失败，账户资产不足，美金层面有效保证金小于 IMR（PM模式也可以尝试IOC订单降低风险）
    OrderFailedAccountBalanceNotEnoughAndAutoBorrowNotEnabledByImr = 51010,
    /// ordId重复
    OrderRepeat = 51011,
    /// 币种不存在
    OrderCurrencyNotFound = 51012,
    /// 指数不存在
    IndexNotFound = 51014,
    /// instId和instType不匹配
    NotMatchByInstIdAndInstType = 51015,
    /// clOrdId重复
    ClOrdIdRepeat = 51016,
    /// 杠杆委托交易借币超出限额
    OrderBorrowLimitExceeded = 51017,
    /// 期权交易账户不能有净开空持仓
    OrderOptionAccountCannotHaveNetOpenShortPosition = 51018,
    /// 期权全仓不能有净开多持仓
    OrderOptionAccountCannotHaveNetOpenLongPosition = 51019,
    /// 委托数量需大于或等于最小下单数量
    OrderQuantityGreaterThanMinimumOrderQuantity = 51020,
    /// 币对或合约待上线
    OrderCurrencyOrContractPending = 51021,
    /// 合约暂停中
    OrderContractPaused = 51022,
    /// 仓位不存在
    OrderPositionNotFound = 51023,
    /// 交易账户冻结
    OrderAccountFrozen = 51024,
    /// 委托笔数超限
    OrderOrderQuantityExceeded = 51025,
    /// 交易产品类型不匹配指数（instType和uly不匹配）
    OrderProductTypeMismatchIndex = 51026,
    /// 合约已到期
    OrderContractExpired = 51027,
    /// 合约交割中
    OrderContractDelivery = 51028,
    /// 合约结算中
    OrderContractSettlement = 51029,
    /// 资金费结算中
    OrderFundingFeeSettlement = 51030,
    /// 委托价格不在平仓限价范围内
    OrderPriceOutOfLiquidationRange = 51031,
    /// 市价全平中
    OrderMarketLiquidation = 51032,
    /// 币对单笔交易已达限额
    OrderCurrencyPairTransactionLimitExceeded = 51033,
    /// 成交速率超出您所设置的上限，请将做市商保护状态重置为 inactive 以继续交易。
    OrderTransactionRateExceeded = 51034,
    /// 用户没有做市订单的下单权限
    OrderMarketOrderPermission = 51035,
    /// 仅 PM 账户的期权业务线支持 MMP 类型订单
    OrderOptionAccountOnlySupportMMP = 51036,
    /// PM 账户模式下，期权仅支持持仓模式为全仓的 MMP 类型订单
    OrderOptionAccountOnlySupportFullMargin = 51042,
    /// 该逐仓仓位不存在
    OrderMarginPositionNotFound = 51043,
    /// 用户没有重置做市商保护状态的权限
    OrderMarketMakerProtectionResetPermission = 59509,
    /// 当前账户风险状态，仅支持降低账户风险方向的IOC订单
    OrderAccountRiskStateOnlySupportIOC = 51037,
    /// 当前风险模块下已经存在降低账户风险方向的IOC类型订单
    OrderAccountRiskStateIOCAlreadyExists = 51038,
    /// PM账户下交割和永续的全仓不能调整杠杆倍数
    OrderContractAccountCannotAdjustLeverage = 51039,
    /// 期权逐仓的买方不能调整保证金
    OrderOptionAccountCannotAdjustMargin = 51040,
    /// PM账户仅支持买卖模式
    OrderOptionAccountOnlySupportBuyAndSell = 51041,
    /// 当前订单类型{param0}， {param1}不支持设置止盈和止损
    OrderOrderTypeNotSupportStopLossAndProfit = 51044,
    /// 止盈触发价格应该大于委托价格
    OrderProfitTriggerPriceShouldBeGreaterThanOrderPrice = 51046,
    /// 止损触发价格应该小于委托价格
    OrderStopLossTriggerPriceShouldBeLessThanOrderPrice = 51047,
    /// 止盈触发价格应该小于委托价格
    OrderProfitTriggerPriceShouldBeLessThanOrderPrice = 51048,
    /// 止损触发价格应该大于委托价格
    OrderStopLossTriggerPriceShouldBeGreaterThanOrderPrice = 51049,
    /// 止盈触发价格应该大于卖一价
    OrderProfitTriggerPriceShouldBeGreaterThanAskPrice = 51050,
    /// 止损触发价格应该小于卖一价
    OrderStopLossTriggerPriceShouldBeLessThanAskPrice = 51051,
    /// 止盈触发价格应该小于买一价
    OrderProfitTriggerPriceShouldBeLessThanAskPrice = 51052,
    /// 止损触发价格应该大于买一价
    OrderStopLossTriggerPriceShouldBeGreaterThanAskPrice = 51053,
    /// 请求超时，请稍候重试
    OrderRequestTimeout = 51054,
    /// 组合保证金模式暂不支持合约网格
    OrderContractGridNotSupport = 51055,
    /// 当前策略不支持该操作
    OrderStrategyNotSupport = 51056,
    /// 当前账户模式暂不支持此交易策略，请前往"交易设置 > 账户模式"进行切换
    OrderAccountModeNotSupport = 51057,
    /// 该策略无仓位
    OrderStrategyNoPosition = 51058,
    /// 策略当前状态不支持此操作
    OrderStrategyStateNotSupport = 51059,
    /// algoClOrdId 重复
    OrderAlgoClOrdIdDuplicate = 51065,
    /// 期权交易不支持市价单，请用限价单平仓
    OrderOptionTransactionNotSupportMarketOrder = 51066,
    /// {param0} 已经在 algoClOrdId 和 attachAlgoClOrdId 中存在。
    OrderAlgoClOrdIdDuplicateParamInAttachAlgoClOrdIdWithAlgoClOrdId = 51068,
    /// 不存在该{param0}相关的期权合约
    OrderOptionContractNotFound = 51069,
    /// 您当前尚未达到升级至该账户模式的要求，请先在官方网站或APP完成账户模式的升级。
    OrderAccountModeUpgradeRequired = 51070,
    /// 当前维护的标签维度倒计时全部撤单达到数量上限
    OrderTagDimensionCancelAllExceeded = 51071,
    /// 您当前身份为现货带单员，设置的带单币对买入时，tdMode 需要使用 spot_isolated
    OrderSpotCopytradingBuyOnlySupportSpotIsolated = 51072,
    /// 您当前身份为现货带单员，卖出带单资产需要使用'/copytrading/close-subposition'接口
    OrderSpotCopytradingSellOnlySupportCloseSubposition = 51073,
    /// 仅现货带单员设置的带单币对支持使用 tdMode：spot_isolated
    OrderSpotCopytradingOnlySupportSpotIsolated = 51074,
    /// 现货跟单平仓单只支持修改价格，不支持修改数量
    OrderSpotCopytradingCloseOnlySupportModifyPrice = 51075,
    /// 分批止盈的每笔止盈止损订单仅支持单向止盈止损，slTriggerPx&slOrdPx 与 tpTriggerPx&tpOrdPx 只能填写一组
    OrderBatchProfitOnlySupportSingleDirection = 51077,
    /// 同一笔订单上附带分批止盈的止盈委托单不能超过 {param0} 笔
    OrderBatchProfitOrderLimitExceeded = 51079,
    /// 同一笔订单上附带分批止盈的止盈触发价类型 (tpTriggerPxType) 必须保持一致
    OrderBatchProfitTriggerPriceTypeMustBeConsistent = 51080,
    /// 同一笔订单上附带分批止盈的止盈触发价 (tpTriggerPx) 不能相等
    OrderBatchProfitTriggerPriceMustBeDifferent = 51081,
    /// 同一笔订单上附带分批止盈，其中触发止盈的止盈委托价 (tpOrdPx) 只能是市价
    OrderBatchProfitTriggerPriceMustBeMarket = 51082,
    /// 同一笔订单上附带分批止盈的止盈数量之和需要等于订单的委托数量
    OrderBatchProfitTotalAmountMustBeEqual = 51083,
    /// 同一笔订单上附带分批止盈的止损委托单不能超过 {param0} 笔
    OrderBatchStopLossOrderLimitExceeded = 51084,
    /// 附带止盈止损开启'开仓价止损'时 (amendPxOnTriggerType 设置为 1)，该笔订单上的止盈委托单必须大于等于 2 笔
    OrderBatchProfitTriggerPriceMustBeGreaterThanOrEqual = 51085,
    /// 同一笔订单上附带止盈止损委托单不能超过 {param0} 笔
    OrderBatchProfitStopLossOrderLimitExceeded = 51086,
    /// 若下单时使用了 attachAlgoOrds 参数，也需要使用 attachAlgoOrds 参数改单；若下单时没有使用 attachAlgoOrds 参数，则不支持使用 attachAlgoOrds 参数改单。
    OrderAttachAlgoOrdsNotSupport = 51538,
    /// 修改同一笔订单上分批止盈中的止盈止损订单时，attachAlgoId 或者 attachAlgoClOrdId 的值不能重复
    OrderBatchProfitAttachAlgoIdDuplicate = 51539,
    /// 改单失败，其中至少有一个附带的止盈止损订单不存在
    OrderBatchProfitStopLossOrderNotFound = 51527,
    /// 该币种取消上线，当前不支持交易
    OrderCurrencyCanceled = 51087,
    /// 对于同一个仓位，仅支持一笔全部平仓的止盈止损挂单
    OrderBatchProfitStopLossOrderLimitExceededForSamePosition = 51088,
    /// 在附带分批止盈时，止盈订单的数量不能为空
    OrderBatchProfitStopLossOrderAmountEmpty = 51089,
    /// 对于绑定了限价止盈的止损订单，不允许修改其委托数量
    OrderBatchProfitStopLossOrderAmountNotAllowed = 51090,
    /// 同一笔订单上附带分批止盈的止盈类型必须保持一致
    OrderBatchProfitStopLossOrderTypeMustBeConsistent = 51091,
    /// 同一笔订单上附带分批止盈的止盈委托价不能相等
    OrderBatchProfitStopLossOrderPriceMustBeDifferent = 51092,
    /// 同一笔订单上附带分批止盈，其中限价止盈的止盈委托价 (tpOrdPx) 不能为 –1 (市价)
    OrderBatchProfitStopLossOrderPriceMustBeMarket = 51093,
    /// 限价止盈时 cxlOnClosePos 需要为 true
    OrderBatchProfitStopLossOrderCxlOnClosePosMustBeTrue = 51096,
    /// 对于绑定了限价止盈的止损订单，不能添加新的止盈
    OrderBatchProfitStopLossOrderNotAllowed = 51098,
    /// 您当前身份为带单交易员，不支持下单限价止盈
    OrderBatchProfitStopLossOrderNotAllowedForCopytrading = 51099,
    /// 操作失败，{businessType}的当前交易品种下，所有合约累计挂单数量不能大于{maxNumberPerInstFamily}(单)。
    OrderMaxNumberPerInstFamily = 51103,
    /// 操作失败，{businessType}的当前交易品种下，所有合约累计挂单张数不能大于{maxSzPerInstFamily} (张)。
    OrderMaxSzPerInstFamily = 51104,
    /// 操作失败，当前合约的持仓张数和同方向挂单张数之和不能大于{maxPositionSzPerInstrument}(张)。
    OrderMaxPositionSzPerInstrument = 51105,
    /// 操作失败，{businessType}的当前交易品种下，所有合约累计持仓张数和同方向挂单张数之和不能大于{maxPostionSzPerInstFamily51106}(张)。
    OrderMaxPostionSzPerInstFamilyWithSameDirection = 51106,
    /// 操作失败，{businessType}的当前交易品种下，所有合约累计持仓张数和双向挂单张数之和不能大于{maxPostionSzPerInstFamily51107}(张)。
    OrderMaxPostionSzPerInstFamilyWithBothDirection = 51107,
    /// 持仓量超过市价全平最大限制
    OrderMarketCloseMaxLimit = 51108,
    /// 订单深度中无买一卖一价
    OrderOrderDepthNoAskPrice = 51109,
    /// 市价全平前请先撤销所有平仓单
    OrderMarketCloseBeforeCancelAll = 51115,
    /// 委托价格或触发价格超过{param0}
    OrderPriceOrTriggerPriceExceeded = 51116,
    /// 平仓单挂单单数超过限制
    OrderMarketCloseOrderLimitExceeded = 51117,
    /// 下单数量不足{param0}张
    OrderOrderAmountNotEnough = 51120,
    /// 下单张数应为一手张数的倍数
    OrderOrderAmountMustBeMultiple = 51121,
    /// 委托价格小于最小值{param0}
    OrderPriceLessThanMin = 51122,
    /// 最小价格增量为空
    OrderMinPriceIncrementEmpty = 51123,
    /// 价格发现期间您只可下限价单
    OrderPriceDiscoveryOnlySupportLimitOrder = 51124,
    /// 当前杠杆存在非只减仓挂单，请撤销所有非只减仓挂单后进行只减仓挂单
    OrderNonOnlyReduceOrderExist = 51125,
    /// 当前杠杆存在只减仓挂单，请撤销所有只减仓挂单后进行非只减仓挂单
    OrderOnlyReduceOrderExist = 51126,
    /// 仓位可用余额为0
    OrderPositionAvailableBalanceZero = 51127,
    /// 仓位正资产小于最小交易单位
    OrderPositionPositiveAssetLessThanMin = 51132,
    /// 跨币种全仓币币不支持只减仓功能
    OrderCrossCurrencyFullMarginNotSupportOnlyReduce = 51133,
    /// 平仓失败，您当前没有杠杆仓位，请关闭只减仓后继续
    OrderCloseFailedNoLeveragePosition = 51134,
    /// 您的平仓价格已触发限价，最高买入价格为{param0}
    OrderClosePriceTriggeredLimitPriceHigh = 51135,
    /// 您的平仓价格已触发限价，最低卖出价格为{param0}
    OrderClosePriceTriggeredLimitPriceLow = 51136,
    /// 买单最高价为 {param0}，请调低价格
    OrderBuyPriceTooHigh = 51137,
    /// 卖单最低价为 {param0}，请调高价格
    OrderSellPriceTooLow = 51138,
    /// 现货模式下币币不支持只减仓功能
    OrderSpotNotSupportOnlyReduce = 51139,
    /// 由于盘口卖单不足，下单失败，请稍后重试
    OrderMarketDepthSellNotEnough = 51140,
    /// 盘口无有效报价，用USDT模式下单无法成交，请尝试切换到币种模式
    OrderMarketDepthNoValidQuote = 51142,
    /// 兑换数量不足
    OrderConvertAmountNotEnough = 51143,
    /// 请使用 {param0} 进行下单
    OrderUseParam0ForOrder = 51144,
    /// 交易期权需要在交易账户资产总价值大于1万美元的前提下，开通期权交易服务
    OrderOptionTradeRequiresAccountValue = 51147,
    /// 下单失败，当前订单若下单成功会造成只减仓订单反向开仓，请撤销或修改原有挂单再进行下单
    OrderOnlyReduceOrderReverseOpen = 51148,
    /// 下单超时，请稍候重试
    OrderOrderTimeout = 51149,
    /// 交易数量或价格的精度超过限制
    OrderOrderAmountOrPricePrecisionExceeded = 51150,
    /// 一键借币模式下，不支持自动借币与自动还币和手动类型混合下单。
    OrderQuickMarginNotSupportMixedOrder = 51152,
    /// 无法在一键借币模式下手动借币，您输入的金额已超过可借上限
    OrderQuickMarginNotSupportManualBorrow = 51153,
    /// 无法手动归还一键借币模式下的借币，您输入的还币金额已超过该币种可用余额
    OrderQuickMarginNotSupportManualRepay = 51154,
    /// 由于您所在国家或地区的合规限制，您无法交易此币对或合约
    OrderNotAllowedByRegulation = 51155,
    /// 自主划转已不支持，请切换至一键借币模式下单 (isoMode=quick_margin)
    OrderQuickMarginNotSupportTransfer = 51158,
    /// 您当前身份为带单交易员，无法切换至组合保证金账户
    OrderCopytradingNotSupportSwitchToCombinedMargin = 51164,

    /// 下单失败，您没有当前合约对应方向的持仓，无法进行平仓或者减仓。
    OrderNoPositionForCloseOrReduce = 51169,
    /// 下单失败，只减仓下单方向不能与持仓方向相同
    OrderOnlyReduceOrderSameDirection = 51170,
    /// 改单失败，当前订单若改单成功会造成只减仓订单反向开仓，请撤销或修改原有挂单再进行改单
    OrderOnlyReduceOrderReverseOpenForModify = 51171,
    /// 无法市价全平，当前仓位暂无负债
    OrderMarketCloseNoDebt = 51173,
    /// 操作失败，当前 {param0} 的累计挂单数量已达上限 {param1} (单)
    OrderMaxNumberPerInstrument = 51174,
    /// 参数 {param0}、{param1} 和 {param2} 不能同时为空
    OrderParam0Param1Param2Empty = 51175,
    /// 参数 {param0}、{param1} 和 {param2} 只能填写一个
    OrderParam0Param1Param2OnlyOne = 51176,
    /// 当前期权订单的价格类型为{param0}，不支持修改{param1}
    OrderOptionPriceTypeNotSupportModify = 51177,
    /// 现货模式下，不支持使用{param0}进行期权下单。
    OrderOptionTradeNotSupportParam0 = 51179,
    /// {param0}的范围应为({param1}, {param2})
    OrderParam0Range = 51180,
    /// 使用{param0}下单，ordType 只能为限价单 (limit)
    OrderOrdTypeOnlyLimit = 51181,
    /// 当前账户期权价格类型 pxUsd 和 pxVol 的挂单数量之和，不能超过 {param0} 个
    OrderOptionPriceTypePxUsdAndPxVolOrderLimit = 51182,
    /// 输入IV值对应的 {param0} 期权价格超过最高买价 {param1} {param2}，请重新输入合理的IV值。
    // OrderOptionPriceTypePxUsdAndPxVolOrderLimitPxUsd = 51183,
    // /// 输入USD订单价格对应的 {param0} 期权价格超过最低卖价 {param1} {param2}，请重新输入合理的USD订单价格。
    // OrderOptionPriceTypePxUsdAndPxVolOrderLimitPxUsd = 51184,
    // /// 输入USD订单价格对应的 {param0} 期权价格超过最高买价 {param1} {param2}，请重新输入合理的USD订单价格。
    // OrderOptionPriceTypePxUsdAndPxVolOrderLimitPxUsd = 51185,
    // /// 在提前挂单期间，您只能下限价单。
    // OrderOptionPriceTypePxUsdAndPxVolOrderLimitPxUsd = 51186,
    // /// 在提前挂单开始后，您才能下限价单。
    // OrderOptionPriceTypePxUsdAndPxVolOrderLimit = 51187,
    // /// 市价委托单笔价值不能超过 1,000,000 USDT
    // OrderOptionPriceTypePxUsdAndPxVolOrderLimit = 51188,
    // /// 市价单下单数量超出最大值
    // OrderOptionPriceTypePxUsdAndPxVolOrderLimit = 51189,
    // /// 普通委托数量超出最大限制{param0}
    // OrderOptionPriceTypePxUsdAndPxVolOrderLimit = 51190,
    /// 分润策略仅支持策略停止时卖币或停止时全部平仓
    OnlySupportSellOrCloseAll = 51220,
    /// 请输入 0-30% 范围内的指定分润比例
    InputRange = 51221,
    /// 该策略不支持分润
    NotSupportFenrun = 51222,
    /// 当前状态您不可以进行分润带单
    NotSupportFenrunForCurrentStatus = 51223,
    /// 该币对不支持分润
    NotSupportFenrunForCurrencyPair = 51224,
    /// 分润跟单策略不支持手动立即触发策略
    NotSupportManualTrigger = 51225,
    /// 分润跟单策略不支持修改策略参数
    NotSupportModify = 51226,
    /// 策略委托价格不在正确范围内
    PriceNotInRange = 51250,
    /// 创建冰山委托时，策略委托类型错误
    TypeError = 51251,
    /// 策略委托数量不在正确范围内
    QuantityNotInRange = 51252,
    /// 冰山委托单笔均值错误
    IcebergValueError = 51254,
    /// 冰山委托单笔委托超限
    IcebergValueExceeded = 51255,
    /// 冰山委托深度错误
    IcebergDepthError = 51256,
    /// 跟踪委托回调服务错误，回调幅度限制为{min}<x<={max}%
    TrackingCallbackError = 51257,
    /// 跟踪委托失败，卖单激活价格需大于最新成交价格
    TrackingSellPriceError = 51258,
    /// 跟踪委托失败，买单激活价格需小于最新成交价格
    TrackingBuyPriceError = 51259,
    /// 每个用户最多可同时持有{param0}笔未成交的跟踪委托
    TrackingMax = 51260,
    /// 每个用户最多可同时持有{param0}笔未成交的止盈止损
    StopLossMax = 51261,
    /// 每个用户最多可同时持有{param0}笔未成交的冰山委托
    OrderProfitStrategyIcebergMax = 51262,
    /// 每个用户最多可同时持有{param0}笔未成交的时间加权单
    TwapMax = 51263,
    /// 时间加权单笔均值超限
    TwapValueExceeded = 51264,
    /// 时间加权单笔上限错误
    TwapMaxLimit = 51265,
    /// 时间加权扫单比例出错
    TwapScanRatioLimit = 51266,
    /// 时间加权扫单范围出错
    TwapScanRangeLimit = 51267,
    /// 时间加权委托间隔错误，应为{min}<=x<={max}
    TwapIntervalLimit = 51268,
    /// 时间加权委托深度限制为 0<x<=1%
    TwapDepthLimit = 51269,
    /// 时间加权委托失败，扫单比例应该为 0<x<=100%
    TwapScanRatioLimitForZeroToOneHundred = 51270,
    /// 时间加权委托失败，扫单范围应该为 0<x<=1%
    TwapScanRangeLimitForZeroToOne = 51271,
    /// 时间加权委托总量应为大于 0
    TwapTotalAmountLimit = 51272,
    /// 时间加权委托总数量需大于单笔上限
    TwapTotalQuantityLimit = 51273,
    /// 止盈止损市价单笔委托数量不能超过最大限制
    StopLossMarketQuantityLimit = 51274,
    /// 止盈止损市价单不能指定价格
    TakeProfitMarketPriceCanNotSpecifyPrice = 51275,
    /// 止盈触发价格不能大于最新成交价
    TakeProfitPriceMaxPriceError = 51276,
    /// 止损触发价格不能小于最新成交价
    StopLossPriceMinPriceError = 51277,
    /// 止盈触发价格不能小于最新成交价
    TakeProfitPriceMaxPriceErrorForTakeProfit = 51278,
    /// 止盈触发价格不能小于最新成交价
    TakeProfitPriceMinPriceErrorForTakeProfit = 51279,
    /// 止损触发价格不能大于最新成交价
    StopLossPriceMaxPriceErrorForStopLoss = 51280,
    /// 计划委托不支持使用tgtCcy参数
    OrderProfitStrategyNotSupportTgtCcy = 51281,
    /// 吃单价优于盘口的比例范围
    OrderProfitStrategyEatPriceRatioRange = 51282,
    /// 时间间隔的范围{param0}s~{param1}s
    TimeIntervalRange = 51283,
    /// 单笔数量的范围{param0}~{param1}
    SingleQuantityRange = 51284,
    /// 委托总量的范围{param0}~{param1}
    TotalAmountRange = 51285,
    /// 下单金额需大于等于{param0}
    OrderAmountGreaterThanOrEqualTo = 51286,
    /// 当前策略不支持此交易品种
    NotSupportThisInstrument = 51287,
    /// 策略正在停止中，请勿重复点击
    StrategyStopping = 51288,
    /// 策略配置不存在，请稍后再试
    StrategyConfigNotExist = 51289,
    /// 策略引擎正在升级，请稍后重试
    StrategyEngineUpgrading = 51290,
    /// 策略不存在或已停止
    StrategyNotExistOrStopped = 51291,
    /// 策略类型不存在
    StrategyTypeNotExist = 51292,
    /// 策略不存在
    StrategyNotExist = 51293,
    /// 该策略暂不能创建，请稍后再试
    StrategyNotCreate = 51294,
    /// PM账户不支持ordType为{param0}的策略委托单
    PMAccountNotSupportOrdType = 51295,
    /// 交割、永续合约的买卖模式下，不支持计划委托
    FuturesAndPerpetualNotSupportPlan = 51298,
    /// 策略委托失败，用户最多可持有{param0}笔该类型委托
    StrategyOrderMax = 51299,
    /// 止盈触发价格不能大于标记价格
    TakeProfitPriceMaxMarkPriceError = 51300,
    /// 止损触发价格不能小于标记价格
    StopLossPriceMinMarkPriceError = 51302,
    /// 止盈触发价格不能小于标记价格
    TakeProfitPriceMinPriceError = 51303,
    /// 止损触发价格不能大于标记价格
    StopLossPriceMaxPriceError = 51304,
    /// 止盈触发价格不能大于指数价格
    TakeProfitPriceMaxIndexPriceError = 51305,
    /// 逐仓自主划转保证金模式不支持ordType为iceberg、twap的策略委托单
    CrossMarginNotSupportIcebergTwap = 51310,
    /// 移动止盈止损委托失败，回调幅度限制为{min}<x<={max}
    MoveTakeProfitStopLossCallbackError = 51311,
    /// 移动止盈止损委托失败，委托数量范围{min}<x<={max}
    MoveTakeProfitStopLossQuantityRange = 51312,
    /// 逐仓自主划转模式不支持策略部分
    CrossMarginNotSupportStrategyPart = 51313,
    /// 币币杠杆不支持计划委托
    SpotNotSupportPlan = 51317,
    /// closeFraction 仅适用于交割合约和永续合约
    CloseFractionOnlyForFuturesAndPerpetual = 51327,
    /// closeFraction 仅适用于只减仓订单
    CloseFractionOnlyForOnlyReduce = 51328,
    /// closeFraction 仅适用于买卖模式
    CloseFractionOnlyForBuyAndSell = 51329,
    /// closeFraction 仅适用于止盈止损市价订单
    CloseFractionOnlyForTakeProfitStopLossMarket = 51330,
    /// closeFraction仅限于平仓单
    CloseFractionOnlyForClose = 51331,
    /// 组合保证金模式不支持closeFraction
    CrossMarginNotSupportCloseFraction = 51332,
    /// 止盈价格需小于区间最低价格
    TakeProfitPriceLessThanMinPrice = 51343,
    /// 止损价格需大于区间最高价格
    StopLossPriceGreaterThanMaxPrice = 51344,
    /// 策略类型不是网格策略
    StrategyTypeNotGrid = 51345,
    /// 最高价格不能低于最低价格
    MaxPriceLessThanMinPrice = 51346,
    /// 暂无可提取利润
    NoProfit = 51347,
    /// 止损价格需小于区间最低价格
    StopLossPriceLessThanMinPrice = 51348,
    /// 止盈价格需大于区间最高价格
    TakeProfitPriceGreaterThanMaxPrice = 51349,
    /// 暂无可推荐参数
    NoRecommendedParameters = 51350,
    /// 单格收益必须大于0
    SingleGridProfitGreaterThan0 = 51351,
    /// 币对数量范围{pairNum1} - {pairNum2}
    PairNumRange = 51352,
    /// 存在重复币对{existingPair}
    ExistingPair = 51353,
    /// 币对比例总和需等于100%
    PairRatioSumEqual100 = 51354,
    /// 时区范围 {timezone1} - {timezone2}
    TimezoneRange = 51357,
    /// 每个币种的投入金额需大于{amount}
    EachCoinInvestmentAmountGreaterThan = 51358,
    /// 暂不支持定投该币种{0}
    NotSupportInvestment = 51359,
    /// 杠杆倍数范围{0}~{1}
    LeverageRange = 51370,
    /// 市场行情不符合策略配置
    MarketNotMatchStrategy = 51380,
    /// 单网格利润率不在区间内
    SingleGridProfitRateNotInRange = 51381,
    /// 策略不支持停止信号触发
    StrategyNotSupportStopSignal = 51382,
    /// 最小价格必须小于最新成交价
    MinPriceLessThanLatestPrice = 51383,
    /// 信号触发价格必须大于最小价格
    SignalTriggerPriceGreaterThanMinPrice = 51384,
    /// 止盈价必须大于最小价格
    TakeProfitPriceGreaterThanMinPrice = 51385,
    /// 最小价格必须大于1/2最新成交价
    MinPriceGreaterThanHalfLatestPrice = 51386,
    /// 止损价格应小于无限网格的区间最低价
    StopLossPriceLessThanMinPriceForInfiniteGrid = 51387,
    /// 策略已在运行中
    StrategyRunning = 51388,
    /// 触发价格需大于止损价格
    TriggerPriceGreaterThanStopLossPrice = 51389,
    /// 止盈价格需大于触发价格
    TakeProfitPriceGreaterThanTriggerPrice = 51390,
    /// 止损价格需小于触发价格
    StopLossPriceLessThanTriggerPrice = 51391,
    /// 触发价格需大于止盈价格
    TriggerPriceGreaterThanTakeProfitPrice = 51392,
    /// 触发价格需小于止损价格
    TriggerPriceLessThanStopLossPrice = 51393,
    /// 触发价格需小于无限网格的区间最低价
    TriggerPriceLessThanStopLossPriceForInfiniteGrid = 51394,
    /// 止盈价格需小于触发价格
    TakeProfitPriceLessThanTriggerPrice = 51395,
    /// 止损价格需大于触发价格
    StopLossPriceGreaterThanTriggerPrice = 51396,
    /// 当前行情满足停止条件，无法创建策略
    CurrentMarketSatisfyStopCondition = 51397,
    /// 当前杠杆下最大可投入金额为 {amountLimit} {quoteCurrency}，请减少投入金额后再试。
    AmountLimit = 51398,
    /// 由于订单已完成、已撤销或不存在，撤单失败
    OrderCompletedOrCanceledOrNotExist = 51399,
    /// 撤单失败，订单不存在（仅适用于价差速递）
    OrderNotExist = 51400,
    /// 撤单失败，订单已撤销（仅适用于价差速递）
    OrderCanceled = 51401,
    /// 撤单失败，订单已完成（仅适用于价差速递）
    OrderCompleted = 51402,
    /// 撤单失败，该委托类型无法进行撤单操作
    OrderTypeNotSupportCancel = 51403,
    /// 价格发现第二阶段您不可撤单
    PriceDiscoverySecondStageNotSupportCancel = 51404,
    /// 撤单失败，您当前没有未成交的订单
    NoUnfilledOrder = 51405,
    /// 撤单数量超过最大允许单数{param0}
    CancelOrderMax = 51406,
    /// ordIds 和 clOrdIds 不能同时为空
    OrdIdsAndClOrdIdsNotBothEmpty = 51407,
    /// 币对 id 或币对名称与订单信息不匹配
    PairIdOrPairNameNotMatchOrderInfo = 51408,
    /// 币对 id 或币对名称不能同时为空
    PairIdOrPairNameNotBothEmpty = 51409,
    /// 撤单失败，订单已处于撤销中或结算中
    OrderCanceledOrSettling = 51410,
    /// 用户没有执行mass cancel的权限
    NoMassCancelPermission = 51411,
    /// 撤单超时，请稍后重试
    CancelTimeout = 51412,
    /// 委托已触发，暂不支持撤单
    OrderTriggeredNotSupportCancel = 51413,
    /// 撤单失败，接口不支持该委托类型的撤单
    OrderTypeNotSupportCancelForInterface = 51414,
    /// 下单失败，现货交易仅支持设置最新价为触发价格，请更改触发价格并重试
    SpotNotSupportLatestPrice = 51415,
    /// 委托已触发，暂不支持撤单
    OrderTriggeredNotSupportCancelForInterface = 51416,
    /// 价格、数量、止盈/止损不能同时为空
    PriceQuantityTakeProfitStopLossNotAllEmpty = 51417,
    /// 修改订单超过最大允许单数{param0}
    ModifyOrderMax = 51418,
    /// 修改订单失败，账户 {param0} 可用余额不足
    ModifyOrderFailedAccountBalanceNotEnough = 51419,
    /// 修改订单失败，账户 {param0} 可用保证金不足
    ModifyOrderFailedAccountMarginNotEnough = 51420,
    /// 修改订单失败，账户 {param0} 可用余额不足，且未开启自动借币
    ModifyOrderFailedAccountBalanceNotEnoughNotAutoBorrow = 51421,
    /// 修改订单失败，账户 {param0} 可用保证金不足，且未开启自动借币（PM模式也可以尝试IOC订单降低风险）
    ModifyOrderFailedAccountMarginNotEnoughNotAutoBorrow = 51422,
    /// 修改订单失败，因为 {param0} 剩余的限额（主账户限额+当前账户锁定的尊享借币额度）不足，导致可借不足（限价挂单以及当前下单需借 {param1}，剩余额度 {param2}，限额 {param3}，已用额度 {param4}。
    ModifyOrderFailedAccountBalanceNotEnoughByTierLimit = 51423,
    /// 修改订单失败，因为 {param0} 剩余的币对限额不足，导致可借不足
    ModifyOrderFailedAccountBalanceNotEnoughByCurrencyPairLimit = 51424,
    /// 修改订单失败，因为 {param0} 剩余的借贷池限额不足，导致可借不足
    ModifyOrderFailedAccountBalanceNotEnoughByBorrowPoolLimit = 51425,
    /// 修改订单失败，账户资产不足，美元层面有效保证金小于 IMR（PM模式也可以尝试IOC订单降低风险）
    ModifyOrderFailedAccountBalanceNotEnoughByImr = 51426,
    /// 修改订单失败，delta 校验未通过，因为若成功下单，adjEq 的变化值将小于 IMR 的变化值。建议增加 adjEq 或减少 IMR 占用（PM模式也可以尝试IOC订单降低
    ModifyOrderFailedDeltaCheck = 51427,
    /// {instId} 不处于集合竞价阶段
    InstIdNotInAuction = 51428,
    /// 订单类型不支持改单
    OrderTypeNotSupportModify = 51429,
    /// 您仅能在币种上线至少 5 分钟后进行市价委托
    SpotNotSupportLatestPriceForAuction = 51430,
    /// 集合竞价第一阶段和第二阶段不允许改单
    AuctionNotSupportModify = 51431,
    /// 修改订单失败,订单已撤销（仅适用于价差速递）
    ModifyOrderFailedOrderCanceled = 51432,
    /// 修改订单失败,订单已完成（仅适用于价差速递）
    ModifyOrderFailedOrderCompleted = 51433,
    /// 操作失败，订单价格不满足Post Only条件
    ModifyOrderFailedOrderPriceNotPostOnly = 51434,
    /// 批量修改订单失败。同一批量改单请求中不允许包含相同订单。
    ModifyOrderFailedBatch = 51435,
    /// 对于正在处理的同一订单，改单请求次数不得超过3次
    ModifyOrderFailedSameOrder = 51436,
    /// 修改订单失败，价格长度不能超过 32 个字符
    ModifyOrderFailedPriceLength = 51437,
    /// 改单失败，当前合约无持仓，无法修改只减仓订单
    ModifyOrderFailedNoPosition = 51438,
    /// 改单失败，止盈止损单不支持增加或删除止盈/止损
    ModifyOrderFailedTakeProfitStopLossNotSupportAddOrDelete = 51439,
    /// 改单失败，止盈止损订单不存在
    ModifyOrderFailedTakeProfitStopLossNotExist = 51440,
    /// 止盈止损不支持修改触发类型
    ModifyOrderFailedTakeProfitStopLossNotSupportModifyTriggerType = 51441,
    /// 改单失败，只有交割、永续合约单可以修改止盈止损
    ModifyOrderFailedTakeProfitStopLossNotSupportModify = 51442,
    /// 改单失败，只减仓订单不能附带止盈止损
    ModifyOrderFailedOnlyReduceOrdersNotSupportTakeProfitStopLoss = 51443,
    /// 改单失败，只有交割、永续合约单可以修改止盈止损
    ModifyOrderFailedOnlyDeliveryOrFutureContractNotSupportTakeProfitStopLoss = 51444,
    /// 改单失败，只减仓订单不能附带止盈止损
    ModifyOrderFailedFutureContractNotSupportTakeProfitStopLoss = 51445,
    /// 改单失败，止盈止损单修改必须保留一个方向
    ModifyOrderFailedTakeProfitStopLossModifyMustKeepOneDirection = 51446,
    /// 期权的 pxVol 或者 pxUsd 订单不支持修改订单数量
    ModifyOrderFailedPxVolOrPxUsdNotSupportModifyQuantity = 51447,
    /// 非期权产品不支持使用 pxUsd 或者 pxVol
    ModifyOrderFailedPxUsdOrPxVolNotSupport = 51448,
    /// 期权的 pxVol 或者 pxUsd 订单不支持修改订单数量
    ModifyOrderFailedPxVolOrPxUsdNotSupportModifyQuantityForOption = 51449,
    /// 非期权产品不支持使用 pxUsd 或者 pxVol
    ModifyOrderFailedPxUsdOrPxVolNotSupportForOption = 51450,
    /// 修改现货或杠杆的止盈止损订单时，仅支持调整价格和数量。如需其他操作，请撤单后重新下单。
    ModifyOrderFailedSpotOrLeverageTakeProfitStopLossNotSupportModify = 51451,
    /// 查询订单的状态不存在
    ModifyOrderFailedOrderStatusNotExist = 51452,
    /// 订单状态和订单id不能同时存在
    ModifyOrderFailedOrderStatusAndOrderIdNotBothEmpty = 51453,
    /// 订单状态或订单id必须存在一个
    ModifyOrderFailedOrderStatusOrOrderIdNotBothEmpty = 51454,
    /// 查询订单不存在
    ModifyOrderFailedOrderNotExist = 51455,
    /// 若想获取文件链接，请先申请下载文件
    ModifyOrderFailedGetFileLink = 51456,
    /// 只允许下载过去两年内的历史成交明细文件
    OnlyAllowDownloadHistoryTradeDetailFileInTheLastTwoYears = 51457,
    /// 无法下载当前季度的历史成交明细
    CannotDownloadHistoryTradeDetailFileOfTheCurrentQuarter = 51458,
    /// 您已申请下载文件，当前状态为进行中
    ModifyOrderFailedGetFileLinkStatus = 51459,
    /// 当前季度无历史成交明细
    NoHistoryTradeDetailFileOfTheCurrentQuarter = 51460,
    /// 只允许下载 2021 年第一季度以来的历史账单流水
    OnlyAllowDownloadHistoryTradeDetailFileSinceTheFirstQuarterOf2021 = 51461,
    /// 无法下载当前季度的账单流水
    CannotDownloadHistoryTradeDetailFileOfTheCurrentQuarterForBill = 51462,
    /// 您不是节点用户，没有相关权限
    YouAreNotANodeUserAndDoNotHaveTheRelevantPermissions = 51463,
    /// 该用户不是您的直客
    TheUserIsNotYourDirectCustomer = 51464,
    /// 请求失败
    RequestFailed = 51820,
    /// 该支付方式不支持
    ThePaymentMethodIsNotSupported = 51821,
    /// 超过询价有效期
    OverTheValidityPeriodOfTheQuotation = 51822,
    /// 买卖交易参数 {param} 与报价不一致
    TheTransactionParametersDoNotMatchTheQuotation = 51823,
    /// 您当前身份为带单交易员。在开平仓模式下，对于带单合约标的不支持使用该接口平仓
    YouAreAManagerAndDoNotSupportUsingThisInterfaceToClosePositions = 51156,
    /// 您当前身份为带单交易员，在买卖模式下，如需使用该接口下单，委托的方向必须与现有持仓和挂单保持一致
    YouAreAManagerAndDoNotSupportUsingThisInterfaceToPlaceOrders = 51159,
    /// 您当前有 {instrument} 挂单，请撤单后重试
    YouHaveOrdersPleaseCancelAndRetry = 51162,
    /// 您当前有 {instrument} 持仓，请平仓后重试
    YouHavePositionsPleaseCloseAndRetry = 51163,
    /// {instrument}只减仓订单数量已达上限 {upLimit}，请撤销部分订单后重新下单。
    TheNumberOfOnlyReduceOrdersHasReachedTheUpperLimitPleaseCancelSomeOrdersAndRetry = 51165,
    /// 当前产品不支持带单
    TheCurrentProductDoesNotSupportOrderTaking = 51166,
    /// 下单失败，因为您存在大宗交易的委托订单，请撤销后重新下单
    OrderFailedBecauseYouHaveADealOrderPleaseCancelAndRetry = 51167,
    /// 下单失败，因为您存在只减仓类型的委托订单，请撤销后重新下单
    OrderFailedBecauseYouHaveOnlyReduceOrdersPleaseCancelAndRetry = 51168,

    /// 币种占比范围 {PercentNum1}%-{PercentNum2}%
    TheRatioOfTheCurrencyIsRange = 51320,
    /// 您正在带单。暂不支持使用套利、冰山或时间加权 (TWAP) 策略带单
    YouAreAManagerAndDoNotSupportUsingThisInterfaceToOrderTaking = 51321,
    /// 您当前身份为带单交易员。您的带单合约持仓已经市价全平，系统已撤销止盈止损委托并进行平仓
    YourOrderHasBeenFullyClosedAtMarketPriceTheSystemHasRevokedTheStopLossOrderAndClosedThePosition =
        51322,
    /// 您当前身份为带单交易员。您的带单合约仓位已设置止盈止损，请先撤销原有止盈止损订单
    YourOrderHasBeenSetStopLossPleaseCancelTheOriginalStopLossOrderFirst = 51323,
    /// 您当前身份为带单交易员，并持有 {instrument} 仓位。平仓委托张数需要与可平张数一致
    YouAreAManagerAndHoldPositionsTheNumberOfOrdersToCloseMustBeEqualToTheNumberOfPositions = 51324,
    /// 您当前身份为带单交易员。下止盈止损单时，请选择市价作为委托价格
    YouAreAManagerAndDoNotSupportUsingThisInterfaceToPlaceStopLossOrder = 51325,
    /// 您当前身份为带单交易员，下止盈止损单时，委托价格类型必须为市价
    YouAreAManagerAndDoNotSupportPriceNeedMarketPrice = 51326,
    /// 暂不支持币币杠杆业务
    TheCurrentProductDoesNotSupportLeverage = 54000,
    /// 只有跨币种全仓账户才能设置自动借币
    OnlyCrossCurrencyFullMarginAccountsCanSetAutomaticBorrow = 54001,
    /// 下单或改单失败，因为批量订单中的一个订单失败了
    OrderOrModifyOrderFailedBecauseOneOfTheBatchOrdersFailed = 54004,
    /// 盘前交割合约请使用逐仓进行交易
    TheFrontEndDeliveryContractMustUseCrossMargin = 54005,
    /// 盘前交易合约用户持仓上限为{posLimit}张
    TheFrontEndTransactionContractUserPositionLimitIsLots = 54006,
    /// 不支持该产品
    ProductIsNotSupported = 54007,
    /// 该操作被"撤销 MMP 订单"接口限制。请通过该接口解除限制。
    TheOperationIsRestrictedByTheInterfaceOfRevokeMMPOrdersPleaseUnblockTheRestrictionThroughThisInterface =
        54008,
    /// {param0}的范围应为 [{param1}，{param2}]
    RangeOfShouldBeRange = 54009,
    /// 盘前交易合约交割前 1 小时内仅允许减少仓位数量，请修改或撤销订单
    TheFrontEndTransactionContractDeliveryOneHourBeforeTheDeliveryPleaseModifyOrCancelTheOrder =
        54011,
    /// 超出 {param0} 美元的买入量限额，余下限额为 {param1} 美元。（集合竞价期间）
    TheBuyAmountExceedsTheLimitOfDollarsDuringTheAuction = 54018,
    /// 超出 {param0} 美元的买入量限额，余下限额为 {param1} 美元。（集合后）
    TheBuyAmountExceedsTheLimitOfDollarsAfterTheAuction = 54019,
    /// 下单失败，在全仓模式下交易交割、永续合约和期权时需开启 {ccy} 质押
    TheBuyAmountExceedsTheLimitOfDollars,
    TheRemainingLimitIsDollars = 54024,
    /// 下单失败，在逐仓模式下交易杠杆、交割、永续合约和期权时需开启 {ccy} 质押
    OrderFailedInMarginModeNeedOpenZhiYin = 54025,
    /// 下单失败，在逐仓模式下交易杠杆币对时需开启 {ccy} 和 {ccy1} 质押
    OrderFailedInMarginModeNeedOpenZhiYinParam1Parma2 = 54026,
    /// 下单失败，交易期权时需开启 {ccy} 质押
    OrderFailedInQiQuanOpenZhiYinParam1 = 54027,
    /// 下单失败，在逐仓模式下交易现货需开启 {ccy} 质押
    OrderFailedInMarginModeNeedOpenZhiYinParam1 = 54028,
    /// {param0} 不存在于 {param1}
    TheParam0DoesNotExistInTheParam1 = 54029,
    /// 下单失败，您的 {param0} 相同交易方向的持仓和挂单总价值不可超过 {param1} 美元，或超过全平台总持仓量的 {param2}
    TheTotalValueOfYourParam0PositionsAndOrdersInTheSameDirectionCannotExceedParam1DollarsOrExceedParam2OfTheTotalPlatformPosition =
        54030,
    /// 下单失败，{param0} 合约已达到 {param1} 美元的持仓限额
    ThePositionLimitOfParam0ContractHasReachedParam1Dollars = 54031,
    /// 下单失败，当前已达到该币种的全平台质押上限，仅支持只减仓订单
    TheTotalPlatformMarginLimitOfParam0HasBeenReached,
    OnlySupportOnlyReduceOrders = 54035,
    /// STP mode 为 cancel both，不支持 FOK 订单
    TheStpModeIsCancelBothDoesNotSupportFokOrders = 54036,
    // 未知错误
    /// 未知错误
    Unknown = 99999,
}

impl fmt::Display for ApiErrorCode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?} ({})", self, *self as i32)
    }
}

impl ApiErrorCode {
    /// 从错误码获取ApiErrorCode枚举
    pub fn from_code(code: u32) -> Self {
        match code {
            0 => Self::Ok,
            1 => Self::OperationFailed,
            2 => Self::PartialSuccess,
            50000 => Self::EmptyBody,
            50001 => Self::ServiceUnavailable,
            50002 => Self::JsonSyntaxError,
            50004 => Self::RequestTimeout,
            50005 => Self::InterfaceDeprecated,
            50006 => Self::InvalidContentType,
            50007 => Self::UserFrozen,
            50008 => Self::UserNotFound,
            50009 => Self::UserMarginFrozen,
            50010 => Self::UserIdEmpty,
            50011 => Self::TooManyRequests,
            50012 => Self::InvalidAccountStatus,
            50013 => Self::SystemBusy,
            50014 => Self::RequiredParameterEmpty,
            50015 => Self::ParameterBothEmpty,
            50016 => Self::ParameterMismatch,
            50017 => Self::PositionAdlFrozen,
            50018 => Self::PositionAdlFrozenForParam,
            50019 => Self::AccountAdlFrozenForAccount,
            50020 => Self::PositionMarginFrozen,
            50021 => Self::PositionMarginFrozenForParam,
            50022 => Self::AccountMarginFrozen,
            50023 => Self::FundingFeeFrozen,
            50024 => Self::ParameterBothExist,
            50025 => Self::ParameterValueExceedLimit,
            50026 => Self::SystemError,
            50027 => Self::AccountTradingRestricted,
            50028 => Self::AccountException,
            50029 => Self::AccountRiskControl,
            50030 => Self::AccountPermissionDenied,
            50032 => Self::AccountCurrencyTradingRestricted,
            50033 => Self::AccountBusinessLineTradingRestricted,
            50035 => Self::ApiKeyIpBindingRequired,
            50036 => Self::ExpTimeTooEarly,
            50037 => Self::OrderExpired,
            50038 => Self::SimulatedTradingNotSupported,
            50039 => Self::TimestampBeforeNotSupported,
            50040 => Self::OperationTooFrequent,
            50041 => Self::UserIdNotInWhiteList,
            50042 => Self::RequestDuplicate,
            50044 => Self::BrokerTypeRequired,
            50045 => Self::SimPosShouldBeEmpty,
            50046 => Self::FunctionMaintenance,
            50047 => Self::KlineAlreadyDelivered,
            50048 => Self::HedgingUnitRiskLevelIncrease,
            50049 => Self::NoPositionLevelInfo,
            50050 => Self::OptionTradingServiceAlreadyActivated,
            50051 => Self::ComplianceRestrictionForCountry,
            50052 => Self::ComplianceRestrictionForCurrency,
            50053 => Self::SimulatedTradingOnly,
            50055 => Self::AssetResetFailedTooManyTimes,
            50056 => Self::OrderOrPositionExists,
            50057 => Self::AssetResetFailed,
            50058 => Self::AssetResetNotSupported,
            50059 => Self::ComplianceRestrictionNotMet,
            50060 => Self::IdentityVerificationRequired,
            50061 => Self::OrderRequestRateExceeded,
            50062 => Self::FunctionUnavailable,
            50063 => Self::ActivationFailed,
            50064 => Self::BorrowingSystemUnavailable,
            50067 => Self::CrossSiteTradingNotSupported,
            50069 => Self::RiskUnitMarginRateCheckFailed,
            50071 => Self::AlreadyExists,
            50100 => Self::ApiFrozen,
            50101 => Self::ApiKeyEnvironmentMismatch,
            50102 => Self::RequestTimestampExpired,
            50103 => Self::MissingOkAccessKey,
            50104 => Self::MissingOkAccessPassphrase,
            50105 => Self::InvalidOkAccessPassphrase,
            50106 => Self::MissingOkAccessSign,
            50107 => Self::MissingOkAccessTimestamp,
            50108 => Self::BrokerIdNotFound,
            50109 => Self::BrokerDomainNotFound,
            50110 => Self::IpNotInApiKeyBindingIpList,
            50111 => Self::InvalidOkAccessKey,
            50112 => Self::InvalidOkAccessTimestamp,
            50113 => Self::InvalidOkAccessSign,
            50114 => Self::InvalidOkAccessAuthorization,
            50115 => Self::InvalidRequestType,
            50116 => Self::FastApiCanOnlyCreateOneApiKey,
            50118 => Self::BrokerNeedToProvideIpToJoinWhiteList,
            50119 => Self::ApiKeyNotFound,
            50120 => Self::ApiKeyPermissionInsufficient,
            50121 => Self::IpAccessDenied,
            50122 => Self::OrderAmountMustBeGreaterThanMinimum,
            51000 => Self::InvalidParameter,
            51001 => Self::InstrumentIdOrSpreadIdNotFound,
            51002 => Self::TradeProductIdNotMatchIndex,
            51003 => Self::OrdIdOrClOrdIdAtLeastOne,
            51004 => Self::OrderFailedInMarginMode,
            51005 => Self::OrderQuantityGreaterThanSingleLimit,
            51006 => Self::OrderPriceNotInLimitRange,
            51007 => Self::OrderQuantityLessThanOne,
            51008 => Self::OrderFailedAccountBalanceNotEnough,
            51010 => Self::OrderFailedAccountBalanceNotEnoughAndAutoBorrowNotEnabledByImr,
            51011 => Self::OrderRepeat,
            51012 => Self::OrderCurrencyNotFound,
            51014 => Self::IndexNotFound,
            51015 => Self::NotMatchByInstIdAndInstType,
            51016 => Self::ClOrdIdRepeat,
            51017 => Self::OrderBorrowLimitExceeded,
            51018 => Self::OrderOptionAccountCannotHaveNetOpenShortPosition,
            51019 => Self::OrderOptionAccountCannotHaveNetOpenLongPosition,
            51020 => Self::OrderQuantityGreaterThanMinimumOrderQuantity,
            51021 => Self::OrderCurrencyOrContractPending,
            51022 => Self::OrderContractPaused,
            51023 => Self::OrderPositionNotFound,
            51024 => Self::OrderAccountFrozen,
            51025 => Self::OrderOrderQuantityExceeded,
            51026 => Self::OrderProductTypeMismatchIndex,
            51027 => Self::OrderContractExpired,
            51028 => Self::OrderContractDelivery,
            51029 => Self::OrderContractSettlement,
            51030 => Self::OrderFundingFeeSettlement,
            51031 => Self::OrderPriceOutOfLiquidationRange,
            51032 => Self::OrderMarketLiquidation,
            51033 => Self::OrderCurrencyPairTransactionLimitExceeded,
            51034 => Self::OrderTransactionRateExceeded,
            51035 => Self::OrderMarketOrderPermission,
            51036 => Self::OrderOptionAccountOnlySupportMMP,
            51037 => Self::OrderAccountRiskStateOnlySupportIOC,
            51038 => Self::OrderAccountRiskStateIOCAlreadyExists,
            51039 => Self::OrderContractAccountCannotAdjustLeverage,
            51040 => Self::OrderOptionAccountCannotAdjustMargin,
            51041 => Self::OrderOptionAccountOnlySupportBuyAndSell,
            51042 => Self::OrderOptionAccountOnlySupportFullMargin,
            51043 => Self::OrderMarginPositionNotFound,
            51044 => Self::OrderOrderTypeNotSupportStopLossAndProfit,
            51046 => Self::OrderProfitTriggerPriceShouldBeGreaterThanOrderPrice,
            51047 => Self::OrderStopLossTriggerPriceShouldBeLessThanOrderPrice,
            51048 => Self::OrderProfitTriggerPriceShouldBeLessThanOrderPrice,
            51049 => Self::OrderStopLossTriggerPriceShouldBeGreaterThanOrderPrice,
            51050 => Self::OrderProfitTriggerPriceShouldBeGreaterThanAskPrice,
            51051 => Self::OrderStopLossTriggerPriceShouldBeLessThanAskPrice,
            51052 => Self::OrderProfitTriggerPriceShouldBeLessThanAskPrice,
            51053 => Self::OrderStopLossTriggerPriceShouldBeGreaterThanAskPrice,
            51054 => Self::OrderRequestTimeout,
            51055 => Self::OrderContractGridNotSupport,
            51056 => Self::OrderStrategyNotSupport,
            51057 => Self::OrderAccountModeNotSupport,
            51058 => Self::OrderStrategyNoPosition,
            51059 => Self::OrderStrategyStateNotSupport,
            51065 => Self::OrderAlgoClOrdIdDuplicate,
            51066 => Self::OrderOptionTransactionNotSupportMarketOrder,
            51068 => Self::OrderAlgoClOrdIdDuplicateParamInAttachAlgoClOrdIdWithAlgoClOrdId,
            51069 => Self::OrderOptionContractNotFound,
            51070 => Self::OrderAccountModeUpgradeRequired,
            51071 => Self::OrderTagDimensionCancelAllExceeded,
            51072 => Self::OrderSpotCopytradingBuyOnlySupportSpotIsolated,
            51073 => Self::OrderSpotCopytradingSellOnlySupportCloseSubposition,
            51074 => Self::OrderSpotCopytradingOnlySupportSpotIsolated,
            51075 => Self::OrderSpotCopytradingCloseOnlySupportModifyPrice,
            51077 => Self::OrderBatchProfitOnlySupportSingleDirection,
            51079 => Self::OrderBatchProfitOrderLimitExceeded,
            51080 => Self::OrderBatchProfitTriggerPriceTypeMustBeConsistent,
            51081 => Self::OrderBatchProfitTriggerPriceMustBeDifferent,
            51082 => Self::OrderBatchProfitTriggerPriceMustBeMarket,
            51083 => Self::OrderBatchProfitTotalAmountMustBeEqual,
            51084 => Self::OrderBatchStopLossOrderLimitExceeded,
            51085 => Self::OrderBatchProfitTriggerPriceMustBeGreaterThanOrEqual,
            51086 => Self::OrderBatchProfitStopLossOrderLimitExceeded,
            51087 => Self::OrderCurrencyCanceled,
            51088 => Self::OrderBatchProfitStopLossOrderLimitExceededForSamePosition,
            51089 => Self::OrderBatchProfitStopLossOrderAmountEmpty,
            51090 => Self::OrderBatchProfitStopLossOrderAmountNotAllowed,
            51091 => Self::OrderBatchProfitStopLossOrderTypeMustBeConsistent,
            51092 => Self::OrderBatchProfitStopLossOrderPriceMustBeDifferent,
            51093 => Self::OrderBatchProfitStopLossOrderPriceMustBeMarket,
            51096 => Self::OrderBatchProfitStopLossOrderCxlOnClosePosMustBeTrue,
            51098 => Self::OrderBatchProfitStopLossOrderNotAllowed,
            51099 => Self::OrderBatchProfitStopLossOrderNotAllowedForCopytrading,
            51103 => Self::OrderMaxNumberPerInstFamily,
            51104 => Self::OrderMaxSzPerInstFamily,
            51105 => Self::OrderMaxPositionSzPerInstrument,
            51106 => Self::OrderMaxPostionSzPerInstFamilyWithSameDirection,
            51107 => Self::OrderMaxPostionSzPerInstFamilyWithBothDirection,
            51108 => Self::OrderMarketCloseMaxLimit,
            51109 => Self::OrderOrderDepthNoAskPrice,
            51115 => Self::OrderMarketCloseBeforeCancelAll,
            51116 => Self::OrderPriceOrTriggerPriceExceeded,
            51117 => Self::OrderMarketCloseOrderLimitExceeded,
            51120 => Self::OrderOrderAmountNotEnough,
            51121 => Self::OrderOrderAmountMustBeMultiple,
            51122 => Self::OrderPriceLessThanMin,
            51123 => Self::OrderMinPriceIncrementEmpty,
            51124 => Self::OrderPriceDiscoveryOnlySupportLimitOrder,
            51125 => Self::OrderNonOnlyReduceOrderExist,
            51126 => Self::OrderOnlyReduceOrderExist,
            51127 => Self::OrderPositionAvailableBalanceZero,
            51132 => Self::OrderPositionPositiveAssetLessThanMin,
            51133 => Self::OrderCrossCurrencyFullMarginNotSupportOnlyReduce,
            51134 => Self::OrderCloseFailedNoLeveragePosition,
            51135 => Self::OrderClosePriceTriggeredLimitPriceHigh,
            51136 => Self::OrderClosePriceTriggeredLimitPriceLow,
            51137 => Self::OrderBuyPriceTooHigh,
            51138 => Self::OrderSellPriceTooLow,
            51139 => Self::OrderSpotNotSupportOnlyReduce,
            51140 => Self::OrderMarketDepthSellNotEnough,
            51142 => Self::OrderMarketDepthNoValidQuote,
            51143 => Self::OrderConvertAmountNotEnough,
            51144 => Self::OrderUseParam0ForOrder,
            51147 => Self::OrderOptionTradeRequiresAccountValue,
            51148 => Self::OrderOnlyReduceOrderReverseOpen,
            51149 => Self::OrderOrderTimeout,
            51150 => Self::OrderOrderAmountOrPricePrecisionExceeded,
            51152 => Self::OrderQuickMarginNotSupportMixedOrder,
            51153 => Self::OrderQuickMarginNotSupportManualBorrow,
            51154 => Self::OrderQuickMarginNotSupportManualRepay,
            51155 => Self::OrderNotAllowedByRegulation,
            51156 => Self::YouAreAManagerAndDoNotSupportUsingThisInterfaceToClosePositions,
            51158 => Self::OrderQuickMarginNotSupportTransfer,
            51159 => Self::YouAreAManagerAndDoNotSupportUsingThisInterfaceToPlaceOrders,
            51162 => Self::YouHaveOrdersPleaseCancelAndRetry,
            51163 => Self::YouHavePositionsPleaseCloseAndRetry,
            51164 => Self::OrderCopytradingNotSupportSwitchToCombinedMargin,
            51165 => Self::TheNumberOfOnlyReduceOrdersHasReachedTheUpperLimitPleaseCancelSomeOrdersAndRetry,
            51166 => Self::TheCurrentProductDoesNotSupportOrderTaking,
            51167 => Self::OrderFailedBecauseYouHaveADealOrderPleaseCancelAndRetry,
            51168 => Self::OrderFailedBecauseYouHaveOnlyReduceOrdersPleaseCancelAndRetry,
            51169 => Self::OrderNoPositionForCloseOrReduce,
            51170 => Self::OrderOnlyReduceOrderSameDirection,
            51171 => Self::OrderOnlyReduceOrderReverseOpenForModify,
            51173 => Self::OrderMarketCloseNoDebt,
            51174 => Self::OrderMaxNumberPerInstrument,
            51175 => Self::OrderParam0Param1Param2Empty,
            51176 => Self::OrderParam0Param1Param2OnlyOne,
            51177 => Self::OrderOptionPriceTypeNotSupportModify,
            51179 => Self::OrderOptionTradeNotSupportParam0,
            51180 => Self::OrderParam0Range,
            51181 => Self::OrderOrdTypeOnlyLimit,
            51182 => Self::OrderOptionPriceTypePxUsdAndPxVolOrderLimit,
            51220 => Self::OnlySupportSellOrCloseAll,
            51221 => Self::InputRange,
            51222 => Self::NotSupportFenrun,
            51223 => Self::NotSupportFenrunForCurrentStatus,
            51224 => Self::NotSupportFenrunForCurrencyPair,
            51225 => Self::NotSupportManualTrigger,
            51226 => Self::NotSupportModify,
            51250 => Self::PriceNotInRange,
            51251 => Self::TypeError,
            51252 => Self::QuantityNotInRange,
            51254 => Self::IcebergValueError,
            51255 => Self::IcebergValueExceeded,
            51256 => Self::IcebergDepthError,
            51257 => Self::TrackingCallbackError,
            51258 => Self::TrackingSellPriceError,
            51259 => Self::TrackingBuyPriceError,
            51260 => Self::TrackingMax,
            51261 => Self::StopLossMax,
            51262 => Self::OrderProfitStrategyIcebergMax,
            51263 => Self::TwapMax,
            51264 => Self::TwapValueExceeded,
            51265 => Self::TwapMaxLimit,
            51266 => Self::TwapScanRatioLimit,
            51267 => Self::TwapScanRangeLimit,
            51268 => Self::TwapIntervalLimit,
            51269 => Self::TwapDepthLimit,
            51270 => Self::TwapScanRatioLimitForZeroToOneHundred,
            51271 => Self::TwapScanRangeLimitForZeroToOne,
            51272 => Self::TwapTotalAmountLimit,
            51273 => Self::TwapTotalQuantityLimit,
            51274 => Self::StopLossMarketQuantityLimit,
            51275 => Self::TakeProfitMarketPriceCanNotSpecifyPrice,
            51276 => Self::TakeProfitPriceMaxPriceError,
            51277 => Self::StopLossPriceMinPriceError,
            51278 => Self::TakeProfitPriceMaxPriceErrorForTakeProfit,
            51279 => Self::TakeProfitPriceMinPriceErrorForTakeProfit,
            51280 => Self::StopLossPriceMaxPriceErrorForStopLoss,
            51281 => Self::OrderProfitStrategyNotSupportTgtCcy,
            51282 => Self::OrderProfitStrategyEatPriceRatioRange,
            51283 => Self::TimeIntervalRange,
            51284 => Self::SingleQuantityRange,
            51285 => Self::TotalAmountRange,
            51286 => Self::OrderAmountGreaterThanOrEqualTo,
            51287 => Self::NotSupportThisInstrument,
            51288 => Self::StrategyStopping,
            51289 => Self::StrategyConfigNotExist,
            51290 => Self::StrategyEngineUpgrading,
            51291 => Self::StrategyNotExistOrStopped,
            51292 => Self::StrategyTypeNotExist,
            51293 => Self::StrategyNotExist,
            51294 => Self::StrategyNotCreate,
            51295 => Self::PMAccountNotSupportOrdType,
            51298 => Self::FuturesAndPerpetualNotSupportPlan,
            51299 => Self::StrategyOrderMax,
            51300 => Self::TakeProfitPriceMaxMarkPriceError,
            51302 => Self::StopLossPriceMinMarkPriceError,
            51303 => Self::TakeProfitPriceMinPriceError,
            51304 => Self::StopLossPriceMaxPriceError,
            51305 => Self::TakeProfitPriceMaxIndexPriceError,
            51310 => Self::CrossMarginNotSupportIcebergTwap,
            51311 => Self::MoveTakeProfitStopLossCallbackError,
            51312 => Self::MoveTakeProfitStopLossQuantityRange,
            51313 => Self::CrossMarginNotSupportStrategyPart,
            51317 => Self::SpotNotSupportPlan,
            51320 => Self::TheRatioOfTheCurrencyIsRange,
            51321 => Self::YouAreAManagerAndDoNotSupportUsingThisInterfaceToOrderTaking,
            51322 => Self::YourOrderHasBeenFullyClosedAtMarketPriceTheSystemHasRevokedTheStopLossOrderAndClosedThePosition,
            51323 => Self::YourOrderHasBeenSetStopLossPleaseCancelTheOriginalStopLossOrderFirst,
            51324 => Self::YouAreAManagerAndHoldPositionsTheNumberOfOrdersToCloseMustBeEqualToTheNumberOfPositions,
            51325 => Self::YouAreAManagerAndDoNotSupportUsingThisInterfaceToPlaceStopLossOrder,
            51326 => Self::YouAreAManagerAndDoNotSupportPriceNeedMarketPrice,
            51327 => Self::CloseFractionOnlyForFuturesAndPerpetual,
            51328 => Self::CloseFractionOnlyForOnlyReduce,
            51329 => Self::CloseFractionOnlyForBuyAndSell,
            51330 => Self::CloseFractionOnlyForTakeProfitStopLossMarket,
            51331 => Self::CloseFractionOnlyForClose,
            51332 => Self::CrossMarginNotSupportCloseFraction,
            51343 => Self::TakeProfitPriceLessThanMinPrice,
            51344 => Self::StopLossPriceGreaterThanMaxPrice,
            51345 => Self::StrategyTypeNotGrid,
            51346 => Self::MaxPriceLessThanMinPrice,
            51347 => Self::NoProfit,
            51348 => Self::StopLossPriceLessThanMinPrice,
            51349 => Self::TakeProfitPriceGreaterThanMaxPrice,
            51350 => Self::NoRecommendedParameters,
            51351 => Self::SingleGridProfitGreaterThan0,
            51352 => Self::PairNumRange,
            51353 => Self::ExistingPair,
            51354 => Self::PairRatioSumEqual100,
            51357 => Self::TimezoneRange,
            51358 => Self::EachCoinInvestmentAmountGreaterThan,
            51359 => Self::NotSupportInvestment,
            51370 => Self::LeverageRange,
            51380 => Self::MarketNotMatchStrategy,
            51381 => Self::SingleGridProfitRateNotInRange,
            51382 => Self::StrategyNotSupportStopSignal,
            51383 => Self::MinPriceLessThanLatestPrice,
            51384 => Self::SignalTriggerPriceGreaterThanMinPrice,
            51385 => Self::TakeProfitPriceGreaterThanMinPrice,
            51386 => Self::MinPriceGreaterThanHalfLatestPrice,
            51387 => Self::StopLossPriceLessThanMinPriceForInfiniteGrid,
            51388 => Self::StrategyRunning,
            51389 => Self::TriggerPriceGreaterThanStopLossPrice,
            51390 => Self::TakeProfitPriceGreaterThanTriggerPrice,
            51391 => Self::StopLossPriceLessThanTriggerPrice,
            51392 => Self::TriggerPriceGreaterThanTakeProfitPrice,
            51393 => Self::TriggerPriceLessThanStopLossPrice,
            51394 => Self::TriggerPriceLessThanStopLossPriceForInfiniteGrid,
            51395 => Self::TakeProfitPriceLessThanTriggerPrice,
            51396 => Self::StopLossPriceGreaterThanTriggerPrice,
            51397 => Self::CurrentMarketSatisfyStopCondition,
            51398 => Self::AmountLimit,
            51399 => Self::OrderCompletedOrCanceledOrNotExist,
            51400 => Self::OrderNotExist,
            51401 => Self::OrderCanceled,
            51402 => Self::OrderCompleted,
            51403 => Self::OrderTypeNotSupportCancel,
            51404 => Self::PriceDiscoverySecondStageNotSupportCancel,
            51405 => Self::NoUnfilledOrder,
            51406 => Self::CancelOrderMax,
            51407 => Self::OrdIdsAndClOrdIdsNotBothEmpty,
            51408 => Self::PairIdOrPairNameNotMatchOrderInfo,
            51409 => Self::PairIdOrPairNameNotBothEmpty,
            51410 => Self::OrderCanceledOrSettling,
            51411 => Self::NoMassCancelPermission,
            51412 => Self::CancelTimeout,
            51413 => Self::OrderTriggeredNotSupportCancel,
            51414 => Self::OrderTypeNotSupportCancelForInterface,
            51415 => Self::SpotNotSupportLatestPrice,
            51416 => Self::OrderTriggeredNotSupportCancelForInterface,
            51417 => Self::PriceQuantityTakeProfitStopLossNotAllEmpty,
            51418 => Self::ModifyOrderMax,
            51419 => Self::ModifyOrderFailedAccountBalanceNotEnough,
            51420 => Self::ModifyOrderFailedAccountMarginNotEnough,
            51421 => Self::ModifyOrderFailedAccountBalanceNotEnoughNotAutoBorrow,
            51422 => Self::ModifyOrderFailedAccountMarginNotEnoughNotAutoBorrow,
            51423 => Self::ModifyOrderFailedAccountBalanceNotEnoughByTierLimit,
            51424 => Self::ModifyOrderFailedAccountBalanceNotEnoughByCurrencyPairLimit,
            51425 => Self::ModifyOrderFailedAccountBalanceNotEnoughByBorrowPoolLimit,
            51426 => Self::ModifyOrderFailedAccountBalanceNotEnoughByImr,
            51427 => Self::ModifyOrderFailedDeltaCheck,
            51428 => Self::InstIdNotInAuction,
            51429 => Self::OrderTypeNotSupportModify,
            51430 => Self::SpotNotSupportLatestPriceForAuction,
            51431 => Self::AuctionNotSupportModify,
            51432 => Self::ModifyOrderFailedOrderCanceled,
            51433 => Self::ModifyOrderFailedOrderCompleted,
            51434 => Self::ModifyOrderFailedOrderPriceNotPostOnly,
            51435 => Self::ModifyOrderFailedBatch,
            51436 => Self::ModifyOrderFailedSameOrder,
            51437 => Self::ModifyOrderFailedPriceLength,
            51438 => Self::ModifyOrderFailedNoPosition,
            51439 => Self::ModifyOrderFailedTakeProfitStopLossNotSupportAddOrDelete,
            51440 => Self::ModifyOrderFailedTakeProfitStopLossNotExist,
            51441 => Self::ModifyOrderFailedTakeProfitStopLossNotSupportModifyTriggerType,
            51442 => Self::ModifyOrderFailedTakeProfitStopLossNotSupportModify,
            51443 => Self::ModifyOrderFailedOnlyReduceOrdersNotSupportTakeProfitStopLoss,
            51444 => Self::ModifyOrderFailedOnlyDeliveryOrFutureContractNotSupportTakeProfitStopLoss,
            51445 => Self::ModifyOrderFailedFutureContractNotSupportTakeProfitStopLoss,
            51446 => Self::ModifyOrderFailedTakeProfitStopLossModifyMustKeepOneDirection,
            51447 => Self::ModifyOrderFailedPxVolOrPxUsdNotSupportModifyQuantity,
            51448 => Self::ModifyOrderFailedPxUsdOrPxVolNotSupport,
            51449 => Self::ModifyOrderFailedPxVolOrPxUsdNotSupportModifyQuantityForOption,
            51450 => Self::ModifyOrderFailedPxUsdOrPxVolNotSupportForOption,
            51451 => Self::ModifyOrderFailedSpotOrLeverageTakeProfitStopLossNotSupportModify,
            51452 => Self::ModifyOrderFailedOrderStatusNotExist,
            51453 => Self::ModifyOrderFailedOrderStatusAndOrderIdNotBothEmpty,
            51454 => Self::ModifyOrderFailedOrderStatusOrOrderIdNotBothEmpty,
            51455 => Self::ModifyOrderFailedOrderNotExist,
            51456 => Self::ModifyOrderFailedGetFileLink,
            51457 => Self::OnlyAllowDownloadHistoryTradeDetailFileInTheLastTwoYears,
            51458 => Self::CannotDownloadHistoryTradeDetailFileOfTheCurrentQuarter,
            51459 => Self::ModifyOrderFailedGetFileLinkStatus,
            51460 => Self::NoHistoryTradeDetailFileOfTheCurrentQuarter,
            51461 => Self::OnlyAllowDownloadHistoryTradeDetailFileSinceTheFirstQuarterOf2021,
            51462 => Self::CannotDownloadHistoryTradeDetailFileOfTheCurrentQuarterForBill,
            51463 => Self::YouAreNotANodeUserAndDoNotHaveTheRelevantPermissions,
            51464 => Self::TheUserIsNotYourDirectCustomer,
            51538 => Self::OrderAttachAlgoOrdsNotSupport,
            51539 => Self::OrderBatchProfitAttachAlgoIdDuplicate,
            51527 => Self::OrderBatchProfitStopLossOrderNotFound,
            51820 => Self::RequestFailed,
            51821 => Self::ThePaymentMethodIsNotSupported,
            51822 => Self::OverTheValidityPeriodOfTheQuotation,
            51823 => Self::TheTransactionParametersDoNotMatchTheQuotation,
            54000 => Self::TheCurrentProductDoesNotSupportLeverage,
            54001 => Self::OnlyCrossCurrencyFullMarginAccountsCanSetAutomaticBorrow,
            54004 => Self::OrderOrModifyOrderFailedBecauseOneOfTheBatchOrdersFailed,
            54005 => Self::TheFrontEndDeliveryContractMustUseCrossMargin,
            54006 => Self::TheFrontEndTransactionContractUserPositionLimitIsLots,
            54007 => Self::ProductIsNotSupported,
            54008 => Self::TheOperationIsRestrictedByTheInterfaceOfRevokeMMPOrdersPleaseUnblockTheRestrictionThroughThisInterface,
            54009 => Self::RangeOfShouldBeRange,
            54011 => Self::TheFrontEndTransactionContractDeliveryOneHourBeforeTheDeliveryPleaseModifyOrCancelTheOrder,
            54018 => Self::TheBuyAmountExceedsTheLimitOfDollarsDuringTheAuction,
            54019 => Self::TheBuyAmountExceedsTheLimitOfDollarsAfterTheAuction,
            54024 => Self::TheBuyAmountExceedsTheLimitOfDollars,
            54025 => Self::OrderFailedInMarginModeNeedOpenZhiYin,
            54026 => Self::OrderFailedInMarginModeNeedOpenZhiYinParam1Parma2,
            54027 => Self::OrderFailedInQiQuanOpenZhiYinParam1,
            54028 => Self::OrderFailedInMarginModeNeedOpenZhiYinParam1,
            54029 => Self::TheParam0DoesNotExistInTheParam1,
            54030 => Self::TheTotalValueOfYourParam0PositionsAndOrdersInTheSameDirectionCannotExceedParam1DollarsOrExceedParam2OfTheTotalPlatformPosition,
            54031 => Self::ThePositionLimitOfParam0ContractHasReachedParam1Dollars,
            54035 => Self::TheTotalPlatformMarginLimitOfParam0HasBeenReached,
            54036 => Self::TheStpModeIsCancelBothDoesNotSupportFokOrders,
            59509 => Self::OrderMarketMakerProtectionResetPermission,
            510041 => Self::OrderFailedInMarginModeForBusinessType,
            510042 => Self::ModifyOrderFailedInMarginMode,
            510043 => Self::ModifyOrderFailedInMarginModeForBusinessTypeBuy,
            510044 => Self::ModifyOrderFailedInMarginModeForBusinessTypeSell,
            _ => Self::Unknown,
        }
    }

    /// 获取错误码对应的数值
    pub fn code(&self) -> u32 {
        *self as u32
    }

    /// 判断是否为成功状态
    pub fn is_success(&self) -> bool {
        matches!(self, Self::Ok)
    }

    /// 判断是否为部分成功状态
    pub fn is_partial_success(&self) -> bool {
        matches!(self, Self::PartialSuccess)
    }

    /// 判断是否为失败状态
    pub fn is_failure(&self) -> bool {
        !self.is_success() && !self.is_partial_success()
    }
}

/// 把任何错误转换为Error类型的结果
pub fn to_err<E: std::error::Error + Send + Sync + 'static>(err: E) -> Error {
    Error::Unknown(err.to_string())
}
