// AUTO-GENERATED. DO NOT MODIFY. YOUR CHANGES WILL BE OVER-WRITTEN
use crate::products::TwilioProgrammableSmsError;
use standard_error::traits::StandardErrorSolutionsTrait;

impl StandardErrorSolutionsTrait for TwilioProgrammableSmsError {
    fn solutions(&self) -> Option<&'static str> {
        match self {
            TwilioProgrammableSmsError::ErrorCode30111 => Some(r#"Please reach out to Twilio support"#),
            TwilioProgrammableSmsError::ErrorCode57016 => Some(r#"Confirm a valid 'Topic' is being passed in request. "#),
            TwilioProgrammableSmsError::ErrorCode30118 => Some(r#"Re-generate and re-upload private key."#),
            TwilioProgrammableSmsError::ErrorCode21725 => Some(r#"- If Brand is REGISTERED, and needs to be updated, contact Twilio support
- If Brand is IN_PROGRESS, wait for it to exit IN_PROGRESS"#),
            TwilioProgrammableSmsError::ErrorCode30006 => Some(r#"* Use [Lookup](https://www.twilio.com/docs/lookup/v2-api/line-type-intelligence) to determine if the number is indeed a landline. If not, try again with a different phone number type. Check out the video linked below for more information. 

https://www.youtube.com/watch?v=d8GfbPBaSuM"#),
            TwilioProgrammableSmsError::ErrorCode30117 => Some(r#"Re-generate certificate ensuring it's in PEM format: https://en.wikipedia.org/wiki/Privacy-Enhanced_Mail"#),
            TwilioProgrammableSmsError::ErrorCode30107 => Some(r#"Please go to the organizations view for your account and ensure that the private certificate has been uploaded for this domain"#),
            TwilioProgrammableSmsError::ErrorCode57013 => Some(r#"Confirm a valid 'Topic' is being passed in request. "#),
            TwilioProgrammableSmsError::ErrorCode57020 => Some(r#"Confirm a valid 'Authorisation' header is being passed in the request"#),
            TwilioProgrammableSmsError::ErrorCode30019 => Some(r#"Content size should be within [carrier limits](https://support.twilio.com/hc/en-us/articles/360018832773-Twilio-Programmable-SMS-Supported-File-Types-and-Size-Limits-for-MMS-Media-Messages). 

Message size should be within the [carrier limits](https://support.twilio.com/hc/en-us/articles/360033806753-Maximum-Message-Length-with-Twilio-Programmable-Messaging). "#),
            TwilioProgrammableSmsError::ErrorCode30043 => Some(r#"Check the SMS Guidelines for the destination country for any restrictions on domestic or international traffic:
- If the message is genuinely domestic, open a support request and provide evidence to clarify.
- If the message is international, open a support request  and request to move your messages to appropriate international gateways."#),
            TwilioProgrammableSmsError::ErrorCode90009 => Some(r#"* Change or generate new message SID."#),
            TwilioProgrammableSmsError::ErrorCode92008 => Some(r#"To create button templates, please use the twilio/call-to-action or twilio/quick-reply template types."#),
            TwilioProgrammableSmsError::ErrorCode90007 => Some(r#"Fix validity period value"#),
            TwilioProgrammableSmsError::ErrorCode30124 => Some(r#"Please follow doc for setting up messaging service sid"#),
            TwilioProgrammableSmsError::ErrorCode63016 => Some(r#"* [Send a WhatsApp message using a template](https://www.twilio.com/docs/whatsapp/api#send-a-whatsapp-message-using-a-template)
* [Include all the necessary parameters for a Content API or Content Editor Template and make sure you aren't sending with the wrong parameters](https://www.twilio.com/docs/content/send-templates-created-with-the-content-editor)
* If you are a publisher, register your Facebook Page with the [Facebook News Page Index](https://www.facebook.com/business/help/316333835842972?id=644465919618833)"#),
            TwilioProgrammableSmsError::ErrorCode21654 => Some(r#"Please provide a ContentSid when the request includes the ContentVariables parameter. Review [this documentation](https://www.twilio.com/docs/content/send-templates-created-with-the-content-template-builder) for how to send Content Templates."#),
            TwilioProgrammableSmsError::ErrorCode30409 => Some(r#"N/A"#),
            TwilioProgrammableSmsError::ErrorCode30119 => Some(r#"Please regenerated and reupload key and certificate ensuring they are both for the correct domain."#),
            TwilioProgrammableSmsError::ErrorCode21606 => Some(r#"* Check that you are using a Twilio phone number with SMS capabilities. You can see your purchased phone numbers and their capabilities in the [Twilio console](https://www.twilio.com/console/phone-numbers/incoming).
* Ensure the number you are using is in [E.164 format](https://www.twilio.com/docs/glossary/what-e164). 
* If you are sending from a Short Code, verify that the country you are sending to matches the country of the Short Code.
* If you have ported/hosted the `From` number, ensure that the process is complete. You can follow up on this in the Twilio console for [porting](https://console.twilio.com/us1/develop/phone-numbers/port-host/porting-requests?frameUrl=%2Fconsole%2Fphone-numbers%2Fporting-requests%3Fx-target-region%3Dus1) and [hosting](https://console.twilio.com/us1/develop/phone-numbers/port-host/hosted-numbers?frameUrl=%2Fconsole%2Fphone-numbers%2Fhosted%3Fx-target-region%3Dus1).
* Check that you are using account credentials that correspond with the account that owns the phone number.
* If you are sending messages with test credentials, verify that you are using one of the [available test credential "magic phone numbers"](https://www.twilio.com/docs/iam/test-credentials#test-sms-messages-parameters-From).
"#),
            TwilioProgrammableSmsError::ErrorCode63031 => Some(r#"Review your `To` and `From` parameters and include the correct values. "#),
            TwilioProgrammableSmsError::ErrorCode23004 => Some(r#"* To resolve this issue, your application needs to be able to handle [opt-out keyword messages](https://support.twilio.com/hc/en-us/articles/223134027-Twilio-support-for-opt-out-keywords-SMS-STOP-filtering-) from end users.
* Users who reply STOP, STOPALL, UNSUBSCRIBE, CANCEL, END, or QUIT should be added to a block list in your application to ensure you do not send them any further messages, unless they opt-in again.
* Users who reply HELP or INFO should be sent an informational message about your application or service.
* Users who reply START, YES, or UNSTOP should be removed from your "STOP" block list so they can receive messages from you again.
* Once you are ready to disable Twilio's built-in opt-out handling and process the above keywords in your own application, please contact Twilio Support through the [Console](https://console.twilio.com) or [Help Center](https://support.twilio.com) to disable all Twilio keyword handling for your account, including Advanced Opt-Out."#),
            TwilioProgrammableSmsError::ErrorCode11751 => Some(r#"If you are sending multiple media files on a message, split them into multiple requests to reduce the size of each request."#),
            TwilioProgrammableSmsError::ErrorCode30036 => Some(r#"You can retry the message with a longer validity period and avoid sending very large batches of messages at a rate that far exceeds your messaging rate limits."#),
            TwilioProgrammableSmsError::ErrorCode30121 => Some(r#"Please follow doc for setting up fallback URL."#),
            TwilioProgrammableSmsError::ErrorCode21611 => Some(r#"* Slow down your sending rate to avoid queuing on your Twilio number.
* Consider using a [Messaging Service](https://www.twilio.com/docs/messaging/services) to better distribute message throughput, if appropriate for your use case.
* Increase your [validity period](https://www.twilio.com/docs/messaging/api/message-resource#create-a-message-resource) if you have it set to lower than 10 hours
"#),
            TwilioProgrammableSmsError::ErrorCode30027 => Some(r#"* If you infrequently exceed your T-Mobile daily message limit, you can consider prioritizing messages (e.g., OTP codes over marketing materials).
* If you frequently exceed your T-Mobile daily message limit, you may be able to increase your daily limit: 
  * For unregistered customers, please consider registering in the [Twilio Console - Trust Hub](https://console.twilio.com/us1/account/trust-hub/a2p?frameUrl=%2Fconsole%2Ftrust-hub%2Fa2p-messaging%2Fwizard%3Fx-target-region%3Dus1&currentFrameUrl=%2Fconsole%2Ftrust-hub%2Fa2p-messaging%2Fwizard%3F__override_layout__%3Dembed%26x-target-region%3Dus1%26activeStep%3DbusinessProfile%253AbusinessProfile%253AcreateBusinessProfileAddress%26bifrost%3Dtrue).
 * For registered customers, if you need to send more than 200,000 message segments to T-Mobile per day, please consider submitting a [Special Business Review](https://support.twilio.com/hc/en-us/articles/4403550579739-T-Mobile-Special-Business-Review-for-A2P-10DLC).
* Other useful links:
 * To view the warning and errors, please visit the [debugger](https://console.twilio.com/us1/monitor/logs/debugger/events?frameUrl=%2Fconsole%2Fdebugger%3F__override_layout__%3Dembed%26quickDate%3D24%26x-target-region%3Dus1%26bifrost%3Dtrue&currentFrameUrl=%2Fconsole%2Fdebugger%3F__override_layout__%3Dembed%26bifrost%3Dtrue%26quickDate%3D24%26x-target-region%3Dus1).
 * To view your daily T-Mobile message limit, please visit [Trust Hub - A2P Messaging](https://console.twilio.com/us1/account/trust-hub/a2p?frameUrl=%2Fconsole%2Ftrust-hub%2Fa2p-messaging%3Fx-target-region%3Dus1).
 * To view your daily T-Mobile usage, please use the Messaging Insights tool."#),
            TwilioProgrammableSmsError::ErrorCode63008 => None,
            TwilioProgrammableSmsError::ErrorCode23002 => Some(r#"* To resolve this issue, your application needs to be able to handle [opt-out keyword messages](https://support.twilio.com/hc/en-us/articles/223134027-Twilio-support-for-opt-out-keywords-SMS-STOP-filtering-) from end users.
* Users who reply STOP, STOPALL, UNSUBSCRIBE, CANCEL, END, or QUIT should be added to a blocklist in your application to ensure you do not send them any further messages, unless they opt-in again.
* Users who reply HELP or INFO should be sent an informational message about your application or service.
* Users who reply START, YES, or UNSTOP should be removed from your "STOP" blocklist so they can receive messages from you again.
* Once you are ready to disable Twilio's built-in opt-out handling and process the above keywords in your own application, please reach out to Twilio Support through the [Console](https://console.twilio.com) or [Help Center](https://support.twilio.com) to disable Twilio's built-in opt-out keyword handling for your account."#),
            TwilioProgrammableSmsError::ErrorCode30011 => Some(r#"For Inbound, it is recommended that you respond to the user sending the MMS to let them know that you cannot receive MMS in this region."#),
            TwilioProgrammableSmsError::ErrorCode30130 => Some(r#"Try to use any other messaging service SID"#),
            TwilioProgrammableSmsError::ErrorCode21627 => Some(r#"Set Max Price to positive float value"#),
            TwilioProgrammableSmsError::ErrorCode23005 => Some(r#"* Disable Fallback to Long Code on your Messaging Services using the [Twilio Console](https://www.twilio.com/console/sms/services) or the [REST API](https://www.twilio.com/docs/messaging/services/api).
* If Fallback to Long Code functionality is needed, implement this functionality in your own application."#),
            TwilioProgrammableSmsError::ErrorCode90014 => Some(r#"Change the Validity Period to be positive integer"#),
            TwilioProgrammableSmsError::ErrorCode92005 => Some(r#"Please provide a ContentSid when the request includes the ContentVariables parameter"#),
            TwilioProgrammableSmsError::ErrorCode21408 => Some(r#"If you wish to send messages to this region, please enable the relevant permissions on your account using the [Messaging Geographic Permissions](https://console.twilio.com/us1/develop/sms/settings/geo-permissions) page."#),
            TwilioProgrammableSmsError::ErrorCode35125 => Some(r#"Wait for the scheduled messages to be sent. At the send_at time, as the messages are sent (or canceled) the limit will free up. If this limitation is a blocker, kindly contact Twilio Support through the [Console](https://console.twilio.com) or [Help Center](https://support.twilio.com)."#),
            TwilioProgrammableSmsError::ErrorCode21712 => None,
            TwilioProgrammableSmsError::ErrorCode57006 => Some(r#"Confirm a valid 'EventType' is being passed in request."#),
            TwilioProgrammableSmsError::ErrorCode57007 => Some(r#"Confirm a 'EventType' is being passed in request."#),
            TwilioProgrammableSmsError::ErrorCode30108 => Some(r#"Please follow setup guides for Click Tracking."#),
            TwilioProgrammableSmsError::ErrorCode57009 => Some(r#"Confirm a valid 'EventType' is being passed in request."#),
            TwilioProgrammableSmsError::ErrorCode30022 => Some(r#"* Slow down your rate of message sending."#),
            TwilioProgrammableSmsError::ErrorCode30100 => Some(r#"Check the Domain SID from your Organization through the Admin Center."#),
            TwilioProgrammableSmsError::ErrorCode30002 => Some(r#"*  Please [contact Twilio support](https://www.twilio.com/console/support/tickets/create).
"#),
            TwilioProgrammableSmsError::ErrorCode92007 => Some(r#"Validate the Content Variables Parameter, it must be a JSON String "#),
            TwilioProgrammableSmsError::ErrorCode21723 => Some(r#"Verify that there are not existing vetting attempts for Campaign Verify that are in progress by checking the `vettingStatus` field of the Fetch Vettings API . There should not be a vetting listed with the `vettingProvider: "CampaignVerify"` that has a `vettingStatus` of `IN_PROGRESS`. Please wait until the vetting status moves to `SUCCESS` or `FAILED` before attempting to import a new Campaign Verify token. 
"#),
            TwilioProgrammableSmsError::ErrorCode30009 => Some(r#"* Ensure the mobile user is in good network coverage and request they try sending the message again
* If the issue is occurring frequently for multiple users, [contact Twilio Support](https://www.twilio.com/console/support/tickets/create) with Message SIDs of affected messages"#),
            TwilioProgrammableSmsError::ErrorCode63028 => None,
            TwilioProgrammableSmsError::ErrorCode30116 => Some(r#"- Generate a new certificate in PEM format and reupload"#),
            TwilioProgrammableSmsError::ErrorCode63001 => Some(r#"If the issue persists, you may need to delete the sender and and re-add it."#),
            TwilioProgrammableSmsError::ErrorCode90006 => Some(r#"Contact Customer Support for help"#),
            TwilioProgrammableSmsError::ErrorCode30122 => Some(r#"Use a correctly formatted URL in the fallbackUrl field"#),
            TwilioProgrammableSmsError::ErrorCode57018 => Some(r#"Confirm a valid 'Event' is being passed in request. "#),
            TwilioProgrammableSmsError::ErrorCode57003 => Some(r#"Confirm a valid 'Secret id' is being passed in request. "#),
            TwilioProgrammableSmsError::ErrorCode30031 => Some(r#"Use a positive float value for MaxRate"#),
            TwilioProgrammableSmsError::ErrorCode57004 => Some(r#"Confirm a valid 'Category' is being passed in request."#),
            TwilioProgrammableSmsError::ErrorCode21614 => Some(r#"* Confirm that the number you are sending to is not a landline, using the [Lookup API](https://www.twilio.com/docs/lookup/v2-api).
* Please verify you have provided a valid mobile number in proper [E.164 format](https://support.twilio.com/hc/en-us/articles/223183008-Formatting-International-Phone-Numbers).
* If you are attempting to send SMS to an IoT or M2M number, check whether the number format is different from the standard mobile numbers in that country or locality. Often, these numbers have additional digits or unusual formats which do not pass Twilio's number validation. If you believe this is the issue, please [contact Twilio Support](https://www.twilio.com/help/contact) for assistance."#),
            TwilioProgrammableSmsError::ErrorCode63023 => None,
            TwilioProgrammableSmsError::ErrorCode30041 => Some(r#"Alphanumeric, Shortcode, Twilio Domestic Longcode must complete and return a Letter of Authorization (LOA) to Twilio in order to continue using Protected Sender IDs to send messages to the United Kingdom. [More details on UK SMS Guidelines](https://www.twilio.com/en-us/guidelines/gb/sms)."#),
            TwilioProgrammableSmsError::ErrorCode21709 => Some(r#"* Follow [these steps](https://support.twilio.com/hc/en-us/articles/223181348-Alphanumeric-Sender-ID-for-Twilio-Programmable-SMS#h_01F4SK0WYHA75NK1DNXGTCFMP8) to verify your Messaging Service is enabled to support Alpha Sender IDs. If it is not, please reach out to Support.
* Verify the Alpha Sender ID you are trying to add to the Messaging Service is valid. View formatting requirements [here](https://support.twilio.com/hc/en-us/articles/223181348-Alphanumeric-Sender-ID-for-Twilio-Programmable-SMS#h_01F4SK0R6NH8AQW36Y4BR16FVA)."#),
            TwilioProgrammableSmsError::ErrorCode21658 => Some(r#"Validate parameter in within character limit"#),
            TwilioProgrammableSmsError::ErrorCode21722 => Some(r#"Verify that you are using a valid token from Campaign Verify that is not expired and hasn’t been used by another A2P brand. Then, try to import your token again."#),
            TwilioProgrammableSmsError::ErrorCode63038 => Some(r#"* Wait for the next day or contact support to raise the limit.
* Check your [support center](https://help.twilio.com/tickets) or email inbox/spam folder for outreach from the Twilio team on how to remove/raise your daily limit."#),
            TwilioProgrammableSmsError::ErrorCode57002 => Some(r#"Confirm a valid 'Secret id' is being passed in request. "#),
            TwilioProgrammableSmsError::ErrorCode63020 => None,
            TwilioProgrammableSmsError::ErrorCode63035 => Some(r#"If device is RCS capable, accept invitation to become a tester. If the device has already accepted the invitation, please confirm that you are not attempting to send from an RCS sender that is regionally restricted."#),
            TwilioProgrammableSmsError::ErrorCode63011 => None,
            TwilioProgrammableSmsError::ErrorCode92004 => Some(r#"Please review that the language code is supported"#),
            TwilioProgrammableSmsError::ErrorCode30133 => Some(r#"Please try uploading again in a few minutes."#),
            TwilioProgrammableSmsError::ErrorCode21720 => Some(r#"Verify that you are spelling the use case correctly in your request and that it is using all uppercase letters. The following resource can also be used to retrieve all valid A2P use cases for a specific Messaging Service: https://messaging.twilio.com/v1/Services/MGXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXX/Compliance/Usa2p/Usecases"#),
            TwilioProgrammableSmsError::ErrorCode57011 => Some(r#"Confirm a valid 'PartnerName' is being passed in request."#),
            TwilioProgrammableSmsError::ErrorCode21711 => Some(r#"Please add this number to a specified Messaging Service. [See here](https://www.twilio.com/docs/messaging/services/api/phonenumber-resource#create-a-phonenumber-resource-add-a-phone-number-to-a-messaging-service) for a code sample. "#),
            TwilioProgrammableSmsError::ErrorCode30400 => Some(r#"- To cancel the message, include Status: canceled.
- To redact the message body, ensure Body is empty. More details [here](https://support.twilio.com/hc/en-us/articles/223133687-Deleting-messages-message-media-or-message-bodies).
- If you want to cancel and redact the message body: First, request the message to be canceled. Then, request for message body redaction. More details [here](https://support.twilio.com/hc/en-us/articles/223133687-Deleting-messages-message-media-or-message-bodies)."#),
            TwilioProgrammableSmsError::ErrorCode21605 => Some(r#"Reduce the number of characters in your message body and review our FAQ on message size constraints."#),
            TwilioProgrammableSmsError::ErrorCode21612 => Some(r#"Consult the linked documentation for each cause. You can also try sending again with a different "To" and "From" combination."#),
            TwilioProgrammableSmsError::ErrorCode57017 => Some(r#"Confirm a valid 'Topic' is being passed in request. "#),
            TwilioProgrammableSmsError::ErrorCode21619 => Some(r#"* Ensure you are specifying the correct API Parameters when sending your message.
* If you are using Content Templates, you need to specify a [Content SID](https://www.twilio.com/docs/content/send-templates-created-with-the-content-template-builder), otherwise you will need to send a [“Body” and/or “Media URL”](https://www.twilio.com/docs/messaging/api/message-resource#create-a-message-resource).
"#),
            TwilioProgrammableSmsError::ErrorCode30007 => Some(r#"* Ensure your messaging use case complies with Twilio's [Messaging Policy](https://www.twilio.com/en-us/legal/messaging-policy) and [Acceptable Use Policy](https://www.twilio.com/en-us/legal/aup)
* Review the information in [How Does Message Filtering Work?](https://support.twilio.com/hc/en-us/articles/223181848-How-Does-Carrier-Filtering-Work-) to understand what causes filtering.
* See [How do I prevent my Twilio messages from being filtered (blocked)?](https://support.twilio.com/hc/en-us/articles/1260803966670-How-do-I-prevent-my-Twilio-messages-from-being-filtered-blocked-) for specific tips on avoiding message filtering.
* If you believe your messages are compliant with Twilio and carrier policies, please collect 3 or more examples of Message SIDs that have the “undelivered” status with error 30007, and then [contact our Support team](https://www.twilio.com/console/support/tickets/create). We can help review your messaging and determine if an error was made, and put you in touch with our Compliance team if needed."#),
            TwilioProgrammableSmsError::ErrorCode63006 => None,
            TwilioProgrammableSmsError::ErrorCode63029 => Some(r#"Please check your destination number/handset and retry the number"#),
            TwilioProgrammableSmsError::ErrorCode30040 => Some(r#"* Learn more about Alpha Sender ID registration requirements [here](https://support.twilio.com/hc/en-us/articles/223133767-International-support-for-Alphanumeric-Sender-ID).
* Use [this form](https://twiliodoer.secure.force.com/SenderId) to register a Sender ID for the destination country."#),
            TwilioProgrammableSmsError::ErrorCode57021 => Some(r#"Pass the correct token."#),
            TwilioProgrammableSmsError::ErrorCode35126 => Some(r#"Change the ScheduleType value to ‘fixed’"#),
            TwilioProgrammableSmsError::ErrorCode30038 => Some(r#"In order to prevent account creation and abuse with Twilio hosted numbers, Twilio does not generally allow the reception of OTP messages. Contact our [Support team](https://www.twilio.com/console/support/tickets/create) if additional information is required."#),
            TwilioProgrammableSmsError::ErrorCode35111 => Some(r#"Check if there is a valid timestamp entered for SendAt when ScheduleType is 'fixed'."#),
            TwilioProgrammableSmsError::ErrorCode30103 => Some(r#"File a support ticket and include the Message SID."#),
            TwilioProgrammableSmsError::ErrorCode30105 => Some(r#"- Contact your account manager or support about extending the retention period of links.
- Set a fallback url to mitigate this error"#),
            TwilioProgrammableSmsError::ErrorCode21710 => Some(r#"* If you know this phone number is properly associated with the Messaging Service, do nothing. 
* Remove the existing phone number, short code or alpha sender ID from the Messaging Service and add it back to the Messaging Service’s Sender Pool."#),
            TwilioProgrammableSmsError::ErrorCode30404 => Some(r#"Confirm that the AccountSid and MessagingServiceSid is correct"#),
            TwilioProgrammableSmsError::ErrorCode57005 => Some(r#"Confirm a valid 'Category' is being passed in request."#),
            TwilioProgrammableSmsError::ErrorCode30129 => Some(r#"Please use a certificate with valid signature"#),
            TwilioProgrammableSmsError::ErrorCode21730 => Some(r#"Try again later because once maintenance activity is completed all requests are processed successfully."#),
            TwilioProgrammableSmsError::ErrorCode30127 => Some(r#"Please follow doc for valid messaging service sid."#),
            TwilioProgrammableSmsError::ErrorCode21902 => Some(r#"Decrease InvoiceTag length"#),
            TwilioProgrammableSmsError::ErrorCode30128 => Some(r#"Try to use any of the action types - ADD, DELETE, REPLACE"#),
            TwilioProgrammableSmsError::ErrorCode30037 => Some(r#"If you believe this configuration is in error you can [contact support](https://support.twilio.com/hc/en-us/articles/360048500694-Contacting-Twilio-Support) to adjust the account configuration."#),
            TwilioProgrammableSmsError::ErrorCode30032 => Some(r#"Please refer to this [Support Article](https://support.twilio.com/hc/en-us/articles/5377174717595-Toll-Free-Message-Verification-for-US-Canada) for the process to get your Toll Free number verified. 

Effective Nov 8th, 2023, messages on restricted toll-free phone numbers are blocked.
Effective Jan 31st, 2024, messages on pending toll-free phone numbers are blocked.

Toll-Free best practices can be found [here](https://support.twilio.com/hc/en-us/articles/360038172934-Information-and-best-practices-for-using-Toll-Free-SMS-and-MMS-in-the-US-and-Canada)"#),
            TwilioProgrammableSmsError::ErrorCode21910 => Some(r#"Ensure `From` and `To` parameters are the same channel, eg. both are SMS or both are WhatsApp."#),
            TwilioProgrammableSmsError::ErrorCode92009 => Some(r#"Please recreate a new template to make any changes."#),
            TwilioProgrammableSmsError::ErrorCode57019 => Some(r#"Confirm a valid 'Authorisation' header is being passed in the request"#),
            TwilioProgrammableSmsError::ErrorCode30125 => Some(r#"You may continue to send messages to other non-US destinations using this phone number. However, if you want to use this phone number to send to recipients in the US, please consider removing all senders from this campaign from the Messaging Services page and then add back the number you wish to use, or register for a standard A2P brand and campaign instead."#),
            TwilioProgrammableSmsError::ErrorCode63025 => None,
            TwilioProgrammableSmsError::ErrorCode30021 => Some(r#"* If the error persists, please contact us to figure out what has happened and how to fix it."#),
            TwilioProgrammableSmsError::ErrorCode30485 => Some(r#"You do not need to take any specific action. The disruption in traffic is usually temporary and will resolve in a couple of hours if we do not detect additional fraudulent activity.

#### Continued issues with error 30485
Twilio's support team can help investigate what went wrong with delivering your message. Please collect 3 or more message SIDs in your [SMS logs](https://www.twilio.com/console/sms/logs) from the last 24 hours that were flagged with Error 30485, and [open a support request.](https://www.twilio.com/console/support/tickets/create)"#),
            TwilioProgrammableSmsError::ErrorCode30123 => Some(r#"Please follow doc for setting up callback URL."#),
            TwilioProgrammableSmsError::ErrorCode63022 => None,
            TwilioProgrammableSmsError::ErrorCode30024 => Some(r#"* If you believe your Numeric Sender ID should be registered, please collect 3 or more examples of Message SIDs that have the “undelivered” status with error 30024, and then contact our Support team. We can help review the registration status to resolve these errors. "#),
            TwilioProgrammableSmsError::ErrorCode30029 => Some(r#"Enable the correct Message Privacy features and use a valid value for ContentRetention"#),
            TwilioProgrammableSmsError::ErrorCode63009 => None,
            TwilioProgrammableSmsError::ErrorCode30114 => Some(r#"Please either specify an earlier date or wait for Twilio to process today's data."#),
            TwilioProgrammableSmsError::ErrorCode30003 => Some(r#"In some cases, a delivery error may occur once due to a transient issue downstream of Twilio. To test whether the issue occurs again, please attempt to send another message to the user via a [REST API](https://help.twilio.com/articles/223133907-Simple-example-for-sending-SMS-or-MMS-messages) request.

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
Twilio's Support team can help investigate what went wrong with delivering your message. Please collect 3 or more message SIDs in your [SMS logs](https://www.twilio.com/console/sms/logs) from the last 24 hours that were flagged with Error 30003, and [open a support request](https://www.twilio.com/console/support/tickets/create)."#),
            TwilioProgrammableSmsError::ErrorCode63003 => Some(r#"* Ensure you have provided the correct To address for the user you are trying to reach
* Ensure that the To address is still active"#),
            TwilioProgrammableSmsError::ErrorCode30131 => Some(r#"Please generate and upload a new certificate."#),
            TwilioProgrammableSmsError::ErrorCode35117 => Some(r#"Verify that the SendAt parameter is using "[YYYY]-[MM]-[DD]T[HH]:[MM]:[SS]Z" format (in UTC)."#),
            TwilioProgrammableSmsError::ErrorCode21655 => Some(r#"Double check the ContentSid parameter that you are using. Make sure it has the proper formatting and starts with an HX prefix."#),
            TwilioProgrammableSmsError::ErrorCode30034 => Some(r#"Associate your US 10DLC number with a registered A2P Campaign by adding it to the corresponding Messaging Service via the Twilio Console or API. Find out how to register using [this guide](https://support.twilio.com/hc/en-us/articles/1260801864489-How-do-I-register-to-use-A2P-10DLC-messaging-). 

Alternatively, you can also use a different number that is already associated with an approved A2P Campaign to send messages in the US.
"#),
            TwilioProgrammableSmsError::ErrorCode63007 => Some(r#"* See [here](https://www.twilio.com/docs/api/channels#channel-addresses) for supported formats of Channel endpoint addresses.
* To send messages using WhatsApp, the `From `address should be `whatsapp:<sandbox phone number>` or` whatsapp:<your twilio number>` . This can be found on the [WhatsApp sandbox page](https://www.twilio.com/console/messaging/whatsapp/sandbox) or [WhatsApp numbers page](https://www.twilio.com/console/messaging/whatsapp/numbers).
* If you are using the WhatsApp sandbox, ensure that you have activated the Twilio Sandbox for WhatsApp. This can be found on the [WhatsApp sandbox page](https://www.twilio.com/console/messaging/whatsapp/sandbox).
* Check your account SID and auth token are correct and correspond to the account that owns the Sender. You can find your account credentials in the Console [here](https://console.twilio.com/).
"#),
            TwilioProgrammableSmsError::ErrorCode30104 => Some(r#"- Contact your account manager or support about extending the retention period of links."#),
            TwilioProgrammableSmsError::ErrorCode30020 => Some(r#"* If the error persists, please contact us to figure out what has happened and how to fix it.
* Note the time of the error and what you were trying to do when it occurred."#),
            TwilioProgrammableSmsError::ErrorCode30450 => Some(r#"You do not need to take any specific action. The disruption in traffic is usually temporary and will resolve in a couple of hours if we do not detect additional fraudulent activity.

#### Continued issues with error 30450
Twilio's support team can help investigate what went wrong with delivering your message. Please collect 3 or more message SIDs in your [SMS logs](https://www.twilio.com/console/sms/logs) from the last 24 hours that were flagged with Error 30453, and [open a support request.](https://www.twilio.com/console/support/tickets/create)

Alternatively, you can use the optional <code style="color : name_color">RiskCheck</code> parameter when [creating a Message](https://www.twilio.com/docs/messaging/api/message-resource#create-a-message-resource) with the [Programmable Messaging API](https://www.twilio.com/docs/messaging/api). To prevent a known/legitimate message from getting blocked in future by SMS Pumping Protection, include the <code style="color : name_color">RiskCheck</code> parameter with value <code style="color : name_color">disable</code> when creating the new Message resource.

You can also mark known/legitimate phone numbers using the Global Safe List feature so that they don’t face any disruption . This provides an additional safety net against false positives, so that the traffic to this number never faces the above scenario. Add known phone numbers to the Safe List by using the [Global Safe List API.](https://www.twilio.com/docs/usage/global-safe-list)"#),
            TwilioProgrammableSmsError::ErrorCode30010 => Some(r#"* Increase the MaxPrice parameter value in your API requests.
* Remove the MaxPrice parameter from your API requests.
* Check your message body for any Unicode-only characters such as emojis, glyphs, or curly apostrophes or quotation marks which could cause the message to require additional segments.
* Consider using the [Smart Encoding](https://www.twilio.com/docs/messaging/services#smart-encoding) feature of [Messaging Services](https://www.twilio.com/docs/messaging/services) to automatically replace [certain Unicode-only characters](https://www.twilio.com/docs/messaging/services/smart-encoding-char-list) with standard GSM equivalents."#),
            TwilioProgrammableSmsError::ErrorCode21610 => Some(r#"* Consider removing this phone number from your list of recipients.
* Request the recipient to resubscribe to your messages by texting in "START" or [another opt-in keyword](https://support.twilio.com/hc/en-us/articles/223134027-Twilio-support-for-opt-out-keywords-SMS-STOP-filtering-) to your Twilio sender.
* Before sending messages to a recipient, ensure they have consented to receive messages from you. Please read these guidelines to understand [messaging opt-in requirements and best practices](https://www.twilio.com/blog/ctia-messaging-principles-and-best-practices)."#),
            TwilioProgrammableSmsError::ErrorCode90001 => Some(r#"Provide correct message SID in the request"#),
            TwilioProgrammableSmsError::ErrorCode21717 => Some(r#"* Verify if the Brand Registration SID has been registered by checking the `status` field using the [Brand API](https://www.twilio.com/docs/sms/a2p-10dlc/onboarding-isv-api#using-get-to-check-brand-registration-status). If the brand has been registered, it will have a status of APPROVED. If this is the case, verify that the Brand Registration SID passed into the request matches the Brand Registration SID that has been registered and retry the request.
* If Brand Registration has an IN_PROGRESS status, registration is in progress. Please wait for it to be in an APPROVED status before moving forward with US A2P campaign use case creation. 
* If Brand Registration has a REGISTRATION_FAILED status, you may need to correct your brand request and re-trigger Brand registration.
* Verify the brand_type of the Brand by checking the `brand_type` field using the [Brand API](https://www.twilio.com/docs/sms/a2p-10dlc/onboarding-isv-api#using-get-to-check-brand-registration-status). If the brand_type is STARTER, the brand can only be used to create STARTER campaign use cases. If it is a STANDARD brand_type, the brand can be used to create any other type of campaign use case."#),
            TwilioProgrammableSmsError::ErrorCode57001 => Some(r#"Confirm a valid 'Secret id' is being passed in request. "#),
            TwilioProgrammableSmsError::ErrorCode30454 => Some(r#"* Consider upgrading your account to remove the restrictions. It takes 3-4 hours to reflect the changes in all systems. Please retry after that."#),
            TwilioProgrammableSmsError::ErrorCode92002 => Some(r#"Please review the amount of variables defined in the 'variables' property"#),
            TwilioProgrammableSmsError::ErrorCode90031 => Some(r#"Specify at least one recipient for Broadcast request"#),
            TwilioProgrammableSmsError::ErrorCode63019 => Some(r#"Please check the media content before retrying"#),
            TwilioProgrammableSmsError::ErrorCode30026 => Some(r#"* No action is required from you. 
* To view the warning and errors in the debugger, please visit [here](https://console.twilio.com/us1/monitor/logs/debugger/events?frameUrl=%2Fconsole%2Fdebugger%3F__override_layout__%3Dembed%26quickDate%3D24%26x-target-region%3Dus1%26bifrost%3Dtrue&currentFrameUrl=%2Fconsole%2Fdebugger%3F__override_layout__%3Dembed%26bifrost%3Dtrue%26quickDate%3D24%26x-target-region%3Dus1).
* To view your daily T-Mobile message limit, please visit [Trust Hub - A2P Messaging](https://console.twilio.com/us1/account/trust-hub/a2p?frameUrl=%2Fconsole%2Ftrust-hub%2Fa2p-messaging%3Fx-target-region%3Dus1), select your US A2P brand, and view the  details of your TCR trust score (registered customers only).
* To view your daily T-Mobile usage, please use the Messaging Insights tool."#),
            TwilioProgrammableSmsError::ErrorCode30115 => Some(r#"Please use YYYY-MM-DD format for the Date field."#),
            TwilioProgrammableSmsError::ErrorCode35118 => Some(r#"Pass the MessagingServiceSid in either the MessagingServiceSid parameter or the From parameter."#),
            TwilioProgrammableSmsError::ErrorCode30039 => Some(r#"If you want to respond to these messages programmatically, you can listen for failed status callbacks with this error code and respond using the Twilio REST API."#),
            TwilioProgrammableSmsError::ErrorCode21652 => None,
            TwilioProgrammableSmsError::ErrorCode30008 => Some(r#"* Check that the phone you were sending to is turned on and can receive non-Twilio SMS
* Ensure that the phone is not roaming off network. We cannot guarantee message delivery on roaming phones.
* Try sending to other phones who have the same mobile carrier (you can use our Lookups API to determine the carrier if you’re unsure). If messages to other phones go through, the issue is likely device related. Try rebooting the device or contact the mobile carrier for help.
* If you are sending SMS from an alphanumeric sender ID, see if using a Twilio phone number works better. We’ve observed that certain networks may block alpha sender IDs.
* Try sending a shorter message to the phone, with simple content that does not include any special characters. This would give our support team an idea as to whether the failure is related to concatenation or character encoding.
* Twilio support can help investigate what went wrong with our carriers. Please open a support request and include a minimum of 3 or more message SIDs where a 30008 error was thrown. Per our carriers' requirements, these SIDs can be no older than 48 hours at most."#),
            TwilioProgrammableSmsError::ErrorCode30042 => Some(r#"Please review the SMS guidelines and use a Sender ID that is compliant with SMS guidelines"#),
            TwilioProgrammableSmsError::ErrorCode63021 => Some(r#"* Inspect the media files being sent and confirm their MIME type is in fact supported by the messaging channel. 
* Confirm message components follow character limitations, proper formatting, and channel specific rules mentioned in docs if applicable. 
* Confirm WhatsApp template sends do not exceed 1024 characters, which can occur when placeholder text substitutions or translations are too long."#),
            TwilioProgrammableSmsError::ErrorCode21624 => Some(r#"Set ValidityPeriod to an integer between 1 and 36,000 seconds (10 hours)"#),
            TwilioProgrammableSmsError::ErrorCode63034 => Some(r#"Re-attempt with media of size 5 MB or less."#),
            TwilioProgrammableSmsError::ErrorCode30126 => Some(r#"* Contact [Twilio Customer Support](https://www.twilio.com/console/support/tickets/create)
"#),
            TwilioProgrammableSmsError::ErrorCode57014 => Some(r#"Confirm a valid 'Event' is being passed in request. "#),
            TwilioProgrammableSmsError::ErrorCode21657 => Some(r#"Please review the SMS guidelines and use a Sender ID that is supported in the specific destination "#),
            TwilioProgrammableSmsError::ErrorCode63018 => Some(r#"* Wait until 24 hours have passed since starting a business-initiated conversation with one or more users"#),
            TwilioProgrammableSmsError::ErrorCode92001 => Some(r#"Please include at least one type within the 'types' property"#),
            TwilioProgrammableSmsError::ErrorCode30001 => Some(r#"* **Recommended** Try using a [Messaging Service](https://www.twilio.com/docs/messaging/services) with multiple senders and it will load balance for you.
* Try adding more senders into your [Messaging Service](https://www.twilio.com/docs/messaging/services).
* If you are messaging in the US or Canada, talk to Twilio Sales about getting a [Toll Free](https://www.twilio.com/en-us/messaging/channels/sms) or [Short Code](https://support.twilio.com/hc/en-us/articles/223182068-What-is-a-Messaging-Short-Code-) number that allows you to send more messages per second.
* Try sending your message again after waiting for some time.
* If you have set a lower Validity Period than the default and maximum value of 36000 seconds (10 hours), try setting a higher Validity Period for your messages."#),
            TwilioProgrammableSmsError::ErrorCode30120 => Some(r#"Please follow doc to upload cert for domain"#),
            TwilioProgrammableSmsError::ErrorCode30113 => Some(r#"Please specify a newer date to retrieve data."#),
            TwilioProgrammableSmsError::ErrorCode30023 => Some(r#"* You must wait till the next calendar day to resume message sending. The day resets at 00:00 Pacific Time (US). Pacific time is affected by Daylight Savings Time and Standard Time switches.
* To avoid exceeding your daily limit, monitor your daily T-Mobile usage using the Messaging Insights tool.
* You can also use the [30025](https://www.twilio.com/docs/api/errors/30025) and [30026](https://www.twilio.com/docs/api/errors/30026) ‘Daily Traffic Used’ and [30027](https://www.twilio.com/docs/api/errors/30027) ‘Daily Traffic Exceeded’ error warnings to understand what percentage of your daily traffic you have used. These warnings are available in the [console Error Logs](https://console.twilio.com/us1/monitor/logs/debugger/events?frameUrl=%2Fconsole%2Fdebugger%3F__override_layout__%3Dembed%26quickDate%3D24%26x-target-region%3Dus1%26bifrost%3Dtrue&currentFrameUrl=%2Fconsole%2Fdebugger%3F__override_layout__%3Dembed%26bifrost%3Dtrue%26quickDate%3D24%26x-target-region%3Dus1).
* For A2P registered customers, view your daily T-Mobile message limit by visiting [Trust Hub - A2P Messaging](https://console.twilio.com/us1/account/trust-hub/a2p?frameUrl=%2Fconsole%2Ftrust-hub%2Fa2p-messaging%3Fx-target-region%3Dus1), selecting your US A2P brand and viewing the details of your TCR trust score.
"#),
            TwilioProgrammableSmsError::ErrorCode63012 => None,
            TwilioProgrammableSmsError::ErrorCode57022 => Some(r#"Provide all the mandatory headers."#),
            TwilioProgrammableSmsError::ErrorCode57008 => Some(r#"Confirm a valid 'EventType' is being passed in request."#),
            TwilioProgrammableSmsError::ErrorCode21729 => Some(r#"Contact Twilio support."#),
            TwilioProgrammableSmsError::ErrorCode21719 => Some(r#"Update the request to pass a compatible A2P/Messaging Service use case. The following resource can be used to retrieve all compatible A2P use cases for a specific Messaging Service: https://messaging.twilio.com/v1/Services/MGXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXX/Compliance/Usa2p/Usecases"#),
            TwilioProgrammableSmsError::ErrorCode21724 => Some(r#"Contact Twilio support"#),
            TwilioProgrammableSmsError::ErrorCode63027 => Some(r#"* Double check your body string parameter to make sure it matches exactly with the approved template. New lines or blank characters may cause an issue.
* If the template was created with Content API or Content Editor. Validate that you are sending templates with the right parameters."#),
            TwilioProgrammableSmsError::ErrorCode30132 => Some(r#"Upload a new valid certificate as outlined in Twilio docs for Link Shortening."#),
            TwilioProgrammableSmsError::ErrorCode30410 => Some(r#" Please attempt to send messages at a later time."#),
            TwilioProgrammableSmsError::ErrorCode63005 => Some(r#"*   Make sure the end user you are messaging has a valid WhatsApp account.
*   Make sure the media file sent is valid. Try to download the media file being sent. Export the file to a different valid format and re-upload it to your storage solution. If sending over WhatsApp, resubmit a new template for approval with the new media file.
*   Confirm that you are sending rich content within the sending session guidelines. If fields that are only supported as part of a template or fields that are only available in session are sent incorrectly, this error may occur. "#),
            TwilioProgrammableSmsError::ErrorCode63013 => Some(r#"Check content is in compliance with Channel providers policy."#),
            TwilioProgrammableSmsError::ErrorCode30017 => Some(r#"* Send your message over a longer period of time, rather than sending in bursts"#),
            TwilioProgrammableSmsError::ErrorCode30028 => Some(r#"Use API version 2008-08-01 or 2010-04-01"#),
            TwilioProgrammableSmsError::ErrorCode21713 => Some(r#"* Verify that you are spelling the use case correctly in your request and that it is using all lowercase letters."#),
            TwilioProgrammableSmsError::ErrorCode21708 => Some(r#"Please include an alphasender in the request"#),
            TwilioProgrammableSmsError::ErrorCode30030 => Some(r#"Enable the correct Message Privacy features and use a valid value for AddressRetention"#),
            TwilioProgrammableSmsError::ErrorCode30500 => Some(r#"* Try sending the message again.
* If the subsequent attempt also fails with this error, [contact support](https://support.twilio.com/hc/en-us/articles/360048500694-Contacting-Twilio-Support) referencing the affected Message Sids for further troubleshooting."#),
            TwilioProgrammableSmsError::ErrorCode21603 => Some(r#"* Specify a valid [Twilio Number](/user/account/phone-numbers/), [Alphanumeric Sender ID](https://support.twilio.com/hc/en-us/articles/223181348-Alphanumeric-Sender-ID-for-Twilio-Programmable-SMS), or [Messaging Service SID](https://www.twilio.com/docs/messaging/services) as the 'From' parameter in your API request
* Specify a valid Messaging Service SID as the 'MessagingServiceSid' parameter in your API request"#),
            TwilioProgrammableSmsError::ErrorCode57010 => Some(r#"Confirm a valid 'PartnerName' is being passed in request."#),
            TwilioProgrammableSmsError::ErrorCode63030 => Some(r#"* Send template message without the media component
* Check url for correctness for url previews"#),
            TwilioProgrammableSmsError::ErrorCode14107 => Some(r#"*  make certain any action or Redirect URL's do not loop back to the same TwiML document.
*  make sure you are not inadvertently sending a large quantity of messages to the same phone number, e.g. a script caught in a loop
* if you need this protection disabled for your Twilio Account, [contact Twilio Support](https://www.twilio.com/console/support/tickets/create). Please note, if you choose to have this feature disabled, you must have protection in place in your application to prevent costly auto-reply loops."#),
            TwilioProgrammableSmsError::ErrorCode63002 => None,
            TwilioProgrammableSmsError::ErrorCode30109 => Some(r#"Make sure you submit a fully qualified URL including:

*   protocol (http:// or https://)
*   hostname
*   file path
*   properly url-encoded query parameters"#),
            TwilioProgrammableSmsError::ErrorCode21656 => Some(r#"* Review the [documentation](https://www.twilio.com/docs/content/using-variables-with-content-api) on how to use Content Variables with Content Templates.
* Validate the ContentVariables Parameter, it must be a JSON String and be properly formatted. 
* Make sure all ContentVariables are defined and have a valid definition at time of send.
"#),
            TwilioProgrammableSmsError::ErrorCode57012 => Some(r#"Confirm signature consists of WebhookURL + canonicalized JSON payload and signed with HMAC-SHA1."#),
            TwilioProgrammableSmsError::ErrorCode30102 => Some(r#"Generate and upload a new TLS certificate for the Domain through Twilio Link Shortening console or via Domain Certificate API."#),
            TwilioProgrammableSmsError::ErrorCode63015 => Some(r#"Instruct the recipient to join your Sandbox by sending the join phrase or keyword shown on the Sandbox page in your Twilio Console."#),
            TwilioProgrammableSmsError::ErrorCode92006 => Some(r#"Double check the Content Sid parameter that you are using. Make sure it has the proper formatting and starts with an HX prefix."#),
            TwilioProgrammableSmsError::ErrorCode63032 => Some(r#"Skip sending messages to this user."#),
            TwilioProgrammableSmsError::ErrorCode30005 => Some(r#"The first step to troubleshooting this issue is to attempt to replicate the problems. Attempt to send another test message to this user via a REST API request, or through the API Explorer in the [Twilio Console](https://www.twilio.com/console/runtime/api-explorer/voice). 

Pay close attention to your request and double check to verify you are attempting to send messages to the correct phone number in the correct [E.164](https://www.twilio.com/docs/glossary/what-e164) format: `[+] [country code] [subscriber number including area code]` 

If you see similar results, continue troubleshooting with the following checklist:
*  Is the destination device powered on?
*  Does the device have sufficient signal? If not power the device off, wait 30 seconds, and then power it back up.
*  Is the device connected to the home carrier's network? We cannot guarantee message delivery on devices roaming off-network.
*  Can the device receive non-Twilio SMS?
*  Can the device receive messages from another Twilio number (non-Alphanumeric Sender ID), or with a shorter one-segment (non-concatenated) body?
*  Can other devices using the same mobile carrier receive your messages?

#### Continued issues with Error 30005
Twilio's Support team can help investigate what went wrong with delivering your message. Please collect 3 or more message SIDs in your [SMS logs](https://www.twilio.com/console/sms/logs) from the last 24 hours that were flagged with `Error 30005`, and [open a support request](https://www.twilio.com/console/support/tickets/create)."#),
            TwilioProgrammableSmsError::ErrorCode30110 => Some(r#"Please reach out to Twilio support"#),
            TwilioProgrammableSmsError::ErrorCode14109 => Some(r#"* If you need to generate >10 outbound messages for one incoming message, please use new API requests to create these messages."#),
            TwilioProgrammableSmsError::ErrorCode35114 => Some(r#"Verify that the SendAt parameter is using "[YYYY]-[MM]-[DD]T[HH]:[MM]:[SS]Z" format (in UTC). 

Check the SendAt parameter to ensure it is is greater than 15 minutes (900 seconds) from request time and 35 days (3024000 seconds) in the future."#),
            TwilioProgrammableSmsError::ErrorCode21704 => Some(r#"* Add at least one phone number or short code to your Messaging Service, or enable Alphanumeric Sender ID."#),
            TwilioProgrammableSmsError::ErrorCode21721 => Some(r#"- Verify if the Brand Registration SID has been registered successfully by checking the `status` field using the [Brand API](https://www.twilio.com/docs/sms/a2p-10dlc/onboarding-isv-api#using-get-to-check-brand-registration-status). If the brand has been registered successfully, it will have a status of `APPROVED`. If this is the case, try to import the token again.
- Verify that the Brand Registration SID is of the correct type by checking the `brand_type` field using the [Brand API](https://www.twilio.com/docs/sms/a2p-10dlc/onboarding-isv-api#using-get-to-check-brand-registration-status). The brand should have a type of “STANDARD.” If the `brand_type` is `STARTER`, the brand does not support importing a Campaign Verify token. Once you’ve verified that you are using a standard brand, try to import the token again
"#),
            TwilioProgrammableSmsError::ErrorCode30033 => Some(r#"Please reach out to the Twilio Support team to receive more details around why your campaign has been suspended, and how to resolve the issue."#),
            TwilioProgrammableSmsError::ErrorCode23001 => Some(r#"* To resolve this issue, your application needs to be able to handle [opt-out keyword messages](https://support.twilio.com/hc/en-us/articles/223134027-Twilio-support-for-opt-out-keywords-SMS-STOP-filtering-) from end users.
* Users who reply STOP, STOPALL, UNSUBSCRIBE, CANCEL, END, or QUIT should be added to a blocklist in your application to ensure you do not send them any further messages, unless they opt-in again.
* Users who reply HELP or INFO should be sent an informational message about your application or service.
* Users who reply START, YES, or UNSTOP should be removed from your "STOP" blocklist so they can receive messages from you again.
* Once you are ready to disable Twilio's built-in opt-out handling and process the above keywords in your own application, please reach out to Twilio Support through the [Console](https://console.twilio.com) or [Help Center](https://support.twilio.com) to disable Twilio's built-in opt-out keyword handling for your account."#),
            TwilioProgrammableSmsError::ErrorCode63033 => Some(r#"The user needs to opt back into WhatsApp marketing messages from your business. You may still send templated WhatsApp messages of other categories, such as Authentication and Utility, to this user."#),
            TwilioProgrammableSmsError::ErrorCode30134 => Some(r#"If you're using a root domain, point the A record of the domain to the following 3 IP addresses only. If other IPs are included, Link Shortening does not work.

- 3.233.187.46
- 3.233.108.250
- 54.157.2.211

If you're using a subdomain, point the CNAME record of the subdomain to the following:

- lsct.ashburn.us1.twilio.com"#),
            TwilioProgrammableSmsError::ErrorCode23003 => Some(r#"* Disable Sticky Sender on your Messaging Services using the [Twilio Console](https://www.twilio.com/console/sms/services) or the [REST API](https://www.twilio.com/docs/messaging/services/api).
* If Sticky Sender functionality is needed, implement this functionality in your own application."#),
            TwilioProgrammableSmsError::ErrorCode63010 => None,
            TwilioProgrammableSmsError::ErrorCode21728 => Some(r#"Please fix the error and retry Campaign registration. Please check out our [support article](https://support.twilio.com/hc/en-us/articles/8959909733403-Changes-to-the-A2P-10DLC-Campaign-Creation-Process) to read about the length requirements on these fields."#),
            TwilioProgrammableSmsError::ErrorCode63017 => None,
            TwilioProgrammableSmsError::ErrorCode30035 => Some(r#"While Number Registration often completes within minutes, under certain circumstances, registration time can widely fluctuate depending on the volume of registration requests. Twilio has received your Number Registration request and is working on configuring your number with ecosystem partners. Please [check the status of your number in the Twilio console](https://console.twilio.com/us1/develop/phone-numbers/manage/incoming/a2p-compliance/status?_gl=1*1ttowh2*_ga*MTY4MDY0MDY3NC4xNjkyODkzNTk4*_ga_RRP8K4M4F3*MTY5NzgzNDU4NC42OC4xLjE2OTc4MzQ2MTcuMC4wLjA.&_ga=2.240863811.1963643099.1697477953-1680640674.1692893598). You will only be able to send messages on numbers that are in the “Registered” status. To get your number registered, please check out [A2P 10DLC Number Registration Best Practices](https://support.twilio.com/hc/en-us/articles/19622397178139-A2P-10DLC-Number-Registration-Best-Practices).

NOTE:  Users of multiple phone numbers should note that antiquated phone number practices to improve deliverability, such as using multiple numbers to increase throughput or rotating them between their customers or use cases, undermine the deliverability benefits of A2P 10DLC and can lead to processing delays related to 30035 errors. We recommend reviewing [A2P 10DLC Number Registration Best Practices](https://support.twilio.com/hc/en-us/articles/19622397178139-A2P-10DLC-Number-Registration-Best-Practices)."#),
            TwilioProgrammableSmsError::ErrorCode92003 => Some(r#"Please include a supported language"#),
            TwilioProgrammableSmsError::ErrorCode21727 => Some(r#"Please include all required parameters and retry Campaign registration. Please check out our [support article](https://support.twilio.com/hc/en-us/articles/8959909733403-Changes-to-the-A2P-10DLC-Campaign-Creation-Process) to understand which fields are required for campaign registration. "#),
            TwilioProgrammableSmsError::ErrorCode63026 => None,
            TwilioProgrammableSmsError::ErrorCode23006 => Some(r#"* Ensure all Twilio numbers and Messaging Services on your account are set to use the POST method when handling incoming messages."#),
            TwilioProgrammableSmsError::ErrorCode21714 => Some(r#"* If you are sending to the United States and Canada, use a [Toll-Free or short code number](https://support.twilio.com/hc/en-us/articles/360038173654-Comparison-of-SMS-messaging-in-the-US-and-Canada-for-long-codes-short-codes-and-toll-free-phone-numbers) with higher message throughput (MPS), instead of adding more local numbers. Most mobile carriers in the US and Canada do not not permit [Application-to-Person (A2P)](https://support.twilio.com/hc/en-us/articles/223133807-What-is-P2P-and-A2P-messaging-) messaging via long codes, so there is a high risk of [carrier filtering](https://support.twilio.com/hc/en-us/articles/223181848-How-Does-Carrier-Filtering-Work-) when sending this type of traffic via long code.
* If you are sending to other countries, you may be eligible to have your Number Pool size limit increased. Please [submit a ticket](https://www.twilio.com/console/support/tickets/create) with a description of your use case and why you need a higher number pool size limit. These requests will be reviewed by our Messaging team."#),
            TwilioProgrammableSmsError::ErrorCode30004 => Some(r#"The first step to troubleshooting this issue is to attempt to replicate the problem. Attempt to send another test message to this user via a REST API request or through the API Explorer in the [Twilio Console](https://www.twilio.com/console/runtime/api-explorer/voice). 

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
Twilio's support team can help investigate what went wrong with delivering your message. Please collect 3 or more message SIDs in your [SMS logs](https://www.twilio.com/console/sms/logs) from the last 24 hours that were flagged with Error 30004, and [open a support request](https://www.twilio.com/console/support/tickets/create)."#),
            TwilioProgrammableSmsError::ErrorCode63037 => Some(r#"Twilio will retry this operation."#),
            TwilioProgrammableSmsError::ErrorCode21715 => Some(r#"* The [Active Numbers](https://www.twilio.com/console/phone-numbers/incoming) page in the Twilio Console will display the capabilities of each of the phone numbers in your account.
* Find a new phone number that has SMS and/or MMS capabilities depending on your use case and business needs, and add that phone number to your Messaging Service."#),
            TwilioProgrammableSmsError::ErrorCode63039 => Some(r#"Soften the content of your message and decrease the rate at which messages are being sent to the user outside of the 24-hour session."#),
            TwilioProgrammableSmsError::ErrorCode30453 => Some(r#"You do not need to take any specific action. The disruption in traffic is usually temporary and will resolve in a couple of hours if we do not detect additional fraudulent activity.

#### Continued issues with error 30453
Twilio's support team can help investigate what went wrong with delivering your message. Please collect 3 or more message SIDs in your [SMS logs](https://www.twilio.com/console/sms/logs) from the last 24 hours that were flagged with Error 30453, and [open a support request.](https://www.twilio.com/console/support/tickets/create)

Alternatively, you can use the optional <code style="color : name_color">RiskCheck</code> parameter when [creating a Message](https://www.twilio.com/docs/messaging/api/message-resource#create-a-message-resource) with the [Programmable Messaging API](https://www.twilio.com/docs/messaging/api). To prevent a known/legitimate message from getting blocked in future by SMS Pumping Protection, include the <code style="color : name_color">RiskCheck</code> parameter with value <code style="color : name_color">disable</code> when creating the new Message resource.

You can also mark known/legitimate phone numbers using the Global Safe List feature so that they don’t face any disruption . This provides an additional safety net against false positives, so that the traffic to this number never faces the above scenario. Add known phone numbers to the Safe List by using the [Global Safe List API.](https://www.twilio.com/docs/usage/global-safe-list)"#),
            TwilioProgrammableSmsError::ErrorCode30106 => Some(r#"Finish onboarding."#),
            TwilioProgrammableSmsError::ErrorCode21726 => Some(r#"Contact Twilio support."#),
            TwilioProgrammableSmsError::ErrorCode35115 => Some(r#"Verify that the SendAt parameter is using "[YYYY]-[MM]-[DD]T[HH]:[MM]:[SS]Z" format (in UTC)."#),
            TwilioProgrammableSmsError::ErrorCode30018 => Some(r#"* If your message is marked "undelivered," your message was likely rejected by the mobile network and you will need to register an Alpha Sender ID to improve delivery to that network.
* If your message is marked "delivered," it may have been delivered successfully. However, it is still strongly recommended to register an Alpha Sender ID for the destination country if possible.
* Learn more about Alpha Sender ID registration requirements [here](https://support.twilio.com/hc/en-us/articles/223133767-International-support-for-Alphanumeric-Sender-ID).
* Use [this form](https://twiliodoer.secure.force.com/SenderId) to register a Sender ID for the destination country.
* If you are sending OTP/2FA codes consider using [Twilio Verify or Authy](https://www.twilio.com/en-us/trusted-activation/verify) which will automatically take care of all applicable registrations."#),
            TwilioProgrammableSmsError::ErrorCode30101 => Some(r#"Please verify the Domain you wish to use with Twilio Link Shortening. "#),
            TwilioProgrammableSmsError::ErrorCode30025 => Some(r#"* No action is required from you. 
* To view the warning and errors in the debugger, please visit [here](https://console.twilio.com/us1/monitor/logs/debugger/events?frameUrl=%2Fconsole%2Fdebugger%3F__override_layout__%3Dembed%26quickDate%3D24%26x-target-region%3Dus1%26bifrost%3Dtrue&currentFrameUrl=%2Fconsole%2Fdebugger%3F__override_layout__%3Dembed%26bifrost%3Dtrue%26quickDate%3D24%26x-target-region%3Dus1).
* To view your daily T-Mobile message limit, please visit [Trust Hub - A2P Messaging](https://console.twilio.com/us1/account/trust-hub/a2p?frameUrl=%2Fconsole%2Ftrust-hub%2Fa2p-messaging%3Fx-target-region%3Dus1), select your US A2P brand, and view the  details of your TCR trust score (registered customers only).
* To view your daily T-Mobile usage, please use the Messaging Insights tool."#),
            TwilioProgrammableSmsError::ErrorCode63014 => None
        }
    }
}
