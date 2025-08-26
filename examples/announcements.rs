use okx::api::announcements::announcements_api::OkxAnnouncements;
use okx::api::api_trait::OkxApiTrait;
use okx::config::Credentials;
use okx::{Error, OkxClient};
#[tokio::main]
async fn main() -> Result<(), Error> {
    env_logger::init();
    //获取公告
    let announcements = OkxAnnouncements::from_env()
        .unwrap()
        .get_announcements(None, None, None)
        .await?;
    println!("公告: {:?}", announcements);

    Ok(())
}
