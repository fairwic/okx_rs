use core::time::Duration;
use okx::websocket::{Args, ChannelType, OkxWebsocketClient};
use tokio::time::sleep;
use okx::Error;

#[tokio::main]
async fn main() -> Result<(), Error> {
    env_logger::init();
    let args = Args::new().with_inst_id("BTC-USDT".to_string());
    let mut client = OkxWebsocketClient::new_public();
    let mut rx = client.connect().await.unwrap();
    client.subscribe(ChannelType::Tickers, args).await.unwrap();
    tokio::spawn(async move {
        while let Some(msg) = rx.recv().await {
            println!("收到公共频道消息: {:?}", msg);
        }
    });
    sleep(Duration::from_secs(100)).await;
    Ok(())
}
