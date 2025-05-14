use okx::config::Credentials;
use okx::{Error, OkxClient, OkxMarket};
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

    let market = OkxMarket::new(client.clone());
    // 获取BTC-USDT的产品行情
    let ticker = market.get_ticker("BTC-USDT-SWAP").await?;
    println!("BTC-USDT 行情: {:?}", ticker);
    Ok(())
}
