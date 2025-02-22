// AUTO-GENERATED. DO NOT MODIFY. YOUR CHANGES WILL BE OVER-WRITTEN
use crate::products::TwilioProgrammableSmsError;
use standard_error::traits::StandardErrorCausesTrait;

impl StandardErrorCausesTrait for TwilioProgrammableSmsError {
    fn causes(&self) -> Option<&'static str> {
        match self {
            TwilioProgrammableSmsError::ErrorCode30111 => Some(r#"- Fraudulent activity has been detected on this url
- Domain owner has requested Twilio not link to their site"#),
            TwilioProgrammableSmsError::ErrorCode57016 => Some(r#"'Topic' is not specified"#),
            TwilioProgrammableSmsError::ErrorCode30118 => Some(r#"- Data in private key may be corrupted
- Private key info could incorrectly formatted
- Private key data could be missing"#),
            TwilioProgrammableSmsError::ErrorCode21725 => Some(r#"Invalid Brand status"#),
            TwilioProgrammableSmsError::ErrorCode30006 => Some(r#"*  The destination number is unable to receive this message. Potential reasons
could include trying to reach a landline or, in the case of short codes, an
unreachable carrier.
* Your message was sent to a landline, or an unreachable carrier for this phone number type."#),
            TwilioProgrammableSmsError::ErrorCode30117 => Some(r#"- Data in Certificate may be corrupted
- Certificate info could incorrectly formatted
- Certificate data could be missing"#),
            TwilioProgrammableSmsError::ErrorCode30107 => Some(r#"Setup and configuration wasn't completed for this domain"#),
            TwilioProgrammableSmsError::ErrorCode57013 => Some(r#"'Topic' is not specified"#),
            TwilioProgrammableSmsError::ErrorCode57020 => Some(r#"Authorization failed"#),
            TwilioProgrammableSmsError::ErrorCode30019 => Some(r#"Content size exceeded carrier limit could be caused by to many characters, or bytes in the message. UCS-2 encoded messages could have different limitations than GSM encoded messages. 

"#),
            TwilioProgrammableSmsError::ErrorCode30043 => Some(r#"The mobile operator flagged the SMS as an international message sent through the domestic gateway."#),
            TwilioProgrammableSmsError::ErrorCode90009 => Some(r#"* Message with the same SID already exists. Conflict when creating a new message."#),
            TwilioProgrammableSmsError::ErrorCode92008 => Some(r#"You attempted to create template of type twilio/button which is no longer supported. "#),
            TwilioProgrammableSmsError::ErrorCode90007 => Some(r#"Invalid validity period value"#),
            TwilioProgrammableSmsError::ErrorCode30124 => Some(r#"User did not include messagingServiceSid field"#),
            TwilioProgrammableSmsError::ErrorCode63016 => Some(r#"* This message failed to be delivered to the user because it was sent outside the messaging channel's allowed conversation window. For WhatsApp messages initiated by the business, you must use a pre-defined template. 
* If the message was created with Content API or Content Editor it may be one of the following 3 issues: 
 * The customer has not included a messaging service SID
 * The customer is still using "body" instead of "Content SID" and "Content Variables"
 * The customer is using content type that is not supported for business initiated conversations, e.g. listpicker or location
* For Facebook Messenger messages initiated by the business, only publishers registered with the Facebook News Page Index (NPI) are allowed to send notification messages outside the 24-hour window."#),
            TwilioProgrammableSmsError::ErrorCode21654 => Some(r#"You attempted to send a message with the ContentVariables parameter but without ContentSid."#),
            TwilioProgrammableSmsError::ErrorCode30409 => Some(r#"Attempted to cancel the message after it has reached a [finalized state](https://support.twilio.com/hc/en-us/articles/223134347-What-are-the-Possible-SMS-and-MMS-Message-Statuses-and-What-do-They-Mean-#:~:text=Finalized%20Message%20Delivery%20Status&text=Twilio%20has%20not%20received%20updated%20delivery%20information%20for%20your%20message.&text=Twilio%20has%20received%20a%20delivery,of%20the%20destination%20handset%2C%20etc.)."#),
            TwilioProgrammableSmsError::ErrorCode30119 => Some(r#"- Certificate and key are for differing domains"#),
            TwilioProgrammableSmsError::ErrorCode21606 => Some(r#"* The number you are using may not be capable of sending messages.
* The number may be formatted incorrectly. Twilio accepts numbers in [E.164 format](https://www.twilio.com/docs/glossary/what-e164).
* If the number is a Short Code, it must be associated with the same country as the destination address.
* The number you are using is in the process of porting/hosting.
* The number you are using does not belong to the account whose credentials are present in the API request.
"#),
            TwilioProgrammableSmsError::ErrorCode63031 => Some(r#"Incorrect `To` and `From` number in your API Request"#),
            TwilioProgrammableSmsError::ErrorCode23004 => Some(r#"* You have Advanced Opt-Out enabled on your Messaging Service. Advanced Opt-Out requires Twilio to save non-redacted phone numbers of customers who have opted out, and is incompatible with Phone Number Redaction."#),
            TwilioProgrammableSmsError::ErrorCode11751 => Some(r#"The total size of the media message request exceeds the maximum size limit for the channel."#),
            TwilioProgrammableSmsError::ErrorCode30036 => Some(r#"The message has spent more than the specified amount of time in Twilio's system. This can be due to a long time spent in the queue due to a combination of your [messaging rate limits](https://support.twilio.com/hc/en-us/articles/115002943027-Understanding-Twilio-Rate-Limits-and-Message-Queues), the number of messages submitted in rapid succession, and the amount of time you have allotted via the [ValidityPeriod setting](https://www.twilio.com/blog/validity-period-promotional-verification-use-cases) in your messaging service or on the message itself."#),
            TwilioProgrammableSmsError::ErrorCode30121 => Some(r#"User did not include fallbackUrl field"#),
            TwilioProgrammableSmsError::ErrorCode21611 => Some(r#"You have attempted to enqueue more than 10 hours' worth of messages on a single Twilio phone number or sender (based on that number's [MPS rate limit](https://support.twilio.com/hc/en-us/articles/115002943027-Understanding-Twilio-Rate-Limits-and-Message-Queues))."#),
            TwilioProgrammableSmsError::ErrorCode30027 => Some(r#"* You have consumed 100% of your T-Mobile daily message limit for the day."#),
            TwilioProgrammableSmsError::ErrorCode63008 => None,
            TwilioProgrammableSmsError::ErrorCode23002 => Some(r#"* Twilio is currently set to handle opt-out keywords for short code numbers on your account. Twilio's opt-out handling requires Twilio to save non-redacted phone numbers of users who have opted out, and is incompatible with Phone Number Redaction."#),
            TwilioProgrammableSmsError::ErrorCode30011 => Some(r#"If you received this error code on an inbound message, there was a failure attempting to receive an incoming MMS, as your Twilio phone number was sent an MMS in a region where Twilio does not support incoming MMS.

If you received this error code on an outbound message, the destination handset either doesn't support MMS, or MMS is not currently enabled on the device."#),
            TwilioProgrammableSmsError::ErrorCode30130 => Some(r#"One of the messaging service SID in the request body is already associated with a domain in the account"#),
            TwilioProgrammableSmsError::ErrorCode21627 => Some(r#"Max Price has a zero or negative value or not a float"#),
            TwilioProgrammableSmsError::ErrorCode23005 => Some(r#"* You have Fallback to Long Code enabled on your Messaging Service. Fallback to Long Code requires Twilio to retain non-redacted phone numbers of recipients, and is incompatible with Phone Number Redaction."#),
            TwilioProgrammableSmsError::ErrorCode90014 => Some(r#"Validity Period is not positive integer"#),
            TwilioProgrammableSmsError::ErrorCode92005 => Some(r#"You attempted to send a message with the ContentVariables parameter but without ContentSid"#),
            TwilioProgrammableSmsError::ErrorCode21408 => Some(r#"* You have attempted to send an SMS or MMS to a region that has not been enabled in your account's [Geo-Permissions](https://console.twilio.com/us1/develop/sms/settings/geo-permissions) settings. "#),
            TwilioProgrammableSmsError::ErrorCode35125 => Some(r#"The account_sid already has at least 500,000 messages scheduled."#),
            TwilioProgrammableSmsError::ErrorCode21712 => None,
            TwilioProgrammableSmsError::ErrorCode57006 => Some(r#"'EventType' is not specified"#),
            TwilioProgrammableSmsError::ErrorCode57007 => Some(r#"'EventType' is not being passed in request"#),
            TwilioProgrammableSmsError::ErrorCode30108 => Some(r#"Necessary setup steps for Click Tracking, including creation of an organization have not been completed."#),
            TwilioProgrammableSmsError::ErrorCode57009 => Some(r#"Specified 'EventType' is too long"#),
            TwilioProgrammableSmsError::ErrorCode30022 => Some(r#"* You are sending messages at a rate higher than allowed by the carriers for your campaign type.
* You might be sending too many messages to the same phone number in a short timeframe."#),
            TwilioProgrammableSmsError::ErrorCode30100 => Some(r#"The Domain SID you're using could belong to another Organization. It could also potentially be an issue of copy and pasting. You may be providing a URL instead of a Domain SID in the Domain SID field."#),
            TwilioProgrammableSmsError::ErrorCode30002 => Some(r#"*  Account was suspended for lack of funds after your message was queued but before it was sent by Twilio.
* Account was suspended because Twilio detected a violation of our Acceptable Use Policy. 
"#),
            TwilioProgrammableSmsError::ErrorCode92007 => Some(r#"The Content Variables Parameter is not a JSON String "#),
            TwilioProgrammableSmsError::ErrorCode21723 => Some(r#"Another Campaign Verify token has a status of `IN_PROGRESS` meaning it is already queued for import. Only one token can be imported at a time. "#),
            TwilioProgrammableSmsError::ErrorCode30009 => Some(r#"* The mobile network or carrier did not successfully send the entire message to Twilio
* The mobile handset had a malfunction and did not successfully send all segments
* The mobile user was in poor network coverage, causing an incomplete message to be sent"#),
            TwilioProgrammableSmsError::ErrorCode63028 => None,
            TwilioProgrammableSmsError::ErrorCode30116 => Some(r#"- Certificate is using a different format
- Certificate has been corrupted"#),
            TwilioProgrammableSmsError::ErrorCode63001 => Some(r#"*   The Channel could not authenticate the request. Please see Channel specific error message for more information.
*   Facebook identified a potential security issue and rotated the token
*   The Facebook user who connected the FB page to Twilio has changed their password
*   The user de-authorized the Twilio app"#),
            TwilioProgrammableSmsError::ErrorCode90006 => Some(r#"We're having trouble completing your request"#),
            TwilioProgrammableSmsError::ErrorCode30122 => Some(r#"The fallbackUrl field has incorrectly formatted URL"#),
            TwilioProgrammableSmsError::ErrorCode57018 => Some(r#"Specified "Event" value must be Map"#),
            TwilioProgrammableSmsError::ErrorCode57003 => Some(r#"'Secret id' is invalid for this Partner"#),
            TwilioProgrammableSmsError::ErrorCode30031 => Some(r#"MaxRate value is not a float or is equal to or less than zero"#),
            TwilioProgrammableSmsError::ErrorCode57004 => Some(r#"'Category' is not specified"#),
            TwilioProgrammableSmsError::ErrorCode21614 => Some(r#"* The number you provided may be a landline number.
* The number you provided may be invalid or formatted incorrectly.
* If you are attempting to send SMS to Internet of Things (IoT) or machine-to-machine (M2M) numbers, the numbers may use a non-standard format that Twilio has not added to our number validation system yet."#),
            TwilioProgrammableSmsError::ErrorCode63023 => None,
            TwilioProgrammableSmsError::ErrorCode30041 => Some(r#"Alphanumeric, Shortcode, Twilio Domestic Longcode must complete and return a Letter of Authorization (LOA) to Twilio in order to continue using Protected Sender IDs to send messages to the United Kingdom. [More details on UK SMS Guidelines](https://www.twilio.com/en-us/guidelines/gb/sms)."#),
            TwilioProgrammableSmsError::ErrorCode21709 => Some(r#"* The Messaging Service is not enabled to support Alpha Sender IDs. 
* The Alpha Sender ID being added to a Messaging Service is invalid. Alphanumeric Sender IDs may be up to 11 characters. They may not be only numbers. Refer to [this page](https://support.twilio.com/hc/en-us/articles/223133867-Using-Alphanumeric-Sender-ID-with-Messaging-Services) for acceptable characters."#),
            TwilioProgrammableSmsError::ErrorCode21658 => Some(r#"Ensure parameter doesn't exceed character limit during content creation"#),
            TwilioProgrammableSmsError::ErrorCode21722 => Some(r#"- The token is not formatted correctly"#),
            TwilioProgrammableSmsError::ErrorCode63038 => Some(r#"Account has sent too many messages."#),
            TwilioProgrammableSmsError::ErrorCode57002 => Some(r#"Specified 'Secret id' is too long."#),
            TwilioProgrammableSmsError::ErrorCode63020 => None,
            TwilioProgrammableSmsError::ErrorCode63035 => Some(r#"Device is not RCS capable, recipient has not accepted to become a tester, or device is not reachable from the RCS sender."#),
            TwilioProgrammableSmsError::ErrorCode63011 => None,
            TwilioProgrammableSmsError::ErrorCode92004 => Some(r#"You attempted to create content, but you sent an invalid 'language code'"#),
            TwilioProgrammableSmsError::ErrorCode30133 => Some(r#"This might happen if an associated domain resource has been deleted and recreated since the last certificate upload."#),
            TwilioProgrammableSmsError::ErrorCode21720 => Some(r#"The A2P use case specified in the request is not a valid A2P use case."#),
            TwilioProgrammableSmsError::ErrorCode57011 => Some(r#"Specified 'PartnerName' is not supported"#),
            TwilioProgrammableSmsError::ErrorCode21711 => Some(r#"* The sender was never associated with a given Messaging Service. "#),
            TwilioProgrammableSmsError::ErrorCode30400 => Some(r#"- POST body is missing parameters Body or Status.
- Attempted to cancel the message and redact the message body at the same time.
- Value for Status is invalid.
- Message body redaction request does not have empty Body parameter."#),
            TwilioProgrammableSmsError::ErrorCode21605 => Some(r#"The SMS body contains too many characters."#),
            TwilioProgrammableSmsError::ErrorCode21612 => Some(r#" **Sender ID restrictions in the destination country:** Many countries limit which numbers, short-codes, and/or alphanumeric senders can be used in that region. Consult the [SMS guidelines for the destination region](https://www.twilio.com/en-us/guidelines/sms).

**Sender ID restrictions on the destination network:** Some networks do not support receiving messages from shortcodes, others might allow SMS from shortcodes but cannot receive MMS messages from shortcodes. Similar restrictions may apply to the use of longcodes from countries that do not match the destination country. Consult the [SMS guidelines for the destination region](https://www.twilio.com/en-us/guidelines/sms).

**Alphanumeric senderIDs:** If you are using an alphanumeric sender ID, the 'To' number must be in a country where alphanumeric sender IDs are supported. Certain countries require pre-registration of alphanumeric sender IDs. A list of countries where alphanumeric sender ID is supported and whether or not pre-registration is required can be found [here](https://support.twilio.com/hc/en-us/articles/223133767-International-support-for-Alphanumeric-Sender-ID).

**Number formatting:** The format you used for the “To” or “From” number may not be formatted using the [E.164 format](https://www.twilio.com/docs/glossary/what-e164). Twilio standardizes numbers using the E.164 format. If the “To” or “From” number of this message were altered by Twilio in a way that you didn’t expect, try re-submitting the message with E.164 formatted addresses.

**Destinations where Twilio does not have connectivity:**  It is possible that Twilio does not yet have service with the carrier you are trying to reach. You can lookup the destination network via [the lookup api](https://www.twilio.com/docs/lookup/v2-api)."#),
            TwilioProgrammableSmsError::ErrorCode57017 => Some(r#"Specified 'Topic' is too long"#),
            TwilioProgrammableSmsError::ErrorCode21619 => Some(r#"You are sending a message using a Messaging Service without specifying a `Body`,`Media URL` or `Content SID`."#),
            TwilioProgrammableSmsError::ErrorCode30007 => Some(r#"* Your message was identified as spam or unwanted messaging by Twilio's message filtering system
* Your message was flagged as objectionable and blocked by a wireless carrier"#),
            TwilioProgrammableSmsError::ErrorCode63006 => None,
            TwilioProgrammableSmsError::ErrorCode63029 => Some(r#"The receiver failed to download the template"#),
            TwilioProgrammableSmsError::ErrorCode30040 => Some(r#"* The recipient you are sending a message to is on a network that requires Alphanumeric Sender ID pre-registration, and our records indicate that your Sender ID is not registered.
* You may have previously received a WARNING 30018 “Destination carrier requires Sender ID pre-registration” code. This is the corresponding ERROR version of that code when a block is indeed happening."#),
            TwilioProgrammableSmsError::ErrorCode57021 => Some(r#"Invalid Token."#),
            TwilioProgrammableSmsError::ErrorCode35126 => Some(r#"You provided a value other than ‘fixed’ in the ScheduleType parameter"#),
            TwilioProgrammableSmsError::ErrorCode30038 => Some(r#"You are receiving inbound OTP messages."#),
            TwilioProgrammableSmsError::ErrorCode35111 => Some(r#"You did not indicate SendAt timestamp."#),
            TwilioProgrammableSmsError::ErrorCode30103 => Some(r#"Unknown service interruption or outage. "#),
            TwilioProgrammableSmsError::ErrorCode30105 => Some(r#"- The url has expired
- A request was sent to a url that wasn't generated by Twilio
- The fallback url isn't set"#),
            TwilioProgrammableSmsError::ErrorCode21710 => Some(r#"* Multiple requests were made to add the phone number, short code or alpha sender ID to the Messaging Service’s Sender Pool. 
* The phone number, short code or alpha sender ID was already added at an earlier time."#),
            TwilioProgrammableSmsError::ErrorCode30404 => Some(r#" Using a base URL that is not `https://api.twilio.com`. For example, making requests to `https://twilio.com` or `https://www.twilio.com` will not work."#),
            TwilioProgrammableSmsError::ErrorCode57005 => Some(r#"Specified 'Category' is too long"#),
            TwilioProgrammableSmsError::ErrorCode30129 => Some(r#"Using self signed certificates"#),
            TwilioProgrammableSmsError::ErrorCode21730 => Some(r#"Currently the system is going through some maintenance."#),
            TwilioProgrammableSmsError::ErrorCode30127 => Some(r#"User included an invalid MessagingServiceSID."#),
            TwilioProgrammableSmsError::ErrorCode21902 => Some(r#"InvoiceTag is too long"#),
            TwilioProgrammableSmsError::ErrorCode30128 => Some(r#"Only ADD, DELETE or REPLACE action types are allowed"#),
            TwilioProgrammableSmsError::ErrorCode30037 => Some(r#"This account is configured to not send outbound messages."#),
            TwilioProgrammableSmsError::ErrorCode30032 => Some(r#"*Your toll-free number has not been Verified to allow sending of traffic in USA and Canada.  

*Your toll-free verification submission was rejected. 
"#),
            TwilioProgrammableSmsError::ErrorCode21910 => Some(r#"You are trying to send a message between different channels, eg. from an SMS capable number to a WhatsApp number."#),
            TwilioProgrammableSmsError::ErrorCode92009 => Some(r#"This template sid has already been submitted for approval."#),
            TwilioProgrammableSmsError::ErrorCode57019 => Some(r#"'Authorization' header is missing or is invalid"#),
            TwilioProgrammableSmsError::ErrorCode30125 => Some(r#"You already have 1 phone number registered with your Sole Proprietor campaign."#),
            TwilioProgrammableSmsError::ErrorCode63025 => None,
            TwilioProgrammableSmsError::ErrorCode30021 => Some(r#"* We screwed up. Sorry!"#),
            TwilioProgrammableSmsError::ErrorCode30485 => Some(r#"* This is because Twilio has identified potential fraudulent messages being sent to the destination you are trying to reach.
* Hence SMS traffic can not be delivered to the destination phone number used for the next couple of hours. The traffic should return to normalcy post that. However, the SMS traffic could be undelivered for an even longer period if fraudulent activity doesn't subside & continues to go on."#),
            TwilioProgrammableSmsError::ErrorCode30123 => Some(r#"User did not include callbackUrl field"#),
            TwilioProgrammableSmsError::ErrorCode63022 => None,
            TwilioProgrammableSmsError::ErrorCode30024 => Some(r#"* The Numeric Sender ID has not been provisioned by the carrier yet. Typically in this situation you would see >90% failures towards the carrier.
* If this is for US A2P 10DLC and you just registered this number, it could take a brief period of time for all carriers to provision the number to your campaign. Typically you would see all messages towards a single carrier fail, if the code isn't provisioned.
* Your shortcode has not been fully provisioned for production traffic, or the carrier has disabled your shortcode due to cancellation or compliance violations. Typically in this situation you would see >90% failures towards the carrier.
* The destination number has recently ported to a new carrier and the port hasn't completed 100%.
* Your toll-free number has not been fully provisioned for production traffic. 
*The destination number is not provisioned to receive SMS messages.

"#),
            TwilioProgrammableSmsError::ErrorCode30029 => Some(r#"ContentRetention is not either 'retain', 'discard' or 'debug' or Message Privacy features are not enabled for this account"#),
            TwilioProgrammableSmsError::ErrorCode63009 => None,
            TwilioProgrammableSmsError::ErrorCode30114 => Some(r#"- You have specified a date in the future
- You have specified today and Twilio has yet to process data for today yet"#),
            TwilioProgrammableSmsError::ErrorCode30003 => Some(r#"*  The destination handset you are trying to reach is switched off or otherwise unavailable.
*  The device you are trying to reach does not have sufficient signal
*  The device cannot receive SMS (for example, the phone number belongs to a landline)
*  There is an issue with the mobile carrier"#),
            TwilioProgrammableSmsError::ErrorCode63003 => Some(r#"* The To address is no invalid or longer belongs to a mobile user
* The To address was entered or formatted incorrectly"#),
            TwilioProgrammableSmsError::ErrorCode30131 => Some(r#"A TLS certificate associated with one of your domains has not been renewed in our system, and the existing certificate will expire soon."#),
            TwilioProgrammableSmsError::ErrorCode35117 => Some(r#"Scheduler currently doesn't support scheduling a message at a fixed time less that 15 minutes from now, or more than 35 days in the future."#),
            TwilioProgrammableSmsError::ErrorCode21655 => Some(r#"The ContentSid you are using is not valid"#),
            TwilioProgrammableSmsError::ErrorCode30034 => Some(r#"You are sending messages to the US using a US 10DLC number that is not associated with an approved A2P 10DLC Campaign."#),
            TwilioProgrammableSmsError::ErrorCode63007 => Some(r#"* Sending a message using a `From` address that is not associated with any Channel or WhatsApp installation.
* Sending a message using a `From` address for a Channel that is pending approval.
* Using incorrect Account Credentials for the account that owns the Sender
"#),
            TwilioProgrammableSmsError::ErrorCode30104 => Some(r#"- The url has expired
- A request was sent to a url that wasn't generated by Twilio"#),
            TwilioProgrammableSmsError::ErrorCode30020 => Some(r#"* We screwed up. Sorry!"#),
            TwilioProgrammableSmsError::ErrorCode30450 => Some(r#"This is because the SMS Pumping Protection feature has identified potential fraudulent messages being sent to the destination you are trying to reach.

A temporary block on SMS traffic has been placed for the next 15-30 mins on the phone number you used. This block will be lifted at the end of this 15-30 minutes period. If further fraudulent activity is detected after this block, this pattern will continue with temporary 15-30 minutes blocks until the issue is no longer observed."#),
            TwilioProgrammableSmsError::ErrorCode30010 => Some(r#"* The message price exceeded the MaxPrice you specified in your API request."#),
            TwilioProgrammableSmsError::ErrorCode21610 => Some(r#"* The end user handset has responded with "STOP" or [another opt-out keyword](https://support.twilio.com/hc/en-us/articles/223134027-Twilio-support-for-opt-out-keywords-SMS-STOP-filtering-).

* The phone number has been reassigned to a different end user recently."#),
            TwilioProgrammableSmsError::ErrorCode90001 => Some(r#"Message SID sent in the request has either wrong prefix or incorrect format"#),
            TwilioProgrammableSmsError::ErrorCode21717 => Some(r#"* The Brand Registration SID passed in the request is not registered.
* The Brand Registration SID passed in the request is not valid for the given use case. Only brands with a STARTER brand_type can create STARTER campaign use cases."#),
            TwilioProgrammableSmsError::ErrorCode57001 => Some(r#"'Secret id' is not specified"#),
            TwilioProgrammableSmsError::ErrorCode30454 => Some(r#"Account exceeded the trial account messages limit."#),
            TwilioProgrammableSmsError::ErrorCode92002 => Some(r#"You attempted to create content, but you send more than '100' variables"#),
            TwilioProgrammableSmsError::ErrorCode90031 => Some(r#"Broadcast 'Recipients' list is empty"#),
            TwilioProgrammableSmsError::ErrorCode63019 => Some(r#"* Twilio failed to download the media from the sender. 
* Attempt to retrieve the media returned 0 bytes. "#),
            TwilioProgrammableSmsError::ErrorCode30026 => Some(r#"* You have consumed 70% of your T-Mobile daily message limit for the day."#),
            TwilioProgrammableSmsError::ErrorCode30115 => Some(r#"- You may have included a timestamp in your date
- You may have abbreviated YYYY to YY"#),
            TwilioProgrammableSmsError::ErrorCode35118 => Some(r#"A MessagingServiceSid was not provided as part of the MessagingServiceSid or From parameter."#),
            TwilioProgrammableSmsError::ErrorCode30039 => Some(r#"You are receiving a message that appears to be machine generated."#),
            TwilioProgrammableSmsError::ErrorCode21652 => None,
            TwilioProgrammableSmsError::ErrorCode30008 => Some(r#"If a message you sent is not delivered to the end device and returns a 30008 error code, this means that delivery of your message failed for unknown reasons.

When Twilio receives a very generic error from our carrier partner that we have no further details about, we associate the message with the error code 30008, letting you know that Twilio truly doesn’t know what caused this error from the provider."#),
            TwilioProgrammableSmsError::ErrorCode30042 => Some(r#"You are using a Sender ID that is not compliant with SMS guidelines"#),
            TwilioProgrammableSmsError::ErrorCode63021 => Some(r#"* Unable to upload the media used in the message because it is not supported by the messaging channel.
* Message contains components not supported in selected messaging channel
* Length of the parameters and the template text exceeds the allowed length of 1024 for WhatsApp."#),
            TwilioProgrammableSmsError::ErrorCode21624 => Some(r#"A custom ValidityPeriod has been set that is higher than 36000 or lower than 1"#),
            TwilioProgrammableSmsError::ErrorCode63034 => Some(r#"The specified message media exceeds the maximum size limit."#),
            TwilioProgrammableSmsError::ErrorCode30126 => Some(r#"Unknown"#),
            TwilioProgrammableSmsError::ErrorCode57014 => Some(r#"'Event' is not specified"#),
            TwilioProgrammableSmsError::ErrorCode21657 => Some(r#"You are using a Sender ID that is not supported in the specific destination"#),
            TwilioProgrammableSmsError::ErrorCode63018 => Some(r#"* Your account is sending messages at a higher combined rate than configured for your account.
* Your WhatsApp Sender has exceeded the messaging limit imposed by WhatsApp. For more information, see [WhatsApp Sender Messaging Limits & Quality Rating](https://support.twilio.com/hc/en-us/articles/360024008153-WhatsApp-Sender-Message-Limits-and-Quality-Rating)"#),
            TwilioProgrammableSmsError::ErrorCode92001 => Some(r#"You attempted to create content, but you forgot to include at least one type within the 'types' property, the 'types' parameter requires at least one type in order to create content. "#),
            TwilioProgrammableSmsError::ErrorCode30001 => Some(r#"* You tried to send too many messages too quickly and your message queue overflowed.
* You tried sending all your messages from a single Sender with an insufficient send rate limit (Message Segments per Second) for that volume, and you need additional throughput to handle the volume.
* There are Messages Per Second (MPS) limits applied for your Senders, Messaging Services, and Accounts that you may be encountering."#),
            TwilioProgrammableSmsError::ErrorCode30120 => Some(r#"User has not uploaded cert for domain"#),
            TwilioProgrammableSmsError::ErrorCode30113 => Some(r#"You have specified a date that is too far in the past for Twilio to have data available."#),
            TwilioProgrammableSmsError::ErrorCode30023 => Some(r#"* You have sent the maximum amount of message [segments](https://www.twilio.com/docs/glossary/what-is-gsm-7-character-encoding#:~:text=Additionally%2C%20for%20long,characters%20per%20segment) for the day on a given Brand.
* You have sent the maximum amount of message [segments](https://www.twilio.com/docs/glossary/what-is-gsm-7-character-encoding#:~:text=Additionally%2C%20for%20long,characters%20per%20segment) for the day across multiple Brands associated with your EIN, which could include Brands associated with other Twilio accounts or providers. 
"#),
            TwilioProgrammableSmsError::ErrorCode63012 => None,
            TwilioProgrammableSmsError::ErrorCode57022 => Some(r#"Mandatory headers might be missing in the request."#),
            TwilioProgrammableSmsError::ErrorCode57008 => Some(r#"Specified 'EventType' format is invalid, String required"#),
            TwilioProgrammableSmsError::ErrorCode21729 => Some(r#"Campaign is suspended."#),
            TwilioProgrammableSmsError::ErrorCode21719 => Some(r#"* Updating a Messaging Service use case to one that is not compatible with the current A2P use case for this messaging service.
* Creating a new A2P campaign with an A2P use case that is not compatible with the current Messaging Service use case."#),
            TwilioProgrammableSmsError::ErrorCode21724 => Some(r#"Too many requests were made to update a Brand"#),
            TwilioProgrammableSmsError::ErrorCode63027 => Some(r#"* Template does not exist for a language and locale
* Template is being sent using the wrong parameters. Content API templates must be sent with a Content Template sid starting with HX and include a Messaging Service. Templates created using the Legacy System through Templates API or the WA Templates Console UI must be sent with a body string.
* Their requested body does not match an approved template."#),
            TwilioProgrammableSmsError::ErrorCode30132 => Some(r#"Invalid certificate format has been uploaded."#),
            TwilioProgrammableSmsError::ErrorCode30410 => Some(r#"Provider side disruptions."#),
            TwilioProgrammableSmsError::ErrorCode63005 => Some(r#"*   The content you are trying to send was rejected by the Channel. Please see Channel specific error message for more information. 
*   When sending messages to an end user on WhatsApp, this error may occur when the end user is not a valid WhatsApp user.
*   When sending messages with media to an end user on WhatsApp, this error may occur when WhatsApp detects an issue with the media file sent."#),
            TwilioProgrammableSmsError::ErrorCode63013 => Some(r#"This message send failed because it violates Channel provider's policy. Please see Channel specific error message for more information. 
*  This error may also be seen if more than 4 consecutive whitespaces replace a placeholder in a WhatsApp message body."#),
            TwilioProgrammableSmsError::ErrorCode30017 => Some(r#"* Carrier is experiencing congestion in MMS traffic"#),
            TwilioProgrammableSmsError::ErrorCode30028 => Some(r#"API version is not specified or is not either 2008-08-01 or 2010-04-01"#),
            TwilioProgrammableSmsError::ErrorCode21713 => Some(r#"* The Messaging Service use case specified in the request is not a valid Messaging Service use case. Valid Messaging Service use cases include: discussion, marketing, notifications, poll, undeclared and verification."#),
            TwilioProgrammableSmsError::ErrorCode21708 => Some(r#"
* Ensure that a request contains an alphasender in it"#),
            TwilioProgrammableSmsError::ErrorCode30030 => Some(r#"AddressRetention is not either 'obfuscate' or 'retain' or Message Privacy features are not enabled for this account"#),
            TwilioProgrammableSmsError::ErrorCode30500 => Some(r#"* If this message was being processed during an incident, it is possible that this message was affected. See [Twilio's status page](https://status.twilio.com/) for ongoing and historical incidents.
* There was an unrecoverable anomaly that occurred while processing this particular message."#),
            TwilioProgrammableSmsError::ErrorCode21603 => Some(r#"* You did not specify a 'From' parameter in your API request"#),
            TwilioProgrammableSmsError::ErrorCode57010 => Some(r#"'PartnerName' is not specified"#),
            TwilioProgrammableSmsError::ErrorCode63030 => Some(r#"* Media message not supported with template messages
* No URL to support preview"#),
            TwilioProgrammableSmsError::ErrorCode14107 => Some(r#"*  repeated rapid responses by the end user (30 replies in less than 30 seconds)
*  infinite loop caused by self referencing Sms or Message verb action URL
*  infinite loop caused by self referencing Redirect verb URL
*  runaway process making repeated outgoing REST API requests"#),
            TwilioProgrammableSmsError::ErrorCode63002 => None,
            TwilioProgrammableSmsError::ErrorCode30109 => Some(r#"- Protocol could be missing
- Hostname could be invalid
- Incorrect encoding"#),
            TwilioProgrammableSmsError::ErrorCode21656 => Some(r#"* The ContentVariables Parameter is not a JSON String. 
* A ContentVariables variable placeholder is improperly formed e.g. {{example}. 
* A ContentVariables variable placeholder is an "empty mustache" e.g. {{}}. 
* The ContentVariables field is defined with improper formatting at time of send. 
* The ContentVariables field contains a variable that was set to null at time of send.
"#),
            TwilioProgrammableSmsError::ErrorCode57012 => Some(r#"Request signature passed in X-Registry-Signature header is invalid"#),
            TwilioProgrammableSmsError::ErrorCode30102 => Some(r#"TLS certificate associated with the Domain is expired."#),
            TwilioProgrammableSmsError::ErrorCode63015 => Some(r#"This message failed to be delivered to the user because it was sent from a Sandbox to a phone number that has not joined your Sandbox"#),
            TwilioProgrammableSmsError::ErrorCode92006 => Some(r#"The Content Sid you are using is not valid"#),
            TwilioProgrammableSmsError::ErrorCode63032 => Some(r#"Failed to send message because this user's phone number is part of an experiment."#),
            TwilioProgrammableSmsError::ErrorCode30005 => Some(r#"*  The destination number you are trying to reach is unknown and may no longer exist.
*  The device you are trying to reach is not on or does not have sufficient signal.
*  The device cannot receive SMS (for example, the phone number belongs to a landline)
*  There is an issue with the mobile carrier"#),
            TwilioProgrammableSmsError::ErrorCode30110 => Some(r#"- Fraudulent activity has been detected on this domain
- Domain owner has requested Twilio not link to their site"#),
            TwilioProgrammableSmsError::ErrorCode14109 => Some(r#"* A runaway loop or other error in your application generated more than 10 TwiML replies to a single incoming message
* You have attempted to send >10 TwiML replies to a single incoming message"#),
            TwilioProgrammableSmsError::ErrorCode35114 => Some(r#"Scheduler currently doesn't support scheduling a message at a fixed time less that 15 minutes from now, or more than 35 days in the future. *Scheduling up to 35 days before is available in Pilot with limited access."#),
            TwilioProgrammableSmsError::ErrorCode21704 => Some(r#"* Your Messaging Service does not contain any senders"#),
            TwilioProgrammableSmsError::ErrorCode21721 => Some(r#"- Your A2P brand has a status of `FAILED` meaning the brand was not successfully registered
- Your A2P brand is of type `STARTER` meaning it is a starter brand, not a standard brand"#),
            TwilioProgrammableSmsError::ErrorCode30033 => Some(r#"Campaign Suspension"#),
            TwilioProgrammableSmsError::ErrorCode23001 => Some(r#"* Twilio is currently set to handle opt-out keywords for long code numbers on your account. Twilio's opt-out handling requires Twilio to save non-redacted phone numbers of users who have opted out, and is incompatible with Phone Number Redaction."#),
            TwilioProgrammableSmsError::ErrorCode63033 => Some(r#"Recipient has opted out of WhatsApp marketing messages from your business."#),
            TwilioProgrammableSmsError::ErrorCode30134 => Some(r#"Domain Dns setup is not completed "#),
            TwilioProgrammableSmsError::ErrorCode23003 => Some(r#"* You have Sticky Sender enabled on your Messaging Service. Sticky Sender requires Twilio to save the non-redacted phone numbers of recipients, and is incompatible with Phone Number Redaction."#),
            TwilioProgrammableSmsError::ErrorCode63010 => None,
            TwilioProgrammableSmsError::ErrorCode21728 => Some(r#"One or more field(s) do not satisfy length requirements."#),
            TwilioProgrammableSmsError::ErrorCode63017 => None,
            TwilioProgrammableSmsError::ErrorCode30035 => Some(r#"Twilio is still processing your Number Registration requests with ecosystem partners for US A2P 10DLC"#),
            TwilioProgrammableSmsError::ErrorCode92003 => Some(r#"You attempted to create content, but you forgot to include the 'language', the language property is required. "#),
            TwilioProgrammableSmsError::ErrorCode21727 => Some(r#"One or more required parameters are missing for Campaign registration"#),
            TwilioProgrammableSmsError::ErrorCode63026 => None,
            TwilioProgrammableSmsError::ErrorCode23006 => Some(r#"* The Incoming Message Webhook is set to the GET method for your Twilio phone number or Messaging Service."#),
            TwilioProgrammableSmsError::ErrorCode21714 => Some(r#"* Your Messaging Service number pool is already full (default: 400 numbers)
* You may have attempted to add more than one alpha sender for the same country to a Messaging Service or Sender Pool. Each country can only have one alpha sender assigned per Messaging Service Number Pool.
"#),
            TwilioProgrammableSmsError::ErrorCode30004 => Some(r#"*  The destination number you are trying to reach is blocked from receiving this message.
*  The device you are trying to reach does not have sufficient signal.
*  The device cannot receive SMS (for example, the phone number belongs to a landline).
*  The destination number is on India's national Do Not Call registry.
*  There is an issue with the mobile carrier.
*  You have sent a message from a US/CA Toll-free number to an end user handset that has previously responded with "STOP" or [another opt-out keyword](https://support.twilio.com/hc/en-us/articles/223134027-Twilio-support-for-opt-out-keywords-SMS-STOP-filtering-)."#),
            TwilioProgrammableSmsError::ErrorCode63037 => Some(r#"An attempt by Twilio failed to upload media to the Channel provider. Twilio will retry this operation."#),
            TwilioProgrammableSmsError::ErrorCode21715 => Some(r#"* The phone number you are trying to add to the Messaging Service does not have SMS or MMS capabilities."#),
            TwilioProgrammableSmsError::ErrorCode63039 => Some(r#"Facebook and/or Facebook users consider the messages abusive or bothersome, falling outside of Facebook's policies."#),
            TwilioProgrammableSmsError::ErrorCode30453 => Some(r#"* This is because Twilio has identified potential fraudulent messages being sent to the destination you are trying to reach.
* Hence SMS traffic can not be delivered to the destination phone number used for the next couple of hours. The traffic should return to normalcy post that. However, the SMS traffic could be undelivered for an even longer period if fraudulent activity doesn't subside & continues to go on."#),
            TwilioProgrammableSmsError::ErrorCode30106 => Some(r#"Onboarding process is incomplete."#),
            TwilioProgrammableSmsError::ErrorCode21726 => Some(r#"The starter brand you are trying to create/update is a Starter brand. See [New Requirements for A2P 10DLC Registrations](https://www.twilio.com/blog/new-requirements-for-a2p-10dlc-registrations) for more info."#),
            TwilioProgrammableSmsError::ErrorCode35115 => Some(r#"Scheduler currently doesn't support scheduling a message at a fixed time less that 15 minutes from now, or more than 35 days in the future."#),
            TwilioProgrammableSmsError::ErrorCode30018 => Some(r#"* The recipient you are sending a message to is on a network that requires alphanumeric sender ID pre-registration, and our records indicate that your sender ID is not registered."#),
            TwilioProgrammableSmsError::ErrorCode30101 => Some(r#"You have yet to verify your Domain in Organization. You could be using the wrong Domain SID in your current request. "#),
            TwilioProgrammableSmsError::ErrorCode30025 => Some(r#"* You have consumed 50% of your T-Mobile daily message limit for the day."#),
            TwilioProgrammableSmsError::ErrorCode63014 => None
        }
    }
}
