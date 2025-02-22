// AUTO-GENERATED. DO NOT MODIFY. YOUR CHANGES WILL BE OVER-WRITTEN
use crate::products::TwilioAiAssistantsError;
use standard_error::traits::StandardErrorCausesTrait;

impl StandardErrorCausesTrait for TwilioAiAssistantsError {
    fn causes(&self) -> Option<&'static str> {
        match self {
            TwilioAiAssistantsError::ErrorCode99001 => Some(r#"AI Assistants encountered a general service error."#),
            TwilioAiAssistantsError::ErrorCode99004 => Some(r#"No further action is required by you."#),
            TwilioAiAssistantsError::ErrorCode99002 => Some(r#"Tool execution could not be completed due to non 2xx response."#),
            TwilioAiAssistantsError::ErrorCode99006 => Some(r#"You have provided an invalid input schema. Check the Tool setting to verify the schema."#),
            TwilioAiAssistantsError::ErrorCode99003 => Some(r#"The provided prompt has failed guardrail checks."#),
            TwilioAiAssistantsError::ErrorCode99005 => Some(r#"Provided URL is invalid or restricted."#)
        }
    }
}
