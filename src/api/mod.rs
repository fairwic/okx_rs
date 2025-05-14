pub mod account;
pub mod trade;
pub mod market;
pub mod public_data;
pub mod asset;
pub mod big_data;
pub mod websocket;

// 重新导出已移动的模块
pub use websocket::OkxWebsocketApi;

// 常量定义
pub const API_ACCOUNT_PATH: &str = "/api/v5/account";
pub const API_TRADE_PATH: &str = "/api/v5/trade";
pub const API_MARKET_PATH: &str = "/api/v5/market";
pub const API_PUBLIC_PATH: &str = "/api/v5/public";
pub const API_ASSET_PATH: &str = "/api/v5/asset";
pub const API_SYSTEM_PATH: &str = "/api/v5/system";
pub const API_BIGDATA_PATH: &str = "/api/v5/rubik"; 