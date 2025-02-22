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
pub enum TwilioAiAssistantsError {
    ErrorCode99001,
    ErrorCode99004,
    ErrorCode99002,
    ErrorCode99006,
    ErrorCode99003,
    ErrorCode99005
}


impl<'de> Deserialize<'de> for TwilioAiAssistantsError {
            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>
    {
            Ok(
                usize::deserialize(deserializer)
            .map(TwilioAiAssistantsError::from_code)?
            .ok_or_else(|| Error::custom("Invalid error code!"))?
            )
    }
}

impl Serialize for TwilioAiAssistantsError {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
            usize::serialize(&self.code(), serializer)
    }
}

use crate::TwilioProductError;
impl Into<TwilioProductError> for TwilioAiAssistantsError {
    fn into(self) -> TwilioProductError {
        TwilioProductError::AiAssistants(self)
    }
}

impl crate::products::SomeTwilioProductError for TwilioAiAssistantsError {}

