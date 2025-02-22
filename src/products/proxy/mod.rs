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
pub enum TwilioProxyError {
    ErrorCode80612,
    ErrorCode80611,
    ErrorCode80911,
    ErrorCode80308,
    ErrorCode80623,
    ErrorCode80621,
    ErrorCode80619,
    ErrorCode80607,
    ErrorCode80306,
    ErrorCode80904,
    ErrorCode80504,
    ErrorCode80802,
    ErrorCode80614,
    ErrorCode80208,
    ErrorCode80301,
    ErrorCode80101,
    ErrorCode80617,
    ErrorCode80206,
    ErrorCode80913,
    ErrorCode80502,
    ErrorCode80624,
    ErrorCode80622,
    ErrorCode80501,
    ErrorCode80618,
    ErrorCode80304,
    ErrorCode80901,
    ErrorCode80505,
    ErrorCode80207,
    ErrorCode80203,
    ErrorCode80201,
    ErrorCode80613,
    ErrorCode80910,
    ErrorCode80305,
    ErrorCode80909,
    ErrorCode80103,
    ErrorCode80616,
    ErrorCode80620,
    ErrorCode80801,
    ErrorCode80506,
    ErrorCode80404,
    ErrorCode80608,
    ErrorCode80615,
    ErrorCode80625
}


impl<'de> Deserialize<'de> for TwilioProxyError {
            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>
    {
            Ok(
                usize::deserialize(deserializer)
            .map(TwilioProxyError::from_code)?
            .ok_or_else(|| Error::custom("Invalid error code!"))?
            )
    }
}

impl Serialize for TwilioProxyError {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
            usize::serialize(&self.code(), serializer)
    }
}

use crate::TwilioProductError;
impl Into<TwilioProductError> for TwilioProxyError {
    fn into(self) -> TwilioProductError {
        TwilioProductError::Proxy(self)
    }
}

impl crate::products::SomeTwilioProductError for TwilioProxyError {}

