// OKX SDK - Rust Client Library
// 提供与OKX交易所API的通信能力

pub mod client;
pub mod error;
pub mod dto;
pub mod api;
pub mod utils;
pub mod config;
pub mod websocket;

/// OKX SDK的版本
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

/// Re-export commonly used modules and functions
pub use client::OkxClient;
pub use api::{
    account::OkxAccount,
    trade::OkxTrade,
    market::OkxMarket,
    public_data::OkxPublicData,
    asset::OkxAsset,
    big_data::OkxBigData,
    websocket::OkxWebsocketApi,
    examples,
};
pub use error::Error;
pub use websocket::OkxWebsocketClient;
use crate::dto::common;
