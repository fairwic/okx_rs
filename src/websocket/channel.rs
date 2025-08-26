use std::collections::HashMap;

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
    pub fn as_str(&self) -> &str {
        match self {
            Self::Tickers => "tickers",
            Self::Candle(interval) => interval,
            Self::Books => "books",
            Self::Books50L => "books-l2-tbt",
            Self::BooksLite => "books5",
            Self::Trades => "trades",
            Self::Account => "account",
            Self::Positions => "positions",
            Self::Orders => "orders",
            Self::AlgoOrders => "orders-algo",
            Self::AdvancedAlgoOrders => "algo-advance",
            Self::OrdersAlgo => "trades",
            Self::Balance => "balance_and_position",
            Self::PositionRisk => "positions-risk",
            Self::BalanceAndPosition => "balance_and_position",
            Self::Greeks => "greeks",
            Self::DepositInfo => "deposit-info",
            Self::Status => "status",
            Self::FundingRate => "funding-rate",
            Self::IndexCandle(interval) => interval,
            Self::IndexTickers => "index-tickers",
            Self::MarkPriceCandle(interval) => interval,
            Self::MarkPrice => "mark-price",
            Self::PriceLimit => "price-limit",
            Self::EstimatedPrice => "estimated-price",
            Self::BlockTickers => "block-tickers",
            Self::Custom(name) => name,
        }
    }
}
