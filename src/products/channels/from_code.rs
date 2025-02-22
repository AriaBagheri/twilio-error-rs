// AUTO-GENERATED. DO NOT MODIFY. YOUR CHANGES WILL BE OVER-WRITTEN
use crate::products::TwilioChannelsError;
use standard_error::traits::StandardErrorFromCodeTrait;

impl StandardErrorFromCodeTrait for TwilioChannelsError {
    fn from_code(code: usize) -> Option<Self> {
        match code {
            63108 => Some(TwilioChannelsError::ErrorCode63108),
            63106 => Some(TwilioChannelsError::ErrorCode63106),
            63103 => Some(TwilioChannelsError::ErrorCode63103),
            63107 => Some(TwilioChannelsError::ErrorCode63107),
            63101 => Some(TwilioChannelsError::ErrorCode63101),
            63105 => Some(TwilioChannelsError::ErrorCode63105),
            63112 => Some(TwilioChannelsError::ErrorCode63112),
            63109 => Some(TwilioChannelsError::ErrorCode63109),
            63100 => Some(TwilioChannelsError::ErrorCode63100),
            63111 => Some(TwilioChannelsError::ErrorCode63111),
            63102 => Some(TwilioChannelsError::ErrorCode63102),
            _ => None
        }
    }
}
