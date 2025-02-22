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
pub enum TwilioFlexError {
    ErrorCode45357,
    ErrorCode45784,
    ErrorCode45796,
    ErrorCode45770,
    ErrorCode45782,
    ErrorCode49008,
    ErrorCode45351,
    ErrorCode45376,
    ErrorCode90002,
    ErrorCode45308,
    ErrorCode45731,
    ErrorCode45601,
    ErrorCode45356,
    ErrorCode45797,
    ErrorCode45776,
    ErrorCode45309,
    ErrorCode45373,
    ErrorCode45101,
    ErrorCode45377,
    ErrorCode45711,
    ErrorCode45362,
    ErrorCode45374,
    ErrorCode45207,
    ErrorCode45773,
    ErrorCode45700,
    ErrorCode45103,
    ErrorCode45361,
    ErrorCode45719,
    ErrorCode45003,
    ErrorCode45307,
    ErrorCode45355,
    ErrorCode45371,
    ErrorCode45762,
    ErrorCode45203,
    ErrorCode45744,
    ErrorCode45109,
    ErrorCode45010,
    ErrorCode45206,
    ErrorCode45701,
    ErrorCode45367,
    ErrorCode45381,
    ErrorCode45794,
    ErrorCode45208,
    ErrorCode45370,
    ErrorCode45358,
    ErrorCode45005,
    ErrorCode45360,
    ErrorCode45733,
    ErrorCode49009,
    ErrorCode45002,
    ErrorCode45760,
    ErrorCode90003,
    ErrorCode45302,
    ErrorCode45375,
    ErrorCode45301,
    ErrorCode45369,
    ErrorCode45366,
    ErrorCode45771,
    ErrorCode45763,
    ErrorCode45209,
    ErrorCode45780,
    ErrorCode45359,
    ErrorCode45009,
    ErrorCode45102,
    ErrorCode45600,
    ErrorCode45352,
    ErrorCode90004,
    ErrorCode45778,
    ErrorCode45745,
    ErrorCode45006,
    ErrorCode45764,
    ErrorCode45201,
    ErrorCode45772,
    ErrorCode45378,
    ErrorCode45795,
    ErrorCode45210,
    ErrorCode45402,
    ErrorCode45785,
    ErrorCode45004,
    ErrorCode45372,
    ErrorCode45211,
    ErrorCode45353,
    ErrorCode45305,
    ErrorCode45777,
    ErrorCode45775,
    ErrorCode93105,
    ErrorCode45741,
    ErrorCode45202,
    ErrorCode45312,
    ErrorCode45007,
    ErrorCode45788,
    ErrorCode45313,
    ErrorCode45781,
    ErrorCode456001,
    ErrorCode456002,
    ErrorCode45401,
    ErrorCode45761,
    ErrorCode45765,
    ErrorCode45789,
    ErrorCode45303,
    ErrorCode45304,
    ErrorCode45205,
    ErrorCode45363,
    ErrorCode45354,
    ErrorCode90000,
    ErrorCode45008,
    ErrorCode45380,
    ErrorCode45212,
    ErrorCode45368,
    ErrorCode45310,
    ErrorCode45001,
    ErrorCode45050,
    ErrorCode45715,
    ErrorCode45306,
    ErrorCode45350,
    ErrorCode45774,
    ErrorCode45379,
    ErrorCode45204
}


impl<'de> Deserialize<'de> for TwilioFlexError {
            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>
    {
            Ok(
                usize::deserialize(deserializer)
            .map(TwilioFlexError::from_code)?
            .ok_or_else(|| Error::custom("Invalid error code!"))?
            )
    }
}

impl Serialize for TwilioFlexError {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
            usize::serialize(&self.code(), serializer)
    }
}

use crate::TwilioProductError;
impl Into<TwilioProductError> for TwilioFlexError {
    fn into(self) -> TwilioProductError {
        TwilioProductError::Flex(self)
    }
}

impl crate::products::SomeTwilioProductError for TwilioFlexError {}

