// AUTO-GENERATED. DO NOT MODIFY. YOUR CHANGES WILL BE OVER-WRITTEN
use crate::products::TwilioContactsError;
use standard_error::traits::StandardErrorCausesTrait;

impl StandardErrorCausesTrait for TwilioContactsError {
    fn causes(&self) -> Option<&'static str> {
        match self {
            TwilioContactsError::ErrorCode19045 => Some(r#"Request with a custom field type other than Text, Number or Date"#),
            TwilioContactsError::ErrorCode19053 => Some(r#"Attempt to create custom field definition using reserved words"#),
            TwilioContactsError::ErrorCode19057 => Some(r#"We're having trouble completing your request "#),
            TwilioContactsError::ErrorCode19056 => Some(r#"Attempt to use invalid content type in input request"#),
            TwilioContactsError::ErrorCode19044 => Some(r#"Request with a custom field name greater than 100 chars"#),
            TwilioContactsError::ErrorCode19046 => Some(r#"Attempt to create more custom fields definitions than allowed"#),
            TwilioContactsError::ErrorCode19055 => Some(r#"User provided an invalid JSON or trying to update fields other than type, description and is_primary of a channel"#),
            TwilioContactsError::ErrorCode19048 => Some(r#"Unique in the request is missing or request contains invalid form data"#),
            TwilioContactsError::ErrorCode19054 => Some(r#"Attempt to create custom field definition without Unique form key"#),
            TwilioContactsError::ErrorCode19047 => Some(r#"Attempt to create custom field definition with empty value"#),
            TwilioContactsError::ErrorCode19052 => Some(r#"Invalid value for page size in the request URL (not an integer; not within a valid range)"#),
            TwilioContactsError::ErrorCode19043 => Some(r#"Custom field definition name in the request already exists"#),
            TwilioContactsError::ErrorCode19049 => Some(r#"Request with a custom field definition that is not defined"#)
        }
    }
}
