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
pub enum TwilioBrandedCommsError {
    ErrorCode60704,
    ErrorCode60723,
    ErrorCode60712,
    ErrorCode60719,
    ErrorCode60721,
    ErrorCode60727,
    ErrorCode60706,
    ErrorCode60709,
    ErrorCode60716,
    ErrorCode60702,
    ErrorCode60714,
    ErrorCode60713,
    ErrorCode60707,
    ErrorCode60710,
    ErrorCode60722,
    ErrorCode60703,
    ErrorCode60700,
    ErrorCode60725,
    ErrorCode60715,
    ErrorCode60717,
    ErrorCode60708,
    ErrorCode60701,
    ErrorCode60726,
    ErrorCode60724,
    ErrorCode60711
}


impl<'de> Deserialize<'de> for TwilioBrandedCommsError {
            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>
    {
            Ok(
                usize::deserialize(deserializer)
            .map(TwilioBrandedCommsError::from_code)?
            .ok_or_else(|| Error::custom("Invalid error code!"))?
            )
    }
}

impl Serialize for TwilioBrandedCommsError {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
            usize::serialize(&self.code(), serializer)
    }
}

use crate::products::TwilioProductError;
impl Into<TwilioProductError> for TwilioBrandedCommsError {
    fn into(self) -> TwilioProductError {
        TwilioProductError::BrandedComms(self)
    }
}

impl crate::products::SomeTwilioProductError for TwilioBrandedCommsError {}

