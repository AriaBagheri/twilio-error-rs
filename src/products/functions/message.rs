// AUTO-GENERATED. DO NOT MODIFY. YOUR CHANGES WILL BE OVER-WRITTEN
use crate::products::TwilioFunctionsError;
use standard_error::traits::StandardErrorMessageTrait;
use std::borrow::Cow;

impl StandardErrorMessageTrait for TwilioFunctionsError {
    fn message(&self) -> Cow<'static, str> {
        match self {
            TwilioFunctionsError::ErrorCode82001 => r#"Function invocation resulted in StatusCode 4xx"#.into(),
            TwilioFunctionsError::ErrorCode82009 => r#"Multi-valued headers not supported"#.into(),
            TwilioFunctionsError::ErrorCode82006 => r#"Environment Context too large"#.into(),
            TwilioFunctionsError::ErrorCode82005 => r#"Function execution resulted in an error log"#.into(),
            TwilioFunctionsError::ErrorCode82002 => r#"Error on Twilio Function response"#.into(),
            TwilioFunctionsError::ErrorCode82004 => r#"Function execution resulted in a warning log"#.into(),
            TwilioFunctionsError::ErrorCode82007 => r#"Unsupported Runtime"#.into(),
            TwilioFunctionsError::ErrorCode82008 => r#"Headers or cookies too large"#.into(),
            TwilioFunctionsError::ErrorCode82003 => r#"Deployment Installation Failure"#.into()
        }
    }
}
