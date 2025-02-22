// AUTO-GENERATED. DO NOT MODIFY. YOUR CHANGES WILL BE OVER-WRITTEN
use crate::products::TwilioEventsError;
use standard_error::traits::StandardErrorCausesTrait;

impl StandardErrorCausesTrait for TwilioEventsError {
    fn causes(&self) -> Option<&'static str> {
        match self {
            TwilioEventsError::ErrorCode91003 => Some(r#"* Have 100 sinks created"#),
            TwilioEventsError::ErrorCode91004 => Some(r#"* Test event does not exist"#),
            TwilioEventsError::ErrorCode93104 => Some(r#"Bad ARN was provided when creating the sink"#),
            TwilioEventsError::ErrorCode91101 => Some(r#"* Event types are duplicated
* Description is empty"#),
            TwilioEventsError::ErrorCode91007 => Some(r#"* There are subscriptions using this sink"#),
            TwilioEventsError::ErrorCode91006 => Some(r#"The id for the sink has to start with a DG, followed by a 32 character UUID"#),
            TwilioEventsError::ErrorCode91005 => Some(r#"* The provided test id is not the one that was sent to the configured stream"#),
            TwilioEventsError::ErrorCode91000 => Some(r#"* Sink type is not Kinesis or Webhook
* Sink Configuration is empty
* Sink Configuration role-arn is empty
* Sink Configuration arn is empty
* Sink Configuration external_id is empty
* Sink Configuration destination is empty
* Sink Configuration method is empty
* Sink Configuration batch_events is empty"#),
            TwilioEventsError::ErrorCode93101 => Some(r#"See the error message for more details"#),
            TwilioEventsError::ErrorCode93103 => Some(r#"See Error Message for more details"#),
            TwilioEventsError::ErrorCode91102 => Some(r#"* Subscription does not exist
* Invalid sid"#),
            TwilioEventsError::ErrorCode91104 => Some(r#"* Event type is not supported by our system"#),
            TwilioEventsError::ErrorCode91103 => Some(r#"* Event types cannot be empty"#),
            TwilioEventsError::ErrorCode91001 => Some(r#"* Sink doesn't exist
* Invalid Sid"#),
            TwilioEventsError::ErrorCode91201 => Some(r#"* The transform template is not a valid template
* The JSON Schema is incorrect
* The value of one of the entry params is not correct"#),
            TwilioEventsError::ErrorCode91002 => Some(r#"Page and PageSize are invalid values"#),
            TwilioEventsError::ErrorCode93100 => Some(r#"* A temporary error failed the replay.
* An incident impacting Event Streams prevented the replay from completing.
* The replay ran for too long."#),
            TwilioEventsError::ErrorCode91100 => Some(r#"* Sink is not in an active state"#),
            TwilioEventsError::ErrorCode93102 => Some(r#"Most likely something has changed with your Role, or this is you first time using this Sink.  See Error Message for more details"#)
        }
    }
}
