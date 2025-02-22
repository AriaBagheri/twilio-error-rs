// AUTO-GENERATED. DO NOT MODIFY. YOUR CHANGES WILL BE OVER-WRITTEN
use crate::products::TwilioChannelsError;
use standard_error::traits::StandardErrorDescriptionTrait;

impl StandardErrorDescriptionTrait for TwilioChannelsError {
    fn description(&self) -> Option<&'static str> {
        match self {
            TwilioChannelsError::ErrorCode63108 => Some(r#"Sender is not ready to add test device"#),
            TwilioChannelsError::ErrorCode63106 => Some(r#"The handset associated with the phone number provided is not RCS capable"#),
            TwilioChannelsError::ErrorCode63103 => Some(r#"The WhatsApp Business Account (WABA) provided is unable to use Twilio's credit line. Due to technical limitations within Meta's systems, once a payment method is added to a WABA it can only be revoked but never fully removed."#),
            TwilioChannelsError::ErrorCode63107 => Some(r#"phone_number provided is not a valid E.164 formatted phone number"#),
            TwilioChannelsError::ErrorCode63101 => Some(r#"Twilio cannot access the WhatsApp Business Account (WABA) with the ID provided."#),
            TwilioChannelsError::ErrorCode63105 => Some(r#"Channel does not currently support the requested action"#),
            TwilioChannelsError::ErrorCode63112 => Some(r#"The Meta and/or WhatsApp Business Accounts connected to this Sender were disabled by Meta or your business verification request is currently pending. To proceed with using WhatsApp Business API features, you must complete the business verification process. Visit your business settings to initiate or resolve the pending business verification request."#),
            TwilioChannelsError::ErrorCode63109 => Some(r#"This Sender has been migrated into a different account."#),
            TwilioChannelsError::ErrorCode63100 => Some(r#"The request body is either empty, not valid JSON, or failed other validation rules. Please refer to the response's message parameter for additional guidance. "#),
            TwilioChannelsError::ErrorCode63111 => Some(r#"Meta returned "not found" for the Sender's phone number or WABA.
"#),
            TwilioChannelsError::ErrorCode63102 => Some(r#"The Twilio Account contains other WhatsApp Senders and is already linked to an existing WhatsApp Business Account (WABA). Twilio Accounts must be linked to only one WABA ID at a time."#)
        }
    }
}
