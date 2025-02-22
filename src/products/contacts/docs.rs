// AUTO-GENERATED. DO NOT MODIFY. YOUR CHANGES WILL BE OVER-WRITTEN
use crate::products::TwilioContactsError;
use standard_error::traits::StandardErrorDocsTrait;

impl StandardErrorDocsTrait for TwilioContactsError {
    fn docs(&self) -> &'static str {
        match self {
            TwilioContactsError::ErrorCode19045 => r#"## ERROR - 19045

### Field definition type is invalid; data types that are supported are text, date, and number

 The custom field type provided in the request is not valid

#### Possible Causes
Request with a custom field type other than Text, Number or Date

#### Possible Solutions
Custom field type can be only Text, Number or Date"#,
            TwilioContactsError::ErrorCode19053 => r#"## ERROR - 19053

### Field definition name cannot be a duplicate of an existing Twilio-defined field

 Request with input field names using reserved words

#### Possible Causes
Attempt to create custom field definition using reserved words

#### Possible Solutions
Update input field name with unreserved words"#,
            TwilioContactsError::ErrorCode19057 => r#"## ERROR - 19057

### Server unavailable or busy

 Server is busy or unavailable

#### Possible Causes
We're having trouble completing your request 

#### Possible Solutions
Please retry again"#,
            TwilioContactsError::ErrorCode19056 => r#"## ERROR - 19056

### Input request content type is invalid

 Input request content type is invalid

#### Possible Causes
Attempt to use invalid content type in input request

#### Possible Solutions
Provide correct content type in input request"#,
            TwilioContactsError::ErrorCode19044 => r#"## ERROR - 19044

### Field definition name exceeded maximum length

 The custom field in the request is more than 100 chars long

#### Possible Causes
Request with a custom field name greater than 100 chars

#### Possible Solutions
Update custom field name to be 100 chars or less"#,
            TwilioContactsError::ErrorCode19046 => r#"## ERROR - 19046

### Number of custom field definitions exceeded limit

 Maximum number of custom field definition allowed is 2000

#### Possible Causes
Attempt to create more custom fields definitions than allowed

#### Possible Solutions
Delete an existing custom field before adding a new one"#,
            TwilioContactsError::ErrorCode19055 => r#"## ERROR - 19055

### When updating a channel, invalid JSON syntax or invalid field that cannot be updated by this endpoint

 Invalid JSON or fields other than type, description and is_primary exist in update channel request

#### Possible Causes
User provided an invalid JSON or trying to update fields other than type, description and is_primary of a channel

#### Possible Solutions
Fix JSON input or change update channel request to update only type, description and is_primary"#,
            TwilioContactsError::ErrorCode19048 => r#"## ERROR - 19048

### Input request body is not properly json formatted

 Input request body is not properly json formatted

#### Possible Causes
Unique in the request is missing or request contains invalid form data

#### Possible Solutions
Provide correct request with unique and form data"#,
            TwilioContactsError::ErrorCode19054 => r#"## ERROR - 19054

### Expected Unique form key in input request is missing

 Expected Unique form key in input request is missing

#### Possible Causes
Attempt to create custom field definition without Unique form key

#### Possible Solutions
Provide Unique form key in input request"#,
            TwilioContactsError::ErrorCode19047 => r#"## ERROR - 19047

### Field definition name cannot be empty

 Input field name cannot be empty

#### Possible Causes
Attempt to create custom field definition with empty value

#### Possible Solutions
Update the input request field with valid name"#,
            TwilioContactsError::ErrorCode19052 => r#"## ERROR - 19052

### Invalid page size for custom field definition

 Invalid page size for custom field definition

#### Possible Causes
Invalid value for page size in the request URL (not an integer; not within a valid range)

#### Possible Solutions
Change the value to a parsable integer or to be within the valid range of 1 to 2000"#,
            TwilioContactsError::ErrorCode19043 => r#"## ERROR - 19043

### Field definition name already exists

 Input field name already exists

#### Possible Causes
Custom field definition name in the request already exists

#### Possible Solutions
Provide the unique custom field definition name in the request "#,
            TwilioContactsError::ErrorCode19049 => r#"## ERROR - 19049

### Custom field definition provided is not defined

 The custom field definition provided in the request does not exist

#### Possible Causes
Request with a custom field definition that is not defined

#### Possible Solutions
Use custom field definition API to define the custom field first"#
        }
    }
}
