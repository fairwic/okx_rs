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
// calendarId	string	经济日历ID
// date	string	actual字段值的预期发布时间，Unix时间戳的毫秒数格式，如 1597026383085
// region	string	国家，地区或实体
// category	string	类别名
// event	string	事件名
// refDate	string	当前事件指向的日期
// actual	string	事件实际值
// previous	string	当前事件上个周期的最新实际值。
// 若发生数据修正，该字段存储上个周期修正后的实际值。
// forecast	string	由权威经济学家共同得出的预测值
// dateSpan	string	0：事件的具体发生时间已知
// 1：事件的具体发生日期已知，但时间未知
// importance	string	重要性
// 1: 低
// 2: 中等
// 3: 高
// uTime	string	当前事件的最新更新时间，Unix时间戳的毫秒数格式，如 1597026383085
// prevInitial	string	该事件上一周期的初始值
// 仅在修正发生时有值
// ccy	string	事件实际值对应的货币
// unit	string	事件实际值对应的单位
/// 经济日历事件
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EconomicEventOkxRespDto {
    /// 经济日历ID
    #[serde(rename = "calendarId")]
    pub calendar_id: String,
    /// 计划发布时间，Unix时间戳的毫秒数格式
    pub date: String,
    /// 经济日历事件的区域
    pub region: String,
    /// 经济日历事件的类别
    pub category: String,
    /// 经济日历事件的指标
    pub event: String,
    /// 经济日历事件的指向日期
    #[serde(rename = "refDate")]
    pub ref_date: String,
    /// 经济日历事件的实际值
    pub actual: String,
    /// 经济日历事件的前值
    pub previous: String,
    /// 经济日历事件的预期值
    pub forecast: String,
    /// 经济日历事件的重要性
    pub importance: String,
    /// 经济日历事件的最新更新时间，Unix时间戳的毫秒数格式
    #[serde(rename = "uTime")]
    pub u_time: String,
    /// 经济日历事件的初始值
    #[serde(rename = "prevInitial")]
    pub prev_initial: String,
    /// 经济日历事件的货币
    pub ccy: String,
    /// 经济日历事件的单位
    pub unit: String,
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