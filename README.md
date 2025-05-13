# OKX API SDK for Rust

这是一个用于与OKX交易所API进行交互的Rust SDK，提供了对OKX V5 API的全面支持。

## 特性

- 支持 REST API 和 WebSocket API
- 完整的类型定义和错误处理
- 模块化和结构化的代码设计
- 提供公共和私有API的访问
- 支持所有主要的API端点：账户、交易、行情数据、资产等
- 自动处理认证和签名
- 内置的重连和心跳机制（WebSocket）
- 支持同步和异步调用

## 安装

在你的`Cargo.toml`文件中添加:

```toml
[dependencies]
okx = "0.1.0"
```

或者直接从GitHub克隆:

```bash
cargo add --git https://github.com/yourusername/okx-sdk-rust
```

## 快速开始

### 设置API密钥

设置环境变量或创建`.env`文件：

```dotenv
OKX_API_KEY=your_api_key
OKX_API_SECRET=your_api_secret
OKX_PASSPHRASE=your_passphrase
OKX_SIMULATED_TRADING=0  # 设置为1使用模拟交易
```

### REST API 示例

```rust
use okx::{create_client, Error};
use okx::api::market::MarketApi;

#[tokio::main]
async fn main() -> Result<(), Error> {
    // 初始化客户端
    let client = create_client()?;
    
    // 获取BTC-USDT的行情
    let ticker = MarketApi::get_ticker(&client, "BTC-USDT-SWAP").await?;
    println!("BTC-USDT 行情: {:?}", ticker);
    
    // 获取账户余额
    // let balances = okx::api::account::AccountApi::get_balance(&client, None).await?;
    // println!("账户余额: {:?}", balances);
    
    Ok(())
}
```

### WebSocket API 示例

```rust
use okx::websocket::{Args, ChannelType, WebsocketApi};
use okx::config::init_env;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 初始化环境
    init_env();
    
    // 创建WebSocket客户端
    let mut ws_client = WebsocketApi::new_public();
    
    // 连接到WebSocket
    let mut rx = ws_client.connect().await?;
    
    // 订阅BTC-USDT行情
    let args = Args::new().with_inst_id("BTC-USDT");
    ws_client.subscribe(ChannelType::Tickers, args).await?;
    
    // 处理接收到的消息
    while let Some(msg) = rx.recv().await {
        println!("收到消息: {}", serde_json::to_string_pretty(&msg)?);
    }
    
    // 关闭连接
    ws_client.close().await;
    
    Ok(())
}
```

## 项目结构

```
src/
├── api/                    # API 实现
│   ├── account/           # 账户相关 API
│   ├── asset/             # 资产相关 API
│   ├── big_data/          # 大数据相关 API
│   ├── market/            # 市场数据 API
│   ├── public_data/       # 公共数据 API
│   ├── trade/             # 交易相关 API
│   └── websocket/         # WebSocket API
├── models/                # 数据模型
│   ├── account/          # 账户相关模型
│   ├── asset/            # 资产相关模型
│   ├── market/           # 市场数据模型
│   └── trade/           # 交易相关模型
├── client.rs             # HTTP 客户端实现
├── config.rs             # 配置管理
├── error.rs              # 错误处理
├── lib.rs               # 库入口
└── utils.rs             # 工具函数
```

## 配置

SDK支持通过环境变量或代码配置：

```rust
use okx::config::{Config, Credentials};

// 通过环境变量配置
let config = Config::default();

// 或者手动配置
let config = Config::default()
    .with_api_url("https://www.okx.com")
    .with_simulated_trading(true);

// 手动设置凭证
let credentials = Credentials::new(
    "your_api_key",
    "your_api_secret",
    "your_passphrase"
);
```

## 开发

```bash
# 克隆仓库
git clone https://github.com/fairwic/okx_rs.git
cd okx_rs

# 运行测试
cargo test

# 运行示例
cargo run --example market
cargo run --example websocket
```

## 许可证

MIT 