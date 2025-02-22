// AUTO-GENERATED. DO NOT MODIFY. YOUR CHANGES WILL BE OVER-WRITTEN
use crate::products::TwilioNotifyError;
use standard_error::traits::StandardErrorFromCodeTrait;

impl StandardErrorFromCodeTrait for TwilioNotifyError {
    fn from_code(code: usize) -> Option<Self> {
        match code {
            52402 => Some(TwilioNotifyError::ErrorCode52402),
            52117 => Some(TwilioNotifyError::ErrorCode52117),
            52311 => Some(TwilioNotifyError::ErrorCode52311),
            52310 => Some(TwilioNotifyError::ErrorCode52310),
            52403 => Some(TwilioNotifyError::ErrorCode52403),
            52115 => Some(TwilioNotifyError::ErrorCode52115),
            52404 => Some(TwilioNotifyError::ErrorCode52404),
            52400 => Some(TwilioNotifyError::ErrorCode52400),
            _ => None
        }
    }
}
