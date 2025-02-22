// AUTO-GENERATED. DO NOT MODIFY. YOUR CHANGES WILL BE OVER-WRITTEN
use crate::products::TwilioEventsError;
use standard_error::traits::StandardErrorMessageTrait;
use std::borrow::Cow;

impl StandardErrorMessageTrait for TwilioEventsError {
    fn message(&self) -> Cow<'static, str> {
        match self {
            TwilioEventsError::ErrorCode91003 => r#"Account reached the max sink limit"#.into(),
            TwilioEventsError::ErrorCode91004 => r#"Test event cannot be found"#.into(),
            TwilioEventsError::ErrorCode93104 => r#"There is an issue with the Kinesis Stream Name or Region"#.into(),
            TwilioEventsError::ErrorCode91101 => r#"Subscription could not be created"#.into(),
            TwilioEventsError::ErrorCode91007 => r#"Sink still in use"#.into(),
            TwilioEventsError::ErrorCode91006 => r#"The sink sid is in an invalid format"#.into(),
            TwilioEventsError::ErrorCode91005 => r#"Test ID is invalid"#.into(),
            TwilioEventsError::ErrorCode91000 => r#"Sink could not be created"#.into(),
            TwilioEventsError::ErrorCode93101 => r#"Unable to deliver events to sink"#.into(),
            TwilioEventsError::ErrorCode93103 => r#"There was an error with your Kinesis stream"#.into(),
            TwilioEventsError::ErrorCode91102 => r#"Subscription could not be found"#.into(),
            TwilioEventsError::ErrorCode91104 => r#"Event type not found"#.into(),
            TwilioEventsError::ErrorCode91103 => r#"Event type list is empty"#.into(),
            TwilioEventsError::ErrorCode91001 => r#"Sink could not be found"#.into(),
            TwilioEventsError::ErrorCode91201 => r#"Bad Request"#.into(),
            TwilioEventsError::ErrorCode91002 => r#"Incorrect values for pagination"#.into(),
            TwilioEventsError::ErrorCode93100 => r#"Replay failed"#.into(),
            TwilioEventsError::ErrorCode91100 => r#"Subscription could not be created"#.into(),
            TwilioEventsError::ErrorCode93102 => r#"There was an error with your AWS role"#.into()
        }
    }
}
