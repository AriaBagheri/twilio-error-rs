// AUTO-GENERATED. DO NOT MODIFY. YOUR CHANGES WILL BE OVER-WRITTEN
use crate::products::TwilioStudioError;
use standard_error::traits::StandardErrorFromCodeTrait;

impl StandardErrorFromCodeTrait for TwilioStudioError {
    fn from_code(code: usize) -> Option<Self> {
        match code {
            81002 => Some(TwilioStudioError::ErrorCode81002),
            81026 => Some(TwilioStudioError::ErrorCode81026),
            81007 => Some(TwilioStudioError::ErrorCode81007),
            81027 => Some(TwilioStudioError::ErrorCode81027),
            81014 => Some(TwilioStudioError::ErrorCode81014),
            81012 => Some(TwilioStudioError::ErrorCode81012),
            81019 => Some(TwilioStudioError::ErrorCode81019),
            81025 => Some(TwilioStudioError::ErrorCode81025),
            81017 => Some(TwilioStudioError::ErrorCode81017),
            81006 => Some(TwilioStudioError::ErrorCode81006),
            81005 => Some(TwilioStudioError::ErrorCode81005),
            81016 => Some(TwilioStudioError::ErrorCode81016),
            81024 => Some(TwilioStudioError::ErrorCode81024),
            81004 => Some(TwilioStudioError::ErrorCode81004),
            81008 => Some(TwilioStudioError::ErrorCode81008),
            81013 => Some(TwilioStudioError::ErrorCode81013),
            81018 => Some(TwilioStudioError::ErrorCode81018),
            81021 => Some(TwilioStudioError::ErrorCode81021),
            81022 => Some(TwilioStudioError::ErrorCode81022),
            81001 => Some(TwilioStudioError::ErrorCode81001),
            81000 => Some(TwilioStudioError::ErrorCode81000),
            81009 => Some(TwilioStudioError::ErrorCode81009),
            81010 => Some(TwilioStudioError::ErrorCode81010),
            81015 => Some(TwilioStudioError::ErrorCode81015),
            81020 => Some(TwilioStudioError::ErrorCode81020),
            81023 => Some(TwilioStudioError::ErrorCode81023),
            81011 => Some(TwilioStudioError::ErrorCode81011),
            _ => None
        }
    }
}
