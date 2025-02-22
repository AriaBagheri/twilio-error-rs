// AUTO-GENERATED. DO NOT MODIFY. YOUR CHANGES WILL BE OVER-WRITTEN
use crate::products::TwilioFunctionsError;
use standard_error::traits::StandardErrorFromCodeTrait;

impl StandardErrorFromCodeTrait for TwilioFunctionsError {
    fn from_code(code: usize) -> Option<Self> {
        match code {
            82001 => Some(TwilioFunctionsError::ErrorCode82001),
            82009 => Some(TwilioFunctionsError::ErrorCode82009),
            82006 => Some(TwilioFunctionsError::ErrorCode82006),
            82005 => Some(TwilioFunctionsError::ErrorCode82005),
            82002 => Some(TwilioFunctionsError::ErrorCode82002),
            82004 => Some(TwilioFunctionsError::ErrorCode82004),
            82007 => Some(TwilioFunctionsError::ErrorCode82007),
            82008 => Some(TwilioFunctionsError::ErrorCode82008),
            82003 => Some(TwilioFunctionsError::ErrorCode82003),
            _ => None
        }
    }
}
