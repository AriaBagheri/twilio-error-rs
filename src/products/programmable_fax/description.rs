// AUTO-GENERATED. DO NOT MODIFY. YOUR CHANGES WILL BE OVER-WRITTEN
use crate::products::TwilioProgrammableFaxError;
use standard_error::traits::StandardErrorDescriptionTrait;

impl StandardErrorDescriptionTrait for TwilioProgrammableFaxError {
    fn description(&self) -> Option<&'static str> {
        match self {
            TwilioProgrammableFaxError::ErrorCode34004 => None,
            TwilioProgrammableFaxError::ErrorCode34003 => None,
            TwilioProgrammableFaxError::ErrorCode34108 => None,
            TwilioProgrammableFaxError::ErrorCode34106 => None,
            TwilioProgrammableFaxError::ErrorCode34002 => None,
            TwilioProgrammableFaxError::ErrorCode34005 => Some(r#"The Programmable Fax API by Twilio is no longer supported"#)
        }
    }
}
