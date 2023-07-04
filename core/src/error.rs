use std::{borrow::Cow, io};

use thiserror::Error;

#[derive(Error, Debug)]
pub enum DigitalisError {
    #[error("IO error: {0}")]
    IOError(#[from] io::Error),
    #[error("JSON error")]
    JsonError(#[from] serde_json::Error),
    #[error("Binary deserialize error: {0}")]
    BinaryDeserializeError(Cow<'static, str>),
    #[error("Channel send error")]
    ChannelSendError,
    #[error("Protobuf error {0}")]
    ProtobufError(#[from] protobuf::Error),
    #[error("Unexpected websocket message: {0}")]
    UnexpectedWebsocketMessage(Cow<'static, str>),
}

pub type DigitalisResult<T> = Result<T, DigitalisError>;
