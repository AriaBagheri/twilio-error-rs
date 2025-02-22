// AUTO-GENERATED. DO NOT MODIFY. YOUR CHANGES WILL BE OVER-WRITTEN
use crate::products::TwilioChannelsError;
use standard_error::traits::StandardErrorMessageTrait;
use std::borrow::Cow;

impl StandardErrorMessageTrait for TwilioChannelsError {
    fn message(&self) -> Cow<'static, str> {
        match self {
            TwilioChannelsError::ErrorCode63108 => r#"Sender is not ready to add test device"#.into(),
            TwilioChannelsError::ErrorCode63106 => r#"phone_number is not a RCS capable number"#.into(),
            TwilioChannelsError::ErrorCode63103 => r#"Cannot assign payment method to WABA provided"#.into(),
            TwilioChannelsError::ErrorCode63107 => r#"phone_number must be a valid E.164 formatted phone number"#.into(),
            TwilioChannelsError::ErrorCode63101 => r#"WABA ID provided is not valid or unable to be used"#.into(),
            TwilioChannelsError::ErrorCode63105 => r#"Channel does not support this action"#.into(),
            TwilioChannelsError::ErrorCode63112 => r#"The Meta and/or WhatsApp Business Accounts connected to this Sender were disabled by Meta."#.into(),
            TwilioChannelsError::ErrorCode63109 => r#"This Sender has been migrated into a different account."#.into(),
            TwilioChannelsError::ErrorCode63100 => r#"Validation Error"#.into(),
            TwilioChannelsError::ErrorCode63111 => r#"Sender's phone number or WABA returned "not found"."#.into(),
            TwilioChannelsError::ErrorCode63102 => r#"Account already linked to another WABA ID"#.into()
        }
    }
}
