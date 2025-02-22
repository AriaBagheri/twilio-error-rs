// AUTO-GENERATED. DO NOT MODIFY. YOUR CHANGES WILL BE OVER-WRITTEN
use crate::products::TwilioProgrammableFaxError;
use standard_error::traits::StandardErrorSolutionsTrait;

impl StandardErrorSolutionsTrait for TwilioProgrammableFaxError {
    fn solutions(&self) -> Option<&'static str> {
        match self {
            TwilioProgrammableFaxError::ErrorCode34004 => None,
            TwilioProgrammableFaxError::ErrorCode34003 => None,
            TwilioProgrammableFaxError::ErrorCode34108 => None,
            TwilioProgrammableFaxError::ErrorCode34106 => None,
            TwilioProgrammableFaxError::ErrorCode34002 => None,
            TwilioProgrammableFaxError::ErrorCode34005 => Some(r#"Please consult the migration guide on: https://support.twilio.com/hc/en-us/articles/223136667-Fax-Support-on-Twilio"#)
        }
    }
}
