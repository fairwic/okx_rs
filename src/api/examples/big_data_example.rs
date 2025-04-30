use crate::api::big_data::OkxBigData;
use crate::client::OkxClient;
use crate::config::init_env;

pub async fn run_big_data_example() -> anyhow::Result<()> {
    // 初始化环境变量
    init_env();
    
    // 创建客户端
    let client = OkxClient::from_env()?;
    
    // 创建大数据API
    let big_data = OkxBigData::new(client);
    
    println!("========== 获取交易大数据支持币种 ==========");
    let support_coins = big_data.get_support_coin().await?;
    println!("支持的币种: {:?}", support_coins);
    
    println!("\n========== 获取主动买入/卖出情况 ==========");
    let taker_volume = big_data.get_taker_volume("BTC", "SPOT", None, None, Some("5m")).await?;
    println!("BTC SPOT 主动买入/卖出量 (5分钟粒度): {:?}", taker_volume);
    
    println!("\n========== 获取合约主动买入/卖出情况 ==========");
    let contract_volume = big_data.get_taker_volume_contract("BTC-USDT-SWAP", Some("5m"), None, None, None, Some("5")).await?;
    println!("BTC-USDT-SWAP 合约主动买入/卖出量 (5分钟粒度, 最近5条): {:?}", contract_volume);
    
    Ok(())
} 