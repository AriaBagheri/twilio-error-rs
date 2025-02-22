// AUTO-GENERATED. DO NOT MODIFY. YOUR CHANGES WILL BE OVER-WRITTEN
use crate::products::TwilioContactsError;
use standard_error::traits::StandardErrorSolutionsTrait;

impl StandardErrorSolutionsTrait for TwilioContactsError {
    fn solutions(&self) -> Option<&'static str> {
        match self {
            TwilioContactsError::ErrorCode19045 => Some(r#"Custom field type can be only Text, Number or Date"#),
            TwilioContactsError::ErrorCode19053 => Some(r#"Update input field name with unreserved words"#),
            TwilioContactsError::ErrorCode19057 => Some(r#"Please retry again"#),
            TwilioContactsError::ErrorCode19056 => Some(r#"Provide correct content type in input request"#),
            TwilioContactsError::ErrorCode19044 => Some(r#"Update custom field name to be 100 chars or less"#),
            TwilioContactsError::ErrorCode19046 => Some(r#"Delete an existing custom field before adding a new one"#),
            TwilioContactsError::ErrorCode19055 => Some(r#"Fix JSON input or change update channel request to update only type, description and is_primary"#),
            TwilioContactsError::ErrorCode19048 => Some(r#"Provide correct request with unique and form data"#),
            TwilioContactsError::ErrorCode19054 => Some(r#"Provide Unique form key in input request"#),
            TwilioContactsError::ErrorCode19047 => Some(r#"Update the input request field with valid name"#),
            TwilioContactsError::ErrorCode19052 => Some(r#"Change the value to a parsable integer or to be within the valid range of 1 to 2000"#),
            TwilioContactsError::ErrorCode19043 => Some(r#"Provide the unique custom field definition name in the request "#),
            TwilioContactsError::ErrorCode19049 => Some(r#"Use custom field definition API to define the custom field first"#)
        }
    }
}
