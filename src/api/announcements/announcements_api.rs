use crate::api::api_trait::OkxApiTrait;
use crate::client::OkxClient;
use crate::enums::language_enums::Language;
use crate::Error;
use reqwest::Method;
use serde::{Deserialize, Serialize};

/// 公告详情
#[derive(Serialize, Deserialize, Debug)]
pub struct AnnouncementDetail {
    #[serde(rename = "annType")]
    pub ann_type: String,
    #[serde(rename = "pTime")]
    pub p_time: String,
    pub title: String,
    pub url: String,
}

/// 公告分页
#[derive(Serialize, Deserialize, Debug)]
pub struct AnnouncementPage {
    pub details: Vec<AnnouncementDetail>,
    #[serde(rename = "totalPage")]
    pub total_page: String,
}

/// OKX 公告 API
pub struct OkxAnnouncements {
    client: OkxClient,
}

impl OkxApiTrait for OkxAnnouncements {
    fn new(client: OkxClient) -> Self {
        Self { client }
    }

    fn client(&self) -> &OkxClient {
        &self.client
    }
}

impl OkxAnnouncements {
    /// 从环境变量创建实例

    pub fn from_env() -> Result<Self, Error> {
        let client = OkxClient::from_env()?;
        Ok(Self { client })
    }

    /// 获取公告
    /// 获取公告信息，以发布时间倒序排序，公告更新不会影响排序。每页默认有 20 条公告
    /// 请求头中 Accept-Language 设置为 en-US 时返回英文公告；设置为 zh-CN 时返回中文公告
    /// 该接口鉴权是可选的：
    /// 当为公共接口时，响应根据请求 IP 进行限制
    /// 限速：5次/2s
    /// ann_type: 可选，公告类型
    /// page: 可选，页数，默认1
    pub async fn get_announcements(
        &self,
        ann_type: Option<String>,
        page: Option<String>,
        language: Option<Language>,
    ) -> Result<Vec<AnnouncementPage>, Error> {
        let path = "/api/v5/support/announcements";
        let mut body = String::new();
        if let Some(t) = ann_type {
            body.push_str(&format!("annType={}", t));
        }
        if let Some(p) = page {
            if !body.is_empty() {
                body.push('&');
            }
            body.push_str(&format!("page={}", p));
        }

        let mut client = self.client.clone();
        if let Some(l) = language {
            client.set_accept_language(l);
        }
        println!("body: {}", body);
        let res = client
            .send_request::<Vec<AnnouncementPage>>(Method::GET, path, &body)
            .await?;
        println!("res: {:?}", res);
        Ok(res)
    }
}
