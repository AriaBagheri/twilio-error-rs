// AUTO-GENERATED. DO NOT MODIFY. YOUR CHANGES WILL BE OVER-WRITTEN
use crate::products::TwilioProgrammableWirelessError;
use standard_error::traits::StandardErrorFromCodeTrait;

impl StandardErrorFromCodeTrait for TwilioProgrammableWirelessError {
    fn from_code(code: usize) -> Option<Self> {
        match code {
            33010 => Some(TwilioProgrammableWirelessError::ErrorCode33010),
            33101 => Some(TwilioProgrammableWirelessError::ErrorCode33101),
            33004 => Some(TwilioProgrammableWirelessError::ErrorCode33004),
            33102 => Some(TwilioProgrammableWirelessError::ErrorCode33102),
            33119 => Some(TwilioProgrammableWirelessError::ErrorCode33119),
            33108 => Some(TwilioProgrammableWirelessError::ErrorCode33108),
            33201 => Some(TwilioProgrammableWirelessError::ErrorCode33201),
            33111 => Some(TwilioProgrammableWirelessError::ErrorCode33111),
            33107 => Some(TwilioProgrammableWirelessError::ErrorCode33107),
            33000 => Some(TwilioProgrammableWirelessError::ErrorCode33000),
            33105 => Some(TwilioProgrammableWirelessError::ErrorCode33105),
            33122 => Some(TwilioProgrammableWirelessError::ErrorCode33122),
            33103 => Some(TwilioProgrammableWirelessError::ErrorCode33103),
            33120 => Some(TwilioProgrammableWirelessError::ErrorCode33120),
            33104 => Some(TwilioProgrammableWirelessError::ErrorCode33104),
            33203 => Some(TwilioProgrammableWirelessError::ErrorCode33203),
            33121 => Some(TwilioProgrammableWirelessError::ErrorCode33121),
            33118 => Some(TwilioProgrammableWirelessError::ErrorCode33118),
            _ => None
        }
    }
}
