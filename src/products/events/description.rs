// AUTO-GENERATED. DO NOT MODIFY. YOUR CHANGES WILL BE OVER-WRITTEN
use crate::products::TwilioEventsError;
use standard_error::traits::StandardErrorDescriptionTrait;

impl StandardErrorDescriptionTrait for TwilioEventsError {
    fn description(&self) -> Option<&'static str> {
        match self {
            TwilioEventsError::ErrorCode91003 => Some(r#"Account reached the max sink limit"#),
            TwilioEventsError::ErrorCode91004 => Some(r#"Test event cannot be found"#),
            TwilioEventsError::ErrorCode93104 => Some(r#"The ARN for your Kinesis Stream is invalid"#),
            TwilioEventsError::ErrorCode91101 => Some(r#"Subscription creation failed"#),
            TwilioEventsError::ErrorCode91007 => Some(r#"Sink cannot be deleted because it is sill in use"#),
            TwilioEventsError::ErrorCode91006 => Some(r#"The sink sid provided does not match the expected format (dg[0-9a-f]{32})"#),
            TwilioEventsError::ErrorCode91005 => Some(r#"Test ID is invalid"#),
            TwilioEventsError::ErrorCode91000 => Some(r#"Creating sink failed due to bad configuration"#),
            TwilioEventsError::ErrorCode93101 => Some(r#"There is an issue with sending event to your sink"#),
            TwilioEventsError::ErrorCode93103 => Some(r#"Event Streams is unable to write to the Kinesis Stream"#),
            TwilioEventsError::ErrorCode91102 => Some(r#"Subscription could not be found"#),
            TwilioEventsError::ErrorCode91104 => Some(r#"Event type not found"#),
            TwilioEventsError::ErrorCode91103 => Some(r#"Event type list is empty"#),
            TwilioEventsError::ErrorCode91001 => Some(r#"Sink could not be found"#),
            TwilioEventsError::ErrorCode91201 => Some(r#"Incorrect information provided in the request "#),
            TwilioEventsError::ErrorCode91002 => Some(r#"Pagination failed due to bad paging values"#),
            TwilioEventsError::ErrorCode93100 => Some(r#"An internal error occurred when trying to replay the events and the replay was aborted."#),
            TwilioEventsError::ErrorCode91100 => Some(r#"Creating subscription for sink failed"#),
            TwilioEventsError::ErrorCode93102 => Some(r#"There is an issue with your role"#)
        }
    }
}
