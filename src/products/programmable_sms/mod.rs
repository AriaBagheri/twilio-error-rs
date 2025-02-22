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
pub enum TwilioProgrammableSmsError {
    ErrorCode30111,
    ErrorCode57016,
    ErrorCode30118,
    ErrorCode21725,
    ErrorCode30006,
    ErrorCode30117,
    ErrorCode30107,
    ErrorCode57013,
    ErrorCode57020,
    ErrorCode30019,
    ErrorCode30043,
    ErrorCode90009,
    ErrorCode92008,
    ErrorCode90007,
    ErrorCode30124,
    ErrorCode63016,
    ErrorCode21654,
    ErrorCode30409,
    ErrorCode30119,
    ErrorCode21606,
    ErrorCode63031,
    ErrorCode23004,
    ErrorCode11751,
    ErrorCode30036,
    ErrorCode30121,
    ErrorCode21611,
    ErrorCode30027,
    ErrorCode63008,
    ErrorCode23002,
    ErrorCode30011,
    ErrorCode30130,
    ErrorCode21627,
    ErrorCode23005,
    ErrorCode90014,
    ErrorCode92005,
    ErrorCode21408,
    ErrorCode35125,
    ErrorCode21712,
    ErrorCode57006,
    ErrorCode57007,
    ErrorCode30108,
    ErrorCode57009,
    ErrorCode30022,
    ErrorCode30100,
    ErrorCode30002,
    ErrorCode92007,
    ErrorCode21723,
    ErrorCode30009,
    ErrorCode63028,
    ErrorCode30116,
    ErrorCode63001,
    ErrorCode90006,
    ErrorCode30122,
    ErrorCode57018,
    ErrorCode57003,
    ErrorCode30031,
    ErrorCode57004,
    ErrorCode21614,
    ErrorCode63023,
    ErrorCode30041,
    ErrorCode21709,
    ErrorCode21658,
    ErrorCode21722,
    ErrorCode63038,
    ErrorCode57002,
    ErrorCode63020,
    ErrorCode63035,
    ErrorCode63011,
    ErrorCode92004,
    ErrorCode30133,
    ErrorCode21720,
    ErrorCode57011,
    ErrorCode21711,
    ErrorCode30400,
    ErrorCode21605,
    ErrorCode21612,
    ErrorCode57017,
    ErrorCode21619,
    ErrorCode30007,
    ErrorCode63006,
    ErrorCode63029,
    ErrorCode30040,
    ErrorCode57021,
    ErrorCode35126,
    ErrorCode30038,
    ErrorCode35111,
    ErrorCode30103,
    ErrorCode30105,
    ErrorCode21710,
    ErrorCode30404,
    ErrorCode57005,
    ErrorCode30129,
    ErrorCode21730,
    ErrorCode30127,
    ErrorCode21902,
    ErrorCode30128,
    ErrorCode30037,
    ErrorCode30032,
    ErrorCode21910,
    ErrorCode92009,
    ErrorCode57019,
    ErrorCode30125,
    ErrorCode63025,
    ErrorCode30021,
    ErrorCode30485,
    ErrorCode30123,
    ErrorCode63022,
    ErrorCode30024,
    ErrorCode30029,
    ErrorCode63009,
    ErrorCode30114,
    ErrorCode30003,
    ErrorCode63003,
    ErrorCode30131,
    ErrorCode35117,
    ErrorCode21655,
    ErrorCode30034,
    ErrorCode63007,
    ErrorCode30104,
    ErrorCode30020,
    ErrorCode30450,
    ErrorCode30010,
    ErrorCode21610,
    ErrorCode90001,
    ErrorCode21717,
    ErrorCode57001,
    ErrorCode30454,
    ErrorCode92002,
    ErrorCode90031,
    ErrorCode63019,
    ErrorCode30026,
    ErrorCode30115,
    ErrorCode35118,
    ErrorCode30039,
    ErrorCode21652,
    ErrorCode30008,
    ErrorCode30042,
    ErrorCode63021,
    ErrorCode21624,
    ErrorCode63034,
    ErrorCode30126,
    ErrorCode57014,
    ErrorCode21657,
    ErrorCode63018,
    ErrorCode92001,
    ErrorCode30001,
    ErrorCode30120,
    ErrorCode30113,
    ErrorCode30023,
    ErrorCode63012,
    ErrorCode57022,
    ErrorCode57008,
    ErrorCode21729,
    ErrorCode21719,
    ErrorCode21724,
    ErrorCode63027,
    ErrorCode30132,
    ErrorCode30410,
    ErrorCode63005,
    ErrorCode63013,
    ErrorCode30017,
    ErrorCode30028,
    ErrorCode21713,
    ErrorCode21708,
    ErrorCode30030,
    ErrorCode30500,
    ErrorCode21603,
    ErrorCode57010,
    ErrorCode63030,
    ErrorCode14107,
    ErrorCode63002,
    ErrorCode30109,
    ErrorCode21656,
    ErrorCode57012,
    ErrorCode30102,
    ErrorCode63015,
    ErrorCode92006,
    ErrorCode63032,
    ErrorCode30005,
    ErrorCode30110,
    ErrorCode14109,
    ErrorCode35114,
    ErrorCode21704,
    ErrorCode21721,
    ErrorCode30033,
    ErrorCode23001,
    ErrorCode63033,
    ErrorCode30134,
    ErrorCode23003,
    ErrorCode63010,
    ErrorCode21728,
    ErrorCode63017,
    ErrorCode30035,
    ErrorCode92003,
    ErrorCode21727,
    ErrorCode63026,
    ErrorCode23006,
    ErrorCode21714,
    ErrorCode30004,
    ErrorCode63037,
    ErrorCode21715,
    ErrorCode63039,
    ErrorCode30453,
    ErrorCode30106,
    ErrorCode21726,
    ErrorCode35115,
    ErrorCode30018,
    ErrorCode30101,
    ErrorCode30025,
    ErrorCode63014
}


impl<'de> Deserialize<'de> for TwilioProgrammableSmsError {
            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>
    {
            Ok(
                usize::deserialize(deserializer)
            .map(TwilioProgrammableSmsError::from_code)?
            .ok_or_else(|| Error::custom("Invalid error code!"))?
            )
    }
}

impl Serialize for TwilioProgrammableSmsError {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
            usize::serialize(&self.code(), serializer)
    }
}

use crate::products::TwilioProductError;
impl Into<TwilioProductError> for TwilioProgrammableSmsError {
    fn into(self) -> TwilioProductError {
        TwilioProductError::ProgrammableSms(self)
    }
}

impl crate::products::SomeTwilioProductError for TwilioProgrammableSmsError {}

