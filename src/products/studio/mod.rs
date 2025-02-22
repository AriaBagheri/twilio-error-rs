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
pub enum TwilioStudioError {
    ErrorCode81002,
    ErrorCode81026,
    ErrorCode81007,
    ErrorCode81027,
    ErrorCode81014,
    ErrorCode81012,
    ErrorCode81019,
    ErrorCode81025,
    ErrorCode81017,
    ErrorCode81006,
    ErrorCode81005,
    ErrorCode81016,
    ErrorCode81024,
    ErrorCode81004,
    ErrorCode81008,
    ErrorCode81013,
    ErrorCode81018,
    ErrorCode81021,
    ErrorCode81022,
    ErrorCode81001,
    ErrorCode81000,
    ErrorCode81009,
    ErrorCode81010,
    ErrorCode81015,
    ErrorCode81020,
    ErrorCode81023,
    ErrorCode81011
}


impl<'de> Deserialize<'de> for TwilioStudioError {
            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>
    {
            Ok(
                usize::deserialize(deserializer)
            .map(TwilioStudioError::from_code)?
            .ok_or_else(|| Error::custom("Invalid error code!"))?
            )
    }
}

impl Serialize for TwilioStudioError {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
            usize::serialize(&self.code(), serializer)
    }
}

use crate::products::TwilioProductError;
impl Into<TwilioProductError> for TwilioStudioError {
    fn into(self) -> TwilioProductError {
        TwilioProductError::Studio(self)
    }
}

impl crate::products::SomeTwilioProductError for TwilioStudioError {}

