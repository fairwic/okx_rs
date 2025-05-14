use okx::config::Credentials;
use okx::{Error, OkxClient, OkxMarket};
#[tokio::main]
async fn main() -> Result<(), Error> {
    env_logger::init();
    let credentials = Credentials::from_env().unwrap();

    let client: OkxClient = OkxClient::new(credentials).unwrap();

    let market = OkxMarket::new(client.clone());
    // 获取BTC-USDT的产品行情
    let ticker = market.get_ticker("BTC-USDT-SWAP").await?;
    println!("BTC-USDT 行情: {:?}", ticker);
    Ok(())
}
