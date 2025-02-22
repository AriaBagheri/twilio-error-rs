// AUTO-GENERATED. DO NOT MODIFY. YOUR CHANGES WILL BE OVER-WRITTEN
use crate::products::TwilioSyncError;
use standard_error::traits::StandardErrorFromCodeTrait;

impl StandardErrorFromCodeTrait for TwilioSyncError {
    fn from_code(code: usize) -> Option<Self> {
        match code {
            54011 => Some(TwilioSyncError::ErrorCode54011),
            54006 => Some(TwilioSyncError::ErrorCode54006),
            54201 => Some(TwilioSyncError::ErrorCode54201),
            54155 => Some(TwilioSyncError::ErrorCode54155),
            54451 => Some(TwilioSyncError::ErrorCode54451),
            54351 => Some(TwilioSyncError::ErrorCode54351),
            54156 => Some(TwilioSyncError::ErrorCode54156),
            54450 => Some(TwilioSyncError::ErrorCode54450),
            54251 => Some(TwilioSyncError::ErrorCode54251),
            54100 => Some(TwilioSyncError::ErrorCode54100),
            54003 => Some(TwilioSyncError::ErrorCode54003),
            54509 => Some(TwilioSyncError::ErrorCode54509),
            54103 => Some(TwilioSyncError::ErrorCode54103),
            54101 => Some(TwilioSyncError::ErrorCode54101),
            54507 => Some(TwilioSyncError::ErrorCode54507),
            54053 => Some(TwilioSyncError::ErrorCode54053),
            54452 => Some(TwilioSyncError::ErrorCode54452),
            54301 => Some(TwilioSyncError::ErrorCode54301),
            54009 => Some(TwilioSyncError::ErrorCode54009),
            54051 => Some(TwilioSyncError::ErrorCode54051),
            54354 => Some(TwilioSyncError::ErrorCode54354),
            54502 => Some(TwilioSyncError::ErrorCode54502),
            54453 => Some(TwilioSyncError::ErrorCode54453),
            54300 => Some(TwilioSyncError::ErrorCode54300),
            54206 => Some(TwilioSyncError::ErrorCode54206),
            54200 => Some(TwilioSyncError::ErrorCode54200),
            54302 => Some(TwilioSyncError::ErrorCode54302),
            54208 => Some(TwilioSyncError::ErrorCode54208),
            54510 => Some(TwilioSyncError::ErrorCode54510),
            54050 => Some(TwilioSyncError::ErrorCode54050),
            54151 => Some(TwilioSyncError::ErrorCode54151),
            54150 => Some(TwilioSyncError::ErrorCode54150),
            54007 => Some(TwilioSyncError::ErrorCode54007),
            54008 => Some(TwilioSyncError::ErrorCode54008),
            54209 => Some(TwilioSyncError::ErrorCode54209),
            54458 => Some(TwilioSyncError::ErrorCode54458),
            54205 => Some(TwilioSyncError::ErrorCode54205),
            54056 => Some(TwilioSyncError::ErrorCode54056),
            54010 => Some(TwilioSyncError::ErrorCode54010),
            54454 => Some(TwilioSyncError::ErrorCode54454),
            54419 => Some(TwilioSyncError::ErrorCode54419),
            54250 => Some(TwilioSyncError::ErrorCode54250),
            _ => None
        }
    }
}
