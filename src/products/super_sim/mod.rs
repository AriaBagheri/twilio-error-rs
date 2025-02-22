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
pub enum TwilioSuperSimError {
    ErrorCode83602,
    ErrorCode83603,
    ErrorCode83003,
    ErrorCode83001,
    ErrorCode83002,
    ErrorCode83007,
    ErrorCode83605,
    ErrorCode83008,
    ErrorCode83010,
    ErrorCode83703,
    ErrorCode83500,
    ErrorCode83705,
    ErrorCode83005,
    ErrorCode83700,
    ErrorCode83011,
    ErrorCode83402,
    ErrorCode83000,
    ErrorCode83702,
    ErrorCode83401,
    ErrorCode83400,
    ErrorCode83004,
    ErrorCode83604,
    ErrorCode83006,
    ErrorCode83600,
    ErrorCode83601,
    ErrorCode83009,
    ErrorCode83701,
    ErrorCode83704
}


impl<'de> Deserialize<'de> for TwilioSuperSimError {
            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>
    {
            Ok(
                usize::deserialize(deserializer)
            .map(TwilioSuperSimError::from_code)?
            .ok_or_else(|| Error::custom("Invalid error code!"))?
            )
    }
}

impl Serialize for TwilioSuperSimError {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
            usize::serialize(&self.code(), serializer)
    }
}

use crate::TwilioProductError;
impl Into<TwilioProductError> for TwilioSuperSimError {
    fn into(self) -> TwilioProductError {
        TwilioProductError::SuperSim(self)
    }
}

impl crate::products::SomeTwilioProductError for TwilioSuperSimError {}

