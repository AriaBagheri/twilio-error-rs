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
pub enum TwilioEventsError {
    ErrorCode91003,
    ErrorCode91004,
    ErrorCode93104,
    ErrorCode91101,
    ErrorCode91007,
    ErrorCode91006,
    ErrorCode91005,
    ErrorCode91000,
    ErrorCode93101,
    ErrorCode93103,
    ErrorCode91102,
    ErrorCode91104,
    ErrorCode91103,
    ErrorCode91001,
    ErrorCode91201,
    ErrorCode91002,
    ErrorCode93100,
    ErrorCode91100,
    ErrorCode93102
}


impl<'de> Deserialize<'de> for TwilioEventsError {
            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>
    {
            Ok(
                usize::deserialize(deserializer)
            .map(TwilioEventsError::from_code)?
            .ok_or_else(|| Error::custom("Invalid error code!"))?
            )
    }
}

impl Serialize for TwilioEventsError {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
            usize::serialize(&self.code(), serializer)
    }
}

use crate::products::TwilioProductError;
impl Into<TwilioProductError> for TwilioEventsError {
    fn into(self) -> TwilioProductError {
        TwilioProductError::Events(self)
    }
}

impl crate::products::SomeTwilioProductError for TwilioEventsError {}

