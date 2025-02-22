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
pub enum TwilioProgrammableVideoError {
    ErrorCode53628,
    ErrorCode53405,
    ErrorCode53000,
    ErrorCode53203,
    ErrorCode53112,
    ErrorCode53102,
    ErrorCode53104,
    ErrorCode53408,
    ErrorCode53106,
    ErrorCode53616,
    ErrorCode53301,
    ErrorCode53300,
    ErrorCode53601,
    ErrorCode53620,
    ErrorCode53627,
    ErrorCode53215,
    ErrorCode53204,
    ErrorCode53621,
    ErrorCode53617,
    ErrorCode53208,
    ErrorCode53205,
    ErrorCode53109,
    ErrorCode53661,
    ErrorCode53120,
    ErrorCode53006,
    ErrorCode53002,
    ErrorCode53207,
    ErrorCode53113,
    ErrorCode53632,
    ErrorCode53103,
    ErrorCode53668,
    ErrorCode53001,
    ErrorCode53206,
    ErrorCode53123,
    ErrorCode53626,
    ErrorCode53664,
    ErrorCode53202,
    ErrorCode53304,
    ErrorCode53622,
    ErrorCode53669,
    ErrorCode53217,
    ErrorCode53501,
    ErrorCode53603,
    ErrorCode53110,
    ErrorCode53500,
    ErrorCode53663,
    ErrorCode53662,
    ErrorCode53605,
    ErrorCode53200,
    ErrorCode53630,
    ErrorCode53107,
    ErrorCode53633,
    ErrorCode53122,
    ErrorCode53625,
    ErrorCode53614,
    ErrorCode53611,
    ErrorCode53402,
    ErrorCode53127,
    ErrorCode53216,
    ErrorCode53101,
    ErrorCode53108,
    ErrorCode53613,
    ErrorCode53400,
    ErrorCode53125,
    ErrorCode53111,
    ErrorCode53607,
    ErrorCode53003,
    ErrorCode53406,
    ErrorCode53667,
    ErrorCode53004,
    ErrorCode53623,
    ErrorCode53302,
    ErrorCode53118,
    ErrorCode53660,
    ErrorCode53666,
    ErrorCode53201,
    ErrorCode53124,
    ErrorCode53407,
    ErrorCode53631,
    ErrorCode53403,
    ErrorCode53105,
    ErrorCode53303,
    ErrorCode53610,
    ErrorCode53126,
    ErrorCode53602,
    ErrorCode53119,
    ErrorCode53604,
    ErrorCode53100,
    ErrorCode53612,
    ErrorCode53600,
    ErrorCode53209,
    ErrorCode53624,
    ErrorCode53121,
    ErrorCode53606,
    ErrorCode53665,
    ErrorCode53615
}


impl<'de> Deserialize<'de> for TwilioProgrammableVideoError {
            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>
    {
            Ok(
                usize::deserialize(deserializer)
            .map(TwilioProgrammableVideoError::from_code)?
            .ok_or_else(|| Error::custom("Invalid error code!"))?
            )
    }
}

impl Serialize for TwilioProgrammableVideoError {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
            usize::serialize(&self.code(), serializer)
    }
}

use crate::products::TwilioProductError;
impl Into<TwilioProductError> for TwilioProgrammableVideoError {
    fn into(self) -> TwilioProductError {
        TwilioProductError::ProgrammableVideo(self)
    }
}

impl crate::products::SomeTwilioProductError for TwilioProgrammableVideoError {}

