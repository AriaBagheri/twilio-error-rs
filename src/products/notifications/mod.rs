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
pub enum TwilioNotificationsError {
    ErrorCode52108,
    ErrorCode52202,
    ErrorCode52052,
    ErrorCode52211,
    ErrorCode52102,
    ErrorCode52143,
    ErrorCode52171,
    ErrorCode52131,
    ErrorCode52133,
    ErrorCode52163,
    ErrorCode52162,
    ErrorCode52103,
    ErrorCode52164,
    ErrorCode52137,
    ErrorCode52145,
    ErrorCode52001,
    ErrorCode52109,
    ErrorCode52172,
    ErrorCode52104,
    ErrorCode52071,
    ErrorCode52105,
    ErrorCode52213,
    ErrorCode52167,
    ErrorCode52214,
    ErrorCode52304,
    ErrorCode52140,
    ErrorCode52215,
    ErrorCode52182,
    ErrorCode52149,
    ErrorCode52305,
    ErrorCode52181,
    ErrorCode52144,
    ErrorCode52166,
    ErrorCode52309,
    ErrorCode52401,
    ErrorCode52203,
    ErrorCode52165,
    ErrorCode52135,
    ErrorCode52107,
    ErrorCode52072,
    ErrorCode52111,
    ErrorCode52307,
    ErrorCode52000,
    ErrorCode52306,
    ErrorCode52136,
    ErrorCode52051,
    ErrorCode52174,
    ErrorCode52302,
    ErrorCode52138,
    ErrorCode52170,
    ErrorCode52141,
    ErrorCode52161,
    ErrorCode52301,
    ErrorCode52142,
    ErrorCode52303,
    ErrorCode52148,
    ErrorCode52212,
    ErrorCode52003,
    ErrorCode52168,
    ErrorCode52147,
    ErrorCode52002,
    ErrorCode52139,
    ErrorCode52101,
    ErrorCode52173,
    ErrorCode52106,
    ErrorCode52110,
    ErrorCode52201,
    ErrorCode52134
}


impl<'de> Deserialize<'de> for TwilioNotificationsError {
            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>
    {
            Ok(
                usize::deserialize(deserializer)
            .map(TwilioNotificationsError::from_code)?
            .ok_or_else(|| Error::custom("Invalid error code!"))?
            )
    }
}

impl Serialize for TwilioNotificationsError {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
            usize::serialize(&self.code(), serializer)
    }
}

use crate::products::TwilioProductError;
impl Into<TwilioProductError> for TwilioNotificationsError {
    fn into(self) -> TwilioProductError {
        TwilioProductError::Notifications(self)
    }
}

impl crate::products::SomeTwilioProductError for TwilioNotificationsError {}

