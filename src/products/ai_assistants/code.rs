// AUTO-GENERATED. DO NOT MODIFY. YOUR CHANGES WILL BE OVER-WRITTEN
use crate::products::TwilioAiAssistantsError;
use standard_error::traits::StandardErrorCodeTrait;

impl StandardErrorCodeTrait for TwilioAiAssistantsError {
    fn code(&self) -> usize {
        match self {
            TwilioAiAssistantsError::ErrorCode99001 => 99001,
            TwilioAiAssistantsError::ErrorCode99004 => 99004,
            TwilioAiAssistantsError::ErrorCode99002 => 99002,
            TwilioAiAssistantsError::ErrorCode99006 => 99006,
            TwilioAiAssistantsError::ErrorCode99003 => 99003,
            TwilioAiAssistantsError::ErrorCode99005 => 99005
        }
    }
}
