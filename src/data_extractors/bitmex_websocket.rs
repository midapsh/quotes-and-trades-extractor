use futures::SinkExt;
// use tokio::net::TcpStream;
// use tokio_tungstenite::tungstenite::stream::MaybeTlsStream;
use tokio_tungstenite::{connect_async, tungstenite::Message as TMessage, WebSocketStream};

// use super::errors::bitmex_errors::CBError;
// use crate::errors::websocket_errors::WSError;

// use tokio_tungstenite::connect_async;

pub struct BitmexWebsocket;

use crate::commands::bitmex_subscribe::{Args, Subscribe, SubscribeCmd};

impl BitmexWebsocket {
    const URL: &'static str = "wss://www.bitmex.com/realtime";

    /// Constructor for simple subcription with product_ids and args
    pub async fn connect(
        args: Args,
    ) -> core::result::Result<
        WebSocketStream<tokio_tungstenite::MaybeTlsStream<tokio::net::TcpStream>>,
        tokio_tungstenite::tungstenite::error::Error,
    > {
        let subscribe = Subscribe {
            _type: SubscribeCmd::Subscribe,
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
        // TODO(hspadim): Add timeout
        let (mut stream, _response) =
            match tokio::time::timeout(std::time::Duration::from_secs(5), connect_async(Self::URL))
                .await
            {
                Ok(it) => it?,
                Err(_) => {
                    return Err(tokio_tungstenite::tungstenite::Error::Io(
                        std::io::Error::new(
                            std::io::ErrorKind::BrokenPipe,
                            "Timeout when trying to connect",
                        ),
                    ));
                }
            };
        println!("WebSocket handshake has been successfully completed: {:?}", subscribe.args);

        let subscribe = serde_json::to_string(&subscribe).unwrap();

        match tokio::time::timeout(
            std::time::Duration::from_secs(5),
            stream.send(TMessage::Text(subscribe)),
        )
        .await
        {
            Ok(it) => it.and_then(|_| {
                println!("subscription sent!");
                Ok(stream)
            }),
            Err(_) => Err(tokio_tungstenite::tungstenite::Error::Io(
                std::io::Error::new(
                    std::io::ErrorKind::BrokenPipe,
                    "Timeout when trying to subscribe",
                ),
            )),
        }
    }
}
