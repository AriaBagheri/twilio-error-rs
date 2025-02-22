// AUTO-GENERATED. DO NOT MODIFY. YOUR CHANGES WILL BE OVER-WRITTEN
use crate::products::TwilioUnderstandError;
use standard_error::traits::StandardErrorCodeTrait;

impl StandardErrorCodeTrait for TwilioUnderstandError {
    fn code(&self) -> usize {
        match self {
            TwilioUnderstandError::ErrorCode90403 => 90403,
            TwilioUnderstandError::ErrorCode90104 => 90104,
            TwilioUnderstandError::ErrorCode90103 => 90103,
            TwilioUnderstandError::ErrorCode90102 => 90102,
            TwilioUnderstandError::ErrorCode90101 => 90101,
            TwilioUnderstandError::ErrorCode90100 => 90100
        }
    }
}
