// AUTO-GENERATED. DO NOT MODIFY. YOUR CHANGES WILL BE OVER-WRITTEN
use crate::products::TwilioProgrammableSmsError;
use standard_error::traits::StandardErrorDocsTrait;

impl StandardErrorDocsTrait for TwilioProgrammableSmsError {
    fn docs(&self) -> &'static str {
        match self {
            TwilioProgrammableSmsError::ErrorCode30111 => r#"## ERROR - 30111

### Url is on a deny list

The URL provided is blocked. The URL provided is blocked.

#### Possible Causes
- Fraudulent activity has been detected on this url
- Domain owner has requested Twilio not link to their site

#### Possible Solutions
Please reach out to Twilio support"#,
            TwilioProgrammableSmsError::ErrorCode57016 => r#"## ERROR - 57016

### 'Topic' is empty

 'Topic' is not specified

#### Possible Causes
'Topic' is not specified

#### Possible Solutions
Confirm a valid 'Topic' is being passed in request. "#,
            TwilioProgrammableSmsError::ErrorCode30118 => r#"## ERROR - 30118

### Private key is invalid

Unable to parse private key We could not parse the private key uploaded in your request.

#### Possible Causes
- Data in private key may be corrupted
- Private key info could incorrectly formatted
- Private key data could be missing

#### Possible Solutions
Re-generate and re-upload private key."#,
            TwilioProgrammableSmsError::ErrorCode21725 => r#"## ERROR - 21725

### Brand can only be updated when in FAILED state

 Brand can only be updated when in FAILED state. Contact support for help.

#### Possible Causes
Invalid Brand status

#### Possible Solutions
- If Brand is REGISTERED, and needs to be updated, contact Twilio support
- If Brand is IN_PROGRESS, wait for it to exit IN_PROGRESS"#,
            TwilioProgrammableSmsError::ErrorCode30006 => r#"## ERROR - 30006

### Landline or unreachable carrier

The destination number is unable to receive this message. Potential reasons could include trying to reach a landline or, in the case of short codes, an unreachable carrier. ### Message Delivery - Landline or unreachable carrier

The destination number is unable to receive this message.


#### Possible Causes
*  The destination number is unable to receive this message. Potential reasons
could include trying to reach a landline or, in the case of short codes, an
unreachable carrier.
* Your message was sent to a landline, or an unreachable carrier for this phone number type.

#### Possible Solutions
* Use [Lookup](https://www.twilio.com/docs/lookup/v2-api/line-type-intelligence) to determine if the number is indeed a landline. If not, try again with a different phone number type. Check out the video linked below for more information. 

https://www.youtube.com/watch?v=d8GfbPBaSuM"#,
            TwilioProgrammableSmsError::ErrorCode30117 => r#"## ERROR - 30117

### Certificate cannot be parsed

Certificate cannot be parsed We could not parse the certificate uploaded in your request.

#### Possible Causes
- Data in Certificate may be corrupted
- Certificate info could incorrectly formatted
- Certificate data could be missing

#### Possible Solutions
Re-generate certificate ensuring it's in PEM format: https://en.wikipedia.org/wiki/Privacy-Enhanced_Mail"#,
            TwilioProgrammableSmsError::ErrorCode30107 => r#"## ERROR - 30107

### Domain private certificate has not been uploaded

The private certificate required for Twilio to process shortening requests through this domain has not been uploaded Twilio requires a private certificate for us to generate and process shortened HTTPS links on your domain. This certificate has not been provided for this domain.

#### Possible Causes
Setup and configuration wasn't completed for this domain

#### Possible Solutions
Please go to the organizations view for your account and ensure that the private certificate has been uploaded for this domain"#,
            TwilioProgrammableSmsError::ErrorCode57013 => r#"## ERROR - 57013

### 'Topic' is absent

 'Topic' is not specified

#### Possible Causes
'Topic' is not specified

#### Possible Solutions
Confirm a valid 'Topic' is being passed in request. "#,
            TwilioProgrammableSmsError::ErrorCode57020 => r#"## ERROR - 57020

### Authorization failed

Authorization failed Authorization failed

#### Possible Causes
Authorization failed

#### Possible Solutions
Confirm a valid 'Authorisation' header is being passed in the request"#,
            TwilioProgrammableSmsError::ErrorCode30019 => r#"## ERROR - 30019

### Content size exceeds carrier limit

Message failed because the size of the content associated with the message exceeded carrier limit 

#### Possible Causes
Content size exceeded carrier limit could be caused by to many characters, or bytes in the message. UCS-2 encoded messages could have different limitations than GSM encoded messages. 



#### Possible Solutions
Content size should be within [carrier limits](https://support.twilio.com/hc/en-us/articles/360018832773-Twilio-Programmable-SMS-Supported-File-Types-and-Size-Limits-for-MMS-Media-Messages). 

Message size should be within the [carrier limits](https://support.twilio.com/hc/en-us/articles/360033806753-Maximum-Message-Length-with-Twilio-Programmable-Messaging). "#,
            TwilioProgrammableSmsError::ErrorCode30043 => r#"## ERROR - 30043

### International SMS via Domestic Gateway

 Message Misrouted as Domestic

#### Possible Causes
The mobile operator flagged the SMS as an international message sent through the domestic gateway.

#### Possible Solutions
Check the SMS Guidelines for the destination country for any restrictions on domestic or international traffic:
- If the message is genuinely domestic, open a support request and provide evidence to clarify.
- If the message is international, open a support request  and request to move your messages to appropriate international gateways."#,
            TwilioProgrammableSmsError::ErrorCode90009 => r#"## ERROR - 90009

### The message SID already exists.

 Message SID already exists

#### Possible Causes
* Message with the same SID already exists. Conflict when creating a new message.

#### Possible Solutions
* Change or generate new message SID."#,
            TwilioProgrammableSmsError::ErrorCode92008 => r#"## ERROR - 92008

### Unsupported Content Type

Creating new twilio/button templates is no longer supported. Creating new twilio/button templates is no longer supported.

#### Possible Causes
You attempted to create template of type twilio/button which is no longer supported. 

#### Possible Solutions
To create button templates, please use the twilio/call-to-action or twilio/quick-reply template types."#,
            TwilioProgrammableSmsError::ErrorCode90007 => r#"## ERROR - 90007

### Invalid validity period value

 ## Error - 90007

### Invalid validity period value

Validity period can only be set to integers between 1 and 36,000 seconds (10 hours max).

#### Possible Causes
Invalid validity period value

#### Possible Solutions
Fix validity period value"#,
            TwilioProgrammableSmsError::ErrorCode30124 => r#"## ERROR - 30124

### MessagingServiceSID cannot be empty or null

MessagingServiceSID is missing MessagingServiceSID is missing

#### Possible Causes
User did not include messagingServiceSid field

#### Possible Solutions
Please follow doc for setting up messaging service sid"#,
            TwilioProgrammableSmsError::ErrorCode63016 => r#"## ERROR - 63016

### Failed to send freeform message because you are outside the allowed window. If you are using WhatsApp, please use a Message Template.

 

#### Possible Causes
* This message failed to be delivered to the user because it was sent outside the messaging channel's allowed conversation window. For WhatsApp messages initiated by the business, you must use a pre-defined template. 
* If the message was created with Content API or Content Editor it may be one of the following 3 issues: 
 * The customer has not included a messaging service SID
 * The customer is still using "body" instead of "Content SID" and "Content Variables"
 * The customer is using content type that is not supported for business initiated conversations, e.g. listpicker or location
* For Facebook Messenger messages initiated by the business, only publishers registered with the Facebook News Page Index (NPI) are allowed to send notification messages outside the 24-hour window.

#### Possible Solutions
* [Send a WhatsApp message using a template](https://www.twilio.com/docs/whatsapp/api#send-a-whatsapp-message-using-a-template)
* [Include all the necessary parameters for a Content API or Content Editor Template and make sure you aren't sending with the wrong parameters](https://www.twilio.com/docs/content/send-templates-created-with-the-content-editor)
* If you are a publisher, register your Facebook Page with the [Facebook News Page Index](https://www.facebook.com/business/help/316333835842972?id=644465919618833)"#,
            TwilioProgrammableSmsError::ErrorCode21654 => r#"## ERROR - 21654

### ContentSid Required

 A ContentSid must be specified if ContentVariables are provided in the request.

#### Possible Causes
You attempted to send a message with the ContentVariables parameter but without ContentSid.

#### Possible Solutions
Please provide a ContentSid when the request includes the ContentVariables parameter. Review [this documentation](https://www.twilio.com/docs/content/send-templates-created-with-the-content-template-builder) for how to send Content Templates."#,
            TwilioProgrammableSmsError::ErrorCode30409 => r#"## ERROR - 30409

### This message cannot be canceled

 

#### Possible Causes
Attempted to cancel the message after it has reached a [finalized state](https://support.twilio.com/hc/en-us/articles/223134347-What-are-the-Possible-SMS-and-MMS-Message-Statuses-and-What-do-They-Mean-#:~:text=Finalized%20Message%20Delivery%20Status&text=Twilio%20has%20not%20received%20updated%20delivery%20information%20for%20your%20message.&text=Twilio%20has%20received%20a%20delivery,of%20the%20destination%20handset%2C%20etc.).

#### Possible Solutions
N/A"#,
            TwilioProgrammableSmsError::ErrorCode30119 => r#"## ERROR - 30119

### Certificate and private key pair is invalid

Private key and certificate mismatch The provided private key and certificate are both are valid but they do not match

#### Possible Causes
- Certificate and key are for differing domains

#### Possible Solutions
Please regenerated and reupload key and certificate ensuring they are both for the correct domain."#,
            TwilioProgrammableSmsError::ErrorCode21606 => r#"## ERROR - 21606

### The 'From' phone number provided is not a valid message-capable Twilio phone number for this destination/account

 You can only send SMS messages from a phone number, Alphanumeric Sender ID or [short code](https://www.twilio.com/docs/glossary/what-is-a-short-code) provided by or ported to Twilio. Phone numbers must be SMS-capable. For Short Codes, the `From` number must be in the same country as the `To` number.

Visit the [phone numbers page](https://www.twilio.com/console/phone-numbers/incoming) of your account portal to view a list of phone numbers that you own and to determine whether they are SMS-capable.

#### Possible Causes
* The number you are using may not be capable of sending messages.
* The number may be formatted incorrectly. Twilio accepts numbers in [E.164 format](https://www.twilio.com/docs/glossary/what-e164).
* If the number is a Short Code, it must be associated with the same country as the destination address.
* The number you are using is in the process of porting/hosting.
* The number you are using does not belong to the account whose credentials are present in the API request.


#### Possible Solutions
* Check that you are using a Twilio phone number with SMS capabilities. You can see your purchased phone numbers and their capabilities in the [Twilio console](https://www.twilio.com/console/phone-numbers/incoming).
* Ensure the number you are using is in [E.164 format](https://www.twilio.com/docs/glossary/what-e164). 
* If you are sending from a Short Code, verify that the country you are sending to matches the country of the Short Code.
* If you have ported/hosted the `From` number, ensure that the process is complete. You can follow up on this in the Twilio console for [porting](https://console.twilio.com/us1/develop/phone-numbers/port-host/porting-requests?frameUrl=%2Fconsole%2Fphone-numbers%2Fporting-requests%3Fx-target-region%3Dus1) and [hosting](https://console.twilio.com/us1/develop/phone-numbers/port-host/hosted-numbers?frameUrl=%2Fconsole%2Fphone-numbers%2Fhosted%3Fx-target-region%3Dus1).
* Check that you are using account credentials that correspond with the account that owns the phone number.
* If you are sending messages with test credentials, verify that you are using one of the [available test credential "magic phone numbers"](https://www.twilio.com/docs/iam/test-credentials#test-sms-messages-parameters-From).
"#,
            TwilioProgrammableSmsError::ErrorCode63031 => r#"## ERROR - 63031

### Channels message cannot have same 'From' and 'To'

 Channels message should not have same `From` and `To` number

#### Possible Causes
Incorrect `To` and `From` number in your API Request

#### Possible Solutions
Review your `To` and `From` parameters and include the correct values. "#,
            TwilioProgrammableSmsError::ErrorCode23004 => r#"## WARNING - 23004

### Message Redaction Incompatible Configuration: Advanced Opt-Out

 Phone Number Redaction is enabled for your account. Advanced Opt-Out is not compatible with Phone Number Redaction and should be disabled.

#### Possible Causes
* You have Advanced Opt-Out enabled on your Messaging Service. Advanced Opt-Out requires Twilio to save non-redacted phone numbers of customers who have opted out, and is incompatible with Phone Number Redaction.

#### Possible Solutions
* To resolve this issue, your application needs to be able to handle [opt-out keyword messages](https://support.twilio.com/hc/en-us/articles/223134027-Twilio-support-for-opt-out-keywords-SMS-STOP-filtering-) from end users.
* Users who reply STOP, STOPALL, UNSUBSCRIBE, CANCEL, END, or QUIT should be added to a block list in your application to ensure you do not send them any further messages, unless they opt-in again.
* Users who reply HELP or INFO should be sent an informational message about your application or service.
* Users who reply START, YES, or UNSTOP should be removed from your "STOP" block list so they can receive messages from you again.
* Once you are ready to disable Twilio's built-in opt-out handling and process the above keywords in your own application, please contact Twilio Support through the [Console](https://console.twilio.com) or [Help Center](https://support.twilio.com) to disable all Twilio keyword handling for your account, including Advanced Opt-Out."#,
            TwilioProgrammableSmsError::ErrorCode11751 => r#"## ERROR - 11751

### Media Message - Media exceeds messaging provider size limit

The total size of the media message request exceeds the maximum size limit for the channel. ## Error - 11751

### Media Message -> Media exceeds messaging provider size limit

Maximum size limit depends on the messaging channel. 
* For an MMS message, the size limit is [5MB](https://www.twilio.com/docs/sms/accepted-mime-types).
* For a WhatsApp message, the size limit is [16MB](https://www.twilio.com/docs/whatsapp/guidance-whatsapp-media-messages).


#### Possible Causes
The total size of the media message request exceeds the maximum size limit for the channel.

#### Possible Solutions
If you are sending multiple media files on a message, split them into multiple requests to reduce the size of each request."#,
            TwilioProgrammableSmsError::ErrorCode30036 => r#"## ERROR - 30036

### Validity Period Expired

 The specified validity period on this message has elapsed.

#### Possible Causes
The message has spent more than the specified amount of time in Twilio's system. This can be due to a long time spent in the queue due to a combination of your [messaging rate limits](https://support.twilio.com/hc/en-us/articles/115002943027-Understanding-Twilio-Rate-Limits-and-Message-Queues), the number of messages submitted in rapid succession, and the amount of time you have allotted via the [ValidityPeriod setting](https://www.twilio.com/blog/validity-period-promotional-verification-use-cases) in your messaging service or on the message itself.

#### Possible Solutions
You can retry the message with a longer validity period and avoid sending very large batches of messages at a rate that far exceeds your messaging rate limits."#,
            TwilioProgrammableSmsError::ErrorCode30121 => r#"## ERROR - 30121

### Fallback URL is missing

Fallback URL is missing, please add an URL. Fallback URL is missing

#### Possible Causes
User did not include fallbackUrl field

#### Possible Solutions
Please follow doc for setting up fallback URL."#,
            TwilioProgrammableSmsError::ErrorCode21611 => r#"## ERROR - 21611

### This 'From' number has exceeded the maximum number of queued messages

You have attempted to enqueue too many messages for a given 'From' number. Twilio queues messages based on the sending rate of a sender or an account. For example a US long code number can send one message [segment](https://www.twilio.com/blog/what-the-heck-is-a-segment.html) per second, and a short code may send over 100 message segments per second.

Messages can only be queued for 10 hours and then they automatically fail. This queue length limit can be reduced by setting a lower [Validity Period](https://www.twilio.com/en-us/blog/validity-period-promotional-verification-use-cases) within your Messaging Service settings or in your API requests to send messages.

You can monitor the length on your queue via the [Queue Insights dashboard](https://console.twilio.com/us1/monitor/insights/sms?frameUrl=/console/sms/insights/delivery?x-target-region%3Dus1&currentFrameUrl=/console/sms/insights/delivery?x-target-region%3Dus1%26__override_layout__%3Dembed%26q%3DKGFjdGl2ZUluc2lnaHRzVmlldzpsYXRlbmN5KQ%253D%253D%26bifrost%3Dtrue).

For more information please view this [FAQ](https://support.twilio.com/hc/en-us/articles/115002943027-Understanding-Twilio-Rate-Limits-and-Message-Queues).

When a queue for a particular 'From' number exceeds 10 hours, this error will be thrown when attempting to create additional messages on the same number.

Note: If you are using a Messaging Service and the numbers in the pool experience queue overload, messages will instead fail after creation with [error 30001](https://www.twilio.com/docs/api/errors/30001).


#### Possible Causes
You have attempted to enqueue more than 10 hours' worth of messages on a single Twilio phone number or sender (based on that number's [MPS rate limit](https://support.twilio.com/hc/en-us/articles/115002943027-Understanding-Twilio-Rate-Limits-and-Message-Queues)).

#### Possible Solutions
* Slow down your sending rate to avoid queuing on your Twilio number.
* Consider using a [Messaging Service](https://www.twilio.com/docs/messaging/services) to better distribute message throughput, if appropriate for your use case.
* Increase your [validity period](https://www.twilio.com/docs/messaging/api/message-resource#create-a-message-resource) if you have it set to lower than 10 hours
"#,
            TwilioProgrammableSmsError::ErrorCode30027 => r#"## ERROR - 30027

### US A2P 10DLC - T-Mobile Daily Message Limit Reached

As part of the US A2P 10DLC initiative, T-Mobile has instituted a daily message limit. You are receiving this error code because you have reached this limit. Please note that this daily limit is based on the sum of SMS segments and MMS messages you send to T-Mobile subscribers on a 24-hour basis, and is reset daily at midnight PT. As you have consumed 100% of your limit, subsequent messages you send to T-Mobile are likely to be filtered. When filtering happens, you will start to receive 30023 error codes and must wait until midnight PT to resume message sending.

For more information on T-Mobile daily limit, please refer to [T-Mobile daily message limits for long code messaging with A2P 10DLC](https://support.twilio.com/hc/en-us/articles/1260804800549-T-Mobile-daily-message-limits-for-long-code-messaging-with-A2P-10DLC). For more information on US A2P 10DLC, please refer to [US A2P 10DLC Documentation](https://support.twilio.com/hc/en-us/articles/1260800720410-What-is-A2P-10DLC-).

Note: this error may not be accurate for certain customers such as those who have been approved of the unlimited tier via the [T-Mobile Special Business Review process](https://support.twilio.com/hc/en-us/articles/4403550579739-T-Mobile-Special-Business-Review-for-A2P-10DLC), those who run non-profit campaigns, and others.

#### Possible Causes
* You have consumed 100% of your T-Mobile daily message limit for the day.

#### Possible Solutions
* If you infrequently exceed your T-Mobile daily message limit, you can consider prioritizing messages (e.g., OTP codes over marketing materials).
* If you frequently exceed your T-Mobile daily message limit, you may be able to increase your daily limit: 
  * For unregistered customers, please consider registering in the [Twilio Console - Trust Hub](https://console.twilio.com/us1/account/trust-hub/a2p?frameUrl=%2Fconsole%2Ftrust-hub%2Fa2p-messaging%2Fwizard%3Fx-target-region%3Dus1&currentFrameUrl=%2Fconsole%2Ftrust-hub%2Fa2p-messaging%2Fwizard%3F__override_layout__%3Dembed%26x-target-region%3Dus1%26activeStep%3DbusinessProfile%253AbusinessProfile%253AcreateBusinessProfileAddress%26bifrost%3Dtrue).
 * For registered customers, if you need to send more than 200,000 message segments to T-Mobile per day, please consider submitting a [Special Business Review](https://support.twilio.com/hc/en-us/articles/4403550579739-T-Mobile-Special-Business-Review-for-A2P-10DLC).
* Other useful links:
 * To view the warning and errors, please visit the [debugger](https://console.twilio.com/us1/monitor/logs/debugger/events?frameUrl=%2Fconsole%2Fdebugger%3F__override_layout__%3Dembed%26quickDate%3D24%26x-target-region%3Dus1%26bifrost%3Dtrue&currentFrameUrl=%2Fconsole%2Fdebugger%3F__override_layout__%3Dembed%26bifrost%3Dtrue%26quickDate%3D24%26x-target-region%3Dus1).
 * To view your daily T-Mobile message limit, please visit [Trust Hub - A2P Messaging](https://console.twilio.com/us1/account/trust-hub/a2p?frameUrl=%2Fconsole%2Ftrust-hub%2Fa2p-messaging%3Fx-target-region%3Dus1).
 * To view your daily T-Mobile usage, please use the Messaging Insights tool."#,
            TwilioProgrammableSmsError::ErrorCode63008 => r#"### Channel misconfigured

#### Possible Causes 
*   Could not execute the request or send a message because the Channel has been misconfigured. Please check the Channel instance configuration in Twilio."#,
            TwilioProgrammableSmsError::ErrorCode23002 => r#"## WARNING - 23002

### Message Redaction Incompatible Configuration: Short code "STOP" filtering

 Phone Number Redaction is enabled for your account. Twilio's default opt-out keyword handling ("SMS STOP filtering") is not compatible with Phone Number Redaction and should be disabled.

#### Possible Causes
* Twilio is currently set to handle opt-out keywords for short code numbers on your account. Twilio's opt-out handling requires Twilio to save non-redacted phone numbers of users who have opted out, and is incompatible with Phone Number Redaction.

#### Possible Solutions
* To resolve this issue, your application needs to be able to handle [opt-out keyword messages](https://support.twilio.com/hc/en-us/articles/223134027-Twilio-support-for-opt-out-keywords-SMS-STOP-filtering-) from end users.
* Users who reply STOP, STOPALL, UNSUBSCRIBE, CANCEL, END, or QUIT should be added to a blocklist in your application to ensure you do not send them any further messages, unless they opt-in again.
* Users who reply HELP or INFO should be sent an informational message about your application or service.
* Users who reply START, YES, or UNSTOP should be removed from your "STOP" blocklist so they can receive messages from you again.
* Once you are ready to disable Twilio's built-in opt-out handling and process the above keywords in your own application, please reach out to Twilio Support through the [Console](https://console.twilio.com) or [Help Center](https://support.twilio.com) to disable Twilio's built-in opt-out keyword handling for your account."#,
            TwilioProgrammableSmsError::ErrorCode30011 => r#"## ERROR - 30011

### MMS not supported by the receiving phone number in this region

 ## Error - 30011 

### MMS not supported by the receiving phone number





#### Possible Causes
If you received this error code on an inbound message, there was a failure attempting to receive an incoming MMS, as your Twilio phone number was sent an MMS in a region where Twilio does not support incoming MMS.

If you received this error code on an outbound message, the destination handset either doesn't support MMS, or MMS is not currently enabled on the device.

#### Possible Solutions
For Inbound, it is recommended that you respond to the user sending the MMS to let them know that you cannot receive MMS in this region."#,
            TwilioProgrammableSmsError::ErrorCode30130 => r#"## ERROR - 30130

### Messaging Service SID already belongs in another domain configuration.

Messaging Service SID is already associated with a domain configuration in the account Messaging Service SID is already associated with a domain configuration in the account

#### Possible Causes
One of the messaging service SID in the request body is already associated with a domain in the account

#### Possible Solutions
Try to use any other messaging service SID"#,
            TwilioProgrammableSmsError::ErrorCode21627 => r#"## ERROR - 21627

### Max Price must be a valid float

 Max Price must be a valid positive float

#### Possible Causes
Max Price has a zero or negative value or not a float

#### Possible Solutions
Set Max Price to positive float value"#,
            TwilioProgrammableSmsError::ErrorCode23005 => r#"## WARNING - 23005

### Phone Number Redaction Incompatible Configuration: Fallback to Long Code

 Phone Number Redaction is enabled for your account. Fallback to Long Code is not compatible with Phone Number Redaction and should be disabled.

#### Possible Causes
* You have Fallback to Long Code enabled on your Messaging Service. Fallback to Long Code requires Twilio to retain non-redacted phone numbers of recipients, and is incompatible with Phone Number Redaction.

#### Possible Solutions
* Disable Fallback to Long Code on your Messaging Services using the [Twilio Console](https://www.twilio.com/console/sms/services) or the [REST API](https://www.twilio.com/docs/messaging/services/api).
* If Fallback to Long Code functionality is needed, implement this functionality in your own application."#,
            TwilioProgrammableSmsError::ErrorCode90014 => r#"## ERROR - 90014

### Validity Period should be positive integer

 Validity Period should be positive integer

#### Possible Causes
Validity Period is not positive integer

#### Possible Solutions
Change the Validity Period to be positive integer"#,
            TwilioProgrammableSmsError::ErrorCode92005 => r#"## ERROR - 92005

### ContentSid Required

A ContentSid must be specified if content variables are provided in the request. 

#### Possible Causes
You attempted to send a message with the ContentVariables parameter but without ContentSid

#### Possible Solutions
Please provide a ContentSid when the request includes the ContentVariables parameter"#,
            TwilioProgrammableSmsError::ErrorCode21408 => r#"## ERROR - 21408

### Permission to send an SMS or MMS has not been enabled for the region indicated by the 'To' number

 You have attempted to send an SMS or MMS to a region that has not been enabled in your account's [Messaging Geo-Permissions](https://console.twilio.com/us1/develop/sms/settings/geo-permissions) settings. These settings exist to help you ensure you only send SMS or MMS to the countries or regions you operate in, and avoid unexpected charges.

#### Possible Causes
* You have attempted to send an SMS or MMS to a region that has not been enabled in your account's [Geo-Permissions](https://console.twilio.com/us1/develop/sms/settings/geo-permissions) settings. 

#### Possible Solutions
If you wish to send messages to this region, please enable the relevant permissions on your account using the [Messaging Geographic Permissions](https://console.twilio.com/us1/develop/sms/settings/geo-permissions) page."#,
            TwilioProgrammableSmsError::ErrorCode35125 => r#"## ERROR - 35125

### Maximum limit reached in the account for scheduling messages

 There is a soft limit of 500,000 scheduled messages at any given time, per account_sid, that can be guaranteed. As each scheduled message is sent, more messages can be scheduled up to that limit.

#### Possible Causes
The account_sid already has at least 500,000 messages scheduled.

#### Possible Solutions
Wait for the scheduled messages to be sent. At the send_at time, as the messages are sent (or canceled) the limit will free up. If this limitation is a blocker, kindly contact Twilio Support through the [Console](https://console.twilio.com) or [Help Center](https://support.twilio.com)."#,
            TwilioProgrammableSmsError::ErrorCode21712 => r#"## Error - 21712

### Phone Number or Short Code is associated with another Messaging Service.

A Twilio phone number or short code can only be associated with one Messaging Service at a time.

Please delete the number from its existing Messaging Service before attempting to add it to a new Messaging Service. [See here](https://www.twilio.com/docs/sms/services/api/phonenumber-resource?code-sample=code-delete-phone-number&code-language=curl&code-sdk-version=json) for a code sample. Removing a Twilio number from a Messaging Service will not remove the number from your account."#,
            TwilioProgrammableSmsError::ErrorCode57006 => r#"## ERROR - 57006

### 'EventType' is empty

 'EventType' is not specified

#### Possible Causes
'EventType' is not specified

#### Possible Solutions
Confirm a valid 'EventType' is being passed in request."#,
            TwilioProgrammableSmsError::ErrorCode57007 => r#"## ERROR - 57007

### 'EventType' is absent

 'EventType' is not being passed in request

#### Possible Causes
'EventType' is not being passed in request

#### Possible Solutions
Confirm a 'EventType' is being passed in request."#,
            TwilioProgrammableSmsError::ErrorCode30108 => r#"## ERROR - 30108

### Twilio account does not belong to an organization

Domains are managed at the Organization level. Please attach this account to an organization and proceed with setup for Click Tracking prior to use. Domains are managed at the Organization level. Please attach this account to an organization and proceed with setup for Click Tracking prior to use.

#### Possible Causes
Necessary setup steps for Click Tracking, including creation of an organization have not been completed.

#### Possible Solutions
Please follow setup guides for Click Tracking."#,
            TwilioProgrammableSmsError::ErrorCode57009 => r#"## ERROR - 57009

### 'EventType' is too long

 Specified 'EventType' is too long

#### Possible Causes
Specified 'EventType' is too long

#### Possible Solutions
Confirm a valid 'EventType' is being passed in request."#,
            TwilioProgrammableSmsError::ErrorCode30022 => r#"## ERROR - 30022

### US A2P 10DLC - Rate Limits Exceeded

Your messages to this particular carrier has exceeded the maximum allowable limits for your phone number and/or campaign. * Your message was rejected by the downstream carriers for exceeding the rate limits allowed by your campaign.
* Rarely, you may receive this error for sending too many messages to the same phone number in a short time frame.

#### Possible Causes
* You are sending messages at a rate higher than allowed by the carriers for your campaign type.
* You might be sending too many messages to the same phone number in a short timeframe.

#### Possible Solutions
* Slow down your rate of message sending."#,
            TwilioProgrammableSmsError::ErrorCode30100 => r#"## ERROR - 30100

### Domain SID is invalid

The Domain SID included in your request is not a valid Domain for your Twilio Organization. The Domain SID included in your request is not a valid Domain for your Twilio Organization.

#### Possible Causes
The Domain SID you're using could belong to another Organization. It could also potentially be an issue of copy and pasting. You may be providing a URL instead of a Domain SID in the Domain SID field.

#### Possible Solutions
Check the Domain SID from your Organization through the Admin Center."#,
            TwilioProgrammableSmsError::ErrorCode30002 => r#"## ERROR - 30002

### Account suspended

Your account was suspended between the time of message send and delivery. Please contact Twilio. This message failed because your account was suspended after this message was queued but before it was sent by Twilio.


#### Possible Causes
*  Account was suspended for lack of funds after your message was queued but before it was sent by Twilio.
* Account was suspended because Twilio detected a violation of our Acceptable Use Policy. 


#### Possible Solutions
*  Please [contact Twilio support](https://www.twilio.com/console/support/tickets/create).
"#,
            TwilioProgrammableSmsError::ErrorCode92007 => r#"## ERROR - 92007

### The Content Variables Parameter is invalid

The Content Variables Parameter is invalid 

#### Possible Causes
The Content Variables Parameter is not a JSON String 

#### Possible Solutions
Validate the Content Variables Parameter, it must be a JSON String "#,
            TwilioProgrammableSmsError::ErrorCode21723 => r#"## ERROR - 21723

### Campaign Verify token import already in progress

 The attempt to import a Campaign Verify token to your brand failed because there is already a token that is queued for importing.

#### Possible Causes
Another Campaign Verify token has a status of `IN_PROGRESS` meaning it is already queued for import. Only one token can be imported at a time. 

#### Possible Solutions
Verify that there are not existing vetting attempts for Campaign Verify that are in progress by checking the `vettingStatus` field of the Fetch Vettings API . There should not be a vetting listed with the `vettingProvider: "CampaignVerify"` that has a `vettingStatus` of `IN_PROGRESS`. Please wait until the vetting status moves to `SUCCESS` or `FAILED` before attempting to import a new Campaign Verify token. 
"#,
            TwilioProgrammableSmsError::ErrorCode30009 => r#"## ERROR - 30009

### Missing inbound segment

 One or more segments associated with your multi-part inbound message was not received.

#### Possible Causes
* The mobile network or carrier did not successfully send the entire message to Twilio
* The mobile handset had a malfunction and did not successfully send all segments
* The mobile user was in poor network coverage, causing an incomplete message to be sent

#### Possible Solutions
* Ensure the mobile user is in good network coverage and request they try sending the message again
* If the issue is occurring frequently for multiple users, [contact Twilio Support](https://www.twilio.com/console/support/tickets/create) with Message SIDs of affected messages"#,
            TwilioProgrammableSmsError::ErrorCode63028 => r#"### Number of parameters provided does not match the expected number of parameters

#### Possible Causes 
* Number of parameters provided does not match the expected number of parameters"#,
            TwilioProgrammableSmsError::ErrorCode30116 => r#"## ERROR - 30116

### Certificate or private key or both are missing

Please use PEM format for uploaded certificates Certificates should be uploaded in PEM format. https://en.wikipedia.org/wiki/Privacy-Enhanced_Mail

#### Possible Causes
- Certificate is using a different format
- Certificate has been corrupted

#### Possible Solutions
- Generate a new certificate in PEM format and reupload"#,
            TwilioProgrammableSmsError::ErrorCode63001 => r#"## ERROR - 63001

### Channel could not authenticate the request. Please see Channel specific error message for more information

 The Channel could not authenticate the request. Please see Channel specific error message for more information.

#### Possible Causes
*   The Channel could not authenticate the request. Please see Channel specific error message for more information.
*   Facebook identified a potential security issue and rotated the token
*   The Facebook user who connected the FB page to Twilio has changed their password
*   The user de-authorized the Twilio app

#### Possible Solutions
If the issue persists, you may need to delete the sender and and re-add it."#,
            TwilioProgrammableSmsError::ErrorCode90006 => r#"## ERROR - 90006

### Invalid direction

 Invalid value for direction. Please contact Customer Support.

#### Possible Causes
We're having trouble completing your request

#### Possible Solutions
Contact Customer Support for help"#,
            TwilioProgrammableSmsError::ErrorCode30122 => r#"## ERROR - 30122

### Fallback URL is invalid

Fallback URL is invalid, please check the format of the URL parameter Fallback URL is invalid

#### Possible Causes
The fallbackUrl field has incorrectly formatted URL

#### Possible Solutions
Use a correctly formatted URL in the fallbackUrl field"#,
            TwilioProgrammableSmsError::ErrorCode57018 => r#"## ERROR - 57018

### 'Event' value type must be Map

 Specified 'Event' value type is invalid, Map required

#### Possible Causes
Specified "Event" value must be Map

#### Possible Solutions
Confirm a valid 'Event' is being passed in request. "#,
            TwilioProgrammableSmsError::ErrorCode57003 => r#"## ERROR - 57003

### 'Secret id' is invalid for this Partner

 'Secret id' is invalid for this Partner

#### Possible Causes
'Secret id' is invalid for this Partner

#### Possible Solutions
Confirm a valid 'Secret id' is being passed in request. "#,
            TwilioProgrammableSmsError::ErrorCode30031 => r#"## ERROR - 30031

### Invalid MaxRate

 MaxRate is not a valid float

#### Possible Causes
MaxRate value is not a float or is equal to or less than zero

#### Possible Solutions
Use a positive float value for MaxRate"#,
            TwilioProgrammableSmsError::ErrorCode57004 => r#"## ERROR - 57004

### 'Category' is empty

 'Category' is not specified

#### Possible Causes
'Category' is not specified

#### Possible Solutions
Confirm a valid 'Category' is being passed in request."#,
            TwilioProgrammableSmsError::ErrorCode21614 => r#"## ERROR - 21614

### 'To' number is not a valid mobile number

 You have attempted to send a SMS with a 'To' number that is not 
a valid mobile number. It is likely that the number that you 
have specified is a landline number or is an invalid number. 

#### Possible Causes
* The number you provided may be a landline number.
* The number you provided may be invalid or formatted incorrectly.
* If you are attempting to send SMS to Internet of Things (IoT) or machine-to-machine (M2M) numbers, the numbers may use a non-standard format that Twilio has not added to our number validation system yet.

#### Possible Solutions
* Confirm that the number you are sending to is not a landline, using the [Lookup API](https://www.twilio.com/docs/lookup/v2-api).
* Please verify you have provided a valid mobile number in proper [E.164 format](https://support.twilio.com/hc/en-us/articles/223183008-Formatting-International-Phone-Numbers).
* If you are attempting to send SMS to an IoT or M2M number, check whether the number format is different from the standard mobile numbers in that country or locality. Often, these numbers have additional digits or unusual formats which do not pass Twilio's number validation. If you believe this is the issue, please [contact Twilio Support](https://www.twilio.com/help/contact) for assistance."#,
            TwilioProgrammableSmsError::ErrorCode63023 => r#"### Channel generic error

#### Possible Causes 
* Twilio encountered a generic error for your message."#,
            TwilioProgrammableSmsError::ErrorCode30041 => r#"## ERROR - 30041

### Message from an unregistered number sent to a United Kingdom number

 Message sent to a United Kingdom number may not be delivered because it is being sent from a number that must be registered.

#### Possible Causes
Alphanumeric, Shortcode, Twilio Domestic Longcode must complete and return a Letter of Authorization (LOA) to Twilio in order to continue using Protected Sender IDs to send messages to the United Kingdom. [More details on UK SMS Guidelines](https://www.twilio.com/en-us/guidelines/gb/sms).

#### Possible Solutions
Alphanumeric, Shortcode, Twilio Domestic Longcode must complete and return a Letter of Authorization (LOA) to Twilio in order to continue using Protected Sender IDs to send messages to the United Kingdom. [More details on UK SMS Guidelines](https://www.twilio.com/en-us/guidelines/gb/sms)."#,
            TwilioProgrammableSmsError::ErrorCode21709 => r#"## ERROR - 21709

### Alpha Sender ID is Invalid or Not Authorized for this Messaging Service

 The Alpha Sender ID being added to a Messaging Service is invalid or Alpha Sender IDs have not been enabled for this Messaging Service.

#### Possible Causes
* The Messaging Service is not enabled to support Alpha Sender IDs. 
* The Alpha Sender ID being added to a Messaging Service is invalid. Alphanumeric Sender IDs may be up to 11 characters. They may not be only numbers. Refer to [this page](https://support.twilio.com/hc/en-us/articles/223133867-Using-Alphanumeric-Sender-ID-with-Messaging-Services) for acceptable characters.

#### Possible Solutions
* Follow [these steps](https://support.twilio.com/hc/en-us/articles/223181348-Alphanumeric-Sender-ID-for-Twilio-Programmable-SMS#h_01F4SK0WYHA75NK1DNXGTCFMP8) to verify your Messaging Service is enabled to support Alpha Sender IDs. If it is not, please reach out to Support.
* Verify the Alpha Sender ID you are trying to add to the Messaging Service is valid. View formatting requirements [here](https://support.twilio.com/hc/en-us/articles/223181348-Alphanumeric-Sender-ID-for-Twilio-Programmable-SMS#h_01F4SK0R6NH8AQW36Y4BR16FVA)."#,
            TwilioProgrammableSmsError::ErrorCode21658 => r#"## ERROR - 21658

### Parameter exceeded character limit

 Parameter exceeded character limit

#### Possible Causes
Ensure parameter doesn't exceed character limit during content creation

#### Possible Solutions
Validate parameter in within character limit"#,
            TwilioProgrammableSmsError::ErrorCode21722 => r#"## ERROR - 21722

### Invalid Campaign Verify token

 The attempt to import a Campaign Verify token to your brand failed because your token is invalid.

#### Possible Causes
- The token is not formatted correctly

#### Possible Solutions
Verify that you are using a valid token from Campaign Verify that is not expired and hasn’t been used by another A2P brand. Then, try to import your token again."#,
            TwilioProgrammableSmsError::ErrorCode63038 => r#"## ERROR - 63038

### Account exceeded the daily messages limit

 Account exceeded the maximum amount of messages per 24 hours.

#### Possible Causes
Account has sent too many messages.

#### Possible Solutions
* Wait for the next day or contact support to raise the limit.
* Check your [support center](https://help.twilio.com/tickets) or email inbox/spam folder for outreach from the Twilio team on how to remove/raise your daily limit."#,
            TwilioProgrammableSmsError::ErrorCode57002 => r#"## ERROR - 57002

### 'Secret id' is too long

 Specified 'Secret id' is too long

#### Possible Causes
Specified 'Secret id' is too long.

#### Possible Solutions
Confirm a valid 'Secret id' is being passed in request. "#,
            TwilioProgrammableSmsError::ErrorCode63020 => r#"### Business Manager Account Error

#### Possible Causes 
* Please use Facebook Business Manager to accept an invitation from Twilio to send messages on your behalf. Also verify the WhatsApp billing account associated with your business"#,
            TwilioProgrammableSmsError::ErrorCode63035 => r#"## ERROR - 63035

### This operation is blocked because the RCS agent has not launched, the recipient has not accepted the invitation to become a tester, or the RCS sender only works in certain regions.

 This operation is blocked because the RCS sender has not launched and the recipient has not been invited and accepted the invitation to become a tester. This may also happen if you are attempting to send to a device with a country code from a region that is not supported by that sender.

#### Possible Causes
Device is not RCS capable, recipient has not accepted to become a tester, or device is not reachable from the RCS sender.

#### Possible Solutions
If device is RCS capable, accept invitation to become a tester. If the device has already accepted the invitation, please confirm that you are not attempting to send from an RCS sender that is regionally restricted."#,
            TwilioProgrammableSmsError::ErrorCode63011 => r#"### Channel returned an invalid request error

#### Possible Causes 
*   Twilio could not process your error because the channel provider failed to receive capabilities for your request."#,
            TwilioProgrammableSmsError::ErrorCode92004 => r#"## ERROR - 92004

### Invalid language code

Invalid language code 

#### Possible Causes
You attempted to create content, but you sent an invalid 'language code'

#### Possible Solutions
Please review that the language code is supported"#,
            TwilioProgrammableSmsError::ErrorCode30133 => r#"## ERROR - 30133

### The certificate could not be uploaded.

The certificate could not be uploaded due to invalid system state. 

#### Possible Causes
This might happen if an associated domain resource has been deleted and recreated since the last certificate upload.

#### Possible Solutions
Please try uploading again in a few minutes."#,
            TwilioProgrammableSmsError::ErrorCode21720 => r#"## ERROR - 21720

### A2P Use Case is Invalid

 The A2P use case you are trying to specify in the request is not a valid use case.

#### Possible Causes
The A2P use case specified in the request is not a valid A2P use case.

#### Possible Solutions
Verify that you are spelling the use case correctly in your request and that it is using all uppercase letters. The following resource can also be used to retrieve all valid A2P use cases for a specific Messaging Service: https://messaging.twilio.com/v1/Services/MGXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXX/Compliance/Usa2p/Usecases"#,
            TwilioProgrammableSmsError::ErrorCode57011 => r#"## ERROR - 57011

### Unsupported Partner name

 Unsupported Partner name

#### Possible Causes
Specified 'PartnerName' is not supported

#### Possible Solutions
Confirm a valid 'PartnerName' is being passed in request."#,
            TwilioProgrammableSmsError::ErrorCode21711 => r#"## WARNING - 21711

### Phone Number, Shortcode, Destination AlphaSender, and Global AlphaSender is not associated to the specified Messaging Service.

 ## Error - 21711

### Phone Number, Shortcode, Destination AlphaSender, and Global AlphaSender is not associated to the specified Messaging Service.

#### Possible Causes
* The sender was never associated with a given Messaging Service. 

#### Possible Solutions
Please add this number to a specified Messaging Service. [See here](https://www.twilio.com/docs/messaging/services/api/phonenumber-resource#create-a-phonenumber-resource-add-a-phone-number-to-a-messaging-service) for a code sample. "#,
            TwilioProgrammableSmsError::ErrorCode30400 => r#"## ERROR - 30400

### Parameters are not valid 

 There is an issue with the request body.

#### Possible Causes
- POST body is missing parameters Body or Status.
- Attempted to cancel the message and redact the message body at the same time.
- Value for Status is invalid.
- Message body redaction request does not have empty Body parameter.

#### Possible Solutions
- To cancel the message, include Status: canceled.
- To redact the message body, ensure Body is empty. More details [here](https://support.twilio.com/hc/en-us/articles/223133687-Deleting-messages-message-media-or-message-bodies).
- If you want to cancel and redact the message body: First, request the message to be canceled. Then, request for message body redaction. More details [here](https://support.twilio.com/hc/en-us/articles/223133687-Deleting-messages-message-media-or-message-bodies)."#,
            TwilioProgrammableSmsError::ErrorCode21605 => r#"## ERROR - 21605

### Maximum body length is 160 characters (old API endpoint)

You are using the deprecated /SMS/Messages API endpoint, which supports up to 160 characters per message. To send up to 1600 characters per message, switch to our upgraded /Messages endpoint. ## Error - 21605

### Maximum body length is 160 characters

This error is specific to the deprecated /SMS/Messages API endpoint. When using this endpoint, your message body must be 160 characters or less. 

You can send messages up to 1600 characters long by using the [/Messages](https://www.twilio.com/docs/sms/api/message#create-a-message-resource) API endpoint.

For more info, please refer to [our FAQ on message size constraints](https://support.twilio.com/hc/en-us/articles/223133347-Size-limitations-of-combining-text-and-images).

#### Possible Causes
The SMS body contains too many characters.

#### Possible Solutions
Reduce the number of characters in your message body and review our FAQ on message size constraints."#,
            TwilioProgrammableSmsError::ErrorCode21612 => r#"## ERROR - 21612

### Message cannot be sent with the current combination of "To" and/or "From" parameters

   Message cannot be sent with the current combination of "To" and/or "From" parameters.

#### Possible Causes
 **Sender ID restrictions in the destination country:** Many countries limit which numbers, short-codes, and/or alphanumeric senders can be used in that region. Consult the [SMS guidelines for the destination region](https://www.twilio.com/en-us/guidelines/sms).

**Sender ID restrictions on the destination network:** Some networks do not support receiving messages from shortcodes, others might allow SMS from shortcodes but cannot receive MMS messages from shortcodes. Similar restrictions may apply to the use of longcodes from countries that do not match the destination country. Consult the [SMS guidelines for the destination region](https://www.twilio.com/en-us/guidelines/sms).

**Alphanumeric senderIDs:** If you are using an alphanumeric sender ID, the 'To' number must be in a country where alphanumeric sender IDs are supported. Certain countries require pre-registration of alphanumeric sender IDs. A list of countries where alphanumeric sender ID is supported and whether or not pre-registration is required can be found [here](https://support.twilio.com/hc/en-us/articles/223133767-International-support-for-Alphanumeric-Sender-ID).

**Number formatting:** The format you used for the “To” or “From” number may not be formatted using the [E.164 format](https://www.twilio.com/docs/glossary/what-e164). Twilio standardizes numbers using the E.164 format. If the “To” or “From” number of this message were altered by Twilio in a way that you didn’t expect, try re-submitting the message with E.164 formatted addresses.

**Destinations where Twilio does not have connectivity:**  It is possible that Twilio does not yet have service with the carrier you are trying to reach. You can lookup the destination network via [the lookup api](https://www.twilio.com/docs/lookup/v2-api).

#### Possible Solutions
Consult the linked documentation for each cause. You can also try sending again with a different "To" and "From" combination."#,
            TwilioProgrammableSmsError::ErrorCode57017 => r#"## ERROR - 57017

### 'Topic' is too long

 Specified 'Topic' is too long

#### Possible Causes
Specified 'Topic' is too long

#### Possible Solutions
Confirm a valid 'Topic' is being passed in request. "#,
            TwilioProgrammableSmsError::ErrorCode21619 => r#"## ERROR - 21619

### A Message Body, Media URL or Content SID is required

A Message Body, Media URL or Content SID is required A Message Body, Media URL or Content SID is required

#### Possible Causes
You are sending a message using a Messaging Service without specifying a `Body`,`Media URL` or `Content SID`.

#### Possible Solutions
* Ensure you are specifying the correct API Parameters when sending your message.
* If you are using Content Templates, you need to specify a [Content SID](https://www.twilio.com/docs/content/send-templates-created-with-the-content-template-builder), otherwise you will need to send a [“Body” and/or “Media URL”](https://www.twilio.com/docs/messaging/api/message-resource#create-a-message-resource).
"#,
            TwilioProgrammableSmsError::ErrorCode30007 => r#"## ERROR - 30007

### Message filtered

Your message content was flagged as going against carrier guidelines. Your message was filtered (blocked) by Twilio or by the carrier. This may be done by Twilio for violating Twilio’s [Messaging Policy](https://www.twilio.com/en-us/legal/messaging-policy) or [Acceptable Use Policy](https://www.twilio.com/en-us/legal/aup), or by a wireless carrier for violating carrier rules or regulations.

Examples of messaging that would be blocked by Twilio are spam, phishing, and fraud. Twilio’s filtering system is in place to protect mobile subscribers from spam or other forms of malicious or unwanted messages.

Wireless carriers have filtering systems in place as well. These carrier filters are used to block abusive traffic, as well as to enforce rules or regulations about what types of messaging are allowed to that country or mobile network.

Learn more about how message filtering works and how to avoid it: [How Does Message Filtering Work?](https://support.twilio.com/hc/en-us/articles/223181848-How-Does-Carrier-Filtering-Work-)

For a detailed explanation of carrier filtering in the United States and Canada, please see [SMS Carrier Filtering in the United States and Canada](https://support.twilio.com/hc/en-us/articles/360022449893-SMS-Carrier-Filtering-in-the-United-States-and-Canada).

#### Possible Causes
* Your message was identified as spam or unwanted messaging by Twilio's message filtering system
* Your message was flagged as objectionable and blocked by a wireless carrier

#### Possible Solutions
* Ensure your messaging use case complies with Twilio's [Messaging Policy](https://www.twilio.com/en-us/legal/messaging-policy) and [Acceptable Use Policy](https://www.twilio.com/en-us/legal/aup)
* Review the information in [How Does Message Filtering Work?](https://support.twilio.com/hc/en-us/articles/223181848-How-Does-Carrier-Filtering-Work-) to understand what causes filtering.
* See [How do I prevent my Twilio messages from being filtered (blocked)?](https://support.twilio.com/hc/en-us/articles/1260803966670-How-do-I-prevent-my-Twilio-messages-from-being-filtered-blocked-) for specific tips on avoiding message filtering.
* If you believe your messages are compliant with Twilio and carrier policies, please collect 3 or more examples of Message SIDs that have the “undelivered” status with error 30007, and then [contact our Support team](https://www.twilio.com/console/support/tickets/create). We can help review your messaging and determine if an error was made, and put you in touch with our Compliance team if needed."#,
            TwilioProgrammableSmsError::ErrorCode63006 => r#"### Could not format given content for the Channel

#### Possible Causes 
*   Incorrect format."#,
            TwilioProgrammableSmsError::ErrorCode63029 => r#"## ERROR - 63029

### The receiver failed to download the template

 The receiver failed to download the template


#### Possible Causes
The receiver failed to download the template

#### Possible Solutions
Please check your destination number/handset and retry the number"#,
            TwilioProgrammableSmsError::ErrorCode30040 => r#"## ERROR - 30040

### Destination carrier requires Sender ID pre-registration

 You sent a message (message SID: SMXXXXX) to a mobile number in a country that requires Alphanumeric Sender ID pre-registration, and the Sender ID you used is not registered to your account.

#### Possible Causes
* The recipient you are sending a message to is on a network that requires Alphanumeric Sender ID pre-registration, and our records indicate that your Sender ID is not registered.
* You may have previously received a WARNING 30018 “Destination carrier requires Sender ID pre-registration” code. This is the corresponding ERROR version of that code when a block is indeed happening.

#### Possible Solutions
* Learn more about Alpha Sender ID registration requirements [here](https://support.twilio.com/hc/en-us/articles/223133767-International-support-for-Alphanumeric-Sender-ID).
* Use [this form](https://twiliodoer.secure.force.com/SenderId) to register a Sender ID for the destination country."#,
            TwilioProgrammableSmsError::ErrorCode57021 => r#"## ERROR - 57021

### Token invalid

 Invalid Token

#### Possible Causes
Invalid Token.

#### Possible Solutions
Pass the correct token."#,
            TwilioProgrammableSmsError::ErrorCode35126 => r#"## ERROR - 35126

### The ScheduleType value provided is not supported for this channel

 The ScheduleType value provided is not supported for the channel you want to send the message on. 

#### Possible Causes
You provided a value other than ‘fixed’ in the ScheduleType parameter

#### Possible Solutions
Change the ScheduleType value to ‘fixed’"#,
            TwilioProgrammableSmsError::ErrorCode30038 => r#"## ERROR - 30038

### OTP Message Body Filtered

 Twilio has determined that the body of this message contains a One-Time Passcode (OTP). In order to prevent the abuse of Twilio numbers, Twilio does not generally allow receiving OTP messages. The passcode content has been redacted and the message failed.

#### Possible Causes
You are receiving inbound OTP messages.

#### Possible Solutions
In order to prevent account creation and abuse with Twilio hosted numbers, Twilio does not generally allow the reception of OTP messages. Contact our [Support team](https://www.twilio.com/console/support/tickets/create) if additional information is required."#,
            TwilioProgrammableSmsError::ErrorCode35111 => r#"## ERROR - 35111

### SendAt timestamp is missing

 You must indicate SendAt timestamp if ScheduleType is 'fixed'.

#### Possible Causes
You did not indicate SendAt timestamp.

#### Possible Solutions
Check if there is a valid timestamp entered for SendAt when ScheduleType is 'fixed'."#,
            TwilioProgrammableSmsError::ErrorCode30103 => r#"## ERROR - 30103

### Links not shortened due to application failure.

Twilio was unable to shorten the links included in your message due to an application error in the Twilio Link Shortening service. Links not shortened due to shortener application failure

#### Possible Causes
Unknown service interruption or outage. 

#### Possible Solutions
File a support ticket and include the Message SID."#,
            TwilioProgrammableSmsError::ErrorCode30105 => r#"## WARNING - 30105

### Shortened link not found and no fallback URL found

We returned a 404 to this user action as we could not find a valid url or fallback url to redirect them to. We received a request to a domain you've registered with Click Tracking but we don't have a record of the shortened url or a fallback url set on the domain.

#### Possible Causes
- The url has expired
- A request was sent to a url that wasn't generated by Twilio
- The fallback url isn't set

#### Possible Solutions
- Contact your account manager or support about extending the retention period of links.
- Set a fallback url to mitigate this error"#,
            TwilioProgrammableSmsError::ErrorCode21710 => r#"## WARNING - 21710

### Phone Number Already Exists in Messaging Service

 The phone number, short code or alpha sender ID already exists in the Messaging Service.

#### Possible Causes
* Multiple requests were made to add the phone number, short code or alpha sender ID to the Messaging Service’s Sender Pool. 
* The phone number, short code or alpha sender ID was already added at an earlier time.

#### Possible Solutions
* If you know this phone number is properly associated with the Messaging Service, do nothing. 
* Remove the existing phone number, short code or alpha sender ID from the Messaging Service and add it back to the Messaging Service’s Sender Pool."#,
            TwilioProgrammableSmsError::ErrorCode30404 => r#"## ERROR - 30404

### Not Found

The resource was not found. ## Not Found

The resource was not found. Here are some examples of cases that may trigger a 404 error.

#### Possible Causes
 Using a base URL that is not `https://api.twilio.com`. For example, making requests to `https://twilio.com` or `https://www.twilio.com` will not work.

#### Possible Solutions
Confirm that the AccountSid and MessagingServiceSid is correct"#,
            TwilioProgrammableSmsError::ErrorCode57005 => r#"## ERROR - 57005

### 'Category' is too long

 Specified 'Category' is too long

#### Possible Causes
Specified 'Category' is too long

#### Possible Solutions
Confirm a valid 'Category' is being passed in request."#,
            TwilioProgrammableSmsError::ErrorCode30129 => r#"## ERROR - 30129

### Certificate is self signed

Please use a certificate with valid signature Certificate is self signed

#### Possible Causes
Using self signed certificates

#### Possible Solutions
Please use a certificate with valid signature"#,
            TwilioProgrammableSmsError::ErrorCode21730 => r#"## ERROR - 21730

### System under maintenance. Please try again later.

 This error is returned by Twilio Systems when the system is going through a scheduled maintenance. During this maintenance window some of the operations are not allowed and this error is returned.

#### Possible Causes
Currently the system is going through some maintenance.

#### Possible Solutions
Try again later because once maintenance activity is completed all requests are processed successfully."#,
            TwilioProgrammableSmsError::ErrorCode30127 => r#"## ERROR - 30127

### MessagingServiceSID is invalid.

 MessagingServiceSID is invalid.

#### Possible Causes
User included an invalid MessagingServiceSID.

#### Possible Solutions
Please follow doc for valid messaging service sid."#,
            TwilioProgrammableSmsError::ErrorCode21902 => r#"## ERROR - 21902

### InvoiceTag length must be between 0 and 32

 InvoiceTag length must be between 0 and 32

#### Possible Causes
InvoiceTag is too long

#### Possible Solutions
Decrease InvoiceTag length"#,
            TwilioProgrammableSmsError::ErrorCode30128 => r#"## ERROR - 30128

### MessagingServiceSidsAction is invalid

MessagingServiceSidsAction is invalid, please check for valid action types MessagingServiceSidAction is invalid

#### Possible Causes
Only ADD, DELETE or REPLACE action types are allowed

#### Possible Solutions
Try to use any of the action types - ADD, DELETE, REPLACE"#,
            TwilioProgrammableSmsError::ErrorCode30037 => r#"## ERROR - 30037

### Outbound Messaging Disabled

 Sending messages from this account has been disabled.

#### Possible Causes
This account is configured to not send outbound messages.

#### Possible Solutions
If you believe this configuration is in error you can [contact support](https://support.twilio.com/hc/en-us/articles/360048500694-Contacting-Twilio-Support) to adjust the account configuration."#,
            TwilioProgrammableSmsError::ErrorCode30032 => r#"## ERROR - 30032

### Toll-Free Number Has Not Been Verified

Toll-Free Verification Required.  You sent a message (message SID: SMXXXXX) from a Toll-Free number to a mobile subscriber in USA or Canada. Twilio requires all Toll-Free numbers to go through a verification process. The Toll-Free number you sent from has not been verified through the required verification process.

#### Possible Causes
*Your toll-free number has not been Verified to allow sending of traffic in USA and Canada.  

*Your toll-free verification submission was rejected. 


#### Possible Solutions
Please refer to this [Support Article](https://support.twilio.com/hc/en-us/articles/5377174717595-Toll-Free-Message-Verification-for-US-Canada) for the process to get your Toll Free number verified. 

Effective Nov 8th, 2023, messages on restricted toll-free phone numbers are blocked.
Effective Jan 31st, 2024, messages on pending toll-free phone numbers are blocked.

Toll-Free best practices can be found [here](https://support.twilio.com/hc/en-us/articles/360038172934-Information-and-best-practices-for-using-Toll-Free-SMS-and-MMS-in-the-US-and-Canada)"#,
            TwilioProgrammableSmsError::ErrorCode21910 => r#"## ERROR - 21910

### Invalid 'From' and 'To' pair. 'From' and 'To' should be of the same channel

 `From` and `To` parameters in the API Request should be of the same channel.

#### Possible Causes
You are trying to send a message between different channels, eg. from an SMS capable number to a WhatsApp number.

#### Possible Solutions
Ensure `From` and `To` parameters are the same channel, eg. both are SMS or both are WhatsApp."#,
            TwilioProgrammableSmsError::ErrorCode92009 => r#"## ERROR - 92009

### The template associated with this SID has already been submitted for approval.

The template associated with this SID has already been submitted for approval. The template associated with this SID has already been submitted for approval.

#### Possible Causes
This template sid has already been submitted for approval.

#### Possible Solutions
Please recreate a new template to make any changes."#,
            TwilioProgrammableSmsError::ErrorCode57019 => r#"## ERROR - 57019

### 'Authorization' header is missing or is invalid

'Authorization' header is missing or is invalid 'Authorization' header is missing or is invalid

#### Possible Causes
'Authorization' header is missing or is invalid

#### Possible Solutions
Confirm a valid 'Authorisation' header is being passed in the request"#,
            TwilioProgrammableSmsError::ErrorCode30125 => r#"## WARNING - 30125

### Your phone number could not be registered with US A2P 10DLC

 We noticed that you have tried to add a number to a messaging service associated with a US A2P 10DLC Sole Proprietor campaign. Please note that all Sole Proprietor campaigns have a max limit of 1 phone number per campaign. Since you have reached this limit previously, your phone number could not be successfully registered for US A2P 10DLC. If you try to send SMS or MMS messages to the US using this phone number, those messages will be considered as unregistered traffic, where higher fees apply.


#### Possible Causes
You already have 1 phone number registered with your Sole Proprietor campaign.

#### Possible Solutions
You may continue to send messages to other non-US destinations using this phone number. However, if you want to use this phone number to send to recipients in the US, please consider removing all senders from this campaign from the Messaging Services page and then add back the number you wish to use, or register for a standard A2P brand and campaign instead."#,
            TwilioProgrammableSmsError::ErrorCode63025 => r#"### Media already exists

#### Possible Causes 
* The media content already exists. Conflict in uploading the media."#,
            TwilioProgrammableSmsError::ErrorCode30021 => r#"## ERROR - 30021

### Internal Failure with messaging service orchestrator

 An internal error has occurred with the messaging service orchestrator that prevented Twilio from processing your response.

#### Possible Causes
* We screwed up. Sorry!

#### Possible Solutions
* If the error persists, please contact us to figure out what has happened and how to fix it."#,
            TwilioProgrammableSmsError::ErrorCode30485 => r#"## ERROR - 30485

### Message couldn't be delivered

Message couldn't be delivered Message couldn’t be delivered to the destination number you are trying to reach temporarily.

#### Possible Causes
* This is because Twilio has identified potential fraudulent messages being sent to the destination you are trying to reach.
* Hence SMS traffic can not be delivered to the destination phone number used for the next couple of hours. The traffic should return to normalcy post that. However, the SMS traffic could be undelivered for an even longer period if fraudulent activity doesn't subside & continues to go on.

#### Possible Solutions
You do not need to take any specific action. The disruption in traffic is usually temporary and will resolve in a couple of hours if we do not detect additional fraudulent activity.

#### Continued issues with error 30485
Twilio's support team can help investigate what went wrong with delivering your message. Please collect 3 or more message SIDs in your [SMS logs](https://www.twilio.com/console/sms/logs) from the last 24 hours that were flagged with Error 30485, and [open a support request.](https://www.twilio.com/console/support/tickets/create)"#,
            TwilioProgrammableSmsError::ErrorCode30123 => r#"## ERROR - 30123

### Callback URL is missing

Callback URL is missing, please add an URL. Callback URL is missing

#### Possible Causes
User did not include callbackUrl field

#### Possible Solutions
Please follow doc for setting up callback URL."#,
            TwilioProgrammableSmsError::ErrorCode63022 => r#"### Invalid vname certificate

#### Possible Causes 
* The vname cert for your business account was invalid. Please reach out to support to verify the cert."#,
            TwilioProgrammableSmsError::ErrorCode30024 => r#"## ERROR - 30024

### Numeric Sender ID Not Provisioned on Carrier

Provisioning Issue with Carrier You sent a message (message SID: SMXXXXX) to a mobile number in a country that requires Numeric Sender ID pre-registration and provisioning. The Numeric Sender ID is not currently provisioned with the carrier.

#### Possible Causes
* The Numeric Sender ID has not been provisioned by the carrier yet. Typically in this situation you would see >90% failures towards the carrier.
* If this is for US A2P 10DLC and you just registered this number, it could take a brief period of time for all carriers to provision the number to your campaign. Typically you would see all messages towards a single carrier fail, if the code isn't provisioned.
* Your shortcode has not been fully provisioned for production traffic, or the carrier has disabled your shortcode due to cancellation or compliance violations. Typically in this situation you would see >90% failures towards the carrier.
* The destination number has recently ported to a new carrier and the port hasn't completed 100%.
* Your toll-free number has not been fully provisioned for production traffic. 
*The destination number is not provisioned to receive SMS messages.



#### Possible Solutions
* If you believe your Numeric Sender ID should be registered, please collect 3 or more examples of Message SIDs that have the “undelivered” status with error 30024, and then contact our Support team. We can help review the registration status to resolve these errors. "#,
            TwilioProgrammableSmsError::ErrorCode30029 => r#"## ERROR - 30029

### Invalid ContentRetention

 ContentRetention value used in the request is invalid

#### Possible Causes
ContentRetention is not either 'retain', 'discard' or 'debug' or Message Privacy features are not enabled for this account

#### Possible Solutions
Enable the correct Message Privacy features and use a valid value for ContentRetention"#,
            TwilioProgrammableSmsError::ErrorCode63009 => r#"### Channel returned an error when executing the request

#### Possible Causes 
*  Please see Channel specific error message for more information."#,
            TwilioProgrammableSmsError::ErrorCode30114 => r#"## ERROR - 30114

### Specified date is not available yet

 Deactivation list data is not yet available for the date that you specified.

#### Possible Causes
- You have specified a date in the future
- You have specified today and Twilio has yet to process data for today yet

#### Possible Solutions
Please either specify an earlier date or wait for Twilio to process today's data."#,
            TwilioProgrammableSmsError::ErrorCode30003 => r#"## ERROR - 30003

### Unreachable destination handset

The destination handset you are trying to reach is switched off or otherwise unavailable. ### Unreachable destination handset

The destination handset you are trying to reach is switched off or otherwise unavailable. 

#### Possible Causes
*  The destination handset you are trying to reach is switched off or otherwise unavailable.
*  The device you are trying to reach does not have sufficient signal
*  The device cannot receive SMS (for example, the phone number belongs to a landline)
*  There is an issue with the mobile carrier

#### Possible Solutions
In some cases, a delivery error may occur once due to a transient issue downstream of Twilio. To test whether the issue occurs again, please attempt to send another message to the user via a [REST API](https://help.twilio.com/articles/223133907-Simple-example-for-sending-SMS-or-MMS-messages) request.

If you see similar results, continue troubleshooting with the following checklist:

*  Is the destination device powered on?
*  Does the device have sufficient signal? If not power the device off, wait 30 seconds, and then power it back up.
*  Is the device connected to the home carrier's network? We cannot guarantee message delivery on devices roaming off-network.
*  Can the device receive non-Twilio SMS?
*  Can the device receive messages from another Twilio number (non-Alphanumeric Sender ID), or with a shorter one-segment (non-concatenated) body?
*  Can other devices using the same mobile carrier receive your messages?

If you can rule out all of the above issues, continue troubleshooting below.

#### Messages sent from short codes
Repeated Error 30003 results on undelivered messages sent from [short codes](https://support.twilio.com/hc/en-us/articles/223182068-What-is-a-Messaging-Short-Code) could occur if the destination user has a wireless plan that does not support short code or "Premium" messages.

**Sprint** customers may be able to allow messages sent from your short code by texting the phrase `Allow XXXXX` to **9999**, replacing `XXXXX` with your short code. However, customers from other wireless carriers will likely need to reach out to their carrier's support group to enable "Premium" or short code messaging.

#### Messages sent from long codes
Repeated Error 30003 results on undelivered messages sent from long codes (regular phone numbers) and Toll-Free numbers could be due to carrier filtering. 

The rules for carrier filtering vary throughout the industry, making it difficult at times to pin down exactly why a message is being filtered out. For more details including potential workarounds, please see this Help Center article ["How Does Carrier Filtering Work?"](https://support.twilio.com/hc/en-us/articles/223181848-How-Does-Carrier-Filtering-Work)

#### Continued Error 30003 issues
Twilio's Support team can help investigate what went wrong with delivering your message. Please collect 3 or more message SIDs in your [SMS logs](https://www.twilio.com/console/sms/logs) from the last 24 hours that were flagged with Error 30003, and [open a support request](https://www.twilio.com/console/support/tickets/create)."#,
            TwilioProgrammableSmsError::ErrorCode63003 => r#"## ERROR - 63003

### Channel could not find To address

 You have tried to send a message to a To address that is inactive or invalid.

#### Possible Causes
* The To address is no invalid or longer belongs to a mobile user
* The To address was entered or formatted incorrectly

#### Possible Solutions
* Ensure you have provided the correct To address for the user you are trying to reach
* Ensure that the To address is still active"#,
            TwilioProgrammableSmsError::ErrorCode30131 => r#"## WARNING - 30131

### Domain's certificate will expire soon

A TLS certificate associated with one of your domains will expire soon. A TLS certificate associated with one of your domains will expire soon.

#### Possible Causes
A TLS certificate associated with one of your domains has not been renewed in our system, and the existing certificate will expire soon.

#### Possible Solutions
Please generate and upload a new certificate."#,
            TwilioProgrammableSmsError::ErrorCode35117 => r#"## ERROR - 35117

### Scheduling does not support this timestamp

 SendAt time must be between 900 seconds and 35 days (3024000 seconds) in the future, inclusive.

#### Possible Causes
Scheduler currently doesn't support scheduling a message at a fixed time less that 15 minutes from now, or more than 35 days in the future.

#### Possible Solutions
Verify that the SendAt parameter is using "[YYYY]-[MM]-[DD]T[HH]:[MM]:[SS]Z" format (in UTC)."#,
            TwilioProgrammableSmsError::ErrorCode21655 => r#"## ERROR - 21655

### The ContentSid is Invalid

The ContentSid you are using is not valid 

#### Possible Causes
The ContentSid you are using is not valid

#### Possible Solutions
Double check the ContentSid parameter that you are using. Make sure it has the proper formatting and starts with an HX prefix."#,
            TwilioProgrammableSmsError::ErrorCode30034 => r#"## ERROR - 30034

### US A2P 10DLC - Message from an Unregistered Number

 Messages sent to US numbers will not be delivered if they are sent from numbers that are not associated with an approved A2P 10DLC Campaign. This [guide](https://support.twilio.com/hc/en-us/articles/4418081745179-How-do-I-check-that-I-have-completed-US-A2P-10DLC-registration-) will help you determine if you have completed registration for A2P 10DLC. To initiate or continue an A2P 10DLC registration, [visit your console here](https://console.twilio.com/us1/develop/sms/regulatory-compliance/a2p-onboarding). Find out how to register using [this guide](https://support.twilio.com/hc/en-us/articles/1260801864489-How-do-I-register-to-use-A2P-10DLC-messaging-).

For a step-by-step walkthrough, check out [this video on resolving Error 30034](https://twil.io/resolve30034).

#### Possible Causes
You are sending messages to the US using a US 10DLC number that is not associated with an approved A2P 10DLC Campaign.

#### Possible Solutions
Associate your US 10DLC number with a registered A2P Campaign by adding it to the corresponding Messaging Service via the Twilio Console or API. Find out how to register using [this guide](https://support.twilio.com/hc/en-us/articles/1260801864489-How-do-I-register-to-use-A2P-10DLC-messaging-). 

Alternatively, you can also use a different number that is already associated with an approved A2P Campaign to send messages in the US.
"#,
            TwilioProgrammableSmsError::ErrorCode63007 => r#"## ERROR - 63007

### Twilio could not find a Channel with the specified 'From' address

 
   

#### Possible Causes
* Sending a message using a `From` address that is not associated with any Channel or WhatsApp installation.
* Sending a message using a `From` address for a Channel that is pending approval.
* Using incorrect Account Credentials for the account that owns the Sender


#### Possible Solutions
* See [here](https://www.twilio.com/docs/api/channels#channel-addresses) for supported formats of Channel endpoint addresses.
* To send messages using WhatsApp, the `From `address should be `whatsapp:<sandbox phone number>` or` whatsapp:<your twilio number>` . This can be found on the [WhatsApp sandbox page](https://www.twilio.com/console/messaging/whatsapp/sandbox) or [WhatsApp numbers page](https://www.twilio.com/console/messaging/whatsapp/numbers).
* If you are using the WhatsApp sandbox, ensure that you have activated the Twilio Sandbox for WhatsApp. This can be found on the [WhatsApp sandbox page](https://www.twilio.com/console/messaging/whatsapp/sandbox).
* Check your account SID and auth token are correct and correspond to the account that owns the Sender. You can find your account credentials in the Console [here](https://console.twilio.com/).
"#,
            TwilioProgrammableSmsError::ErrorCode30104 => r#"## WARNING - 30104

### Shortened link not found. Click redirected to fallback Url

Twilio was unable to find a record of the shortened link that this request was sent to. We received a request to a domain you've registered with Click Tracking but we don't have a record of the shortened url.

#### Possible Causes
- The url has expired
- A request was sent to a url that wasn't generated by Twilio

#### Possible Solutions
- Contact your account manager or support about extending the retention period of links."#,
            TwilioProgrammableSmsError::ErrorCode30020 => r#"## ERROR - 30020

### Internal Failure with Message Scheduling

 An internal error has occurred with Message Scheduling that prevented Twilio from processing your request.

#### Possible Causes
* We screwed up. Sorry!

#### Possible Solutions
* If the error persists, please contact us to figure out what has happened and how to fix it.
* Note the time of the error and what you were trying to do when it occurred."#,
            TwilioProgrammableSmsError::ErrorCode30450 => r#"## ERROR - 30450

### Message delivery blocked

 The destination number you are trying to reach is temporarily blocked from receiving this message.

#### Possible Causes
This is because the SMS Pumping Protection feature has identified potential fraudulent messages being sent to the destination you are trying to reach.

A temporary block on SMS traffic has been placed for the next 15-30 mins on the phone number you used. This block will be lifted at the end of this 15-30 minutes period. If further fraudulent activity is detected after this block, this pattern will continue with temporary 15-30 minutes blocks until the issue is no longer observed.

#### Possible Solutions
You do not need to take any specific action. The disruption in traffic is usually temporary and will resolve in a couple of hours if we do not detect additional fraudulent activity.

#### Continued issues with error 30450
Twilio's support team can help investigate what went wrong with delivering your message. Please collect 3 or more message SIDs in your [SMS logs](https://www.twilio.com/console/sms/logs) from the last 24 hours that were flagged with Error 30453, and [open a support request.](https://www.twilio.com/console/support/tickets/create)

Alternatively, you can use the optional <code style="color : name_color">RiskCheck</code> parameter when [creating a Message](https://www.twilio.com/docs/messaging/api/message-resource#create-a-message-resource) with the [Programmable Messaging API](https://www.twilio.com/docs/messaging/api). To prevent a known/legitimate message from getting blocked in future by SMS Pumping Protection, include the <code style="color : name_color">RiskCheck</code> parameter with value <code style="color : name_color">disable</code> when creating the new Message resource.

You can also mark known/legitimate phone numbers using the Global Safe List feature so that they don’t face any disruption . This provides an additional safety net against false positives, so that the traffic to this number never faces the above scenario. Add known phone numbers to the Safe List by using the [Global Safe List API.](https://www.twilio.com/docs/usage/global-safe-list)"#,
            TwilioProgrammableSmsError::ErrorCode30010 => r#"## ERROR - 30010

### Message price exceeds max price

 The error code is obsolete. Find more information in the [changelog](https://www.twilio.com/en-us/changelog/end-of-life--eol--of-twilio-messaging-maxprice-api-parameter).


You provided the [MaxPrice parameter](https://support.twilio.com/hc/en-us/articles/360014170533-Using-MaxPrice-with-Twilio-SMS) in your API request. MaxPrice will prevent a message from sending if it exceeds the price you specified.

Please note that SMS pricing is per-segment. SMS messages longer than 160 characters (or longer than 70 characters, if emojis or special characters are used) will require multiple segments. For a detailed explanation of message segmentation and encoding, see [What the Heck is a Segment?](https://www.twilio.com/blog/what-the-heck-is-a-segment.html).

#### Possible Causes
* The message price exceeded the MaxPrice you specified in your API request.

#### Possible Solutions
* Increase the MaxPrice parameter value in your API requests.
* Remove the MaxPrice parameter from your API requests.
* Check your message body for any Unicode-only characters such as emojis, glyphs, or curly apostrophes or quotation marks which could cause the message to require additional segments.
* Consider using the [Smart Encoding](https://www.twilio.com/docs/messaging/services#smart-encoding) feature of [Messaging Services](https://www.twilio.com/docs/messaging/services) to automatically replace [certain Unicode-only characters](https://www.twilio.com/docs/messaging/services/smart-encoding-char-list) with standard GSM equivalents."#,
            TwilioProgrammableSmsError::ErrorCode21610 => r#"## ERROR - 21610

### Attempt to send to unsubscribed recipient

 The person you are trying to message has opted out of receiving messages from your Twilio phone number, Channels sender, or Messaging Service.

You have attempted to message a 'To' number that has replied with "STOP" to one of your previous messages. You will not be able to send to the phone number specified in the 'To' parameter until the subscriber identified by the phone number has responded with "START". 

Please see [this FAQ](https://support.twilio.com/hc/en-us/articles/223134027-Twilio-support-for-opt-out-keywords-SMS-STOP-filtering-) for more information about how Twilio handles opt-out and opt-in.

#### Possible Causes
* The end user handset has responded with "STOP" or [another opt-out keyword](https://support.twilio.com/hc/en-us/articles/223134027-Twilio-support-for-opt-out-keywords-SMS-STOP-filtering-).

* The phone number has been reassigned to a different end user recently.

#### Possible Solutions
* Consider removing this phone number from your list of recipients.
* Request the recipient to resubscribe to your messages by texting in "START" or [another opt-in keyword](https://support.twilio.com/hc/en-us/articles/223134027-Twilio-support-for-opt-out-keywords-SMS-STOP-filtering-) to your Twilio sender.
* Before sending messages to a recipient, ensure they have consented to receive messages from you. Please read these guidelines to understand [messaging opt-in requirements and best practices](https://www.twilio.com/blog/ctia-messaging-principles-and-best-practices)."#,
            TwilioProgrammableSmsError::ErrorCode90001 => r#"## ERROR - 90001

### Message SID is invalid

 An invalid message SID has been sent in the request

#### Possible Causes
Message SID sent in the request has either wrong prefix or incorrect format

#### Possible Solutions
Provide correct message SID in the request"#,
            TwilioProgrammableSmsError::ErrorCode21717 => r#"## ERROR - 21717

### Brand Registration SID for US A2P Campaign Use Case is Not Registered or Not Valid

 The Brand Registration SID you are trying to use to register a new US A2P campaign use case is not registered or the Brand Registration SID is not valid for the given use case.

#### Possible Causes
* The Brand Registration SID passed in the request is not registered.
* The Brand Registration SID passed in the request is not valid for the given use case. Only brands with a STARTER brand_type can create STARTER campaign use cases.

#### Possible Solutions
* Verify if the Brand Registration SID has been registered by checking the `status` field using the [Brand API](https://www.twilio.com/docs/sms/a2p-10dlc/onboarding-isv-api#using-get-to-check-brand-registration-status). If the brand has been registered, it will have a status of APPROVED. If this is the case, verify that the Brand Registration SID passed into the request matches the Brand Registration SID that has been registered and retry the request.
* If Brand Registration has an IN_PROGRESS status, registration is in progress. Please wait for it to be in an APPROVED status before moving forward with US A2P campaign use case creation. 
* If Brand Registration has a REGISTRATION_FAILED status, you may need to correct your brand request and re-trigger Brand registration.
* Verify the brand_type of the Brand by checking the `brand_type` field using the [Brand API](https://www.twilio.com/docs/sms/a2p-10dlc/onboarding-isv-api#using-get-to-check-brand-registration-status). If the brand_type is STARTER, the brand can only be used to create STARTER campaign use cases. If it is a STANDARD brand_type, the brand can be used to create any other type of campaign use case."#,
            TwilioProgrammableSmsError::ErrorCode57001 => r#"## ERROR - 57001

### 'Secret id' is empty

 'Secret id' is not specified

#### Possible Causes
'Secret id' is not specified

#### Possible Solutions
Confirm a valid 'Secret id' is being passed in request. "#,
            TwilioProgrammableSmsError::ErrorCode30454 => r#"## ERROR - 30454

### Account exceeded the messages limit

Account exceeded the messages limit Account sent too many messages which exceeded the messages limit.

#### Possible Causes
Account exceeded the trial account messages limit.

#### Possible Solutions
* Consider upgrading your account to remove the restrictions. It takes 3-4 hours to reflect the changes in all systems. Please retry after that."#,
            TwilioProgrammableSmsError::ErrorCode92002 => r#"## ERROR - 92002

### The 'variables' parameter exceeds the allowed limit

Invalid variables. The max amount of variables allowed is 100 

#### Possible Causes
You attempted to create content, but you send more than '100' variables

#### Possible Solutions
Please review the amount of variables defined in the 'variables' property"#,
            TwilioProgrammableSmsError::ErrorCode90031 => r#"## ERROR - 90031

### Broadcast Recipients list is empty [deprecated] 

 Broadcast 'Recipients' list is empty

#### Possible Causes
Broadcast 'Recipients' list is empty

#### Possible Solutions
Specify at least one recipient for Broadcast request"#,
            TwilioProgrammableSmsError::ErrorCode63019 => r#"## ERROR - 63019

### Media failed to download

 

#### Possible Causes
* Twilio failed to download the media from the sender. 
* Attempt to retrieve the media returned 0 bytes. 

#### Possible Solutions
Please check the media content before retrying"#,
            TwilioProgrammableSmsError::ErrorCode30026 => r#"## WARNING - 30026

### US A2P 10DLC - 70% T-Mobile Daily Message Limit Consumed

As part of the US A2P 10DLC initiative, T-Mobile has instituted a daily message limit. We’re sending you this warning to let you know that you have consumed 70% of your T-Mobile daily limit. Please note that this daily limit is based on the sum of SMS segments and MMS messages you send to T-Mobile subscribers on a 24-hour basis, and is reset daily at midnight PT. When you hit 100% of your daily message limit, you will start to receive error code 30023 for each subsequent message you send to T-Mobile. Those messages will not be delivered, and you must wait until midnight PT to resume message sending.

For more information on T-Mobile daily limit, please refer to [T-Mobile daily message limits for long code messaging with A2P 10DLC](https://support.twilio.com/hc/en-us/articles/1260804800549-T-Mobile-daily-message-limits-for-long-code-messaging-with-A2P-10DLC). For more information on US A2P 10DLC, please refer to [US A2P 10DLC Documentation](https://support.twilio.com/hc/en-us/articles/1260800720410-What-is-A2P-10DLC-).

Please disregard this warning if you have been approved for the unlimited tier via the [T-Mobile Special Business Review process](https://support.twilio.com/hc/en-us/articles/4403550579739-T-Mobile-Special-Business-Review-for-A2P-10DLC).

Please note that if you have multiple providers sending registered SMS or MMS to T-Mobile on your behalf, this warning might not be accurate.

#### Possible Causes
* You have consumed 70% of your T-Mobile daily message limit for the day.

#### Possible Solutions
* No action is required from you. 
* To view the warning and errors in the debugger, please visit [here](https://console.twilio.com/us1/monitor/logs/debugger/events?frameUrl=%2Fconsole%2Fdebugger%3F__override_layout__%3Dembed%26quickDate%3D24%26x-target-region%3Dus1%26bifrost%3Dtrue&currentFrameUrl=%2Fconsole%2Fdebugger%3F__override_layout__%3Dembed%26bifrost%3Dtrue%26quickDate%3D24%26x-target-region%3Dus1).
* To view your daily T-Mobile message limit, please visit [Trust Hub - A2P Messaging](https://console.twilio.com/us1/account/trust-hub/a2p?frameUrl=%2Fconsole%2Ftrust-hub%2Fa2p-messaging%3Fx-target-region%3Dus1), select your US A2P brand, and view the  details of your TCR trust score (registered customers only).
* To view your daily T-Mobile usage, please use the Messaging Insights tool."#,
            TwilioProgrammableSmsError::ErrorCode30115 => r#"## ERROR - 30115

### Date format is incorrect

 The date format you have used is incorrect.

#### Possible Causes
- You may have included a timestamp in your date
- You may have abbreviated YYYY to YY

#### Possible Solutions
Please use YYYY-MM-DD format for the Date field."#,
            TwilioProgrammableSmsError::ErrorCode35118 => r#"## ERROR - 35118

### MessagingServiceSid is required to schedule a message

 Message Scheduling is only accessible on Messaging Services. You need to pass a MessageServiceSid instead or along with a From Phone Number in order to schedule a message.

#### Possible Causes
A MessagingServiceSid was not provided as part of the MessagingServiceSid or From parameter.

#### Possible Solutions
Pass the MessagingServiceSid in either the MessagingServiceSid parameter or the From parameter."#,
            TwilioProgrammableSmsError::ErrorCode30039 => r#"## WARNING - 30039

### Filtered to Prevent Message Loops

 This inbound message appears to be machine generated. Responding to this message programmatically could lead to endless messaging loops. For that reason, Twilio has failed the message with this error code and did not POST to the number’s configured TwiML URLs.

#### Possible Causes
You are receiving a message that appears to be machine generated.

#### Possible Solutions
If you want to respond to these messages programmatically, you can listen for failed status callbacks with this error code and respond using the Twilio REST API."#,
            TwilioProgrammableSmsError::ErrorCode21652 => r#"## Error - 21652

### Maximum subject text length is 40 characters

The subject text of your message should be 40 characters or less.
"#,
            TwilioProgrammableSmsError::ErrorCode30008 => r#"## ERROR - 30008

### Unknown error

 ## Error - 30008

### Message Delivery - Unknown error


#### Possible Causes
If a message you sent is not delivered to the end device and returns a 30008 error code, this means that delivery of your message failed for unknown reasons.

When Twilio receives a very generic error from our carrier partner that we have no further details about, we associate the message with the error code 30008, letting you know that Twilio truly doesn’t know what caused this error from the provider.

#### Possible Solutions
* Check that the phone you were sending to is turned on and can receive non-Twilio SMS
* Ensure that the phone is not roaming off network. We cannot guarantee message delivery on roaming phones.
* Try sending to other phones who have the same mobile carrier (you can use our Lookups API to determine the carrier if you’re unsure). If messages to other phones go through, the issue is likely device related. Try rebooting the device or contact the mobile carrier for help.
* If you are sending SMS from an alphanumeric sender ID, see if using a Twilio phone number works better. We’ve observed that certain networks may block alpha sender IDs.
* Try sending a shorter message to the phone, with simple content that does not include any special characters. This would give our support team an idea as to whether the failure is related to concatenation or character encoding.
* Twilio support can help investigate what went wrong with our carriers. Please open a support request and include a minimum of 3 or more message SIDs where a 30008 error was thrown. Per our carriers' requirements, these SIDs can be no older than 48 hours at most."#,
            TwilioProgrammableSmsError::ErrorCode30042 => r#"## ERROR - 30042

### The Sender ID is blocked as generic or it contains special characters

Compliance issue in destination The Sender ID has been blocked for being generic by Twilio’s service provider, containing non compliant special characters or relating to fraudulent activities.

#### Possible Causes
You are using a Sender ID that is not compliant with SMS guidelines

#### Possible Solutions
Please review the SMS guidelines and use a Sender ID that is compliant with SMS guidelines"#,
            TwilioProgrammableSmsError::ErrorCode63021 => r#"## ERROR - 63021

### Channel invalid content error

 Message contains content that is rejected or unsupported by the messaging channel.

#### Possible Causes
* Unable to upload the media used in the message because it is not supported by the messaging channel.
* Message contains components not supported in selected messaging channel
* Length of the parameters and the template text exceeds the allowed length of 1024 for WhatsApp.

#### Possible Solutions
* Inspect the media files being sent and confirm their MIME type is in fact supported by the messaging channel. 
* Confirm message components follow character limitations, proper formatting, and channel specific rules mentioned in docs if applicable. 
* Confirm WhatsApp template sends do not exceed 1024 characters, which can occur when placeholder text substitutions or translations are too long."#,
            TwilioProgrammableSmsError::ErrorCode21624 => r#"## ERROR - 21624

### Invalid validity period value

 ## Error - 21624

### Invalid validity period value

Validity period can only be set to integers between 1 and 36,000 seconds (10 hours max).

#### Possible Causes
A custom ValidityPeriod has been set that is higher than 36000 or lower than 1

#### Possible Solutions
Set ValidityPeriod to an integer between 1 and 36,000 seconds (10 hours)"#,
            TwilioProgrammableSmsError::ErrorCode63034 => r#"## ERROR - 63034

### Media exceeds size limit

 The specified message media exceeds the maximum size limit. You may re-attempt with media of size 5 MB or less.

#### Possible Causes
The specified message media exceeds the maximum size limit.

#### Possible Solutions
Re-attempt with media of size 5 MB or less."#,
            TwilioProgrammableSmsError::ErrorCode30126 => r#"## ERROR - 30126

### Your 10DLC number failed to be registered 

 Your phone number failed registration/deregistration due to Unknown reasons

#### Possible Causes
Unknown

#### Possible Solutions
* Contact [Twilio Customer Support](https://www.twilio.com/console/support/tickets/create)
"#,
            TwilioProgrammableSmsError::ErrorCode57014 => r#"## ERROR - 57014

### 'Event' data in payload is absent

 'Event' is not specified

#### Possible Causes
'Event' is not specified

#### Possible Solutions
Confirm a valid 'Event' is being passed in request. "#,
            TwilioProgrammableSmsError::ErrorCode21657 => r#"## ERROR - 21657

### The Sender ID is invalid 

 The Sender ID is Invalid

#### Possible Causes
You are using a Sender ID that is not supported in the specific destination

#### Possible Solutions
Please review the SMS guidelines and use a Sender ID that is supported in the specific destination "#,
            TwilioProgrammableSmsError::ErrorCode63018 => r#"## ERROR - 63018

### Rate limit exceeded for Channel

 

#### Possible Causes
* Your account is sending messages at a higher combined rate than configured for your account.
* Your WhatsApp Sender has exceeded the messaging limit imposed by WhatsApp. For more information, see [WhatsApp Sender Messaging Limits & Quality Rating](https://support.twilio.com/hc/en-us/articles/360024008153-WhatsApp-Sender-Message-Limits-and-Quality-Rating)

#### Possible Solutions
* Wait until 24 hours have passed since starting a business-initiated conversation with one or more users"#,
            TwilioProgrammableSmsError::ErrorCode92001 => r#"## ERROR - 92001

### 'Types' Parameter Required

You attempted to create content, but you forgot to include at least one type within the 'types' property the 'types' parameter requires at least one type in order to create content.  ## Error - 92001 

### 'types' Parameter Required

You attempted to create content, but you forgot to include at least one type within the 'types' property the 'types' parameter requires at least one type in order to create content. 


#### Possible Causes
You attempted to create content, but you forgot to include at least one type within the 'types' property, the 'types' parameter requires at least one type in order to create content. 

#### Possible Solutions
Please include at least one type within the 'types' property"#,
            TwilioProgrammableSmsError::ErrorCode30001 => r#"## ERROR - 30001

### Queue overflow

 Twilio queues messages based on the sending rate of a sender or an account. For example a US long code number can send one message [segment](https://www.twilio.com/blog/what-the-heck-is-a-segment.html) per second, and a short code may send over 100 message segments per second. 

Messages can only be queued for 10 hours and then they automatically fail. This queue length limit can be reduced by setting a lower [Validity Period](https://www.twilio.com/blog/take-more-control-of-outbound-messages-using-validity-period-html) within your Messaging Service settings or in your API requests to send messages.

You can monitor the length on your queue via the [Queue Insights dashboard](https://www.twilio.com/console/sms/insights/queue).

For more information please view this [FAQ](https://support.twilio.com/hc/en-us/articles/115002943027-Understanding-Twilio-Rate-Limits-and-Message-Queues). 


#### Possible Causes
* You tried to send too many messages too quickly and your message queue overflowed.
* You tried sending all your messages from a single Sender with an insufficient send rate limit (Message Segments per Second) for that volume, and you need additional throughput to handle the volume.
* There are Messages Per Second (MPS) limits applied for your Senders, Messaging Services, and Accounts that you may be encountering.

#### Possible Solutions
* **Recommended** Try using a [Messaging Service](https://www.twilio.com/docs/messaging/services) with multiple senders and it will load balance for you.
* Try adding more senders into your [Messaging Service](https://www.twilio.com/docs/messaging/services).
* If you are messaging in the US or Canada, talk to Twilio Sales about getting a [Toll Free](https://www.twilio.com/en-us/messaging/channels/sms) or [Short Code](https://support.twilio.com/hc/en-us/articles/223182068-What-is-a-Messaging-Short-Code-) number that allows you to send more messages per second.
* Try sending your message again after waiting for some time.
* If you have set a lower Validity Period than the default and maximum value of 36000 seconds (10 hours), try setting a higher Validity Period for your messages."#,
            TwilioProgrammableSmsError::ErrorCode30120 => r#"## ERROR - 30120

###  Domain certificate and private key are not uploaded

Given Domain is not onboarded, please upload cert for domain first Given Domain is not onboarded

#### Possible Causes
User has not uploaded cert for domain

#### Possible Solutions
Please follow doc to upload cert for domain"#,
            TwilioProgrammableSmsError::ErrorCode30113 => r#"## ERROR - 30113

### Specified date is too old

 Historical data for Deactivation lists is only available from August 27th, 2020 onwards.

#### Possible Causes
You have specified a date that is too far in the past for Twilio to have data available.

#### Possible Solutions
Please specify a newer date to retrieve data."#,
            TwilioProgrammableSmsError::ErrorCode30023 => r#"## ERROR - 30023

### US A2P 10DLC - Daily Message Cap Reached

 You have sent the maximum allowable daily messages for your Brand to the carrier. 

Daily limits are based on your Brand Trust Score. You can check both your Trust Score and your daily Limit in the console [here](https://console.twilio.com/us1/develop/sms/regulatory-compliance/brands).

Note: That the daily limit applies to the Brand (per EIN) and is based on the total number of outbound SMS segments and MMS messages sent to T-Mobile (including Sprint and MetroPCS) across any messaging platform you have registered your Brand with.

Please refer to our documentation on [T-Mobile daily message limits for long code messaging with A2P 10DLC](https://support.twilio.com/hc/en-us/articles/1260804800549-T-Mobile-daily-message-limits-for-long-code-messaging-with-A2P-10DLC). For more info about A2P 10DLC, refer to our [US A2P 10DLC Documentation](https://support.twilio.com/hc/en-us/articles/1260800720410-What-is-A2P-10DLC-).


#### Possible Causes
* You have sent the maximum amount of message [segments](https://www.twilio.com/docs/glossary/what-is-gsm-7-character-encoding#:~:text=Additionally%2C%20for%20long,characters%20per%20segment) for the day on a given Brand.
* You have sent the maximum amount of message [segments](https://www.twilio.com/docs/glossary/what-is-gsm-7-character-encoding#:~:text=Additionally%2C%20for%20long,characters%20per%20segment) for the day across multiple Brands associated with your EIN, which could include Brands associated with other Twilio accounts or providers. 


#### Possible Solutions
* You must wait till the next calendar day to resume message sending. The day resets at 00:00 Pacific Time (US). Pacific time is affected by Daylight Savings Time and Standard Time switches.
* To avoid exceeding your daily limit, monitor your daily T-Mobile usage using the Messaging Insights tool.
* You can also use the [30025](https://www.twilio.com/docs/api/errors/30025) and [30026](https://www.twilio.com/docs/api/errors/30026) ‘Daily Traffic Used’ and [30027](https://www.twilio.com/docs/api/errors/30027) ‘Daily Traffic Exceeded’ error warnings to understand what percentage of your daily traffic you have used. These warnings are available in the [console Error Logs](https://console.twilio.com/us1/monitor/logs/debugger/events?frameUrl=%2Fconsole%2Fdebugger%3F__override_layout__%3Dembed%26quickDate%3D24%26x-target-region%3Dus1%26bifrost%3Dtrue&currentFrameUrl=%2Fconsole%2Fdebugger%3F__override_layout__%3Dembed%26bifrost%3Dtrue%26quickDate%3D24%26x-target-region%3Dus1).
* For A2P registered customers, view your daily T-Mobile message limit by visiting [Trust Hub - A2P Messaging](https://console.twilio.com/us1/account/trust-hub/a2p?frameUrl=%2Fconsole%2Ftrust-hub%2Fa2p-messaging%3Fx-target-region%3Dus1), selecting your US A2P brand and viewing the details of your TCR trust score.
"#,
            TwilioProgrammableSmsError::ErrorCode63012 => r#"### Channel returned an internal service error

#### Possible Causes 
*   Channel provider returned an internal service error (HTTP 5xx). "#,
            TwilioProgrammableSmsError::ErrorCode57022 => r#"## ERROR - 57022

### Required header is missing or invalid

 Required header is missing or invalid

#### Possible Causes
Mandatory headers might be missing in the request.

#### Possible Solutions
Provide all the mandatory headers."#,
            TwilioProgrammableSmsError::ErrorCode57008 => r#"## ERROR - 57008

### 'EventType' format must be String

 Specified 'EventType' format must be String

#### Possible Causes
Specified 'EventType' format is invalid, String required

#### Possible Solutions
Confirm a valid 'EventType' is being passed in request."#,
            TwilioProgrammableSmsError::ErrorCode21729 => r#"## ERROR - 21729

### Cannot perform operation on suspended campaign

 The operation you're trying to perform on this campaign is not allowed because the campaign is in a suspended state.

#### Possible Causes
Campaign is suspended.

#### Possible Solutions
Contact Twilio support."#,
            TwilioProgrammableSmsError::ErrorCode21719 => r#"## ERROR - 21719

### Incompatible Messaging Service/A2P Use Cases

 Request was unsuccessful due to incompatibility between the Messaging Service use case and the A2P use case for the messaging service.

#### Possible Causes
* Updating a Messaging Service use case to one that is not compatible with the current A2P use case for this messaging service.
* Creating a new A2P campaign with an A2P use case that is not compatible with the current Messaging Service use case.

#### Possible Solutions
Update the request to pass a compatible A2P/Messaging Service use case. The following resource can be used to retrieve all compatible A2P use cases for a specific Messaging Service: https://messaging.twilio.com/v1/Services/MGXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXX/Compliance/Usa2p/Usecases"#,
            TwilioProgrammableSmsError::ErrorCode21724 => r#"## ERROR - 21724

### Brand update count exceeded

 Maximum number of brand update requests reached on this brand. Contact support for help.

#### Possible Causes
Too many requests were made to update a Brand

#### Possible Solutions
Contact Twilio support"#,
            TwilioProgrammableSmsError::ErrorCode63027 => r#"## ERROR - 63027

### Template does not exist for a language and locale

 ### Template does not exist for a language and locale

#### Possible Causes
* Template does not exist for a language and locale
* Template is being sent using the wrong parameters. Content API templates must be sent with a Content Template sid starting with HX and include a Messaging Service. Templates created using the Legacy System through Templates API or the WA Templates Console UI must be sent with a body string.
* Their requested body does not match an approved template.

#### Possible Solutions
* Double check your body string parameter to make sure it matches exactly with the approved template. New lines or blank characters may cause an issue.
* If the template was created with Content API or Content Editor. Validate that you are sending templates with the right parameters."#,
            TwilioProgrammableSmsError::ErrorCode30132 => r#"## ERROR - 30132

### Certificate cannot be validated. 

We were not able to validate your certificate for your domain. Please upload a new valid certificate.

#### Possible Causes
Invalid certificate format has been uploaded.

#### Possible Solutions
Upload a new valid certificate as outlined in Twilio docs for Link Shortening."#,
            TwilioProgrammableSmsError::ErrorCode30410 => r#"## ERROR - 30410

### Provider Timeout Error

 The provider used may be experiencing disruptions resulting in errors or request timeouts. Messages are failed and not retried to avoid duplicate message delivery.

#### Possible Causes
Provider side disruptions.

#### Possible Solutions
 Please attempt to send messages at a later time."#,
            TwilioProgrammableSmsError::ErrorCode63005 => r#"## ERROR - 63005

### Channel did not accept given content. Please see Channel specific error message for more information

 ### Channel did not accept given content

#### Possible Causes
*   The content you are trying to send was rejected by the Channel. Please see Channel specific error message for more information. 
*   When sending messages to an end user on WhatsApp, this error may occur when the end user is not a valid WhatsApp user.
*   When sending messages with media to an end user on WhatsApp, this error may occur when WhatsApp detects an issue with the media file sent.

#### Possible Solutions
*   Make sure the end user you are messaging has a valid WhatsApp account.
*   Make sure the media file sent is valid. Try to download the media file being sent. Export the file to a different valid format and re-upload it to your storage solution. If sending over WhatsApp, resubmit a new template for approval with the new media file.
*   Confirm that you are sending rich content within the sending session guidelines. If fields that are only supported as part of a template or fields that are only available in session are sent incorrectly, this error may occur. "#,
            TwilioProgrammableSmsError::ErrorCode63013 => r#"## ERROR - 63013

### Channel policy violation

This message send failed because it violates Channel provider's policy. 



#### Possible Causes
This message send failed because it violates Channel provider's policy. Please see Channel specific error message for more information. 
*  This error may also be seen if more than 4 consecutive whitespaces replace a placeholder in a WhatsApp message body.

#### Possible Solutions
Check content is in compliance with Channel providers policy."#,
            TwilioProgrammableSmsError::ErrorCode30017 => r#"## ERROR - 30017

### Carrier network congestion

Message request was rejected because the carrier has high traffic. A 30017 error is an indicator that the downstream carrier is experiencing performance issues due to high traffic. During times of carrier network congestion, we re-try the request, and return this error if the request is not accepted.

#### Possible Causes
* Carrier is experiencing congestion in MMS traffic

#### Possible Solutions
* Send your message over a longer period of time, rather than sending in bursts"#,
            TwilioProgrammableSmsError::ErrorCode30028 => r#"## ERROR - 30028

### Invalid API version

 API version used in the request is invalid

#### Possible Causes
API version is not specified or is not either 2008-08-01 or 2010-04-01

#### Possible Solutions
Use API version 2008-08-01 or 2010-04-01"#,
            TwilioProgrammableSmsError::ErrorCode21713 => r#"## ERROR - 21713

### Messaging Service Use Case is Invalid

 The Messaging Service use case you are trying to specify in the request is not a valid use case. 

#### Possible Causes
* The Messaging Service use case specified in the request is not a valid Messaging Service use case. Valid Messaging Service use cases include: discussion, marketing, notifications, poll, undeclared and verification.

#### Possible Solutions
* Verify that you are spelling the use case correctly in your request and that it is using all lowercase letters."#,
            TwilioProgrammableSmsError::ErrorCode21708 => r#"## WARNING - 21708

### Alpha Sender ID Missing from the request

 ## Error - 21708

### Alpha Sender ID Missing from the request 

#### Possible Causes

* Ensure that a request contains an alphasender in it

#### Possible Solutions
Please include an alphasender in the request"#,
            TwilioProgrammableSmsError::ErrorCode30030 => r#"## ERROR - 30030

### Invalid AddressRetention

 AddressRetention value used in the request is invalid

#### Possible Causes
AddressRetention is not either 'obfuscate' or 'retain' or Message Privacy features are not enabled for this account

#### Possible Solutions
Enable the correct Message Privacy features and use a valid value for AddressRetention"#,
            TwilioProgrammableSmsError::ErrorCode30500 => r#"## ERROR - 30500

### Twilio Internal Error

 Twilio's platform encountered an internal error while processing this message.

#### Possible Causes
* If this message was being processed during an incident, it is possible that this message was affected. See [Twilio's status page](https://status.twilio.com/) for ongoing and historical incidents.
* There was an unrecoverable anomaly that occurred while processing this particular message.

#### Possible Solutions
* Try sending the message again.
* If the subsequent attempt also fails with this error, [contact support](https://support.twilio.com/hc/en-us/articles/360048500694-Contacting-Twilio-Support) referencing the affected Message Sids for further troubleshooting."#,
            TwilioProgrammableSmsError::ErrorCode21603 => r#"## ERROR - 21603

### A 'From' or 'MessagingServiceSid' parameter is required to send a message

Make sure to specify a valid Twilio Phone Number, Alpha Sender ID, or Messaging Service SID in your API requests The 'From' parameter in your API request to send an outbound message did not include a valid [Twilio Number](/user/account/phone-numbers/), [Alphanumeric Sender ID](https://support.twilio.com/hc/en-us/articles/223181348-Alphanumeric-Sender-ID-for-Twilio-Programmable-SMS), or [Messaging Service SID](https://www.twilio.com/docs/messaging/services).
	
You can provide any of the above sender types in your From parameter. You may also provide a Messaging Service SID using the dedicated 'MessagingServiceSid' parameter, either instead of or in addition to a specific 'From' parameter.

#### Possible Causes
* You did not specify a 'From' parameter in your API request

#### Possible Solutions
* Specify a valid [Twilio Number](/user/account/phone-numbers/), [Alphanumeric Sender ID](https://support.twilio.com/hc/en-us/articles/223181348-Alphanumeric-Sender-ID-for-Twilio-Programmable-SMS), or [Messaging Service SID](https://www.twilio.com/docs/messaging/services) as the 'From' parameter in your API request
* Specify a valid Messaging Service SID as the 'MessagingServiceSid' parameter in your API request"#,
            TwilioProgrammableSmsError::ErrorCode57010 => r#"## ERROR - 57010

### 'PartnerName' is absent

 'PartnerName' is not specified

#### Possible Causes
'PartnerName' is not specified

#### Possible Solutions
Confirm a valid 'PartnerName' is being passed in request."#,
            TwilioProgrammableSmsError::ErrorCode63030 => r#"## ERROR - 63030

### Unsupported parameter for type of channels message

 ### Media messages are not supported with template (structured) messages. Please check your message and try sending a template message without the media
### URL preview messages sent with incorrect URL format

#### Possible Causes
* Media message not supported with template messages
* No URL to support preview

#### Possible Solutions
* Send template message without the media component
* Check url for correctness for url previews"#,
            TwilioProgrammableSmsError::ErrorCode14107 => r#"## ERROR - 14107

### SMS send rate limit exceeded

Too many messages have been sent between two numbers within a short time period, possibly indicating a runaway job or infinite loop. There is a limit of 30 outbound replies between two phone numbers in a short period. Too many messages have been sent between two numbers within a short time period, possibly indicating a runaway job or infinite loop. There is a limit of 30 outbound replies between two phone numbers in a 30-second period.

A counter is set for each outbound reply in a conversation (between two numbers). When the first reply occurs, the counter starts at 1. If the next reply is sent less than 30 seconds after the prior reply, then the counter goes to 2. This will continue if each reply is less than 30 seconds, until the counter surpasses 30; at this time, Twilio will set any further messages in the loop to "failed" status for 30 seconds, with this error code.

#### Possible Causes
*  repeated rapid responses by the end user (30 replies in less than 30 seconds)
*  infinite loop caused by self referencing Sms or Message verb action URL
*  infinite loop caused by self referencing Redirect verb URL
*  runaway process making repeated outgoing REST API requests

#### Possible Solutions
*  make certain any action or Redirect URL's do not loop back to the same TwiML document.
*  make sure you are not inadvertently sending a large quantity of messages to the same phone number, e.g. a script caught in a loop
* if you need this protection disabled for your Twilio Account, [contact Twilio Support](https://www.twilio.com/console/support/tickets/create). Please note, if you choose to have this feature disabled, you must have protection in place in your application to prevent costly auto-reply loops."#,
            TwilioProgrammableSmsError::ErrorCode63002 => r#"### Channel could not find the From address

#### Possible Causes 
*   Incorrect From address or trying to use a From address that is not supported by this Channel when trying to send a message. See [here](https://www.twilio.com/docs/api/channels#channel-addresses) for supported formats of Channel endpoint addresses. "#,
            TwilioProgrammableSmsError::ErrorCode30109 => r#"## ERROR - 30109

### Callback URL is invalid

The URL specified in your API request for callback is invalid The URL specific in your API request for Click Tracking is invalid

#### Possible Causes
- Protocol could be missing
- Hostname could be invalid
- Incorrect encoding

#### Possible Solutions
Make sure you submit a fully qualified URL including:

*   protocol (http:// or https://)
*   hostname
*   file path
*   properly url-encoded query parameters"#,
            TwilioProgrammableSmsError::ErrorCode21656 => r#"## ERROR - 21656

### The ContentVariables Parameter is invalid

 

#### Possible Causes
* The ContentVariables Parameter is not a JSON String. 
* A ContentVariables variable placeholder is improperly formed e.g. {{example}. 
* A ContentVariables variable placeholder is an "empty mustache" e.g. {{}}. 
* The ContentVariables field is defined with improper formatting at time of send. 
* The ContentVariables field contains a variable that was set to null at time of send.


#### Possible Solutions
* Review the [documentation](https://www.twilio.com/docs/content/using-variables-with-content-api) on how to use Content Variables with Content Templates.
* Validate the ContentVariables Parameter, it must be a JSON String and be properly formatted. 
* Make sure all ContentVariables are defined and have a valid definition at time of send.
"#,
            TwilioProgrammableSmsError::ErrorCode57012 => r#"## ERROR - 57012

### Signature invalid

 Signature invalid

#### Possible Causes
Request signature passed in X-Registry-Signature header is invalid

#### Possible Solutions
Confirm signature consists of WebhookURL + canonicalized JSON payload and signed with HMAC-SHA1."#,
            TwilioProgrammableSmsError::ErrorCode30102 => r#"## ERROR - 30102

### TLS certificate for your Domain has expired.

TLS certificate associated with the Domain is expired. TLS certificate for the Domain you are using with Twilio Link Shortening has been expired. 

#### Possible Causes
TLS certificate associated with the Domain is expired.

#### Possible Solutions
Generate and upload a new TLS certificate for the Domain through Twilio Link Shortening console or via Domain Certificate API."#,
            TwilioProgrammableSmsError::ErrorCode63015 => r#"## ERROR - 63015

### Channel Sandbox can only send messages to phone numbers that have joined the Sandbox

 Channel Sandbox can only send messages to phone numbers that have joined the Sandbox

#### Possible Causes
This message failed to be delivered to the user because it was sent from a Sandbox to a phone number that has not joined your Sandbox

#### Possible Solutions
Instruct the recipient to join your Sandbox by sending the join phrase or keyword shown on the Sandbox page in your Twilio Console."#,
            TwilioProgrammableSmsError::ErrorCode92006 => r#"## ERROR - 92006

### The Content Sid is Invalid

The Content Sid you are using is not valid 

#### Possible Causes
The Content Sid you are using is not valid

#### Possible Solutions
Double check the Content Sid parameter that you are using. Make sure it has the proper formatting and starts with an HX prefix."#,
            TwilioProgrammableSmsError::ErrorCode63032 => r#"## ERROR - 63032

### We cannot send this message to this user because of a WhatsApp limitation. 

 User's number is part of an experiment.

#### Possible Causes
Failed to send message because this user's phone number is part of an experiment.

#### Possible Solutions
Skip sending messages to this user."#,
            TwilioProgrammableSmsError::ErrorCode30005 => r#"## ERROR - 30005

### Unknown destination handset

The destination number you are trying to reach is unknown and may no longer exist. ## Error - 30005

## Message Delivery - Unknown destination handset



#### Possible Causes
*  The destination number you are trying to reach is unknown and may no longer exist.
*  The device you are trying to reach is not on or does not have sufficient signal.
*  The device cannot receive SMS (for example, the phone number belongs to a landline)
*  There is an issue with the mobile carrier

#### Possible Solutions
The first step to troubleshooting this issue is to attempt to replicate the problems. Attempt to send another test message to this user via a REST API request, or through the API Explorer in the [Twilio Console](https://www.twilio.com/console/runtime/api-explorer/voice). 

Pay close attention to your request and double check to verify you are attempting to send messages to the correct phone number in the correct [E.164](https://www.twilio.com/docs/glossary/what-e164) format: `[+] [country code] [subscriber number including area code]` 

If you see similar results, continue troubleshooting with the following checklist:
*  Is the destination device powered on?
*  Does the device have sufficient signal? If not power the device off, wait 30 seconds, and then power it back up.
*  Is the device connected to the home carrier's network? We cannot guarantee message delivery on devices roaming off-network.
*  Can the device receive non-Twilio SMS?
*  Can the device receive messages from another Twilio number (non-Alphanumeric Sender ID), or with a shorter one-segment (non-concatenated) body?
*  Can other devices using the same mobile carrier receive your messages?

#### Continued issues with Error 30005
Twilio's Support team can help investigate what went wrong with delivering your message. Please collect 3 or more message SIDs in your [SMS logs](https://www.twilio.com/console/sms/logs) from the last 24 hours that were flagged with `Error 30005`, and [open a support request](https://www.twilio.com/console/support/tickets/create)."#,
            TwilioProgrammableSmsError::ErrorCode30110 => r#"## ERROR - 30110

### Domain is blocked

The provided URL to resolve this link to is on a blocked domain. The provided URL to resolve the link is on a blocked domain.

#### Possible Causes
- Fraudulent activity has been detected on this domain
- Domain owner has requested Twilio not link to their site

#### Possible Solutions
Please reach out to Twilio support"#,
            TwilioProgrammableSmsError::ErrorCode14109 => r#"## WARNING - 14109

### TwiML Reply message limit exceeded

Too many messages have been sent in reply to an incoming message. The current is 10 ## Warning - 14109 

### TwiML Reply message limit exceeded

Too many TwiML reply messages have been sent in response to a single incoming message. You can send a maximum of 10 TwiML replies per incoming message.

#### Possible Causes
* A runaway loop or other error in your application generated more than 10 TwiML replies to a single incoming message
* You have attempted to send >10 TwiML replies to a single incoming message

#### Possible Solutions
* If you need to generate >10 outbound messages for one incoming message, please use new API requests to create these messages."#,
            TwilioProgrammableSmsError::ErrorCode35114 => r#"## ERROR - 35114

### Scheduling does not support this timestamp

 SendAt time must be between 900 seconds and 35 days (3024000 seconds) in the future, inclusive.

#### Possible Causes
Scheduler currently doesn't support scheduling a message at a fixed time less that 15 minutes from now, or more than 35 days in the future. *Scheduling up to 35 days before is available in Pilot with limited access.

#### Possible Solutions
Verify that the SendAt parameter is using "[YYYY]-[MM]-[DD]T[HH]:[MM]:[SS]Z" format (in UTC). 

Check the SendAt parameter to ensure it is is greater than 15 minutes (900 seconds) from request time and 35 days (3024000 seconds) in the future."#,
            TwilioProgrammableSmsError::ErrorCode21704 => r#"## ERROR - 21704

### The Messaging Service contains no phone numbers

 ## Error - 21704

### The Messaging Service contains no senders.

A Messaging Service requires at least one phone number, Alphanumeric Sender ID or short code to send messages.







#### Possible Causes
* Your Messaging Service does not contain any senders

#### Possible Solutions
* Add at least one phone number or short code to your Messaging Service, or enable Alphanumeric Sender ID."#,
            TwilioProgrammableSmsError::ErrorCode21721 => r#"## ERROR - 21721

### Cannot import Campaign Verify token due to incompatible A2P brand

 The attempt to import a Campaign Verify token to your brand failed because your A2P brand is in an incompatible state.

#### Possible Causes
- Your A2P brand has a status of `FAILED` meaning the brand was not successfully registered
- Your A2P brand is of type `STARTER` meaning it is a starter brand, not a standard brand

#### Possible Solutions
- Verify if the Brand Registration SID has been registered successfully by checking the `status` field using the [Brand API](https://www.twilio.com/docs/sms/a2p-10dlc/onboarding-isv-api#using-get-to-check-brand-registration-status). If the brand has been registered successfully, it will have a status of `APPROVED`. If this is the case, try to import the token again.
- Verify that the Brand Registration SID is of the correct type by checking the `brand_type` field using the [Brand API](https://www.twilio.com/docs/sms/a2p-10dlc/onboarding-isv-api#using-get-to-check-brand-registration-status). The brand should have a type of “STANDARD.” If the `brand_type` is `STARTER`, the brand does not support importing a Campaign Verify token. Once you’ve verified that you are using a standard brand, try to import the token again
"#,
            TwilioProgrammableSmsError::ErrorCode30033 => r#"## ERROR - 30033

### US A2P 10DLC - Campaign Suspended

 Your message could not be sent because you are sending this message from a suspended US A2P 10DLC Campaign.

#### Possible Causes
Campaign Suspension

#### Possible Solutions
Please reach out to the Twilio Support team to receive more details around why your campaign has been suspended, and how to resolve the issue."#,
            TwilioProgrammableSmsError::ErrorCode23001 => r#"## WARNING - 23001

### Message Redaction Incompatible Configuration: Long code STOP filtering

 Phone Number Redaction is enabled for your account. Twilio's default opt-out keyword handling ("SMS STOP filtering") is not compatible with Phone Number Redaction and should be disabled.

#### Possible Causes
* Twilio is currently set to handle opt-out keywords for long code numbers on your account. Twilio's opt-out handling requires Twilio to save non-redacted phone numbers of users who have opted out, and is incompatible with Phone Number Redaction.

#### Possible Solutions
* To resolve this issue, your application needs to be able to handle [opt-out keyword messages](https://support.twilio.com/hc/en-us/articles/223134027-Twilio-support-for-opt-out-keywords-SMS-STOP-filtering-) from end users.
* Users who reply STOP, STOPALL, UNSUBSCRIBE, CANCEL, END, or QUIT should be added to a blocklist in your application to ensure you do not send them any further messages, unless they opt-in again.
* Users who reply HELP or INFO should be sent an informational message about your application or service.
* Users who reply START, YES, or UNSTOP should be removed from your "STOP" blocklist so they can receive messages from you again.
* Once you are ready to disable Twilio's built-in opt-out handling and process the above keywords in your own application, please reach out to Twilio Support through the [Console](https://console.twilio.com) or [Help Center](https://support.twilio.com) to disable Twilio's built-in opt-out keyword handling for your account."#,
            TwilioProgrammableSmsError::ErrorCode63033 => r#"## ERROR - 63033

### Recipient opted out to receive message

 The WhatsApp message you are attempting to send uses a "Marketing" category template and the user you are trying to reach has opted out from receiving your marketing messages.

#### Possible Causes
Recipient has opted out of WhatsApp marketing messages from your business.

#### Possible Solutions
The user needs to opt back into WhatsApp marketing messages from your business. You may still send templated WhatsApp messages of other categories, such as Authentication and Utility, to this user."#,
            TwilioProgrammableSmsError::ErrorCode30134 => r#"## ERROR - 30134

### Invalid Dns Setup for Link shortening

 

#### Possible Causes
Domain Dns setup is not completed 

#### Possible Solutions
If you're using a root domain, point the A record of the domain to the following 3 IP addresses only. If other IPs are included, Link Shortening does not work.

- 3.233.187.46
- 3.233.108.250
- 54.157.2.211

If you're using a subdomain, point the CNAME record of the subdomain to the following:

- lsct.ashburn.us1.twilio.com"#,
            TwilioProgrammableSmsError::ErrorCode23003 => r#"## WARNING - 23003

### Message Redaction Incompatible Configuration: Sticky Sender

 Phone Number Redaction is enabled for your account. Sticky Sender is not compatible with Phone Number Redaction and should be disabled.

#### Possible Causes
* You have Sticky Sender enabled on your Messaging Service. Sticky Sender requires Twilio to save the non-redacted phone numbers of recipients, and is incompatible with Phone Number Redaction.

#### Possible Solutions
* Disable Sticky Sender on your Messaging Services using the [Twilio Console](https://www.twilio.com/console/sms/services) or the [REST API](https://www.twilio.com/docs/messaging/services/api).
* If Sticky Sender functionality is needed, implement this functionality in your own application."#,
            TwilioProgrammableSmsError::ErrorCode63010 => r#"### Twilio Channels Internal error

#### Possible Causes 
*   Twilio's platform encountered an internal error processing this message"#,
            TwilioProgrammableSmsError::ErrorCode21728 => r#"## ERROR - 21728

### Campaign registration failed due to length validation failures

 We could not register your US A2P Campaign because one or more field(s) do not satisfy length requirements.

#### Possible Causes
One or more field(s) do not satisfy length requirements.

#### Possible Solutions
Please fix the error and retry Campaign registration. Please check out our [support article](https://support.twilio.com/hc/en-us/articles/8959909733403-Changes-to-the-A2P-10DLC-Campaign-Creation-Process) to read about the length requirements on these fields."#,
            TwilioProgrammableSmsError::ErrorCode63017 => r#"### Rate limit exceeded

#### Possible Causes 
* Your account or subaccounts are sending messages at a higher combined rate than 2000 messages-per-second."#,
            TwilioProgrammableSmsError::ErrorCode30035 => r#"## ERROR - 30035

### US A2P 10DLC - Message from a number still being configured

 We could not send your message because this number you're sending from is still being configured. Twilio has received your request to register or deregister this number under a US A2P campaign, but has not completed the necessary configurations with ecosystem partners (“Number Registration”). You will not be able to send messages on the A2P routes until your number status is changed to “Registered”.

#### Possible Causes
Twilio is still processing your Number Registration requests with ecosystem partners for US A2P 10DLC

#### Possible Solutions
While Number Registration often completes within minutes, under certain circumstances, registration time can widely fluctuate depending on the volume of registration requests. Twilio has received your Number Registration request and is working on configuring your number with ecosystem partners. Please [check the status of your number in the Twilio console](https://console.twilio.com/us1/develop/phone-numbers/manage/incoming/a2p-compliance/status?_gl=1*1ttowh2*_ga*MTY4MDY0MDY3NC4xNjkyODkzNTk4*_ga_RRP8K4M4F3*MTY5NzgzNDU4NC42OC4xLjE2OTc4MzQ2MTcuMC4wLjA.&_ga=2.240863811.1963643099.1697477953-1680640674.1692893598). You will only be able to send messages on numbers that are in the “Registered” status. To get your number registered, please check out [A2P 10DLC Number Registration Best Practices](https://support.twilio.com/hc/en-us/articles/19622397178139-A2P-10DLC-Number-Registration-Best-Practices).

NOTE:  Users of multiple phone numbers should note that antiquated phone number practices to improve deliverability, such as using multiple numbers to increase throughput or rotating them between their customers or use cases, undermine the deliverability benefits of A2P 10DLC and can lead to processing delays related to 30035 errors. We recommend reviewing [A2P 10DLC Number Registration Best Practices](https://support.twilio.com/hc/en-us/articles/19622397178139-A2P-10DLC-Number-Registration-Best-Practices)."#,
            TwilioProgrammableSmsError::ErrorCode92003 => r#"## ERROR - 92003

### 'language' Parameter Required

Invalid language. Language is required 

#### Possible Causes
You attempted to create content, but you forgot to include the 'language', the language property is required. 

#### Possible Solutions
Please include a supported language"#,
            TwilioProgrammableSmsError::ErrorCode21727 => r#"## ERROR - 21727

### Campaign registration failed due to missing parameter(s)

 We could not register your US A2P Campaign because one or more required parameter(s) are missing

#### Possible Causes
One or more required parameters are missing for Campaign registration

#### Possible Solutions
Please include all required parameters and retry Campaign registration. Please check out our [support article](https://support.twilio.com/hc/en-us/articles/8959909733403-Changes-to-the-A2P-10DLC-Campaign-Creation-Process) to understand which fields are required for campaign registration. "#,
            TwilioProgrammableSmsError::ErrorCode63026 => r#"### Channel sender content flagged as spam

#### Possible Causes 
* Sender content was flagged as spam. Please check the content of the message."#,
            TwilioProgrammableSmsError::ErrorCode23006 => r#"## WARNING - 23006

### Message Redaction Incompatible Configuration: Inbound Webhook GET Requests

 Message Redaction is enabled for your account. Using GET requests to handle incoming messages is not compatible with either Message Body Redaction or Phone Number Redaction.

#### Possible Causes
* The Incoming Message Webhook is set to the GET method for your Twilio phone number or Messaging Service.

#### Possible Solutions
* Ensure all Twilio numbers and Messaging Services on your account are set to use the POST method when handling incoming messages."#,
            TwilioProgrammableSmsError::ErrorCode21714 => r#"## ERROR - 21714

### Messaging Service Number Pool size limit reached

 You have attempted to add a Twilio phone number, alpha sender, or short code to your Messaging Service Number Pool, but you are already at the limit of phone numbers or alpha senders. By default, a Messaging Service can contain up to a total of 400 Twilio phone numbers and short codes and only one Alpha Sender per country.


#### Possible Causes
* Your Messaging Service number pool is already full (default: 400 numbers)
* You may have attempted to add more than one alpha sender for the same country to a Messaging Service or Sender Pool. Each country can only have one alpha sender assigned per Messaging Service Number Pool.


#### Possible Solutions
* If you are sending to the United States and Canada, use a [Toll-Free or short code number](https://support.twilio.com/hc/en-us/articles/360038173654-Comparison-of-SMS-messaging-in-the-US-and-Canada-for-long-codes-short-codes-and-toll-free-phone-numbers) with higher message throughput (MPS), instead of adding more local numbers. Most mobile carriers in the US and Canada do not not permit [Application-to-Person (A2P)](https://support.twilio.com/hc/en-us/articles/223133807-What-is-P2P-and-A2P-messaging-) messaging via long codes, so there is a high risk of [carrier filtering](https://support.twilio.com/hc/en-us/articles/223181848-How-Does-Carrier-Filtering-Work-) when sending this type of traffic via long code.
* If you are sending to other countries, you may be eligible to have your Number Pool size limit increased. Please [submit a ticket](https://www.twilio.com/console/support/tickets/create) with a description of your use case and why you need a higher number pool size limit. These requests will be reviewed by our Messaging team."#,
            TwilioProgrammableSmsError::ErrorCode30004 => r#"## ERROR - 30004

### Message blocked

The destination number you are trying to reach is blocked from receiving this message. ## Error - 30004

## Message Delivery - Message blocked

#### Possible Causes
*  The destination number you are trying to reach is blocked from receiving this message.
*  The device you are trying to reach does not have sufficient signal.
*  The device cannot receive SMS (for example, the phone number belongs to a landline).
*  The destination number is on India's national Do Not Call registry.
*  There is an issue with the mobile carrier.
*  You have sent a message from a US/CA Toll-free number to an end user handset that has previously responded with "STOP" or [another opt-out keyword](https://support.twilio.com/hc/en-us/articles/223134027-Twilio-support-for-opt-out-keywords-SMS-STOP-filtering-).

#### Possible Solutions
The first step to troubleshooting this issue is to attempt to replicate the problem. Attempt to send another test message to this user via a REST API request or through the API Explorer in the [Twilio Console](https://www.twilio.com/console/runtime/api-explorer/voice). 

If you see similar results, continue troubleshooting with the following checklist:
*  Is the destination device powered on?
*  Does the device have sufficient signal? If not, power the device off, then wait 30 seconds and then power it back up.
*  Is the device connected to the home carrier's network? We cannot guarantee message delivery to devices roaming off-network.
*  Can the device receive non-Twilio SMS?
*  Can the device receive messages from another Twilio number (not using an [Alphanumeric Sender ID](https://www.twilio.com/docs/glossary/what-alphanumeric-sender-id)), or with a shorter one-segment (non-[concatenated](https://support.twilio.com/hc/en-us/articles/223181508-Does-Twilio-support-concatenated-SMS-messages-or-messages-over-160-characters)) body?
*  Can other devices using the same mobile carrier receive your messages?

If you can rule out all of the above issues, continue troubleshooting below.

#### "Undelivered" messages sent to India
Error 30004 results on undelivered messages sent to India could be due to the destination number being listed on the national Do Not Call (DNC) registry. 

For more information on sending messages to India, including links for updating DNC settings, please see our help center article on [limitations for sending SMS to India](https://support.twilio.com/hc/en-us/articles/223134167-Limitations-sending-SMS-messages-to-Indian-mobile-devices).

#### "Failed" messages
Repeated messages with a `Failed` status and error 30004 results indicate Twilio may be filtering some of your messages internally. 

This filtering is in place to help protect our customers, and the rules attempt to match what we have observed from the industry. For more details, including potential workarounds, please see our help center article ["How Does Carrier Filtering Work?"](https://support.twilio.com/hc/en-us/articles/223181848-How-Does-Carrier-Filtering-Work)

#### Continued issues with error 30004
Twilio's support team can help investigate what went wrong with delivering your message. Please collect 3 or more message SIDs in your [SMS logs](https://www.twilio.com/console/sms/logs) from the last 24 hours that were flagged with Error 30004, and [open a support request](https://www.twilio.com/console/support/tickets/create)."#,
            TwilioProgrammableSmsError::ErrorCode63037 => r#"## WARNING - 63037

### Channel Media Upload Error

 Media failed to upload to provider

#### Possible Causes
An attempt by Twilio failed to upload media to the Channel provider. Twilio will retry this operation.

#### Possible Solutions
Twilio will retry this operation."#,
            TwilioProgrammableSmsError::ErrorCode21715 => r#"## ERROR - 21715

### Phone Number Does Not Have Correct Messaging Service Capabilities

 In order for a phone number to work in a Messaging Service it must have at least SMS or MMS capabilities. If it does not have either, it will fail.

#### Possible Causes
* The phone number you are trying to add to the Messaging Service does not have SMS or MMS capabilities.

#### Possible Solutions
* The [Active Numbers](https://www.twilio.com/console/phone-numbers/incoming) page in the Twilio Console will display the capabilities of each of the phone numbers in your account.
* Find a new phone number that has SMS and/or MMS capabilities depending on your use case and business needs, and add that phone number to your Messaging Service."#,
            TwilioProgrammableSmsError::ErrorCode63039 => r#"## ERROR - 63039

### Warning! Facebook says your page is engaging in behavior that may be considered bothersome or abusive by users. To avoid messaging restrictions being placed on your Page, Facebook requires you to immediately decrease the rate at which you are sending messages outside the 24-hour window to this person.

 

#### Possible Causes
Facebook and/or Facebook users consider the messages abusive or bothersome, falling outside of Facebook's policies.

#### Possible Solutions
Soften the content of your message and decrease the rate at which messages are being sent to the user outside of the 24-hour session."#,
            TwilioProgrammableSmsError::ErrorCode30453 => r#"## ERROR - 30453

### Message couldn't be delivered

 Message couldn’t be delivered to the destination number you are trying to reach temporarily.

#### Possible Causes
* This is because Twilio has identified potential fraudulent messages being sent to the destination you are trying to reach.
* Hence SMS traffic can not be delivered to the destination phone number used for the next couple of hours. The traffic should return to normalcy post that. However, the SMS traffic could be undelivered for an even longer period if fraudulent activity doesn't subside & continues to go on.

#### Possible Solutions
You do not need to take any specific action. The disruption in traffic is usually temporary and will resolve in a couple of hours if we do not detect additional fraudulent activity.

#### Continued issues with error 30453
Twilio's support team can help investigate what went wrong with delivering your message. Please collect 3 or more message SIDs in your [SMS logs](https://www.twilio.com/console/sms/logs) from the last 24 hours that were flagged with Error 30453, and [open a support request.](https://www.twilio.com/console/support/tickets/create)

Alternatively, you can use the optional <code style="color : name_color">RiskCheck</code> parameter when [creating a Message](https://www.twilio.com/docs/messaging/api/message-resource#create-a-message-resource) with the [Programmable Messaging API](https://www.twilio.com/docs/messaging/api). To prevent a known/legitimate message from getting blocked in future by SMS Pumping Protection, include the <code style="color : name_color">RiskCheck</code> parameter with value <code style="color : name_color">disable</code> when creating the new Message resource.

You can also mark known/legitimate phone numbers using the Global Safe List feature so that they don’t face any disruption . This provides an additional safety net against false positives, so that the traffic to this number never faces the above scenario. Add known phone numbers to the Safe List by using the [Global Safe List API.](https://www.twilio.com/docs/usage/global-safe-list)"#,
            TwilioProgrammableSmsError::ErrorCode30106 => r#"## ERROR - 30106

### Domain has not been set up for this account

Domain has not been set up for this account Domain has not been setup for click tracking

#### Possible Causes
Onboarding process is incomplete.

#### Possible Solutions
Finish onboarding."#,
            TwilioProgrammableSmsError::ErrorCode21726 => r#"## ERROR - 21726

### Starter brand registrations and updates are temporarily disabled

 Starter brands cannot be created or updated at this time. Contact support for help.

#### Possible Causes
The starter brand you are trying to create/update is a Starter brand. See [New Requirements for A2P 10DLC Registrations](https://www.twilio.com/blog/new-requirements-for-a2p-10dlc-registrations) for more info.

#### Possible Solutions
Contact Twilio support."#,
            TwilioProgrammableSmsError::ErrorCode35115 => r#"## ERROR - 35115

### Scheduling does not support this timestamp

 SendAt time must be between 900 seconds and 35 days (3024000 seconds) in the future, inclusive.

#### Possible Causes
Scheduler currently doesn't support scheduling a message at a fixed time less that 15 minutes from now, or more than 35 days in the future.

#### Possible Solutions
Verify that the SendAt parameter is using "[YYYY]-[MM]-[DD]T[HH]:[MM]:[SS]Z" format (in UTC)."#,
            TwilioProgrammableSmsError::ErrorCode30018 => r#"## WARNING - 30018

### Destination carrier requires sender ID pre-registration

 You sent a message (message SID: SMXXXXX) to a mobile number in a country that requires Alphanumeric Sender ID pre-registration. Our records indicate that you do not have a registered Alphanumeric Sender ID for this country. This can result in lower delivery quality.

If you are sending transactional SMS (like OTP/auth codes, account-related alerts) you should pre-register for the best possible delivery quality.

#### Possible Causes
* The recipient you are sending a message to is on a network that requires alphanumeric sender ID pre-registration, and our records indicate that your sender ID is not registered.

#### Possible Solutions
* If your message is marked "undelivered," your message was likely rejected by the mobile network and you will need to register an Alpha Sender ID to improve delivery to that network.
* If your message is marked "delivered," it may have been delivered successfully. However, it is still strongly recommended to register an Alpha Sender ID for the destination country if possible.
* Learn more about Alpha Sender ID registration requirements [here](https://support.twilio.com/hc/en-us/articles/223133767-International-support-for-Alphanumeric-Sender-ID).
* Use [this form](https://twiliodoer.secure.force.com/SenderId) to register a Sender ID for the destination country.
* If you are sending OTP/2FA codes consider using [Twilio Verify or Authy](https://www.twilio.com/en-us/trusted-activation/verify) which will automatically take care of all applicable registrations."#,
            TwilioProgrammableSmsError::ErrorCode30101 => r#"## ERROR - 30101

### Domain is unverified

The domain has not been verified to work with Twilio Link Shortening. Domain need to be verified under your Organization through the Admin Center in order use it with Twilio Link Shortening.

#### Possible Causes
You have yet to verify your Domain in Organization. You could be using the wrong Domain SID in your current request. 

#### Possible Solutions
Please verify the Domain you wish to use with Twilio Link Shortening. "#,
            TwilioProgrammableSmsError::ErrorCode30025 => r#"## WARNING - 30025

### US A2P 10DLC - 50% T-Mobile Daily Message Limit Consumed

As part of the US A2P 10DLC initiative, T-Mobile has instituted a daily message limit. We’re sending you this warning to let you know that you have consumed 50% of your T-Mobile daily limit. Please note that this daily limit is based on the sum of SMS segments and MMS messages you send to T-Mobile subscribers on a 24-hour basis, and is reset daily at midnight PT. When you hit 100% of your daily message limit, you will start to receive error code 30023 for each subsequent message you send to T-Mobile. Those messages will not be delivered, and you must wait until midnight PT to resume message sending.

For more information on T-Mobile daily limit, please refer to [T-Mobile daily message limits for long code messaging with A2P 10DLC](https://support.twilio.com/hc/en-us/articles/1260804800549-T-Mobile-daily-message-limits-for-long-code-messaging-with-A2P-10DLC). For more information on US A2P 10DLC, please refer to [US A2P 10DLC Documentation](https://support.twilio.com/hc/en-us/articles/1260800720410-What-is-A2P-10DLC-).

Please disregard this warning if you have been approved for the unlimited tier via the [T-Mobile Special Business Review process](https://support.twilio.com/hc/en-us/articles/4403550579739-T-Mobile-Special-Business-Review-for-A2P-10DLC).

Please note that if you have multiple providers sending registered SMS or MMS to T-Mobile on your behalf, this warning might not be accurate.

#### Possible Causes
* You have consumed 50% of your T-Mobile daily message limit for the day.

#### Possible Solutions
* No action is required from you. 
* To view the warning and errors in the debugger, please visit [here](https://console.twilio.com/us1/monitor/logs/debugger/events?frameUrl=%2Fconsole%2Fdebugger%3F__override_layout__%3Dembed%26quickDate%3D24%26x-target-region%3Dus1%26bifrost%3Dtrue&currentFrameUrl=%2Fconsole%2Fdebugger%3F__override_layout__%3Dembed%26bifrost%3Dtrue%26quickDate%3D24%26x-target-region%3Dus1).
* To view your daily T-Mobile message limit, please visit [Trust Hub - A2P Messaging](https://console.twilio.com/us1/account/trust-hub/a2p?frameUrl=%2Fconsole%2Ftrust-hub%2Fa2p-messaging%3Fx-target-region%3Dus1), select your US A2P brand, and view the  details of your TCR trust score (registered customers only).
* To view your daily T-Mobile usage, please use the Messaging Insights tool."#,
            TwilioProgrammableSmsError::ErrorCode63014 => r#"### Channel message blocked by user action

#### Possible Causes 
*   This message failed to be delivered to the user because it was blocked by some action on their part.  Please see Channel specific error message for more information."#
        }
    }
}
