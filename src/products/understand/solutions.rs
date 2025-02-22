// AUTO-GENERATED. DO NOT MODIFY. YOUR CHANGES WILL BE OVER-WRITTEN
use crate::products::TwilioUnderstandError;
use standard_error::traits::StandardErrorSolutionsTrait;

impl StandardErrorSolutionsTrait for TwilioUnderstandError {
    fn solutions(&self) -> Option<&'static str> {
        match self {
            TwilioUnderstandError::ErrorCode90403 => Some(r#"Moving the assistant (export and import) into the service account or service account's one of the sub-accounts would solve the problem.  
"#),
            TwilioUnderstandError::ErrorCode90104 => Some(r#"Please ensure the type in the Collect Question is valid."#),
            TwilioUnderstandError::ErrorCode90103 => Some(r#"If the error persists, <a href="/help/contact">contact us</a> to figure out what has happened and how to fix it.
Note the time of the error and what you were trying to do when it occurred. Thank you!"#),
            TwilioUnderstandError::ErrorCode90102 => Some(r#"If the error persists, <a href="/help/contact">contact us</a> to figure out what has happened and how to fix it.
Note the time of the error and what you were trying to do when it occurred. Thank you!"#),
            TwilioUnderstandError::ErrorCode90101 => Some(r#"Change to a new unique name"#),
            TwilioUnderstandError::ErrorCode90100 => Some(r#"Test your JSON response against the Actions Schema (https://carnelian-neanderthal-8008.twil.io/assets/ActionsSchema.json)"#)
        }
    }
}
