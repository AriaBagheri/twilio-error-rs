// AUTO-GENERATED. DO NOT MODIFY. YOUR CHANGES WILL BE OVER-WRITTEN
use crate::products::TwilioProgrammableFaxError;
use standard_error::traits::StandardErrorCodeTrait;

impl StandardErrorCodeTrait for TwilioProgrammableFaxError {
    fn code(&self) -> usize {
        match self {
            TwilioProgrammableFaxError::ErrorCode34004 => 34004,
            TwilioProgrammableFaxError::ErrorCode34003 => 34003,
            TwilioProgrammableFaxError::ErrorCode34108 => 34108,
            TwilioProgrammableFaxError::ErrorCode34106 => 34106,
            TwilioProgrammableFaxError::ErrorCode34002 => 34002,
            TwilioProgrammableFaxError::ErrorCode34005 => 34005
        }
    }
}
