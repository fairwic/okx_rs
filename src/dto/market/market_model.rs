use serde::{Deserialize, Serialize};

/// 行情数据
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Ticker {
    /// 产品ID
    #[serde(rename = "instId")]
    pub inst_id: String,
    /// 最新成交价
    pub last: String,
    /// 最新成交的数量
    #[serde(rename = "lastSz")]
    pub last_size: String,
    /// 买一价
    #[serde(rename = "bidPx")]
    pub bid_price: String,
    /// 买一价的挂单数数量
    #[serde(rename = "bidSz")]
    pub bid_size: String,
    /// 卖一价
    #[serde(rename = "askPx")]
    pub ask_price: String,
    /// 卖一价的挂单数量
    #[serde(rename = "askSz")]
    pub ask_size: String,
    /// 24小时开盘价
    pub open24h: String,
    /// 24小时最高价
    pub high24h: String,
    /// 24小时最低价
    pub low24h: String,
    /// 持仓量
    #[serde(rename = "volCcy24h")]
    pub volume_currency_24h: String,
    /// 成交量
    pub vol24h: String,
    /// 时间戳
    pub ts: String,
}

/// K线数据
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Candle {
    /// 开盘时间
    pub ts: String,
    /// 开盘价格
    pub open: String,
    /// 最高价格
    pub high: String,
    /// 最低价格
    pub low: String,
    /// 收盘价格
    pub close: String,
    /// 成交量
    pub vol: String,
    /// 成交量，以货币计量
    #[serde(rename = "volCcy")]
    pub vol_currency: String,
}

/// 深度数据
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Depth {
    /// 产品ID
    #[serde(rename = "instId")]
    pub inst_id: String,
    /// 卖方深度
    pub asks: Vec<Vec<String>>,
    /// 买方深度
    pub bids: Vec<Vec<String>>,
    /// 时间戳
    pub ts: String,
}

/// 交易对详情
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Instrument {
    /// 产品类型
    #[serde(rename = "instType")]
    pub inst_type: String,
    /// 产品ID
    #[serde(rename = "instId")]
    pub inst_id: String,
    /// 标的指数
    #[serde(rename = "uly", skip_serializing_if = "Option::is_none")]
    pub underlying: Option<String>,
    /// 交易货币币种
    #[serde(rename = "baseCcy", skip_serializing_if = "Option::is_none")]
    pub base_currency: Option<String>,
    /// 计价货币币种
    #[serde(rename = "quoteCcy", skip_serializing_if = "Option::is_none")]
    pub quote_currency: Option<String>,
    /// 下单价格精度
    #[serde(rename = "tickSz")]
    pub tick_size: String,
    /// 下单数量精度
    #[serde(rename = "lotSz")]
    pub lot_size: String,
    /// 最小下单数量
    #[serde(rename = "minSz")]
    pub min_size: String,
    /// 产品状态
    pub state: String,
} 