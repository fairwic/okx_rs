use crate::config::Credentials;
use crate::websocket::{Args, ChannelType, OkxWebsocketClient};
use crate::Error;
use tokio::sync::mpsc::Receiver;

pub struct OkxWebsocketApi {
    client: OkxWebsocketClient,
}

impl OkxWebsocketApi {
    pub fn new_public() -> Self {
        let client = OkxWebsocketClient::new_public();
        OkxWebsocketApi { client }
    }

    pub fn new_private(credentials: Credentials) -> Self {
        let client = OkxWebsocketClient::new_private(credentials);
        OkxWebsocketApi { client }
    }

    pub async fn connect(&mut self) -> Result<Receiver<serde_json::Value>, Error> {
        self.client.connect().await
    }

    pub async fn subscribe(&self, channel: ChannelType, args: Args) -> Result<(), Error> {
        self.client.subscribe(channel, args).await
    }

    pub async fn unsubscribe(&self, channel: ChannelType, args: Args) -> Result<(), Error> {
        self.client.unsubscribe(channel, args).await
    }
}
