use serde::{Deserialize, Serialize};

/// 系统时间信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SystemTime {
    /// 系统时间戳（Unix时间戳，以毫秒为单位）
    pub ts: String,
}

/// 系统状态
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SystemStatus {
    /// 系统维护计划的标题
    pub title: String,
    /// 系统状态
    pub state: String,
    /// 系统维护开始时间（以毫秒为单位）
    pub begin: Option<String>,
    /// 系统维护结束时间（以毫秒为单位）
    pub end: Option<String>,
    /// 系统维护的详细信息
    pub href: Option<String>,
    /// 服务类型
    #[serde(rename = "serviceType")]
    pub service_type: String,
    /// 系统维护计划ID
    pub system: Option<String>,
    /// 维护公告的详细信息
    #[serde(rename = "scheDesc")]
    pub schedule_description: Option<String>,
}

/// 经济日历事件
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EconomicEvent {
    /// 计划发布时间，Unix时间戳的毫秒数格式
    pub time: String,
    /// 经济日历事件的区域
    pub region: String,
    /// 经济日历事件的详情
    pub detail: String,
    /// 经济日历事件的指标
    pub importance: String,
    /// 经济日历事件的实际值
    pub actual: Option<String>,
    /// 经济日历事件的预期值
    pub consensus: Option<String>,
    /// 经济日历事件的前值
    pub previous: Option<String>,
}

/// API利率限制信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RateLimit {
    /// API请求接口
    pub endpoint: String,
    /// 已使用的请求数
    pub used: String,
    /// 每路径请求速率上限
    pub limit: String,
    /// API限制的窗口时间（毫秒）
    #[serde(rename = "intervalSec")]
    pub interval_sec: String,
} 