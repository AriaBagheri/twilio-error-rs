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
pub enum TwilioTaskrouterError {
    ErrorCode40114,
    ErrorCode40121,
    ErrorCode14233,
    ErrorCode40142,
    ErrorCode14236,
    ErrorCode40143,
    ErrorCode40001,
    ErrorCode40120,
    ErrorCode40130,
    ErrorCode40138,
    ErrorCode40135,
    ErrorCode40152,
    ErrorCode40005,
    ErrorCode14238,
    ErrorCode40155,
    ErrorCode40140,
    ErrorCode14230,
    ErrorCode14217,
    ErrorCode40158,
    ErrorCode40157,
    ErrorCode40137,
    ErrorCode40147,
    ErrorCode40151,
    ErrorCode40134,
    ErrorCode14239,
    ErrorCode40000,
    ErrorCode40123,
    ErrorCode14222,
    ErrorCode14240,
    ErrorCode14235,
    ErrorCode14234,
    ErrorCode14237,
    ErrorCode40111,
    ErrorCode14241,
    ErrorCode40144,
    ErrorCode40100,
    ErrorCode40149,
    ErrorCode40131,
    ErrorCode40141,
    ErrorCode14231,
    ErrorCode55555,
    ErrorCode40153,
    ErrorCode14232,
    ErrorCode40145,
    ErrorCode40136,
    ErrorCode40132,
    ErrorCode40112,
    ErrorCode40122,
    ErrorCode40113,
    ErrorCode40133,
    ErrorCode40148,
    ErrorCode40154,
    ErrorCode40159,
    ErrorCode40139,
    ErrorCode40110,
    ErrorCode40146
}


impl<'de> Deserialize<'de> for TwilioTaskrouterError {
            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>
    {
            Ok(
                usize::deserialize(deserializer)
            .map(TwilioTaskrouterError::from_code)?
            .ok_or_else(|| Error::custom("Invalid error code!"))?
            )
    }
}

impl Serialize for TwilioTaskrouterError {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
            usize::serialize(&self.code(), serializer)
    }
}

use crate::products::TwilioProductError;
impl Into<TwilioProductError> for TwilioTaskrouterError {
    fn into(self) -> TwilioProductError {
        TwilioProductError::Taskrouter(self)
    }
}

impl crate::products::SomeTwilioProductError for TwilioTaskrouterError {}

