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
pub enum TwilioSyncError {
    ErrorCode54011,
    ErrorCode54006,
    ErrorCode54201,
    ErrorCode54155,
    ErrorCode54451,
    ErrorCode54351,
    ErrorCode54156,
    ErrorCode54450,
    ErrorCode54251,
    ErrorCode54100,
    ErrorCode54003,
    ErrorCode54509,
    ErrorCode54103,
    ErrorCode54101,
    ErrorCode54507,
    ErrorCode54053,
    ErrorCode54452,
    ErrorCode54301,
    ErrorCode54009,
    ErrorCode54051,
    ErrorCode54354,
    ErrorCode54502,
    ErrorCode54453,
    ErrorCode54300,
    ErrorCode54206,
    ErrorCode54200,
    ErrorCode54302,
    ErrorCode54208,
    ErrorCode54510,
    ErrorCode54050,
    ErrorCode54151,
    ErrorCode54150,
    ErrorCode54007,
    ErrorCode54008,
    ErrorCode54209,
    ErrorCode54458,
    ErrorCode54205,
    ErrorCode54056,
    ErrorCode54010,
    ErrorCode54454,
    ErrorCode54419,
    ErrorCode54250
}


impl<'de> Deserialize<'de> for TwilioSyncError {
            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>
    {
            Ok(
                usize::deserialize(deserializer)
            .map(TwilioSyncError::from_code)?
            .ok_or_else(|| Error::custom("Invalid error code!"))?
            )
    }
}

impl Serialize for TwilioSyncError {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
            usize::serialize(&self.code(), serializer)
    }
}

use crate::products::TwilioProductError;
impl Into<TwilioProductError> for TwilioSyncError {
    fn into(self) -> TwilioProductError {
        TwilioProductError::Sync(self)
    }
}

impl crate::products::SomeTwilioProductError for TwilioSyncError {}

