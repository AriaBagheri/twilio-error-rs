// AUTO-GENERATED. DO NOT MODIFY. YOUR CHANGES WILL BE OVER-WRITTEN
use crate::products::TwilioContactsError;
use standard_error::traits::StandardErrorFromCodeTrait;

impl StandardErrorFromCodeTrait for TwilioContactsError {
    fn from_code(code: usize) -> Option<Self> {
        match code {
            19045 => Some(TwilioContactsError::ErrorCode19045),
            19053 => Some(TwilioContactsError::ErrorCode19053),
            19057 => Some(TwilioContactsError::ErrorCode19057),
            19056 => Some(TwilioContactsError::ErrorCode19056),
            19044 => Some(TwilioContactsError::ErrorCode19044),
            19046 => Some(TwilioContactsError::ErrorCode19046),
            19055 => Some(TwilioContactsError::ErrorCode19055),
            19048 => Some(TwilioContactsError::ErrorCode19048),
            19054 => Some(TwilioContactsError::ErrorCode19054),
            19047 => Some(TwilioContactsError::ErrorCode19047),
            19052 => Some(TwilioContactsError::ErrorCode19052),
            19043 => Some(TwilioContactsError::ErrorCode19043),
            19049 => Some(TwilioContactsError::ErrorCode19049),
            _ => None
        }
    }
}
