// AUTO-GENERATED. DO NOT MODIFY. YOUR CHANGES WILL BE OVER-WRITTEN
use crate::products::TwilioProgrammableChatError;
use standard_error::traits::StandardErrorDocsTrait;

impl StandardErrorDocsTrait for TwilioProgrammableChatError {
    fn docs(&self) -> &'static str {
        match self {
            TwilioProgrammableChatError::ErrorCode50304 => r#"## Error - 50304

### Programmable Chat: Attributes not valid JSON	 

#### Possible Causes 
*  Specified Channel's Attributes are not valid JSON.

#### Possible Solutions
*  Confirm a valid Channel's Attributes in JSON format are being passed in request. "#,
            TwilioProgrammableChatError::ErrorCode50412 => r#"
## Error - 50412

### Conversations: Participant proxy address is empty
The provided participant proxy address parameter must not be empty.

#### Possible Causes
* MessagingBinding.ProxyAddress parameter provided with the request contains an empty string.

#### Possible Solutions
* Add a non-empty MessagingBinding.ProxyAddress parameter to the API request and supply a valid participant proxy address matching your desired channel, e.g. SMS, MMS or WhatsApp.

"#,
            TwilioProgrammableChatError::ErrorCode50417 => r#"
## Error - 50417

### Conversations: Participants limit exceeded
The total number of participants in this conversation exceeds the limit.

#### Possible Causes
* You are trying to add more native Programmable Chat participants than allowed in this conversation. Maximum number of conversation participants is 100 by default, can be increased to 1000 programmatically.

#### Possible Solutions
* Increase the [maximum limit](https://www.twilio.com/docs/chat/rest/service-resource#update-a-service-resource) by using Developer Console or REST API. Up to [1000 participants](https://www.twilio.com/docs/conversations/conversations-limits) are supported.
* Retrieve a [list of current participants](https://www.twilio.com/docs/conversations/api/conversation-participant-resource#read-multiple-conversationparticipant-resources) in this conversation and [remove](https://www.twilio.com/docs/conversations/api/conversation-participant-resource#delete-a-conversationparticipant-resource) the ones that are no longer required.
* Consider reusing participant identity across multiple clients, if user behavior is similar and data may be shared.

"#,
            TwilioProgrammableChatError::ErrorCode50208 => r#"## Error - 50208

### Programmable Chat: User channel limit exceeded

#### Possible Causes 
*  User has joined too many channels

#### Possible Solutions
*  Increase user channel limit for service instance
*  Remove user from channels"#,
            TwilioProgrammableChatError::ErrorCode50052 => r#"## Error - 50052

### Programmable Chat: Invalid consumption interval format	

#### Possible Causes 
*  Request does not contain valid consumption interval.

#### Possible Solutions
*  Confirm a valid consumption interval format is being passed in request. "#,
            TwilioProgrammableChatError::ErrorCode50205 => r#"## Error - 50205

### Programmable Chat: User unauthorized to set role

#### Possible Causes 
*  Specified User does not have Role or Permissions to set the Role.

#### Possible Solutions
*  Confirm a valid User having valid Role and Permissions is being passed in request. "#,
            TwilioProgrammableChatError::ErrorCode50207 => r#"## Error - 50207

### Programmable Chat: Identity too long

#### Possible Causes 
*  Specified User's Identity parameter is too long.

#### Possible Solutions
*  Confirm a valid length User's Identity is being passed in request. "#,
            TwilioProgrammableChatError::ErrorCode50054 => r#"## Error - 50054

### Programmable Chat: Invalid webhook format

#### Possible Causes 
*  Request does not contain correctly formatted parameters for webhooks.

#### Possible Solutions
*  Confirm a valid webhook parameters are being passed in request. "#,
            TwilioProgrammableChatError::ErrorCode50074 => r#"## ERROR - 50074

### Global actions per second limit exceeded

Service instance global actions per second limit exceeded. 

#### Possible Causes
*  Your application is generating too many actions per second (APS) for Chat service instance either to a specific Chat API endpoint or across different Chat API endpoints
*  Note that the APS limit is enforced for commands such as sending a Message or creating a Channel - not Read actions

#### Possible Solutions
*  Make sure your application (client or backend) uses a good exponential back-off algorithm like the [one advocated by Amazon](https://www.awsarchitectureblog.com/2015/03/backoff.html) to retry on HTTP 429 (rate limiting) responses
*  Reduce the rate of requests and introduce basic rate control, e.g. for user or channel provisioning use-cases
*  Reduce number of concurrent requests"#,
            TwilioProgrammableChatError::ErrorCode50384 => r#"## ERROR - 50384

### Initial state can't be 'closed

It's not allowed to set 'closed' state for newly created converastion You are trying to set 'closed' state during conversation creation which is not allowed

#### Possible Causes
* 'State' parameter value is set to 'closed' which is not allowed during conversation creation

#### Possible Solutions
* Change 'State' parameter value to either 'active' or 'inactive' during conversation creation"#,
            TwilioProgrammableChatError::ErrorCode50434 => r#"## ERROR - 50434

### Participant projected address not provided

No Projected Address provided for this participant the group conversation. This is a Group MMS conversation, but the given participant has neither an Address nor a ProjectedAddress.

#### Possible Causes
* You may have inadvertently provided an Identity (chat user), rather than an Address (mobile phone).
* You may have mistyped or inadvertently left out the ProjectedAddress parameter.

#### Possible Solutions
* [Provide a Projected Address](https://www.twilio.com/docs/conversations/group-texting#scenario-1-set-up-a-group-message-with-one-chat-participant-and-two-sms-participants) when adding this Participant to the Group Conversation. This is the correct path when adding Chat participants to a Group MMS conversation."#,
            TwilioProgrammableChatError::ErrorCode50509 => r#"## Error - 50509

### Programmable Chat: Media message body update not allowed

#### Possible Causes 
*  Media message body cannot be updated. 
"#,
            TwilioProgrammableChatError::ErrorCode50077 => r#"## ERROR - 50077

### Invalid pre-webhook url

Request does not contain a valid url for pre-webhook. 

#### Possible Causes
*  Provided url has the wrong format. 

#### Possible Solutions
*  Make sure that url is complete and properly encoded.
*  Make sure that url contains valid protocol and hostname and doesn't contain invalid characters."#,
            TwilioProgrammableChatError::ErrorCode50451 => r#"## ERROR - 50451

### One user identifier parameter for lookup at a time is allowed. Please, use either Identity or Address.

Request should contain either Identity or Address parameter, not both. 

#### Possible Causes
*  Both Identity and Address were provided

#### Possible Solutions
*  Please, provide either Identity or Address"#,
            TwilioProgrammableChatError::ErrorCode50059 => r#"## Error - 50059

### Programmable Chat: Notification sound name too long

#### Possible Causes 
*  Specified notification sound parameter is too long.

#### Possible Solutions
*  Confirm a valid length notification sound parameter is being passed in request. "#,
            TwilioProgrammableChatError::ErrorCode50431 => r#"
## Error - 50431

### Conversations: Participant SID not provided
Conversation participant SID parameter is required but was not provided.

#### Possible Causes
* The API expected a participant SID input parameter, but it was either not provided or was misspelled.

#### Possible Solutions
* Add a valid participant SID parameter to the API request and confirm that it is not misspelled.

"#,
            TwilioProgrammableChatError::ErrorCode50375 => r#"## ERROR - 50375

### TimeToInactive should be greater or equal to 1 minute

Duration to transfer channel to inactive state can't be shorter than 1 minute. 'TimeToInactive' duration should be greater or equal to 1 minute.

#### Possible Causes
* 'TimeToInactive' field is set to duration which is less than 1 minute.

#### Possible Solutions
* Set 'TimeToInactive' field to duration which is longer than 1 minute."#,
            TwilioProgrammableChatError::ErrorCode50380 => r#"## ERROR - 50380

### TimeToClosed format is invalid

'TimeToClosed' parameter format should be ISO8601 duration 'TimeToClosed' parameter supported values are ISO8601 durations.

#### Possible Causes
* 'TimeToClosed' parameter value is not ISO8601 duration.

#### Possible Solutions
* Use ISO8601 duration format value for 'TimeToInactive' parameter. For example PT0S, PT10M, P1D etc.."#,
            TwilioProgrammableChatError::ErrorCode50449 => r#"## ERROR - 50449

### Conflicting channel modification

The requested channel is already being added or removed by a concurrent API request. The requested channel is already being added or removed from this service instance concurrently with another API request. The last request will be rejected to avoid inconsistent state.

#### Possible Causes
* You might be adding a new channel to the service instance while the channel with the same unique name is being removed from another thread.
* You might be submitting multiple concurrent requests to add a channel with the same unique name to the service instance.
* You might be submitting multiple concurrent requests to delete the same channel by SID or unique name from the service instance.

#### Possible Solutions
* Implement an operation retrier and repeat the failed channel API requests after an interval of time, using an exponential backoff algorithm.
* Review your application logic that caused the race condition when adding or removing users. Perhaps the conflicting operations are happening in a loop that could be avoided.
* Serialize your API requests that add or remove channels. Wait until the original request completes and returns an API response, before sending any successive channel requests."#,
            TwilioProgrammableChatError::ErrorCode50057 => r#"## Error - 50057

### Programmable Chat: Webhook call failed to execute successfully

#### Possible Causes 
*  The Webhook endpoint called returned an error code

#### Possible Solutions
* Check your Webhook execution logic to ensure it is processing the webhook calls correctly and passing back 200 OK"#,
            TwilioProgrammableChatError::ErrorCode50340 => r#"
## Error - 50340

### Conversations: Messaging service SID not provided
Messaging service SID parameter is required but was not provided with an API request.

#### Possible Causes
* The API expected a MessagingServiceSid input parameter, but it was either not provided or was misspelled.

#### Possible Solutions
* Add a valid MessagingServiceSid parameter to the API request and confirm that it is not misspelled.

"#,
            TwilioProgrammableChatError::ErrorCode50377 => r#"## ERROR - 50377

### Can't update conversation as it's in final closed state

Conversation is in closed state and can't be used anymore Conversation state is 'closed' so it can't be used for chatting anymore.

#### Possible Causes
* Conversation state is set manually or by timer to 'closed' which means that conversation can't be updated anymore.

#### Possible Solutions
* Create new conversation in 'active' state without 'TimeToClosed' or with some big duration to avoid unexpected movement to 'closed' state."#,
            TwilioProgrammableChatError::ErrorCode50506 => r#"## Error - 50506

### Programmable Chat: Media SID not provided	

#### Possible Causes 
*  Request does not contain required Media SID parameter.

#### Possible Solutions
*  Confirm a valid Media SID is being passed in request. "#,
            TwilioProgrammableChatError::ErrorCode50101 => r#"## Error - 50101

### Programmable Chat: Channel role not found

#### Possible Causes 
*  Specified Channel Role does not exist. 

#### Possible Solutions
*  Confirm a valid Channel Role is being passed in request. "#,
            TwilioProgrammableChatError::ErrorCode50332 => r#"## Error - 50332

### Programmable Chat: Channel webhook url too long

#### Possible Causes 
*  Channel webhook url provided in request is too long.

#### Possible Solutions
*  Confirm a valid parameter is passed in request. "#,
            TwilioProgrammableChatError::ErrorCode50393 => r#"## ERROR - 50393

### Type value should be 'private'.

Type parameter value is required and should be 'private'. Migration to 'public' type is not allowed. 

#### Possible Causes
*  No Type parameter provided
*  Provided Type parameter not equal to 'private'

#### Possible Solutions
* Change Type parameter to 'private'"#,
            TwilioProgrammableChatError::ErrorCode50436 => r#"## ERROR - 50436

### Participant limit exceeded for group conversation

The maximum of participants allowed in a group conversation has been exceeded. There is a limit to how many participants can be active in the same conversation at once. Adding this participant would exceed that limit.

#### Possible Causes
* You may have failed to remove old participants in a long-running conversation.
* You may have reached an edge-case in your application that creates unusually large conversations, in excess of the limit.

#### Possible Solutions
* Drop another participant from the conversation before adding new one."#,
            TwilioProgrammableChatError::ErrorCode50374 => r#"## ERROR - 50374

### 'State' field can't be empty

'State' field can't be empty or removed from channel. 'State' field can't be removed or set to empty if it was already set before.

#### Possible Causes
* 'State' field is set to empty string value when channel has some non-empty state.

#### Possible Solutions
* Remove 'State' field from request parameters.
* Set 'State' field to some valid value('active', 'inactive', 'closed', etc.)"#,
            TwilioProgrammableChatError::ErrorCode50330 => r#"## Error - 50330

### Programmable Chat: Channel webhook filter not provided

#### Possible Causes 
*  Request does not contain parameter for channel webhook filter.

#### Possible Solutions
*  Confirm a valid parameter is passed in request. "#,
            TwilioProgrammableChatError::ErrorCode50382 => r#"## ERROR - 50382

### Timer can't be set without state

You are trying to set 'TimeToInactive' and/or 'TimeToClosed' without 'State' parameter in request You are trying to set 'TimeToInactive' and/or 'TimeToClosed' without 'State' parameter in request during conversation creation

#### Possible Causes
* 'State' parameter is missing while 'TimeToInactive' and/or 'TimeToClosed' are present

#### Possible Solutions
* Add 'State' parameter in conversation creation request
* Remove 'TimeToInactive' and/or 'TimeToClosed' from conversation creation request"#,
            TwilioProgrammableChatError::ErrorCode50418 => r#"
## Error - 50418

### Conversations: Non-Chat participants limit exceeded
The total number of SMS, WhatsApp, or other non-Chat participants in this conversation exceeds the allowed limit.

#### Possible Causes
* You are trying to add more non-Chat participants than allowed in this conversation. [Maximum number](https://www.twilio.com/docs/conversations/conversations-limits) of conversation participants is 50.

#### Possible Solutions
* Retrieve a [list of current participants](https://www.twilio.com/docs/conversations/api/conversation-participant-resource#read-multiple-conversationparticipant-resources) in this conversation and [remove](https://www.twilio.com/docs/conversations/api/conversation-participant-resource#delete-a-conversationparticipant-resource) the ones that are no longer required.

"#,
            TwilioProgrammableChatError::ErrorCode50378 => r#"## ERROR - 50378

### 'State' parameter value is invalid

'State' parameter value is set to some not supported value 'State' parameter value is set to some not supported value. Supported values are 'active', 'inactive', 'closed'.

#### Possible Causes
* 'State' parameter value is set to values different from 'active', 'inactive', 'closed'.

#### Possible Solutions
* Use following values for 'State' parameter: 'active', 'inactive', 'closed'."#,
            TwilioProgrammableChatError::ErrorCode50438 => r#"## ERROR - 50438

### Group conversation with given participant list already exists

There is an existing (open) group conversation with the same participant list. There is an existing group conversation with the same participant list.

#### Possible Causes
* You previously created a conflicting conversation
* Twilio created a conversation — which now conflicts — in the course of Autocreation.

#### Possible Solutions
* If the existing conversation is no longer in use, delete or [close the conversation](https://www.twilio.com/docs/conversations/states-timers#closed-conversations).
* Alternatively, locate the other conversation and use that in place of this one."#,
            TwilioProgrammableChatError::ErrorCode50107 => r#"## Error - 50107

### Programmable Chat: User not authorized for command	

#### Possible Causes 
*  Specified User is not authorized for this command due to his Role or Permissions set.

#### Possible Solutions
*  Confirm a valid Role and Permissions exist for this User to execute request's provided command. "#,
            TwilioProgrammableChatError::ErrorCode50201 => r#"## Error - 50201

### Programmable Chat: User already exists

#### Possible Causes 
*  Specified User already exists for this Service.

#### Possible Solutions
*  Confirm a valid non-existing User is being passed in request. "#,
            TwilioProgrammableChatError::ErrorCode50328 => r#"## Error - 50328

### Programmable Chat: Channel webhook type not provided

#### Possible Causes 
*  Request does not contain parameter for channel webhook type.

#### Possible Solutions
*  Confirm a valid parameter is passed in request. "#,
            TwilioProgrammableChatError::ErrorCode50104 => r#"## Error - 50104

### Programmable Chat: Permission not found	

#### Possible Causes 
*  Specified Permission does not exist.

#### Possible Solutions
*  Confirm a valid Permission is being passed in request. "#,
            TwilioProgrammableChatError::ErrorCode50309 => r#"## ERROR - 50309

### Programmable Chat: Invalid Date Updated parameter

Request does not contain correctly formatted parameter for Date Updated ## Error - 50309

### Programmable Chat: Invalid Date Updated parameter

#### Possible Causes 
*  Request does not contain correctly formatted parameter for Date Updated.

#### Possible Solutions
*  Confirm a valid Date Updated parameter is passed in request. 

#### Possible Causes
*  Request does not contain correctly formatted parameter for Date Updated.

#### Possible Solutions
*  Confirm a valid Date Updated parameter is passed in request. "#,
            TwilioProgrammableChatError::ErrorCode50363 => r#"
## Error - 50363

### Conversations: Invalid conversation webhook SID
An invalid conversation webhook SID was supplied.

#### Possible Causes
* Webhook SID parameter provided with the request does not match a valid SID pattern (WHxx).

#### Possible Solutions
* Add a valid Webhook SID parameter to the API request and supply a SID that exists under your account.

"#,
            TwilioProgrammableChatError::ErrorCode50414 => r#"
## Error - 50414

### Conversations: Participant address type does not match proxy address type
The provided participant address mismatches the proxy address channel type.

#### Possible Causes
* MessagingBinding.Address and MessagingBinding.ProxyAddress passed to the API do not belong to the same communication channel.

#### Possible Solutions
* Ensure that both MessagingBinding.Address and MessagingBinding.ProxyAddress are both of the same type, e.g. both SMS or both WhatsApp.

"#,
            TwilioProgrammableChatError::ErrorCode50310 => r#"## Error - 50310

### Programmable Chat: Author parameter is too long

#### Possible Causes 
*  Author parameter is too long.

#### Possible Solutions
*  Confirm a valid length Author is being passed in request. "#,
            TwilioProgrammableChatError::ErrorCode50371 => r#"
## Error - 50371

### Conversations: Conversation webhook trigger not provided
Scoped webhook trigger keywords are required but were not provided.

#### Possible Causes
* The API expected at least one Configuration.Triggers input parameter, but it was either not provided or was misspelled.

#### Possible Solutions
* Add a one or more valid Configuration.Triggers parameters to the API request and confirm that they are not misspelled.

"#,
            TwilioProgrammableChatError::ErrorCode50373 => r#"
## Error - 50373

### Conversations: No Messaging Service assigned to Conversation
This Conversation does not have an assigned Messaging Service, therefore cannot accept non-chat participants.

#### Possible Causes
* You may be attempting to add a non-Chat participant (e.g. on SMS) to a channel from Programmable Chat. As Chat precedes the Conversations product, these channels first need to be provided a Messaging Service before they can access SMS, WhatsApp, or other channels.

#### Possible Solutions
* Update the Conversation with any existing Messaging Service SID on your account. After that, you will be able to add non-Chat participants and continue the conversation with them involved.


"#,
            TwilioProgrammableChatError::ErrorCode50442 => r#"## ERROR - 50442

### Failed to remove the projected address of a participant.

The participant should have a projected address in order to remove it. The conversation participant being updated does not have a projected address.

#### Possible Causes
* The participant does not have the projected address you are trying to remove.

#### Possible Solutions
* Check that the request does not update projected address of this particular participant."#,
            TwilioProgrammableChatError::ErrorCode50370 => r#"
## Error - 50370

### Conversations: Conversation webhook filter not provided
Scoped webhook filter parameter is required but was not provided.

#### Possible Causes
* The API expected at least one Configuration.Filters input parameter, but it was either not provided or was misspelled.

#### Possible Solutions
* Add one or more valid Configuration.Filters parameters to the API request and confirm that they are not misspelled.

"#,
            TwilioProgrammableChatError::ErrorCode50511 => r#"
## Error - 50511

### Conversations: Invalid message media content type
An invalid multimedia attachment content type was supplied.

#### Possible Causes
* The MIME content type of multimedia attachment you are trying to send is not acceptable for this conversation.

#### Possible Solutions
* Make sure that the content type of multimedia attachments conform to channel constraints. Check for accepted MIME types in [API documentation](https://www.twilio.com/docs/conversations/conversations-limits#media-limits).  

"#,
            TwilioProgrammableChatError::ErrorCode50327 => r#"## Error - 50327

### Programmable Chat: Invalid channel webhook flow sid

#### Possible Causes 
*  Request does not contain correct parameter for channel webhook flow sid.

#### Possible Solutions
*  Confirm a valid parameter is passed in request. "#,
            TwilioProgrammableChatError::ErrorCode50211 => r#"
## Error - 50211

### Conversations: Identity parameter not acceptable for this Participant
Participants on SMS, WhatsApp or other non-Chat channels cannot have Identities.

#### Possible Causes
This error is fired when the parameter *Identity* is provided together with either:
* MessagingBinding.Address,
* MessagingBinding.ProxyAddress, or
* both of the above.

As the error indicates, such a combination is not allowed. You can provide either an address binding or an Identity, but not both.

#### Possible Solutions
* Avoid using both Identity and MessagingBinding.Address parameters when creating a Conversation participant. Instead, distinguish the use-case for your participants upfront and supply either the Identity for users of Programmable Chat Web/iOS/Android SDKs, or MessagingBinding.Address for non-Chat users such as SMS, MMS, WhatsApp, etc.
"#,
            TwilioProgrammableChatError::ErrorCode50602 => r#"## ERROR - 50602

### Programmable Chat: Cannot decline invite when already channel member.

Specified user is already member of this channel so his invitation is not present. 

#### Possible Causes
User was added directly to the channel and his invite was removed so he can't decline it.

#### Possible Solutions
User shouldn't decline invite if he was already added to channel."#,
            TwilioProgrammableChatError::ErrorCode50437 => r#"## ERROR - 50437

### Group Participant already exists

Participant already exists in the group conversation. A participant matching this one already exists in the conversation.

#### Possible Causes
* An participant with the same ProxyAddress+Address pair already exists in this conversation
* A native GMMS Participant with the same Address exists in this conversation
* A Chat Participant with the same Identity and Project Address exists in this conversation

This error may result from a race condition, where a participant was inadvertently added twice.

#### Possible Solutions
* Add the participant with a different Address or Identity
* Ignore this error — your participant is already present."#,
            TwilioProgrammableChatError::ErrorCode50051 => r#"## Error - 50051

### Programmable Chat: Service SID not provided

#### Possible Causes 
*  Request does not contain required Service SID parameter.

#### Possible Solutions
*  Confirm a valid Service SID is being passed in request. "#,
            TwilioProgrammableChatError::ErrorCode50408 => r#"
## Error - 50408

### Conversations: Invalid participant proxy address
An invalid participant proxy address was supplied.

#### Possible Causes
* Participant MessagingBinding.ProxyAddress parameter provided with the request is not a valid conversation proxy address.

#### Possible Solutions
* Check that the length of MessagingBinding.ProxyAddress parameter does not exceed 256 characters.
* Ensure that MessagingBinding.ProxyAddress parameter contains an address with valid syntax (e.g. SMS phone numbers must follow [E.164 format](https://www.twilio.com/docs/glossary/what-e164)).

"#,
            TwilioProgrammableChatError::ErrorCode50440 => r#"## ERROR - 50440

### Failed to remove the proxy address of a participant.

The participant should have a proxy address in order to remove it. The conversation participant being updated does not have a proxy address.

#### Possible Causes
* The participant does not have a proxy address you are trying to remove.
* The participant you are updating is not an SMS participant.

#### Possible Solutions
* Check that the request does not update proxy address of this particular participant."#,
            TwilioProgrammableChatError::ErrorCode50351 => r#"
## Error - 50351

### Conversations: Conversation SID not provided
Conversation SID parameter is required but was not provided.

#### Possible Causes
Your API request did not include a ConversationSid form parameter.

#### Possible Solutions
Add a valid ConversationSid parameter to the API request, making sure you haven't spelled the parameter name wrong.

"#,
            TwilioProgrammableChatError::ErrorCode50213 => r#"## ERROR - 50213

### Conflicting user modification

The requested user is already being added or removed from this channel by a concurrent API request. The requested user is already being added or removed from this service instance concurrently with another API request. The last request will be rejected to avoid inconsistent state.

#### Possible Causes
* You might be adding a new user to the service instance while the user with the same identity is being removed from another thread.
* You might be submitting multiple concurrent requests to add a user with the same identity to the service instance.
* You might be submitting multiple concurrent requests to delete the same user by SID or identity from the service instance.

#### Possible Solutions
* Implement an operation retrier and repeat the failed user API requests after an interval of time, using an exponential backoff algorithm.
* Review your application logic that caused the race condition when adding or removing users. Perhaps the conflicting operations are happening in a loop that could be avoided."#,
            TwilioProgrammableChatError::ErrorCode50100 => r#"## Error - 50100

### Programmable Chat: Role not found

#### Possible Causes 
*  Specified Role does not exist.

#### Possible Solutions
*  Confirm a valid Role is being passed in request. "#,
            TwilioProgrammableChatError::ErrorCode50369 => r#"
## Error - 50369

### Conversations: Conversation webhook URL not provided
Conversation webhook URL parameter is required but was not provided.

#### Possible Causes
* The API expected a Configuration.Url input parameter, but it was either not provided or was misspelled.

#### Possible Solutions
* Add a valid Configuration.Url parameter to the API request and confirm that it is not misspelled.

"#,
            TwilioProgrammableChatError::ErrorCode50516 => r#"## ERROR - 50516

### Programmable Chat: Message index is not a number

Programmable Chat: Message index is not a number Not a number string was provided as message index.

#### Possible Causes
* Some text or message sid was provided as message index

#### Possible Solutions
* Proper message index should be provided."#,
            TwilioProgrammableChatError::ErrorCode50204 => r#"## Error - 50204

### Programmable Chat: Identity not provided	

#### Possible Causes 
*  Request does not contain required "Identity" parameter.

#### Possible Solutions
*  Confirm a valid "Identity" parameter is being passed in request. "#,
            TwilioProgrammableChatError::ErrorCode50416 => r#"
## Error - 50416

### Conversations: Participant and proxy address pair is already in use
The requested participant and proxy address pair is already registered in the system.

#### Possible Causes
* You are attempting to map a non-Chat (e.g. SMS) conversation member with participant address and proxy address that are already assigned to an active conversation. For example, there may be only one unique mapping between your conversation participant's public phone number and internal (proxying) Twilio phone number, in order to route inbound messages correctly.

#### Possible Solutions
* [Locate](https://www.twilio.com/docs/conversations/api/conversation-resource#read-multiple-conversation-resources) a conflicting active conversation that has an identical participant added to it. Either [delete](https://www.twilio.com/docs/conversations/api/conversation-resource#delete-a-conversation-resource) old conversations completely if you don't need the message history, or just remove the participant that uses these numbers in a previous conversation.

"#,
            TwilioProgrammableChatError::ErrorCode50053 => r#"## Error - 50053

### Programmable Chat: Invalid typing indicator format

#### Possible Causes 
*  Request does not contain valid typing indicator timeout.

#### Possible Solutions
*  Confirm a valid typing indicator timeout is being passed in request. 
"#,
            TwilioProgrammableChatError::ErrorCode50513 => r#"## ERROR - 50513

###  Message author should be among Group MMS participants.

Author identity or address not found among Group MMS participants Author identity or address not found among Group MMS participants.

#### Possible Causes
* Provided author Address is not among Group MMS participants.
* Provided author Identity is not among Group MMS participants.

#### Possible Solutions
* Check that the Author is participant of Group MMS conversation."#,
            TwilioProgrammableChatError::ErrorCode50419 => r#"
## Error - 50419

### Programmable Chat: Conflicting member modification
The requested member is already being added or removed from this channel concurrently with another API request. The last request will be rejected to avoid inconsistent state.

#### Possible Causes
* You might be adding a new member to the channel while the member with the same identity is being removed from another thread.
* You might be submitting multiple concurrent requests to add a member with the same identity to the channel.
* You might be submitting multiple concurrent requests to delete the same member by SID or identity from the channel.

#### Possible Solutions
* Implement an operation retrier and repeat the failed member API requests after an interval of time, using an exponential backoff algorithm.
* Review your application logic that caused the race condition when adding or removing members. Perhaps the conflicting operations are happening in a loop that could be avoided.
* Serialize your API requests that add or remove members. Wait until the original request completes and returns an API response, before sending any successive member requests.
"#,
            TwilioProgrammableChatError::ErrorCode50413 => r#"
## Error - 50413

### Conversations: Account is not authorized to use proxy address
Your account is not authorized to use the requested proxy address.

#### Possible Causes
* The provided conversation proxy address is not allowed to be used under your account.

#### Possible Solutions
* Check that MessagingBinding.ProxyAddress parameter passed to API contains a proxy address that belongs to your account. For example, use a Twilio phone number that is purchased under the same project.

"#,
            TwilioProgrammableChatError::ErrorCode50362 => r#"
## Error - 50362

### Conversations: Too many conversation webhook triggers
The number of keywords used to trigger the webhook configured for this conversation would have exceeded the limit.

#### Possible Causes
You are attempting to add more than 5 keywords as triggers for this webhook.

#### Possible Solutions
* Retrieve a list of the [existing keywords](https://www.twilio.com/docs/conversations/api/conversation-scoped-webhook-resource#fetch-a-conversationscopedwebhook-resource) used as triggers and remove the ones that are redundant or obsolete.
* Consider creating a new scoped webhook with an alternative set of trigger keywords.

"#,
            TwilioProgrammableChatError::ErrorCode50320 => r#"## Error - 50320

### Programmable Chat: Channel webhook not found

#### Possible Causes 
*  Specified channel webhook does not exist.

#### Possible Solutions
*  Confirm a valid channel webhook SID is provided in request."#,
            TwilioProgrammableChatError::ErrorCode50202 => r#"## Error - 50202

### Programmable Chat: User SID not provided

#### Possible Causes 
*  Request does not contain required User SID parameter.

#### Possible Solutions
*  Confirm a valid User SID is being passed in request. "#,
            TwilioProgrammableChatError::ErrorCode50105 => r#"## Error - 50105

### Programmable Chat: Invalid role type	

#### Possible Causes 
*  Specified Role Type does not exist. 

#### Possible Solutions
*  Confirm a valid Role Type is being passed in request. "#,
            TwilioProgrammableChatError::ErrorCode50058 => r#"## Error - 50058

### Programmable Chat: Notification template too long

#### Possible Causes 
*  Specified notification template parameter is too long.

#### Possible Solutions
*  Confirm a valid length notification template is being passed in request. "#,
            TwilioProgrammableChatError::ErrorCode50422 => r#"## ERROR - 50422

### Non-chat conversation participants limit exceeded

The total number of non-chat participants in this conversation exceeds the allowed limit. 

#### Possible Causes
You are trying to add more non-chat participants than allowed in this conversation. Maximum number of conversation participants is 50.
* You are trying to add more non-Chat participants than allowed in this conversation. [Maximum number](https://www.twilio.com/docs/conversations/conversations-limits) of conversation participants is 50.

#### Possible Solutions
* Retrieve a [list of current participants](https://www.twilio.com/docs/conversations/api/conversation-participant-resource#read-multiple-conversationparticipant-resources) in this conversation and [remove](https://www.twilio.com/docs/conversations/api/conversation-participant-resource#delete-a-conversationparticipant-resource) the ones that are no longer required."#,
            TwilioProgrammableChatError::ErrorCode50003 => r#"## Error - 50003

### Programmable Chat: Friendly name too long

#### Possible Causes 
*  Specified friendly name is too long

#### Possible Solutions
*  Confirm a valid FriendlyName is being passed in request. "#,
            TwilioProgrammableChatError::ErrorCode50324 => r#"## Error - 50324

### Programmable Chat: Invalid channel webhook type

#### Possible Causes 
*  Request does not contain correctly formatted parameter for channel webhook type.

#### Possible Solutions
*  Confirm a valid parameter is passed in request. "#,
            TwilioProgrammableChatError::ErrorCode50421 => r#"## ERROR - 50421

### Invalid last read message index format

Request does not contain correctly formatted parameter for last read message index 

#### Possible Causes
Request does not contain correctly formatted parameter for last read message index

#### Possible Solutions
Confirm a valid parameter is passed in request."#,
            TwilioProgrammableChatError::ErrorCode50361 => r#"
## Error - 50361

### Conversations: Too many conversation webhooks
The limit for conversation-scoped webhooks is five; this request would have created a sixth, and is thus rejected.

#### Possible Causes
You may have forgotten to remove other webhooks before adding this one. Alternatively, your use-case may include scenarios requiring more than five.

#### Possible Solutions
* Retrieve a list of the [existing webhooks](https://www.twilio.com/docs/conversations/api/conversation-scoped-webhook-resource#read-multiple-conversationscopedwebhook-resources) for this conversation and [remove](https://www.twilio.com/docs/conversations/api/conversation-scoped-webhook-resource#delete-a-conversationscopedwebhook-resource) the ones that are redundant or obsolete.
* Consider combining some of the [existing webhooks](https://www.twilio.com/docs/conversations/api/conversation-scoped-webhook-resource#read-multiple-conversationscopedwebhook-resources) and forward events to a single destination.
* Consider using [global webhooks](https://www.twilio.com/docs/conversations/api/conversation-webhook-resource) instead of conversation-scoped hooks.
"#,
            TwilioProgrammableChatError::ErrorCode50364 => r#"
## Error - 50364

### Conversations: Invalid conversation webhook type
An invalid conversation webhook target was supplied.

#### Possible Causes
* Webhook Target parameter provided with the request does not match any of valid targets: webhook, studio, trigger.

#### Possible Solutions
* Add a valid Target parameter to the webhook API request and supply one of the supported targets.

"#,
            TwilioProgrammableChatError::ErrorCode50415 => r#"
## Error - 50415

### Conversations: Proxy address is not WhatsApp enabled sender
The requested participant proxy address is not a correctly configured WhatsApp number.

#### Possible Causes
* Provided MessagingBinding.ProxyAddress parameter contains a participant number that is not recognized as a valid WhatsApp number.

#### Possible Solutions
* Ensure that MessagingBinding.ProxyAddress is an [on-boarded WhatsApp number](https://www.twilio.com/docs/sms/whatsapp/api) assigned to your number pool.

"#,
            TwilioProgrammableChatError::ErrorCode50212 => r#"## ERROR - 50212

### Programmable Chat: User already invited

User with specified Identity already invited to this Channel 

#### Possible Causes
User with specified Identity already invited to this Channel.

#### Possible Solutions
Confirm that the Identity passed in the request is not already invited to the intended channel."#,
            TwilioProgrammableChatError::ErrorCode50600 => r#"## Error - 50600

### Programmable Chat: Invite SID not provided

#### Possible Causes 
*  Request does not contain required Invite SID parameter.

#### Possible Solutions
*  Confirm a valid Invite SID is being passed in request. "#,
            TwilioProgrammableChatError::ErrorCode50203 => r#"## Error - 50203

### Programmable Chat: Identity reserved	 

#### Possible Causes 
*  Specified identity is reserved for internal use.

#### Possible Solutions
*  Choose different valid identity and provide it in request. "#,
            TwilioProgrammableChatError::ErrorCode50056 => r#"## Error - 50056

### Programmable Chat: Webhook disabled processing of command

#### Possible Causes 
*  Defined webhook returned other than "200 OK" HTTP code and Programmable Chat Service disabled processing of command.

#### Possible Solutions
*  Confirm that defined webhooks are processing Programmable Chat Service's webhook calls correctly. "#,
            TwilioProgrammableChatError::ErrorCode50403 => r#"## Error - 50403

### Programmable Chat: Channel member limit exceeded

#### Possible Causes 
* Channel has too many members

#### Possible Solutions
*  Increase channel member limit for service instance
*  Remove members from channel"#,
            TwilioProgrammableChatError::ErrorCode50420 => r#"## ERROR - 50420

### Participant Messaging Binding type does not support all of the provided Messaging Binding parameters

Sms Participant should not have MessagingBinding.Name or MessagingBinding.Level 

#### Possible Causes
Participant is being created with not supported MessagingBinding parameters. Sms Participant should not have MessagingBinding.Name or MessagingBinding.Level.

#### Possible Solutions
Remove MessagingBinding.Name and MessagingBinding.Level from the request."#,
            TwilioProgrammableChatError::ErrorCode50321 => r#"## Error - 50321

### Programmable Chat: Too many channel webhooks

#### Possible Causes 
*  Webhooks limit exceeded for given channel.

#### Possible Solutions
*  Reduce number of webhooks for channel before creating new one."#,
            TwilioProgrammableChatError::ErrorCode50502 => r#"## Error - 50502

### Programmable Chat: Message index not provided

#### Possible Causes 
*  Request does not contain required Message Index parameter.

#### Possible Solutions
*  Confirm a valid Message Index is being passed in request. "#,
            TwilioProgrammableChatError::ErrorCode19038 => r#"## ERROR - 19038

### Page token must be bigger than or equal to 0

 

#### Possible Causes
Get chat messages PageToken is smaller than 0

#### Possible Solutions
Make sure PageToken query parameter is not smaller than 0"#,
            TwilioProgrammableChatError::ErrorCode50401 => r#"## Error - 50401

### Programmable Chat: Member SID not provided

#### Possible Causes 
*  Request does not contain required Member SID parameter. 

#### Possible Solutions
*  Confirm a valid Member SID is being passed in request. "#,
            TwilioProgrammableChatError::ErrorCode50507 => r#"## Error - 50507

### Programmable Chat: Media not found	

#### Possible Causes 
*  Media with specified SID does not exist. 

#### Possible Solutions
*  Confirm a valid Media SID is being passed in request. "#,
            TwilioProgrammableChatError::ErrorCode50206 => r#"## Error - 50206

### Programmable Chat: Identity should not match user SID pattern

#### Possible Causes 
*  Specified User's Identity matches User SID Pattern "US[0-9a-f]{32}".

#### Possible Solutions
*  Confirm a valid User's Identity is being passed in request. "#,
            TwilioProgrammableChatError::ErrorCode50435 => r#"## ERROR - 50435

### Group MMS activation failed.

Group MMS activation with the new list of participants has failed. The conversation's Group MMS could not be activated.

#### Possible Causes
* The conversation does not have at least three participants with messaging binding.
* The conversation has got a participant with identity but without projected address.
* The conversation's participants do not make a valid address list for group messaging.

#### Possible Solutions
* Check that all the participants have messaging bindings.
* Check that there is at least one participant with user address only.
* Check that there is at least one participant with a proxy address or a projected address."#,
            TwilioProgrammableChatError::ErrorCode50303 => r#"## Error - 50303

### Programmable Chat: Attributes too long

#### Possible Causes 
*  Specified Channel's Attributes parameter is too long.

#### Possible Solutions
*  Confirm a valid length Channel's Attributes is being passed in request. "#,
            TwilioProgrammableChatError::ErrorCode50078 => r#"## ERROR - 50078

### Invalid post-webhook url

Request does not contain a valid url for post-webhook. 

#### Possible Causes
*  Provided url has the wrong format.

#### Possible Solutions
*  Make sure that url is complete and properly encoded.
*  Make sure that url contains valid protocol and hostname and doesn't contain invalid characters."#,
            TwilioProgrammableChatError::ErrorCode50379 => r#"## ERROR - 50379

### TimeToInactive format is invalid

'TimeToInactive' parameter format should be ISO8601 duration 'TimeToInactive' parameter supports values which are ISO8601 durations.

#### Possible Causes
* 'TimeToInactive' parameter value is not ISO8601 duration.

#### Possible Solutions
* Use ISO8601 duration format value for 'TimeToInactive' parameter. For example PT0S, PT10M, P1D etc.."#,
            TwilioProgrammableChatError::ErrorCode50350 => r#"
## Error - 50350

### Conversations: Conversation not found
The requested conversation SID was not found.

#### Possible Causes
The conversation SID you are requesting does not exist under your account.

#### Possible Solutions
* Check that you have provided a Conversation SID and not some other SID (should have the prefix CH).
* Check that the requested conversation SID belongs to the account that your REST request has targeted.

"#,
            TwilioProgrammableChatError::ErrorCode50060 => r#"## Error - 50060

### Programmable Chat: Invalid user channels limit format

#### Possible Causes 
*  Request does not contain correctly formatted parameter for user channels limit.

#### Possible Solutions
*  Confirm a valid user channels limit parameter is passed in request. "#,
            TwilioProgrammableChatError::ErrorCode50331 => r#"## Error - 50331

### Programmable Chat: Channel webhook trigger not provided

#### Possible Causes 
*  Request does not contain parameter for channel webhook trigger.

#### Possible Solutions
*  Confirm a valid parameter is passed in request. "#,
            TwilioProgrammableChatError::ErrorCode50329 => r#"## Error - 50329

### Programmable Chat: Channel webhook url not provided

#### Possible Causes 
*  Request does not contain parameter for channel webhook url.

#### Possible Solutions
*  Confirm a valid parameter is passed in request. "#,
            TwilioProgrammableChatError::ErrorCode50441 => r#"## ERROR - 50441

### Failed to add a proxy address to a participant.

The participant should have an address in order to add a proxy address. You cannot add a proxy address to the participant.

#### Possible Causes
* The participant to which you are trying to add a proxy address, does not have a user address.

#### Possible Solutions
* Check that the request does not update proxy address of this particular participant."#,
            TwilioProgrammableChatError::ErrorCode50391 => r#"## ERROR - 50391

### Conflicting conversation modification

The requested conversation is already being added or removed by a concurrent API request. The requested conversation is already being added or removed from this service instance concurrently with another API request. The last request will be rejected to avoid inconsistent state.

#### Possible Causes
* You might be adding a new conversation to the service instance while the conversation with the same unique name is being removed from another thread.
* You might be submitting multiple concurrent requests to add a conversation with the same unique name to the service instance.
* You might be submitting multiple concurrent requests to delete the same conversation by SID or unique name from the service instance.

#### Possible Solutions
* Implement an operation retrier and repeat the failed conversation API requests after an interval of time, using an exponential backoff algorithm.
* Review your application logic that caused the race condition when adding or removing users. Perhaps the conflicting operations are happening in a loop that could be avoided.
* Serialize your API requests that add or remove conversations. Wait until the original request completes and returns an API response, before sending any successive conversation requests."#,
            TwilioProgrammableChatError::ErrorCode50106 => r#"## Error - 50106

### Programmable Chat: Channel creator role not found

#### Possible Causes 
*  Valid Channel Creator Role does not exist in this service deployment.

#### Possible Solutions
*  Confirm a valid Channel Creator Role exists in this service deployment. "#,
            TwilioProgrammableChatError::ErrorCode50368 => r#"
## Error - 50368

### Conversations: Conversation webhook type not provided
Conversation webhook target parameter is required but was not provided.

#### Possible Causes
* The API expected a Target input parameter, but it was either not provided or was misspelled.

#### Possible Solutions
* Add a valid Target parameter to the API request and confirm that it is not misspelled.

"#,
            TwilioProgrammableChatError::ErrorCode50214 => r#"## ERROR - 50214

### User conversation limit exceeded

 

#### Possible Causes
User has joined too many conversations

#### Possible Solutions
*  Increase user conversation limit for service instance
*  Remove user from conversations"#,
            TwilioProgrammableChatError::ErrorCode50512 => r#"
## Error - 50512

### Conversations: Message media size is too large
The requested multimedia attachment size exceeds the allowed limit.

#### Possible Causes
* The size of multimedia attachment you are trying to send is not acceptable for this conversation.

#### Possible Solutions
* Make sure that the size of multimedia attachments co form to channel limitations. Check for media size constraints in [API documentation](https://www.twilio.com/docs/conversations/conversations-limits#media-limits).

"#,
            TwilioProgrammableChatError::ErrorCode50326 => r#"## Error - 50326

### Programmable Chat: Invalid channel webhook trigger

#### Possible Causes 
*  Request does not contain correct parameter for channel webhook trigger.

#### Possible Solutions
*  Confirm a valid parameter is passed in request. "#,
            TwilioProgrammableChatError::ErrorCode50405 => r#"## Error - 50405

### Programmable Chat: Invalid last consumption timestamp format

#### Possible Causes 
*  Request does not contain correctly formatted parameter for last consumption timestamp

#### Possible Solutions
*  Confirm a valid parameter is passed in request. "#,
            TwilioProgrammableChatError::ErrorCode50006 => r#"## ERROR - 50006

### Programmable Chat: Invalid Account SID

Specified Account SID does not match pattern "AC[0-9a-f]{32}" 

#### Possible Causes
Account SID in request does not match pattern "AC[0-9a-f]{32}".

#### Possible Solutions
Confirm a valid Account SID is being passed in request and matches pattern "AC[0-9a-f]{32}". "#,
            TwilioProgrammableChatError::ErrorCode50209 => r#"## Error - 50209

### Programmable Chat: Invalid binding type

#### Possible Causes 
*  Specified binding type does not exist.

#### Possible Solutions
*  Confirm a valid binding type is being passed in request. "#,
            TwilioProgrammableChatError::ErrorCode50306 => r#"## Error - 50306

### Programmable Chat: Unique name should not match channel SID pattern

#### Possible Causes 
*  Specified Channel's Unique Name matches Channel SID Pattern "CH[0-9a-f]{32}".

#### Possible Solutions
*  Confirm a valid Channel's Unique Name is being passed in request. "#,
            TwilioProgrammableChatError::ErrorCode50508 => r#"## Error - 50508

### Programmable Chat: Media already sent with another message

#### Possible Causes 
*  Media with specified SID is already sent with another Chat message. 

#### Possible Solutions
*  Confirm a valid Media SID is being passed in request. "#,
            TwilioProgrammableChatError::ErrorCode20161 => r#"## ERROR - 20161

### Programmable Chat: Parameters are not specified for update request

 Update request doesn't contain any parameters

#### Possible Causes
* You may have passed parameters, but named them incorrectly.
* You may have forgotten to pass form-encoded parameters with your request

#### Possible Solutions
Compare your request with the documentation for this resource. Be sure you are submitting form-encoded parameters."#,
            TwilioProgrammableChatError::ErrorCode50400 => r#"## Error - 50400

### Programmable Chat: User not member of channel

#### Possible Causes 
*  Specified User is not member of this Channel.

#### Possible Solutions
*  Confirm a valid User who is Member of this Channel is being passed in request. "#,
            TwilioProgrammableChatError::ErrorCode50341 => r#"
## Error - 50341

### Conversations: Invalid messaging service SID
An invalid messaging service SID was supplied.

#### Possible Causes
The MessagingServiceSid provided with the request does not match a valid SID pattern (MGxx).

#### Possible Solutions
Verify that the SID provided is prefixed with MG and is not missing digits. Your messaging services can be enumerated at https://twilio.com/console/sms/services.
"#,
            TwilioProgrammableChatError::ErrorCode50407 => r#"
## Error - 50407

### Conversations: Invalid messaging binding address
An invalid participant public address was supplied.

#### Possible Causes
* Participant MessagingBinding.Address parameter provided with the request is not a valid participant address.

#### Possible Solutions
* Check that the length of MessagingBinding.Address parameter does not exceed 256 characters.
* Ensure that MessagingBinding.Address parameter contains an address with valid syntax (e.g. SMS phone numbers must follow [E.164 format](https://www.twilio.com/docs/glossary/what-e164)).

"#,
            TwilioProgrammableChatError::ErrorCode50433 => r#"
## Error - 50433

### Conversations: Participant already exists
The requested Chat participant is already mapped to a conversation.

#### Possible Causes
* You are attempting to add a Programmable Chat participant to a conversation that already has another participant with the same identity.

#### Possible Solutions
* [Retrieve a list](https://www.twilio.com/docs/conversations/api/conversation-participant-resource#read-multiple-conversationparticipant-resources) of existing participants in this conversation and remove the one with the same identity using [REST API](https://www.twilio.com/docs/conversations/api/conversation-participant-resource#delete-a-conversationparticipant-resource).
* Perhaps it's not an issue, and we can continue as-is. Instead of adding a new participant, reuse the existing one, potentially [update its attributes](https://www.twilio.com/docs/conversations/api/conversation-participant-resource#update-a-conversationparticipant-resource) if necessary.
"#,
            TwilioProgrammableChatError::ErrorCode50200 => r#"## Error - 50200

### Programmable Chat: User not found

#### Possible Causes 
*  Specified User does not exist.

#### Possible Solutions
*  Confirm a valid User is being passed in request. "#,
            TwilioProgrammableChatError::ErrorCode50402 => r#"## Error - 500402

### Programmable Chat: Member not found

#### Possible Causes 
*  Specified Member does not exist.

#### Possible Solutions
*  Confirm a valid Member is being passed in request. "#,
            TwilioProgrammableChatError::ErrorCode50432 => r#"
## Error - 50432

### Conversations: Participant not found
The requested conversation participant SID was not found.

#### Possible Causes
* The participant SID you are requesting does not exist in this conversation.

#### Possible Solutions
* Check that the requested participant SID is not misspelled.
* Check that the requested participant SID was added to this conversation. Use REST API to [fetch a list](https://www.twilio.com/docs/conversations/api/conversation-participant-resource#read-multiple-conversationparticipant-resources) of existing conversation participants.

"#,
            TwilioProgrammableChatError::ErrorCode50367 => r#"
## Error - 50367

### Conversations: Invalid conversation webhook flow SID
An invalid conversation webhook flow SID was supplied.

#### Possible Causes
* Webhook Configuration.FlowSid parameter provided with the request does not match a valid Studio flow SID pattern (FWxx).

#### Possible Solutions
* Add a valid Configuration.FlowSid parameter to the API request and supply a Studio flow SID that exists under your account.

"#,
            TwilioProgrammableChatError::ErrorCode50055 => r#"## Error - 50055

### Programmable Chat: Invalid webhook method

#### Possible Causes 
*  Request does not contain valid HTTP method for webhook.

#### Possible Solutions
*  Confirm a valid HTTP method for webhook is being passed in request. "#,
            TwilioProgrammableChatError::ErrorCode50061 => r#"## Error - 50061

### Programmable Chat: Invalid channel members limit format

#### Possible Causes 
*  Request does not contain correctly formatted parameter for channel members limit.

#### Possible Solutions
*  Confirm a valid channel members limit parameter is passed in request. "#,
            TwilioProgrammableChatError::ErrorCode50004 => r#"## Error - 50004

### Programmable Chat: Unique Name too long

#### Possible Causes 
*  Specified unique name is too long

#### Possible Solutions
*  Confirm a valid UniqueName is being passed in request. "#,
            TwilioProgrammableChatError::ErrorCode50610 => r#"## WARNING - 50610

### Address configuration deleted

Address configuration was deleted which affects auto-create functionality 

#### Possible Causes
The Twilio phone number associated with the address configuration was either released or ported out

#### Possible Solutions
 "#,
            TwilioProgrammableChatError::ErrorCode50305 => r#"## Error - 50305

### Programmable Chat: Channel SID not provided

#### Possible Causes 
*  Request does not contain required Channel SID parameter.

#### Possible Solutions
*  Confirm a valid Channel SID is being passed in request. "#,
            TwilioProgrammableChatError::ErrorCode50322 => r#"## Error - 50322

### Programmable Chat: Too many channel webhook triggers

#### Possible Causes 
*  Triggers limit exceeded for given channel webhook.

#### Possible Solutions
*  Reduce number of triggers for given channel webhook before creating new one."#,
            TwilioProgrammableChatError::ErrorCode50334 => r#"## ERROR - 50334

### Channel deletion operation is in progress

Channel deletion operation is in progress, all the channel modifications are discarded. 

#### Possible Causes
* Your API request tried to update the channel while the channel deletion operation was in progress.

#### Possible Solutions
* Do not try to update the channel while the deletion operation is in progress."#,
            TwilioProgrammableChatError::ErrorCode50406 => r#"## Error - 50406

### Programmable Chat: Invalid last consumed message index format

#### Possible Causes 
*  Request does not contain correctly formatted parameter for last consumed message index

#### Possible Solutions
*  Confirm a valid parameter is passed in request. "#,
            TwilioProgrammableChatError::ErrorCode50050 => r#"## Error - 50050

### Programmable Chat: Service Instance not found

#### Possible Causes 
*  Specified Service Instance does not exist.

#### Possible Solutions
*  Confirm a valid Service Instance is being passed in request. "#,
            TwilioProgrammableChatError::ErrorCode50349 => r#"## ERROR - 50349

### Conflicting channel modification

The requested channel is already being added or removed by a concurrent API request. The requested channel is already being added or removed from this service instance concurrently with another API request. The last request will be rejected to avoid inconsistent state.

#### Possible Causes
* You might be adding a new channel to the service instance while the channel with the same unique name is being removed from another thread.
* You might be submitting multiple concurrent requests to add a channel with the same unique name to the service instance.
* You might be submitting multiple concurrent requests to delete the same channel by SID or unique name from the service instance.

#### Possible Solutions
* Implement an operation retrier and repeat the failed channel API requests after an interval of time, using an exponential backoff algorithm.
* Review your application logic that caused the race condition when adding or removing users. Perhaps the conflicting operations are happening in a loop that could be avoided.
* Serialize your API requests that add or remove channels. Wait until the original request completes and returns an API response, before sending any successive channel requests."#,
            TwilioProgrammableChatError::ErrorCode50503 => r#"## Error - 50503

### Programmable Chat: Message body not provided

#### Possible Causes 
*  Request does not contain required Message Body parameter.

#### Possible Solutions
*  Confirm a valid Message Body is being passed in request. "#,
            TwilioProgrammableChatError::ErrorCode50109 => r#"## ERROR - 50109

### Conversation role not found

Specified Conversation Role does not exist. 

#### Possible Causes
Specified Conversation Role does not exist.

#### Possible Solutions
Confirm a valid Conversation Role is being passed in request."#,
            TwilioProgrammableChatError::ErrorCode50065 => r#"## Error - 50065

### Programmable Chat: Invalid webhook retry count

#### Possible Causes 
* Request does not contain correctly formatted parameter for webhook retry count.

#### Possible Solutions
*  Confirm a valid webhook parameters are being passed in request. "#,
            TwilioProgrammableChatError::ErrorCode50443 => r#"## ERROR - 50443

### Failed to add a projected address to a participant.

The participant should not have a messaging binding in order to add a projected address. The conversation participant being updated can not have a projected address.

#### Possible Causes
* You are trying to add a projected address to a participant with messaging binding assigned.

#### Possible Solutions
* Check that the request does not update projected address of this particular participant."#,
            TwilioProgrammableChatError::ErrorCode50342 => r#"
## Error - 50342

### Conversations: Messaging service does not belong to account
The requested messaging service SID does not belong to your account.

#### Possible Causes
You may have copied/pasted incorrectly, or your API request targeted a different account (e.g. if you have several Projects or several Twilio accounts).

#### Possible Solutions
Double-check the Messaging Service SID, confirming that it belongs to your account. Check whether you are authenticating against Twilio with the same account. With these matching, the request will succeed.

"#,
            TwilioProgrammableChatError::ErrorCode50076 => r#"## ERROR - 50076

### Webhook failed to execute successfully due to timeout

Webhook's response isn't received in 5 seconds Programmable Chat: Webhook failed to execute successfully due to timeout

#### Possible Causes
*  Application side processing is exceeding 5 seconds.
*  Some network blips might cause delay in returning the response to chat

#### Possible Solutions
*  Make sure that webhook processing time is less than 5 seconds
*  Make webhook processing asynchronous on the app side and return response immediately "#,
            TwilioProgrammableChatError::ErrorCode50390 => r#"## ERROR - 50390

### Unique name should not match conversation sid pattern

Specified Conversation's Unique Name matches Conversation SID Pattern "CH[0-9a-f]{32}". 

#### Possible Causes
Specified Conversation's Unique Name matches Conversation SID Pattern "CH[0-9a-f]{32}".

#### Possible Solutions
Confirm a valid Conversation's Unique Name is being passed in request."#,
            TwilioProgrammableChatError::ErrorCode50376 => r#"## ERROR - 50376

### TimeToClosed should be greater or equal to 10 minutes

Duration to transfer channel to closed state can't be shorter than 10 minutes. 'TimeToClosed' duration should be greater or equal to 10 minutes.

#### Possible Causes
* 'TimeToClosed' field is set to duration which is less than 10 minutes.

#### Possible Solutions
* Set 'TimeToClosed' field to duration which is longer than 10 minutes."#,
            TwilioProgrammableChatError::ErrorCode50366 => r#"
## Error - 50366

### Conversations: Invalid conversation webhook trigger
An invalid conversation webhook trigger keyword was supplied.

#### Possible Causes
* One of Configuration.Triggers keywords provided with the request is an empty string.
* One of Configuration.Triggers keywords provided with the request exceeds the maximum length limit of 248 characters.

#### Possible Solutions
* Ensure that a list of valid Configuration.Triggers keywords are passed to the API request.

"#,
            TwilioProgrammableChatError::ErrorCode50392 => r#"## ERROR - 50392

### PreWebhookTimeout should be greater than 0ms

Duration of waiting for pre-webhook response cannot be 0ms or less. 'PreWebhookTimeout' should be greater than 0ms.

#### Possible Causes
* 'PreWebhookTimeout' field is set with a value less than or equal to 0.

#### Possible Solutions
* set 'PreWebhookTimeout' field with a value greater than 0."#,
            TwilioProgrammableChatError::ErrorCode50500 => r#"## Error - 50500

### Programmable Chat: Message not found	

#### Possible Causes 
*  Specified Message does not exist.

#### Possible Solutions
*  Confirm a valid Message is being passed in request. "#,
            TwilioProgrammableChatError::ErrorCode50002 => r#"## Error - 50002

### Programmable Chat: Resource is being deleted

#### Possible Causes 
*  The resource your request relates to is currently being deleted.

#### Possible Solutions
*  Try the same request again later."#,
            TwilioProgrammableChatError::ErrorCode50308 => r#"## Error - 50308

### Programmable Chat: Invalid Date Created parameter

#### Possible Causes 
*  Request does not contain correctly formatted parameter for Date Created.

#### Possible Solutions
*  Confirm a valid Date Created parameter is passed in request. "#,
            TwilioProgrammableChatError::ErrorCode50103 => r#"## Error - 50103

### Programmable Chat: Role SID not provided

#### Possible Causes 
*  Request does not contain required Role SID parameter.

#### Possible Solutions
*  Confirm a valid Role SID is being passed in request. "#,
            TwilioProgrammableChatError::ErrorCode50504 => r#"## ERROR - 50504

### Programmable Chat: Message body parameter is too long

Message body parameter is too long ## Error - 50504

### Programmable Chat: Message body parameter is too long

#### Possible Causes
Message body parameter is too long. Visit the [Chat Limits](https://www.twilio.com/docs/chat/chat-limits) documentation to learn more about the maximum message length. 

#### Possible Solutions
Confirm a valid length Message body is being passed in the request. Additionally, consider intercepting messages with [Event Webhooks](https://www.twilio.com/docs/chat/webhook-events#webhook-bodies-pre-event) to validate or reject messages."#,
            TwilioProgrammableChatError::ErrorCode50307 => r#"## Error - 50307

### Programmable Chat: Channel with provided unique name already exists	

#### Possible Causes 
*  Specified Channel's Unique Name already exists in this Service deployment.

#### Possible Solutions
*  Confirm a valid non-existing Channel's Unique Name is being passed in request. "#,
            TwilioProgrammableChatError::ErrorCode50300 => r#"## Error - 50300

### Programmable Chat: Channel not found

#### Possible Causes 
*  Specified Channel does not exist.

#### Possible Solutions
*  Confirm a valid Channel is being passed in request. "#,
            TwilioProgrammableChatError::ErrorCode50409 => r#"
## Error - 50409

### Conversations: Participant address equals proxy address
The supplied participant address must not equal its proxy address.

#### Possible Causes
* When requesting a participant to be added to a conversation, you specified the identical participant address and its proxy address parameters.

#### Possible Solutions
* Provide a participant's public address (e.g. user's SMS/MMS number) different from a conversation proxy number (e.g. Twilio SMS/MMS number) when adding a participant to the conference.

"#,
            TwilioProgrammableChatError::ErrorCode50325 => r#"## Error - 50325

### Programmable Chat: Invalid channel webhook filter

#### Possible Causes 
*  Request does not contain correct parameter for channel webhook filter.

#### Possible Solutions
*  Confirm a valid parameter is passed in request. "#,
            TwilioProgrammableChatError::ErrorCode50404 => r#"### Programmable Chat: Member already exists

#### Possible Causes 

*  Member with specified Identity already exists in this Channel.

#### Possible Solutions

*  Confirm that the Member Identity passed in the request is not already a member of the intended channel."#,
            TwilioProgrammableChatError::ErrorCode50323 => r#"## Error - 50323

### Programmable Chat: Invalid channel webhook sid

#### Possible Causes 
*  Request does not contain correctly formatted parameter for channel webhook sid

#### Possible Solutions
*  Confirm a valid parameter is passed in request. "#,
            TwilioProgrammableChatError::ErrorCode50063 => r#"## Error - 50063

### Programmable Chat: Actions per second limit exceeded

#### Possible Causes 
*  Your application is generating too many actions per second (APS) for Chat service instance
*  Note that the APS limit is enforced for commands such as sending a Message or creating a Channel - not Read actions

#### Possible Solutions
*  Make sure your application (client or backend) uses a good exponential back-off algorithm like the [one advocated by Amazon](https://www.awsarchitectureblog.com/2015/03/backoff.html) to retry on HTTP 429 (rate limiting) responses
*  Reduce the rate of requests and introduce basic rate control, e.g. for user or channel provisioning use-cases
*  Reduce number of concurrent requests
"#,
            TwilioProgrammableChatError::ErrorCode50360 => r#"
## Error - 50360

### Conversations: Conversation webhook not found
The requested webhook SID was not found.

#### Possible Causes
* The conversation webhook SID you are requesting does not exist in that conversation.

#### Possible Solutions
* Check that the requested webhook SID is actually a Webhook SID, and is not misspelled (prefix should be WH).
* Check that the requested webhook SID belongs to the targeted conversation.
"#,
            TwilioProgrammableChatError::ErrorCode50444 => r#"## ERROR - 50444

### Failed to add an identity to a participant.

The participant should not have an identity in order to add an identity. The conversation participant being updated can not have an identity.

#### Possible Causes
* You are trying to add an identity to a participant with identity already assigned.
* You are trying to add an identity to a participant without projected address.

#### Possible Solutions
* Check that the request does not update identity of this particular participant."#,
            TwilioProgrammableChatError::ErrorCode50302 => r#"## Error - 50302

### Programmable Chat: Unknown channel command

#### Possible Causes 
*  Specified Channel Command does not exist.

#### Possible Solutions
*  Confirm a valid Channel Command is being passed in request. "#,
            TwilioProgrammableChatError::ErrorCode50102 => r#"## Error - 50102

### Programmable Chat: Deployment role not found	

#### Possible Causes 
*  Specified Deployment Role does not exist. 

#### Possible Solutions
*  Confirm a valid Deployment Role is being passed in request. "#,
            TwilioProgrammableChatError::ErrorCode50411 => r#"
## Error - 50411

### Conversations: Participant address is empty
The provided participant public address parameter must not be empty.

#### Possible Causes
* MessagingBinding.Address parameter provided with the request contains an empty string.

#### Possible Solutions
* Add a non-empty MessagingBinding.Address parameter to the API request and supply a valid participant address matching your desired channel, e.g. SMS, MMS or WhatsApp.

"#,
            TwilioProgrammableChatError::ErrorCode50372 => r#"
## Error - 50372

### Conversations: Conversation webhook URL is too long
The length of provided conversation webhook URL exceeds the limit.

#### Possible Causes
* Your API request contains a parameter containing the URL that is longer than 512 characters.

#### Possible Solutions
* Provide a shorter webhook URL with your API request.

"#,
            TwilioProgrammableChatError::ErrorCode50430 => r#"
## Error - 50430

### Conversations: Participant is not a member of conversation
The requested participant is not a member of this conversation.

#### Possible Causes
* The requested participant was removed from this conversation, or was never added to it.
* The requested participant SID is misspelled.
* The requested participant is a member of a different conversation.

#### Possible Solutions
* Ensure that the participant is added to the conversation. Use API to [create a new participant](https://www.twilio.com/docs/conversations/api/conversation-participant-resource#add-a-conversation-participant-sms).

"#,
            TwilioProgrammableChatError::ErrorCode50000 => r#"## Error - 50000

### Programmable Chat: FriendlyName not provided

#### Possible Causes 
*  Request does not contain required "FriendlyName" parameter.

#### Possible Solutions
*  Confirm a valid FriendlyName is being passed in request. "#,
            TwilioProgrammableChatError::ErrorCode50601 => r#"## Error - 50601

### Programmable Chat: Invite not found

#### Possible Causes 
*  Specified Invite does not exist. 

#### Possible Solutions
*  Confirm a valid Invite is being passed in request. "#,
            TwilioProgrammableChatError::ErrorCode50439 => r#"## ERROR - 50439

### Account is not authorized to use the projected address

Address used as Projected Address does not belong to the account. Address used as Projected Address does not belong to the account.

#### Possible Causes
* The account does not own the Address (eg. phone number) used as a Projected Address.

#### Possible Solutions
* Use a different address owned by the account."#,
            TwilioProgrammableChatError::ErrorCode50510 => r#"
## Error - 50510

### Conversations: Invalid message media SID
An invalid multimedia attachment SID was supplied for this message.

#### Possible Causes
* MediaSid parameter provided with the request does not match a valid media SID pattern (MExx).

#### Possible Solutions
* Add a valid MediaSid parameter to the API request and supply a media SID that is uploaded under your service instance.

"#,
            TwilioProgrammableChatError::ErrorCode50347 => r#"
## Error - 50347

### Programmable Chat: Parameters are missing for channel update request
There are no parameters specified in this channel update request.

#### Possible Causes
* No parameters were set in the channel update request.
* Parameters specified in the channel update API request have incorrect names.

#### Possible Solutions
* Verify that at least one parameter is provided to the channel update API.
* Validate that parameter names are correct according to the [API documentation](https://www.twilio.com/docs/chat/rest/channel-resource#update-a-channel-resource).
"#,
            TwilioProgrammableChatError::ErrorCode50210 => r#"## Error - 50210

### Programmable Chat: Invalid notification level

#### Possible Causes 
* Specified notification level does not exist.

#### Possible Solutions
* Confirm a valid notification level is being passed in request. "#,
            TwilioProgrammableChatError::ErrorCode50501 => r#"## Error - 50501

### Programmable Chat: Message SID not provided	

#### Possible Causes 
*  Request does not contain required Message SID parameter.

#### Possible Solutions
*  Confirm a valid Message SID is being passed in request. "#,
            TwilioProgrammableChatError::ErrorCode50001 => r#"## Error - 50001

### Programmable Chat: Account SID not provided 

#### Possible Causes 
*  Request does not contain required Account SID parameter.

#### Possible Solutions
*  Confirm a valid Account SID is being passed in request."#,
            TwilioProgrammableChatError::ErrorCode50505 => r#"## Error - 50505

### Programmable Chat: Last Updated By parameter is too long

#### Possible Causes 
*  Last Updated By parameter is too long.

#### Possible Solutions
*  Confirm a valid length Last Updated By is being passed in request. "#,
            TwilioProgrammableChatError::ErrorCode50365 => r#"
## Error - 50365

### Conversations: Invalid conversation webhook filter
An invalid conversation webhook filter was supplied.

#### Possible Causes
* One of Configuration.Filters webhook parameters provided with the request does represent a valid webhook action filter.

#### Possible Solutions
* Ensure that a list of valid Configuration.Filters parameters are passed to the API request, refer to the list of [supported actions](https://www.twilio.com/docs/conversations/conversations-webhooks#webhook-action-triggers).

"#,
            TwilioProgrammableChatError::ErrorCode50301 => r#"## Error - 50301

### Programmable Chat: Channel key not provided	

#### Possible Causes 
*  Request does not contain required Channel SID or Channel's Unique Name parameters.

#### Possible Solutions
*  Confirm a valid Channel SID or Channel's Unique Name is being passed in request. "#,
            TwilioProgrammableChatError::ErrorCode50385 => r#"## ERROR - 50385

### Conversations are disabled in this region

Conversations API is not supported in this Twilio region Conversations features do not function in the region you've selected. The API will return this error to indicate that, preventing further activity.

#### Possible Causes
* You may be using a Programmable Chat service that in the ie1 region (at chat.ie1.twilio.com). While Conversations is backwards compatible with Chat in other respects, the product presently works only in the default region (US East).
* You may have inadvertently specified a region (other than the default) in your Conversations SDKs.

#### Possible Solutions
If you need access to Conversations hosted in a Twilio datacenter outside the US, please reach out to Twilio Support through the [Console](https://console.twilio.com) or [Help Center](https://support.twilio.com) so we can prioritize it accordingly. In the meantime, use the default Twilio URLs (e.g. conversations.twilio.com)."#
        }
    }
}
