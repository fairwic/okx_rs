use std::collections::HashMap;
use std::borrow::Cow;

/// WebSocket通道类型
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ChannelType {
    /// 产品行情频道
    Tickers,
    /// 产品K线频道
    Candle(String),
    /// 产品深度频道
    Books,
    /// 产品成交频道
    Trades,
    /// 账户频道
    Account,
    /// 持仓频道
    Positions,
    /// 订单频道
    Orders,
    /// 算法订单频道
    AlgoOrders,
    /// 高级算法订单频道
    AdvancedAlgoOrders,
    /// 用户交易频道
    OrdersAlgo,
    /// 资金频道
    Balance,
    /// 持仓风险频道
    PositionRisk,
    /// 账户余额和持仓频道
    BalanceAndPosition,
    /// 希腊字母频道
    Greeks,
    /// 存款账户信息频道
    DepositInfo,
    /// 系统状态频道
    Status,
    /// 平台公共资金费率频道
    FundingRate,
    /// 指数K线频道
    IndexCandle(String),
    /// 指数行情频道
    IndexTickers,
    /// 标记价格K线频道
    MarkPriceCandle(String),
    /// 标记价格频道
    MarkPrice,
    /// 限价频道
    PriceLimit,
    /// 估算交割/行权价格频道
    EstimatedPrice,
    /// 平台公共5档深度频道
    BooksLite,
    /// 平台公共200档深度频道
    Books50L,
    /// 大宗交易行情频道
    BlockTickers,
    /// 自定义频道
    Custom(String),
}

/// 通道参数
#[derive(Debug, Clone, Default)]
pub struct Args {
    /// 产品ID
    pub inst_id: Option<String>,
    /// 通道参数
    pub params: HashMap<String, String>,
}

impl Args {
    /// 创建新的参数
    pub fn new() -> Self {
        Self {
            inst_id: None,
            params: HashMap::new(),
        }
    }

    /// 设置产品ID
    pub fn with_inst_id(mut self, inst_id: impl Into<String>) -> Self {
        self.inst_id = Some(inst_id.into());
        self
    }

    /// 添加参数
    pub fn with_param(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.params.insert(key.into(), value.into());
        self
    }
}

impl ChannelType {
    /// 获取通道名称
    pub fn as_str(&self) -> Cow<'_, str> {
        match self {
            Self::Tickers => Cow::Borrowed("tickers"),
            Self::Candle(interval) => Cow::Owned(format!("candle{}", interval)),
            Self::Books => Cow::Borrowed("books"),
            Self::Books50L => Cow::Borrowed("books-l2-tbt"),
            Self::BooksLite => Cow::Borrowed("books5"),
            Self::Trades => Cow::Borrowed("trades"),
            Self::Account => Cow::Borrowed("account"),
            Self::Positions => Cow::Borrowed("positions"),
            Self::Orders => Cow::Borrowed("orders"),
            Self::AlgoOrders => Cow::Borrowed("orders-algo"),
            Self::AdvancedAlgoOrders => Cow::Borrowed("algo-advance"),
            Self::OrdersAlgo => Cow::Borrowed("trades"),
            Self::Balance => Cow::Borrowed("balance_and_position"),
            Self::PositionRisk => Cow::Borrowed("positions-risk"),
            Self::BalanceAndPosition => Cow::Borrowed("balance_and_position"),
            Self::Greeks => Cow::Borrowed("greeks"),
            Self::DepositInfo => Cow::Borrowed("deposit-info"),
            Self::Status => Cow::Borrowed("status"),
            Self::FundingRate => Cow::Borrowed("funding-rate"),
            Self::IndexCandle(interval) => Cow::Borrowed(interval),
            Self::IndexTickers => Cow::Borrowed("index-tickers"),
            Self::MarkPriceCandle(interval) => Cow::Borrowed(interval),
            Self::MarkPrice => Cow::Borrowed("mark-price"),
            Self::PriceLimit => Cow::Borrowed("price-limit"),
            Self::EstimatedPrice => Cow::Borrowed("estimated-price"),
            Self::BlockTickers => Cow::Borrowed("block-tickers"),
            Self::Custom(name) => Cow::Borrowed(name),
        }
    }
}
