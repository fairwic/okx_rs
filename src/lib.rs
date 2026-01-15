// OKX SDK - Rust Client Library
// 提供与OKX交易所API的通信能力

pub mod api;
pub mod client;
pub mod config;
pub mod debug_helper;
pub mod dto;
pub mod enums;
pub mod error;
pub mod utils;
pub mod websocket;

/// OKX SDK的版本
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

pub use api::{
    account::OkxAccount, asset::OkxAsset, big_data::OkxBigData, market::OkxMarket,
    public_data::OkxPublicData, trade::OkxTrade, websocket::OkxWebsocketApi,
};
/// Re-export commonly used modules and functions
pub use client::OkxClient;
pub use error::Error;
pub use websocket::OkxWebsocketClient;
