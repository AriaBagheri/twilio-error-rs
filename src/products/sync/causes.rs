// AUTO-GENERATED. DO NOT MODIFY. YOUR CHANGES WILL BE OVER-WRITTEN
use crate::products::TwilioSyncError;
use standard_error::traits::StandardErrorCausesTrait;

impl StandardErrorCausesTrait for TwilioSyncError {
    fn causes(&self) -> Option<&'static str> {
        match self {
            TwilioSyncError::ErrorCode54011 => None,
            TwilioSyncError::ErrorCode54006 => None,
            TwilioSyncError::ErrorCode54201 => None,
            TwilioSyncError::ErrorCode54155 => None,
            TwilioSyncError::ErrorCode54451 => None,
            TwilioSyncError::ErrorCode54351 => None,
            TwilioSyncError::ErrorCode54156 => None,
            TwilioSyncError::ErrorCode54450 => None,
            TwilioSyncError::ErrorCode54251 => None,
            TwilioSyncError::ErrorCode54100 => None,
            TwilioSyncError::ErrorCode54003 => None,
            TwilioSyncError::ErrorCode54509 => None,
            TwilioSyncError::ErrorCode54103 => None,
            TwilioSyncError::ErrorCode54101 => None,
            TwilioSyncError::ErrorCode54507 => None,
            TwilioSyncError::ErrorCode54053 => None,
            TwilioSyncError::ErrorCode54452 => None,
            TwilioSyncError::ErrorCode54301 => None,
            TwilioSyncError::ErrorCode54009 => None,
            TwilioSyncError::ErrorCode54051 => Some(r#"* You did not pass a correctly-formatted URL.
* You are crafting HTTP requests yourself and passed a URL, but did not percent-encoded it.
* You passed a webhook URL that is longer than 512 characters."#),
            TwilioSyncError::ErrorCode54354 => None,
            TwilioSyncError::ErrorCode54502 => None,
            TwilioSyncError::ErrorCode54453 => None,
            TwilioSyncError::ErrorCode54300 => None,
            TwilioSyncError::ErrorCode54206 => None,
            TwilioSyncError::ErrorCode54200 => None,
            TwilioSyncError::ErrorCode54302 => None,
            TwilioSyncError::ErrorCode54208 => None,
            TwilioSyncError::ErrorCode54510 => None,
            TwilioSyncError::ErrorCode54050 => None,
            TwilioSyncError::ErrorCode54151 => None,
            TwilioSyncError::ErrorCode54150 => None,
            TwilioSyncError::ErrorCode54007 => None,
            TwilioSyncError::ErrorCode54008 => None,
            TwilioSyncError::ErrorCode54209 => None,
            TwilioSyncError::ErrorCode54458 => None,
            TwilioSyncError::ErrorCode54205 => None,
            TwilioSyncError::ErrorCode54056 => None,
            TwilioSyncError::ErrorCode54010 => None,
            TwilioSyncError::ErrorCode54454 => Some(r#"The value supplied by the PageSize query string parameter is not an integer in the required range."#),
            TwilioSyncError::ErrorCode54419 => None,
            TwilioSyncError::ErrorCode54250 => None
        }
    }
}
