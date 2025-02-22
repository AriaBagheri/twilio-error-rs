// AUTO-GENERATED. DO NOT MODIFY. YOUR CHANGES WILL BE OVER-WRITTEN
use crate::products::TwilioFunctionsError;
use standard_error::traits::StandardErrorCausesTrait;

impl StandardErrorCausesTrait for TwilioFunctionsError {
    fn causes(&self) -> Option<&'static str> {
        match self {
            TwilioFunctionsError::ErrorCode82001 => Some(r#"* Function route does not exist.
* Function is set to Private and invocation request is not signed."#),
            TwilioFunctionsError::ErrorCode82009 => Some(r#"A Function invocation has resulted in a Twilio Response with multi-valued headers (i.e. the header value is an array). Multi-valued headers are not supported in Classic Functions, or in @twilio/runtime-handler dependency versions below 1.2.0."#),
            TwilioFunctionsError::ErrorCode82006 => Some(r#"Context size consists of account metadata, domain name, path and Environment variables stored as a JSON string. If this string exceeds 3KB, invocations may fail."#),
            TwilioFunctionsError::ErrorCode82005 => Some(r#"Function execution code path lead to console.error(...)"#),
            TwilioFunctionsError::ErrorCode82002 => Some(r#"* Your Function timed out before responding.
* Your Function returned an error response."#),
            TwilioFunctionsError::ErrorCode82004 => Some(r#"Function execution code path lead to console.warn(...)"#),
            TwilioFunctionsError::ErrorCode82007 => Some(r#"* A Serverless Build request was made without providing the runtime and the runtime used for the last successful build in the service (e.g. a Node.js version that is no longer supported on the platform).
* A Serverless Build request was made with a specified runtime (e.g. node10) that is currently not supported on the platform."#),
            TwilioFunctionsError::ErrorCode82008 => Some(r#"1. The user has passed request/response headers larger than 4kb. 
2. Request/response has more than 75 headers+cookies."#),
            TwilioFunctionsError::ErrorCode82003 => Some(r#"* NPM registry is degraded.
* Your Service instance is trying to install a package that requires a gcc compiler. 
* Your Service instance resulted in out of space error due to large packages."#)
        }
    }
}
