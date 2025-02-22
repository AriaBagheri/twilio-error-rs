// AUTO-GENERATED. DO NOT MODIFY. YOUR CHANGES WILL BE OVER-WRITTEN
use crate::products::TwilioFunctionsError;
use standard_error::traits::StandardErrorDescriptionTrait;

impl StandardErrorDescriptionTrait for TwilioFunctionsError {
    fn description(&self) -> Option<&'static str> {
        match self {
            TwilioFunctionsError::ErrorCode82001 => Some(r#"Your Function invocation returned a 4xx StatusCode."#),
            TwilioFunctionsError::ErrorCode82009 => Some(r#"Twilio was unable to process the correct response to your Function invocation because your version of Functions is incompatible with multi-valued HTTP response headers."#),
            TwilioFunctionsError::ErrorCode82006 => Some(r#"Context of the function has exceeded maximum limit. Reduce the context size and try again."#),
            TwilioFunctionsError::ErrorCode82005 => Some(r#"Your Function execution resulted in logging an error message"#),
            TwilioFunctionsError::ErrorCode82002 => Some(r#"Your Function invocation resulted in StatusCode 5xx."#),
            TwilioFunctionsError::ErrorCode82004 => Some(r#"Your Function execution resulted in logging a warning message"#),
            TwilioFunctionsError::ErrorCode82007 => Some(r#"The runtime used in the Serverless Build request is not supported on the platform. A runtime defines the environment your Functions will be executed in (e.g. which Node.js version to use for your Functions)."#),
            TwilioFunctionsError::ErrorCode82008 => Some(r#"This error is thrown when request or response headers are greater than the enforced limits."#),
            TwilioFunctionsError::ErrorCode82003 => Some(r#"Your latest Functions and Private Assets failed to be deployed."#)
        }
    }
}
