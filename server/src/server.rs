use async_trait::async_trait;
use digitalis_core::{client::ClientMessage, Control};
use futures_util::{SinkExt, StreamExt};
use tokio::{
    net::{TcpListener, TcpStream},
    sync::mpsc::channel,
};
use tokio_tungstenite::{accept_hdr_async, tungstenite::Message};

use crate::{
    accept::Accept,
    client::{Client, ClientId},
    DigitalisResult,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ServerOption {
    pub host: String,
    pub port: u16,
}

impl ServerOption {
    pub async fn tcp_connect(&self) -> DigitalisResult<TcpListener> {
        let addr = format!("{}:{}", self.host, self.port);
        let listner = TcpListener::bind(&addr).await?;
        Ok(listner)
    }
}

impl Default for ServerOption {
    fn default() -> Self {
        Self {
            host: "localhost".into(),
            port: 8765,
        }
    }
}

#[async_trait]
pub trait DigitalisServer {
    async fn accept_connection(&self, client_id: ClientId, stream: TcpStream) {
        log::info!(
            "Try to accept connection: {:?}",
            stream.peer_addr().expect("Fail to get peer addr")
        );

        let (mut sender, mut receiver) = accept_hdr_async(stream, Accept {})
            .await
            .expect("Fail to accept")
            .split();

        let (msg_queue_tx, mut msg_queue_rx) = channel::<Message>(100);

        let client = Client::new(client_id, msg_queue_tx);

        self.on_open(&client).unwrap();

        loop {
            tokio::select! {
                Some(msg) = msg_queue_rx.recv() => {
                    if let Err(e) = sender.send(msg).await {
                        log::error!("Fail to send message to client {}: {e:?}", client_id);
                    }
                }

                Some(msg) = receiver.next() => {
                    let msg = msg.expect("Fail to get message");
                    log::debug!("Received message from client {}: {:?}", client.id(), msg);

                    let msg = match ClientMessage::from_ws_message(msg) {
                        Ok(msg) => msg,
                        Err(e) => {
                            log::error!("Fail to parse message from client {}: {e:?}", client.id());
                            continue;
                        }
                    };

                    match self.handle_message(&client, msg) {
                        Ok(Control::Exit) => return,
                        Ok(Control::Continue) => continue,
                        Err(e) => {
                            log::error!("Fail to handle message from client {}: {e:?}", client.id());
                        }
                    }
                }
            }
        }
    }

    fn handle_message(&self, client: &Client, msg: ClientMessage) -> DigitalisResult<Control>;

    fn on_open(&self, client: &Client) -> DigitalisResult<()>;
}
