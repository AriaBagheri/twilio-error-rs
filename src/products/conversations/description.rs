// AUTO-GENERATED. DO NOT MODIFY. YOUR CHANGES WILL BE OVER-WRITTEN
use crate::products::TwilioConversationsError;
use standard_error::traits::StandardErrorDescriptionTrait;

impl StandardErrorDescriptionTrait for TwilioConversationsError {
    fn description(&self) -> Option<&'static str> {
        match self {
            TwilioConversationsError::ErrorCode50707 => None,
            TwilioConversationsError::ErrorCode50426 => Some(r#"Request contains more participants than allowed per request."#),
            TwilioConversationsError::ErrorCode50534 => Some(r#"Unable to read Channel Metadata for Message"#),
            TwilioConversationsError::ErrorCode50526 => Some(r#"An invalid value was specified for the message content sid"#),
            TwilioConversationsError::ErrorCode50386 => Some(r#"When conversation state is 'initializing', conversation may not be updated or used for message sending until it reaches states 'active' or 'inactive'. 'Initializing' state appears in GMMS conversations when not all resources were yet created to start message sending. It's possible when Conversation with Participants endpoint is used for GMMS conversation creation or when GMMS conversation is autocreated by inbound sms message."#),
            TwilioConversationsError::ErrorCode50705 => None,
            TwilioConversationsError::ErrorCode50529 => Some(r#"Allowed total length for message content variables is sum of key, value and JSON-specific characters' lengths."#),
            TwilioConversationsError::ErrorCode50708 => None,
            TwilioConversationsError::ErrorCode50704 => None,
            TwilioConversationsError::ErrorCode50396 => Some(r#"EndDate parameter should be greater than StartDate parameter"#),
            TwilioConversationsError::ErrorCode50452 => Some(r#"Triggered when an Account attempts to use GMMS without the appropriate permissions"#),
            TwilioConversationsError::ErrorCode50514 => Some(r#"The requested message is already being modified in this service instance concurrently with another API request. The last request will be rejected to avoid inconsistent state."#),
            TwilioConversationsError::ErrorCode50528 => Some(r#"Message content variables are defined without a proper content sid"#),
            TwilioConversationsError::ErrorCode50542 => Some(r#"Unsupported Content Template Type."#),
            TwilioConversationsError::ErrorCode50701 => None,
            TwilioConversationsError::ErrorCode50532 => Some(r#"Unable to create Channel Metadata for the Message"#),
            TwilioConversationsError::ErrorCode50706 => None,
            TwilioConversationsError::ErrorCode50424 => Some(r#"One of the requests in provided participants list is not a valid JSON object."#),
            TwilioConversationsError::ErrorCode50700 => None,
            TwilioConversationsError::ErrorCode50703 => None,
            TwilioConversationsError::ErrorCode50527 => Some(r#"An invalid value was specified for the message content variables"#),
            TwilioConversationsError::ErrorCode50427 => Some(r#"One or more other errors occurred during multiple participants creation. This error is received in [Error logs](https://www.twilio.com/docs/messaging/guides/debugging-tools#how-to-use-the-twilio-error-logs) in Twilio Console and contains the list of other [errors](https://www.twilio.com/docs/api/errors)."#),
            TwilioConversationsError::ErrorCode50541 => Some(r#"Length of value in message content variables is restricted to 256 characters."#),
            TwilioConversationsError::ErrorCode50710 => None,
            TwilioConversationsError::ErrorCode50709 => None,
            TwilioConversationsError::ErrorCode50531 => Some(r#"User is not authorized to access the requested resource"#),
            TwilioConversationsError::ErrorCode50530 => Some(r#"Channel Metadata was not found for the Message"#),
            TwilioConversationsError::ErrorCode50533 => Some(r#"The Channel Metadata Content was not in the correct format."#),
            TwilioConversationsError::ErrorCode50540 => Some(r#"Length of key in message content variables is restricted to 16 characters."#),
            TwilioConversationsError::ErrorCode50702 => None,
            TwilioConversationsError::ErrorCode50711 => None,
            TwilioConversationsError::ErrorCode50453 => Some(r#"Triggered when account is trying to add or update the participant with a proxy address that is not supported in given channel, e.g. email, etc."#),
            TwilioConversationsError::ErrorCode50425 => Some(r#"One or more of the requested participant and proxy address pairs are already registered in the system."#)
        }
    }
}
