use digitalis_core::{DigitalisError, DigitalisResult};
use tokio::sync::mpsc::Sender;
use tokio_tungstenite::tungstenite::Message;

pub type ClientId = u32;
pub type MessageSender = Sender<Message>;

#[derive(Debug, Clone)]
pub struct Client {
    id: ClientId,
    sender: MessageSender,
}

impl Client {
    pub const fn new(id: ClientId, sender: MessageSender) -> Self {
        Self { id, sender }
    }

    pub const fn id(&self) -> ClientId {
        self.id
    }

    pub fn nonblocking_send(&self, msg: Message) -> DigitalisResult<()> {
        self.sender
            .try_send(msg)
            .map_err(|_| DigitalisError::ChannelSendError)
    }
}
