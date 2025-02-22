// AUTO-GENERATED. DO NOT MODIFY. YOUR CHANGES WILL BE OVER-WRITTEN
use crate::products::TwilioInterconnectError;
use standard_error::traits::StandardErrorFromCodeTrait;

impl StandardErrorFromCodeTrait for TwilioInterconnectError {
    fn from_code(code: usize) -> Option<Self> {
        match code {
            62034 => Some(TwilioInterconnectError::ErrorCode62034),
            62015 => Some(TwilioInterconnectError::ErrorCode62015),
            32303 => Some(TwilioInterconnectError::ErrorCode32303),
            32301 => Some(TwilioInterconnectError::ErrorCode32301),
            62028 => Some(TwilioInterconnectError::ErrorCode62028),
            62200 => Some(TwilioInterconnectError::ErrorCode62200),
            62001 => Some(TwilioInterconnectError::ErrorCode62001),
            62005 => Some(TwilioInterconnectError::ErrorCode62005),
            62017 => Some(TwilioInterconnectError::ErrorCode62017),
            62006 => Some(TwilioInterconnectError::ErrorCode62006),
            62003 => Some(TwilioInterconnectError::ErrorCode62003),
            62025 => Some(TwilioInterconnectError::ErrorCode62025),
            62012 => Some(TwilioInterconnectError::ErrorCode62012),
            62053 => Some(TwilioInterconnectError::ErrorCode62053),
            62020 => Some(TwilioInterconnectError::ErrorCode62020),
            62021 => Some(TwilioInterconnectError::ErrorCode62021),
            62009 => Some(TwilioInterconnectError::ErrorCode62009),
            62024 => Some(TwilioInterconnectError::ErrorCode62024),
            62027 => Some(TwilioInterconnectError::ErrorCode62027),
            62008 => Some(TwilioInterconnectError::ErrorCode62008),
            62100 => Some(TwilioInterconnectError::ErrorCode62100),
            62007 => Some(TwilioInterconnectError::ErrorCode62007),
            62019 => Some(TwilioInterconnectError::ErrorCode62019),
            32304 => Some(TwilioInterconnectError::ErrorCode32304),
            62014 => Some(TwilioInterconnectError::ErrorCode62014),
            62010 => Some(TwilioInterconnectError::ErrorCode62010),
            62002 => Some(TwilioInterconnectError::ErrorCode62002),
            62004 => Some(TwilioInterconnectError::ErrorCode62004),
            62026 => Some(TwilioInterconnectError::ErrorCode62026),
            62023 => Some(TwilioInterconnectError::ErrorCode62023),
            62018 => Some(TwilioInterconnectError::ErrorCode62018),
            62000 => Some(TwilioInterconnectError::ErrorCode62000),
            62013 => Some(TwilioInterconnectError::ErrorCode62013),
            62035 => Some(TwilioInterconnectError::ErrorCode62035),
            62022 => Some(TwilioInterconnectError::ErrorCode62022),
            62016 => Some(TwilioInterconnectError::ErrorCode62016),
            62052 => Some(TwilioInterconnectError::ErrorCode62052),
            62011 => Some(TwilioInterconnectError::ErrorCode62011),
            32302 => Some(TwilioInterconnectError::ErrorCode32302),
            62220 => Some(TwilioInterconnectError::ErrorCode62220),
            _ => None
        }
    }
}
