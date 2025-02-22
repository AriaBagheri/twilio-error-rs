// AUTO-GENERATED. DO NOT MODIFY. YOUR CHANGES WILL BE OVER-WRITTEN
use crate::products::TwilioConversationsError;
use standard_error::traits::StandardErrorCausesTrait;

impl StandardErrorCausesTrait for TwilioConversationsError {
    fn causes(&self) -> Option<&'static str> {
        match self {
            TwilioConversationsError::ErrorCode50707 => Some(r#"You are trying to create virtual agent participant for account that is not allowed to use it."#),
            TwilioConversationsError::ErrorCode50426 => Some(r#"You are attempting to add more participants than it's allowed to in one request of conversation with participants creation."#),
            TwilioConversationsError::ErrorCode50534 => Some(r#"* Channel Metadata was in an invalid format
* Channel Metadata required keys were missing
* Error querying for Channel Metadata
* Confirm the channel type provides Channel Metadata"#),
            TwilioConversationsError::ErrorCode50526 => Some(r#"Content SID does not match the pattern HX[0-9a-f]{32}"#),
            TwilioConversationsError::ErrorCode50386 => Some(r#"Participants of GMMS conversation are not yet created in background."#),
            TwilioConversationsError::ErrorCode50705 => Some(r#"You are trying to specify a different field value for ChatbotConfiguration.ModuleSid."#),
            TwilioConversationsError::ErrorCode50529 => Some(r#"Either too many variables are defined (max 100 in total), or too long keys (max 16 characters per variable), or too long values (max 256 characters per variable)."#),
            TwilioConversationsError::ErrorCode50708 => Some(r#"You are trying to create Virtual Agent participant without ChatbotConfiguration.FriendlyName parameter."#),
            TwilioConversationsError::ErrorCode50704 => Some(r#"You are trying to specify different field value for ChatbotConfiguration.AddonSid."#),
            TwilioConversationsError::ErrorCode50396 => Some(r#"EndDate parameter is before StartDate parameter"#),
            TwilioConversationsError::ErrorCode50452 => Some(r#"Missing permissions for GMMS"#),
            TwilioConversationsError::ErrorCode50514 => Some(r#"* You might be submitting multiple concurrent requests to modify a message or its attributes with the same SID in the service instance."#),
            TwilioConversationsError::ErrorCode50528 => Some(r#"Content variables are present but there is no content sid defined"#),
            TwilioConversationsError::ErrorCode50542 => Some(r#"You attempted to send a message using a content template type that is not yet supported in Conversations API."#),
            TwilioConversationsError::ErrorCode50701 => Some(r#"There is already a Virtual Agent participant in the conversation."#),
            TwilioConversationsError::ErrorCode50532 => Some(r#"* Channel Metadata was in an invalid format
* Channel Metadata required keys were missing
* A persistence error occured"#),
            TwilioConversationsError::ErrorCode50706 => Some(r#"You are trying to edit Virtual Agent participant information."#),
            TwilioConversationsError::ErrorCode50424 => Some(r#"You are attempting to use invalid JSON object in the list of requests to create participants. If Conversations API is used it means that one of form parameters 'Participant' contains invalid JSON."#),
            TwilioConversationsError::ErrorCode50700 => Some(r#"Provided Virtual Agent provider doesn't match the list of available providers for conversations or has different case"#),
            TwilioConversationsError::ErrorCode50703 => Some(r#"Either ChatbotConfiguration.ModuleSid or ChatbotConfiguration.ModuleSid are not specified for DialogflowCX Virtual Agent participant."#),
            TwilioConversationsError::ErrorCode50527 => Some(r#"Content variables value is invalid JSON object"#),
            TwilioConversationsError::ErrorCode50427 => Some(r#"The error during multiple participants creation is unexpected due to extensive validation done before conversation with multiple participants creation. The error may appear if there are simultaneous conflicting requests. To learn more about the possible issues that happened during participants creation read the description of the received error in [Twilio Console Error Logs](https://console.twilio.com/us1/monitor/logs/debugger/errors) and check the errors. The Possible Causes of these errors caused the failure of the whole participants creation process. "#),
            TwilioConversationsError::ErrorCode50541 => Some(r#"One or many values in key-value pairs of message content variables are hitting limit of allowed 256 characters."#),
            TwilioConversationsError::ErrorCode50710 => Some(r#"You are trying to create Virtual Agent participant with ChatbotConfiguration.InitialContext which is longer than 32768 characters."#),
            TwilioConversationsError::ErrorCode50709 => Some(r#"You are trying to create Virtual Agent participant with ChatbotConfiguration.FriendlyName which is longer than 256 characters."#),
            TwilioConversationsError::ErrorCode50531 => Some(r#"User making the request does not have the appropriate permission:  readChannelMetadata"#),
            TwilioConversationsError::ErrorCode50530 => Some(r#"* Channel Metadata was not found for this message
* Channel Metadata is only relevant for select channel types "#),
            TwilioConversationsError::ErrorCode50533 => Some(r#"* Channel Metadata Json was expected.
* Channel Metadata required keys were missing"#),
            TwilioConversationsError::ErrorCode50540 => Some(r#"One or many keys in key-value pairs of message content variables are hitting limit of allowed 16 characters."#),
            TwilioConversationsError::ErrorCode50702 => Some(r#"Virtual Agent participant should have only ChatbotConfiguration prefixed fields. Other fields are not available for Virtual Agent participants."#),
            TwilioConversationsError::ErrorCode50711 => Some(r#"You are trying to create Virtual Agent participant with ChatbotConfiguration.FriendlyName as "system"."#),
            TwilioConversationsError::ErrorCode50453 => Some(r#"Proxy Address for participant is not supported in given channel"#),
            TwilioConversationsError::ErrorCode50425 => Some(r#"You are attempting to map a non-Chat (e.g. SMS) conversation participants with participant address and proxy address that are already assigned to an active conversation. There may be only one unique mapping between your conversation participant's public phone number and internal (proxying) Twilio phone number, in order to route inbound messages correctly."#)
        }
    }
}
