use std::{collections::HashMap, mem::size_of, sync::Arc};

use bytes::{Buf, BufMut, BytesMut};
use serde::{Deserialize, Serialize};
use serde_with::serde_as;
use tungstenite::Message;

use crate::{common::*, DigitalisError, DigitalisResult};

macro_rules! impl_enum_from {
    ($parent:ident, $child:ident, $child_ty:ident) => {
        impl From<$child_ty> for $parent {
            fn from(msg: $child_ty) -> Self {
                $parent::$child(msg)
            }
        }

        impl $child {
            pub fn into_message(self) -> DigitalisResult<Message> {
                $parent::from(self).to_message()
            }
        }
    };
    ($parent:ident, $child:ident) => {
        impl_enum_from!($parent, $child, $child);
    };
}

macro_rules! impl_into_text_message {
    ($parent:ident, $child:ident) => {};
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "op", rename_all = "camelCase")]
pub enum ServerJsonMessage {
    ServerInfo(ServerInfo),
    Status(Status),
    Advertise(Advertise),
    Unadvertise(Unadvertise),
    ParameterValues(ParameterValues),
    AdvertiseServices(AdvertiseServices),
    UnadvertiseServices(UnadvertiseServices),
    ConnectionGraphUpdate(ConnectionGraphUpdate),
}

