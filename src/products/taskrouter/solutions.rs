// AUTO-GENERATED. DO NOT MODIFY. YOUR CHANGES WILL BE OVER-WRITTEN
use crate::products::TwilioTaskrouterError;
use standard_error::traits::StandardErrorSolutionsTrait;

impl StandardErrorSolutionsTrait for TwilioTaskrouterError {
    fn solutions(&self) -> Option<&'static str> {
        match self {
            TwilioTaskrouterError::ErrorCode40114 => None,
            TwilioTaskrouterError::ErrorCode40121 => None,
            TwilioTaskrouterError::ErrorCode14233 => None,
            TwilioTaskrouterError::ErrorCode40142 => Some(r#"* Ensure that the Conference instruction is issued on an `<Enqueue>` created Task. (Inbound Voice call)
* Ensure that ‘outbound_to’ property is present in the Task’s attributes. (Outbound Voice call)"#),
            TwilioTaskrouterError::ErrorCode14236 => None,
            TwilioTaskrouterError::ErrorCode40143 => None,
            TwilioTaskrouterError::ErrorCode40001 => Some(r#"Ensure your JSON is not escaped"#),
            TwilioTaskrouterError::ErrorCode40120 => None,
            TwilioTaskrouterError::ErrorCode40130 => None,
            TwilioTaskrouterError::ErrorCode40138 => Some(r#"* Supply a valid 'from' parameter when issuing the Conference instruction

#### Example

```
{
	"instruction": "conference",
	"to": "client:alice",
	"from": "+18001234567"
}
```"#),
            TwilioTaskrouterError::ErrorCode40135 => None,
            TwilioTaskrouterError::ErrorCode40152 => Some(r#"Ensure the Worker is a member of the TaskQueue referenced in the Workflow Target. Find out more about the Known Worker Routing feature here: https://www.twilio.com/docs/taskrouter/workflow-configuration/known-agent-routing"#),
            TwilioTaskrouterError::ErrorCode40005 => Some(r#"Do not respond to the Assignment Callback or issue an empty json response."#),
            TwilioTaskrouterError::ErrorCode14238 => None,
            TwilioTaskrouterError::ErrorCode40155 => Some(r#"Ensure the Worker is a member of the Workspace referenced by the Workflow that is configured to use Known Agent Routing. 
Find out more about the Known Worker Routing feature here: https://www.twilio.com/docs/taskrouter/workflow-configuration/known-agent-routing"#),
            TwilioTaskrouterError::ErrorCode40140 => Some(r#"* Only issue Dequeue instruction on Tasks created via the `<Enqueue>` verb
* Ensure that `call_sid` property is present in the Task's attributes"#),
            TwilioTaskrouterError::ErrorCode14230 => None,
            TwilioTaskrouterError::ErrorCode14217 => None,
            TwilioTaskrouterError::ErrorCode40158 => Some(r#""status_callback_url" should be a well formed HTTP or HTTPS URL string."#),
            TwilioTaskrouterError::ErrorCode40157 => Some(r#""status_callback_url" should be a well formed HTTP or HTTPS URL string."#),
            TwilioTaskrouterError::ErrorCode40137 => Some(r#"* Supply a valid 'to' parameter when issuing the Conference instruction

#### Example

```
{
	"instruction": "conference",
	"to": "client:alice",
	"from": "+18001234567"
}
```"#),
            TwilioTaskrouterError::ErrorCode40147 => Some(r#"* Add a valid `contact_uri` to the Supervisor’s Worker attributes."#),
            TwilioTaskrouterError::ErrorCode40151 => None,
            TwilioTaskrouterError::ErrorCode40134 => None,
            TwilioTaskrouterError::ErrorCode14239 => None,
            TwilioTaskrouterError::ErrorCode40000 => None,
            TwilioTaskrouterError::ErrorCode40123 => None,
            TwilioTaskrouterError::ErrorCode14222 => None,
            TwilioTaskrouterError::ErrorCode14240 => Some(r#"Reduce TaskRouter Workers currently in "Active" status"#),
            TwilioTaskrouterError::ErrorCode14235 => None,
            TwilioTaskrouterError::ErrorCode14234 => None,
            TwilioTaskrouterError::ErrorCode14237 => None,
            TwilioTaskrouterError::ErrorCode40111 => None,
            TwilioTaskrouterError::ErrorCode14241 => Some(r#"Pass a <start_date> that is within 30 days."#),
            TwilioTaskrouterError::ErrorCode40144 => None,
            TwilioTaskrouterError::ErrorCode40100 => Some(r#"* Confirm a valid `ActivitySid` value is is being passed by your code into the request payload JSON’s `activity_sid` parameter: `{ "instruction": "reject", "activity_sid": "WA0123456789abcdef0123456789abcdef" }`"#),
            TwilioTaskrouterError::ErrorCode40149 => Some(r#"* Confirm the Conference instruction is issued on the correct voice Task.
* Ensure that the `conference_sid` is present in the Task’s attributes."#),
            TwilioTaskrouterError::ErrorCode40131 => None,
            TwilioTaskrouterError::ErrorCode40141 => None,
            TwilioTaskrouterError::ErrorCode14231 => None,
            TwilioTaskrouterError::ErrorCode55555 => Some(r#"* Ensure only valid instructions are being used. A list of valid instructions can be found at https://www.twilio.com/docs/taskrouter/api/reservations#action-update"#),
            TwilioTaskrouterError::ErrorCode40153 => Some(r#"Utilize the latest Twilio helper libraries. Rely upon the PageToken instead of the Page Number for paging."#),
            TwilioTaskrouterError::ErrorCode14232 => None,
            TwilioTaskrouterError::ErrorCode40145 => None,
            TwilioTaskrouterError::ErrorCode40136 => None,
            TwilioTaskrouterError::ErrorCode40132 => None,
            TwilioTaskrouterError::ErrorCode40112 => None,
            TwilioTaskrouterError::ErrorCode40122 => None,
            TwilioTaskrouterError::ErrorCode40113 => None,
            TwilioTaskrouterError::ErrorCode40133 => None,
            TwilioTaskrouterError::ErrorCode40148 => Some(r#"*  Ensure that a valid `from` property is provided in the Task’s attributes."#),
            TwilioTaskrouterError::ErrorCode40154 => Some(r#"Ensure the Task field referenced in the Workflow definition is provided in the Task's attributes. If Worker information is provided, make sure that the Worker does exist in TaskRouter. Find out more about the Known Worker Routing feature here: https://www.twilio.com/docs/taskrouter/workflow-configuration/known-agent-routing"#),
            TwilioTaskrouterError::ErrorCode40159 => Some(r#""status_callback_url" should be a well formed HTTP or HTTPS URL string."#),
            TwilioTaskrouterError::ErrorCode40139 => Some(r#"* Ensure that a valid ActivitySid is provided in the `post_work_activity_sid` property in the Worker’s attributes or request parameters.

#### Example

```
{
	"instruction": "conference",
	"to": "client:alice",
	"from": "bob",
	"post_work_activity_sid": "WA0123456789abcdef0123456789abcdef"
}
```
"#),
            TwilioTaskrouterError::ErrorCode40110 => None,
            TwilioTaskrouterError::ErrorCode40146 => Some(r#"* Confirm that the Reservation has been accepted by the assigned Worker"#)
        }
    }
}
