// AUTO-GENERATED. DO NOT MODIFY. YOUR CHANGES WILL BE OVER-WRITTEN
use crate::products::TwilioUnderstandError;
use standard_error::traits::StandardErrorCausesTrait;

impl StandardErrorCausesTrait for TwilioUnderstandError {
    fn causes(&self) -> Option<&'static str> {
        match self {
            TwilioUnderstandError::ErrorCode90403 => Some(r#"The account owns the messaging/voice service does not have any parent/sub-account relationship with the account owns the Autopilot assistant. "#),
            TwilioUnderstandError::ErrorCode90104 => Some(r#"Collect Question has an Invalid Field Type"#),
            TwilioUnderstandError::ErrorCode90103 => Some(r#"An internal error rendered us unable to process the answer. Apologies!"#),
            TwilioUnderstandError::ErrorCode90102 => Some(r#"An internal error rendered us unable to start the collection. Apologies!"#),
            TwilioUnderstandError::ErrorCode90101 => Some(r#"A same unique name has been created before"#),
            TwilioUnderstandError::ErrorCode90100 => Some(r#"Actions JSON does not comply with the Actions Schema (https://carnelian-neanderthal-8008.twil.io/assets/ActionsSchema.json)"#)
        }
    }
}
