// AUTO-GENERATED. DO NOT MODIFY. YOUR CHANGES WILL BE OVER-WRITTEN
use crate::products::TwilioEventsError;
use standard_error::traits::StandardErrorSolutionsTrait;

impl StandardErrorSolutionsTrait for TwilioEventsError {
    fn solutions(&self) -> Option<&'static str> {
        match self {
            TwilioEventsError::ErrorCode91003 => Some(r#"* Cannot have more than 100 sinks. Make sure to delete at least one sinks before creating another."#),
            TwilioEventsError::ErrorCode91004 => Some(r#"* Make sure you send the test event before validating."#),
            TwilioEventsError::ErrorCode93104 => Some(r#"Create a new sink and put in a valid ARN"#),
            TwilioEventsError::ErrorCode91101 => Some(r#"* Ensure event types are not duplicated
* Description is not empty"#),
            TwilioEventsError::ErrorCode91007 => Some(r#"* Delete all the subscriptions that use this sink. You can use the Event Streams - Subscriptions API endpoint to list all your subscriptions filtered by SinkSid and then delete them."#),
            TwilioEventsError::ErrorCode91006 => Some(r#"Make sure the sink sid is in the expected format"#),
            TwilioEventsError::ErrorCode91005 => Some(r#"* Ensure test id is valid and exists in your stream"#),
            TwilioEventsError::ErrorCode91000 => Some(r#"* Description cannot be empty
* Sink configuration values cannot be empty
* Sink type must be Kinesis or Webhook"#),
            TwilioEventsError::ErrorCode93101 => Some(r#"Verify that your sink destination is up and has not changed"#),
            TwilioEventsError::ErrorCode93103 => Some(r#"Check your Kinesis Stream and ensure it is configured correctly for the load of events you are subscribed too,  Check to see if there is an AWS outage with Kinesis streams."#),
            TwilioEventsError::ErrorCode91102 => Some(r#"* Make sure subscription exists
* Pass an valid sid for the subscription"#),
            TwilioEventsError::ErrorCode91104 => Some(r#"* Use a supported Event"#),
            TwilioEventsError::ErrorCode91103 => Some(r#"* Ensure event types are not empty"#),
            TwilioEventsError::ErrorCode91001 => Some(r#"* Ensure Sink exists
* Valid Sink ID"#),
            TwilioEventsError::ErrorCode91201 => Some(r#"* Recheck the input information and validate against examples"#),
            TwilioEventsError::ErrorCode91002 => Some(r#"Page and PageSize must be integers greater than or equal than zero"#),
            TwilioEventsError::ErrorCode93100 => Some(r#"* If there is an incident in the [status page](https://status.twilio.com/), wait for it to be mitigated and retry the replay.
* Otherwise, wait a few minutes and retry the replay."#),
            TwilioEventsError::ErrorCode91100 => Some(r#"* Ensure sink is in an active state before creating subscription"#),
            TwilioEventsError::ErrorCode93102 => Some(r#"If this Role was working check to see if it was changed.  If you need those changes you will need to create a new sink and subscription to receive your events.  If this is the first time using this Sink, verify the config is correct for this sink."#)
        }
    }
}
