use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct TickerOkxResDto {
    #[serde(rename = "instType")]
    pub inst_type: String,
    #[serde(rename = "instId")]
    pub inst_id: String,
    pub last: String,
    #[serde(rename = "lastSz")]
    pub last_sz: String,
    #[serde(rename = "askPx")]
    pub ask_px: String,
    #[serde(rename = "askSz")]
    pub ask_sz: String,
    #[serde(rename = "bidPx")]
    pub bid_px: String,
    #[serde(rename = "bidSz")]
    pub bid_sz: String,
    pub open24h: String,
    pub high24h: String,
    pub low24h: String,
    #[serde(rename = "volCcy24h")]
    pub vol_ccy24h: String,
    pub vol24h: String,
    #[serde(rename = "sodUtc0")]
    pub sod_utc0: String,
    #[serde(rename = "sodUtc8")]
    pub sod_utc8: String,
    pub ts: String,
}

/// K线数据
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CandleOkxRespDto {
    /// 开盘时间
    pub ts: String,
    /// 开盘价格
    pub o: String,
    /// 最高价格
    pub h: String,
    /// 最低价格
    pub l: String,
    /// 收盘价格
    pub c: String,
    /// 成交量
    pub v: String,
    /// 成交量，以货币计量
    pub vol_ccy: String,
    /// 成交量，以货币计量
    pub vol_ccy_quote: String,
    /// 是否已确认
    pub confirm: String,
}

impl CandleOkxRespDto {
    pub fn from_vec(v: Vec<String>) -> Self {
        // 这里请根据你的结构体字段实际情况进行赋值
        // 例如：
        Self {
            ts: v[0].clone(),
            o: v[1].clone(),
            h: v[2].clone(),
            l: v[3].clone(),
            c: v[4].clone(),
            v: v[5].clone(),
            vol_ccy: v[6].clone(),
            vol_ccy_quote: v[7].clone(),
            confirm: v[8].clone(),
        }
    }
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
pub struct InstrumentOkxResDto {
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

/// 成交数据 (Trade)
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TradeOkxResDto {
    /// 产品ID
    #[serde(rename = "instId")]
    pub inst_id: String,
    /// 成交ID
    #[serde(rename = "tradeId")]
    pub trade_id: String,
    /// 成交价格
    #[serde(rename = "px")]
    pub px: String,
    /// 成交数量
    #[serde(rename = "sz")]
    pub sz: String,
    /// 成交方向 (buy, sell)
    pub side: String,
    /// 成交时间
    pub ts: String,
}
