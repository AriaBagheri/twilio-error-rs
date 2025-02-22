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
pub enum TwilioProgrammableChatError {
    ErrorCode50304,
    ErrorCode50412,
    ErrorCode50417,
    ErrorCode50208,
    ErrorCode50052,
    ErrorCode50205,
    ErrorCode50207,
    ErrorCode50054,
    ErrorCode50074,
    ErrorCode50384,
    ErrorCode50434,
    ErrorCode50509,
    ErrorCode50077,
    ErrorCode50451,
    ErrorCode50059,
    ErrorCode50431,
    ErrorCode50375,
    ErrorCode50380,
    ErrorCode50449,
    ErrorCode50057,
    ErrorCode50340,
    ErrorCode50377,
    ErrorCode50506,
    ErrorCode50101,
    ErrorCode50332,
    ErrorCode50393,
    ErrorCode50436,
    ErrorCode50374,
    ErrorCode50330,
    ErrorCode50382,
    ErrorCode50418,
    ErrorCode50378,
    ErrorCode50438,
    ErrorCode50107,
    ErrorCode50201,
    ErrorCode50328,
    ErrorCode50104,
    ErrorCode50309,
    ErrorCode50363,
    ErrorCode50414,
    ErrorCode50310,
    ErrorCode50371,
    ErrorCode50373,
    ErrorCode50442,
    ErrorCode50370,
    ErrorCode50511,
    ErrorCode50327,
    ErrorCode50211,
    ErrorCode50602,
    ErrorCode50437,
    ErrorCode50051,
    ErrorCode50408,
    ErrorCode50440,
    ErrorCode50351,
    ErrorCode50213,
    ErrorCode50100,
    ErrorCode50369,
    ErrorCode50516,
    ErrorCode50204,
    ErrorCode50416,
    ErrorCode50053,
    ErrorCode50513,
    ErrorCode50419,
    ErrorCode50413,
    ErrorCode50362,
    ErrorCode50320,
    ErrorCode50202,
    ErrorCode50105,
    ErrorCode50058,
    ErrorCode50422,
    ErrorCode50003,
    ErrorCode50324,
    ErrorCode50421,
    ErrorCode50361,
    ErrorCode50364,
    ErrorCode50415,
    ErrorCode50212,
    ErrorCode50600,
    ErrorCode50203,
    ErrorCode50056,
    ErrorCode50403,
    ErrorCode50420,
    ErrorCode50321,
    ErrorCode50502,
    ErrorCode19038,
    ErrorCode50401,
    ErrorCode50507,
    ErrorCode50206,
    ErrorCode50435,
    ErrorCode50303,
    ErrorCode50078,
    ErrorCode50379,
    ErrorCode50350,
    ErrorCode50060,
    ErrorCode50331,
    ErrorCode50329,
    ErrorCode50441,
    ErrorCode50391,
    ErrorCode50106,
    ErrorCode50368,
    ErrorCode50214,
    ErrorCode50512,
    ErrorCode50326,
    ErrorCode50405,
    ErrorCode50006,
    ErrorCode50209,
    ErrorCode50306,
    ErrorCode50508,
    ErrorCode20161,
    ErrorCode50400,
    ErrorCode50341,
    ErrorCode50407,
    ErrorCode50433,
    ErrorCode50200,
    ErrorCode50402,
    ErrorCode50432,
    ErrorCode50367,
    ErrorCode50055,
    ErrorCode50061,
    ErrorCode50004,
    ErrorCode50610,
    ErrorCode50305,
    ErrorCode50322,
    ErrorCode50334,
    ErrorCode50406,
    ErrorCode50050,
    ErrorCode50349,
    ErrorCode50503,
    ErrorCode50109,
    ErrorCode50065,
    ErrorCode50443,
    ErrorCode50342,
    ErrorCode50076,
    ErrorCode50390,
    ErrorCode50376,
    ErrorCode50366,
    ErrorCode50392,
    ErrorCode50500,
    ErrorCode50002,
    ErrorCode50308,
    ErrorCode50103,
    ErrorCode50504,
    ErrorCode50307,
    ErrorCode50300,
    ErrorCode50409,
    ErrorCode50325,
    ErrorCode50404,
    ErrorCode50323,
    ErrorCode50063,
    ErrorCode50360,
    ErrorCode50444,
    ErrorCode50302,
    ErrorCode50102,
    ErrorCode50411,
    ErrorCode50372,
    ErrorCode50430,
    ErrorCode50000,
    ErrorCode50601,
    ErrorCode50439,
    ErrorCode50510,
    ErrorCode50347,
    ErrorCode50210,
    ErrorCode50501,
    ErrorCode50001,
    ErrorCode50505,
    ErrorCode50365,
    ErrorCode50301,
    ErrorCode50385
}


impl<'de> Deserialize<'de> for TwilioProgrammableChatError {
            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>
    {
            Ok(
                usize::deserialize(deserializer)
            .map(TwilioProgrammableChatError::from_code)?
            .ok_or_else(|| Error::custom("Invalid error code!"))?
            )
    }
}

impl Serialize for TwilioProgrammableChatError {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
            usize::serialize(&self.code(), serializer)
    }
}

use crate::products::TwilioProductError;
impl Into<TwilioProductError> for TwilioProgrammableChatError {
    fn into(self) -> TwilioProductError {
        TwilioProductError::ProgrammableChat(self)
    }
}

impl crate::products::SomeTwilioProductError for TwilioProgrammableChatError {}

