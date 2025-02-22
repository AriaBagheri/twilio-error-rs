// AUTO-GENERATED. DO NOT MODIFY. YOUR CHANGES WILL BE OVER-WRITTEN
use crate::products::TwilioTaskrouterError;
use standard_error::traits::StandardErrorDocsTrait;

impl StandardErrorDocsTrait for TwilioTaskrouterError {
    fn docs(&self) -> &'static str {
        match self {
            TwilioTaskrouterError::ErrorCode40114 => r#"## Error - 40114

### The Reservation must currently be Pending - Call Instruction

#### Possible Causes 
*  The Reservation has been moved to a non-Pending state.

#### Possible Solutions
*  Make certain that the Reservation is currently Pending. 

#### Example

```
{
	"instruction": "call",
	"to": "client:alice",
	"from": "bob",
	"url": "https://example.com/twiml"
}
```"#,
            TwilioTaskrouterError::ErrorCode40121 => r#"## Error - 40121

### Missing or Invalid 'call_sid' parameter - Redirect Instruction

#### Possible Causes 
*  You did not provide a "call_sid" parameter on your 'redirect' instruction.
*  You provided an invalid value for the "call_sid" parameter. 

#### Possible Solutions
*  Make certain "call_sid" is provided an is a 34char string starting with CA that represents a live call. 

#### Example

```
{
	"instruction": "redirect",
	"call_sid": "CA0123456789abcdef0123456789abcdef",
	"url": "https://example.com/twiml?querystring=1234"
}
```"#,
            TwilioTaskrouterError::ErrorCode14233 => r#"## Error - 14233

### Conference: Provided Timeout was not a valid integer

Provided Timeout is invalid.
"#,
            TwilioTaskrouterError::ErrorCode40142 => r#"## ERROR - 40142

### Failed to issue Conference instruction due to missing 'call_sid' or 'outbound_to' properties

  

#### Possible Causes
*  The Task was not created via the `<Enqueue>` verb. (Inbound Voice call)
* The `call_sid` property is missing the Task’s attributes. (Inbound Voice call)
* The `outbound_to` property is missing in the Task’s attributes. (Outbound Voice call)


#### Possible Solutions
* Ensure that the Conference instruction is issued on an `<Enqueue>` created Task. (Inbound Voice call)
* Ensure that ‘outbound_to’ property is present in the Task’s attributes. (Outbound Voice call)"#,
            TwilioTaskrouterError::ErrorCode14236 => r#"## Error - 14236

### Conference: Invalid ReservationSid

The provided ReservationSid on the Conference TwiML was invalid.
"#,
            TwilioTaskrouterError::ErrorCode40143 => r#"## Error - 40143

### Invalid Activity Sid - Conference Instruction

#### Possible Causes 
*  You provided a 'post_work_activity_sid' that was invalid or no longer exists.  

#### Possible Solutions
*  Confirm a valid ActivitySid is being passed in the 'post_work_activity_sid' parameter and it exists in your workspace. 

#### Example

```
{
	"instruction": "conference",
	"room_name": "abc",
	"to": "client:alice",
	"from": "18001234567",
	"post_work_activity_sid": "WA0123456789abcdef0123456789abcdef"
}
```"#,
            TwilioTaskrouterError::ErrorCode40001 => r#"## WARNING - 40001

### Could not parse Assignment Instruction response as JSON. Ensure your JSON is not escaped

  
#### Example

##### Valid JSON (Not Escaped)
~~~json
{
  "instruction": "call",
  "from": "+15558675309",
  "url": "http://example.com/agent_answer",   
  "status_callback_url":
    "http://example.com/agent_answer_status_callback"
}
~~~

##### Invalid JSON (Escaped)
~~~json
{\r\n  \"instruction\": \"call\",\r\n  \"from\": \"+15558675309\",\r\n  \"url\": \"http:\/\/example.com\/agent_answer\",   \r\n  \"status_callback_url\":\r\n    \"http:\/\/example.com\/agent_answer_status_callback\"\r\n}
~~~



#### Possible Causes
JSON is escaped

#### Possible Solutions
Ensure your JSON is not escaped"#,
            TwilioTaskrouterError::ErrorCode40120 => r#"## Error - 40120

### Missing or Invalid 'url' parameter - Redirect Instruction

#### Possible Causes 
*  You did not provide a "url" parameter on your 'redirect' instruction.
*  You provided an invalid format in your "url" field.  

#### Possible Solutions
*  Make certain "url" is provided and contains a well formed HTTP or HTTPS URL. 

#### Example

```
{
	"instruction": "redirect",
	"call_sid": "CA0123456789abcdef0123456789abcdef",
	"url": "https://example.com/twiml?querystring=1234"
}
```"#,
            TwilioTaskrouterError::ErrorCode40130 => r#"## Error - 40130

### Invalid Activity Sid - Dequeue Instruction

#### Possible Causes 
*  You provided a "post_work_activity_sid"" that was invalid or no longer exists.  

#### Possible Solutions
*  Confirm a valid ActivitySid is being passed in the "post_work_activity_sid" parameter exists in your workspace. 

#### Example

```
{
	"instruction": "dequeue",
	"to": "client:alice",
	"from": "bob",
	"post_work_activity_sid": "WA0123456789abcdef0123456789abcdef"
}
```"#,
            TwilioTaskrouterError::ErrorCode40138 => r#"## ERROR - 40138

### Missing 'from' parameter when issuing Conference instruction

 A valid 'from' parameter was not passed to the 'conference' instruction

#### Possible Causes
* A 'from' parameter was not passed when issuing the Conference instruction
* The 'from' parameter supplied to the Conference instruction was invalid

#### Possible Solutions
* Supply a valid 'from' parameter when issuing the Conference instruction

#### Example

```
{
	"instruction": "conference",
	"to": "client:alice",
	"from": "+18001234567"
}
```"#,
            TwilioTaskrouterError::ErrorCode40135 => r#"## Error - 40135

### The Reservation must currently be Pending - Dequeue Instruction

#### Possible Causes 
*  The Reservation has been moved to a non-Pending state.

#### Possible Solutions
*  Make certain that the Reservation is currently Pending. 


#### Example

```
{
	"instruction": "dequeue",
	"to": "client:alice",
	"from": "18001234567",
	"post_work_activity_sid": "WA0123456789abcdef0123456789abcdef"
}
```"#,
            TwilioTaskrouterError::ErrorCode40152 => r#"## ERROR - 40152

### Invalid Queue for Known Worker

 Worker isn't part of the queue specified in the Workflow target for re-targeted Task.

#### Possible Causes
The Worker targeted by the Known Worker Routing feature is not a member of the TaskQueue referenced in the Workflow Target

#### Possible Solutions
Ensure the Worker is a member of the TaskQueue referenced in the Workflow Target. Find out more about the Known Worker Routing feature here: https://www.twilio.com/docs/taskrouter/workflow-configuration/known-agent-routing"#,
            TwilioTaskrouterError::ErrorCode40005 => r#"## WARNING - 40005

### Assignment Callback response does not contain Instruction

 Assignment Callback response does not contain 'instruction'



#### Possible Causes
Assignment Callback response is a 200 with some json that doesn't contain 'instruction'.

#### Possible Solutions
Do not respond to the Assignment Callback or issue an empty json response."#,
            TwilioTaskrouterError::ErrorCode14238 => r#"## Error - 14238

### Conference: Unable to accept or find the Reservation

Unable to accept or find the provided Reservation.
"#,
            TwilioTaskrouterError::ErrorCode40155 => r#"## ERROR - 40155

### Invalid Workspace for Known Worker

 Worker is not part of the workspace referenced by the Workflow target

#### Possible Causes
The Worker targeted by the Known Worker Routing feature is not a member of the Workspace referenced by the Workflow Target

#### Possible Solutions
Ensure the Worker is a member of the Workspace referenced by the Workflow that is configured to use Known Agent Routing. 
Find out more about the Known Worker Routing feature here: https://www.twilio.com/docs/taskrouter/workflow-configuration/known-agent-routing"#,
            TwilioTaskrouterError::ErrorCode40140 => r#"## ERROR - 40140

### Failed to issue Dequeue instruction due to missing 'call_sid' property

  

#### Possible Causes
* The Task was not created via the `<Enqueue>` verb
* The `call_sid` property is missing in the Task's attributes

#### Possible Solutions
* Only issue Dequeue instruction on Tasks created via the `<Enqueue>` verb
* Ensure that `call_sid` property is present in the Task's attributes"#,
            TwilioTaskrouterError::ErrorCode14230 => r#"## Error - 14230

### Conference: Provided WorkflowSid was not valid

Provided Workflow is invalid.
"#,
            TwilioTaskrouterError::ErrorCode14217 => r#"### Dial->Queue: Could not find or accept provided reservationSid

Provided ReservationSid could not be found or has been canceled / rescinded."#,
            TwilioTaskrouterError::ErrorCode40158 => r#"## ERROR - 40158

### Call Instruction does not have a valid 'status_callback_url' parameter

 Invalid 'status_callback_url' parameter - Call Instruction

#### Possible Causes
Provided value for the "status_callback_url" parameter is either an empty string or of an incorrect datatype.

#### Possible Solutions
"status_callback_url" should be a well formed HTTP or HTTPS URL string."#,
            TwilioTaskrouterError::ErrorCode40157 => r#"## ERROR - 40157

### Dequeue Instruction does not have a valid 'status_callback_url' parameter

 Invalid 'status_callback_url' parameter - Dequeue Instruction

#### Possible Causes
You have provided either an empty string or a value with incorrect datatype for the “status_callback_url” parameter.

#### Possible Solutions
"status_callback_url" should be a well formed HTTP or HTTPS URL string."#,
            TwilioTaskrouterError::ErrorCode40137 => r#"## ERROR - 40137

### Missing 'to' parameter when issuing Conference instruction

  A valid 'to' parameter must be provided when using the Conference Instruction



#### Possible Causes
- A 'to' parameter was not passed when issuing the Conference instruction
- The 'to' parameter supplied to the Conference instruction was invalid

#### Possible Solutions
* Supply a valid 'to' parameter when issuing the Conference instruction

#### Example

```
{
	"instruction": "conference",
	"to": "client:alice",
	"from": "+18001234567"
}
```"#,
            TwilioTaskrouterError::ErrorCode40147 => r#"## ERROR - 40147

### Failed to issue Supervise instruction due to missing or invalid 'contact_uri' property

  

#### Possible Causes
* The `contact_uri` is missing in the Supervisor's Worker attributes.
* The `contact_uri` in the Supervisor's Worker attributes is invalid.

#### Possible Solutions
* Add a valid `contact_uri` to the Supervisor’s Worker attributes."#,
            TwilioTaskrouterError::ErrorCode40151 => r#"## Error - 40151

### Call API Error - Invalid instruction issued on a task being transferred

#### Possible Causes 
*  Instruction is not acceptable for transferred tasks 
*  Internal Twilio API error. 

#### Possible Solutions
*  Make certain that instruction is accept, reject, conference, or supervise

#### Example

```
{
	"instruction": "conference"
}
```"#,
            TwilioTaskrouterError::ErrorCode40134 => r#"## Error - 40134

### Invalid 'status_callback_events' parameter - Dequeue Instruction

#### Possible Causes 
*  You provided an invalid format in your "status_callback_events" field.  

#### Possible Solutions
*  Make certain "status_callback_events" contains a valid comma separated string of events to subscribe to if responding to an assignment callback
*  Make certain each "DequeueStatusCallbackEvent" contains an acceptable event to subscribe to if making a POST /v1/Workspaces/{WorkspaceSid}/Tasks/{TaskSid}/Reservations/{ReservationSid}

#### Example

```
{
	"instruction": "dequeue",
	"to": "client:alice",
	"from": "bob",
	"post_work_activity_sid": "WA0123456789abcdef0123456789abcdef",
    "status_callback_events": "initiated,ringing,answered,completed"
}
```"#,
            TwilioTaskrouterError::ErrorCode14239 => r#"## Error - 14239

### Conference: Unable to update Worker's activity

Unable to update the Worker to the provided ActivitySid. "#,
            TwilioTaskrouterError::ErrorCode40000 => r#"### Content-Type of 'application/json' not set

#### Possible Causes 
*  You responded with a 200 HTTP response but did not provide a Content-Type header of 'application/json'

#### Possible Solutions
*  Add Content-Type header of 'application/json'

#### Example

~~~.json
{
	"instruction": "accept"
}
~~~

with no Content-Type header provided.
"#,
            TwilioTaskrouterError::ErrorCode40123 => r#"## Error - 40123

### The Reservation must currently be Pending - Redirect Instruction

#### Possible Causes 
*  The Reservation has been moved to a non-Pending state.

#### Possible Solutions
*  Make certain that the Reservation is currently Pending. 

#### Example

```
{
	"instruction": "redirect",
	"call_sid": "CA0123456789abcdef0123456789abcdef",
	"url": "https://example.com/twiml?querystring=1234"
}
```"#,
            TwilioTaskrouterError::ErrorCode14222 => r#"## Error - 14222

### Enqueue: Could not create Task

Could not match the task to a target given the defined attributes.
"#,
            TwilioTaskrouterError::ErrorCode14240 => r#"## ERROR - 14240

### Max concurrent Workers exceeded

 Max concurrent Workers exceeded

#### Possible Causes
Max concurrent worker limit met/exceeded

#### Possible Solutions
Reduce TaskRouter Workers currently in "Active" status"#,
            TwilioTaskrouterError::ErrorCode14235 => r#"## Error - 14235

### Conference: Unable to cleanup Task

Unable to cancel the Task created by Conference upon a TwiML error or task hangup.
"#,
            TwilioTaskrouterError::ErrorCode14234 => r#"## Error - 14234

### Conference: Unable to create Task

Unable to create the Task specified under Conference upon a TwiML error.
"#,
            TwilioTaskrouterError::ErrorCode14237 => r#"## Error - 14237

### Conference: Invalid PostWorkActivitySid

The provided PostWorkActivitySid on the Conference TwiML was an invalid ActivitySid.
"#,
            TwilioTaskrouterError::ErrorCode40111 => r#"## Error - 40111

### Missing or Invalid 'from' parameter - Call Instruction

#### Possible Causes 
*  You did not provide a "from" parameter on your 'call' instruction.
*  You provided an invalid format in your "from" field.  

#### Possible Solutions
*  Make certain "from" contains a validated caller ID if you're dialing the PSTN. 
*  Make certain "from" contains an acceptable caller field when connecting to client or SIP.  

#### Example

```
{
	"instruction": "call",
	"to": "client:alice",
	"from": "bob",
	"url": "https://example.com/twiml"
}
```"#,
            TwilioTaskrouterError::ErrorCode14241 => r#"## WARNING - 14241

### start_date passed to TaskRouter statistics is older than 30 days.

 Starting Nov. 30, 2021 we will no longer return data older than 30 days, and instead return a 400-response for requests with <start_date> older than 30 days.

#### Possible Causes
<start_date> is older than 30 days.

#### Possible Solutions
Pass a <start_date> that is within 30 days."#,
            TwilioTaskrouterError::ErrorCode40144 => r#"## Error - 40144

### Reservation is no longer Pending - Conference Instruction

#### Possible Causes 
* The original caller has hungup
* The Task has been moved into a 'canceled' state

#### Possible Solutions
*  Make sure the Task created has not been canceled prior to issuing 'conference' instruction

#### Example

```
{
	"instruction": "conference",
	"room_name": "abc",
	"to": "client:alice",
	"from": "18001234567",
	"post_work_activity_sid": "WA0123456789abcdef0123456789abcdef"
}
```"#,
            TwilioTaskrouterError::ErrorCode40100 => r#"## ERROR - 40100

### TaskRouter->Reject

Your Reject instruction does not include a valid `activity_sid` parameter. &nbsp;

#### Possible Causes
* You did not include an `activity_sid` parameter in your `reject` instruction.
* You did include an `activity_sid` but it was invalid or no longer exists. 

#### Possible Solutions
* Confirm a valid `ActivitySid` value is is being passed by your code into the request payload JSON’s `activity_sid` parameter: `{ "instruction": "reject", "activity_sid": "WA0123456789abcdef0123456789abcdef" }`"#,
            TwilioTaskrouterError::ErrorCode40149 => r#"## ERROR - 40149

### Failed to issue Conference instruction due to missing ‘conference_sid’

 The conference instruction issued as a result of the transfer can only be issued on a task which has a `conference_sid` in its attributes.

#### Possible Causes
* The Conference instruction was issued on the wrong Task.
* The Task attributes were modified and no longer contain the `conference_sid` property.

#### Possible Solutions
* Confirm the Conference instruction is issued on the correct voice Task.
* Ensure that the `conference_sid` is present in the Task’s attributes."#,
            TwilioTaskrouterError::ErrorCode40131 => r#"## Error - 40131

### Missing or Invalid 'to' parameter - Dequeue Instruction

#### Possible Causes 
*  You did not provide a "to" parameter on your 'dequeue' instruction.
*  You provided an invalid format in your "to" field.  

#### Possible Solutions
*  Make certain "to" contains a valid phone number, client URI, or SIP URI. 

#### Example

```
{
	"instruction": "dequeue",
	"to": "client:alice",
	"from": "bob",
	"post_work_activity_sid": "WA0123456789abcdef0123456789abcdef"
}
```"#,
            TwilioTaskrouterError::ErrorCode40141 => r#"## Error - 40141

### Missing or Invalid 'from' parameter - Conference Instruction

#### Possible Causes
*  You did not provide a 'from' parameter on your 'conference' instruction.
*  You provided an invalid format in your "from" field.

#### Possible Solutions
*  Make certain 'from' contains a validated caller ID if you're dialing the PSTN.
*  Make certain 'from' contains an acceptable caller field.

#### Example

```
{
	"instruction": "conference",
	"room_name": "abc",
	"to": "client:alice",
	"from": "18001234567",
	"post_work_activity_sid": "WA0123456789abcdef0123456789abcdef"
}
```
"#,
            TwilioTaskrouterError::ErrorCode14231 => r#"## Error - 14231

### Conference: Provided Attributes JSON was not valid

Provided Attributes is not valid JSON.
"#,
            TwilioTaskrouterError::ErrorCode55555 => r#"## ERROR - 55555

### Invalid Instruction passed to TaskRouter

 An invalid instruction was passed to TaskRouter when responding to a pending Reservation.

#### Possible Causes
* An invalid instruction was passed to TaskRouter.


#### Possible Solutions
* Ensure only valid instructions are being used. A list of valid instructions can be found at https://www.twilio.com/docs/taskrouter/api/reservations#action-update"#,
            TwilioTaskrouterError::ErrorCode40153 => r#"## WARNING - 40153

### Absolute Paging used when iterating TaskRouter resources

 The absolute paging query type that is being used has been deprecated. Starting Dec. 1, 2021 we will no longer return data for this query, and instead return a 400-response for requests of query type /Workers?Page=<x>&PageSize=<y>.

Please update your application to instead utilize relative paging: /Workers?PageToken=PA<task_sid>, where <task_sid> is the last item of the current page.

#### Possible Causes
Not using the latest Twilio helper libraries. Constructing requests directly which use Page & PageSize query parameters directly.

#### Possible Solutions
Utilize the latest Twilio helper libraries. Rely upon the PageToken instead of the Page Number for paging."#,
            TwilioTaskrouterError::ErrorCode14232 => r#"## Error - 14232

### Conference: Provided Priority was not a valid integer

Provided Priority is invalid.
"#,
            TwilioTaskrouterError::ErrorCode40145 => r#"## Error - 40145

### Call API Error - Conference Instruction

#### Possible Causes 
*  Un-routable address provided for 'to' field. 
*  Invalid caller-id passed to 'from' field. 
*  Missing 'room_name' field.
*  Internal Twilio API error. 

#### Possible Solutions
*  Make certain 'to' contains a routable phone number, client URI or SIP URI. 
*  Make certain 'to' is to a country you have permissions set to call.
*  Make certain 'from' contains an acceptable caller ID for the chosen 'to' address.
*  Make certain 'room_name' is a valid Conference Room Name.

#### Example

```
{
	"instruction": "conference",
	"room_name": "abc",
	"to": "client:alice",
	"from": "18001234567",
	"post_work_activity_sid": "WA0123456789abcdef0123456789abcdef"
}
```"#,
            TwilioTaskrouterError::ErrorCode40136 => r#"## Error - 40136

### The Reservation must currently be Pending - Conference Instruction

#### Possible Causes 
*  The Reservation has been moved to a non-Pending state.

#### Possible Solutions
*  Make certain that the Reservation is currently Pending. 


#### Example

```
{
	"instruction": "conference",
	"to": "client:alice",
	"from": "+18001234567"
}
```"#,
            TwilioTaskrouterError::ErrorCode40132 => r#"## Error - 40132

### Missing or Invalid 'from' parameter - Dequeue Instruction

#### Possible Causes 
*  You did not provide a "from" parameter on your 'dequeue' instruction.
*  You provided an invalid format in your "from" field.  

#### Possible Solutions
*  Make certain "from" contains a validated caller ID if you're dialing the PSTN. 
*  Make certain "from" contains an acceptable caller field when connecting to client or SIP.  

#### Example

```
{
	"instruction": "dequeue",
	"to": "client:alice",
	"from": "bob",
	"post_work_activity_sid": "WA0123456789abcdef0123456789abcdef"
}
```"#,
            TwilioTaskrouterError::ErrorCode40112 => r#"## Error - 40112

### Missing or Invalid 'url' parameter - Call Instruction

#### Possible Causes 
*  You did not provide a "url" parameter on your 'call' instruction.
*  You provided an invalid format in your "url" field.  

#### Possible Solutions
*  Make certain "url" is provided and contains a well formed HTTP or HTTPS URL. 

#### Example

```
{
	"instruction": "call",
	"to": "client:alice",
	"from": "bob",
	"url": "https://example.com/twiml?querystring=1234"
}
```"#,
            TwilioTaskrouterError::ErrorCode40122 => r#"## Error - 40122

### Call Redirection API Error - Redirect Instruction

#### Possible Causes 
*  The call being redirected is no longer in progress.
*  The call_sid passed doesn't reference an actual call

#### Possible Solutions
*  Make certain "call_sid" is provided an is a 34 char string starting with CA that represents a live call. 

#### Example

```
{
	"instruction": "redirect",
	"call_sid": "CA0123456789abcdef0123456789abcdef",
	"url": "https://example.com/twiml?querystring=1234"
}
```"#,
            TwilioTaskrouterError::ErrorCode40113 => r#"## Error - 40113

### Error Making Outgoing Call - Call Instruction

#### Possible Causes 
*  Un-routable address provided for "to" field. 
*  Invalid caller-id passed to "from" field. 
*  Poorly formed URL passed to "url" field. 
*  Internal Twilio API error.  

#### Possible Solutions
*  Make certain "to" contains a routable phone number, client URI or SIP URI. 
*  Make certain "to" is to a country you have permissions set to call.
*  Make certain "from" contains an acceptable caller ID for the chosen "to" address.
*  Make certain "url" is a valid HTTP/HTTPS URL. 

#### Example

```
{
	"instruction": "call",
	"to": "client:alice",
	"from": "bob",
	"url": "https://example.com/twiml"
}
```"#,
            TwilioTaskrouterError::ErrorCode40133 => r#"## Error - 40133

### Call API Error - Dequeue Instruction

#### Possible Causes 
*  Un-routable address provided for "to" field. 
*  Invalid caller-id passed to "from" field. 
*  Internal Twilio API error. 

#### Possible Solutions
*  Make certain "to" contains a routable phone number, client URI or SIP URI. 
*  Make certain "to" is to a country you have permissions set to call.
*  Make certain "from" contains an acceptable caller ID for the chosen "to" address.

#### Example

```
{
	"instruction": "dequeue",
	"to": "client:alice",
	"from": "18001234567",
	"post_work_activity_sid": "WA0123456789abcdef0123456789abcdef"
}
```"#,
            TwilioTaskrouterError::ErrorCode40148 => r#"## ERROR - 40148

### Failed to issue Supervise instruction due to missing ‘from’ property

 A valid `from` parameter was not passed to the ‘supervise’ instruction

#### Possible Causes
* The `from` property is missing in the Task’s attributes.
* The `from` property in the Task’s attributes is invalid.

#### Possible Solutions
*  Ensure that a valid `from` property is provided in the Task’s attributes."#,
            TwilioTaskrouterError::ErrorCode40154 => r#"## ERROR - 40154

### Invalid Known Worker information

 Worker information is missing or not valid.

#### Possible Causes
The Task does not have a field that holds the Known Worker information, the Worker information specified is invalid, or the specified Worker does not exist in TaskRouter.

#### Possible Solutions
Ensure the Task field referenced in the Workflow definition is provided in the Task's attributes. If Worker information is provided, make sure that the Worker does exist in TaskRouter. Find out more about the Known Worker Routing feature here: https://www.twilio.com/docs/taskrouter/workflow-configuration/known-agent-routing"#,
            TwilioTaskrouterError::ErrorCode40159 => r#"## ERROR - 40159

### Conference Instruction does not have a valid 'status_callback_url' parameter

 Invalid 'status_callback_url' parameter - Conference Instruction

#### Possible Causes
Provided value for the "status_callback_url" parameter is either an empty string or of an incorrect datatype.

#### Possible Solutions
"status_callback_url" should be a well formed HTTP or HTTPS URL string."#,
            TwilioTaskrouterError::ErrorCode40139 => r#"## ERROR - 40139

### Failed to issue Conference Instruction due to invalid ActivitySid

 Conference Instruction requires a valid `post_work_activity_sid`

#### Possible Causes
*  The provided  `post_work_activity_sid` is invalid or no longer exists in the operating Workspace.  

#### Possible Solutions
* Ensure that a valid ActivitySid is provided in the `post_work_activity_sid` property in the Worker’s attributes or request parameters.

#### Example

```
{
	"instruction": "conference",
	"to": "client:alice",
	"from": "bob",
	"post_work_activity_sid": "WA0123456789abcdef0123456789abcdef"
}
```
"#,
            TwilioTaskrouterError::ErrorCode40110 => r#"## Error - 40110

### Missing or Invalid 'to' parameter - Call Instruction

#### Possible Causes 
*  You did not provide a "to" parameter on your 'call' instruction.
*  You provided an invalid format in your "to" field.  

#### Possible Solutions
*  Make certain "to" contains a valid phone number, client URI, or SIP URI. 

#### Example

```
{
	"instruction": "call",
	"to": "client:alice",
	"from": "bob",
	"url": "https://example.com/twiml"
}
```"#,
            TwilioTaskrouterError::ErrorCode40146 => r#"## ERROR - 40146

### Failed to issue Supervise Instruction due to invalid Reservation state

 The Reservation is not in a valid state for the Supervise instruction to be used, the Reservation cannot be pending or canceled.

#### Possible Causes
* A Reservation has not yet been accepted by a Worker.
* The Reservation has already been canceled.

#### Possible Solutions
* Confirm that the Reservation has been accepted by the assigned Worker"#
        }
    }
}
