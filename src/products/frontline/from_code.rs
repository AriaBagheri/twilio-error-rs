// AUTO-GENERATED. DO NOT MODIFY. YOUR CHANGES WILL BE OVER-WRITTEN
use crate::products::TwilioFrontlineError;
use standard_error::traits::StandardErrorFromCodeTrait;

impl StandardErrorFromCodeTrait for TwilioFrontlineError {
    fn from_code(code: usize) -> Option<Self> {
        match code {
            48028 => Some(TwilioFrontlineError::ErrorCode48028),
            48005 => Some(TwilioFrontlineError::ErrorCode48005),
            48031 => Some(TwilioFrontlineError::ErrorCode48031),
            48029 => Some(TwilioFrontlineError::ErrorCode48029),
            48011 => Some(TwilioFrontlineError::ErrorCode48011),
            48027 => Some(TwilioFrontlineError::ErrorCode48027),
            48025 => Some(TwilioFrontlineError::ErrorCode48025),
            48032 => Some(TwilioFrontlineError::ErrorCode48032),
            48030 => Some(TwilioFrontlineError::ErrorCode48030),
            48050 => Some(TwilioFrontlineError::ErrorCode48050),
            48001 => Some(TwilioFrontlineError::ErrorCode48001),
            48024 => Some(TwilioFrontlineError::ErrorCode48024),
            48010 => Some(TwilioFrontlineError::ErrorCode48010),
            48004 => Some(TwilioFrontlineError::ErrorCode48004),
            48000 => Some(TwilioFrontlineError::ErrorCode48000),
            48002 => Some(TwilioFrontlineError::ErrorCode48002),
            48026 => Some(TwilioFrontlineError::ErrorCode48026),
            48003 => Some(TwilioFrontlineError::ErrorCode48003),
            48023 => Some(TwilioFrontlineError::ErrorCode48023),
            48033 => Some(TwilioFrontlineError::ErrorCode48033),
            _ => None
        }
    }
}
