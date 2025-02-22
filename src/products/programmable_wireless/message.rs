// AUTO-GENERATED. DO NOT MODIFY. YOUR CHANGES WILL BE OVER-WRITTEN
use crate::products::TwilioProgrammableWirelessError;
use standard_error::traits::StandardErrorMessageTrait;
use std::borrow::Cow;

impl StandardErrorMessageTrait for TwilioProgrammableWirelessError {
    fn message(&self) -> Cow<'static, str> {
        match self {
            TwilioProgrammableWirelessError::ErrorCode33010 => r#"Conflicting update"#.into(),
            TwilioProgrammableWirelessError::ErrorCode33101 => r#"Invalid Parameter Value	"#.into(),
            TwilioProgrammableWirelessError::ErrorCode33004 => r#"Service is unavailable	"#.into(),
            TwilioProgrammableWirelessError::ErrorCode33102 => r#"Parameter missing"#.into(),
            TwilioProgrammableWirelessError::ErrorCode33119 => r#"SIM connectivity reset not allowed"#.into(),
            TwilioProgrammableWirelessError::ErrorCode33108 => r#"Rate Plan not found"#.into(),
            TwilioProgrammableWirelessError::ErrorCode33201 => r#"Unauthorized"#.into(),
            TwilioProgrammableWirelessError::ErrorCode33111 => r#"Command exceeded max length"#.into(),
            TwilioProgrammableWirelessError::ErrorCode33107 => r#"SIM not found"#.into(),
            TwilioProgrammableWirelessError::ErrorCode33000 => r#"Generic Error"#.into(),
            TwilioProgrammableWirelessError::ErrorCode33105 => r#"Transition invalid"#.into(),
            TwilioProgrammableWirelessError::ErrorCode33122 => r#"Rate Plan Is Not Allowed"#.into(),
            TwilioProgrammableWirelessError::ErrorCode33103 => r#"Paging information invalid"#.into(),
            TwilioProgrammableWirelessError::ErrorCode33120 => r#"SIM connectivity reset in progress"#.into(),
            TwilioProgrammableWirelessError::ErrorCode33104 => r#"Configuration incomplete"#.into(),
            TwilioProgrammableWirelessError::ErrorCode33203 => r#"Messaging not allowed"#.into(),
            TwilioProgrammableWirelessError::ErrorCode33121 => r#"Invalid Parameter Combination"#.into(),
            TwilioProgrammableWirelessError::ErrorCode33118 => r#"Number of Commands exceeded"#.into()
        }
    }
}
