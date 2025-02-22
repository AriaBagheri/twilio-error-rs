// AUTO-GENERATED. DO NOT MODIFY. YOUR CHANGES WILL BE OVER-WRITTEN
use crate::products::TwilioLookupError;
use standard_error::traits::StandardErrorFromCodeTrait;

impl StandardErrorFromCodeTrait for TwilioLookupError {
    fn from_code(code: usize) -> Option<Self> {
        match code {
            60616 => Some(TwilioLookupError::ErrorCode60616),
            60699 => Some(TwilioLookupError::ErrorCode60699),
            60606 => Some(TwilioLookupError::ErrorCode60606),
            60620 => Some(TwilioLookupError::ErrorCode60620),
            60614 => Some(TwilioLookupError::ErrorCode60614),
            60624 => Some(TwilioLookupError::ErrorCode60624),
            60611 => Some(TwilioLookupError::ErrorCode60611),
            60619 => Some(TwilioLookupError::ErrorCode60619),
            60612 => Some(TwilioLookupError::ErrorCode60612),
            60621 => Some(TwilioLookupError::ErrorCode60621),
            60609 => Some(TwilioLookupError::ErrorCode60609),
            60607 => Some(TwilioLookupError::ErrorCode60607),
            60618 => Some(TwilioLookupError::ErrorCode60618),
            60601 => Some(TwilioLookupError::ErrorCode60601),
            60610 => Some(TwilioLookupError::ErrorCode60610),
            60608 => Some(TwilioLookupError::ErrorCode60608),
            60623 => Some(TwilioLookupError::ErrorCode60623),
            60617 => Some(TwilioLookupError::ErrorCode60617),
            60613 => Some(TwilioLookupError::ErrorCode60613),
            60622 => Some(TwilioLookupError::ErrorCode60622),
            60600 => Some(TwilioLookupError::ErrorCode60600),
            _ => None
        }
    }
}
