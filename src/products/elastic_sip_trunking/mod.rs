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
pub enum TwilioElasticSipTrunkingError {
    ErrorCode21244,
    ErrorCode21257,
    ErrorCode21253,
    ErrorCode32208,
    ErrorCode32219,
    ErrorCode21248,
    ErrorCode32202,
    ErrorCode32205,
    ErrorCode21252,
    ErrorCode32207,
    ErrorCode21249,
    ErrorCode21256,
    ErrorCode21259,
    ErrorCode32218,
    ErrorCode32001,
    ErrorCode32005,
    ErrorCode21247,
    ErrorCode32203,
    ErrorCode32100,
    ErrorCode32012,
    ErrorCode32011,
    ErrorCode32201,
    ErrorCode21245,
    ErrorCode32010,
    ErrorCode32206,
    ErrorCode21260,
    ErrorCode21261,
    ErrorCode32115,
    ErrorCode21258,
    ErrorCode21634,
    ErrorCode32222,
    ErrorCode21254,
    ErrorCode21251,
    ErrorCode32204,
    ErrorCode32200,
    ErrorCode32020
}


impl<'de> Deserialize<'de> for TwilioElasticSipTrunkingError {
            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>
    {
            Ok(
                usize::deserialize(deserializer)
            .map(TwilioElasticSipTrunkingError::from_code)?
            .ok_or_else(|| Error::custom("Invalid error code!"))?
            )
    }
}

impl Serialize for TwilioElasticSipTrunkingError {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
            usize::serialize(&self.code(), serializer)
    }
}

use crate::TwilioProductError;
impl Into<TwilioProductError> for TwilioElasticSipTrunkingError {
    fn into(self) -> TwilioProductError {
        TwilioProductError::ElasticSipTrunking(self)
    }
}

impl crate::products::SomeTwilioProductError for TwilioElasticSipTrunkingError {}

