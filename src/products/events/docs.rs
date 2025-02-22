// AUTO-GENERATED. DO NOT MODIFY. YOUR CHANGES WILL BE OVER-WRITTEN
use crate::products::TwilioEventsError;
use standard_error::traits::StandardErrorDocsTrait;

impl StandardErrorDocsTrait for TwilioEventsError {
    fn docs(&self) -> &'static str {
        match self {
            TwilioEventsError::ErrorCode91003 => r#"## ERROR - 91003

### Account reached the max sink limit

 Account reached the max sink limit

#### Possible Causes
* Have 100 sinks created

#### Possible Solutions
* Cannot have more than 100 sinks. Make sure to delete at least one sinks before creating another."#,
            TwilioEventsError::ErrorCode91004 => r#"## ERROR - 91004

### Test event cannot be found

 Test event cannot be found

#### Possible Causes
* Test event does not exist

#### Possible Solutions
* Make sure you send the test event before validating."#,
            TwilioEventsError::ErrorCode93104 => r#"## ERROR - 93104

### There is an issue with the Kinesis Stream Name or Region

The ARN provided for your Kinesis Stream is invalid The ARN for your Kinesis Stream is invalid

#### Possible Causes
Bad ARN was provided when creating the sink

#### Possible Solutions
Create a new sink and put in a valid ARN"#,
            TwilioEventsError::ErrorCode91101 => r#"## ERROR - 91101

### Subscription could not be created

 Subscription creation failed

#### Possible Causes
* Event types are duplicated
* Description is empty

#### Possible Solutions
* Ensure event types are not duplicated
* Description is not empty"#,
            TwilioEventsError::ErrorCode91007 => r#"## ERROR - 91007

### Sink still in use

There are subscriptions using this sink Sink cannot be deleted because it is sill in use

#### Possible Causes
* There are subscriptions using this sink

#### Possible Solutions
* Delete all the subscriptions that use this sink. You can use the Event Streams - Subscriptions API endpoint to list all your subscriptions filtered by SinkSid and then delete them."#,
            TwilioEventsError::ErrorCode91006 => r#"## ERROR - 91006

### The sink sid is in an invalid format

 The sink sid provided does not match the expected format (dg[0-9a-f]{32})

#### Possible Causes
The id for the sink has to start with a DG, followed by a 32 character UUID

#### Possible Solutions
Make sure the sink sid is in the expected format"#,
            TwilioEventsError::ErrorCode91005 => r#"## ERROR - 91005

### Test ID is invalid

 Test ID is invalid

#### Possible Causes
* The provided test id is not the one that was sent to the configured stream

#### Possible Solutions
* Ensure test id is valid and exists in your stream"#,
            TwilioEventsError::ErrorCode91000 => r#"## ERROR - 91000

### Sink could not be created

 Creating sink failed due to bad configuration

#### Possible Causes
* Sink type is not Kinesis or Webhook
* Sink Configuration is empty
* Sink Configuration role-arn is empty
* Sink Configuration arn is empty
* Sink Configuration external_id is empty
* Sink Configuration destination is empty
* Sink Configuration method is empty
* Sink Configuration batch_events is empty

#### Possible Solutions
* Description cannot be empty
* Sink configuration values cannot be empty
* Sink type must be Kinesis or Webhook"#,
            TwilioEventsError::ErrorCode93101 => r#"## ERROR - 93101

### Unable to deliver events to sink

The sink you are using for your Event Streams subscription is having issues There is an issue with sending event to your sink

#### Possible Causes
See the error message for more details

#### Possible Solutions
Verify that your sink destination is up and has not changed"#,
            TwilioEventsError::ErrorCode93103 => r#"## ERROR - 93103

### There was an error with your Kinesis stream

The Kinesis Stream associated with this Sink is having issues. Event Streams is unable to write to the Kinesis Stream

#### Possible Causes
See Error Message for more details

#### Possible Solutions
Check your Kinesis Stream and ensure it is configured correctly for the load of events you are subscribed too,  Check to see if there is an AWS outage with Kinesis streams."#,
            TwilioEventsError::ErrorCode91102 => r#"## ERROR - 91102

### Subscription could not be found

 Subscription could not be found

#### Possible Causes
* Subscription does not exist
* Invalid sid

#### Possible Solutions
* Make sure subscription exists
* Pass an valid sid for the subscription"#,
            TwilioEventsError::ErrorCode91104 => r#"## ERROR - 91104

### Event type not found

 Event type not found

#### Possible Causes
* Event type is not supported by our system

#### Possible Solutions
* Use a supported Event"#,
            TwilioEventsError::ErrorCode91103 => r#"## ERROR - 91103

### Event type list is empty

 Event type list is empty

#### Possible Causes
* Event types cannot be empty

#### Possible Solutions
* Ensure event types are not empty"#,
            TwilioEventsError::ErrorCode91001 => r#"## ERROR - 91001

### Sink could not be found

 Sink could not be found

#### Possible Causes
* Sink doesn't exist
* Invalid Sid

#### Possible Solutions
* Ensure Sink exists
* Valid Sink ID"#,
            TwilioEventsError::ErrorCode91201 => r#"## ERROR - 91201

### Bad Request

 Incorrect information provided in the request 

#### Possible Causes
* The transform template is not a valid template
* The JSON Schema is incorrect
* The value of one of the entry params is not correct

#### Possible Solutions
* Recheck the input information and validate against examples"#,
            TwilioEventsError::ErrorCode91002 => r#"## ERROR - 91002

### Incorrect values for pagination

 Pagination failed due to bad paging values

#### Possible Causes
Page and PageSize are invalid values

#### Possible Solutions
Page and PageSize must be integers greater than or equal than zero"#,
            TwilioEventsError::ErrorCode93100 => r#"## ERROR - 93100

### Replay failed

Replay could not be completed due to an internal error An internal error occurred when trying to replay the events and the replay was aborted.

#### Possible Causes
* A temporary error failed the replay.
* An incident impacting Event Streams prevented the replay from completing.
* The replay ran for too long.

#### Possible Solutions
* If there is an incident in the [status page](https://status.twilio.com/), wait for it to be mitigated and retry the replay.
* Otherwise, wait a few minutes and retry the replay."#,
            TwilioEventsError::ErrorCode91100 => r#"## ERROR - 91100

### Subscription could not be created

 Creating subscription for sink failed

#### Possible Causes
* Sink is not in an active state

#### Possible Solutions
* Ensure sink is in an active state before creating subscription"#,
            TwilioEventsError::ErrorCode93102 => r#"## ERROR - 93102

### There was an error with your AWS role

The role associated with your AWS Sink is not allowing Event Streams to send events There is an issue with your role

#### Possible Causes
Most likely something has changed with your Role, or this is you first time using this Sink.  See Error Message for more details

#### Possible Solutions
If this Role was working check to see if it was changed.  If you need those changes you will need to create a new sink and subscription to receive your events.  If this is the first time using this Sink, verify the config is correct for this sink."#
        }
    }
}
