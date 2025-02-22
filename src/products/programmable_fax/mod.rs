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
pub enum TwilioProgrammableFaxError {
    ErrorCode34004,
    ErrorCode34003,
    ErrorCode34108,
    ErrorCode34106,
    ErrorCode34002,
    ErrorCode34005
}


impl<'de> Deserialize<'de> for TwilioProgrammableFaxError {
            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>
    {
            Ok(
                usize::deserialize(deserializer)
            .map(TwilioProgrammableFaxError::from_code)?
            .ok_or_else(|| Error::custom("Invalid error code!"))?
            )
    }
}

impl Serialize for TwilioProgrammableFaxError {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
            usize::serialize(&self.code(), serializer)
    }
}

use crate::products::TwilioProductError;
impl Into<TwilioProductError> for TwilioProgrammableFaxError {
    fn into(self) -> TwilioProductError {
        TwilioProductError::ProgrammableFax(self)
    }
}

impl crate::products::SomeTwilioProductError for TwilioProgrammableFaxError {}

