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
pub enum TwilioPhoneNumbersError {
    ErrorCode22225,
    ErrorCode22152,
    ErrorCode22155,
    ErrorCode22205,
    ErrorCode22212,
    ErrorCode22112,
    ErrorCode18056,
    ErrorCode22213,
    ErrorCode22210,
    ErrorCode18060,
    ErrorCode18605,
    ErrorCode18609,
    ErrorCode22171,
    ErrorCode22122,
    ErrorCode21628,
    ErrorCode22118,
    ErrorCode22217,
    ErrorCode18008,
    ErrorCode22133,
    ErrorCode22130,
    ErrorCode22114,
    ErrorCode22111,
    ErrorCode22151,
    ErrorCode21650,
    ErrorCode18036,
    ErrorCode18608,
    ErrorCode18053,
    ErrorCode22153,
    ErrorCode18051,
    ErrorCode18604,
    ErrorCode22401,
    ErrorCode22200,
    ErrorCode18606,
    ErrorCode22102,
    ErrorCode18607,
    ErrorCode18005,
    ErrorCode18017,
    ErrorCode21648,
    ErrorCode18052,
    ErrorCode22402,
    ErrorCode21630,
    ErrorCode21449,
    ErrorCode18055,
    ErrorCode22215,
    ErrorCode18054,
    ErrorCode18603,
    ErrorCode18058,
    ErrorCode18012,
    ErrorCode22403,
    ErrorCode22400,
    ErrorCode18061,
    ErrorCode18024,
    ErrorCode18602,
    ErrorCode22224,
    ErrorCode22207,
    ErrorCode22230,
    ErrorCode22159,
    ErrorCode22204,
    ErrorCode22113,
    ErrorCode22203,
    ErrorCode18099,
    ErrorCode18022,
    ErrorCode22214,
    ErrorCode22209,
    ErrorCode22404,
    ErrorCode22119,
    ErrorCode18038,
    ErrorCode22105,
    ErrorCode22117,
    ErrorCode22108,
    ErrorCode10002,
    ErrorCode18004,
    ErrorCode18014,
    ErrorCode18001,
    ErrorCode22222,
    ErrorCode18011,
    ErrorCode22218,
    ErrorCode22208,
    ErrorCode22120,
    ErrorCode22206,
    ErrorCode22135,
    ErrorCode22500,
    ErrorCode18007,
    ErrorCode22121,
    ErrorCode22211,
    ErrorCode22202,
    ErrorCode22115,
    ErrorCode18010,
    ErrorCode22170,
    ErrorCode22158,
    ErrorCode22157,
    ErrorCode18020,
    ErrorCode22136,
    ErrorCode22104,
    ErrorCode22106,
    ErrorCode18002,
    ErrorCode18018,
    ErrorCode22221,
    ErrorCode21631,
    ErrorCode21615,
    ErrorCode18003,
    ErrorCode18062,
    ErrorCode22132,
    ErrorCode22101,
    ErrorCode22201,
    ErrorCode22100,
    ErrorCode22150,
    ErrorCode22173,
    ErrorCode22103,
    ErrorCode18009,
    ErrorCode22131,
    ErrorCode22109,
    ErrorCode22156,
    ErrorCode21645,
    ErrorCode21651,
    ErrorCode18015,
    ErrorCode18023,
    ErrorCode22199,
    ErrorCode21649,
    ErrorCode22116,
    ErrorCode22350,
    ErrorCode22110,
    ErrorCode18013,
    ErrorCode18037,
    ErrorCode22172,
    ErrorCode21646,
    ErrorCode18021,
    ErrorCode22216,
    ErrorCode22123,
    ErrorCode22154,
    ErrorCode22228,
    ErrorCode21629,
    ErrorCode18016,
    ErrorCode18050,
    ErrorCode22107,
    ErrorCode18601,
    ErrorCode22300,
    ErrorCode22229,
    ErrorCode18610,
    ErrorCode18059,
    ErrorCode180017,
    ErrorCode18006,
    ErrorCode21647,
    ErrorCode22219,
    ErrorCode21644,
    ErrorCode18019,
    ErrorCode18039,
    ErrorCode18057,
    ErrorCode22226
}


impl<'de> Deserialize<'de> for TwilioPhoneNumbersError {
            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>
    {
            Ok(
                usize::deserialize(deserializer)
            .map(TwilioPhoneNumbersError::from_code)?
            .ok_or_else(|| Error::custom("Invalid error code!"))?
            )
    }
}

impl Serialize for TwilioPhoneNumbersError {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
            usize::serialize(&self.code(), serializer)
    }
}

use crate::products::TwilioProductError;
impl Into<TwilioProductError> for TwilioPhoneNumbersError {
    fn into(self) -> TwilioProductError {
        TwilioProductError::PhoneNumbers(self)
    }
}

impl crate::products::SomeTwilioProductError for TwilioPhoneNumbersError {}

