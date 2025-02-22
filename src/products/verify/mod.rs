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
pub enum TwilioVerifyError {
    ErrorCode60367,
    ErrorCode60215,
    ErrorCode60378,
    ErrorCode60374,
    ErrorCode60373,
    ErrorCode60213,
    ErrorCode60424,
    ErrorCode60388,
    ErrorCode60365,
    ErrorCode60237,
    ErrorCode60312,
    ErrorCode60318,
    ErrorCode60234,
    ErrorCode68006,
    ErrorCode60235,
    ErrorCode60407,
    ErrorCode60306,
    ErrorCode60236,
    ErrorCode60421,
    ErrorCode60218,
    ErrorCode60310,
    ErrorCode60381,
    ErrorCode60377,
    ErrorCode60382,
    ErrorCode60440,
    ErrorCode60211,
    ErrorCode60324,
    ErrorCode60240,
    ErrorCode60205,
    ErrorCode60519,
    ErrorCode60247,
    ErrorCode60229,
    ErrorCode60246,
    ErrorCode60375,
    ErrorCode60425,
    ErrorCode60431,
    ErrorCode60500,
    ErrorCode60371,
    ErrorCode60401,
    ErrorCode60410,
    ErrorCode60604,
    ErrorCode60311,
    ErrorCode60517,
    ErrorCode60245,
    ErrorCode60369,
    ErrorCode60441,
    ErrorCode60602,
    ErrorCode60433,
    ErrorCode60212,
    ErrorCode60208,
    ErrorCode60385,
    ErrorCode60532,
    ErrorCode60386,
    ErrorCode60390,
    ErrorCode60361,
    ErrorCode60408,
    ErrorCode60323,
    ErrorCode60411,
    ErrorCode60209,
    ErrorCode68002,
    ErrorCode60325,
    ErrorCode60221,
    ErrorCode60363,
    ErrorCode60383,
    ErrorCode60315,
    ErrorCode60404,
    ErrorCode60437,
    ErrorCode60534,
    ErrorCode60434,
    ErrorCode60223,
    ErrorCode60368,
    ErrorCode60510,
    ErrorCode60228,
    ErrorCode60232,
    ErrorCode60326,
    ErrorCode60308,
    ErrorCode60317,
    ErrorCode60242,
    ErrorCode60202,
    ErrorCode60380,
    ErrorCode60200,
    ErrorCode60333,
    ErrorCode60313,
    ErrorCode60427,
    ErrorCode60224,
    ErrorCode60366,
    ErrorCode60550,
    ErrorCode60362,
    ErrorCode60220,
    ErrorCode60372,
    ErrorCode60511,
    ErrorCode60201,
    ErrorCode60436,
    ErrorCode60307,
    ErrorCode60330,
    ErrorCode60376,
    ErrorCode60520,
    ErrorCode60300,
    ErrorCode60406,
    ErrorCode60392,
    ErrorCode60329,
    ErrorCode60420,
    ErrorCode60370,
    ErrorCode60533,
    ErrorCode60238,
    ErrorCode60403,
    ErrorCode60387,
    ErrorCode60531,
    ErrorCode60239,
    ErrorCode60402,
    ErrorCode68007,
    ErrorCode60364,
    ErrorCode60244,
    ErrorCode60225,
    ErrorCode60540,
    ErrorCode60423,
    ErrorCode60226,
    ErrorCode60222,
    ErrorCode60217,
    ErrorCode68005,
    ErrorCode60432,
    ErrorCode60334,
    ErrorCode60207,
    ErrorCode60605,
    ErrorCode60428,
    ErrorCode60204,
    ErrorCode60231,
    ErrorCode60227,
    ErrorCode60603,
    ErrorCode68004,
    ErrorCode60391,
    ErrorCode68003,
    ErrorCode60206,
    ErrorCode60219,
    ErrorCode60233,
    ErrorCode60728,
    ErrorCode60409,
    ErrorCode60331,
    ErrorCode60405,
    ErrorCode60322,
    ErrorCode60379,
    ErrorCode60328,
    ErrorCode60314,
    ErrorCode60302,
    ErrorCode60301,
    ErrorCode60305,
    ErrorCode60203,
    ErrorCode60422,
    ErrorCode60384,
    ErrorCode60309,
    ErrorCode60243,
    ErrorCode60214,
    ErrorCode68001,
    ErrorCode60241,
    ErrorCode60426,
    ErrorCode60210,
    ErrorCode60327
}


impl<'de> Deserialize<'de> for TwilioVerifyError {
            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>
    {
            Ok(
                usize::deserialize(deserializer)
            .map(TwilioVerifyError::from_code)?
            .ok_or_else(|| Error::custom("Invalid error code!"))?
            )
    }
}

impl Serialize for TwilioVerifyError {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
            usize::serialize(&self.code(), serializer)
    }
}

use crate::products::TwilioProductError;
impl Into<TwilioProductError> for TwilioVerifyError {
    fn into(self) -> TwilioProductError {
        TwilioProductError::Verify(self)
    }
}

impl crate::products::SomeTwilioProductError for TwilioVerifyError {}

