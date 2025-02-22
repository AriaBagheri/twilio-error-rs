// AUTO-GENERATED. DO NOT MODIFY. YOUR CHANGES WILL BE OVER-WRITTEN
use crate::products::TwilioFunctionsError;
use standard_error::traits::StandardErrorCodeTrait;

impl StandardErrorCodeTrait for TwilioFunctionsError {
    fn code(&self) -> usize {
        match self {
            TwilioFunctionsError::ErrorCode82001 => 82001,
            TwilioFunctionsError::ErrorCode82009 => 82009,
            TwilioFunctionsError::ErrorCode82006 => 82006,
            TwilioFunctionsError::ErrorCode82005 => 82005,
            TwilioFunctionsError::ErrorCode82002 => 82002,
            TwilioFunctionsError::ErrorCode82004 => 82004,
            TwilioFunctionsError::ErrorCode82007 => 82007,
            TwilioFunctionsError::ErrorCode82008 => 82008,
            TwilioFunctionsError::ErrorCode82003 => 82003
        }
    }
}
