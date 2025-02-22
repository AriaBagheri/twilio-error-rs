// AUTO-GENERATED. DO NOT MODIFY. YOUR CHANGES WILL BE OVER-WRITTEN
use crate::products::TwilioStudioError;
use standard_error::traits::StandardErrorMessageTrait;
use std::borrow::Cow;

impl StandardErrorMessageTrait for TwilioStudioError {
    fn message(&self) -> Cow<'static, str> {
        match self {
            TwilioStudioError::ErrorCode81002 => r#"Unexpected event while processing Widget"#.into(),
            TwilioStudioError::ErrorCode81026 => r#"Studio Execution failed because Flow exceeds maximum allowed widgets"#.into(),
            TwilioStudioError::ErrorCode81007 => r#"Connecting to a Call timed out"#.into(),
            TwilioStudioError::ErrorCode81027 => r#"Error parsing type in Studio widget"#.into(),
            TwilioStudioError::ErrorCode81014 => r#"There was an internal error while processing an HTTP request"#.into(),
            TwilioStudioError::ErrorCode81012 => r#"Failed to update Sync service"#.into(),
            TwilioStudioError::ErrorCode81019 => r#"Twilio phone number using deprecated API version"#.into(),
            TwilioStudioError::ErrorCode81025 => r#"Studio Flow exceeds maximum allowed widgets"#.into(),
            TwilioStudioError::ErrorCode81017 => r#"Error in Twilio Function Response"#.into(),
            TwilioStudioError::ErrorCode81006 => r#"Failed to create Chat Channel"#.into(),
            TwilioStudioError::ErrorCode81005 => r#"Failed to transition because no match was found"#.into(),
            TwilioStudioError::ErrorCode81016 => r#"Outbound HTTP Request Failed"#.into(),
            TwilioStudioError::ErrorCode81024 => r#"Subflow Error"#.into(),
            TwilioStudioError::ErrorCode81004 => r#"Failed to add member to Chat Channel"#.into(),
            TwilioStudioError::ErrorCode81008 => r#"Failed to connect to outgoing Call"#.into(),
            TwilioStudioError::ErrorCode81013 => r#"Failed to invoke Understand API"#.into(),
            TwilioStudioError::ErrorCode81018 => r#"Template evaluation error in Studio widget"#.into(),
            TwilioStudioError::ErrorCode81021 => r#"Flow revision must be an integer or enum(LatestPublished, LatestRevision)"#.into(),
            TwilioStudioError::ErrorCode81022 => r#"Flow definition validation failed"#.into(),
            TwilioStudioError::ErrorCode81001 => r#"The Widget has exceeded max steps in a loop"#.into(),
            TwilioStudioError::ErrorCode81000 => r#"The Execution has exceeded max steps allowed for a flow"#.into(),
            TwilioStudioError::ErrorCode81009 => r#"Timed out enqueueing Call"#.into(),
            TwilioStudioError::ErrorCode81010 => r#"There was an internal error while processing a Function"#.into(),
            TwilioStudioError::ErrorCode81015 => r#"Failed to Create Task"#.into(),
            TwilioStudioError::ErrorCode81020 => r#"Unsupported Trigger Type"#.into(),
            TwilioStudioError::ErrorCode81023 => r#"Creating an Execution via REST API failed due to malformed contact parameters"#.into(),
            TwilioStudioError::ErrorCode81011 => r#"Failed to send Message"#.into()
        }
    }
}
