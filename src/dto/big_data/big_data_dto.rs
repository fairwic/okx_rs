/*大数据API所需的数据模型*/
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct SupportCoin {
    pub contract: Vec<String>,
    pub option: Vec<String>,
    pub spot: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct TakerVolumeArr {
    // 返回值数组顺序分别为是：[ts,sellVol,buyVol]
    pub arr: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LongShortAccountRatioArr {
    pub ts: String,
    pub long_short_acct_ratio: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LongShortPositionRatioArr {
    pub ts: String,
    pub long_short_acct_ratio: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct TakerVolume {
    pub ts: String,       // 时间戳
    pub sell_vol: String, // 卖出量
    pub buy_vol: String,  // 买入量
}
