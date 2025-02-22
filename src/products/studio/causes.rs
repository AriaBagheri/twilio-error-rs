// AUTO-GENERATED. DO NOT MODIFY. YOUR CHANGES WILL BE OVER-WRITTEN
use crate::products::TwilioStudioError;
use standard_error::traits::StandardErrorCausesTrait;

impl StandardErrorCausesTrait for TwilioStudioError {
    fn causes(&self) -> Option<&'static str> {
        match self {
            TwilioStudioError::ErrorCode81002 => Some(r#"* A new call, message, or REST API Execution was initiated for a contact address (e.g. phone number) that already has an active Execution.
* An event came in while processing a widget that cannot handle that event (e.g. incoming message received during Run Function widget).
* A hangup for a phone call was detected while processing a widget that does not have a "Hangup" transition (e.g. HTTP Request or Send Message).
* There was no transition that matched the received event."#),
            TwilioStudioError::ErrorCode81026 => Some(r#"* Too many widgets in your Subflows
* Subflows reused too many times in a parent Flow"#),
            TwilioStudioError::ErrorCode81007 => None,
            TwilioStudioError::ErrorCode81027 => Some(r#"* Tried to convert a variable to an incompatible type (e.g. "foo" to JSON Object type)
* The provided JSON string is not valid"#),
            TwilioStudioError::ErrorCode81014 => Some(r#"* Parameter name uses a reserved word, e.g. "method"
* URL was set to a Liquid variable that did not evaluate to a valid URL at runtime
* A transient internal error occurred"#),
            TwilioStudioError::ErrorCode81012 => None,
            TwilioStudioError::ErrorCode81019 => Some(r#"The Twilio phone number is configured for the 2008-08-01 API"#),
            TwilioStudioError::ErrorCode81025 => Some(r#"* Too many widgets in your Subflows
* Subflows reused too many times in a parent Flow"#),
            TwilioStudioError::ErrorCode81017 => None,
            TwilioStudioError::ErrorCode81006 => None,
            TwilioStudioError::ErrorCode81005 => None,
            TwilioStudioError::ErrorCode81016 => None,
            TwilioStudioError::ErrorCode81024 => Some(r#"* Subflow definition does not exist
* Subflow is trying to invoke another Subflow"#),
            TwilioStudioError::ErrorCode81004 => None,
            TwilioStudioError::ErrorCode81008 => None,
            TwilioStudioError::ErrorCode81013 => None,
            TwilioStudioError::ErrorCode81018 => Some(r#"* The syntax used in your widget caused an error
* An array index referenced in your widget didn't exist at execution time
* Your Liquid curly braces are mismatched
* You used a `+` character in your template
* You have improperly assigned a variable
* You used a non-ASCII character in a variable name
* A tag or filter used in your template does not exist"#),
            TwilioStudioError::ErrorCode81021 => Some(r#"* Revision did not match a valid option"#),
            TwilioStudioError::ErrorCode81022 => Some(r#"* Definition contains properties that do not match the schema
* Widget names are not unique
* Transition references a widget that doesn't exist"#),
            TwilioStudioError::ErrorCode81001 => None,
            TwilioStudioError::ErrorCode81000 => Some(r#"* You have too many steps in your flow."#),
            TwilioStudioError::ErrorCode81009 => None,
            TwilioStudioError::ErrorCode81010 => None,
            TwilioStudioError::ErrorCode81015 => None,
            TwilioStudioError::ErrorCode81020 => Some(r#"* Flow was triggered by an incoming event that isn't supported for this widget"#),
            TwilioStudioError::ErrorCode81023 => Some(r#"* Collectively, the To and From parameters exceeded 300 bytes."#),
            TwilioStudioError::ErrorCode81011 => None
        }
    }
}
