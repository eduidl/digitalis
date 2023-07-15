use protobuf::MessageFull;
use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::schema::base64_proto;

pub type Id = u32;
pub type ChannelId = u32;
pub type SubscriptionId = u32;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum Capability {
    ClientPublish,
    Parameters,
    ParametersSubscribe,
    Time,
    Services,
    ConnectionGraph,
    Assets,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum MessageEncoding {
    Json,
    Protobuf,
    FlatBuffer,
    Ros1,
    /// for ROS 2
    Cdr,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum SchemaEncoding {
    JsonSchema,
    Protobuf,
    FlatBuffer,
    Ros1Msg,
    Ros2Msg,
    Ros2Idl,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AdvertiseChannel {
    pub id: ChannelId,
    pub topic: String,
    pub encoding: MessageEncoding,
    pub schema_name: String,
    pub schema: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schema_encoding: Option<SchemaEncoding>,
}

impl AdvertiseChannel {
    pub fn protobuf<T: MessageFull>(id: u32, topic: &str) -> Self {
        Self {
            id,
            topic: topic.into(),
            encoding: MessageEncoding::Protobuf,
            schema_name: T::descriptor().full_name().into(),
            schema: base64_proto::<T>(),
            schema_encoding: None,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SubscribeChannel {
    pub id: SubscriptionId,
    pub channel_id: ChannelId,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Parameter {
    pub name: String,
    pub value: Value,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<ParameterType>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ParameterType {
    ByteArray,
}
