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
pub enum TwilioConversationsError {
    ErrorCode50707,
    ErrorCode50426,
    ErrorCode50534,
    ErrorCode50526,
    ErrorCode50386,
    ErrorCode50705,
    ErrorCode50529,
    ErrorCode50708,
    ErrorCode50704,
    ErrorCode50396,
    ErrorCode50452,
    ErrorCode50514,
    ErrorCode50528,
    ErrorCode50542,
    ErrorCode50701,
    ErrorCode50532,
    ErrorCode50706,
    ErrorCode50424,
    ErrorCode50700,
    ErrorCode50703,
    ErrorCode50527,
    ErrorCode50427,
    ErrorCode50541,
    ErrorCode50710,
    ErrorCode50709,
    ErrorCode50531,
    ErrorCode50530,
    ErrorCode50533,
    ErrorCode50540,
    ErrorCode50702,
    ErrorCode50711,
    ErrorCode50453,
    ErrorCode50425
}


impl<'de> Deserialize<'de> for TwilioConversationsError {
            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>
    {
            Ok(
                usize::deserialize(deserializer)
            .map(TwilioConversationsError::from_code)?
            .ok_or_else(|| Error::custom("Invalid error code!"))?
            )
    }
}

impl Serialize for TwilioConversationsError {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
            usize::serialize(&self.code(), serializer)
    }
}

use crate::products::TwilioProductError;
impl Into<TwilioProductError> for TwilioConversationsError {
    fn into(self) -> TwilioProductError {
        TwilioProductError::Conversations(self)
    }
}

impl crate::products::SomeTwilioProductError for TwilioConversationsError {}

