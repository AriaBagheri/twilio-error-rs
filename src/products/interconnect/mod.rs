// AUTO-GENERATED. DO NOT MODIFY. YOUR CHANGES WILL BE OVER-WRITTEN
mod message;
mod docs;
mod causes;
mod solutions;
mod description;
mod code;
mod from_code;

pub use message::*;
pub use docs::*;
pub use causes::*;
pub use solutions::*;
pub use description::*;
pub use code::*;
pub use from_code::*;

use serde::{Deserialize, Deserializer, Serialize, Serializer};
use serde::de::Error;
use standard_error::traits::{StandardErrorCodeTrait, StandardErrorFromCodeTrait};

#[derive(Debug, Clone)]
pub enum TwilioInterconnectError {
    ErrorCode62034,
    ErrorCode62015,
    ErrorCode32303,
    ErrorCode32301,
    ErrorCode62028,
    ErrorCode62200,
    ErrorCode62001,
    ErrorCode62005,
    ErrorCode62017,
    ErrorCode62006,
    ErrorCode62003,
    ErrorCode62025,
    ErrorCode62012,
    ErrorCode62053,
    ErrorCode62020,
    ErrorCode62021,
    ErrorCode62009,
    ErrorCode62024,
    ErrorCode62027,
    ErrorCode62008,
    ErrorCode62100,
    ErrorCode62007,
    ErrorCode62019,
    ErrorCode32304,
    ErrorCode62014,
    ErrorCode62010,
    ErrorCode62002,
    ErrorCode62004,
    ErrorCode62026,
    ErrorCode62023,
    ErrorCode62018,
    ErrorCode62000,
    ErrorCode62013,
    ErrorCode62035,
    ErrorCode62022,
    ErrorCode62016,
    ErrorCode62052,
    ErrorCode62011,
    ErrorCode32302,
    ErrorCode62220
}


impl<'de> Deserialize<'de> for TwilioInterconnectError {
            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>
    {
            Ok(
                usize::deserialize(deserializer)
            .map(TwilioInterconnectError::from_code)?
            .ok_or_else(|| Error::custom("Invalid error code!"))?
            )
    }
}

impl Serialize for TwilioInterconnectError {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
            usize::serialize(&self.code(), serializer)
    }
}

use crate::products::TwilioProductError;
impl Into<TwilioProductError> for TwilioInterconnectError {
    fn into(self) -> TwilioProductError {
        TwilioProductError::Interconnect(self)
    }
}

impl crate::products::SomeTwilioProductError for TwilioInterconnectError {}

