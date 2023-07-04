use std::{mem::size_of, sync::Arc};

use bytes::{Buf, BufMut};
use serde::{Deserialize, Serialize};
use tungstenite::Message;

use crate::{common::*, DigitalisError, DigitalisResult};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ClientMessage {
    Subscribe(Subscribe),
    Unsubscribe(Unsubscribe),
    Advertise(ClientAdvertise),
    Unadvertise(ClientUnadvertise),
    GetParameters(GetParameters),
    SetParameters(SetParameters),
    SubscribeParameterUpdates(SubscribeParameterUpdates),
    UnsubscribeParameterUpdates(UnsubscribeParameterUpdates),
    SubscribeConnectionGraph,
    UnsubscribeConnectionGraph,
    MessageData(MessageData),
    ServiceCallRequest(ServiceCallRequest),
    Close,
}

impl ClientMessage {
    pub fn from_ws_message(raw_msg: Message) -> DigitalisResult<Self> {
        match raw_msg {
            Message::Binary(msg) => {
                Ok(ClientBinaryMessage::deserialize(&mut msg.as_slice())?.into())
            }
            Message::Text(msg) => Ok(ClientJsonMessage::deserialize(&msg)?.into()),
            Message::Close(_) => Ok(Self::Close),
            m => Err(DigitalisError::UnexpectedWebsocketMessage(
                format!("{}", m).into(),
            )),
        }
    }
}

impl From<ClientJsonMessage> for ClientMessage {
    fn from(msg: ClientJsonMessage) -> Self {
        use ClientJsonMessage::*;
        match msg {
            Subscribe(msg) => Self::Subscribe(msg),
            Unsubscribe(msg) => Self::Unsubscribe(msg),
            Advertise(msg) => Self::Advertise(msg),
            Unadvertise(msg) => Self::Unadvertise(msg),
            GetParameters(msg) => Self::GetParameters(msg),
            SetParameters(msg) => Self::SetParameters(msg),
            SubscribeParameterUpdates(msg) => Self::SubscribeParameterUpdates(msg),
            UnsubscribeParameterUpdates(msg) => Self::UnsubscribeParameterUpdates(msg),
            SubscribeConnectionGraph => Self::SubscribeConnectionGraph,
            UnsubscribeConnectionGraph => Self::UnsubscribeConnectionGraph,
        }
    }
}

impl From<ClientBinaryMessage> for ClientMessage {
    fn from(msg: ClientBinaryMessage) -> Self {
        use ClientBinaryMessage::*;
        match msg {
            MessageData(msg) => Self::MessageData(msg),
            ServiceCallRequest(msg) => Self::ServiceCallRequest(msg),
        }
    }
}

macro_rules! impl_enum_from {
    ($parent:ident, $child:ident,$child_ty:ident) => {
        impl From<$child_ty> for $parent {
            fn from(msg: $child_ty) -> Self {
                $parent::$child(msg)
            }
        }
    };
    ($parent:ident, $child:ident) => {
        impl_enum_from!($parent, $child, $child);
    };
}

macro_rules! impl_into_text_message {
    ($parent:ident, $child:ident) => {
        impl $child {
            pub fn into_message(self) -> DigitalisResult<Message> {
                $parent::from(self).to_message()
            }
        }
    };
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "op", rename_all = "camelCase")]
pub enum ClientJsonMessage {
    Subscribe(Subscribe),
    Unsubscribe(Unsubscribe),
    Advertise(ClientAdvertise),
    Unadvertise(ClientUnadvertise),
    GetParameters(GetParameters),
    SetParameters(SetParameters),
    SubscribeParameterUpdates(SubscribeParameterUpdates),
    UnsubscribeParameterUpdates(UnsubscribeParameterUpdates),
    SubscribeConnectionGraph,
    UnsubscribeConnectionGraph,
}

impl ClientJsonMessage {
    pub fn to_message(&self) -> DigitalisResult<Message> {
        Ok(Message::Text(self.serialize()?))
    }

    pub fn serialize(&self) -> DigitalisResult<String> {
        Ok(serde_json::to_string(self)?)
    }

    pub fn deserialize(text: &str) -> DigitalisResult<Self> {
        Ok(serde_json::from_str(text)?)
    }
}

impl_enum_from!(ClientJsonMessage, Subscribe);
impl_enum_from!(ClientJsonMessage, Unsubscribe);
impl_enum_from!(ClientJsonMessage, Advertise, ClientAdvertise);
impl_enum_from!(ClientJsonMessage, Unadvertise, ClientUnadvertise);
impl_enum_from!(ClientJsonMessage, GetParameters);
impl_enum_from!(ClientJsonMessage, SetParameters);
impl_enum_from!(ClientJsonMessage, SubscribeParameterUpdates);
impl_enum_from!(ClientJsonMessage, UnsubscribeParameterUpdates);

