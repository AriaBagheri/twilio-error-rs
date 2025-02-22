// AUTO-GENERATED. DO NOT MODIFY. YOUR CHANGES WILL BE OVER-WRITTEN
use crate::products::TwilioPlatformError;
use standard_error::traits::StandardErrorSolutionsTrait;

impl StandardErrorSolutionsTrait for TwilioPlatformError {
    fn solutions(&self) -> Option<&'static str> {
        match self {
            TwilioPlatformError::ErrorCode20101 => None,
            TwilioPlatformError::ErrorCode20409 => Some(r#"Wait until the resource is in a modifiable state."#),
            TwilioPlatformError::ErrorCode20162 => Some(r#"* Implement an operation retrier and repeat the failed API requests after an interval of time using, exponential backoff algorithm.
* Serialize your API requests that modify the same resources, and wait until the original request completes and returns an API response, before sending any successive modifications."#),
            TwilioPlatformError::ErrorCode20157 => None,
            TwilioPlatformError::ErrorCode21102 => None,
            TwilioPlatformError::ErrorCode20159 => None,
            TwilioPlatformError::ErrorCode20105 => None,
            TwilioPlatformError::ErrorCode20404 => Some(r#"-"#),
            TwilioPlatformError::ErrorCode20154 => None,
            TwilioPlatformError::ErrorCode20107 => None,
            TwilioPlatformError::ErrorCode20155 => None,
            TwilioPlatformError::ErrorCode20153 => None,
            TwilioPlatformError::ErrorCode20156 => None,
            TwilioPlatformError::ErrorCode20104 => None,
            TwilioPlatformError::ErrorCode20102 => None,
            TwilioPlatformError::ErrorCode97001 => Some(r#"* Verify your authorization server is running and healthy.
* Verify your authorization server is not protected by a firewall which blocks the requests.
* Update the webhook settings with valid client credentials.
* Update the webhook settings to disable OAuth.
* Verify the authorization server issues tokens of type `Bearer`."#),
            TwilioPlatformError::ErrorCode20151 => None,
            TwilioPlatformError::ErrorCode20103 => None,
            TwilioPlatformError::ErrorCode21481 => None,
            TwilioPlatformError::ErrorCode20403 => Some(r#"Reach out to Twilio Support."#),
            TwilioPlatformError::ErrorCode20160 => None,
            TwilioPlatformError::ErrorCode20152 => None,
            TwilioPlatformError::ErrorCode20106 => None
        }
    }
}
