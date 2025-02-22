// AUTO-GENERATED. DO NOT MODIFY. YOUR CHANGES WILL BE OVER-WRITTEN
use crate::products::TwilioFrontlineError;
use standard_error::traits::StandardErrorCausesTrait;

impl StandardErrorCausesTrait for TwilioFrontlineError {
    fn causes(&self) -> Option<&'static str> {
        match self {
            TwilioFrontlineError::ErrorCode48028 => Some(r#"* The outgoing conversation callback returned a Twilio proxy number that was created under a different Twilio account."#),
            TwilioFrontlineError::ErrorCode48005 => Some(r#"Your Frontline Integration Service side processing is exceeding 6 seconds."#),
            TwilioFrontlineError::ErrorCode48031 => Some(r#"* Another Frontline user is already engaged in the conversation with the same contact using the same Twilio proxy number.
* A conversation was created with the same contact using the same Twilio proxy number, but a Frontline user was never added to the conversation (i.e. the conversation was never routed to a Frontline user)."#),
            TwilioFrontlineError::ErrorCode48029 => Some(r#"* The outgoing conversation callback returned a Twilio proxy number that does not match the channel type of the contact address."#),
            TwilioFrontlineError::ErrorCode48011 => Some(r#"- We failed to execute callback request."#),
            TwilioFrontlineError::ErrorCode48027 => Some(r#"* The outgoing conversation callback returned a Twilio proxy number that was the same as the contact address."#),
            TwilioFrontlineError::ErrorCode48025 => Some(r#"* The contact phone number used to initiate a new conversation was invalid."#),
            TwilioFrontlineError::ErrorCode48032 => Some(r#"* You may have deleted the Messaging service that auto-creates conversations in the Frontline Conversations service."#),
            TwilioFrontlineError::ErrorCode48030 => Some(r#"* The Outgoing Conversation callback returned a Twilio proxy number that was properly formatted, but not configured as a WhatsApp-enabled sender."#),
            TwilioFrontlineError::ErrorCode48050 => Some(r#"* Frontline encountered an internal error."#),
            TwilioFrontlineError::ErrorCode48001 => Some(r#"Callback URL is not set in Twilio Frontline Console."#),
            TwilioFrontlineError::ErrorCode48024 => Some(r#"* The Conversations user associated to the given contact attempted to create a conversation over Chat channel and exceeded the Conversations service’s maximum user conversations limit."#),
            TwilioFrontlineError::ErrorCode48010 => Some(r#"- The callback endpoint returned an error code.
- The callback processing is exceeding 10 seconds."#),
            TwilioFrontlineError::ErrorCode48004 => Some(r#"Your callback returned an error response, the Callback URL provided might be wrong or callback endpoint might be failing to process request."#),
            TwilioFrontlineError::ErrorCode48000 => Some(r#"- You might be using an old version of Frontline application."#),
            TwilioFrontlineError::ErrorCode48002 => Some(r#"The Callback URL provided is invalid."#),
            TwilioFrontlineError::ErrorCode48026 => Some(r#"* The outgoing conversation callback returned an invalid Twilio proxy number or no number at all."#),
            TwilioFrontlineError::ErrorCode48003 => Some(r#"* Your Frontline integration service returned a response which is invalid.
* Your service returned a response that was too large. Responses are capped at 64k."#),
            TwilioFrontlineError::ErrorCode48023 => Some(r#"* The Conversations user associated to the given Frontline user attempted to create a conversation and exceeded the Conversations service’s maximum user conversations limit."#),
            TwilioFrontlineError::ErrorCode48033 => Some(r#"* The contact chat identity was not formatted properly.
* A reserved identity was used for contact chat identity."#)
        }
    }
}
