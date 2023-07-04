use std::sync::Arc;

use digitalis_core::{
    common::{ChannelId, SubscriptionId},
    server::MessageData,
    Control, DigitalisError, DigitalisResult, MessageMinimal,
};
use tokio::sync::mpsc;

use crate::client::{Client, ClientId};

#[derive(Debug)]
pub struct Channel {
    _id: ChannelId,
    subscriptions: Vec<(Client, SubscriptionId)>,
}

impl Channel {
    fn new(id: ChannelId) -> Self {
        Self {
            _id: id,
            subscriptions: Default::default(),
        }
    }

    pub fn start(id: ChannelId) -> ChannelQueue {
        let (tx, mut rx) = mpsc::channel(100);

        tokio::spawn(async move {
            let mut channel = Self::new(id);

            loop {
                if let Some(msg) = rx.recv().await {
                    match channel.handle_message(msg) {
                        Ok(Control::Continue) => {}
                        Ok(Control::Exit) => break,
                        Err(e) => {
                            log::error!("Fail to handle message: {}", e);
                        }
                    }
                }
            }
        });

        ChannelQueue::new(id, tx)
    }

    pub fn handle_message(&mut self, msg: ChannelMessage) -> DigitalisResult<Control> {
        use ChannelMessage::*;

        match msg {
            Subscribe { client, id } => {
                self.subscriptions.push((client, id));
            }
            Unsubscribe { client_id, ids } => {
                self.subscriptions
                    .retain(|(c, i)| c.id() != client_id || !ids.contains(i));
            }
            UnsubscribeAll { client_id } => {
                self.subscriptions.retain(|(c, _)| c.id() != client_id);
            }
            Broadcast {
                message,
                receive_timestamp,
            } => {
                if self.subscriptions.is_empty() {
                    return Ok(Control::Continue);
                }

                let payload = Arc::new(message.write_to_bytes()?);

                for (client, id) in self.subscriptions.iter() {
                    let msg = MessageData {
                        subscription_id: *id,
                        receive_timestamp,
                        payload: Arc::clone(&payload),
                    }
                    .into_message()?;

                    if let Err(e) = client.nonblocking_send(msg) {
                        log::error!("Fail to send message to client: {e:?}");
                    }
                }
            }
            Terminate => return Ok(Control::Exit),
        }

        Ok(Control::Continue)
    }
}

#[allow(missing_debug_implementations)]
pub enum ChannelMessage {
    Subscribe {
        client: Client,
        id: SubscriptionId,
    },
    Unsubscribe {
        client_id: ClientId,
        ids: Arc<Vec<SubscriptionId>>,
    },
    UnsubscribeAll {
        client_id: ClientId,
    },
    Broadcast {
        message: Box<dyn MessageMinimal>,
        receive_timestamp: u64,
    },
    Terminate,
}

#[derive(Debug, Clone)]
pub struct ChannelQueue {
    id: ChannelId,
    sender: mpsc::Sender<ChannelMessage>,
}

impl ChannelQueue {
    pub const fn new(id: ChannelId, sender: mpsc::Sender<ChannelMessage>) -> Self {
        Self { id, sender }
    }

    pub const fn id(&self) -> ChannelId {
        self.id
    }

    fn try_send(&self, msg: ChannelMessage) -> DigitalisResult<()> {
        self.sender
            .try_send(msg)
            .map_err(|_| DigitalisError::ChannelSendError)
    }

    pub fn broadcast<T: MessageMinimal>(
        &self,
        msg: T,
        receive_timestamp: u64,
    ) -> DigitalisResult<()> {
        self.try_send(ChannelMessage::Broadcast {
            message: Box::new(msg),
            receive_timestamp,
        })
    }

    pub fn subscribe(&self, client: Client, id: SubscriptionId) -> DigitalisResult<()> {
        self.try_send(ChannelMessage::Subscribe { client, id })
    }

    pub fn unsubscribe(
        &self,
        client_id: ClientId,
        ids: Arc<Vec<SubscriptionId>>,
    ) -> DigitalisResult<()> {
        self.try_send(ChannelMessage::Unsubscribe { client_id, ids })
    }

    pub fn unsubscribe_all(&self, client_id: ClientId) -> DigitalisResult<()> {
        self.try_send(ChannelMessage::UnsubscribeAll { client_id })
    }

    pub fn terminate(&self) -> DigitalisResult<()> {
        self.try_send(ChannelMessage::Terminate)
    }
}
