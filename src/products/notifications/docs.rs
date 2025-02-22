// AUTO-GENERATED. DO NOT MODIFY. YOUR CHANGES WILL BE OVER-WRITTEN
use crate::products::TwilioNotificationsError;
use standard_error::traits::StandardErrorDocsTrait;

impl StandardErrorDocsTrait for TwilioNotificationsError {
    fn docs(&self) -> &'static str {
        match self {
            TwilioNotificationsError::ErrorCode52108 => r#"## Error - 52108

### GCM/FCM device message rate exceeded

#### Possible Causes 
*  The rate of messages to a particular device via GCM/FCM is too high.

#### Possible Solutions
*  Reduce the number of messages sent to this device and do not immediately retry sending to this device."#,
            TwilioNotificationsError::ErrorCode52202 => r#"## Error - 52202

### Facebook page ID is not connected to Twilio

#### Possible Causes 
* Facebook page configured in Notify Service instance is not connected to Twilio
* The connection may have been removed after the Notify Service had been configured

#### Possible Solutions
* [Configure a new Facebook page](https://www.twilio.com/console/notify/services) for your Notify Service
* [Connect the Facebook page](https://www.twilio.com/console/channels) to your Twilio account"#,
            TwilioNotificationsError::ErrorCode52052 => r#"## Warning - 52052

### Client connection not found

#### Possible Causes 
*  Client connection not found.

#### Possible Solutions
*  No action needed. The client will automatically reconnect when possible."#,
            TwilioNotificationsError::ErrorCode52211 => r#"## Error 52211

### Too many Alexa notifications

Your message has been rejected by the Alexa channel due too many messages.

### Possible causes
- You are making Send Notification requests too fast.
- You have made some bulk notification requests that created too many Alexa messages too fast.

### Possible solutions
- Slow down the rate of Send Notification requests.
- Target less recipients per bulk notification request and send the bulk requests more slowly."#,
            TwilioNotificationsError::ErrorCode52102 => r#"## Error - 52102

### Invalid GCM/FCM registration token

#### Possible Causes 
*  Invalid GCM/FCM registration token provided via the Android SDK or the REST API.

#### Possible Solutions
*  Make sure you do not modify the registration token before providing it to Twilio via the SDK or the REST API.
*  If you are using a command line tool to provide the registration token, make sure that it is URL-encoded."#,
            TwilioNotificationsError::ErrorCode52143 => r#"## ERROR - 52143

### The push notification was rejected by APNs

 The push notification was rejected by APNs for some reason.

NB: Twilio push infrastructure does not support the notifications to not main certificate topic. So application device token and certificate subject must have the same bundle id. For instance, for VoIP notifications the certificate subject must end with "*.voip" and the device token app bundle ends with "*.voip".

#### Possible Causes
* The push notification subject in the certificate is incorrect
* The device token cannot be used with the specified subject
* Pushing to the specified topic is not allowed
* An internal Twilio error has occurred

#### Possible Solutions
* Verify that your certificate is correct
* If the problem persists, please contact [customer support](https://www.twilio.com/help/contact)."#,
            TwilioNotificationsError::ErrorCode52171 => r#"## Error - 52171

### Webhook Credentials request signature was not verified

#### Possible Causes 
*  Your token endpoint was not able to verify the request signature

#### Possible Solutions
*  Verify that your Webhook Credentials are configured with the correct values
*  Verify that your signature verification logic is working as expected"#,
            TwilioNotificationsError::ErrorCode52131 => r#"## Error - 52131

### Invalid APNs credentials

#### Possible Causes 
*  Credential (certificate and private key) is invalid.
*  Certificate expired.

#### Possible Solutions
*  Make sure you have valid certificate and private key provided in your Credential."#,
            TwilioNotificationsError::ErrorCode52133 => r#"## Error - 52133

### Invalid APNs device token size

#### Possible Causes 
*  Invalid APNs token was provided.

#### Possible Solutions
*  Make sure you do not modify the APNs device token before providing it to Twilio."#,
            TwilioNotificationsError::ErrorCode52163 => r#"## Error - 52163

### Incorrect URL used to retrieve Webhook Credentials

#### Possible Causes 
*  An incorrect URL was specified in your Webhook Credentials URL field
*  The URL was correct, but returned a 404 response

#### Possible Solutions
*  Verify your Webhook Credential URL and response"#,
            TwilioNotificationsError::ErrorCode52162 => r#"## Error - 52162

### Credentials do not belong to used account

#### Possible Causes 
*  The credential you have tried to use does not belong to the account.

#### Possible Solutions
*  Use a credential that belongs to the account."#,
            TwilioNotificationsError::ErrorCode52103 => r#"## Error - 52103

### GCM/FCM client uninstalled or turned off notifications

#### Possible Causes 
*  GCM/FCM client application was uninstalled or the user turned off push notifications.

#### Possible Solutions
*  No action needed. We have disabled the registration token and we will not send new notifications to it."#,
            TwilioNotificationsError::ErrorCode52164 => r#"## Error - 52131

### Invalid APNs credentials

#### Possible Causes 
*  The Webhook Credential was configured for the wrong Binding Type
*  Your token endpoint responded with the respective error code by mistake
*  An internal Twilio error occurred

#### Possible Solutions
*  Verify that your Webhook Credential is set up for the correct Binding Type
*  Verify that your token endpoint is set up correctly"#,
            TwilioNotificationsError::ErrorCode52137 => r#"## Error - 52137

### Invalid size of subject in APNs certificate

#### Possible Causes 
*  The subject of APNs certificate is of invalid size.

#### Possible Solutions
*  The subject of the APNs certificate is the bundle ID of your application. Make sure you have a valid bundle ID defined when generating the certificate signing request."#,
            TwilioNotificationsError::ErrorCode52145 => r#"## Error - 52145

### Failed to authenticate with APNs

#### Possible Causes 
*  Unable to complete the TLS handshake with APNs.

#### Possible Solutions
* Verify that your certificate and private key are correct and can be used together.
* Verify that your certificate has not been revoked.
* If the problem persists, please contact [customer support](https://www.twilio.com/help/contact)."#,
            TwilioNotificationsError::ErrorCode52001 => r#"## Error 52001

### Invalid destination binding

The message has been rejected by channel due to an invalid destination information. This specific message has not been sent.

### Possible causes
- Address or identity or notification protocol version in a Notify Binding (or ToBinding) were invalid

### Possible solutions
- Make sure that you do not modify the address before creating the Binding or sending ToBinding property of Notification Request. 
"#,
            TwilioNotificationsError::ErrorCode52109 => r#"## Error - 52109

### GCM/FCM unauthorized error

#### Possible Causes 
*  ApiKey/Secret is revoked or invalid.

#### Possible Solutions
Provide a new API Key/Secret and make sure your application is configured to use it.

* If using IP-Messaging, update your Credential with the new API Key/Secret.
* If using User Notifications, you can either update your Credential or create a new Credential and update your Service's GcmCredentialSid/FcmCredentialSid."#,
            TwilioNotificationsError::ErrorCode52172 => r#"## Error - 52172

### Unexpected Binding Type used for Webhook Credentials request

#### Possible Causes 
* Your token endpoint responded with an error indicating that the Binding Type used was not correct

#### Possible Solutions
* Verify that your Webhook Credential is configured correctly
* Verify that your token endpoint is configured correctly
* If the problem persists, please contact [customer support](https://www.twilio.com/help/contact)."#,
            TwilioNotificationsError::ErrorCode52104 => r#"## Error - 52104

### Mismatched GCM/FCM sender ID

#### Possible Causes 
*  GCM/FCM API Key provided is not allowed to send notifications to the registration token.

#### Possible Solutions
Make sure that you are using an API Key/Secret that is allowed to send notifications to your application.

*	Add the Project number of the Google Developers Console/Firebase Console project that the API Key/Secret belongs to to the allowed sender IDs of your application
*	Alternatively use an API Key/Secret from a project that is already allowed to send notifications to your application"#,
            TwilioNotificationsError::ErrorCode52071 => r#"## Error - 52071

### RtdNotification: Internal error when sending notification via mqtt client connection

#### Possible Causes 
*  Temporary outage of client connection mqtt gateway.

#### Possible Solutions
*  Ask customer support for investigation. "#,
            TwilioNotificationsError::ErrorCode52105 => r#"## Error - 52105

### RtdNotification: Invalid GCM/FCM package name

#### Possible Causes 
*  Application configuration problem.

#### Possible Solutions
*  Make sure the message was addressed to a registration token whose package name matches the value passed in the request. "#,
            TwilioNotificationsError::ErrorCode52213 => r#"## Error 52213

### Invalid Alexa user ID

The message has been rejected by Alexa channel due to an invalid User ID. This specific message has not been sent.

### Possible causes
- The user ID (Address) in a Notify Binding was invalid

### Possible solutions
- Make sure that you do not modify the user ID before creating the Binding. 
"#,
            TwilioNotificationsError::ErrorCode52167 => r#"## Error - 52167

### Invalid Webhook Credentials response

#### Possible Causes 
*  Your Webhook Credentials response did not adhere to the required schema

#### Possible Solutions
*  Verify that your Webhook Credentials response is correct"#,
            TwilioNotificationsError::ErrorCode52214 => r#"## Error - 52214

### Alexa skill not connected to Twilio

#### Possible Causes 
* Alexa skill ID configured for Notify Service is not connected to Twilio
* After the Notify Service had been configured, the Alexa skill was disconnected from Twilio

#### Possible Solutions
* [Configure a new Alexa skill](https://www.twilio.com/console/notify/services) for your Notify Service
* [Connect the Alexa skill](https://www.twilio.com/console/channels) to your Twilio Account"#,
            TwilioNotificationsError::ErrorCode52304 => r#"## Warning - 52304

### User already exists.

#### Possible Causes 
*  User with this identity has already been created, possibly during create binding operation.

#### Possible Solutions
*  As user already exists, no extra actions are required. However one might want to make sure that the existing user has all the necessary attributes, e.g. segments."#,
            TwilioNotificationsError::ErrorCode52140 => r#"## Error - 52140

### Unknown APNs error

#### Possible Causes 
*  Unknown APNs error.

#### Possible Solutions
*  If the problem persists, please contact [customer support](https://www.twilio.com/help/contact)."#,
            TwilioNotificationsError::ErrorCode52215 => r#"## Error 52215

### Missing Title parameter for Alexa

Message has been rejected by Alexa channel due to missing Title parameter. Title is a required parameter for Alexa channel. This specific message has not been sent. 

### Possible causes
- Have not provided the Title parameter neither in the generic payload nor in the Alexa specific parameter. 

### Possible solutions
- Make sure that you provide the Title parameter for the Alexa channel either in the [generic payload](https://www.twilio.com/docs/api/notify/rest/notifications#generic-payload-parameters) or the [Alexa specific parameter](https://www.twilio.com/docs/api/notify/rest/notifications#sending-channel-specific-notifications)."#,
            TwilioNotificationsError::ErrorCode52182 => r#"## Error - 52182

### Messaging Service not specified

In order to send SMS messages with Notify you need to configure a [Messaging Service](https://www.twilio.com/docs/api/rest/messaging-services-and-copilot-overview) and connect it with your Notify Service. Messaging Services make it easy to build SMS application by automatically selecting an optimal From phone number from pool you can define. 

#### Possible causes
- MessagingServiceSid in Notify Service is not specified

#### Possible solutions
- Configure a Messaging Service in the [console](/console/notify/services) or via the [REST API](https://www.twilio.com/docs/api/notify/rest/services#update-a-service). You can create a new Messaging Service in the [console](https://www.twilio.com/console/sms/services) or via the [REST API](https://www.twilio.com/docs/api/rest/messaging-services)."#,
            TwilioNotificationsError::ErrorCode52149 => r#"## Error - 52149

### Invalid APNs provider token used

#### Possible Causes 
* The APNs provider token is malformed or otherwise incorrect

#### Possible Solutions
* Verify that your Webhook Credential is configured correctly
* Verify that your token endpoint is configured correctly
* If the problem persists, please contact [customer support](https://www.twilio.com/help/contact)."#,
            TwilioNotificationsError::ErrorCode52305 => r#"## Warning - 52305

### User is already a member of this segment.

#### Possible Causes 
*  User has already been added to this segment.

#### Possible Solutions
*  As user is already a member of the required segment, no actions are needed."#,
            TwilioNotificationsError::ErrorCode52181 => r#"## Error - 52181

### RtdNotification: Global SMS notification per second limit exceeded

#### Possible Causes 
* Somebody is using SMS broadcast
* General high load

#### Possible Solutions
* At the moment there is a global SMS rate limit. This should be increased or eliminated entirely in the near future, but that requires some development. Please notify RTD Notifications team if this error starts occurring."#,
            TwilioNotificationsError::ErrorCode52144 => r#"## Error - 52144

### APNs experienced an internal error

#### Possible Causes 
* APNs had an unexpected error
* APNS is unavailable
* Request to APNs timed out

#### Possible Solutions
* Retry the request at a later time
*  If the problem persists, please contact [customer support](https://www.twilio.com/help/contact)."#,
            TwilioNotificationsError::ErrorCode52166 => r#"## Error - 52166

### Webhook Credentials endpoint responded with Internal Error

#### Possible Causes 
*  Your Webhook Credentials endpoint is misconfigured

#### Possible Solutions
*  Verify that your token endpoint is configured correctly"#,
            TwilioNotificationsError::ErrorCode52309 => r#"## Error - 52309

### RtdNotification: Channel provider replied with an error

#### Possible Causes 
*  Some errors in channel configuration: destination address, credentials, permissions, etc.

#### Possible Solutions
*  Check channel configuration or re-install the channel and destination binding. Check response message from the provider in a debugger alert."#,
            TwilioNotificationsError::ErrorCode52401 => r#"## Warning - 52401

### Too many notification log events 

#### Possible Causes 
*  Notification requests resulted in too many delivery attempts. As a result, some of the notification events were rate limited and might not be visible in the notification log in console. The message delivery itself is not affected.

#### Possible Solutions
* When sending large broadcasts, slow down the rate of Send Notification requests, e.g. to 1 / s"#,
            TwilioNotificationsError::ErrorCode52203 => r#"## Error - 52203

### Missing body for Facebook Messenger delivery attempt

Messenger messages need to have some content. You can specify this either via the [generic Body parameter](https://www.twilio.com/docs/api/notify/rest/notifications#generic-payload-parameters) of the Notification request or the [FacebookMessenger channel specific parameter](https://www.twilio.com/docs/api/notify/rest/notifications#channel-specific-parameters).

#### Possible causes
- Notification has neither Body not FacebookMessenger parameter set

#### Possible solutions
- Specify either Body or FacebookMessenger parameter when sending a Notification"#,
            TwilioNotificationsError::ErrorCode52165 => r#"## Error - 52165

### No Credentials found for the supplied Identifier

#### Possible Causes 
*  Your Webhook Credential has a misconfigured Identifier value
*  Your token endpoint responded with the respective error code by mistake
*  An internal Twilio error occurred

#### Possible Solutions
*  Verify that your Webhook Credential is configured correctly
*  Verify that your token endpoint is set up correctly"#,
            TwilioNotificationsError::ErrorCode52135 => r#"## Error - 52135

### Missing subject in APNs certificate

#### Possible Causes 
*  The subject of APNs certificate is missing.

#### Possible Solutions
*  The subject of the APNs certificate is the bundle ID of your application. Make sure you have a bundle ID defined when generating the certificate signing request."#,
            TwilioNotificationsError::ErrorCode52107 => r#"## Error - 52107

### Invalid custom key for GCM/FCM

#### Possible Causes 
*  You have specified an invalid key in the custom key-value pair section of the payload.

#### Possible Solutions
*  GCM/FCM rejects some keys including from, gcm and anything prefixed by google. Make sure you are not using any of these keys."#,
            TwilioNotificationsError::ErrorCode52072 => r#"## Warning - 52072

### RtdNotification: Mqtt client connection not created or closed

#### Possible Causes 
*  Most probably mqtt client had disconnected but registration didn't removed yet.

#### Possible Solutions
*  Check this warning happens only once for certain registration id. "#,
            TwilioNotificationsError::ErrorCode52111 => r#"## Error - 52111

### GCM/FCM unknown error

#### Possible Causes 
*  Unknown GCM/FCM error.

#### Possible Solutions
*  If the problem persists, contact [customer support](https://www.twilio.com/help/contact) and provide the time of the error and details about the requests that are failing."#,
            TwilioNotificationsError::ErrorCode52307 => r#"## Error - 52307

### No users with provided segments

No users were found belonging to the specified segment(s)

#### Possible Causes 
* A notification was sent to one or more user segments, but no users belong to these segments

#### Possible Solutions
* Check if correct segments were specified
* Check if users are assigned to correct segments"#,
            TwilioNotificationsError::ErrorCode52000 => r#"## Error - 52000

### Internal error

#### Possible Causes 
*  An internal error has occurred that prevented Twilio from processing of your request.

#### Possible Solutions
* Check our [status page](http://status.twilio.com) to see if we are having an outage. 
* If our status page has no information, [contact support](https://www.twilio.com/help/contact) with details about the requests that are failing, and we will investigate."#,
            TwilioNotificationsError::ErrorCode52306 => r#"## Error - 52306

### Cannot delete User resource with Bindings

#### Possible Causes 
*  User has at least one Binding.

#### Possible Solutions
*  Make sure you delete all Bindings of a User before you try to delete the User."#,
            TwilioNotificationsError::ErrorCode52136 => r#"## Error - 52136

### Missing payload for APNs delivery

#### Possible Causes 
*  APNs delivery had no payload.

#### Possible Solutions
*  Make sure to provide a payload using at least one of Body, Sound, Action, Data or Aps parameters."#,
            TwilioNotificationsError::ErrorCode52051 => r#"## Error - 52051

### Internal error when sending notification via client connection

#### Possible Causes 
*  Our client connection gateway is potentially unavailable. We apologize.

#### Possible Solutions
* Check our [status page](http://status.twilio.com) to see if we are having an outage. 
* If our status page has no information, [contact support](https://www.twilio.com/help/contact) with details about the requests that are failing, and we will investigate."#,
            TwilioNotificationsError::ErrorCode52174 => r#"## Error - 52174

### Unexpected error response received for Webhook Credentials request

#### Possible Causes 
* Your token endpoint responded with an unexpected error

#### Possible Solutions
* Verify that your Webhook Credential is configured correctly
* Verify that your token endpoint is configured correctly
* If the problem persists, please contact [customer support](https://www.twilio.com/help/contact)."#,
            TwilioNotificationsError::ErrorCode52302 => r#"## ERROR - 52302

### Too many recipients

Request has exceed the maximum recipients limit. No deliveries were attempted. ## Error - 52302

### Request has exceeded the maximum recipients limit. No deliveries were attempted.

#### Possible Causes
An attempt was made to send a broadcast notification that targeted too many recipients.

#### Possible Solutions
*  Send several notifications that target fewer recipients (e.g. by partitioning your audience using more Segments and Tags).
*  If there is no good way to partition your audience, please reach out to Twilio Support through the [Console](https://console.twilio.com) or [Help Center](https://support.twilio.com)."#,
            TwilioNotificationsError::ErrorCode52138 => r#"## Error - 52138

### APNs payload too large

#### Possible Causes 
*  Generated APNs payload exceeded limit.

#### Possible Solutions
*  The maximum payload size allowed for iOS 8+ devices is 2KB for earlier devices it is 256 B. Revise your payload so that it does not exceed this limit."#,
            TwilioNotificationsError::ErrorCode52170 => r#"## Error - 52170

### Too many Webhook Credential requests

#### Possible Causes 
*  You are sending Notifications too quickly
*  Your Webhook Credentials are not being cached correctly

#### Possible Solutions
*  Rate limit your requests accordingly
*  Verify the contents of your token endpoint response"#,
            TwilioNotificationsError::ErrorCode52141 => r#"## Error - 52141

### The provided APNs device token has been unregistered

#### Possible Causes 
*  APNs notification delivery rejected because the app has either been uninstalled or notifications have been disabled.
#### Possible Solutions
* If the offending Binding still exists, remove it manually or avoid sending Notifications to it.
* If the problem persists, please contact [customer support](https://www.twilio.com/help/contact)."#,
            TwilioNotificationsError::ErrorCode52161 => r#"## Error - 52161

### Empty Credentials

#### Possible Causes 
*  Credentials are invalid or do not exist for provided credential sid.

#### Possible Solutions
*  Make sure you have credential is valid."#,
            TwilioNotificationsError::ErrorCode52301 => r#"## Warning - 52301

### Destination list for requested parameters is empty

#### Possible Causes 
*  No bindings were selected by the provided combination of parameters.

#### Possible Solutions
*  This may have been your intention, in that case no action is needed.
*  In case you expected to notify some of your bindings then review your parameters and bindings/registrations."#,
            TwilioNotificationsError::ErrorCode52142 => r#"## Error - 52142

### The provided APNs device token is not correct

#### Possible Causes 
*  The APNs device token is either malformed or missing.

#### Possible Solutions
*  Please verify that you're capturing the device token correctly.
*  If the problem persists, please contact [customer support](https://www.twilio.com/help/contact)."#,
            TwilioNotificationsError::ErrorCode52303 => r#"## Error - 52303

### Concurrent User update

Updating the same User resource in multiple concurrent requests may cause data inconsistency. This may result in that searches using multiple Segments will not find your User resource even if it should belong to all the specified Segments.

#### Possible Causes 
* Updating the same User resource (e.g. adding or removing it to/from Segments) in multiple requests concurrently.

#### Possible Solutions
* Change implementation so that if you need to add a User to multiple Segments at once you wait for each request to complete before trying the next one.
* For past errors, just read the affected User resource by using the [Read User API](https://www.twilio.com/docs/notify/api/users#retrieve-a-user) this will fix the data inconsistency."#,
            TwilioNotificationsError::ErrorCode52148 => r#"## Error - 52148

### Expired APNs provider token used

#### Possible Causes 
* The APNs provider token has been in the cache for too long
* The APNs provider token is generated incorrectly

#### Possible Solutions
* Verify that your Webhook Credential is configured correctly
* Verify that your token endpoint is configured correctly
* If the problem persists, please contact [customer support](https://www.twilio.com/help/contact)."#,
            TwilioNotificationsError::ErrorCode52212 => r#"## Error 52212

### Missing Alexa skill ID

Your Alexa message was rejected because the Alexa Skill ID was missing. This specific message was not sent.

### Possible causes
- You have not specified the Alexa Skill ID in your Notify Service.

### Possible solutions
- Connect your Alexa Skill via the [Channels console](https://www.twilio.com/console/channels) then connect your skill to your [Notify Service](https://www.twilio.com/console/notify/services)."#,
            TwilioNotificationsError::ErrorCode52003 => r#"## Error - 52003

### Invalid Credential Type

#### Possible Causes 
*  Type of credential provided in access token does not match that of the registration. For example you included the SID of a APS type Credential in the access token for an Android application using GCM.

#### Possible Solutions
*  Make sure you provide the right Credential SID in the access token."#,
            TwilioNotificationsError::ErrorCode52168 => r#"## Error - 52168

### Webhook Credentials request timed out

#### Possible Causes 
*  The request attempt to the URL specified in your Webhook Credential timed out

#### Possible Solutions
*  Poor network conditions between Twilio and your endpoint
*  Large delay during request processing by your endpoint"#,
            TwilioNotificationsError::ErrorCode52147 => r#"## Error - 52147

### Too many APNs provider token updates"

#### Possible Causes 
* A new APNs provider token is being generated too often
* The APNs provider token is not cached by Twilio

#### Possible Solutions
* Verify that your Webhook Credential is configured correctly
* Verify that your token endpoint is configured correctly
* If the problem persists, please contact [customer support](https://www.twilio.com/help/contact)."#,
            TwilioNotificationsError::ErrorCode52002 => r#"## Error - 52002

### Invalid Credential Sid
In order to send push notifications to iOS and Android devices you need to configure a Credential that allows Twilio to send notifications to your app. 

You can configure these at different places depending on which Twilio product you use:

* Chat: Configure in access token
* Notify: Configure default Credential in Notify Service or Binding specific one in Binding
* Voice SDK: Configure in access token

#### Possible Causes 
*   Credential SID is not configured correctly

#### Possible Solutions
* If using Chat include Credential SID in access token. Check out this [guide](https://www.twilio.com/docs/api/ip-messaging/guides/push-notifications-ios)
*  If using Notify, either configure default Credential SID in your [Service](https://www.twilio.com/console/notify/services) or provide the Credential SID in the [Create Binding](https://www.twilio.com/docs/api/notify/rest/bindings#create-a-binding) request
* If using Voice SDK, include Credential SID in access token. Check out our [documentation](https://www.twilio.com/docs/api/voice-sdk) for details"#,
            TwilioNotificationsError::ErrorCode52139 => r#"## Error - 52139

### APNs service shutdown

#### Possible Causes 
*  APNs delivery rejected because APNs service is shutting down (e.g. for maintenance).

#### Possible Solutions
*  If the problem persists, please contact [customer support](https://www.twilio.com/help/contact)."#,
            TwilioNotificationsError::ErrorCode52101 => r#"## Error - 52101

### Invalid GCM Api Key or FCM Secret

#### Possible Causes 
*  The GCM API Key or FCM Secret provided is invalid.

#### Possible Solutions
* Provide a valid GCM API Key or FCM Secret"#,
            TwilioNotificationsError::ErrorCode52173 => r#"## Error - 52173

### Unexpected Binding Type used for Webhook Credentials request

#### Possible Causes 
* Your token endpoint responded with an error indicating that the Identifier used was not correct

#### Possible Solutions
* Verify that your Webhook Credential is configured correctly
* Verify that your token endpoint is configured correctly
* If the problem persists, please contact [customer support](https://www.twilio.com/help/contact)."#,
            TwilioNotificationsError::ErrorCode52106 => r#"## Error - 52106

### Notification too large for GCM/FCM

#### Possible Causes 
*  Payload generated for GCM/FCM exceeded the 4 KB limit.

#### Possible Solutions
*  Reduce the size of the payload."#,
            TwilioNotificationsError::ErrorCode52110 => r#"## Error - 52110

### GCM/FCM service unavailable

#### Possible Causes 
*  GCM/FCM service is temporarily unavailable or Twilio cannot connect to it.

#### Possible Solutions
*  Check our [status page](http://status.twilio.com) to see if we are having an outage. 
*  If the problem persists, [contact support](https://www.twilio.com/help/contact) and we will investigate."#,
            TwilioNotificationsError::ErrorCode52201 => r#"## Error - 52201

### RtdNotification: Too many Facebook notifications from same page ID

#### Possible Causes 
* Customer is trying to broadcast Facebook messenger notifications 

#### Possible Solutions
* Please notify RTD Notifications team if this error starts occurring."#,
            TwilioNotificationsError::ErrorCode52134 => r#"## Error - 52134

### Invalid APNs device token

#### Possible Causes 
*  APNs device token format is invalid.
*  Trying to use a development certificate for a production application or vice-versa.

#### Possible Solutions
*  Make sure you do not modify the device token before providing it to Twilio. If using a command line tool make sure to url-encode the token.
*  Make sure that you are using a development certificate for a development application and a production certificate for a production application.
	* If you are using a traditional certificate, you can tell its environment by looking at the common name field
		* For a development certificate, it will start with: Apple Development IOS Push Services
		* For a production certificate, it will start with: Apple Push Services
	* If you are using a universal certificate, you can use it for both development and production applications. However you have to make sure that you have properly indicated the type of application by setting the sandbox parameter when you create the credential. You can check the sandbox flag of your credential via the [REST API](/docs/api/ip-messaging/rest/credentials#action-get)."#
        }
    }
}
