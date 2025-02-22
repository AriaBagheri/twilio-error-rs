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
pub enum TwilioFrontlineError {
    ErrorCode48028,
    ErrorCode48005,
    ErrorCode48031,
    ErrorCode48029,
    ErrorCode48011,
    ErrorCode48027,
    ErrorCode48025,
    ErrorCode48032,
    ErrorCode48030,
    ErrorCode48050,
    ErrorCode48001,
    ErrorCode48024,
    ErrorCode48010,
    ErrorCode48004,
    ErrorCode48000,
    ErrorCode48002,
    ErrorCode48026,
    ErrorCode48003,
    ErrorCode48023,
    ErrorCode48033
}


impl<'de> Deserialize<'de> for TwilioFrontlineError {
            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>
    {
            Ok(
                usize::deserialize(deserializer)
            .map(TwilioFrontlineError::from_code)?
            .ok_or_else(|| Error::custom("Invalid error code!"))?
            )
    }
}

impl Serialize for TwilioFrontlineError {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
            usize::serialize(&self.code(), serializer)
    }
}

use crate::TwilioProductError;
impl Into<TwilioProductError> for TwilioFrontlineError {
    fn into(self) -> TwilioProductError {
        TwilioProductError::Frontline(self)
    }
}

impl crate::products::SomeTwilioProductError for TwilioFrontlineError {}

