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
pub enum TwilioProgrammableVoiceError {
    ErrorCode11750,
    ErrorCode16026,
    ErrorCode13802,
    ErrorCode16002,
    ErrorCode16003,
    ErrorCode64106,
    ErrorCode64017,
    ErrorCode32702,
    ErrorCode32652,
    ErrorCode64009,
    ErrorCode64016,
    ErrorCode37000,
    ErrorCode13342,
    ErrorCode21302,
    ErrorCode31429,
    ErrorCode13801,
    ErrorCode31904,
    ErrorCode32653,
    ErrorCode64002,
    ErrorCode31922,
    ErrorCode16020,
    ErrorCode31901,
    ErrorCode31940,
    ErrorCode17000,
    ErrorCode13328,
    ErrorCode13332,
    ErrorCode64011,
    ErrorCode64005,
    ErrorCode13335,
    ErrorCode31952,
    ErrorCode16099,
    ErrorCode32701,
    ErrorCode13330,
    ErrorCode31409,
    ErrorCode16102,
    ErrorCode31009,
    ErrorCode16101,
    ErrorCode15004,
    ErrorCode13804,
    ErrorCode16106,
    ErrorCode31103,
    ErrorCode32019,
    ErrorCode21263,
    ErrorCode31502,
    ErrorCode31206,
    ErrorCode16011,
    ErrorCode64023,
    ErrorCode31408,
    ErrorCode64018,
    ErrorCode53404,
    ErrorCode16023,
    ErrorCode31209,
    ErrorCode13805,
    ErrorCode16022,
    ErrorCode64010,
    ErrorCode16025,
    ErrorCode13331,
    ErrorCode31402,
    ErrorCode64019,
    ErrorCode17009,
    ErrorCode31211,
    ErrorCode64006,
    ErrorCode32021,
    ErrorCode31604,
    ErrorCode32500,
    ErrorCode32101,
    ErrorCode64014,
    ErrorCode17001,
    ErrorCode32009,
    ErrorCode64004,
    ErrorCode64022,
    ErrorCode31002,
    ErrorCode13247,
    ErrorCode13257,
    ErrorCode31202,
    ErrorCode31212,
    ErrorCode32503,
    ErrorCode13329,
    ErrorCode64105,
    ErrorCode32601,
    ErrorCode64108,
    ErrorCode32606,
    ErrorCode31920,
    ErrorCode17007,
    ErrorCode13333,
    ErrorCode31530,
    ErrorCode13227,
    ErrorCode32007,
    ErrorCode31104,
    ErrorCode31208,
    ErrorCode31910,
    ErrorCode31953,
    ErrorCode16024,
    ErrorCode32650,
    ErrorCode17008,
    ErrorCode32106,
    ErrorCode32014,
    ErrorCode32651,
    ErrorCode13326,
    ErrorCode31426,
    ErrorCode13750,
    ErrorCode13240,
    ErrorCode16110,
    ErrorCode13220,
    ErrorCode21301,
    ErrorCode31504,
    ErrorCode31930,
    ErrorCode37001,
    ErrorCode32603,
    ErrorCode17002,
    ErrorCode32008,
    ErrorCode32711,
    ErrorCode16010,
    ErrorCode16108,
    ErrorCode13521,
    ErrorCode16113,
    ErrorCode15009,
    ErrorCode10005,
    ErrorCode64104,
    ErrorCode32220,
    ErrorCode13340,
    ErrorCode17400,
    ErrorCode32215,
    ErrorCode13430,
    ErrorCode22005,
    ErrorCode31486,
    ErrorCode14226,
    ErrorCode32654,
    ErrorCode31008,
    ErrorCode31000,
    ErrorCode31923,
    ErrorCode13327,
    ErrorCode16105,
    ErrorCode16028,
    ErrorCode13223,
    ErrorCode64107,
    ErrorCode31207,
    ErrorCode13218,
    ErrorCode64008,
    ErrorCode13238,
    ErrorCode64007,
    ErrorCode13239,
    ErrorCode31600,
    ErrorCode14219,
    ErrorCode31480,
    ErrorCode13810,
    ErrorCode31487,
    ErrorCode32015,
    ErrorCode32212,
    ErrorCode31903,
    ErrorCode32505,
    ErrorCode31500,
    ErrorCode64103,
    ErrorCode13511,
    ErrorCode32105,
    ErrorCode31400,
    ErrorCode31404,
    ErrorCode64015,
    ErrorCode31902,
    ErrorCode32602,
    ErrorCode13321,
    ErrorCode31102,
    ErrorCode31941,
    ErrorCode13338,
    ErrorCode64001,
    ErrorCode32401,
    ErrorCode31205,
    ErrorCode31003,
    ErrorCode21234,
    ErrorCode31105,
    ErrorCode32504,
    ErrorCode21262,
    ErrorCode32700,
    ErrorCode32002,
    ErrorCode64110,
    ErrorCode21300,
    ErrorCode31203,
    ErrorCode31950,
    ErrorCode21215,
    ErrorCode13621,
    ErrorCode32506,
    ErrorCode31603,
    ErrorCode21216,
    ErrorCode31302,
    ErrorCode64021,
    ErrorCode13803,
    ErrorCode32710,
    ErrorCode64013,
    ErrorCode13619,
    ErrorCode31005,
    ErrorCode32604,
    ErrorCode32013,
    ErrorCode31951,
    ErrorCode32605,
    ErrorCode16104,
    ErrorCode31204,
    ErrorCode32022,
    ErrorCode64012,
    ErrorCode31301,
    ErrorCode32210,
    ErrorCode31960,
    ErrorCode53401,
    ErrorCode64024,
    ErrorCode31106,
    ErrorCode14300,
    ErrorCode64020,
    ErrorCode64101,
    ErrorCode32703,
    ErrorCode64102,
    ErrorCode32214,
    ErrorCode32221,
    ErrorCode16109,
    ErrorCode31900,
    ErrorCode32400,
    ErrorCode16107,
    ErrorCode31403,
    ErrorCode32223,
    ErrorCode13513,
    ErrorCode32600,
    ErrorCode31931,
    ErrorCode31924,
    ErrorCode16021,
    ErrorCode31100,
    ErrorCode31201,
    ErrorCode16000,
    ErrorCode13219,
    ErrorCode14207,
    ErrorCode31101,
    ErrorCode13512,
    ErrorCode31007,
    ErrorCode31503,
    ErrorCode13620,
    ErrorCode31001,
    ErrorCode13337,
    ErrorCode13258,
    ErrorCode32501,
    ErrorCode13256,
    ErrorCode13214,
    ErrorCode16111,
    ErrorCode32655,
    ErrorCode16001,
    ErrorCode13622,
    ErrorCode13334,
    ErrorCode31401,
    ErrorCode13225,
    ErrorCode22001,
    ErrorCode31942,
    ErrorCode31481,
    ErrorCode32502,
    ErrorCode31006,
    ErrorCode13699,
    ErrorCode31921,
    ErrorCode32018,
    ErrorCode64003,
    ErrorCode64109,
    ErrorCode16027,
    ErrorCode31484,
    ErrorCode31210,
    ErrorCode16112,
    ErrorCode13216,
    ErrorCode14218
}


impl<'de> Deserialize<'de> for TwilioProgrammableVoiceError {
            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>
    {
            Ok(
                usize::deserialize(deserializer)
            .map(TwilioProgrammableVoiceError::from_code)?
            .ok_or_else(|| Error::custom("Invalid error code!"))?
            )
    }
}

impl Serialize for TwilioProgrammableVoiceError {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
            usize::serialize(&self.code(), serializer)
    }
}

use crate::TwilioProductError;
impl Into<TwilioProductError> for TwilioProgrammableVoiceError {
    fn into(self) -> TwilioProductError {
        TwilioProductError::ProgrammableVoice(self)
    }
}

impl crate::products::SomeTwilioProductError for TwilioProgrammableVoiceError {}

