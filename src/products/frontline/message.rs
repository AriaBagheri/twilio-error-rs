// AUTO-GENERATED. DO NOT MODIFY. YOUR CHANGES WILL BE OVER-WRITTEN
use crate::products::TwilioFrontlineError;
use standard_error::traits::StandardErrorMessageTrait;
use std::borrow::Cow;

impl StandardErrorMessageTrait for TwilioFrontlineError {
    fn message(&self) -> Cow<'static, str> {
        match self {
            TwilioFrontlineError::ErrorCode48028 => r#"Outgoing conversation: Unauthorized use of the proxy address"#.into(),
            TwilioFrontlineError::ErrorCode48005 => r#"Callback failed due to timeout"#.into(),
            TwilioFrontlineError::ErrorCode48031 => r#"Outgoing conversation: Conversation with this contact and proxy address already exists"#.into(),
            TwilioFrontlineError::ErrorCode48029 => r#"Outgoing conversation: Contact address type does not match proxy address type"#.into(),
            TwilioFrontlineError::ErrorCode48011 => r#"Custom Routing Callback failed due to an internal error"#.into(),
            TwilioFrontlineError::ErrorCode48027 => r#"Outgoing conversation: Proxy address equals contact address"#.into(),
            TwilioFrontlineError::ErrorCode48025 => r#"Outgoing conversation: Invalid contact address"#.into(),
            TwilioFrontlineError::ErrorCode48032 => r#"Outgoing conversation: Missing Messaging service"#.into(),
            TwilioFrontlineError::ErrorCode48030 => r#"Outgoing conversation: Proxy address is not WhatsApp-enabled sender"#.into(),
            TwilioFrontlineError::ErrorCode48050 => r#"Internal service error"#.into(),
            TwilioFrontlineError::ErrorCode48001 => r#"Callback URL is not set"#.into(),
            TwilioFrontlineError::ErrorCode48024 => r#"Contact conversation limit exceeded"#.into(),
            TwilioFrontlineError::ErrorCode48010 => r#"Custom Routing Callback failed to execute successfully"#.into(),
            TwilioFrontlineError::ErrorCode48004 => r#"Callback returned an error"#.into(),
            TwilioFrontlineError::ErrorCode48000 => r#"Invalid request payload"#.into(),
            TwilioFrontlineError::ErrorCode48002 => r#"Callback URL is invalid"#.into(),
            TwilioFrontlineError::ErrorCode48026 => r#"Outgoing conversation: Invalid proxy address"#.into(),
            TwilioFrontlineError::ErrorCode48003 => r#"Callback returned an invalid response"#.into(),
            TwilioFrontlineError::ErrorCode48023 => r#"Frontline user conversation limit exceeded"#.into(),
            TwilioFrontlineError::ErrorCode48033 => r#"Outgoing conversation: Invalid contact identity"#.into()
        }
    }
}
