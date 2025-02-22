// AUTO-GENERATED. DO NOT MODIFY. YOUR CHANGES WILL BE OVER-WRITTEN
use crate::products::TwilioStudioError;
use standard_error::traits::StandardErrorDocsTrait;

impl StandardErrorDocsTrait for TwilioStudioError {
    fn docs(&self) -> &'static str {
        match self {
            TwilioStudioError::ErrorCode81002 => r#"## WARNING - 81002

### Unexpected event while processing Widget

 An unexpected event was received while processing a widget. Studio ignored this event and did not transition to another widget because there was no matching transition available to handle the event. 

*Note:* If the Execution ended correctly, this warning can be ignored.

#### Possible Causes
* A new call, message, or REST API Execution was initiated for a contact address (e.g. phone number) that already has an active Execution.
* An event came in while processing a widget that cannot handle that event (e.g. incoming message received during Run Function widget).
* A hangup for a phone call was detected while processing a widget that does not have a "Hangup" transition (e.g. HTTP Request or Send Message).
* There was no transition that matched the received event.

#### Possible Solutions
* Incoming Calls only: Enable [Concurrent Calls](https://www.twilio.com/docs/studio/user-guide#handling-concurrent-calls-from-the-same-number-in-studio) in the Flow's Trigger widget.
* REST API-triggered Executions: Use [Studio REST API v2](https://www.twilio.com/docs/studio/rest-api/v2/execution#create-a-new-execution) to create Executions and end any that are still active for the same contact.
* Return results for Function calls and HTTP requests quickly.
* End any [stuck Executions](https://support.twilio.com/hc/en-us/articles/360019383714-Understanding-and-Avoiding-Stuck-Executions-in-Twilio-Studio) via Twilio Console."#,
            TwilioStudioError::ErrorCode81026 => r#"## ERROR - 81026

### Studio Execution failed because Flow exceeds maximum allowed widgets

 The Execution failed because the cumulative number of widgets in your Studio Flow and all its linked Subflows exceeds the maximum allowed widget count of 2,000. All Executions for this Flow will fail until you reduce the number of widgets to 2,000 or fewer.

#### Possible Causes
* Too many widgets in your Subflows
* Subflows reused too many times in a parent Flow

#### Possible Solutions
* Reduce the number of widgets in your Subflows"#,
            TwilioStudioError::ErrorCode81007 => r#"## Error - 81007

### Connecting to a Call timed out
Studio timed out while executing the Connect Caller widget in the Flow.

#### Possible Causes
* The call could not be completed before the end of the timeout period.

#### Possible Solutions
* Check the Widget configuration. Are all fields filled out correctly?"#,
            TwilioStudioError::ErrorCode81027 => r#"## WARNING - 81027

### Error parsing type in Studio widget

 An error occurred when Studio attempted to parse a type in a widget

#### Possible Causes
* Tried to convert a variable to an incompatible type (e.g. "foo" to JSON Object type)
* The provided JSON string is not valid

#### Possible Solutions
* Verify the runtime data of your flow execution by checking the Flow Data and inspecting any variables you may have referenced
* Ensure that the configured type matches the variable data."#,
            TwilioStudioError::ErrorCode81014 => r#"## ERROR - 81014

### There was an internal error while processing an HTTP request

 An internal error occurred while Studio was executing an HTTP Request widget. 

#### Possible Causes
* Parameter name uses a reserved word, e.g. "method"
* URL was set to a Liquid variable that did not evaluate to a valid URL at runtime
* A transient internal error occurred

#### Possible Solutions
* Remove reserved words from the parameter names, e.g. "method"
* Check Flow logs to ensure a valid URL is being used in Liquid variables
* Retry the request
* Check [Twilio Status](https://status.twilio.com/)
* Contact [Twilio Support](https://www.twilio.com/help/contact)"#,
            TwilioStudioError::ErrorCode81012 => r#"## Error - 81012

### Failed to update Sync service
Studio was unable to update the Sync Service while executing the Flow

#### Possible Causes
* The request to the Sync API failed.
* Invalid values were passed to the Sync API.

#### Possible Solutions
* Check the Widget configuration. Are all fields filled out correctly?"#,
            TwilioStudioError::ErrorCode81019 => r#"## ERROR - 81019

### Twilio phone number using deprecated API version

Studio does not support 2008-08-01 API The Twilio phone number connected to your Studio Flow is configured for the deprecated 2008-08-01 API, which is not compatible. Update the phone number API version to 2010-04-01.

#### Possible Causes
The Twilio phone number is configured for the 2008-08-01 API

#### Possible Solutions
Update the phone number API version to 2010-04-01"#,
            TwilioStudioError::ErrorCode81025 => r#"## WARNING - 81025

### Studio Flow exceeds maximum allowed widgets

 The cumulative number of widgets in your Studio Flow and all its linked Subflows exceeds the maximum allowed widget count of 2,000. You must reduce the number of widgets to 2,000 or fewer to prevent future Executions from failing.

#### Possible Causes
* Too many widgets in your Subflows
* Subflows reused too many times in a parent Flow

#### Possible Solutions
* Reduce the number of widgets in your Subflows"#,
            TwilioStudioError::ErrorCode81017 => r#"## Error - 81017

### Error on Twilio Function response
There was an error in the response back from a Twilio Function attached to the Studio flow.

#### Possible Causes
* Your Function timed out before responding
* Your Function returned an error response

#### Possible Solutions
* Your Function must contain a callback.
* Make sure you place the Function callback `callback(err, response)` is placed correctly in your Function code.
 * If you are using a JavaScript promise, make sure the callback is called in both success and catch blocks.
* Your Function responded with an error."#,
            TwilioStudioError::ErrorCode81006 => r#"## Error - 81006

### Failed to create Chat Channel
Studio was unable to create the Chat Channel while executing the Flow

#### Possible Causes
* The request to the Chat API failed.
* Invalid values were passed to the Chat API.

#### Possible Solutions
* Check the Widget configuration. Are all fields filled out correctly?"#,
            TwilioStudioError::ErrorCode81005 => r#"## Warning - 81005

### Failed to transition because no match was found
Studio was unable to transition to the next widget because no matching transition was found in the Split widget.

#### Possible Causes
* Missing values in the match field.
* Match field is in the wrong format.
* Regular expression is too restrictive.

#### Possible Solutions
* Check the values that you’re matching against in the Split widget."#,
            TwilioStudioError::ErrorCode81016 => r#"## Error - 81016

### Outgoing HTTP request failed
The outgoing HTTP request from a Studio widget failed.

#### Possible Causes
* The URL you are requesting is incorrect
* The response is badly formed
* The URL returned a 4xx or 5xx error code

#### Possible Solutions
* Make sure the request results in a response code 2xx or 3xx
"#,
            TwilioStudioError::ErrorCode81024 => r#"## ERROR - 81024

### Subflow Error

Subflow could not be executed 

#### Possible Causes
* Subflow definition does not exist
* Subflow is trying to invoke another Subflow

#### Possible Solutions
* Ensure Subflow exists
* Remove Subflow widget from Subflow definition"#,
            TwilioStudioError::ErrorCode81004 => r#"## Error - 81004

### Failed to add member to Chat Channel
Studio was unable to add a member to the Chat Channel while executing the Flow

#### Possible Causes
* The request to the Chat API failed.
* Invalid values were passed to the Chat API.

#### Possible Solutions
* Check the Widget configuration. Are all fields filled out correctly?"#,
            TwilioStudioError::ErrorCode81008 => r#"## Error - 81008

### Failed to connect to outgoing Call
Studio was unable to connect a call while processing the Outgoing Call widget.

#### Possible Causes
* The request to the Voice API failed.
* Invalid values were passed to the Voice API.

#### Possible Solutions
* Check the Widget configuration. Are all fields filled out correctly?"#,
            TwilioStudioError::ErrorCode81013 => r#"## Error - 81013

### Failed to invoke Understand API
Studio was unable to invoke the Understand API while executing the Flow

#### Possible Causes
* The request to the Understand API failed.
* Invalid values were passed to the Understand API.

#### Possible Solutions
* Check the Widget configuration. Are all fields filled out correctly?"#,
            TwilioStudioError::ErrorCode81018 => r#"## WARNING - 81018

### Template evaluation error in Studio widget

 An error occurred when Studio tried to evaluate your template syntax.

#### Possible Causes
* The syntax used in your widget caused an error
* An array index referenced in your widget didn't exist at execution time
* Your Liquid curly braces are mismatched
* You used a `+` character in your template
* You have improperly assigned a variable
* You used a non-ASCII character in a variable name
* A tag or filter used in your template does not exist

#### Possible Solutions
* Check the syntax used in your widget (See Liquid docs [here](https://www.twilio.com/docs/studio/user-guide/liquid-template-language))
* Verify the runtime data of your flow execution by checking the Flow Data and inspecting any variables you may have referenced
* Verify that your Liquid curly braces are matched
* Verify that all tags and filters used in your template are correct (The list of available filters is [here](https://www.twilio.com/docs/studio/user-guide/liquid-template-language#standard-filters))
* Use only ASCII characters in variable names"#,
            TwilioStudioError::ErrorCode81021 => r#"## ERROR - 81021

### Flow revision must be an integer or enum(LatestPublished, LatestRevision)

 The Revision was invalid. Revision must be an integer or enum(LatestPublished, LatestRevision).

#### Possible Causes
* Revision did not match a valid option

#### Possible Solutions
* Check the value and ensure it is either an integer or one of the magic identifiers: LatestPublished or LatestRevision"#,
            TwilioStudioError::ErrorCode81022 => r#"## ERROR - 81022

### Flow definition validation failed

 The Flow Definition is invalid. Check the error response details for an array of validation failures.

#### Possible Causes
* Definition contains properties that do not match the schema
* Widget names are not unique
* Transition references a widget that doesn't exist

#### Possible Solutions
* Verify the definition adheres to the JSON schema
* Ensure widget names are all unique
* Remove transitions that reference widgets that don't exist"#,
            TwilioStudioError::ErrorCode81001 => r#"## Warning - 81001

### Widget has exceeded max steps in a loop
This limitation is enforced to prevent infinite loops. Please try to design your flows such that they terminate.

#### Possible Causes
* You have a single widget looping back on itself too many times.

#### Possible Solutions
* Remove the single widget loop."#,
            TwilioStudioError::ErrorCode81000 => r#"## WARNING - 81000

### The Execution has exceeded max steps allowed for a flow

 This limitation is enforced to prevent infinite loops. Please try to design your flows such that they terminate.

#### Possible Causes
* You have too many steps in your flow.

#### Possible Solutions
* Reduce the number of steps in your flow."#,
            TwilioStudioError::ErrorCode81009 => r#"## Error - 81009

### Timed out enqueueing Call
Studio timed out while executing the Enqueue Call widget in the Flow.

#### Possible Causes
* The request to the Taskrouter API failed.
* Invalid values were passed to the Taskrouter API.

#### Possible Solutions
* Check the Widget configuration. Are all fields filled out correctly?"#,
            TwilioStudioError::ErrorCode81010 => r#"## Error - 81010

### There was an internal error while processing a Function
Studio could not invoke the Function service.

#### Possible Causes
* The Function returned a 4xx or 5xx status code
* The Function timed out or did not include `callback()`
* There may be an issue with Twilio Function Service

#### Possible Solutions
* Verify your Function calls `callback()` to avoid timeouts
* Check your Function logs for runtime errors
* Ensure you're returning the correct status code"#,
            TwilioStudioError::ErrorCode81015 => r#"## Error - 81015

### Task was not successfully created
An error occurred when Studio attempted to create task.

#### Possible Causes
* Required values are missing from the widget.
* Values are not correctly formatted (e.g. Attributes)

#### Possible Solutions
* Check the Widget configuration. Are all fields filled out correctly?
"#,
            TwilioStudioError::ErrorCode81020 => r#"## WARNING - 81020

### Unsupported Trigger Type

 An error occurred when Studio tried to run a widget that is not supported for the trigger type.

#### Possible Causes
* Flow was triggered by an incoming event that isn't supported for this widget

#### Possible Solutions
* Flow should be updated to use only the supported trigger types for this widget
* For Send To Flex widget, only incoming call and incoming chat are supported"#,
            TwilioStudioError::ErrorCode81023 => r#"## ERROR - 81023

### Creating an Execution via REST API failed due to malformed contact parameters

When creating a flow execution via the REST API, the To and From parameters may not collectively exceed 300 bytes. The Create Execution endpoint was called with To and/or From parameters that could not be processed.

#### Possible Causes
* Collectively, the To and From parameters exceeded 300 bytes.

#### Possible Solutions
Double-check the provided To and From parameters for syntax errors."#,
            TwilioStudioError::ErrorCode81011 => r#"## Error - 81011

### Failed to send Message
Studio was unable to send a Message. This is usually due to incorrect setup of a Message Widget.

#### Possible Causes
* You have an invalid number.
* You are missing a message body.
* You are missing a Media URL

#### Possible Solutions
* Ensure the message number is valid.
* Ensure you have set a correct message body in the Widget.
* Ensure you have setup a Media URL."#
        }
    }
}
