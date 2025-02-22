// AUTO-GENERATED. DO NOT MODIFY. YOUR CHANGES WILL BE OVER-WRITTEN
use crate::products::TwilioNotifyError;
use standard_error::traits::StandardErrorCausesTrait;

impl StandardErrorCausesTrait for TwilioNotifyError {
    fn causes(&self) -> Option<&'static str> {
        match self {
            TwilioNotifyError::ErrorCode52402 => Some(r#"Various limitations such as reaching the maximum allowed template size of 6400 bytes or encountering a rendering timeout of 3000 milliseconds might contribute to this error."#),
            TwilioNotifyError::ErrorCode52117 => Some(r#"Detected the use of a legacy FCM server key credential. This credential has stopped working after Google has deprecated the legacy FCM API in June 2024. Your action is required in order for FCM push notifications to continue to work as the legacy FCM API is deprecated."#),
            TwilioNotifyError::ErrorCode52311 => Some(r#"Invalid callback URL or Internal Server Error"#),
            TwilioNotifyError::ErrorCode52310 => Some(r#"Notification took too much time or there are too many notifications"#),
            TwilioNotifyError::ErrorCode52403 => Some(r#"The error might be attributed to syntax errors within the provided text template."#),
            TwilioNotifyError::ErrorCode52115 => Some(r#"The role(s) granted to the provided service account do not include the cloudmessaging.messages.create permission"#),
            TwilioNotifyError::ErrorCode52404 => Some(r#"1. Missing variables might have been overlooked during template creation.
2. Variables could have been mistyped or not properly declared."#),
            TwilioNotifyError::ErrorCode52400 => Some(r#"Some syntax-specific iteration operators in the template, such as for-loops, cycles, table-rows, and others, may have reached the maximum limit."#)
        }
    }
}
