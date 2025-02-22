// AUTO-GENERATED. DO NOT MODIFY. YOUR CHANGES WILL BE OVER-WRITTEN
use crate::products::TwilioNotifyError;
use standard_error::traits::StandardErrorCodeTrait;

impl StandardErrorCodeTrait for TwilioNotifyError {
    fn code(&self) -> usize {
        match self {
            TwilioNotifyError::ErrorCode52402 => 52402,
            TwilioNotifyError::ErrorCode52117 => 52117,
            TwilioNotifyError::ErrorCode52311 => 52311,
            TwilioNotifyError::ErrorCode52310 => 52310,
            TwilioNotifyError::ErrorCode52403 => 52403,
            TwilioNotifyError::ErrorCode52115 => 52115,
            TwilioNotifyError::ErrorCode52404 => 52404,
            TwilioNotifyError::ErrorCode52400 => 52400
        }
    }
}
