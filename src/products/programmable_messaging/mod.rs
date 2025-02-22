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
pub enum TwilioProgrammableMessagingError {
    ErrorCode20504,
    ErrorCode30884,
    ErrorCode35133,
    ErrorCode30758,
    ErrorCode21660,
    ErrorCode30897,
    ErrorCode63036,
    ErrorCode30447,
    ErrorCode30444,
    ErrorCode30474,
    ErrorCode36005,
    ErrorCode30440,
    ErrorCode30901,
    ErrorCode30749,
    ErrorCode30888,
    ErrorCode21736,
    ErrorCode11321,
    ErrorCode30480,
    ErrorCode30750,
    ErrorCode30994,
    ErrorCode63041,
    ErrorCode30890,
    ErrorCode11322,
    ErrorCode63047,
    ErrorCode30473,
    ErrorCode30647,
    ErrorCode216602,
    ErrorCode30446,
    ErrorCode21737,
    ErrorCode30471,
    ErrorCode21731,
    ErrorCode30460,
    ErrorCode36001,
    ErrorCode30754,
    ErrorCode20410,
    ErrorCode30646,
    ErrorCode36004,
    ErrorCode30610,
    ErrorCode21659,
    ErrorCode30734,
    ErrorCode30475,
    ErrorCode30993,
    ErrorCode30903,
    ErrorCode30701,
    ErrorCode21267,
    ErrorCode21732,
    ErrorCode30445,
    ErrorCode21266,
    ErrorCode30757,
    ErrorCode30456,
    ErrorCode63051,
    ErrorCode11100,
    ErrorCode63046,
    ErrorCode30908,
    ErrorCode21703,
    ErrorCode30463,
    ErrorCode30895,
    ErrorCode21635,
    ErrorCode30729,
    ErrorCode30896,
    ErrorCode30906,
    ErrorCode30907,
    ErrorCode30902,
    ErrorCode30441,
    ErrorCode30465,
    ErrorCode35127,
    ErrorCode63045,
    ErrorCode30797,
    ErrorCode30885,
    ErrorCode30894,
    ErrorCode36009,
    ErrorCode30620,
    ErrorCode30702,
    ErrorCode30991,
    ErrorCode36008,
    ErrorCode30883,
    ErrorCode30442,
    ErrorCode30892,
    ErrorCode30751,
    ErrorCode30891,
    ErrorCode30455,
    ErrorCode63043,
    ErrorCode36003,
    ErrorCode35134,
    ErrorCode36007,
    ErrorCode30796,
    ErrorCode30470,
    ErrorCode30476,
    ErrorCode30990,
    ErrorCode30436,
    ErrorCode30452,
    ErrorCode30793,
    ErrorCode30468,
    ErrorCode63052,
    ErrorCode30748,
    ErrorCode30992,
    ErrorCode30451,
    ErrorCode30457,
    ErrorCode30467,
    ErrorCode30798,
    ErrorCode30461,
    ErrorCode30893,
    ErrorCode30469,
    ErrorCode30753,
    ErrorCode30448,
    ErrorCode30449,
    ErrorCode30790,
    ErrorCode30459,
    ErrorCode30795,
    ErrorCode30437,
    ErrorCode30887,
    ErrorCode30880,
    ErrorCode30899,
    ErrorCode30905,
    ErrorCode36006,
    ErrorCode30479,
    ErrorCode30886,
    ErrorCode30882,
    ErrorCode21733,
    ErrorCode30464,
    ErrorCode30752,
    ErrorCode35132,
    ErrorCode21268,
    ErrorCode30472,
    ErrorCode30135,
    ErrorCode30898,
    ErrorCode21265,
    ErrorCode30904,
    ErrorCode30791,
    ErrorCode30794,
    ErrorCode30889,
    ErrorCode30478,
    ErrorCode63040,
    ErrorCode30909,
    ErrorCode30443,
    ErrorCode30462,
    ErrorCode63024,
    ErrorCode30136,
    ErrorCode30703,
    ErrorCode30799,
    ErrorCode11206,
    ErrorCode63042,
    ErrorCode30712,
    ErrorCode30466,
    ErrorCode21661,
    ErrorCode30756,
    ErrorCode30747,
    ErrorCode30881,
    ErrorCode30615,
    ErrorCode30458,
    ErrorCode30755,
    ErrorCode36002,
    ErrorCode63044,
    ErrorCode30792,
    ErrorCode30900,
    ErrorCode63049,
    ErrorCode30477,
    ErrorCode30630
}


impl<'de> Deserialize<'de> for TwilioProgrammableMessagingError {
            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>
    {
            Ok(
                usize::deserialize(deserializer)
            .map(TwilioProgrammableMessagingError::from_code)?
            .ok_or_else(|| Error::custom("Invalid error code!"))?
            )
    }
}

impl Serialize for TwilioProgrammableMessagingError {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
            usize::serialize(&self.code(), serializer)
    }
}

use crate::TwilioProductError;
impl Into<TwilioProductError> for TwilioProgrammableMessagingError {
    fn into(self) -> TwilioProductError {
        TwilioProductError::ProgrammableMessaging(self)
    }
}

impl crate::products::SomeTwilioProductError for TwilioProgrammableMessagingError {}

