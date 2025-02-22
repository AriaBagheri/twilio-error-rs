// AUTO-GENERATED. DO NOT MODIFY. YOUR CHANGES WILL BE OVER-WRITTEN
use crate::products::TwilioTaskrouterError;
use standard_error::traits::StandardErrorCausesTrait;

impl StandardErrorCausesTrait for TwilioTaskrouterError {
    fn causes(&self) -> Option<&'static str> {
        match self {
            TwilioTaskrouterError::ErrorCode40114 => None,
            TwilioTaskrouterError::ErrorCode40121 => None,
            TwilioTaskrouterError::ErrorCode14233 => None,
            TwilioTaskrouterError::ErrorCode40142 => Some(r#"*  The Task was not created via the `<Enqueue>` verb. (Inbound Voice call)
* The `call_sid` property is missing the Task’s attributes. (Inbound Voice call)
* The `outbound_to` property is missing in the Task’s attributes. (Outbound Voice call)
"#),
            TwilioTaskrouterError::ErrorCode14236 => None,
            TwilioTaskrouterError::ErrorCode40143 => None,
            TwilioTaskrouterError::ErrorCode40001 => Some(r#"JSON is escaped"#),
            TwilioTaskrouterError::ErrorCode40120 => None,
            TwilioTaskrouterError::ErrorCode40130 => None,
            TwilioTaskrouterError::ErrorCode40138 => Some(r#"* A 'from' parameter was not passed when issuing the Conference instruction
* The 'from' parameter supplied to the Conference instruction was invalid"#),
            TwilioTaskrouterError::ErrorCode40135 => None,
            TwilioTaskrouterError::ErrorCode40152 => Some(r#"The Worker targeted by the Known Worker Routing feature is not a member of the TaskQueue referenced in the Workflow Target"#),
            TwilioTaskrouterError::ErrorCode40005 => Some(r#"Assignment Callback response is a 200 with some json that doesn't contain 'instruction'."#),
            TwilioTaskrouterError::ErrorCode14238 => None,
            TwilioTaskrouterError::ErrorCode40155 => Some(r#"The Worker targeted by the Known Worker Routing feature is not a member of the Workspace referenced by the Workflow Target"#),
            TwilioTaskrouterError::ErrorCode40140 => Some(r#"* The Task was not created via the `<Enqueue>` verb
* The `call_sid` property is missing in the Task's attributes"#),
            TwilioTaskrouterError::ErrorCode14230 => None,
            TwilioTaskrouterError::ErrorCode14217 => None,
            TwilioTaskrouterError::ErrorCode40158 => Some(r#"Provided value for the "status_callback_url" parameter is either an empty string or of an incorrect datatype."#),
            TwilioTaskrouterError::ErrorCode40157 => Some(r#"You have provided either an empty string or a value with incorrect datatype for the “status_callback_url” parameter."#),
            TwilioTaskrouterError::ErrorCode40137 => Some(r#"- A 'to' parameter was not passed when issuing the Conference instruction
- The 'to' parameter supplied to the Conference instruction was invalid"#),
            TwilioTaskrouterError::ErrorCode40147 => Some(r#"* The `contact_uri` is missing in the Supervisor's Worker attributes.
* The `contact_uri` in the Supervisor's Worker attributes is invalid."#),
            TwilioTaskrouterError::ErrorCode40151 => None,
            TwilioTaskrouterError::ErrorCode40134 => None,
            TwilioTaskrouterError::ErrorCode14239 => None,
            TwilioTaskrouterError::ErrorCode40000 => None,
            TwilioTaskrouterError::ErrorCode40123 => None,
            TwilioTaskrouterError::ErrorCode14222 => None,
            TwilioTaskrouterError::ErrorCode14240 => Some(r#"Max concurrent worker limit met/exceeded"#),
            TwilioTaskrouterError::ErrorCode14235 => None,
            TwilioTaskrouterError::ErrorCode14234 => None,
            TwilioTaskrouterError::ErrorCode14237 => None,
            TwilioTaskrouterError::ErrorCode40111 => None,
            TwilioTaskrouterError::ErrorCode14241 => Some(r#"<start_date> is older than 30 days."#),
            TwilioTaskrouterError::ErrorCode40144 => None,
            TwilioTaskrouterError::ErrorCode40100 => Some(r#"* You did not include an `activity_sid` parameter in your `reject` instruction.
* You did include an `activity_sid` but it was invalid or no longer exists. "#),
            TwilioTaskrouterError::ErrorCode40149 => Some(r#"* The Conference instruction was issued on the wrong Task.
* The Task attributes were modified and no longer contain the `conference_sid` property."#),
            TwilioTaskrouterError::ErrorCode40131 => None,
            TwilioTaskrouterError::ErrorCode40141 => None,
            TwilioTaskrouterError::ErrorCode14231 => None,
            TwilioTaskrouterError::ErrorCode55555 => Some(r#"* An invalid instruction was passed to TaskRouter.
"#),
            TwilioTaskrouterError::ErrorCode40153 => Some(r#"Not using the latest Twilio helper libraries. Constructing requests directly which use Page & PageSize query parameters directly."#),
            TwilioTaskrouterError::ErrorCode14232 => None,
            TwilioTaskrouterError::ErrorCode40145 => None,
            TwilioTaskrouterError::ErrorCode40136 => None,
            TwilioTaskrouterError::ErrorCode40132 => None,
            TwilioTaskrouterError::ErrorCode40112 => None,
            TwilioTaskrouterError::ErrorCode40122 => None,
            TwilioTaskrouterError::ErrorCode40113 => None,
            TwilioTaskrouterError::ErrorCode40133 => None,
            TwilioTaskrouterError::ErrorCode40148 => Some(r#"* The `from` property is missing in the Task’s attributes.
* The `from` property in the Task’s attributes is invalid."#),
            TwilioTaskrouterError::ErrorCode40154 => Some(r#"The Task does not have a field that holds the Known Worker information, the Worker information specified is invalid, or the specified Worker does not exist in TaskRouter."#),
            TwilioTaskrouterError::ErrorCode40159 => Some(r#"Provided value for the "status_callback_url" parameter is either an empty string or of an incorrect datatype."#),
            TwilioTaskrouterError::ErrorCode40139 => Some(r#"*  The provided  `post_work_activity_sid` is invalid or no longer exists in the operating Workspace.  "#),
            TwilioTaskrouterError::ErrorCode40110 => None,
            TwilioTaskrouterError::ErrorCode40146 => Some(r#"* A Reservation has not yet been accepted by a Worker.
* The Reservation has already been canceled."#)
        }
    }
}
