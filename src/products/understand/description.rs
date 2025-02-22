// AUTO-GENERATED. DO NOT MODIFY. YOUR CHANGES WILL BE OVER-WRITTEN
use crate::products::TwilioUnderstandError;
use standard_error::traits::StandardErrorDescriptionTrait;

impl StandardErrorDescriptionTrait for TwilioUnderstandError {
    fn description(&self) -> Option<&'static str> {
        match self {
            TwilioUnderstandError::ErrorCode90403 => Some(r#"Signature validation is failed. "#),
            TwilioUnderstandError::ErrorCode90104 => Some(r#"The Field Type in Collect JSON is Invalid"#),
            TwilioUnderstandError::ErrorCode90103 => Some(r#"Error processing answer during collection"#),
            TwilioUnderstandError::ErrorCode90102 => Some(r#"Autopilot was unable to start the collection"#),
            TwilioUnderstandError::ErrorCode90101 => Some(r#"Unique name user created already exists"#),
            TwilioUnderstandError::ErrorCode90100 => Some(r#"Autopilot Actions Configured on the Task are Invalid"#)
        }
    }
}
