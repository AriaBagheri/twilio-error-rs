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
pub enum TwilioChannelsError {
    ErrorCode63108,
    ErrorCode63106,
    ErrorCode63103,
    ErrorCode63107,
    ErrorCode63101,
    ErrorCode63105,
    ErrorCode63112,
    ErrorCode63109,
    ErrorCode63100,
    ErrorCode63111,
    ErrorCode63102
}


impl<'de> Deserialize<'de> for TwilioChannelsError {
            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>
    {
            Ok(
                usize::deserialize(deserializer)
            .map(TwilioChannelsError::from_code)?
            .ok_or_else(|| Error::custom("Invalid error code!"))?
            )
    }
}

impl Serialize for TwilioChannelsError {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
            usize::serialize(&self.code(), serializer)
    }
}

use crate::products::TwilioProductError;
impl Into<TwilioProductError> for TwilioChannelsError {
    fn into(self) -> TwilioProductError {
        TwilioProductError::Channels(self)
    }
}

impl crate::products::SomeTwilioProductError for TwilioChannelsError {}

