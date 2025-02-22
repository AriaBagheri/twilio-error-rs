// AUTO-GENERATED. DO NOT MODIFY. YOUR CHANGES WILL BE OVER-WRITTEN
use crate::products::TwilioAiAssistantsError;
use standard_error::traits::StandardErrorFromCodeTrait;

impl StandardErrorFromCodeTrait for TwilioAiAssistantsError {
    fn from_code(code: usize) -> Option<Self> {
        match code {
            99001 => Some(TwilioAiAssistantsError::ErrorCode99001),
            99004 => Some(TwilioAiAssistantsError::ErrorCode99004),
            99002 => Some(TwilioAiAssistantsError::ErrorCode99002),
            99006 => Some(TwilioAiAssistantsError::ErrorCode99006),
            99003 => Some(TwilioAiAssistantsError::ErrorCode99003),
            99005 => Some(TwilioAiAssistantsError::ErrorCode99005),
            _ => None
        }
    }
}
