// AUTO-GENERATED. DO NOT MODIFY. YOUR CHANGES WILL BE OVER-WRITTEN
use crate::products::TwilioPlatformError;
use standard_error::traits::StandardErrorFromCodeTrait;

impl StandardErrorFromCodeTrait for TwilioPlatformError {
    fn from_code(code: usize) -> Option<Self> {
        match code {
            20101 => Some(TwilioPlatformError::ErrorCode20101),
            20409 => Some(TwilioPlatformError::ErrorCode20409),
            20162 => Some(TwilioPlatformError::ErrorCode20162),
            20157 => Some(TwilioPlatformError::ErrorCode20157),
            21102 => Some(TwilioPlatformError::ErrorCode21102),
            20159 => Some(TwilioPlatformError::ErrorCode20159),
            20105 => Some(TwilioPlatformError::ErrorCode20105),
            20404 => Some(TwilioPlatformError::ErrorCode20404),
            20154 => Some(TwilioPlatformError::ErrorCode20154),
            20107 => Some(TwilioPlatformError::ErrorCode20107),
            20155 => Some(TwilioPlatformError::ErrorCode20155),
            20153 => Some(TwilioPlatformError::ErrorCode20153),
            20156 => Some(TwilioPlatformError::ErrorCode20156),
            20104 => Some(TwilioPlatformError::ErrorCode20104),
            20102 => Some(TwilioPlatformError::ErrorCode20102),
            97001 => Some(TwilioPlatformError::ErrorCode97001),
            20151 => Some(TwilioPlatformError::ErrorCode20151),
            20103 => Some(TwilioPlatformError::ErrorCode20103),
            21481 => Some(TwilioPlatformError::ErrorCode21481),
            20403 => Some(TwilioPlatformError::ErrorCode20403),
            20160 => Some(TwilioPlatformError::ErrorCode20160),
            20152 => Some(TwilioPlatformError::ErrorCode20152),
            20106 => Some(TwilioPlatformError::ErrorCode20106),
            _ => None
        }
    }
}
