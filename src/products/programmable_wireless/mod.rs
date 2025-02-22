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
pub enum TwilioProgrammableWirelessError {
    ErrorCode33010,
    ErrorCode33101,
    ErrorCode33004,
    ErrorCode33102,
    ErrorCode33119,
    ErrorCode33108,
    ErrorCode33201,
    ErrorCode33111,
    ErrorCode33107,
    ErrorCode33000,
    ErrorCode33105,
    ErrorCode33122,
    ErrorCode33103,
    ErrorCode33120,
    ErrorCode33104,
    ErrorCode33203,
    ErrorCode33121,
    ErrorCode33118
}


impl<'de> Deserialize<'de> for TwilioProgrammableWirelessError {
            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>
    {
            Ok(
                usize::deserialize(deserializer)
            .map(TwilioProgrammableWirelessError::from_code)?
            .ok_or_else(|| Error::custom("Invalid error code!"))?
            )
    }
}

impl Serialize for TwilioProgrammableWirelessError {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
            usize::serialize(&self.code(), serializer)
    }
}

use crate::products::TwilioProductError;
impl Into<TwilioProductError> for TwilioProgrammableWirelessError {
    fn into(self) -> TwilioProductError {
        TwilioProductError::ProgrammableWireless(self)
    }
}

impl crate::products::SomeTwilioProductError for TwilioProgrammableWirelessError {}

