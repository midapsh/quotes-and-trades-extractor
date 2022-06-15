use futures::{SinkExt, TryStreamExt};
// use tokio::net::TcpStream;
// use tokio_tungstenite::tungstenite::stream::MaybeTlsStream;

use tokio_tungstenite::{connect_async, tungstenite::Message as TMessage, WebSocketStream};

// use super::errors::deribit_errors::CBError;
// use crate::errors::websocket_errors::WSError;
use exchange_extractor::configuration;

// use tokio_tungstenite::connect_async;

pub struct DeribitWebsocket;

use crate::commands::deribit_subscribe::{Args, IdCmd, JsonRpc, MethodCmd, Subscribe};

impl DeribitWebsocket {
    const URL: &'static str = "wss://www.deribit.com/ws/api/v2";
    // const URL: &'static str = "wss://test.deribit.com/ws/api/v2";

    /// Constructor for simple subcription with product_ids and args
    pub async fn connect(
        args: Args,
    ) -> core::result::Result<
        WebSocketStream<tokio_tungstenite::MaybeTlsStream<tokio::net::TcpStream>>,
        tokio_tungstenite::tungstenite::error::Error,
    > {
        let subscribe = Subscribe {
            json_rpc: JsonRpc::Version("2.0".to_string()),
            id: IdCmd::Id(1),
            _type: MethodCmd::Subscribe,
            args: args,
        };

        Self::connect_with_sub(subscribe).await
    }

    /// Constructor for extended subcription via Subscribe structure
    pub async fn connect_with_sub(
        subscribe: Subscribe,
    ) -> core::result::Result<
        WebSocketStream<tokio_tungstenite::MaybeTlsStream<tokio::net::TcpStream>>,
        tokio_tungstenite::tungstenite::error::Error,
    > {
        let settings = configuration::get_configuration().unwrap();

        let (mut stream, _response) = connect_async(Self::URL).await?;
        println!("WebSocket handshake has been successfully completed");

        let auth_msg = serde_json::json!({
            "jsonrpc": "2.0",
            "id": 0,
            "method": "public/auth",
            "params": {
                "grant_type": "client_credentials",
                "client_id":settings.deribit.api_key,
                "client_secret":settings.deribit.api_secret
            }
        })
        .to_string();
        println!("{}", auth_msg);
        stream.send(TMessage::Text(auth_msg)).await?;
        let a = stream.try_next().await?;
        println!("{:?}", a);
        let subscribe = serde_json::to_string(&subscribe).unwrap();

        stream.send(TMessage::Text(subscribe)).await?;
        println!("subscription sent");

        Ok(stream)
    }
}
