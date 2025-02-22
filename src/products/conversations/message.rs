// AUTO-GENERATED. DO NOT MODIFY. YOUR CHANGES WILL BE OVER-WRITTEN
use crate::products::TwilioConversationsError;
use standard_error::traits::StandardErrorMessageTrait;
use std::borrow::Cow;

impl StandardErrorMessageTrait for TwilioConversationsError {
    fn message(&self) -> Cow<'static, str> {
        match self {
            TwilioConversationsError::ErrorCode50707 => r#"Virtual Agent participant creation is not available for your account."#.into(),
            TwilioConversationsError::ErrorCode50426 => r#"Request contains too many participants."#.into(),
            TwilioConversationsError::ErrorCode50534 => r#"Failed to Read ChannelMetadata"#.into(),
            TwilioConversationsError::ErrorCode50526 => r#"Invalid content SID"#.into(),
            TwilioConversationsError::ErrorCode50386 => r#"Conversation in 'initializing' state may not be updated or used for message sending"#.into(),
            TwilioConversationsError::ErrorCode50705 => r#"Provided module sid should have prefix XE."#.into(),
            TwilioConversationsError::ErrorCode50529 => r#"Content variables are too long"#.into(),
            TwilioConversationsError::ErrorCode50708 => r#"ChatbotConfiguration.FriendlyName is required parameter."#.into(),
            TwilioConversationsError::ErrorCode50704 => r#"Provided addon sid should have prefix XB."#.into(),
            TwilioConversationsError::ErrorCode50396 => r#"EndDate parameter should be greater than StartDate parameter"#.into(),
            TwilioConversationsError::ErrorCode50452 => r#"Group MMS is not enabled for this Account"#.into(),
            TwilioConversationsError::ErrorCode50514 => r#"Conflicting message modification"#.into(),
            TwilioConversationsError::ErrorCode50528 => r#"Content SID is missing"#.into(),
            TwilioConversationsError::ErrorCode50542 => r#"Unsupported Content Template Type"#.into(),
            TwilioConversationsError::ErrorCode50701 => r#"Only one Virtual Agent participant is allowed per conversation."#.into(),
            TwilioConversationsError::ErrorCode50532 => r#"Failed to create Channel Metadata"#.into(),
            TwilioConversationsError::ErrorCode50706 => r#"Updating Virtual Agent participant information is not allowed."#.into(),
            TwilioConversationsError::ErrorCode50424 => r#"One of the JSON requests for participant creation is invalid."#.into(),
            TwilioConversationsError::ErrorCode50700 => r#"Virtual Agent provider is invalid."#.into(),
            TwilioConversationsError::ErrorCode50703 => r#"DialogflowCX provider must contain module sid and addon sid."#.into(),
            TwilioConversationsError::ErrorCode50527 => r#"Invalid content variables format"#.into(),
            TwilioConversationsError::ErrorCode50427 => r#"Errors occurred during multiple participants creation."#.into(),
            TwilioConversationsError::ErrorCode50541 => r#"Content variable value is too long"#.into(),
            TwilioConversationsError::ErrorCode50710 => r#"ChatbotConfiguration.InitialContext is invalid."#.into(),
            TwilioConversationsError::ErrorCode50709 => r#"ChatbotConfiguration.FriendlyName is invalid."#.into(),
            TwilioConversationsError::ErrorCode50531 => r#"Not Authorized to make this request"#.into(),
            TwilioConversationsError::ErrorCode50530 => r#"Channel Metadata not found"#.into(),
            TwilioConversationsError::ErrorCode50533 => r#"Failed to parse ChannelMetadata Json"#.into(),
            TwilioConversationsError::ErrorCode50540 => r#"Content variable key is too long"#.into(),
            TwilioConversationsError::ErrorCode50702 => r#"Verify Agent participant should not have identity, user address, proxy address, projected address."#.into(),
            TwilioConversationsError::ErrorCode50711 => r#"ChatbotConfiguration.FriendlyName is reserved."#.into(),
            TwilioConversationsError::ErrorCode50453 => r#"Proxy Address of participant is not supported for this channel"#.into(),
            TwilioConversationsError::ErrorCode50425 => r#"Participant and proxy address pairs for one or more participants already in use."#.into()
        }
    }
}
