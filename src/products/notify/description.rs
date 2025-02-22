// AUTO-GENERATED. DO NOT MODIFY. YOUR CHANGES WILL BE OVER-WRITTEN
use crate::products::TwilioNotifyError;
use standard_error::traits::StandardErrorDescriptionTrait;

impl StandardErrorDescriptionTrait for TwilioNotifyError {
    fn description(&self) -> Option<&'static str> {
        match self {
            TwilioNotifyError::ErrorCode52402 => Some(r#"The error pertains to a general rendering issue with variables in an successfully parsed template."#),
            TwilioNotifyError::ErrorCode52117 => Some(r#"Google has deprecated the legacy FCM API in June 2024: https://help.twilio.com/articles/20768292997147-Updating-Twilio-Push-for-FCM-HTTP-v1-API"#),
            TwilioNotifyError::ErrorCode52311 => None,
            TwilioNotifyError::ErrorCode52310 => Some(r#"Notification expired during processing"#),
            TwilioNotifyError::ErrorCode52403 => Some(r#"An error may arise if the provided text template in the notification request cannot be parsed, potentially due to syntax errors in the input."#),
            TwilioNotifyError::ErrorCode52115 => Some(r#"The service account associated with the provided FCM credentials does not have the required permissions to send push notifications"#),
            TwilioNotifyError::ErrorCode52404 => Some(r#"The error indicates that there are one or more undefined variables in the provided template of the notification payload."#),
            TwilioNotifyError::ErrorCode52400 => Some(r#"Your template contains an iterative operation that has reached the maximum allowed limit of 1000 iterations in its definition."#)
        }
    }
}
