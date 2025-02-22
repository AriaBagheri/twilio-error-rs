// AUTO-GENERATED. DO NOT MODIFY. YOUR CHANGES WILL BE OVER-WRITTEN
use crate::products::TwilioProgrammableFaxError;
use standard_error::traits::StandardErrorFromCodeTrait;

impl StandardErrorFromCodeTrait for TwilioProgrammableFaxError {
    fn from_code(code: usize) -> Option<Self> {
        match code {
            34004 => Some(TwilioProgrammableFaxError::ErrorCode34004),
            34003 => Some(TwilioProgrammableFaxError::ErrorCode34003),
            34108 => Some(TwilioProgrammableFaxError::ErrorCode34108),
            34106 => Some(TwilioProgrammableFaxError::ErrorCode34106),
            34002 => Some(TwilioProgrammableFaxError::ErrorCode34002),
            34005 => Some(TwilioProgrammableFaxError::ErrorCode34005),
            _ => None
        }
    }
}
