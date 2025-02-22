// AUTO-GENERATED. DO NOT MODIFY. YOUR CHANGES WILL BE OVER-WRITTEN
use crate::products::TwilioAiAssistantsError;
use standard_error::traits::StandardErrorMessageTrait;
use std::borrow::Cow;

impl StandardErrorMessageTrait for TwilioAiAssistantsError {
    fn message(&self) -> Cow<'static, str> {
        match self {
            TwilioAiAssistantsError::ErrorCode99001 => r#"General Service Error"#.into(),
            TwilioAiAssistantsError::ErrorCode99004 => r#"Guardrails response check failed"#.into(),
            TwilioAiAssistantsError::ErrorCode99002 => r#"Tool Execution Error"#.into(),
            TwilioAiAssistantsError::ErrorCode99006 => r#"Tool input schema is invalid."#.into(),
            TwilioAiAssistantsError::ErrorCode99003 => r#"Guardrails input check failed"#.into(),
            TwilioAiAssistantsError::ErrorCode99005 => r#"Invalid or Unresolved External Tool Endpoint"#.into()
        }
    }
}
