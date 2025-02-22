// AUTO-GENERATED. DO NOT MODIFY. YOUR CHANGES WILL BE OVER-WRITTEN
use crate::products::TwilioContactsError;
use standard_error::traits::StandardErrorDescriptionTrait;

impl StandardErrorDescriptionTrait for TwilioContactsError {
    fn description(&self) -> Option<&'static str> {
        match self {
            TwilioContactsError::ErrorCode19045 => Some(r#"The custom field type provided in the request is not valid"#),
            TwilioContactsError::ErrorCode19053 => Some(r#"Request with input field names using reserved words"#),
            TwilioContactsError::ErrorCode19057 => Some(r#"Server is busy or unavailable"#),
            TwilioContactsError::ErrorCode19056 => Some(r#"Input request content type is invalid"#),
            TwilioContactsError::ErrorCode19044 => Some(r#"The custom field in the request is more than 100 chars long"#),
            TwilioContactsError::ErrorCode19046 => Some(r#"Maximum number of custom field definition allowed is 2000"#),
            TwilioContactsError::ErrorCode19055 => Some(r#"Invalid JSON or fields other than type, description and is_primary exist in update channel request"#),
            TwilioContactsError::ErrorCode19048 => Some(r#"Input request body is not properly json formatted"#),
            TwilioContactsError::ErrorCode19054 => Some(r#"Expected Unique form key in input request is missing"#),
            TwilioContactsError::ErrorCode19047 => Some(r#"Input field name cannot be empty"#),
            TwilioContactsError::ErrorCode19052 => Some(r#"Invalid page size for custom field definition"#),
            TwilioContactsError::ErrorCode19043 => Some(r#"Input field name already exists"#),
            TwilioContactsError::ErrorCode19049 => Some(r#"The custom field definition provided in the request does not exist"#)
        }
    }
}
