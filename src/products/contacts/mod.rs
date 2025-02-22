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
pub enum TwilioContactsError {
    ErrorCode19045,
    ErrorCode19053,
    ErrorCode19057,
    ErrorCode19056,
    ErrorCode19044,
    ErrorCode19046,
    ErrorCode19055,
    ErrorCode19048,
    ErrorCode19054,
    ErrorCode19047,
    ErrorCode19052,
    ErrorCode19043,
    ErrorCode19049
}


impl<'de> Deserialize<'de> for TwilioContactsError {
            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>
    {
            Ok(
                usize::deserialize(deserializer)
            .map(TwilioContactsError::from_code)?
            .ok_or_else(|| Error::custom("Invalid error code!"))?
            )
    }
}

impl Serialize for TwilioContactsError {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
            usize::serialize(&self.code(), serializer)
    }
}

use crate::TwilioProductError;
impl Into<TwilioProductError> for TwilioContactsError {
    fn into(self) -> TwilioProductError {
        TwilioProductError::Contacts(self)
    }
}

impl crate::products::SomeTwilioProductError for TwilioContactsError {}

