// AUTO-GENERATED. DO NOT MODIFY. YOUR CHANGES WILL BE OVER-WRITTEN
use crate::products::TwilioConversationsError;
use standard_error::traits::StandardErrorDocsTrait;

impl StandardErrorDocsTrait for TwilioConversationsError {
    fn docs(&self) -> &'static str {
        match self {
            TwilioConversationsError::ErrorCode50707 => r#"## ERROR - 50707

### Virtual Agent participant creation is not available for your account.

Virtual Agent participant creation is not available for your account. 

#### Possible Causes
You are trying to create virtual agent participant for account that is not allowed to use it.

#### Possible Solutions
* Please contact Twilio support to enable Virtual Agent participant creation for your account."#,
            TwilioConversationsError::ErrorCode50426 => r#"## ERROR - 50426

### Request contains too many participants.

Request contains more participants than allowed per request. Request contains more participants than allowed per request.

#### Possible Causes
You are attempting to add more participants than it's allowed to in one request of conversation with participants creation.

#### Possible Solutions
Reduce the amount of participants in the request to the allowed limit. Add all the rest of the participants using [Participant Resource](https://www.twilio.com/docs/conversations/api/conversation-participant-resource)."#,
            TwilioConversationsError::ErrorCode50534 => r#"## ERROR - 50534

### Failed to Read ChannelMetadata

 Unable to read Channel Metadata for Message

#### Possible Causes
* Channel Metadata was in an invalid format
* Channel Metadata required keys were missing
* Error querying for Channel Metadata
* Confirm the channel type provides Channel Metadata

#### Possible Solutions
* Retry Channel Metadata query"#,
            TwilioConversationsError::ErrorCode50526 => r#"## ERROR - 50526

### Invalid content SID

Specified Content SID does not match pattern HX[0-9a-f]{32} An invalid value was specified for the message content sid

#### Possible Causes
Content SID does not match the pattern HX[0-9a-f]{32}

#### Possible Solutions
Proper message Content SID value should be provided."#,
            TwilioConversationsError::ErrorCode50386 => r#"## ERROR - 50386

### Conversation in 'initializing' state may not be updated or used for message sending

Conversation in 'initializing' state may not be updated or used for message sending When conversation state is 'initializing', conversation may not be updated or used for message sending until it reaches states 'active' or 'inactive'. 'Initializing' state appears in GMMS conversations when not all resources were yet created to start message sending. It's possible when Conversation with Participants endpoint is used for GMMS conversation creation or when GMMS conversation is autocreated by inbound sms message.

#### Possible Causes
Participants of GMMS conversation are not yet created in background.

#### Possible Solutions
Wait till participants are created. To ensure that state is not 'initializing' wither fetch conversation till state changes to 'active' or 'inactive' or use onConversationStateUpdated webhook."#,
            TwilioConversationsError::ErrorCode50705 => r#"## ERROR - 50705

### Provided module sid should have prefix XE.

Provided module sid should have prefix XE. 

#### Possible Causes
You are trying to specify a different field value for ChatbotConfiguration.ModuleSid.

#### Possible Solutions
* Double check and fix value for ChatbotConfiguration.ModuleSid. It should start with XE prefix."#,
            TwilioConversationsError::ErrorCode50529 => r#"## ERROR - 50529

### Content variables are too long

Message content variables are exceeding allowed total length of 27,801 characters Allowed total length for message content variables is sum of key, value and JSON-specific characters' lengths.

#### Possible Causes
Either too many variables are defined (max 100 in total), or too long keys (max 16 characters per variable), or too long values (max 256 characters per variable).

#### Possible Solutions
Follow the length limitations:
- variables: 100 in total;
- key: 16 characters per variable;
- value: 256 characters per variable."#,
            TwilioConversationsError::ErrorCode50708 => r#"## ERROR - 50708

### ChatbotConfiguration.FriendlyName is required parameter.

ChatbotConfiguration.FriendlyName is required parameter. 

#### Possible Causes
You are trying to create Virtual Agent participant without ChatbotConfiguration.FriendlyName parameter.

#### Possible Solutions
* Specify ChatbotConfiguration.FriendlyName parameter."#,
            TwilioConversationsError::ErrorCode50704 => r#"## ERROR - 50704

### Provided addon sid should have prefix XB.

Provided addon sid should have prefix XB. 

#### Possible Causes
You are trying to specify different field value for ChatbotConfiguration.AddonSid.

#### Possible Solutions
* Double check and fix value for ChatbotConfiguration.AddonSid. It should start with XB prefix."#,
            TwilioConversationsError::ErrorCode50396 => r#"## ERROR - 50396

### EndDate parameter should be greater than StartDate parameter

 EndDate parameter should be greater than StartDate parameter

#### Possible Causes
EndDate parameter is before StartDate parameter

#### Possible Solutions
Set EndDate parameter greater than StartDate parameter"#,
            TwilioConversationsError::ErrorCode50452 => r#"## ERROR - 50452

### Group MMS is not enabled for this Account

 Triggered when an Account attempts to use GMMS without the appropriate permissions

#### Possible Causes
Missing permissions for GMMS

#### Possible Solutions
Verify if GMMS allowed for this Account"#,
            TwilioConversationsError::ErrorCode50514 => r#"## ERROR - 50514

### Conflicting message modification

The requested message is already being modified by a concurrent API request. The requested message is already being modified in this service instance concurrently with another API request. The last request will be rejected to avoid inconsistent state.

#### Possible Causes
* You might be submitting multiple concurrent requests to modify a message or its attributes with the same SID in the service instance.

#### Possible Solutions
* Implement an operation retrier and repeat the failed API requests after an interval of time, using an exponential backoff algorithm.
* Review your application logic that caused the race condition when modifying a message. Perhaps the conflicting operations are happening in a loop that could be avoided.
* Serialize your API requests that modify conversation messages. Wait until the original request completes and returns an API response, before sending any successive conversation requests."#,
            TwilioConversationsError::ErrorCode50528 => r#"## ERROR - 50528

### Content SID is missing

Message content variables are defined without a proper content sid Message content variables are defined without a proper content sid

#### Possible Causes
Content variables are present but there is no content sid defined

#### Possible Solutions
Message content sid should be present along with content variables"#,
            TwilioConversationsError::ErrorCode50542 => r#"## ERROR - 50542

### Unsupported Content Template Type

 Unsupported Content Template Type.

#### Possible Causes
You attempted to send a message using a content template type that is not yet supported in Conversations API.

#### Possible Solutions
Do not use the given Content Template when sending messages using Conversations API."#,
            TwilioConversationsError::ErrorCode50701 => r#"## ERROR - 50701

### Only one Virtual Agent participant is allowed per conversation.

Only one Virtual Agent participant is allowed per conversation. 

#### Possible Causes
There is already a Virtual Agent participant in the conversation.

#### Possible Solutions
* Remove existing Virtual Agent participant from the conversation before adding new one."#,
            TwilioConversationsError::ErrorCode50532 => r#"## ERROR - 50532

### Failed to create Channel Metadata

 Unable to create Channel Metadata for the Message

#### Possible Causes
* Channel Metadata was in an invalid format
* Channel Metadata required keys were missing
* A persistence error occured

#### Possible Solutions
* Validate Channel Metadata was in the correct format
* Retry Channel Metadata creation"#,
            TwilioConversationsError::ErrorCode50706 => r#"## ERROR - 50706

### Updating Virtual Agent participant information is not allowed.

Updating Virtual Agent participant information is not allowed. 

#### Possible Causes
You are trying to edit Virtual Agent participant information.

#### Possible Solutions
* If you want to change any fields of Virtual Agent participant you should delete it and create new one."#,
            TwilioConversationsError::ErrorCode50424 => r#"## ERROR - 50424

### One of the JSON requests for participant creation is invalid.

One of the requests in provided participants list is not a valid JSON object. One of the requests in provided participants list is not a valid JSON object.

#### Possible Causes
You are attempting to use invalid JSON object in the list of requests to create participants. If Conversations API is used it means that one of form parameters 'Participant' contains invalid JSON.

#### Possible Solutions
Validate that all the requests in the list of participants are strings that contain valid JSON objects. Make amendments to the requests that are not valid."#,
            TwilioConversationsError::ErrorCode50700 => r#"## ERROR - 50700

### Virtual Agent provider is invalid.

Provided Virtual Agent provider doesn't match the list of available providers for conversations. 

#### Possible Causes
Provided Virtual Agent provider doesn't match the list of available providers for conversations or has different case

#### Possible Solutions
* Check list of available Virtual Agent providers in Conversations documentation. Please note that values are case sensitive."#,
            TwilioConversationsError::ErrorCode50703 => r#"## ERROR - 50703

### DialogflowCX provider must contain module sid and addon sid.

DialogflowCX provider must contain module sid and addon sid. 

#### Possible Causes
Either ChatbotConfiguration.ModuleSid or ChatbotConfiguration.ModuleSid are not specified for DialogflowCX Virtual Agent participant.

#### Possible Solutions
* If you try to create DialogflowCX provider, you must specify ChatbotConfiguration.ModuleSid and ChatbotConfiguration.ModuleSid."#,
            TwilioConversationsError::ErrorCode50527 => r#"## ERROR - 50527

### Invalid content variables format

Specified content variables value is invalid JSON object An invalid value was specified for the message content variables

#### Possible Causes
Content variables value is invalid JSON object

#### Possible Solutions
Message content variables value should be a valid JSON object."#,
            TwilioConversationsError::ErrorCode50427 => r#"## ERROR - 50427

### Errors occurred during multiple participants creation.

One or more other errors occurred during multiple participants creation. One or more other errors occurred during multiple participants creation. This error is received in [Error logs](https://www.twilio.com/docs/messaging/guides/debugging-tools#how-to-use-the-twilio-error-logs) in Twilio Console and contains the list of other [errors](https://www.twilio.com/docs/api/errors).

#### Possible Causes
The error during multiple participants creation is unexpected due to extensive validation done before conversation with multiple participants creation. The error may appear if there are simultaneous conflicting requests. To learn more about the possible issues that happened during participants creation read the description of the received error in [Twilio Console Error Logs](https://console.twilio.com/us1/monitor/logs/debugger/errors) and check the errors. The Possible Causes of these errors caused the failure of the whole participants creation process. 

#### Possible Solutions
It's recommended to retry the request after some time. In case of this error the failed conversation is moved to closed state and all the resources occupied by this conversation are released, that allows retry not to fail."#,
            TwilioConversationsError::ErrorCode50541 => r#"## ERROR - 50541

### Content variable value is too long

Value in key-value pair of message content variables is exceeding allowed maximum length of 256 characters Length of value in message content variables is restricted to 256 characters.

#### Possible Causes
One or many values in key-value pairs of message content variables are hitting limit of allowed 256 characters.

#### Possible Solutions
Reduce the length of value characters to maximum 256 in message content variables."#,
            TwilioConversationsError::ErrorCode50710 => r#"## ERROR - 50710

### ChatbotConfiguration.InitialContext is invalid.

ChatbotConfiguration.InitialContext is invalid. 

#### Possible Causes
You are trying to create Virtual Agent participant with ChatbotConfiguration.InitialContext which is longer than 32768 characters.

#### Possible Solutions
* Update ChatbotConfiguration.InitialContext parameter according to provided validation rules."#,
            TwilioConversationsError::ErrorCode50709 => r#"## ERROR - 50709

### ChatbotConfiguration.FriendlyName is invalid.

ChatbotConfiguration.FriendlyName is invalid. 

#### Possible Causes
You are trying to create Virtual Agent participant with ChatbotConfiguration.FriendlyName which is longer than 256 characters.

#### Possible Solutions
* Update ChatbotConfiguration.FriendlyName parameter according to provided validation rules."#,
            TwilioConversationsError::ErrorCode50531 => r#"## ERROR - 50531

### Not Authorized to make this request

 User is not authorized to access the requested resource

#### Possible Causes
User making the request does not have the appropriate permission:  readChannelMetadata

#### Possible Solutions
Add readChannelMetadata permissions to the requesting user's role"#,
            TwilioConversationsError::ErrorCode50530 => r#"## WARNING - 50530

### Channel Metadata not found

 Channel Metadata was not found for the Message

#### Possible Causes
* Channel Metadata was not found for this message
* Channel Metadata is only relevant for select channel types 

#### Possible Solutions
* Confirm the channel type provides Channel Metadata."#,
            TwilioConversationsError::ErrorCode50533 => r#"## ERROR - 50533

### Failed to parse ChannelMetadata Json

 The Channel Metadata Content was not in the correct format.

#### Possible Causes
* Channel Metadata Json was expected.
* Channel Metadata required keys were missing

#### Possible Solutions
* Fix Channel Metadata payload and retry"#,
            TwilioConversationsError::ErrorCode50540 => r#"## ERROR - 50540

### Content variable key is too long

Key in key-value pair of message content variables is exceeding allowed maximum length of 16 characters Length of key in message content variables is restricted to 16 characters.

#### Possible Causes
One or many keys in key-value pairs of message content variables are hitting limit of allowed 16 characters.

#### Possible Solutions
Reduce the length of key characters to maximum 16 in message content variables."#,
            TwilioConversationsError::ErrorCode50702 => r#"## ERROR - 50702

### Verify Agent participant should not have identity, user address, proxy address, projected address.

Verify Agent participant should not have identity, user address, proxy address, projected address. 

#### Possible Causes
Virtual Agent participant should have only ChatbotConfiguration prefixed fields. Other fields are not available for Virtual Agent participants.

#### Possible Solutions
* Check your request and remove identity, user address, proxy address, projected address from it."#,
            TwilioConversationsError::ErrorCode50711 => r#"## ERROR - 50711

### ChatbotConfiguration.FriendlyName is reserved.

ChatbotConfiguration.FriendlyName is reserved. 

#### Possible Causes
You are trying to create Virtual Agent participant with ChatbotConfiguration.FriendlyName as "system".

#### Possible Solutions
* "system" is a reserved name for Virtual Agent participant. Please use another name."#,
            TwilioConversationsError::ErrorCode50453 => r#"## ERROR - 50453

### Proxy Address of participant is not supported for this channel

 Triggered when account is trying to add or update the participant with a proxy address that is not supported in given channel, e.g. email, etc.

#### Possible Causes
Proxy Address for participant is not supported in given channel

#### Possible Solutions
Remove proxy address field of the participant"#,
            TwilioConversationsError::ErrorCode50425 => r#"## ERROR - 50425

### Participant and proxy address pairs for one or more participants already in use.

One or more of the requested participant and proxy address pairs are already registered in the system. One or more of the requested participant and proxy address pairs are already registered in the system.

#### Possible Causes
You are attempting to map a non-Chat (e.g. SMS) conversation participants with participant address and proxy address that are already assigned to an active conversation. There may be only one unique mapping between your conversation participant's public phone number and internal (proxying) Twilio phone number, in order to route inbound messages correctly.

#### Possible Solutions
[Locate](https://www.twilio.com/docs/conversations/api/conversation-resource#read-multiple-conversation-resources) a conflicting active conversation that has an identical participant added to it. To use such participant in the new conversation either [delete](https://www.twilio.com/docs/conversations/api/conversation-resource#delete-a-conversation-resource) old conversations completely if you don't need the message history, or just remove the participant that uses these numbers in a previous conversation."#
        }
    }
}
