// AUTO-GENERATED. DO NOT MODIFY. YOUR CHANGES WILL BE OVER-WRITTEN
use crate::products::TwilioNotifyError;
use standard_error::traits::StandardErrorMessageTrait;
use std::borrow::Cow;

impl StandardErrorMessageTrait for TwilioNotifyError {
    fn message(&self) -> Cow<'static, str> {
        match self {
            TwilioNotifyError::ErrorCode52402 => r#"General rendering problem with variables in a parsed template"#.into(),
            TwilioNotifyError::ErrorCode52117 => r#"Deprecated GCM/FCM API is used to send notifications"#.into(),
            TwilioNotifyError::ErrorCode52311 => r#"Delivery callback invocation failed"#.into(),
            TwilioNotifyError::ErrorCode52310 => r#"Notification TTL has expired"#.into(),
            TwilioNotifyError::ErrorCode52403 => r#"Provided text template in the notification request cannot be parsed"#.into(),
            TwilioNotifyError::ErrorCode52115 => r#"Service account does not have permission to send FCM messages"#.into(),
            TwilioNotifyError::ErrorCode52404 => r#"Undefined variable within the notification payload template"#.into(),
            TwilioNotifyError::ErrorCode52400 => r#"Exceeded maximum iterations in template rendering"#.into()
        }
    }
}
