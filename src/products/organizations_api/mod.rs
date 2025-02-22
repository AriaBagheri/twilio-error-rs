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
pub enum TwilioOrganizationsApiError {
    ErrorCode25008,
    ErrorCode25013,
    ErrorCode25004,
    ErrorCode25015,
    ErrorCode25203,
    ErrorCode25106,
    ErrorCode25100,
    ErrorCode25201,
    ErrorCode25001,
    ErrorCode25018,
    ErrorCode25021,
    ErrorCode25022,
    ErrorCode25002,
    ErrorCode25016,
    ErrorCode25007,
    ErrorCode25006,
    ErrorCode25101,
    ErrorCode25023,
    ErrorCode25009,
    ErrorCode25005,
    ErrorCode25017,
    ErrorCode25107,
    ErrorCode25011,
    ErrorCode25109,
    ErrorCode25003,
    ErrorCode25200,
    ErrorCode25103,
    ErrorCode25202,
    ErrorCode25019,
    ErrorCode25012,
    ErrorCode25014,
    ErrorCode25104,
    ErrorCode25102,
    ErrorCode25020,
    ErrorCode25010,
    ErrorCode25105
}


impl<'de> Deserialize<'de> for TwilioOrganizationsApiError {
            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>
    {
            Ok(
                usize::deserialize(deserializer)
            .map(TwilioOrganizationsApiError::from_code)?
            .ok_or_else(|| Error::custom("Invalid error code!"))?
            )
    }
}

impl Serialize for TwilioOrganizationsApiError {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
            usize::serialize(&self.code(), serializer)
    }
}

use crate::products::TwilioProductError;
impl Into<TwilioProductError> for TwilioOrganizationsApiError {
    fn into(self) -> TwilioProductError {
        TwilioProductError::OrganizationsApi(self)
    }
}

impl crate::products::SomeTwilioProductError for TwilioOrganizationsApiError {}

