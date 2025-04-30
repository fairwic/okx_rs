use crate::api::websocket::OkxWebsocketApi;
use crate::websocket::{ChannelType, Args};
use crate::config::init_env;
use tokio::sync::mpsc::Receiver;

pub async fn run_websocket_example() -> anyhow::Result<()> {
    // 初始化环境变量
    init_env();
    
    println!("启动WebSocket客户端...");
    
    // 创建WebSocket客户端
    let mut ws_api = OkxWebsocketApi::new_public();
    
    // 连接到WebSocket服务器
    let mut rx = ws_api.connect().await?;
    
    // 订阅比特币行情数据
    let args = Args::new().with_inst_id("BTC-USDT");
    ws_api.subscribe(ChannelType::Tickers, args).await?;
    
    // 处理接收到的消息
    println!("等待消息...");
    
    process_messages(rx, 5).await;
    
    Ok(())
}

async fn process_messages(mut rx: Receiver<serde_json::Value>, max_messages: usize) {
    let mut counter = 0;
    
    while let Some(msg) = rx.recv().await {
        println!("收到消息: {}", msg);
        counter += 1;
        
        if counter >= max_messages {
            println!("已收到{}条消息示例", max_messages);
            break;
        }
    }
} 