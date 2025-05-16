use crate::client::OkxClient;
use crate::error::Error;

pub trait OkxApi {
    fn new(client: OkxClient) -> Self;
    fn from_env() -> Result<Self, Error> where Self: Sized;
    fn client(&self) -> &OkxClient;
}
    // pub fn new(client: OkxClient) -> Self {
    //     Self { client }
    // }
    
    // /// 从环境变量创建一个新的OkxMarket实例
    // pub fn from_env() -> Result<Self, Error> {
    //     let client = OkxClient::from_env()?;
    //     Ok(Self { client })
    // }
    
    // /// 获取内部客户端引用
    // pub fn client(&self) -> &OkxClient {
    //     &self.client
    // }