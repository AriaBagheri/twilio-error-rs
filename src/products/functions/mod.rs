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
pub enum TwilioFunctionsError {
    ErrorCode82001,
    ErrorCode82009,
    ErrorCode82006,
    ErrorCode82005,
    ErrorCode82002,
    ErrorCode82004,
    ErrorCode82007,
    ErrorCode82008,
    ErrorCode82003
}


impl<'de> Deserialize<'de> for TwilioFunctionsError {
            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>
    {
            Ok(
                usize::deserialize(deserializer)
            .map(TwilioFunctionsError::from_code)?
            .ok_or_else(|| Error::custom("Invalid error code!"))?
            )
    }
}

impl Serialize for TwilioFunctionsError {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
            usize::serialize(&self.code(), serializer)
    }
}

use crate::products::TwilioProductError;
impl Into<TwilioProductError> for TwilioFunctionsError {
    fn into(self) -> TwilioProductError {
        TwilioProductError::Functions(self)
    }
}

impl crate::products::SomeTwilioProductError for TwilioFunctionsError {}

