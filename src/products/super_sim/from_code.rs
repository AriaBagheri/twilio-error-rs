// AUTO-GENERATED. DO NOT MODIFY. YOUR CHANGES WILL BE OVER-WRITTEN
use crate::products::TwilioSuperSimError;
use standard_error::traits::StandardErrorFromCodeTrait;

impl StandardErrorFromCodeTrait for TwilioSuperSimError {
    fn from_code(code: usize) -> Option<Self> {
        match code {
            83602 => Some(TwilioSuperSimError::ErrorCode83602),
            83603 => Some(TwilioSuperSimError::ErrorCode83603),
            83003 => Some(TwilioSuperSimError::ErrorCode83003),
            83001 => Some(TwilioSuperSimError::ErrorCode83001),
            83002 => Some(TwilioSuperSimError::ErrorCode83002),
            83007 => Some(TwilioSuperSimError::ErrorCode83007),
            83605 => Some(TwilioSuperSimError::ErrorCode83605),
            83008 => Some(TwilioSuperSimError::ErrorCode83008),
            83010 => Some(TwilioSuperSimError::ErrorCode83010),
            83703 => Some(TwilioSuperSimError::ErrorCode83703),
            83500 => Some(TwilioSuperSimError::ErrorCode83500),
            83705 => Some(TwilioSuperSimError::ErrorCode83705),
            83005 => Some(TwilioSuperSimError::ErrorCode83005),
            83700 => Some(TwilioSuperSimError::ErrorCode83700),
            83011 => Some(TwilioSuperSimError::ErrorCode83011),
            83402 => Some(TwilioSuperSimError::ErrorCode83402),
            83000 => Some(TwilioSuperSimError::ErrorCode83000),
            83702 => Some(TwilioSuperSimError::ErrorCode83702),
            83401 => Some(TwilioSuperSimError::ErrorCode83401),
            83400 => Some(TwilioSuperSimError::ErrorCode83400),
            83004 => Some(TwilioSuperSimError::ErrorCode83004),
            83604 => Some(TwilioSuperSimError::ErrorCode83604),
            83006 => Some(TwilioSuperSimError::ErrorCode83006),
            83600 => Some(TwilioSuperSimError::ErrorCode83600),
            83601 => Some(TwilioSuperSimError::ErrorCode83601),
            83009 => Some(TwilioSuperSimError::ErrorCode83009),
            83701 => Some(TwilioSuperSimError::ErrorCode83701),
            83704 => Some(TwilioSuperSimError::ErrorCode83704),
            _ => None
        }
    }
}
