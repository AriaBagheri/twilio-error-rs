// AUTO-GENERATED. DO NOT MODIFY. YOUR CHANGES WILL BE OVER-WRITTEN
use crate::products::TwilioConversationsError;
use standard_error::traits::StandardErrorSolutionsTrait;

impl StandardErrorSolutionsTrait for TwilioConversationsError {
    fn solutions(&self) -> Option<&'static str> {
        match self {
            TwilioConversationsError::ErrorCode50707 => Some(r#"* Please contact Twilio support to enable Virtual Agent participant creation for your account."#),
            TwilioConversationsError::ErrorCode50426 => Some(r#"Reduce the amount of participants in the request to the allowed limit. Add all the rest of the participants using [Participant Resource](https://www.twilio.com/docs/conversations/api/conversation-participant-resource)."#),
            TwilioConversationsError::ErrorCode50534 => Some(r#"* Retry Channel Metadata query"#),
            TwilioConversationsError::ErrorCode50526 => Some(r#"Proper message Content SID value should be provided."#),
            TwilioConversationsError::ErrorCode50386 => Some(r#"Wait till participants are created. To ensure that state is not 'initializing' wither fetch conversation till state changes to 'active' or 'inactive' or use onConversationStateUpdated webhook."#),
            TwilioConversationsError::ErrorCode50705 => Some(r#"* Double check and fix value for ChatbotConfiguration.ModuleSid. It should start with XE prefix."#),
            TwilioConversationsError::ErrorCode50529 => Some(r#"Follow the length limitations:
- variables: 100 in total;
- key: 16 characters per variable;
- value: 256 characters per variable."#),
            TwilioConversationsError::ErrorCode50708 => Some(r#"* Specify ChatbotConfiguration.FriendlyName parameter."#),
            TwilioConversationsError::ErrorCode50704 => Some(r#"* Double check and fix value for ChatbotConfiguration.AddonSid. It should start with XB prefix."#),
            TwilioConversationsError::ErrorCode50396 => Some(r#"Set EndDate parameter greater than StartDate parameter"#),
            TwilioConversationsError::ErrorCode50452 => Some(r#"Verify if GMMS allowed for this Account"#),
            TwilioConversationsError::ErrorCode50514 => Some(r#"* Implement an operation retrier and repeat the failed API requests after an interval of time, using an exponential backoff algorithm.
* Review your application logic that caused the race condition when modifying a message. Perhaps the conflicting operations are happening in a loop that could be avoided.
* Serialize your API requests that modify conversation messages. Wait until the original request completes and returns an API response, before sending any successive conversation requests."#),
            TwilioConversationsError::ErrorCode50528 => Some(r#"Message content sid should be present along with content variables"#),
            TwilioConversationsError::ErrorCode50542 => Some(r#"Do not use the given Content Template when sending messages using Conversations API."#),
            TwilioConversationsError::ErrorCode50701 => Some(r#"* Remove existing Virtual Agent participant from the conversation before adding new one."#),
            TwilioConversationsError::ErrorCode50532 => Some(r#"* Validate Channel Metadata was in the correct format
* Retry Channel Metadata creation"#),
            TwilioConversationsError::ErrorCode50706 => Some(r#"* If you want to change any fields of Virtual Agent participant you should delete it and create new one."#),
            TwilioConversationsError::ErrorCode50424 => Some(r#"Validate that all the requests in the list of participants are strings that contain valid JSON objects. Make amendments to the requests that are not valid."#),
            TwilioConversationsError::ErrorCode50700 => Some(r#"* Check list of available Virtual Agent providers in Conversations documentation. Please note that values are case sensitive."#),
            TwilioConversationsError::ErrorCode50703 => Some(r#"* If you try to create DialogflowCX provider, you must specify ChatbotConfiguration.ModuleSid and ChatbotConfiguration.ModuleSid."#),
            TwilioConversationsError::ErrorCode50527 => Some(r#"Message content variables value should be a valid JSON object."#),
            TwilioConversationsError::ErrorCode50427 => Some(r#"It's recommended to retry the request after some time. In case of this error the failed conversation is moved to closed state and all the resources occupied by this conversation are released, that allows retry not to fail."#),
            TwilioConversationsError::ErrorCode50541 => Some(r#"Reduce the length of value characters to maximum 256 in message content variables."#),
            TwilioConversationsError::ErrorCode50710 => Some(r#"* Update ChatbotConfiguration.InitialContext parameter according to provided validation rules."#),
            TwilioConversationsError::ErrorCode50709 => Some(r#"* Update ChatbotConfiguration.FriendlyName parameter according to provided validation rules."#),
            TwilioConversationsError::ErrorCode50531 => Some(r#"Add readChannelMetadata permissions to the requesting user's role"#),
            TwilioConversationsError::ErrorCode50530 => Some(r#"* Confirm the channel type provides Channel Metadata."#),
            TwilioConversationsError::ErrorCode50533 => Some(r#"* Fix Channel Metadata payload and retry"#),
            TwilioConversationsError::ErrorCode50540 => Some(r#"Reduce the length of key characters to maximum 16 in message content variables."#),
            TwilioConversationsError::ErrorCode50702 => Some(r#"* Check your request and remove identity, user address, proxy address, projected address from it."#),
            TwilioConversationsError::ErrorCode50711 => Some(r#"* "system" is a reserved name for Virtual Agent participant. Please use another name."#),
            TwilioConversationsError::ErrorCode50453 => Some(r#"Remove proxy address field of the participant"#),
            TwilioConversationsError::ErrorCode50425 => Some(r#"[Locate](https://www.twilio.com/docs/conversations/api/conversation-resource#read-multiple-conversation-resources) a conflicting active conversation that has an identical participant added to it. To use such participant in the new conversation either [delete](https://www.twilio.com/docs/conversations/api/conversation-resource#delete-a-conversation-resource) old conversations completely if you don't need the message history, or just remove the participant that uses these numbers in a previous conversation."#)
        }
    }
}
