// AUTO-GENERATED. DO NOT MODIFY. YOUR CHANGES WILL BE OVER-WRITTEN
use crate::products::TwilioProgrammableFaxError;
use standard_error::traits::StandardErrorCausesTrait;

impl StandardErrorCausesTrait for TwilioProgrammableFaxError {
    fn causes(&self) -> Option<&'static str> {
        match self {
            TwilioProgrammableFaxError::ErrorCode34004 => None,
            TwilioProgrammableFaxError::ErrorCode34003 => None,
            TwilioProgrammableFaxError::ErrorCode34108 => None,
            TwilioProgrammableFaxError::ErrorCode34106 => None,
            TwilioProgrammableFaxError::ErrorCode34002 => None,
            TwilioProgrammableFaxError::ErrorCode34005 => Some(r#"You are trying to use the Programmable Fax API which is no longer available"#)
        }
    }
}
