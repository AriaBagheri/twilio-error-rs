// AUTO-GENERATED. DO NOT MODIFY. YOUR CHANGES WILL BE OVER-WRITTEN
use crate::products::TwilioContactsError;
use standard_error::traits::StandardErrorMessageTrait;
use std::borrow::Cow;

impl StandardErrorMessageTrait for TwilioContactsError {
    fn message(&self) -> Cow<'static, str> {
        match self {
            TwilioContactsError::ErrorCode19045 => r#"Field definition type is invalid; data types that are supported are text, date, and number"#.into(),
            TwilioContactsError::ErrorCode19053 => r#"Field definition name cannot be a duplicate of an existing Twilio-defined field"#.into(),
            TwilioContactsError::ErrorCode19057 => r#"Server unavailable or busy"#.into(),
            TwilioContactsError::ErrorCode19056 => r#"Input request content type is invalid"#.into(),
            TwilioContactsError::ErrorCode19044 => r#"Field definition name exceeded maximum length"#.into(),
            TwilioContactsError::ErrorCode19046 => r#"Number of custom field definitions exceeded limit"#.into(),
            TwilioContactsError::ErrorCode19055 => r#"When updating a channel, invalid JSON syntax or invalid field that cannot be updated by this endpoint"#.into(),
            TwilioContactsError::ErrorCode19048 => r#"Input request body is not properly json formatted"#.into(),
            TwilioContactsError::ErrorCode19054 => r#"Expected Unique form key in input request is missing"#.into(),
            TwilioContactsError::ErrorCode19047 => r#"Field definition name cannot be empty"#.into(),
            TwilioContactsError::ErrorCode19052 => r#"Invalid page size for custom field definition"#.into(),
            TwilioContactsError::ErrorCode19043 => r#"Field definition name already exists"#.into(),
            TwilioContactsError::ErrorCode19049 => r#"Custom field definition provided is not defined"#.into()
        }
    }
}
