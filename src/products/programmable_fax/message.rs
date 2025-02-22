// AUTO-GENERATED. DO NOT MODIFY. YOUR CHANGES WILL BE OVER-WRITTEN
use crate::products::TwilioProgrammableFaxError;
use standard_error::traits::StandardErrorMessageTrait;
use std::borrow::Cow;

impl StandardErrorMessageTrait for TwilioProgrammableFaxError {
    fn message(&self) -> Cow<'static, str> {
        match self {
            TwilioProgrammableFaxError::ErrorCode34004 => r#"Error during fax transmission"#.into(),
            TwilioProgrammableFaxError::ErrorCode34003 => r#"Callee did not answer"#.into(),
            TwilioProgrammableFaxError::ErrorCode34108 => r#"Other End Incompatible"#.into(),
            TwilioProgrammableFaxError::ErrorCode34106 => r#"No Fax TwiML action specified"#.into(),
            TwilioProgrammableFaxError::ErrorCode34002 => r#"Callee Busy"#.into(),
            TwilioProgrammableFaxError::ErrorCode34005 => r#"Programmable Fax is no longer available"#.into()
        }
    }
}
