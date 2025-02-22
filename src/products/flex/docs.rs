// AUTO-GENERATED. DO NOT MODIFY. YOUR CHANGES WILL BE OVER-WRITTEN
use crate::products::TwilioFlexError;
use standard_error::traits::StandardErrorDocsTrait;

impl StandardErrorDocsTrait for TwilioFlexError {
    fn docs(&self) -> &'static str {
        match self {
            TwilioFlexError::ErrorCode45357 => r#"##  - 45357

### 

 

#### Possible Causes


#### Possible Solutions
"#,
            TwilioFlexError::ErrorCode45784 => r#"## ERROR - 45784

### Internal Error. Failed to get deployment keys.

 An unexpected error occurred when processing get all deployment keys request.

#### Possible Causes
We encountered an unexpected error when processing get all deployment keys request.

#### Possible Solutions
Retry the request.

Check our status page to see if we are having an outage.

If our status page has no information, contact support with details about the requests that are failing, and we will investigate."#,
            TwilioFlexError::ErrorCode45796 => r#"## WARNING - 45796

### Failed to perform this operation. There  was a conflict with the state of the  resource.

 Failed to perform this operation. There  was a conflict with the state of the  resource.

#### Possible Causes
Conflict in the state of target resource possibly because of concurrent requests.

#### Possible Solutions
Retry the request."#,
            TwilioFlexError::ErrorCode45770 => r#"## ERROR - 45770

### Failed to create InteractionChannel. Downstream Error. 

 An error occurred while creating an interactionChannel. Downstream Error.

#### Possible Causes
We encountered an unexpected error while processing createInteractionChannel Request, Downstream Error.

#### Possible Solutions
"Retry the request.

Check our status page to see if we are having an outage.

If our status page has no information, contact support with details about the requests that are failing, and we will investigate.""#,
            TwilioFlexError::ErrorCode45782 => r#"## ERROR - 45782

### Internal Error. Failed to delete deployment key.

 An unexpected error occurred when processing delete deployment key request.

#### Possible Causes
We encountered an unexpected error when deleting deployment key. 

#### Possible Solutions
Retry the request.

Check our status page to see if we are having an outage.

If our status page has no information, contact support with details about the requests that are failing, and we will investigate."#,
            TwilioFlexError::ErrorCode49008 => r#"## ERROR - 49008

### Profile Connector Installation Not Found

 The value inside of the UniqueName field (legacy name) is not found to be associated with a known entity, either in the Twilio marketplace, or in our internal Profile Connector SID Database.

#### Possible Causes
The Profile Connector {Profile Connector Instance SID or Unique Name} was not found.

#### Possible Solutions
Ensure that you have the correct Profile Connector Instance SID."#,
            TwilioFlexError::ErrorCode45351 => r#"##  - 45351

### 

 

#### Possible Causes


#### Possible Solutions
"#,
            TwilioFlexError::ErrorCode45376 => r#"## WARNING - 45376

### Failed to add a participant.  Conversation was closed or not found.

 Failed to add Participant <participant_sid>. Conversation <conversation_sid> was closed or not found.

#### Possible Causes
We did not add a Participant to a Channel because the Channel has either been closed or deleted.

#### Possible Solutions
Ensure that the target Conversation is in the correct state."#,
            TwilioFlexError::ErrorCode90002 => r#"## ERROR - 90002

### Too Many Errors

Your Flex application has exceeded the publish rate limit of errors per second. Only a partial set of errors have been published instead. 

#### Possible Causes
* Your Flex Application is sending too many errors to the Twilio Debugger
* Your Flex Plugin is misconfigured causing a lot of errors to be published

#### Possible Solutions
* Ensure that you have put log messages appropriately in your plugin to reduce the number errors being published
* There could be errors outside of your plugins, within Twilio that are causing it to throw errors
* Use `try/catch` around methods and handle exceptions gracefully
* Use `.catch` for all HTTP requests to handle exceptions gracefully"#,
            TwilioFlexError::ErrorCode45308 => r#"## WARNING - 45308

### Add Participant Not Allowed

 Adding participant to the meeting is not allowed in the current meeting state

#### Possible Causes
Meeting is not in the right state for adding the participants

#### Possible Solutions
Retry operation"#,
            TwilioFlexError::ErrorCode45731 => r#"## ERROR - 45731

### Pre-engagement data too large. 

 Failed to create webchat conversation. Pre-engagement data too large. 

#### Possible Causes
Pre engagment data is over the maximum allowed size of 16KB. 

#### Possible Solutions
Ensure pre-engagement data is less than 16KB. 

Read more about creating valid attributes [here.](https://www.twilio.com/docs/conversations/attributes)"#,
            TwilioFlexError::ErrorCode45601 => r#"## ERROR - 45601

### Custom Flex UI error

 Error thrown by a user’s custom code/plugin. 

#### Possible Causes
* Custom code has been written to throw and report a Flex error event

#### Possible Solutions
* Fix custom code so that the error is not thrown
* Remove code which throws and reports a custom Flex error event"#,
            TwilioFlexError::ErrorCode45356 => r#"##  - 45356

### 

 

#### Possible Causes


#### Possible Solutions
"#,
            TwilioFlexError::ErrorCode45797 => r#"## WARNING - 45797

### Support Access Token Generated

 A support access token was generated for this account, providing read-only access to this member of Twilio Support to account data, including worker, task, interaction and configuration data. 

#### Possible Causes
A member of Twilio Support obtained customer approval to generate a support access token in order to provide support to the customer. 

#### Possible Solutions
This support access token will expire after 1hr."#,
            TwilioFlexError::ErrorCode45776 => r#"## ERROR - 45776

### Failed to fetch Configuration for deployment key.

 Failed to fetch Configuration for deployment key.

#### Possible Causes
An unexpected error occurred while fetching configuration associated with the given deployment key.

#### Possible Solutions
Retry the request.

Check our status page to see if we are having an outage.

If our status page has no information, contact support with details about the requests that are failing, and we will investigate."#,
            TwilioFlexError::ErrorCode45309 => r#"## ERROR - 45309

### Modify Participant Not Allowed

 Error occurred when trying to modify a participant

#### Possible Causes
Meeting is not setup successfully for the operation

#### Possible Solutions
Retry operation."#,
            TwilioFlexError::ErrorCode45373 => r#"##  - 45373

### 

 

#### Possible Causes


#### Possible Solutions
"#,
            TwilioFlexError::ErrorCode45101 => r#"## ERROR - 45101

### Configuration Not Found

 

#### Possible Causes
Queried configuration is not yet created for specified account.

#### Possible Solutions
Ensure, that configuration for specific account is created, create one if not."#,
            TwilioFlexError::ErrorCode45377 => r#"## ERROR - 45377

### Account SID not authorized. 

 The account sid provided is not authorized to make changes to this Channel.

#### Possible Causes
The request was not completed because the AccountSid provided does not match the credentials in our records. 

#### Possible Solutions
Ensure the AccountSid being used is the same as the one used to create the resource. "#,
            TwilioFlexError::ErrorCode45711 => r#"## WARNING - 45711

### Failed to create webchat participant. Unauthorized 

 Failed to create webchat participant. Credentials provided in request are invalid for the action. 

#### Possible Causes
Request is attempting to access resource where access is not allowed for given credentials.

#### Possible Solutions
Check that accessed resource is set correctly or apply for permissions."#,
            TwilioFlexError::ErrorCode45362 => r#"##  - 45362

### 

 

#### Possible Causes


#### Possible Solutions
"#,
            TwilioFlexError::ErrorCode45374 => r#"##  - 45374

### 

 

#### Possible Causes


#### Possible Solutions
"#,
            TwilioFlexError::ErrorCode45207 => r#"## ERROR - 45207

### Chat User per Chat Channel limit reached. User is part of too many Chat Channels.

 

#### Possible Causes
Flex Agents as part of standard Flex Orchestration are removed from Chat Channels once they are done working a specific Task related to a Channel. Possible cause to this error is that this orchestration is not working or Flex is unaware that the Custom Task Channel used has chat capabilities. 

#### Possible Solutions
Please check that Agents are not part of large number of inactive Channels/Tasks, review any custom programmability you may have in Flex UI (please review Flex Documentation). You can control the Chat Channels and Members through Programmable Chat API. You can also change the Channel Limit in Twilio Programmable Chat Console but keep in mind this cannot be extended indefinitely. "#,
            TwilioFlexError::ErrorCode45773 => r#"## ERROR - 45773

### Internal Error. Failed to create token.

 An unexpected error occurred when processing init token request.

#### Possible Causes
We encountered an unexpected error when creating token.

#### Possible Solutions
Retry the request.

Check our status page to see if we are having an outage.

If our status page has no information, contact support with details about the requests that are failing, and we will investigate."#,
            TwilioFlexError::ErrorCode45700 => r#"## ERROR - 45700

### Unexpected error occurred. Unable to process WebChat request.

 An unexpected error occurred when processing create WebChat conversation request. 

#### Possible Causes
We encountered an unexpected error while processing your create WebChat endpoint.

#### Possible Solutions
Retry the request.

Check our [status page](https://status.twilio.com/) to see if we are having an outage.

If our status page has no information, [contact support](https://www.twilio.com/help/contact) with details about the requests that are failing, and we will investigate."#,
            TwilioFlexError::ErrorCode45103 => r#"## ERROR - 45103

### Workspace Not Configured

 

#### Possible Causes
Workspace is not defined in the configuration.

#### Possible Solutions
Update the configuration: set the workspace."#,
            TwilioFlexError::ErrorCode45361 => r#"##  - 45361

### 

 

#### Possible Causes


#### Possible Solutions
"#,
            TwilioFlexError::ErrorCode45719 => r#"## WARNING - 45719

### Failed to create webchat participant. Too many requests 

 Failed to create participant for webchat conversation. Too many requests 

#### Possible Causes
Your account is sending too many concurrent Rest API requests to Twilio servers.

#### Possible Solutions
An API Request that receives an error 429 response is never processed and is always safe to retry. Please wait for a short period of time and make the request again, or alter your client's settings to issue fewer concurrent requests to the Twilio API."#,
            TwilioFlexError::ErrorCode45003 => r#"## ERROR - 45003

### Authorization Error

 

#### Possible Causes
Attempt to access resource where access is not allowed for given credentials.

#### Possible Solutions
Check that accessed resource is set correctly or apply for permissions."#,
            TwilioFlexError::ErrorCode45307 => r#"## WARNING - 45307

### Add Participant Not Allowed

 Adding participant to the meeting is not allowed in the current meeting state

#### Possible Causes
Meeting is not in the right state for adding the participants

#### Possible Solutions
Retry operation"#,
            TwilioFlexError::ErrorCode45355 => r#"##  - 45355

### 

 

#### Possible Causes


#### Possible Solutions
"#,
            TwilioFlexError::ErrorCode45371 => r#"##  - 45371

### 

 

#### Possible Causes


#### Possible Solutions
"#,
            TwilioFlexError::ErrorCode45762 => r#"## ERROR - 45762

### Failed to validate address configuration. Auto create type is empty or invalid. 

 Failed to validate address configuration associated with provided address SID. Auto create type is empty or invalid. 

#### Possible Causes
Address configuration associated with the provided address SID has an invalid or missing auto create type.  

The address configuration could be incorrect or out of date. 

#### Possible Solutions
Ensure that the address SID provided is valid and associated with the correct account.

Ensure the auto create type is present and set in the address configuration and is set to one of the valid types: webhook, studio, or default. More information on creating and updating an address configuration resource can be found [here.](https://www.twilio.com/docs/conversations/api/address-configuration-resource#addressconfiguration-properties)"#,
            TwilioFlexError::ErrorCode45203 => r#"## ERROR - 45203

### Requested Chat User not found

 

#### Possible Causes
Chat User related to a Chat Channel could not be found. 

#### Possible Solutions
Chat User is created automatically by Flex for any inbound end customer requests and automatically when an Agent connects to Flex where that Agent already does not have a Chat User. Chat User is synchronized to Identity Provider provided information (via SSO) on role and name. Please try again and ensure that the relevant Chat Users are not removed in your own implementations. "#,
            TwilioFlexError::ErrorCode45744 => r#"## ERROR - 45744

### Failed to create webchat conversation. A resource provided could not be found. 

 Failed to create webchat conversation for provided address SID. Resource not found

#### Possible Causes
A resource provided in create webchat conversation request could not be found by conversations API. 

#### Possible Solutions
Ensure the account credentials and address SID provided in the request are correct and are all associated with the intended account. 

Ensure there's a default messaging service configured.
Read more about setting up a service [here.](https://www.twilio.com/docs/conversations/fundamentals#configuring-default-conversation-services-and-messaging-services)

Read more about the create conversation request [here.](https://www.twilio.com/docs/conversations/api/conversation-resource#create-a-conversation-resource)

Read more about messaging service resource [here.](https://www.twilio.com/docs/messaging/services/api)"#,
            TwilioFlexError::ErrorCode45109 => r#"## ERROR - 45109

### Skills Limit Exceeded

 The number of Skills exceeds the limits.

#### Possible Causes
Application attempted adding skills beyond Maximum Limit

#### Possible Solutions
Delete some existing Skills or Contact Customer Support to increase your Skills Limit"#,
            TwilioFlexError::ErrorCode45010 => r#"## ERROR - 45010

### Rate Limit Exceeded

 The rate of access to this resource exceeds the prescribed limits.

#### Possible Causes
Your application is requesting Flex API resources at a rate that is higher than expected.

#### Possible Solutions
Verify that your application is not misbehaving and the load is expected. If everything is as expected then contact Customer Support to review your use case and limits."#,
            TwilioFlexError::ErrorCode45206 => r#"## WARNING - 45206

### Chat User is already a member of the Chat Channel

 

#### Possible Causes
The request could not be completed due to a conflict as the Chat User is already part of the desired Chat Channel. This is possibly related to duplicate requests to Twilio Flex.

#### Possible Solutions
No further action is necessary however please verify your business logic to avoid duplicate requests."#,
            TwilioFlexError::ErrorCode45701 => r#"## ERROR - 45701

### Unexpected error occurred. Service unavailable. 

 An unexpected error occurred when processing create WebChat request. Our service is currently unavailable.

#### Possible Causes
We encountered an unexpected error while processing your request. Our service is currently unavailable. 

#### Possible Solutions
Retry the request.

Check our [status page](https://status.twilio.com/) to see if we are having an outage.

If our status page has no information, [contact support](https://www.twilio.com/help/contact) with details about the requests that are failing, and we will investigate."#,
            TwilioFlexError::ErrorCode45367 => r#"##  - 45367

### 

 

#### Possible Causes


#### Possible Solutions
"#,
            TwilioFlexError::ErrorCode45381 => r#"## ERROR - 45381

### Failed to activate Channel. Downstream error.

 Failed to activate a Channel <channel sid> due to an error from Twilio Conversations API.

#### Possible Causes
Conversations API returned an error

#### Possible Solutions
Check our status page to see if we are having an outage.

If our status page has no information, contact support with details about the requests that are failing, and we will investigate."#,
            TwilioFlexError::ErrorCode45794 => r#"## WARNING - 45794

### Max deployment keys limit reached for this account.

 Max deployment keys limit reached for this account.

#### Possible Causes
Account already has maximum allowed number of deployment keys.

#### Possible Solutions
Delete any of the existing deployment key to create more."#,
            TwilioFlexError::ErrorCode45208 => r#"## WARNING - 45208

### Chat Channel with this unique name already exists

 

#### Possible Causes
An Active Chat Channel already exists with this unique name - new Channel not created. 

#### Possible Solutions
Please use the existing active Chat Channel, mark the Channel as inactive by setting Channel Attributes status to 'inactive', remove the Chat Channel or use a different unique name. "#,
            TwilioFlexError::ErrorCode45370 => r#"##  - 45370

### 

 

#### Possible Causes


#### Possible Solutions
"#,
            TwilioFlexError::ErrorCode45358 => r#"##  - 45358

### 

 

#### Possible Causes


#### Possible Solutions
"#,
            TwilioFlexError::ErrorCode45005 => r#"## ERROR - 45005

### Service Unavailable

 

#### Possible Causes
Service or one of it dependent services does not respond, timeout, etc.

#### Possible Solutions
Try the request again."#,
            TwilioFlexError::ErrorCode45360 => r#"##  - 45360

### 

 

#### Possible Causes


#### Possible Solutions
"#,
            TwilioFlexError::ErrorCode45733 => r#"## ERROR - 45733

### Pre-engagement data is not in valid JSON format.

 Failed to create webchat conversation. Pre-engagement data not in valid JSON format. 

#### Possible Causes
Pre-engagement data is not in valid JSON format.

#### Possible Solutions
Ensure pre-engagement data is in valid JSON format.

Read more about creating valid attributes [here.](https://www.twilio.com/docs/conversations/attributes)"#,
            TwilioFlexError::ErrorCode49009 => r#"## ERROR - 49009

### Credentials Malformed

 The credential SID associated with either the Profile Connector Instance SID or the Unique Name was invalid. Please check configuration and update credentials

#### Possible Causes
Credentials are present, but aren’t parse-able due to an invalid SID placed inside of the configuration.

#### Possible Solutions
Please ensure the correct value inside of the SegmentUnifyApiKey."#,
            TwilioFlexError::ErrorCode45002 => r#"## ERROR - 45002

### Authentication Error

 

#### Possible Causes
Attempt to access resource with invalid credentials or without credentials.

#### Possible Solutions
Check the credentials and try again."#,
            TwilioFlexError::ErrorCode45760 => r#"## ERROR - 45760

### Unable to validate address configuration. Auto create data missing.  

 Unable to validate address configuration for provided address SID. Auto create data missing.  

#### Possible Causes
Address configuration data associated with the provided address SID is missing auto create data. 

#### Possible Solutions
Ensure that the address SID provided is valid and associated with the correct account. 

Ensure that the auto create data associated with the address is present and in the correct format. More information on creating and updating an address configuration resource can be found [here.](https://www.twilio.com/docs/conversations/api/address-configuration-resource#addressconfiguration-properties)"#,
            TwilioFlexError::ErrorCode90003 => r#"## ERROR - 90003

### Unable to fetch plugins from the Custom Plugins URL

 Twilio could not fetch your Flex plugins from your provided plugins URL

#### Possible Causes
* An invalid content type or no content being returned when Twilio tries to retrieve your plugins using the custom plugins URL.

#### Possible Solutions
* Check if the custom plugins URL has been set correctly 
* The custom plugin URL must return a valid JSON object

See the [Deploying Flex Plugins](/docs/flex/developer/plugins/cli#deploying-to-your-own-cdn) docs for more information."#,
            TwilioFlexError::ErrorCode45302 => r#"## WARNING - 45302

### Participant was disconnected before setting up Meeting

 User was disconnected from media before setting up Meeting

#### Possible Causes
Bad connectivity, User error

#### Possible Solutions
Please verify user connectivity and retry"#,
            TwilioFlexError::ErrorCode45375 => r#"##  - 45375

### 

 

#### Possible Causes


#### Possible Solutions
"#,
            TwilioFlexError::ErrorCode45301 => r#"## ERROR - 45301

### Error occurred when connecting to a Meeting Participant

 Error occurred when connecting to a user

#### Possible Causes
Flex encountered a service error when connecting to a user

#### Possible Solutions
No further information available."#,
            TwilioFlexError::ErrorCode45369 => r#"##  - 45369

### 

 

#### Possible Causes


#### Possible Solutions
"#,
            TwilioFlexError::ErrorCode45366 => r#"##  - 45366

### 

 

#### Possible Causes


#### Possible Solutions
"#,
            TwilioFlexError::ErrorCode45771 => r#"## ERROR - 45771

### Failed to create InboundConversation. Internal Error.

 An unexpected error occurred while creating Inbound Conversation. 

#### Possible Causes
We encountered an unexpected error processing the request.

#### Possible Solutions
"Retry the request.

Check our status page to see if we are having an outage.

If our status page has no information, contact support with details about the requests that are failing, and we will investigate.""#,
            TwilioFlexError::ErrorCode45763 => r#"## ERROR - 45763

### Unable to validate address configuration. Auto create studio flow SID missing. 

 Unable to validate address configuration for provided address SID. Auto create studio flow SID missing. 

#### Possible Causes
Address configuration associated with the provided address SID has auto create type set to STUDIO but no corresponding studio flow SID. 

#### Possible Solutions
Ensure that the address SID provided is valid and associated with the correct account. 

Ensure that the auto create type in address configuration is correct. If your auto create type is set to STUDIO, ensure the studio flow SID is present and correct in address configuration. 

Read more about creating and updating an address configuration resource [here.](https://www.twilio.com/docs/conversations/api/address-configuration-resource#create-an-addressconfiguration-resource)

Read more about creating a studio flow [here.](https://www.twilio.com/docs/usage/webhooks/webhooks-overview)"#,
            TwilioFlexError::ErrorCode45209 => r#"## ERROR - 45209

### Adding a user to the Chat Channel has timed out waiting for a response from Twilio Chat.

 Adding user to Chat Channel has failed because of a timeout

#### Possible Causes
Adding a user to the Chat Channel has timed out waiting for a response from Twilio Chat.

#### Possible Solutions
Please refresh your browser and retry your Webchat experience."#,
            TwilioFlexError::ErrorCode45780 => r#"## WARNING - 45780

### Failed to perform this operation. Account is sending too many concurrent API  requests.

 Failed to perform this operation. Account is sending too many concurrent API  requests.

#### Possible Causes
Your account is sending too many concurrent requests.

#### Possible Solutions
Please wait for a short period of time and make the request again, or alter your client's settings to issue fewer concurrent requests to the Twilio API."#,
            TwilioFlexError::ErrorCode45359 => r#"##  - 45359

### 

 

#### Possible Causes


#### Possible Solutions
"#,
            TwilioFlexError::ErrorCode45009 => r#"## ERROR - 45009

### Configuration Error

 

#### Possible Causes
A Flex component was not configured correctly.

#### Possible Solutions
Revise and verify the configuration. If everything is as expected then contact Customer Support to review your use case."#,
            TwilioFlexError::ErrorCode45102 => r#"## ERROR - 45102

### Collision On Configuration Change

 

#### Possible Causes
Configuration for the same account was changed by other request mostly at the same time.

#### Possible Solutions
Try to update the configuration again."#,
            TwilioFlexError::ErrorCode45600 => r#"## ERROR - 45600

### Flex UI error

Error thrown by Flex UI. Flex UI instance experienced a thrown Flex UI Error
A JavaScript error has occurred within Flex UI.

#### Possible Causes
* There may have been an issue initializing Flex UI (no network, wrong config)
* There may be an error happening in Flex UI code
* There may be an error happening in user's custom code/plugin

#### Possible Solutions
* Fix custom code
* Fix configuration"#,
            TwilioFlexError::ErrorCode45352 => r#"##  - 45352

### 

 

#### Possible Causes


#### Possible Solutions
"#,
            TwilioFlexError::ErrorCode90004 => r#"## ERROR - 90004

### Message Truncated

 A debug-event was too large to be published to Twilio Debugger.

#### Possible Causes
* You have logged a message field that exceeds a total of 5000 allowed bytes

#### Possible Solutions
* Try to separate out your logs into individual messages"#,
            TwilioFlexError::ErrorCode45778 => r#"## WARNING - 45778

### Configuration not found for provided Deployment Key.

 Deployment configuration not found for provided deployment key.

#### Possible Causes
No deployment configuration associated with the given deployment key.

#### Possible Solutions
Ensure the deployment key is correct and there exists a valid deployment configuration associated with it."#,
            TwilioFlexError::ErrorCode45745 => r#"## ERROR - 45745

### Invalid or missing parameters in the create conversation request.

 Failed to create webchat conversation for provided address SID due to invalid request parameters.

#### Possible Causes
Invalid or missing parameters in the create conversation request.

#### Possible Solutions
Ensure that request parameters are in accordance with the public documentation."#,
            TwilioFlexError::ErrorCode45006 => r#"## ERROR - 45006

### Resource Not Found

 

#### Possible Causes
Requested resource could not be found.

#### Possible Solutions
Check that accessed resource is set correctly."#,
            TwilioFlexError::ErrorCode45764 => r#"## ERROR - 45764

### Unable to validate address configuration. Auto create webhook URL missing. 

 Unable to validate address configuration for provided address SID. Auto create webhook URL missing. 

#### Possible Causes
Address configuration associated with the provided address SID has auto create type set to WEBHOOK but no webhook URL was provided.

#### Possible Solutions
Ensure that the address SID provided is valid and associated with the correct account.

Ensure that the auto create type provided is correct. If your auto create type is WEBHOOK, ensure the webhook url is present and valid in the address configuration. 

Read more about creating and updating an address configuration resource [here.](https://www.twilio.com/docs/conversations/api/address-configuration-resource#create-an-addressconfiguration-resource)

Read more about webhooks and their applications [here.](https://www.twilio.com/docs/usage/webhooks/webhooks-overview)"#,
            TwilioFlexError::ErrorCode45201 => r#"## ERROR - 45201

### Resource Not Found

 

#### Possible Causes
Requested resource could not be found.

#### Possible Solutions
Check that accessed resource is set correctly."#,
            TwilioFlexError::ErrorCode45772 => r#"## WARNING - 45772

### Failed to perform operation. Required header is either missing or incorrect.

 Failed to perform operation. Required header is either missing or incorrect.

#### Possible Causes
Mandatory headers might be missing in the request.

#### Possible Solutions
Provide all the mandatory headers."#,
            TwilioFlexError::ErrorCode45378 => r#"## ERROR - 45378

### Failed to inactivate Channel. Internal error.

 Failed to inactivate a Channel <channel sid> due to an internal error.

#### Possible Causes
We failed to inactive a Channel due to an unexpected error condition.

#### Possible Solutions
Check our status page to see if we are having an outage.

If our status page has no information, contact support with details about the requests that are failing, and we will investigate."#,
            TwilioFlexError::ErrorCode45795 => r#"## WARNING - 45795

### Friendly name must be between 1 and  64 characters long.

 Friendly name must be between 1 and  64 characters long.

#### Possible Causes
The friendly name must consists of at least 1 character and up to a maximum of 64 characters.

#### Possible Solutions
Provide a friendly name that is between 1 and 64 characters in length."#,
            TwilioFlexError::ErrorCode45210 => r#"## ERROR - 45210

### Flex Flow creation failed

 Flex Flow creation failed.

#### Possible Causes
Required Flow attributes were missing from the request or collided with existing Flows.

#### Possible Solutions
Check the returned error details for additional info and adjust the request as necessary."#,
            TwilioFlexError::ErrorCode45402 => r#"## WARNING - 45402

### RTA feed callback response time exceeded the threshold

 The RTA Feed callback is taking longer than a defined threshold to respond

#### Possible Causes
Issues on WFM vendor side

#### Possible Solutions
Please contact the WFM Vendor for troubleshooting"#,
            TwilioFlexError::ErrorCode45785 => r#"## ERROR - 45785

### Internal Error. Failed to delete deployment keys.

 An unexpected error occurred when processing delete all deployment keys request.

#### Possible Causes
We encountered an unexpected error when deleting deployment keys.

#### Possible Solutions
Retry the request.

Check our status page to see if we are having an outage.

If our status page has no information, contact support with details about the requests that are failing, and we will investigate."#,
            TwilioFlexError::ErrorCode45004 => r#"## ERROR - 45004

### Validation Error

 

#### Possible Causes
Request body validation fails.

#### Possible Solutions
Review the input and fix (if possible) the problem mentioned in the error message."#,
            TwilioFlexError::ErrorCode45372 => r#"##  - 45372

### 

 

#### Possible Causes


#### Possible Solutions
"#,
            TwilioFlexError::ErrorCode45211 => r#"## ERROR - 45211

### Provided identity is reserved by system

 Provided identity is reserved by system.

#### Possible Causes
Will pop up when users try to create a new channel with a reserved identity.

#### Possible Solutions
Change the identity field in the request and try again"#,
            TwilioFlexError::ErrorCode45353 => r#"##  - 45353

### 

 

#### Possible Causes


#### Possible Solutions
"#,
            TwilioFlexError::ErrorCode45305 => r#"## WARNING - 45305

###  No answer from Participant

 No media response from user

#### Possible Causes
Inactive user

#### Possible Solutions
Retry operation."#,
            TwilioFlexError::ErrorCode45777 => r#"## WARNING - 45777

### Address Configuration not found for deployment key.

 Address Configuration not found for deployment key.

#### Possible Causes
No address configuration is linked with the given deployment key.

#### Possible Solutions
Ensure the deployment key is correct and there exists a valid address configuration Sid associated with it."#,
            TwilioFlexError::ErrorCode45775 => r#"## WARNING - 45775

### Failed to refresh token. Invalid token provided.

 Failed to refresh token. Invalid token provided.

#### Possible Causes
Token provided in the request is invalid.

#### Possible Solutions
Provide the correct token."#,
            TwilioFlexError::ErrorCode93105 => r#"## ERROR - 93105

### Consent has already been provided for this account and vendor

 Error surfaces when consent data has been provided

#### Possible Causes
The consent submission happens more than once

#### Possible Solutions
Do not submit consent data more than once"#,
            TwilioFlexError::ErrorCode45741 => r#"## ERROR - 45741

### Failed to create webchat conversation. Account not authorized 

 Failed to create webchat conversation for provided address SID. Account not authorized 

#### Possible Causes
Request attempted to access resource where access is not allowed for given credentials.

#### Possible Solutions
Check that credentials are correct.

Check that accessed resource is set correctly or apply for permissions."#,
            TwilioFlexError::ErrorCode45202 => r#"## ERROR - 45202

### Resource Already Exists

 

#### Possible Causes
The request could not be completed due to a conflict with the current state of the resource.

#### Possible Solutions
Check that resource to be updated is set correctly."#,
            TwilioFlexError::ErrorCode45312 => r#"## ERROR - 45312

### Remove Participant Request Failed

 Error occurred when trying to remove a participant

#### Possible Causes
Voice API returned an error while trying to remove the participant

#### Possible Solutions
Retry operation"#,
            TwilioFlexError::ErrorCode45007 => r#"## ERROR - 45007

### Resource Conflict Error

 

#### Possible Causes
The request could not be completed due to a conflict with the current state of the target resource.

#### Possible Solutions
Please verify the current status of the resource to see if desired updates are already present in the system and that the request does not contain information that conflicts with the current state.
More context of the specific resource can be seen under the console message body."#,
            TwilioFlexError::ErrorCode45788 => r#"## ERROR - 45788

### Internal Error. Failed to update deployment key.

 An unexpected error occurred when processing update deployment key request.

#### Possible Causes
We encountered an unexpected error when updating deployment key.

#### Possible Solutions
Retry the request.

Check our status page to see if we are having an outage.

If our status page has no information, contact support with details about the requests that are failing, and we will investigate."#,
            TwilioFlexError::ErrorCode45313 => r#"## ERROR - 45313

### Transfer Failed

 Transfer failed to complete 

#### Possible Causes
Transfer process took too long to complete or the voice API returned error

#### Possible Solutions
Retry operation"#,
            TwilioFlexError::ErrorCode45781 => r#"## ERROR - 45781

### Internal Error. Failed to create deployment key.

 An unexpected error occurred when processing create deployment key request.

#### Possible Causes
We encountered an unexpected error when creating deployment key.

#### Possible Solutions
Retry the request.

Check our status page to see if we are having an outage.

If our status page has no information, contact support with details about the requests that are failing, and we will investigate."#,
            TwilioFlexError::ErrorCode456001 => r#"## WARNING - 456001

### Approaching Flex Plugin creation limit

 You are approaching the limit of how many Flex Plugins you can have on your account. Once you reach the limit, you may need to archive old Plugins to create any new one.

#### Possible Causes
You are approaching 25 Flex Plugins on your account.

#### Possible Solutions
Archive older Plugins if they are no longer used."#,
            TwilioFlexError::ErrorCode456002 => r#"## WARNING - 456002

### Approaching Flex Plugin Version creation limit

 You are approaching the limit of how many Flex Plugin Versions you can have. Once you reach the limit, you may need to archive old Versions to create any new one.

#### Possible Causes
You are approaching 500 Flex Plugin Versions on your account.

#### Possible Solutions
Archive older Plugin Versions if they are no longer used."#,
            TwilioFlexError::ErrorCode45401 => r#"## WARNING - 45401

### RTA feed callback returned not successful response code

 Call to defined rta_callback_url returned with not successful response code

#### Possible Causes
RTA callback is misconfigured or down

#### Possible Solutions
Change account configuration and set correct value for rta_callback_url"#,
            TwilioFlexError::ErrorCode45761 => r#"## ERROR - 45761

### Failed to validate address configuration. Auto create not enabled.

 Unable to validate address configuration for provided address SID. Auto create not enabled.

#### Possible Causes
Address configuration data associated with the provided address SID does not have auto create enabled in address configuration. When disabled, conversations will not be auto-created on incoming messages.  

#### Possible Solutions
"Ensure the address SID provided is valid and associated with the correct account.

Ensure the address configuration associated with the address is correct. Read more about creating and updating an address configuration resource [here.](https://www.twilio.com/docs/conversations/api/address-configuration-resource#fetch-an-addressconfiguration-resource)""#,
            TwilioFlexError::ErrorCode45765 => r#"## ERROR - 45765

### Address configuration data not found

 No address configuration data found for provided address SID. 

#### Possible Causes
No address configuration has been created for the provided address SID.

The address SID provided is incorrect.

#### Possible Solutions
"Ensure the provided address SID is correct and that the asscoiated address configuration has been created. 

Read more about creating and updating an address configuration resource [here.](https://www.twilio.com/docs/conversations/api/address-configuration-resource#create-an-addressconfiguration-resource)""#,
            TwilioFlexError::ErrorCode45789 => r#"## WARNING - 45789

### Deployment key is already linked to other address  configuration sid.

 Deployment key is already linked to another address  configuration Sid.

#### Possible Causes
You cannot link deployment key to multiple address configuration Sids.

#### Possible Solutions
First unlink the deployment key from current address configuration Sid."#,
            TwilioFlexError::ErrorCode45303 => r#"## WARNING - 45303

### Participant was busy

 User was busy and could not connect to media

#### Possible Causes
User was already engaged on the media channel

#### Possible Solutions
Disconnect media and retry."#,
            TwilioFlexError::ErrorCode45304 => r#"## ERROR - 45304

### Internal timeout error

 Internal timeout error

#### Possible Causes
Flex encountered a timeout error

#### Possible Solutions
Retry operation."#,
            TwilioFlexError::ErrorCode45205 => r#"## ERROR - 45205

### Requested Flex Flow is not found

 

#### Possible Causes
The Flex Flow referenced in this request or related Messaging request cannot be found.

#### Possible Solutions
Please ensure you have correctly configured a Flex Flow related to your Messaging Channel (e.g. sms) and Integration Type (e.g. studio). You can use the Flex API or Flex Console to do this."#,
            TwilioFlexError::ErrorCode45363 => r#"##  - 45363

### 

 

#### Possible Causes


#### Possible Solutions
"#,
            TwilioFlexError::ErrorCode45354 => r#"##  - 45354

### 

 

#### Possible Causes


#### Possible Solutions
"#,
            TwilioFlexError::ErrorCode90000 => r#"## Error - 90000

### Uncaught Flex JavaScript Exception
A JavaScript error has occurred within Flex UI.

#### Possible Causes
* There may have been an issue initializing Flex
* There may be an uncaught exception within a Flex plugin"#,
            TwilioFlexError::ErrorCode45008 => r#"## ERROR - 45008

### Unprocessable Request

 

#### Possible Causes
The server is not able to process the request due to something that is perceived to be a client error (e.g. malformed request syntax or invalid request message).

#### Possible Solutions
Please verify the request details and ensure Flex setup and preconditions for this request have been met before trying again. "#,
            TwilioFlexError::ErrorCode45380 => r#"## ERROR - 45380

### Failed to activate Channel. Internal error.

 Failed to activate a Channel <channel sid> due to an internal error.

#### Possible Causes
We failed to active a Channel due to an unexpected error condition.

#### Possible Solutions
Check our status page to see if we are having an outage.

If our status page has no information, contact support with details about the requests that are failing, and we will investigate."#,
            TwilioFlexError::ErrorCode45212 => r#"## ERROR - 45212

### Flex Flow creation failed

 Flex Flow creation failed

#### Possible Causes
Some request attributes were either missing, invalid, or collided with existing Flex Flows.

#### Possible Solutions
Check the returned error details for additional info and adjust the request as necessary."#,
            TwilioFlexError::ErrorCode45368 => r#"##  - 45368

### 

 

#### Possible Causes


#### Possible Solutions
"#,
            TwilioFlexError::ErrorCode45310 => r#"## ERROR - 45310

### Participant Not Found

 Participant is not found in the meeting

#### Possible Causes
Participant is not in the meeting anymore

#### Possible Solutions
No action required"#,
            TwilioFlexError::ErrorCode45001 => r#"## ERROR - 45001

### General Service Error

 

#### Possible Causes
Flex encountered a general service error.

#### Possible Solutions
No further information available."#,
            TwilioFlexError::ErrorCode45050 => r#"## ERROR - 45050

### Authentication error - Missing User Attributes

 Authentication error - Missing User Attributes

#### Possible Causes
Missing or invalid required attributes, such as full_name, email or roles

#### Possible Solutions
Ensure that the user authentication response sent to Flex includes required user attributes."#,
            TwilioFlexError::ErrorCode45715 => r#"## WARNING - 45715

### Failed to create webchat participant. Invalid request parameters. 

 Failed to create participant due to invalid request parameters. 

#### Possible Causes
The address configuration associated with the address SID provided could have invalid parameters. This could be because the account credentials provided are not associated with the address SID provided. 

#### Possible Solutions
Ensure the account credentials and address SID provided are correct and are all associated with the intended account. 

Ensure that the request parameters are in accordance with the public documentation."#,
            TwilioFlexError::ErrorCode45306 => r#"## WARNING - 45306

### Meeting was canceled

 Operation was canceled

#### Possible Causes
Operation was canceled

#### Possible Solutions
No action required."#,
            TwilioFlexError::ErrorCode45350 => r#"##  - 45350

### 

 

#### Possible Causes


#### Possible Solutions
"#,
            TwilioFlexError::ErrorCode45774 => r#"## ERROR - 45774

### Internal Error. Failed to refresh token.

 An unexpected error occurred when processing refresh token request.

#### Possible Causes
We encountered an unexpected error when refreshing token.

#### Possible Solutions
Retry the request.

Check our status page to see if we are having an outage.

If our status page has no information, contact support with details about the requests that are failing, and we will investigate."#,
            TwilioFlexError::ErrorCode45379 => r#"## ERROR - 45379

### Failed to inactivate Channel. Downstream error.

 Failed to inactivate a Channel <channel sid> due to an error from Twilio Conversations API.

#### Possible Causes
Conversations API returned an error

#### Possible Solutions
Check our status page to see if we are having an outage.

If our status page has no information, contact support with details about the requests that are failing, and we will investigate."#,
            TwilioFlexError::ErrorCode45204 => r#"## WARNING - 45204

### Requested Flex Chat Channel is not found

 

#### Possible Causes
Flex Channel could not be found. 

#### Possible Solutions
Please check your request details. Flex Channel is created automatically on inbound messages and can be created by developer via Flex API for outbound use cases.

Check our [Flex Chat Channel Resource documentation](https://www.twilio.com/docs/flex/developer/messaging/chat-channel), or see how to [add a Custom Chat Channel to Flex](https://www.twilio.com/blog/add-custom-chat-channel-twilio-flex)."#
        }
    }
}
