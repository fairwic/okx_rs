use crate::dto::market_dto::TickerOkxResDto;
use serde::{Deserialize, Serialize};
// {"arg":{"channel":"candle1H","instId":"BTC-USDT-SWAP"},"data":[["1747141200000","103644.1","103700","103629.2","103700","11316.08","113.1608","11731625.855","0"]]}
#[derive(Serialize, Deserialize, Debug)]
pub struct CandleArg {
    pub channel: String,
    #[serde(rename = "instId")]
    pub inst_id: String,
}
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct CandleOkxWsResDto {
    #[serde(rename = "arg")]
    pub arg: CandleArg,
    #[serde(rename = "data")]
    pub data: Vec<Vec<String>>,
}

// {"arg":{"channel":"tickers","instId":"BTC-USDT-SWAP"},"data":[{"instType":"SWAP","instId":"BTC-USDT-SWAP","last":"103700","lastSz":"0.01","askPx":"103700","askSz":"1004.09","bidPx":"103699.9","bidSz":"299.02","open24h":"103897.5","high24h":"104628.1","low24h":"100700","sodUtc0":"102740.1","sodUtc8":"102480.1","volCcy24h":"109187.9625","vol24h":"10918796.25","ts":"1747141319306"}]}
#[derive(Serialize, Deserialize, Debug)]
pub struct TickerArgData {
    #[serde(rename = "instId")]
    pub inst_id: String,
    pub channel: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TickerOkxResWsDto {
    #[serde(rename = "arg")]
    pub arg: TickerArgData,
    #[serde(rename = "data")]
    pub data: Vec<TickerOkxResDto>,
}

// Object {"code": String("60012"), "connId": String("5c13bf3a"), "event": String("error"), "msg": String("Illegal request: {\"ping\":1747191152912}")}
#[derive(Serialize, Deserialize, Debug)]
pub struct CommonOkxWsResDto {
    pub code: String,
    #[serde(rename = "connId")]
    pub conn_id: String,
    pub event: String,
    pub msg: String,
}
