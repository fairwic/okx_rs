use serde::{Deserialize, Serialize};

/// 语言
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Language {
    /// 中文
    #[serde(rename = "zh-CN")]
    ZhCn,
    /// 英文
    #[serde(rename = "en-US")]
    EnUs,
}

impl Language {
    pub fn to_string(&self) -> &str {
        match self {
            Language::ZhCn => "zh-CN",
            Language::EnUs => "en-US",
        }
    }
}
