use okx::config::Credentials;
use okx::{Error, OkxAsset, OkxClient};
use okx::api::api_trait::OkxApiTrait;
#[tokio::main]
async fn main() -> Result<(), Error> {
    env_logger::init();
    let credentials = Credentials::new(
        "e5df0bda-e1d7-46d8-ba1c-b64e0412f8f6",
        "67742A52509EF4B31B65C3279F02F97D",
        "Fwc_okx_main_520",
        "0",
    ); // 初始化客户端
    let client: OkxClient = OkxClient::new(credentials).unwrap();
    //获取asset账户余额
    let balances = OkxAsset::new(client).get_balances(None).await?;
    println!("账户余额: {:?}", balances);

    Ok(())
}