impl_into_text_message!(ClientJsonMessage, Subscribe);
impl_into_text_message!(ClientJsonMessage, Unsubscribe);
impl_into_text_message!(ClientJsonMessage, ClientAdvertise);
impl_into_text_message!(ClientJsonMessage, ClientUnadvertise);
impl_into_text_message!(ClientJsonMessage, GetParameters);
impl_into_text_message!(ClientJsonMessage, SetParameters);
impl_into_text_message!(ClientJsonMessage, SubscribeParameterUpdates);
impl_into_text_message!(ClientJsonMessage, UnsubscribeParameterUpdates);

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Subscribe {
    pub subscriptions: Vec<SubscribeChannel>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Unsubscribe {
    pub subscription_ids: Vec<SubscriptionId>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ClientAdvertise {
    pub channels: Vec<AdvertiseChannel>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ClientUnadvertise {
    pub channel_ids: Vec<ChannelId>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetParameters {
    pub parameter_names: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SetParameters {
    pub parameters: Vec<Parameter>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SubscribeParameterUpdates {
    pub parameter_names: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UnsubscribeParameterUpdates {
    pub parameter_names: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ClientBinaryMessage {
    MessageData(MessageData),
    ServiceCallRequest(ServiceCallRequest),
}

impl ClientBinaryMessage {
    pub fn serialize<T: BufMut>(&self, buf: &mut T) {
        match self {
            Self::MessageData(msg) => {
                buf.put_u8(0x01);
                msg.serialize(buf);
            }
            Self::ServiceCallRequest(msg) => {
                buf.put_u8(0x02);
                msg.serialize(buf);
            }
        }
    }

    pub fn deserialize<T: Buf>(buf: &mut T) -> DigitalisResult<Self> {
        Ok(match buf.get_u8() {
            0x01 => Self::from(MessageData::deserialize(buf)?),
            0x02 => Self::from(ServiceCallRequest::deserialize(buf)?),
            x => {
                return Err(DigitalisError::BinaryDeserializeError(
                    format!("Unknown protocol {}", x).into(),
                ))
            }
        })
    }
}

impl_enum_from!(ClientBinaryMessage, MessageData);
impl_enum_from!(ClientBinaryMessage, ServiceCallRequest);

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MessageData {
    pub channel_id: ChannelId,
    pub payload: Arc<Vec<u8>>,
}

impl MessageData {
    fn serialize<T: BufMut>(&self, buf: &mut T) {
        buf.put_u32_le(self.channel_id);
        buf.put_slice(&self.payload);
    }

    fn deserialize<T: Buf>(buf: &mut T) -> DigitalisResult<Self> {
        if buf.remaining() < size_of::<u32>() {
            return Err(DigitalisError::BinaryDeserializeError(
                "Data is too short".into(),
            ));
        }

        let channel_id = buf.get_u32_le();
        let payload = buf.chunk().to_vec();
        buf.advance(payload.len());

        Ok(Self {
            channel_id,
            payload: Arc::new(payload),
        })
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ServiceCallRequest {
    pub service_id: ChannelId,
    pub call_id: u32,
    pub encoding: Vec<u8>,
    pub payload: Vec<u8>,
}

impl ServiceCallRequest {
    fn serialize<T: BufMut>(&self, buf: &mut T) {
        buf.put_u32_le(self.service_id);
        buf.put_u32_le(self.call_id);
        buf.put_u32_le(self.encoding.len() as u32);
        buf.put_slice(&self.encoding);
        buf.put_slice(&self.payload);
    }

    fn deserialize<T: Buf>(buf: &mut T) -> DigitalisResult<Self> {
        if buf.remaining() < size_of::<u32>() * 3 {
            return Err(DigitalisError::BinaryDeserializeError(
                "Data is too short".into(),
            ));
        }

        let service_id = buf.get_u32_le();
        let call_id = buf.get_u32_le();

        let encoding_len = buf.get_u32_le() as usize;
        if buf.remaining() < encoding_len {
            return Err(DigitalisError::BinaryDeserializeError(
                "Data is too short".into(),
            ));
        }
        let encoding = buf.chunk()[..encoding_len].to_vec();
        buf.advance(encoding.len());

        let payload = buf.chunk().to_vec();
        buf.advance(payload.len());

        Ok(Self {
            service_id,
            call_id,
            encoding,
            payload,
        })
    }
}
