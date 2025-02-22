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
pub enum TwilioPlatformError {
    ErrorCode20101,
    ErrorCode20409,
    ErrorCode20162,
    ErrorCode20157,
    ErrorCode21102,
    ErrorCode20159,
    ErrorCode20105,
    ErrorCode20404,
    ErrorCode20154,
    ErrorCode20107,
    ErrorCode20155,
    ErrorCode20153,
    ErrorCode20156,
    ErrorCode20104,
    ErrorCode20102,
    ErrorCode97001,
    ErrorCode20151,
    ErrorCode20103,
    ErrorCode21481,
    ErrorCode20403,
    ErrorCode20160,
    ErrorCode20152,
    ErrorCode20106
}


impl<'de> Deserialize<'de> for TwilioPlatformError {
            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>
    {
            Ok(
                usize::deserialize(deserializer)
            .map(TwilioPlatformError::from_code)?
            .ok_or_else(|| Error::custom("Invalid error code!"))?
            )
    }
}

impl Serialize for TwilioPlatformError {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
            usize::serialize(&self.code(), serializer)
    }
}

use crate::TwilioProductError;
impl Into<TwilioProductError> for TwilioPlatformError {
    fn into(self) -> TwilioProductError {
        TwilioProductError::Platform(self)
    }
}

impl crate::products::SomeTwilioProductError for TwilioPlatformError {}

