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
pub enum TwilioLookupError {
    ErrorCode60616,
    ErrorCode60699,
    ErrorCode60606,
    ErrorCode60620,
    ErrorCode60614,
    ErrorCode60624,
    ErrorCode60611,
    ErrorCode60619,
    ErrorCode60612,
    ErrorCode60621,
    ErrorCode60609,
    ErrorCode60607,
    ErrorCode60618,
    ErrorCode60601,
    ErrorCode60610,
    ErrorCode60608,
    ErrorCode60623,
    ErrorCode60617,
    ErrorCode60613,
    ErrorCode60622,
    ErrorCode60600
}


impl<'de> Deserialize<'de> for TwilioLookupError {
            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>
    {
            Ok(
                usize::deserialize(deserializer)
            .map(TwilioLookupError::from_code)?
            .ok_or_else(|| Error::custom("Invalid error code!"))?
            )
    }
}

impl Serialize for TwilioLookupError {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
            usize::serialize(&self.code(), serializer)
    }
}

use crate::products::TwilioProductError;
impl Into<TwilioProductError> for TwilioLookupError {
    fn into(self) -> TwilioProductError {
        TwilioProductError::Lookup(self)
    }
}

impl crate::products::SomeTwilioProductError for TwilioLookupError {}

