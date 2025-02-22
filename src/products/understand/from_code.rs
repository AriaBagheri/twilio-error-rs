// AUTO-GENERATED. DO NOT MODIFY. YOUR CHANGES WILL BE OVER-WRITTEN
use crate::products::TwilioUnderstandError;
use standard_error::traits::StandardErrorFromCodeTrait;

impl StandardErrorFromCodeTrait for TwilioUnderstandError {
    fn from_code(code: usize) -> Option<Self> {
        match code {
            90403 => Some(TwilioUnderstandError::ErrorCode90403),
            90104 => Some(TwilioUnderstandError::ErrorCode90104),
            90103 => Some(TwilioUnderstandError::ErrorCode90103),
            90102 => Some(TwilioUnderstandError::ErrorCode90102),
            90101 => Some(TwilioUnderstandError::ErrorCode90101),
            90100 => Some(TwilioUnderstandError::ErrorCode90100),
            _ => None
        }
    }
}
