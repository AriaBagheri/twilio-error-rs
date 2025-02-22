// AUTO-GENERATED. DO NOT MODIFY. YOUR CHANGES WILL BE OVER-WRITTEN
use crate::products::TwilioUnderstandError;
use standard_error::traits::StandardErrorMessageTrait;
use std::borrow::Cow;

impl StandardErrorMessageTrait for TwilioUnderstandError {
    fn message(&self) -> Cow<'static, str> {
        match self {
            TwilioUnderstandError::ErrorCode90403 => r#"[Autopilot] Signature validation failed"#.into(),
            TwilioUnderstandError::ErrorCode90104 => r#"Invalid Collect Field Type"#.into(),
            TwilioUnderstandError::ErrorCode90103 => r#"Error processing answer during collection"#.into(),
            TwilioUnderstandError::ErrorCode90102 => r#"Assistant failure to start collection"#.into(),
            TwilioUnderstandError::ErrorCode90101 => r#"Unique Name Already Exists"#.into(),
            TwilioUnderstandError::ErrorCode90100 => r#"Invalid Autopilot Actions JSON"#.into()
        }
    }
}
