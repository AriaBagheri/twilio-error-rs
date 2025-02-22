// AUTO-GENERATED. DO NOT MODIFY. YOUR CHANGES WILL BE OVER-WRITTEN
use crate::products::TwilioPlatformError;
use standard_error::traits::StandardErrorCausesTrait;

impl StandardErrorCausesTrait for TwilioPlatformError {
    fn causes(&self) -> Option<&'static str> {
        match self {
            TwilioPlatformError::ErrorCode20101 => None,
            TwilioPlatformError::ErrorCode20409 => Some(r#"The resource is not in a state that allows this modification."#),
            TwilioPlatformError::ErrorCode20162 => Some(r#"The same API resource is being updated by multiple simultaneous API requests from  different threads in a conflicting fashion."#),
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
            TwilioPlatformError::ErrorCode97001 => Some(r#"* The authorization service is unavailable.
* The authorization service is unreachable.
* The client credentials you configured have been rotated or removed.
* The authorization server has started issuing a token which is not of type `Bearer`."#),
            TwilioPlatformError::ErrorCode20151 => None,
            TwilioPlatformError::ErrorCode20103 => None,
            TwilioPlatformError::ErrorCode21481 => None,
            TwilioPlatformError::ErrorCode20403 => Some(r#"The account has been suspended or closed."#),
            TwilioPlatformError::ErrorCode20160 => None,
            TwilioPlatformError::ErrorCode20152 => None,
            TwilioPlatformError::ErrorCode20106 => None
        }
    }
}
