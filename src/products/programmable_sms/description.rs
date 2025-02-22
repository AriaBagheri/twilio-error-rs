// AUTO-GENERATED. DO NOT MODIFY. YOUR CHANGES WILL BE OVER-WRITTEN
use crate::products::TwilioProgrammableSmsError;
use standard_error::traits::StandardErrorDescriptionTrait;

impl StandardErrorDescriptionTrait for TwilioProgrammableSmsError {
    fn description(&self) -> Option<&'static str> {
        match self {
            TwilioProgrammableSmsError::ErrorCode30111 => Some(r#"The URL provided is blocked."#),
            TwilioProgrammableSmsError::ErrorCode57016 => Some(r#"'Topic' is not specified"#),
            TwilioProgrammableSmsError::ErrorCode30118 => Some(r#"We could not parse the private key uploaded in your request."#),
            TwilioProgrammableSmsError::ErrorCode21725 => Some(r#"Brand can only be updated when in FAILED state. Contact support for help."#),
            TwilioProgrammableSmsError::ErrorCode30006 => Some(r#"### Message Delivery - Landline or unreachable carrier

The destination number is unable to receive this message.
"#),
            TwilioProgrammableSmsError::ErrorCode30117 => Some(r#"We could not parse the certificate uploaded in your request."#),
            TwilioProgrammableSmsError::ErrorCode30107 => Some(r#"Twilio requires a private certificate for us to generate and process shortened HTTPS links on your domain. This certificate has not been provided for this domain."#),
            TwilioProgrammableSmsError::ErrorCode57013 => Some(r#"'Topic' is not specified"#),
            TwilioProgrammableSmsError::ErrorCode57020 => Some(r#"Authorization failed"#),
            TwilioProgrammableSmsError::ErrorCode30019 => None,
            TwilioProgrammableSmsError::ErrorCode30043 => Some(r#"Message Misrouted as Domestic"#),
            TwilioProgrammableSmsError::ErrorCode90009 => Some(r#"Message SID already exists"#),
            TwilioProgrammableSmsError::ErrorCode92008 => Some(r#"Creating new twilio/button templates is no longer supported."#),
            TwilioProgrammableSmsError::ErrorCode90007 => Some(r#"## Error - 90007

### Invalid validity period value

Validity period can only be set to integers between 1 and 36,000 seconds (10 hours max)."#),
            TwilioProgrammableSmsError::ErrorCode30124 => Some(r#"MessagingServiceSID is missing"#),
            TwilioProgrammableSmsError::ErrorCode63016 => None,
            TwilioProgrammableSmsError::ErrorCode21654 => Some(r#"A ContentSid must be specified if ContentVariables are provided in the request."#),
            TwilioProgrammableSmsError::ErrorCode30409 => None,
            TwilioProgrammableSmsError::ErrorCode30119 => Some(r#"The provided private key and certificate are both are valid but they do not match"#),
            TwilioProgrammableSmsError::ErrorCode21606 => Some(r#"You can only send SMS messages from a phone number, Alphanumeric Sender ID or [short code](https://www.twilio.com/docs/glossary/what-is-a-short-code) provided by or ported to Twilio. Phone numbers must be SMS-capable. For Short Codes, the `From` number must be in the same country as the `To` number.

Visit the [phone numbers page](https://www.twilio.com/console/phone-numbers/incoming) of your account portal to view a list of phone numbers that you own and to determine whether they are SMS-capable."#),
            TwilioProgrammableSmsError::ErrorCode63031 => Some(r#"Channels message should not have same `From` and `To` number"#),
            TwilioProgrammableSmsError::ErrorCode23004 => Some(r#"Phone Number Redaction is enabled for your account. Advanced Opt-Out is not compatible with Phone Number Redaction and should be disabled."#),
            TwilioProgrammableSmsError::ErrorCode11751 => Some(r#"## Error - 11751

### Media Message -> Media exceeds messaging provider size limit

Maximum size limit depends on the messaging channel. 
* For an MMS message, the size limit is [5MB](https://www.twilio.com/docs/sms/accepted-mime-types).
* For a WhatsApp message, the size limit is [16MB](https://www.twilio.com/docs/whatsapp/guidance-whatsapp-media-messages).
"#),
            TwilioProgrammableSmsError::ErrorCode30036 => Some(r#"The specified validity period on this message has elapsed."#),
            TwilioProgrammableSmsError::ErrorCode30121 => Some(r#"Fallback URL is missing"#),
            TwilioProgrammableSmsError::ErrorCode21611 => Some(r#"Twilio queues messages based on the sending rate of a sender or an account. For example a US long code number can send one message [segment](https://www.twilio.com/blog/what-the-heck-is-a-segment.html) per second, and a short code may send over 100 message segments per second.

Messages can only be queued for 10 hours and then they automatically fail. This queue length limit can be reduced by setting a lower [Validity Period](https://www.twilio.com/en-us/blog/validity-period-promotional-verification-use-cases) within your Messaging Service settings or in your API requests to send messages.

You can monitor the length on your queue via the [Queue Insights dashboard](https://console.twilio.com/us1/monitor/insights/sms?frameUrl=/console/sms/insights/delivery?x-target-region%3Dus1&currentFrameUrl=/console/sms/insights/delivery?x-target-region%3Dus1%26__override_layout__%3Dembed%26q%3DKGFjdGl2ZUluc2lnaHRzVmlldzpsYXRlbmN5KQ%253D%253D%26bifrost%3Dtrue).

For more information please view this [FAQ](https://support.twilio.com/hc/en-us/articles/115002943027-Understanding-Twilio-Rate-Limits-and-Message-Queues).

When a queue for a particular 'From' number exceeds 10 hours, this error will be thrown when attempting to create additional messages on the same number.

Note: If you are using a Messaging Service and the numbers in the pool experience queue overload, messages will instead fail after creation with [error 30001](https://www.twilio.com/docs/api/errors/30001).
"#),
            TwilioProgrammableSmsError::ErrorCode30027 => Some(r#"Please note that this daily limit is based on the sum of SMS segments and MMS messages you send to T-Mobile subscribers on a 24-hour basis, and is reset daily at midnight PT. As you have consumed 100% of your limit, subsequent messages you send to T-Mobile are likely to be filtered. When filtering happens, you will start to receive 30023 error codes and must wait until midnight PT to resume message sending.

For more information on T-Mobile daily limit, please refer to [T-Mobile daily message limits for long code messaging with A2P 10DLC](https://support.twilio.com/hc/en-us/articles/1260804800549-T-Mobile-daily-message-limits-for-long-code-messaging-with-A2P-10DLC). For more information on US A2P 10DLC, please refer to [US A2P 10DLC Documentation](https://support.twilio.com/hc/en-us/articles/1260800720410-What-is-A2P-10DLC-).

Note: this error may not be accurate for certain customers such as those who have been approved of the unlimited tier via the [T-Mobile Special Business Review process](https://support.twilio.com/hc/en-us/articles/4403550579739-T-Mobile-Special-Business-Review-for-A2P-10DLC), those who run non-profit campaigns, and others."#),
            TwilioProgrammableSmsError::ErrorCode63008 => None,
            TwilioProgrammableSmsError::ErrorCode23002 => Some(r#"Phone Number Redaction is enabled for your account. Twilio's default opt-out keyword handling ("SMS STOP filtering") is not compatible with Phone Number Redaction and should be disabled."#),
            TwilioProgrammableSmsError::ErrorCode30011 => Some(r#"## Error - 30011 

### MMS not supported by the receiving phone number



"#),
            TwilioProgrammableSmsError::ErrorCode30130 => Some(r#"Messaging Service SID is already associated with a domain configuration in the account"#),
            TwilioProgrammableSmsError::ErrorCode21627 => Some(r#"Max Price must be a valid positive float"#),
            TwilioProgrammableSmsError::ErrorCode23005 => Some(r#"Phone Number Redaction is enabled for your account. Fallback to Long Code is not compatible with Phone Number Redaction and should be disabled."#),
            TwilioProgrammableSmsError::ErrorCode90014 => Some(r#"Validity Period should be positive integer"#),
            TwilioProgrammableSmsError::ErrorCode92005 => None,
            TwilioProgrammableSmsError::ErrorCode21408 => Some(r#"You have attempted to send an SMS or MMS to a region that has not been enabled in your account's [Messaging Geo-Permissions](https://console.twilio.com/us1/develop/sms/settings/geo-permissions) settings. These settings exist to help you ensure you only send SMS or MMS to the countries or regions you operate in, and avoid unexpected charges."#),
            TwilioProgrammableSmsError::ErrorCode35125 => Some(r#"There is a soft limit of 500,000 scheduled messages at any given time, per account_sid, that can be guaranteed. As each scheduled message is sent, more messages can be scheduled up to that limit."#),
            TwilioProgrammableSmsError::ErrorCode21712 => None,
            TwilioProgrammableSmsError::ErrorCode57006 => Some(r#"'EventType' is not specified"#),
            TwilioProgrammableSmsError::ErrorCode57007 => Some(r#"'EventType' is not being passed in request"#),
            TwilioProgrammableSmsError::ErrorCode30108 => Some(r#"Domains are managed at the Organization level. Please attach this account to an organization and proceed with setup for Click Tracking prior to use."#),
            TwilioProgrammableSmsError::ErrorCode57009 => Some(r#"Specified 'EventType' is too long"#),
            TwilioProgrammableSmsError::ErrorCode30022 => Some(r#"* Your message was rejected by the downstream carriers for exceeding the rate limits allowed by your campaign.
* Rarely, you may receive this error for sending too many messages to the same phone number in a short time frame."#),
            TwilioProgrammableSmsError::ErrorCode30100 => Some(r#"The Domain SID included in your request is not a valid Domain for your Twilio Organization."#),
            TwilioProgrammableSmsError::ErrorCode30002 => Some(r#"This message failed because your account was suspended after this message was queued but before it was sent by Twilio.
"#),
            TwilioProgrammableSmsError::ErrorCode92007 => None,
            TwilioProgrammableSmsError::ErrorCode21723 => Some(r#"The attempt to import a Campaign Verify token to your brand failed because there is already a token that is queued for importing."#),
            TwilioProgrammableSmsError::ErrorCode30009 => Some(r#"One or more segments associated with your multi-part inbound message was not received."#),
            TwilioProgrammableSmsError::ErrorCode63028 => None,
            TwilioProgrammableSmsError::ErrorCode30116 => Some(r#"Certificates should be uploaded in PEM format. https://en.wikipedia.org/wiki/Privacy-Enhanced_Mail"#),
            TwilioProgrammableSmsError::ErrorCode63001 => Some(r#"The Channel could not authenticate the request. Please see Channel specific error message for more information."#),
            TwilioProgrammableSmsError::ErrorCode90006 => Some(r#"Invalid value for direction. Please contact Customer Support."#),
            TwilioProgrammableSmsError::ErrorCode30122 => Some(r#"Fallback URL is invalid"#),
            TwilioProgrammableSmsError::ErrorCode57018 => Some(r#"Specified 'Event' value type is invalid, Map required"#),
            TwilioProgrammableSmsError::ErrorCode57003 => Some(r#"'Secret id' is invalid for this Partner"#),
            TwilioProgrammableSmsError::ErrorCode30031 => Some(r#"MaxRate is not a valid float"#),
            TwilioProgrammableSmsError::ErrorCode57004 => Some(r#"'Category' is not specified"#),
            TwilioProgrammableSmsError::ErrorCode21614 => Some(r#"You have attempted to send a SMS with a 'To' number that is not 
a valid mobile number. It is likely that the number that you 
have specified is a landline number or is an invalid number. "#),
            TwilioProgrammableSmsError::ErrorCode63023 => None,
            TwilioProgrammableSmsError::ErrorCode30041 => Some(r#"Message sent to a United Kingdom number may not be delivered because it is being sent from a number that must be registered."#),
            TwilioProgrammableSmsError::ErrorCode21709 => Some(r#"The Alpha Sender ID being added to a Messaging Service is invalid or Alpha Sender IDs have not been enabled for this Messaging Service."#),
            TwilioProgrammableSmsError::ErrorCode21658 => Some(r#"Parameter exceeded character limit"#),
            TwilioProgrammableSmsError::ErrorCode21722 => Some(r#"The attempt to import a Campaign Verify token to your brand failed because your token is invalid."#),
            TwilioProgrammableSmsError::ErrorCode63038 => Some(r#"Account exceeded the maximum amount of messages per 24 hours."#),
            TwilioProgrammableSmsError::ErrorCode57002 => Some(r#"Specified 'Secret id' is too long"#),
            TwilioProgrammableSmsError::ErrorCode63020 => None,
            TwilioProgrammableSmsError::ErrorCode63035 => Some(r#"This operation is blocked because the RCS sender has not launched and the recipient has not been invited and accepted the invitation to become a tester. This may also happen if you are attempting to send to a device with a country code from a region that is not supported by that sender."#),
            TwilioProgrammableSmsError::ErrorCode63011 => None,
            TwilioProgrammableSmsError::ErrorCode92004 => None,
            TwilioProgrammableSmsError::ErrorCode30133 => None,
            TwilioProgrammableSmsError::ErrorCode21720 => Some(r#"The A2P use case you are trying to specify in the request is not a valid use case."#),
            TwilioProgrammableSmsError::ErrorCode57011 => Some(r#"Unsupported Partner name"#),
            TwilioProgrammableSmsError::ErrorCode21711 => Some(r#"## Error - 21711

### Phone Number, Shortcode, Destination AlphaSender, and Global AlphaSender is not associated to the specified Messaging Service."#),
            TwilioProgrammableSmsError::ErrorCode30400 => Some(r#"There is an issue with the request body."#),
            TwilioProgrammableSmsError::ErrorCode21605 => Some(r#"## Error - 21605

### Maximum body length is 160 characters

This error is specific to the deprecated /SMS/Messages API endpoint. When using this endpoint, your message body must be 160 characters or less. 

You can send messages up to 1600 characters long by using the [/Messages](https://www.twilio.com/docs/sms/api/message#create-a-message-resource) API endpoint.

For more info, please refer to [our FAQ on message size constraints](https://support.twilio.com/hc/en-us/articles/223133347-Size-limitations-of-combining-text-and-images)."#),
            TwilioProgrammableSmsError::ErrorCode21612 => Some(r#" Message cannot be sent with the current combination of "To" and/or "From" parameters."#),
            TwilioProgrammableSmsError::ErrorCode57017 => Some(r#"Specified 'Topic' is too long"#),
            TwilioProgrammableSmsError::ErrorCode21619 => Some(r#"A Message Body, Media URL or Content SID is required"#),
            TwilioProgrammableSmsError::ErrorCode30007 => Some(r#"Your message was filtered (blocked) by Twilio or by the carrier. This may be done by Twilio for violating Twilio’s [Messaging Policy](https://www.twilio.com/en-us/legal/messaging-policy) or [Acceptable Use Policy](https://www.twilio.com/en-us/legal/aup), or by a wireless carrier for violating carrier rules or regulations.

Examples of messaging that would be blocked by Twilio are spam, phishing, and fraud. Twilio’s filtering system is in place to protect mobile subscribers from spam or other forms of malicious or unwanted messages.

Wireless carriers have filtering systems in place as well. These carrier filters are used to block abusive traffic, as well as to enforce rules or regulations about what types of messaging are allowed to that country or mobile network.

Learn more about how message filtering works and how to avoid it: [How Does Message Filtering Work?](https://support.twilio.com/hc/en-us/articles/223181848-How-Does-Carrier-Filtering-Work-)

For a detailed explanation of carrier filtering in the United States and Canada, please see [SMS Carrier Filtering in the United States and Canada](https://support.twilio.com/hc/en-us/articles/360022449893-SMS-Carrier-Filtering-in-the-United-States-and-Canada)."#),
            TwilioProgrammableSmsError::ErrorCode63006 => None,
            TwilioProgrammableSmsError::ErrorCode63029 => Some(r#"The receiver failed to download the template
"#),
            TwilioProgrammableSmsError::ErrorCode30040 => Some(r#"You sent a message (message SID: SMXXXXX) to a mobile number in a country that requires Alphanumeric Sender ID pre-registration, and the Sender ID you used is not registered to your account."#),
            TwilioProgrammableSmsError::ErrorCode57021 => Some(r#"Invalid Token"#),
            TwilioProgrammableSmsError::ErrorCode35126 => Some(r#"The ScheduleType value provided is not supported for the channel you want to send the message on. "#),
            TwilioProgrammableSmsError::ErrorCode30038 => Some(r#"Twilio has determined that the body of this message contains a One-Time Passcode (OTP). In order to prevent the abuse of Twilio numbers, Twilio does not generally allow receiving OTP messages. The passcode content has been redacted and the message failed."#),
            TwilioProgrammableSmsError::ErrorCode35111 => Some(r#"You must indicate SendAt timestamp if ScheduleType is 'fixed'."#),
            TwilioProgrammableSmsError::ErrorCode30103 => Some(r#"Links not shortened due to shortener application failure"#),
            TwilioProgrammableSmsError::ErrorCode30105 => Some(r#"We received a request to a domain you've registered with Click Tracking but we don't have a record of the shortened url or a fallback url set on the domain."#),
            TwilioProgrammableSmsError::ErrorCode21710 => Some(r#"The phone number, short code or alpha sender ID already exists in the Messaging Service."#),
            TwilioProgrammableSmsError::ErrorCode30404 => Some(r#"## Not Found

The resource was not found. Here are some examples of cases that may trigger a 404 error."#),
            TwilioProgrammableSmsError::ErrorCode57005 => Some(r#"Specified 'Category' is too long"#),
            TwilioProgrammableSmsError::ErrorCode30129 => Some(r#"Certificate is self signed"#),
            TwilioProgrammableSmsError::ErrorCode21730 => Some(r#"This error is returned by Twilio Systems when the system is going through a scheduled maintenance. During this maintenance window some of the operations are not allowed and this error is returned."#),
            TwilioProgrammableSmsError::ErrorCode30127 => Some(r#"MessagingServiceSID is invalid."#),
            TwilioProgrammableSmsError::ErrorCode21902 => Some(r#"InvoiceTag length must be between 0 and 32"#),
            TwilioProgrammableSmsError::ErrorCode30128 => Some(r#"MessagingServiceSidAction is invalid"#),
            TwilioProgrammableSmsError::ErrorCode30037 => Some(r#"Sending messages from this account has been disabled."#),
            TwilioProgrammableSmsError::ErrorCode30032 => Some(r#"You sent a message (message SID: SMXXXXX) from a Toll-Free number to a mobile subscriber in USA or Canada. Twilio requires all Toll-Free numbers to go through a verification process. The Toll-Free number you sent from has not been verified through the required verification process."#),
            TwilioProgrammableSmsError::ErrorCode21910 => Some(r#"`From` and `To` parameters in the API Request should be of the same channel."#),
            TwilioProgrammableSmsError::ErrorCode92009 => Some(r#"The template associated with this SID has already been submitted for approval."#),
            TwilioProgrammableSmsError::ErrorCode57019 => Some(r#"'Authorization' header is missing or is invalid"#),
            TwilioProgrammableSmsError::ErrorCode30125 => Some(r#"We noticed that you have tried to add a number to a messaging service associated with a US A2P 10DLC Sole Proprietor campaign. Please note that all Sole Proprietor campaigns have a max limit of 1 phone number per campaign. Since you have reached this limit previously, your phone number could not be successfully registered for US A2P 10DLC. If you try to send SMS or MMS messages to the US using this phone number, those messages will be considered as unregistered traffic, where higher fees apply.
"#),
            TwilioProgrammableSmsError::ErrorCode63025 => None,
            TwilioProgrammableSmsError::ErrorCode30021 => Some(r#"An internal error has occurred with the messaging service orchestrator that prevented Twilio from processing your response."#),
            TwilioProgrammableSmsError::ErrorCode30485 => Some(r#"Message couldn’t be delivered to the destination number you are trying to reach temporarily."#),
            TwilioProgrammableSmsError::ErrorCode30123 => Some(r#"Callback URL is missing"#),
            TwilioProgrammableSmsError::ErrorCode63022 => None,
            TwilioProgrammableSmsError::ErrorCode30024 => Some(r#"You sent a message (message SID: SMXXXXX) to a mobile number in a country that requires Numeric Sender ID pre-registration and provisioning. The Numeric Sender ID is not currently provisioned with the carrier."#),
            TwilioProgrammableSmsError::ErrorCode30029 => Some(r#"ContentRetention value used in the request is invalid"#),
            TwilioProgrammableSmsError::ErrorCode63009 => None,
            TwilioProgrammableSmsError::ErrorCode30114 => Some(r#"Deactivation list data is not yet available for the date that you specified."#),
            TwilioProgrammableSmsError::ErrorCode30003 => Some(r#"### Unreachable destination handset

The destination handset you are trying to reach is switched off or otherwise unavailable. "#),
            TwilioProgrammableSmsError::ErrorCode63003 => Some(r#"You have tried to send a message to a To address that is inactive or invalid."#),
            TwilioProgrammableSmsError::ErrorCode30131 => Some(r#"A TLS certificate associated with one of your domains will expire soon."#),
            TwilioProgrammableSmsError::ErrorCode35117 => Some(r#"SendAt time must be between 900 seconds and 35 days (3024000 seconds) in the future, inclusive."#),
            TwilioProgrammableSmsError::ErrorCode21655 => None,
            TwilioProgrammableSmsError::ErrorCode30034 => Some(r#"Messages sent to US numbers will not be delivered if they are sent from numbers that are not associated with an approved A2P 10DLC Campaign. This [guide](https://support.twilio.com/hc/en-us/articles/4418081745179-How-do-I-check-that-I-have-completed-US-A2P-10DLC-registration-) will help you determine if you have completed registration for A2P 10DLC. To initiate or continue an A2P 10DLC registration, [visit your console here](https://console.twilio.com/us1/develop/sms/regulatory-compliance/a2p-onboarding). Find out how to register using [this guide](https://support.twilio.com/hc/en-us/articles/1260801864489-How-do-I-register-to-use-A2P-10DLC-messaging-).

For a step-by-step walkthrough, check out [this video on resolving Error 30034](https://twil.io/resolve30034)."#),
            TwilioProgrammableSmsError::ErrorCode63007 => Some(r#"
   "#),
            TwilioProgrammableSmsError::ErrorCode30104 => Some(r#"We received a request to a domain you've registered with Click Tracking but we don't have a record of the shortened url."#),
            TwilioProgrammableSmsError::ErrorCode30020 => Some(r#"An internal error has occurred with Message Scheduling that prevented Twilio from processing your request."#),
            TwilioProgrammableSmsError::ErrorCode30450 => Some(r#"The destination number you are trying to reach is temporarily blocked from receiving this message."#),
            TwilioProgrammableSmsError::ErrorCode30010 => Some(r#"The error code is obsolete. Find more information in the [changelog](https://www.twilio.com/en-us/changelog/end-of-life--eol--of-twilio-messaging-maxprice-api-parameter).


You provided the [MaxPrice parameter](https://support.twilio.com/hc/en-us/articles/360014170533-Using-MaxPrice-with-Twilio-SMS) in your API request. MaxPrice will prevent a message from sending if it exceeds the price you specified.

Please note that SMS pricing is per-segment. SMS messages longer than 160 characters (or longer than 70 characters, if emojis or special characters are used) will require multiple segments. For a detailed explanation of message segmentation and encoding, see [What the Heck is a Segment?](https://www.twilio.com/blog/what-the-heck-is-a-segment.html)."#),
            TwilioProgrammableSmsError::ErrorCode21610 => Some(r#"The person you are trying to message has opted out of receiving messages from your Twilio phone number, Channels sender, or Messaging Service.

You have attempted to message a 'To' number that has replied with "STOP" to one of your previous messages. You will not be able to send to the phone number specified in the 'To' parameter until the subscriber identified by the phone number has responded with "START". 

Please see [this FAQ](https://support.twilio.com/hc/en-us/articles/223134027-Twilio-support-for-opt-out-keywords-SMS-STOP-filtering-) for more information about how Twilio handles opt-out and opt-in."#),
            TwilioProgrammableSmsError::ErrorCode90001 => Some(r#"An invalid message SID has been sent in the request"#),
            TwilioProgrammableSmsError::ErrorCode21717 => Some(r#"The Brand Registration SID you are trying to use to register a new US A2P campaign use case is not registered or the Brand Registration SID is not valid for the given use case."#),
            TwilioProgrammableSmsError::ErrorCode57001 => Some(r#"'Secret id' is not specified"#),
            TwilioProgrammableSmsError::ErrorCode30454 => Some(r#"Account sent too many messages which exceeded the messages limit."#),
            TwilioProgrammableSmsError::ErrorCode92002 => None,
            TwilioProgrammableSmsError::ErrorCode90031 => Some(r#"Broadcast 'Recipients' list is empty"#),
            TwilioProgrammableSmsError::ErrorCode63019 => None,
            TwilioProgrammableSmsError::ErrorCode30026 => Some(r#"Please note that this daily limit is based on the sum of SMS segments and MMS messages you send to T-Mobile subscribers on a 24-hour basis, and is reset daily at midnight PT. When you hit 100% of your daily message limit, you will start to receive error code 30023 for each subsequent message you send to T-Mobile. Those messages will not be delivered, and you must wait until midnight PT to resume message sending.

For more information on T-Mobile daily limit, please refer to [T-Mobile daily message limits for long code messaging with A2P 10DLC](https://support.twilio.com/hc/en-us/articles/1260804800549-T-Mobile-daily-message-limits-for-long-code-messaging-with-A2P-10DLC). For more information on US A2P 10DLC, please refer to [US A2P 10DLC Documentation](https://support.twilio.com/hc/en-us/articles/1260800720410-What-is-A2P-10DLC-).

Please disregard this warning if you have been approved for the unlimited tier via the [T-Mobile Special Business Review process](https://support.twilio.com/hc/en-us/articles/4403550579739-T-Mobile-Special-Business-Review-for-A2P-10DLC).

Please note that if you have multiple providers sending registered SMS or MMS to T-Mobile on your behalf, this warning might not be accurate."#),
            TwilioProgrammableSmsError::ErrorCode30115 => Some(r#"The date format you have used is incorrect."#),
            TwilioProgrammableSmsError::ErrorCode35118 => Some(r#"Message Scheduling is only accessible on Messaging Services. You need to pass a MessageServiceSid instead or along with a From Phone Number in order to schedule a message."#),
            TwilioProgrammableSmsError::ErrorCode30039 => Some(r#"This inbound message appears to be machine generated. Responding to this message programmatically could lead to endless messaging loops. For that reason, Twilio has failed the message with this error code and did not POST to the number’s configured TwiML URLs."#),
            TwilioProgrammableSmsError::ErrorCode21652 => None,
            TwilioProgrammableSmsError::ErrorCode30008 => Some(r#"## Error - 30008

### Message Delivery - Unknown error
"#),
            TwilioProgrammableSmsError::ErrorCode30042 => Some(r#"The Sender ID has been blocked for being generic by Twilio’s service provider, containing non compliant special characters or relating to fraudulent activities."#),
            TwilioProgrammableSmsError::ErrorCode63021 => Some(r#"Message contains content that is rejected or unsupported by the messaging channel."#),
            TwilioProgrammableSmsError::ErrorCode21624 => Some(r#"## Error - 21624

### Invalid validity period value

Validity period can only be set to integers between 1 and 36,000 seconds (10 hours max)."#),
            TwilioProgrammableSmsError::ErrorCode63034 => Some(r#"The specified message media exceeds the maximum size limit. You may re-attempt with media of size 5 MB or less."#),
            TwilioProgrammableSmsError::ErrorCode30126 => Some(r#"Your phone number failed registration/deregistration due to Unknown reasons"#),
            TwilioProgrammableSmsError::ErrorCode57014 => Some(r#"'Event' is not specified"#),
            TwilioProgrammableSmsError::ErrorCode21657 => Some(r#"The Sender ID is Invalid"#),
            TwilioProgrammableSmsError::ErrorCode63018 => None,
            TwilioProgrammableSmsError::ErrorCode92001 => Some(r#"## Error - 92001 

### 'types' Parameter Required

You attempted to create content, but you forgot to include at least one type within the 'types' property the 'types' parameter requires at least one type in order to create content. 
"#),
            TwilioProgrammableSmsError::ErrorCode30001 => Some(r#"Twilio queues messages based on the sending rate of a sender or an account. For example a US long code number can send one message [segment](https://www.twilio.com/blog/what-the-heck-is-a-segment.html) per second, and a short code may send over 100 message segments per second. 

Messages can only be queued for 10 hours and then they automatically fail. This queue length limit can be reduced by setting a lower [Validity Period](https://www.twilio.com/blog/take-more-control-of-outbound-messages-using-validity-period-html) within your Messaging Service settings or in your API requests to send messages.

You can monitor the length on your queue via the [Queue Insights dashboard](https://www.twilio.com/console/sms/insights/queue).

For more information please view this [FAQ](https://support.twilio.com/hc/en-us/articles/115002943027-Understanding-Twilio-Rate-Limits-and-Message-Queues). 
"#),
            TwilioProgrammableSmsError::ErrorCode30120 => Some(r#"Given Domain is not onboarded"#),
            TwilioProgrammableSmsError::ErrorCode30113 => Some(r#"Historical data for Deactivation lists is only available from August 27th, 2020 onwards."#),
            TwilioProgrammableSmsError::ErrorCode30023 => Some(r#"You have sent the maximum allowable daily messages for your Brand to the carrier. 

Daily limits are based on your Brand Trust Score. You can check both your Trust Score and your daily Limit in the console [here](https://console.twilio.com/us1/develop/sms/regulatory-compliance/brands).

Note: That the daily limit applies to the Brand (per EIN) and is based on the total number of outbound SMS segments and MMS messages sent to T-Mobile (including Sprint and MetroPCS) across any messaging platform you have registered your Brand with.

Please refer to our documentation on [T-Mobile daily message limits for long code messaging with A2P 10DLC](https://support.twilio.com/hc/en-us/articles/1260804800549-T-Mobile-daily-message-limits-for-long-code-messaging-with-A2P-10DLC). For more info about A2P 10DLC, refer to our [US A2P 10DLC Documentation](https://support.twilio.com/hc/en-us/articles/1260800720410-What-is-A2P-10DLC-).
"#),
            TwilioProgrammableSmsError::ErrorCode63012 => None,
            TwilioProgrammableSmsError::ErrorCode57022 => Some(r#"Required header is missing or invalid"#),
            TwilioProgrammableSmsError::ErrorCode57008 => Some(r#"Specified 'EventType' format must be String"#),
            TwilioProgrammableSmsError::ErrorCode21729 => Some(r#"The operation you're trying to perform on this campaign is not allowed because the campaign is in a suspended state."#),
            TwilioProgrammableSmsError::ErrorCode21719 => Some(r#"Request was unsuccessful due to incompatibility between the Messaging Service use case and the A2P use case for the messaging service."#),
            TwilioProgrammableSmsError::ErrorCode21724 => Some(r#"Maximum number of brand update requests reached on this brand. Contact support for help."#),
            TwilioProgrammableSmsError::ErrorCode63027 => Some(r#"### Template does not exist for a language and locale"#),
            TwilioProgrammableSmsError::ErrorCode30132 => Some(r#"Please upload a new valid certificate."#),
            TwilioProgrammableSmsError::ErrorCode30410 => Some(r#"The provider used may be experiencing disruptions resulting in errors or request timeouts. Messages are failed and not retried to avoid duplicate message delivery."#),
            TwilioProgrammableSmsError::ErrorCode63005 => Some(r#"### Channel did not accept given content"#),
            TwilioProgrammableSmsError::ErrorCode63013 => Some(r#"

"#),
            TwilioProgrammableSmsError::ErrorCode30017 => Some(r#"A 30017 error is an indicator that the downstream carrier is experiencing performance issues due to high traffic. During times of carrier network congestion, we re-try the request, and return this error if the request is not accepted."#),
            TwilioProgrammableSmsError::ErrorCode30028 => Some(r#"API version used in the request is invalid"#),
            TwilioProgrammableSmsError::ErrorCode21713 => Some(r#"The Messaging Service use case you are trying to specify in the request is not a valid use case. "#),
            TwilioProgrammableSmsError::ErrorCode21708 => Some(r#"## Error - 21708

### Alpha Sender ID Missing from the request "#),
            TwilioProgrammableSmsError::ErrorCode30030 => Some(r#"AddressRetention value used in the request is invalid"#),
            TwilioProgrammableSmsError::ErrorCode30500 => Some(r#"Twilio's platform encountered an internal error while processing this message."#),
            TwilioProgrammableSmsError::ErrorCode21603 => Some(r#"The 'From' parameter in your API request to send an outbound message did not include a valid [Twilio Number](/user/account/phone-numbers/), [Alphanumeric Sender ID](https://support.twilio.com/hc/en-us/articles/223181348-Alphanumeric-Sender-ID-for-Twilio-Programmable-SMS), or [Messaging Service SID](https://www.twilio.com/docs/messaging/services).
	
You can provide any of the above sender types in your From parameter. You may also provide a Messaging Service SID using the dedicated 'MessagingServiceSid' parameter, either instead of or in addition to a specific 'From' parameter."#),
            TwilioProgrammableSmsError::ErrorCode57010 => Some(r#"'PartnerName' is not specified"#),
            TwilioProgrammableSmsError::ErrorCode63030 => Some(r#"### Media messages are not supported with template (structured) messages. Please check your message and try sending a template message without the media
### URL preview messages sent with incorrect URL format"#),
            TwilioProgrammableSmsError::ErrorCode14107 => Some(r#"Too many messages have been sent between two numbers within a short time period, possibly indicating a runaway job or infinite loop. There is a limit of 30 outbound replies between two phone numbers in a 30-second period.

A counter is set for each outbound reply in a conversation (between two numbers). When the first reply occurs, the counter starts at 1. If the next reply is sent less than 30 seconds after the prior reply, then the counter goes to 2. This will continue if each reply is less than 30 seconds, until the counter surpasses 30; at this time, Twilio will set any further messages in the loop to "failed" status for 30 seconds, with this error code."#),
            TwilioProgrammableSmsError::ErrorCode63002 => None,
            TwilioProgrammableSmsError::ErrorCode30109 => Some(r#"The URL specific in your API request for Click Tracking is invalid"#),
            TwilioProgrammableSmsError::ErrorCode21656 => Some(r#"The ContentVariables Parameter is invalid."#),
            TwilioProgrammableSmsError::ErrorCode57012 => Some(r#"Signature invalid"#),
            TwilioProgrammableSmsError::ErrorCode30102 => Some(r#"TLS certificate for the Domain you are using with Twilio Link Shortening has been expired. "#),
            TwilioProgrammableSmsError::ErrorCode63015 => Some(r#"Channel Sandbox can only send messages to phone numbers that have joined the Sandbox"#),
            TwilioProgrammableSmsError::ErrorCode92006 => None,
            TwilioProgrammableSmsError::ErrorCode63032 => Some(r#"User's number is part of an experiment."#),
            TwilioProgrammableSmsError::ErrorCode30005 => Some(r#"## Error - 30005

## Message Delivery - Unknown destination handset

"#),
            TwilioProgrammableSmsError::ErrorCode30110 => Some(r#"The provided URL to resolve the link is on a blocked domain."#),
            TwilioProgrammableSmsError::ErrorCode14109 => Some(r#"## Warning - 14109 

### TwiML Reply message limit exceeded

Too many TwiML reply messages have been sent in response to a single incoming message. You can send a maximum of 10 TwiML replies per incoming message."#),
            TwilioProgrammableSmsError::ErrorCode35114 => Some(r#"SendAt time must be between 900 seconds and 35 days (3024000 seconds) in the future, inclusive."#),
            TwilioProgrammableSmsError::ErrorCode21704 => Some(r#"## Error - 21704

### The Messaging Service contains no senders.

A Messaging Service requires at least one phone number, Alphanumeric Sender ID or short code to send messages.





"#),
            TwilioProgrammableSmsError::ErrorCode21721 => Some(r#"The attempt to import a Campaign Verify token to your brand failed because your A2P brand is in an incompatible state."#),
            TwilioProgrammableSmsError::ErrorCode30033 => Some(r#"Your message could not be sent because you are sending this message from a suspended US A2P 10DLC Campaign."#),
            TwilioProgrammableSmsError::ErrorCode23001 => Some(r#"Phone Number Redaction is enabled for your account. Twilio's default opt-out keyword handling ("SMS STOP filtering") is not compatible with Phone Number Redaction and should be disabled."#),
            TwilioProgrammableSmsError::ErrorCode63033 => Some(r#"The WhatsApp message you are attempting to send uses a "Marketing" category template and the user you are trying to reach has opted out from receiving your marketing messages."#),
            TwilioProgrammableSmsError::ErrorCode30134 => None,
            TwilioProgrammableSmsError::ErrorCode23003 => Some(r#"Phone Number Redaction is enabled for your account. Sticky Sender is not compatible with Phone Number Redaction and should be disabled."#),
            TwilioProgrammableSmsError::ErrorCode63010 => None,
            TwilioProgrammableSmsError::ErrorCode21728 => Some(r#"We could not register your US A2P Campaign because one or more field(s) do not satisfy length requirements."#),
            TwilioProgrammableSmsError::ErrorCode63017 => None,
            TwilioProgrammableSmsError::ErrorCode30035 => Some(r#"We could not send your message because this number you're sending from is still being configured. Twilio has received your request to register or deregister this number under a US A2P campaign, but has not completed the necessary configurations with ecosystem partners (“Number Registration”). You will not be able to send messages on the A2P routes until your number status is changed to “Registered”."#),
            TwilioProgrammableSmsError::ErrorCode92003 => None,
            TwilioProgrammableSmsError::ErrorCode21727 => Some(r#"We could not register your US A2P Campaign because one or more required parameter(s) are missing"#),
            TwilioProgrammableSmsError::ErrorCode63026 => None,
            TwilioProgrammableSmsError::ErrorCode23006 => Some(r#"Message Redaction is enabled for your account. Using GET requests to handle incoming messages is not compatible with either Message Body Redaction or Phone Number Redaction."#),
            TwilioProgrammableSmsError::ErrorCode21714 => Some(r#"You have attempted to add a Twilio phone number, alpha sender, or short code to your Messaging Service Number Pool, but you are already at the limit of phone numbers or alpha senders. By default, a Messaging Service can contain up to a total of 400 Twilio phone numbers and short codes and only one Alpha Sender per country.
"#),
            TwilioProgrammableSmsError::ErrorCode30004 => Some(r#"## Error - 30004

## Message Delivery - Message blocked"#),
            TwilioProgrammableSmsError::ErrorCode63037 => Some(r#"Media failed to upload to provider"#),
            TwilioProgrammableSmsError::ErrorCode21715 => Some(r#"In order for a phone number to work in a Messaging Service it must have at least SMS or MMS capabilities. If it does not have either, it will fail."#),
            TwilioProgrammableSmsError::ErrorCode63039 => None,
            TwilioProgrammableSmsError::ErrorCode30453 => Some(r#"Message couldn’t be delivered to the destination number you are trying to reach temporarily."#),
            TwilioProgrammableSmsError::ErrorCode30106 => Some(r#"Domain has not been setup for click tracking"#),
            TwilioProgrammableSmsError::ErrorCode21726 => Some(r#"Starter brands cannot be created or updated at this time. Contact support for help."#),
            TwilioProgrammableSmsError::ErrorCode35115 => Some(r#"SendAt time must be between 900 seconds and 35 days (3024000 seconds) in the future, inclusive."#),
            TwilioProgrammableSmsError::ErrorCode30018 => Some(r#"You sent a message (message SID: SMXXXXX) to a mobile number in a country that requires Alphanumeric Sender ID pre-registration. Our records indicate that you do not have a registered Alphanumeric Sender ID for this country. This can result in lower delivery quality.

If you are sending transactional SMS (like OTP/auth codes, account-related alerts) you should pre-register for the best possible delivery quality."#),
            TwilioProgrammableSmsError::ErrorCode30101 => Some(r#"Domain need to be verified under your Organization through the Admin Center in order use it with Twilio Link Shortening."#),
            TwilioProgrammableSmsError::ErrorCode30025 => Some(r#"Please note that this daily limit is based on the sum of SMS segments and MMS messages you send to T-Mobile subscribers on a 24-hour basis, and is reset daily at midnight PT. When you hit 100% of your daily message limit, you will start to receive error code 30023 for each subsequent message you send to T-Mobile. Those messages will not be delivered, and you must wait until midnight PT to resume message sending.

For more information on T-Mobile daily limit, please refer to [T-Mobile daily message limits for long code messaging with A2P 10DLC](https://support.twilio.com/hc/en-us/articles/1260804800549-T-Mobile-daily-message-limits-for-long-code-messaging-with-A2P-10DLC). For more information on US A2P 10DLC, please refer to [US A2P 10DLC Documentation](https://support.twilio.com/hc/en-us/articles/1260800720410-What-is-A2P-10DLC-).

Please disregard this warning if you have been approved for the unlimited tier via the [T-Mobile Special Business Review process](https://support.twilio.com/hc/en-us/articles/4403550579739-T-Mobile-Special-Business-Review-for-A2P-10DLC).

Please note that if you have multiple providers sending registered SMS or MMS to T-Mobile on your behalf, this warning might not be accurate."#),
            TwilioProgrammableSmsError::ErrorCode63014 => None
        }
    }
}
