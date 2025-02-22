// AUTO-GENERATED. DO NOT MODIFY. YOUR CHANGES WILL BE OVER-WRITTEN
use crate::products::TwilioEventsError;
use standard_error::traits::StandardErrorFromCodeTrait;

impl StandardErrorFromCodeTrait for TwilioEventsError {
    fn from_code(code: usize) -> Option<Self> {
        match code {
            91003 => Some(TwilioEventsError::ErrorCode91003),
            91004 => Some(TwilioEventsError::ErrorCode91004),
            93104 => Some(TwilioEventsError::ErrorCode93104),
            91101 => Some(TwilioEventsError::ErrorCode91101),
            91007 => Some(TwilioEventsError::ErrorCode91007),
            91006 => Some(TwilioEventsError::ErrorCode91006),
            91005 => Some(TwilioEventsError::ErrorCode91005),
            91000 => Some(TwilioEventsError::ErrorCode91000),
            93101 => Some(TwilioEventsError::ErrorCode93101),
            93103 => Some(TwilioEventsError::ErrorCode93103),
            91102 => Some(TwilioEventsError::ErrorCode91102),
            91104 => Some(TwilioEventsError::ErrorCode91104),
            91103 => Some(TwilioEventsError::ErrorCode91103),
            91001 => Some(TwilioEventsError::ErrorCode91001),
            91201 => Some(TwilioEventsError::ErrorCode91201),
            91002 => Some(TwilioEventsError::ErrorCode91002),
            93100 => Some(TwilioEventsError::ErrorCode93100),
            91100 => Some(TwilioEventsError::ErrorCode91100),
            93102 => Some(TwilioEventsError::ErrorCode93102),
            _ => None
        }
    }
}
