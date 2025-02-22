// AUTO-GENERATED. DO NOT MODIFY. YOUR CHANGES WILL BE OVER-WRITTEN
use crate::products::TwilioFrontlineError;
use standard_error::traits::StandardErrorDescriptionTrait;

impl StandardErrorDescriptionTrait for TwilioFrontlineError {
    fn description(&self) -> Option<&'static str> {
        match self {
            TwilioFrontlineError::ErrorCode48028 => Some(r#"A conversation could not be created because your Twilio account is not authorized to use the provided Twilio proxy address."#),
            TwilioFrontlineError::ErrorCode48005 => None,
            TwilioFrontlineError::ErrorCode48031 => Some(r#"A conversation could not be created because the requested contact address and Twilio proxy number are already in an active conversation in the same channel."#),
            TwilioFrontlineError::ErrorCode48029 => Some(r#"A conversation could not be created because the provided Twilio proxy address does not match the contact’s expected channel type."#),
            TwilioFrontlineError::ErrorCode48011 => None,
            TwilioFrontlineError::ErrorCode48027 => Some(r#"A conversation could not be created because the provided Twilio proxy address was the same as contact address."#),
            TwilioFrontlineError::ErrorCode48025 => Some(r#"A conversation could not be created because the provided contact address was invalid."#),
            TwilioFrontlineError::ErrorCode48032 => Some(r#"A conversation could not be created because the Messaging service required for Frontline is missing."#),
            TwilioFrontlineError::ErrorCode48030 => Some(r#"A WhatsApp conversation could not be created because the provided Twilio proxy number is not a WhatsApp-enabled sender."#),
            TwilioFrontlineError::ErrorCode48050 => None,
            TwilioFrontlineError::ErrorCode48001 => None,
            TwilioFrontlineError::ErrorCode48024 => Some(r#"A Chat conversation could not be created because the Conversations user associated to the given contact is a participant in too many conversations."#),
            TwilioFrontlineError::ErrorCode48010 => None,
            TwilioFrontlineError::ErrorCode48004 => None,
            TwilioFrontlineError::ErrorCode48000 => None,
            TwilioFrontlineError::ErrorCode48002 => None,
            TwilioFrontlineError::ErrorCode48026 => Some(r#"A conversation could not be created because the provided Twilio proxy address for the Frontline user was invalid."#),
            TwilioFrontlineError::ErrorCode48003 => None,
            TwilioFrontlineError::ErrorCode48023 => Some(r#"A conversation could not be created because the Conversations user associated to the given Frontline user is a participant in too many conversations."#),
            TwilioFrontlineError::ErrorCode48033 => Some(r#"A conversation could not be created because the provided contact chat identity was invalid."#)
        }
    }
}
