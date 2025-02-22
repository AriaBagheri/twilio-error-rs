// AUTO-GENERATED. DO NOT MODIFY. YOUR CHANGES WILL BE OVER-WRITTEN
use crate::products::TwilioTaskrouterError;
use standard_error::traits::StandardErrorDescriptionTrait;

impl StandardErrorDescriptionTrait for TwilioTaskrouterError {
    fn description(&self) -> Option<&'static str> {
        match self {
            TwilioTaskrouterError::ErrorCode40114 => None,
            TwilioTaskrouterError::ErrorCode40121 => None,
            TwilioTaskrouterError::ErrorCode14233 => None,
            TwilioTaskrouterError::ErrorCode40142 => None,
            TwilioTaskrouterError::ErrorCode14236 => None,
            TwilioTaskrouterError::ErrorCode40143 => None,
            TwilioTaskrouterError::ErrorCode40001 => Some(r#" 
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

"#),
            TwilioTaskrouterError::ErrorCode40120 => None,
            TwilioTaskrouterError::ErrorCode40130 => None,
            TwilioTaskrouterError::ErrorCode40138 => Some(r#"A valid 'from' parameter was not passed to the 'conference' instruction"#),
            TwilioTaskrouterError::ErrorCode40135 => None,
            TwilioTaskrouterError::ErrorCode40152 => Some(r#"Worker isn't part of the queue specified in the Workflow target for re-targeted Task."#),
            TwilioTaskrouterError::ErrorCode40005 => Some(r#"Assignment Callback response does not contain 'instruction'

"#),
            TwilioTaskrouterError::ErrorCode14238 => None,
            TwilioTaskrouterError::ErrorCode40155 => Some(r#"Worker is not part of the workspace referenced by the Workflow target"#),
            TwilioTaskrouterError::ErrorCode40140 => None,
            TwilioTaskrouterError::ErrorCode14230 => None,
            TwilioTaskrouterError::ErrorCode14217 => None,
            TwilioTaskrouterError::ErrorCode40158 => Some(r#"Invalid 'status_callback_url' parameter - Call Instruction"#),
            TwilioTaskrouterError::ErrorCode40157 => Some(r#"Invalid 'status_callback_url' parameter - Dequeue Instruction"#),
            TwilioTaskrouterError::ErrorCode40137 => Some(r#"A valid 'to' parameter must be provided when using the Conference Instruction

"#),
            TwilioTaskrouterError::ErrorCode40147 => Some(r#" "#),
            TwilioTaskrouterError::ErrorCode40151 => None,
            TwilioTaskrouterError::ErrorCode40134 => None,
            TwilioTaskrouterError::ErrorCode14239 => None,
            TwilioTaskrouterError::ErrorCode40000 => None,
            TwilioTaskrouterError::ErrorCode40123 => None,
            TwilioTaskrouterError::ErrorCode14222 => None,
            TwilioTaskrouterError::ErrorCode14240 => Some(r#"Max concurrent Workers exceeded"#),
            TwilioTaskrouterError::ErrorCode14235 => None,
            TwilioTaskrouterError::ErrorCode14234 => None,
            TwilioTaskrouterError::ErrorCode14237 => None,
            TwilioTaskrouterError::ErrorCode40111 => None,
            TwilioTaskrouterError::ErrorCode14241 => Some(r#"Starting Nov. 30, 2021 we will no longer return data older than 30 days, and instead return a 400-response for requests with <start_date> older than 30 days."#),
            TwilioTaskrouterError::ErrorCode40144 => None,
            TwilioTaskrouterError::ErrorCode40100 => Some(r#"&nbsp;"#),
            TwilioTaskrouterError::ErrorCode40149 => Some(r#"The conference instruction issued as a result of the transfer can only be issued on a task which has a `conference_sid` in its attributes."#),
            TwilioTaskrouterError::ErrorCode40131 => None,
            TwilioTaskrouterError::ErrorCode40141 => None,
            TwilioTaskrouterError::ErrorCode14231 => None,
            TwilioTaskrouterError::ErrorCode55555 => Some(r#"An invalid instruction was passed to TaskRouter when responding to a pending Reservation."#),
            TwilioTaskrouterError::ErrorCode40153 => Some(r#"The absolute paging query type that is being used has been deprecated. Starting Dec. 1, 2021 we will no longer return data for this query, and instead return a 400-response for requests of query type /Workers?Page=<x>&PageSize=<y>.

Please update your application to instead utilize relative paging: /Workers?PageToken=PA<task_sid>, where <task_sid> is the last item of the current page."#),
            TwilioTaskrouterError::ErrorCode14232 => None,
            TwilioTaskrouterError::ErrorCode40145 => None,
            TwilioTaskrouterError::ErrorCode40136 => None,
            TwilioTaskrouterError::ErrorCode40132 => None,
            TwilioTaskrouterError::ErrorCode40112 => None,
            TwilioTaskrouterError::ErrorCode40122 => None,
            TwilioTaskrouterError::ErrorCode40113 => None,
            TwilioTaskrouterError::ErrorCode40133 => None,
            TwilioTaskrouterError::ErrorCode40148 => Some(r#"A valid `from` parameter was not passed to the ‘supervise’ instruction"#),
            TwilioTaskrouterError::ErrorCode40154 => Some(r#"Worker information is missing or not valid."#),
            TwilioTaskrouterError::ErrorCode40159 => Some(r#"Invalid 'status_callback_url' parameter - Conference Instruction"#),
            TwilioTaskrouterError::ErrorCode40139 => Some(r#"Conference Instruction requires a valid `post_work_activity_sid`"#),
            TwilioTaskrouterError::ErrorCode40110 => None,
            TwilioTaskrouterError::ErrorCode40146 => Some(r#"The Reservation is not in a valid state for the Supervise instruction to be used, the Reservation cannot be pending or canceled."#)
        }
    }
}
