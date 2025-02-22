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
pub enum TwilioVoiceIntelligenceError {
    ErrorCode95112,
    ErrorCode95105,
    ErrorCode95201,
    ErrorCode95104,
    ErrorCode95110,
    ErrorCode95115,
    ErrorCode95116,
    ErrorCode95100,
    ErrorCode95119,
    ErrorCode95108,
    ErrorCode95114,
    ErrorCode95200,
    ErrorCode95118,
    ErrorCode95111,
    ErrorCode95302,
    ErrorCode95300,
    ErrorCode95006,
    ErrorCode95005,
    ErrorCode95102,
    ErrorCode95106,
    ErrorCode95109,
    ErrorCode95250,
    ErrorCode95301,
    ErrorCode95113,
    ErrorCode95103
}


impl<'de> Deserialize<'de> for TwilioVoiceIntelligenceError {
            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>
    {
            Ok(
                usize::deserialize(deserializer)
            .map(TwilioVoiceIntelligenceError::from_code)?
            .ok_or_else(|| Error::custom("Invalid error code!"))?
            )
    }
}

impl Serialize for TwilioVoiceIntelligenceError {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
            usize::serialize(&self.code(), serializer)
    }
}

use crate::products::TwilioProductError;
impl Into<TwilioProductError> for TwilioVoiceIntelligenceError {
    fn into(self) -> TwilioProductError {
        TwilioProductError::VoiceIntelligence(self)
    }
}

impl crate::products::SomeTwilioProductError for TwilioVoiceIntelligenceError {}