impl ServerJsonMessage {
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

impl_enum_from!(ServerJsonMessage, ServerInfo);
impl_enum_from!(ServerJsonMessage, Status);
impl_enum_from!(ServerJsonMessage, Advertise);
impl_enum_from!(ServerJsonMessage, Unadvertise);
impl_enum_from!(ServerJsonMessage, ParameterValues);
impl_enum_from!(ServerJsonMessage, AdvertiseServices);
impl_enum_from!(ServerJsonMessage, UnadvertiseServices);

impl_into_text_message!(ServerJsonMessage, ServerInfo);
impl_into_text_message!(ServerJsonMessage, Status);
impl_into_text_message!(ServerJsonMessage, Advertise);
impl_into_text_message!(ServerJsonMessage, Unadvertise);
impl_into_text_message!(ServerJsonMessage, ParameterValues);
impl_into_text_message!(ServerJsonMessage, AdvertiseServices);
impl_into_text_message!(ServerJsonMessage, UnadvertiseServices);

#[serde_as]
#[derive(Debug, Clone, PartialEq, Eq, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ServerInfo {
    pub name: String,
    pub capabilities: Vec<Capability>,
    pub supported_encodings: Vec<MessageEncoding>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<HashMap<String, serde_json::Value>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub session_id: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Status {
    pub level: Level,
    pub message: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Level {
    Info = 0,
    Warning = 1,
    Error = 2,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Advertise {
    pub channels: Vec<AdvertiseChannel>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Unadvertise {
    pub channel_ids: Vec<ChannelId>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ParameterValues {
    pub parameters: Vec<Parameter>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AdvertiseServices {
    pub services: Vec<Service>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Service {
    pub id: ChannelId,
    pub name: String,
    pub r#type: String,
    pub request_schema: String,
    pub response_schema: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UnadvertiseServices {
    pub ids: Vec<ChannelId>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ConnectionGraphUpdate {
    pub publish_topics: Vec<PublishedTopic>,
    pub suscribed_topics: Vec<SubscribedTopic>,
    pub advertised_services: Vec<AdvertisedService>,
    pub removed_topics: Vec<String>,
    pub removed_services: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PublishedTopic {
    pub name: String,
    pub publisher_ids: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SubscribedTopic {
    pub name: String,
    pub subscriber_ids: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AdvertisedService {
    pub name: String,
    pub provider_ids: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ServerBinaryMessage {
    MessageData(MessageData),
    Time(Time),
    ServiceCallResponse(ServiceCallResponse),
}

impl ServerBinaryMessage {
    pub fn to_message(self) -> DigitalisResult<Message> {
        let mut buf = BytesMut::new();
        self.serialize(&mut buf);
        Ok(Message::Binary(buf.into()))
    }

    pub fn serialize<T: BufMut>(&self, buf: &mut T) {
        match self {
            Self::MessageData(msg) => {
                buf.put_u8(0x01);
                msg.serialize(buf);
            }
            Self::Time(msg) => {
                buf.put_u8(0x02);
                msg.serialize(buf);
            }
            Self::ServiceCallResponse(msg) => {
                buf.put_u8(0x03);
                msg.serialize(buf);
            }
        }
    }

    pub fn deserialize<T: Buf>(buf: &mut T) -> DigitalisResult<Self> {
        Ok(match buf.get_u8() {
            0x01 => Self::from(MessageData::deserialize(buf)?),
            0x02 => Self::from(Time::deserialize(buf)?),
            0x03 => Self::from(ServiceCallResponse::deserialize(buf)?),
            x => {
                return Err(DigitalisError::BinaryDeserializeError(
                    format!("Unknown protocol {}", x).into(),
                ))
            }
        })
    }
}

impl_enum_from!(ServerBinaryMessage, MessageData);
impl_enum_from!(ServerBinaryMessage, Time);
impl_enum_from!(ServerBinaryMessage, ServiceCallResponse);

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MessageData {
    pub subscription_id: SubscriptionId,
    pub receive_timestamp: u64,
    pub payload: Arc<Vec<u8>>,
}

impl MessageData {
    fn serialize<T: BufMut>(&self, buf: &mut T) {
        buf.put_u32_le(self.subscription_id);
        buf.put_u64_le(self.receive_timestamp);
        buf.put_slice(&self.payload);
    }

    fn deserialize<T: Buf>(buf: &mut T) -> DigitalisResult<Self> {
        if buf.remaining() < size_of::<u32>() + size_of::<u64>() {
            return Err(DigitalisError::BinaryDeserializeError(
                "Data is too short".into(),
            ));
        }

        let subscription_id = buf.get_u32_le();
        let receive_timestamp = buf.get_u64_le();
        let payload = buf.chunk().to_vec();
        buf.advance(payload.len());

        Ok(Self {
            subscription_id,
            receive_timestamp,
            payload: Arc::new(payload),
        })
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Time {
    pub timestamp: u64,
}

impl Time {
    fn serialize<T: BufMut>(&self, buf: &mut T) {
        buf.put_u64_le(self.timestamp);
    }

    fn deserialize<T: Buf>(buf: &mut T) -> DigitalisResult<Self> {
        if buf.remaining() != size_of::<u64>() {
            return Err(DigitalisError::BinaryDeserializeError(
                "Data is too short".into(),
            ));
        }

        Ok(Self {
            timestamp: buf.get_u64_le(),
        })
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ServiceCallResponse {
    pub service_id: ChannelId,
    pub call_id: u32,
    pub encoding: Vec<u8>,
    pub payload: Vec<u8>,
}

impl ServiceCallResponse {
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

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_serialize_and_deserialize_message_data() {
        let msg = MessageData {
            subscription_id: 25,
            receive_timestamp: 23893748,
            payload: Arc::new(vec![1, 23, 125]),
        };

        let mut buf = Vec::new();
        msg.serialize(&mut buf);
        let msg2 = MessageData::deserialize(&mut buf.as_slice()).unwrap();

        assert_eq!(msg, msg2);
    }

    #[test]
    fn test_serialize_and_deserialize_time() {
        let msg = Time {
            timestamp: 23893748,
        };

        let mut buf = Vec::new();
        msg.serialize(&mut buf);
        let msg2 = Time::deserialize(&mut buf.as_slice()).unwrap();

        assert_eq!(msg, msg2);
    }

    #[test]
    fn test_serialize_and_deserialize_service_call_response() {
        let msg = ServiceCallResponse {
            service_id: 25,
            call_id: 23893748,
            encoding: vec![1, 23, 125],
            payload: vec![25, 225, 23, 125],
        };

        let mut buf = Vec::new();
        msg.serialize(&mut buf);
        let msg2 = ServiceCallResponse::deserialize(&mut buf.as_slice()).unwrap();

        assert_eq!(msg, msg2);
    }
}
