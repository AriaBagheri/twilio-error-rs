// AUTO-GENERATED. DO NOT MODIFY. YOUR CHANGES WILL BE OVER-WRITTEN
use crate::products::TwilioProgrammableMessagingError;
use standard_error::traits::StandardErrorDocsTrait;

impl StandardErrorDocsTrait for TwilioProgrammableMessagingError {
    fn docs(&self) -> &'static str {
        match self {
            TwilioProgrammableMessagingError::ErrorCode20504 => r#"## ERROR - 20504

### Twilio Internal Error

 Twilio's platform encountered an internal error while processing this message.

#### Possible Causes
* If this message was being processed during an incident, it is possible that this message was affected. See [Twilio's status page](https://status.twilio.com/) for ongoing and historical incidents.
* There was an unrecoverable anomaly that occurred while processing this particular message.

#### Possible Solutions
* Try sending the message again.
* If the subsequent attempt also fails with this error, [contact support](https://support.twilio.com/hc/en-us/articles/360048500694-Contacting-Twilio-Support) referencing the affected Message Sids for further troubleshooting."#,
            TwilioProgrammableMessagingError::ErrorCode30884 => r#"## ERROR - 30884

### Campaign vetting rejection - Spam/Phishing

 The campaign submission has been reviewed and it was rejected because it was found to be a known Spam or Phishing campaign.

#### Possible Causes
The campaign cannot be approved because it found to be a known Spam or Phishing campaign.

#### Possible Solutions
The rejected campaign is ineligible for resubmission. If you feel that this was in error, please contact Twilio Customer Support.
 
Additional help resources

* [A2P 10DLC campaign approval best practices](https://support.twilio.com/hc/en-us/articles/11847054539547-A2P-10DLC-Campaign-Approval-Best-Practices)"#,
            TwilioProgrammableMessagingError::ErrorCode35133 => r#"## ERROR - 35133

### Invalid 'Messages' array provided in the request

 The 'Messages' array provided in the request in invalid. It is either empty or contains too many items

#### Possible Causes
The Messages array is either empty or contains too many items

#### Possible Solutions
Check if the Messages array is empty. If not, please ensure it doesn't contain too many items"#,
            TwilioProgrammableMessagingError::ErrorCode30758 => r#"## ERROR - 30758

### Unknown Error Code

 Registration Authority was not provided for brand registration to complete.

#### Possible Causes
Missing Registration Authority

#### Possible Solutions
Update the Registration Authority field. "#,
            TwilioProgrammableMessagingError::ErrorCode21660 => r#"## ERROR - 21660

### Mismatch between the 'From' number and the account

 The `From` number you are using to send an SMS is not associated with your account. 


#### Possible Causes
The number you are using does not belong to the account whose credentials are present in the API request.

#### Possible Solutions
Check that you are using account credentials that correspond with the account that owns the phone number."#,
            TwilioProgrammableMessagingError::ErrorCode30897 => r#"## ERROR - 30897

### Campaign vetting rejection - Disallowed Content

 The campaign submission has been reviewed and it was rejected due to Disallowed Content.

#### Possible Causes
The campaign cannot be approved because it has disallowed content types, such as Loan Marketing, 3rd party debt collection, gambling, sweepstakes, stock alerts, cryptocurrency, risk investments, debt reduction, credit repair, 3rd party lead generation, federally illegal substances.

#### Possible Solutions
The rejected campaign is ineligible for resubmission. If you feel that this was in error, please contact Twilio Customer Support.
 
Additional help resources

* [Forbidden message categories](https://support.twilio.com/hc/en-us/articles/360045004974-Forbidden-message-categories-for-SMS-and-MMS-in-the-US-and-Canada ) 

* [A2P 10DLC campaign approval best practices](https://support.twilio.com/hc/en-us/articles/11847054539547-A2P-10DLC-Campaign-Approval-Best-Practices)"#,
            TwilioProgrammableMessagingError::ErrorCode63036 => r#"## ERROR - 63036

### The specified phone number cannot be reached using RCS at this time.

 The specified phone number cannot be reached using RCS at this time.

#### Possible Causes
- Device is switched off
- Device does not have an internet connection
- Device is not RCS Business Messaging capable

#### Possible Solutions
Turn on the device, connect to internet, and enable RCS and RCS Business Messaging if the device is running iOS ([more details](https://help.twilio.com/articles/29076535334043-RCS-Messaging-Best-Practices-and-FAQ))"#,
            TwilioProgrammableMessagingError::ErrorCode30447 => r#"## ERROR - 30447

### Toll Free verification rejection - phone number not provisioned to Twilio

 The Toll-Free phone number verification submission has been reviewed and it was rejected. Effective Jan 31st, 2024, messaging traffic on this Toll-Free number is blocked until it’s verified.

Verification is one component of ensuring A2P SMS is used by businesses in a legal and compliant manner. The review process looks at the business sending messaging (not the software powering it), the use case, sample content, the web presence, the opt-in/consent collected and associated website terms and privacy policies.

#### Possible Causes
* The toll-free verification was submitted before the toll-free phone number completed porting to Twilio. 
* The toll-free phone number is not provisioned to Twilio.

#### Possible Solutions
* If the number is/was with another messaging provider at the time of verification, please ensure the porting or hosting process has been initiated and is complete. Then Delete the rejected toll-free verification and submit a new request after the port completes.
* If it’s not a porting scenario, contact Twilio Support.

[Toll-Free best practices](https://support.twilio.com/hc/en-us/articles/360038172934-Information-and-best-practices-for-using-Toll-Free-SMS-and-MMS-in-the-US-and-Canada)

[Why was my Toll Free Verification Rejected?](https://support.twilio.com/hc/en-us/articles/9321443984155-Why-Was-My-Toll-Free-Verification-Rejected) "#,
            TwilioProgrammableMessagingError::ErrorCode30444 => r#"## ERROR - 30444

### Toll Free verification rejection - Disallowed: Fraud

 The Toll-Free phone number verification submission has been reviewed and it was rejected.Effective Jan 31st, 2024, messaging traffic on this Toll-Free number is blocked until it’s verified.

Verification is one component of ensuring A2P SMS is used by businesses in a legal and compliant manner. The review process looks at the business sending messaging (not the software powering it), the use case, sample content, the web presence, the opt-in/consent collected and associated website terms and privacy policies.

#### Possible Causes
The toll-free phone number cannot be verified because the submission indicated that the business is associated with or the toll-free verification submission indicated Fraud. 

#### Possible Solutions
If you are confident that your business and messaging use case does not fit into forbidden message categories, contact Twilio Support to provide Additional Information that explains your use case.


[Forbidden Message Categories](https://support.twilio.com/hc/en-us/articles/360045004974-Forbidden-message-categories-for-SMS-and-MMS-in-the-US-and-Canada)

[Why was my Toll Free Verification Rejected?](https://support.twilio.com/hc/en-us/articles/9321443984155-Why-Was-My-Toll-Free-Verification-Rejected) 

[Required information for Toll-free verification](https://support.twilio.com/hc/en-us/articles/13264118705051-Required-Information-for-Toll-Free-Verification)

[Toll-Free best practices](https://support.twilio.com/hc/en-us/articles/360038172934-Information-and-best-practices-for-using-Toll-Free-SMS-and-MMS-in-the-US-and-Canada)

[Twilio’s Messaging Policy](https://www.twilio.com/en-us/legal/messaging-policy)"#,
            TwilioProgrammableMessagingError::ErrorCode30474 => r#"## ERROR - 30474

### Toll Free verification rejection - Need end business

 The Toll-Free phone number verification submission has been reviewed and it was rejected.Effective Jan 31st, 2024, messaging traffic on this Toll-Free number is blocked until it’s verified.

Verification is one component of ensuring A2P SMS is used by businesses in a legal and compliant manner. The review process looks at the business sending messaging (not the software powering it), the use case, sample content, the web presence, the opt-in/consent collected and associated website terms and privacy policies.

#### Possible Causes
The toll-free phone number cannot be verified because the ISV information was submitted instead of the end business. For ISVs or systems & software powering SMS: If your software enables other businesses to send SMS/MMS to their customers, it’s expected that the information provided represents the end-business (your customer) that is engaging with the opted-in consumer, since they have the relationship with the end consumer (the receiver of the messages). 

When you submit a Verification, you need to collect and disclose the information of the business. As a best practice, this should become part of your onboarding process and customer education. Note that there is no trial experience supported on toll-free phone numbers within the industry, so if you have a trial product that includes SMS/MMS, you must collect for a full verification to send messages. 

Submissions that are missing information for the end-business, or are populated with ISV/aggregator information may be rejected. Exceptions may apply when the use case clearly showcases that the ISV manages opt-in mechanisms and is the sole message content creator.

Note: If you are an ISV, prior to submitting a Toll-Free Verification request with Twilio, you will need to create a TrustHub primary Customer Profile for your business within your parent account and get it approved. A primary customer profile must have the business identity set to ISV, and then submitted and approved before registering end businesses. For step-by-step guidance, see [Create a Primary Customer Profile](https://www.twilio.com/docs/trust-hub/trusthub-rest-api/console-create-a-primary-customer-profile).

#### Possible Solutions
Re-submit the toll-free verification with your end-business’s information (address, website, etc)


Resources:

[Toll-free verification for ISVs]( https://support.twilio.com/hc/en-us/articles/13263383206299-Toll-Free-Verification-for-ISVs) 

[Why was my Toll Free Verification Rejected?](https://support.twilio.com/hc/en-us/articles/9321443984155-Why-Was-My-Toll-Free-Verification-Rejected) 

[Required information for Toll-free verification](https://support.twilio.com/hc/en-us/articles/13264118705051-Required-Information-for-Toll-Free-Verification)

[Toll-Free best practices](https://support.twilio.com/hc/en-us/articles/360038172934-Information-and-best-practices-for-using-Toll-Free-SMS-and-MMS-in-the-US-and-Canada)

[Twilio’s Messaging Policy](https://www.twilio.com/en-us/legal/messaging-policy)"#,
            TwilioProgrammableMessagingError::ErrorCode36005 => r#"## ERROR - 36005

### Broadcast File Type Invalid

 Uploaded file is of an invalid type

#### Possible Causes
* You attempted to upload a file that isn't of type .json.gz

#### Possible Solutions
* Upload a file of type .json.gz (G-Zipped JSON)"#,
            TwilioProgrammableMessagingError::ErrorCode30440 => r#"## ERROR - 30440

### Toll Free verification rejection - Unknown Error

 The toll-free phone number cannot be verified due to an unknown error. Messaging traffic on this Toll-Free phone number is blocked.

#### Possible Causes
Unknown

#### Possible Solutions
* Contact [Twilio Customer Support](https://www.twilio.com/console/support/tickets/create)

See [Toll-Free best practices](https://support.twilio.com/hc/en-us/articles/360038172934-Information-and-best-practices-for-using-Toll-Free-SMS-and-MMS-in-the-US-and-Canada) for resources on how to implement Toll-Free messaging.

See [what happens to traffic](https://support.twilio.com/hc/en-us/articles/9321443984155-Why-Was-My-Toll-Free-Verification-Rejected-#:~:text=deceptive%20marketing%20tactics-,What%20happens%20to%20my%20traffic%20if%20my%20Toll%2DFree%20number%20is%20rejected%3F,-If%20your%20Toll) if a Toll-Free verification is rejected"#,
            TwilioProgrammableMessagingError::ErrorCode30901 => r#"## ERROR - 30901

### Campaign rejection - The campaign registration request timed out.

 The campaign registration request timed out due to an error.

#### Possible Causes
Reasons for this failure can be:

* Review was not completed
* Third parties involved in campaign registration did not respond within defined SLA

#### Possible Solutions
Please retry with a new campaign registration."#,
            TwilioProgrammableMessagingError::ErrorCode30749 => r#"## ERROR - 30749

### Brand Registration Failure: Email address duplicate threshold reached

 Brand can not be registered with provided email address since duplicate threshold limit has been reached for it.

#### Possible Causes
An email address can not be used multiple times for Sole Proprietor Brand registrations. It is possible that the Starter customer profile bundle used for Sole Proprietor brand registration contains email address which has been used to register one or more Sole Proprietor Brands already. 

#### Possible Solutions
Please edit the starter customer profile and use a different email address. Once submitted for review, you can resubmit the brand registration request. 

Additional help resources

* [Troubleshooting US A2P Brand registration failures]"#,
            TwilioProgrammableMessagingError::ErrorCode30888 => r#"## ERROR - 30888

### Campaign vetting rejection - Age Gate Not Present / Not Acceptable

 The campaign submission has been reviewed and it was rejected due to Age Gate issues.

#### Possible Causes
The campaign cannot be approved because you indicated that the campaign includes age-gated content, but an Age Gate is not present or is not acceptable on your website and/or opt-in policy.

#### Possible Solutions
Please verify and add a robust age gate to your website or opt-in policy. Resubmit the campaign with the updated age gate information for review.
 
Additional help resources

* [Age Gating](https://www.twilio.com/en-us/legal/messaging-policy#:~:text=Age%20and%20Geographic%20Gating&text=In%20addition%20to%20obtaining%20consent,where%20the%20recipient%20is%20located)

* [A2P 10DLC campaign approval best practices](https://support.twilio.com/hc/en-us/articles/11847054539547-A2P-10DLC-Campaign-Approval-Best-Practices)"#,
            TwilioProgrammableMessagingError::ErrorCode21736 => r#"## ERROR - 21736

### Brand 2FA Failed

 The Brand 2FA attempt failed to validate the Brand Contact Email provided.

#### Possible Causes
* The Brand Contact Email provided is invalid or failed validation.
* Brand Contact Email is missing.
* The 2FA email sent was not actioned on or completed. 

#### Possible Solutions
* Verify that the Brand Contact Email provided is correct. If not, update or add the email via the Brand API or Console.
* Verify if the Brand 2FA email was actioned upon and completed within 7 days. If not, use the following API or Console to re-send the Brand 2FA email."#,
            TwilioProgrammableMessagingError::ErrorCode11321 => r#"## WARNING - 11321

### Misconfigured webhook

 You received a message to one of your Twilio numbers, but we could not send a request to your web application using your provided webhook URL because its format is invalid.


#### Possible Causes
Misconfigured URL for phone number 

#### Possible Solutions
* Verify the URL in the Twilio Console to ensure it is correct and accessible
* Make sure you submit a fully qualified URL including: protocol (http:// or https://), hostname, file path, and properly url-encoded query parameters. 
* Twilio must be able to reach this URL over the public Internet.
"#,
            TwilioProgrammableMessagingError::ErrorCode30480 => r#"## ERROR - 30480

### Toll-Free phone number verification rejection - Disallowed - Phishing

 The Toll-Free phone number verification submission has been reviewed and it was rejected. Messaging traffic on this Toll-Free phone number is blocked.

Verification is one component of ensuring A2P SMS is used by businesses in a legal and compliant manner. The review process looks at the business sending messaging (not the software powering it), the use case, sample content, the web presence, the opt-in/consent collected and associated website terms and privacy policies.

#### Possible Causes
The toll-free phone number cannot be verified because the business or message content was found to be associated with a known Phishing campaign. 

#### Possible Solutions
Contact Twilio Support to provide Additional Information that explains your use case.

Resources: 

[Why was my Toll Free Verification Rejected?](https://support.twilio.com/hc/en-us/articles/9321443984155-Why-Was-My-Toll-Free-Verification-Rejected) 

[Required information for Toll-free verification](https://support.twilio.com/hc/en-us/articles/13264118705051-Required-Information-for-Toll-Free-Verification)

[Toll-Free best practices](https://support.twilio.com/hc/en-us/articles/360038172934-Information-and-best-practices-for-using-Toll-Free-SMS-and-MMS-in-the-US-and-Canada)"#,
            TwilioProgrammableMessagingError::ErrorCode30750 => r#"## ERROR - 30750

### Brand Registration Failure: Mobile phone number duplicate threshold reached

 Brand can not be registered with provided mobile phone number since duplicate threshold limit has been reached for it.

#### Possible Causes
Each Sole Proprietor Brand registration requires a unique active wireless mobile phone number. It is possible that the Sole Proprietor A2P profile bundle used for brand registration contains a mobile number which has been used to register a Sole Proprietor Brand already. 

#### Possible Solutions
Please use a different mobile phone number.
Please edit the Sole Proprietor A2P profile bundle and use a different active wireless mobile number for OTP verification. Once submitted for review, you can resubmit the brand registration request. 

Additional help resources

* [Troubleshooting US A2P Brand registration failures]"#,
            TwilioProgrammableMessagingError::ErrorCode30994 => r#"## ERROR - 30994

### Campaign Registration Failed

 Campaign Registration failed due to it's brand being in a pending state. Additional information may be required for the brand registration to proceed.

#### Possible Causes
It's possible the brand associated with this campaign is awaiting 2FA email verification.

#### Possible Solutions
Check your inbox for a 2FA email verification. If issue persists, please contact support."#,
            TwilioProgrammableMessagingError::ErrorCode63041 => r#"## ERROR - 63041

### Template paused

 WhatsApp has reported that the template has been temporarily paused due to recurring negative feedback from customers. Message templates with this status cannot be sent to customers. For the most updated guidelines, please view [WhatsApp’s documentation](https://developers.facebook.com/docs/whatsapp/message-templates/guidelines/#template-pausing). 

#### Possible Causes
* Negative customer feedback (i.e. multiple customers reporting messages or blocking number)

#### Possible Solutions
* Use a different template
* Edit this template and re-submit for review"#,
            TwilioProgrammableMessagingError::ErrorCode30890 => r#"## ERROR - 30890

### Campaign vetting rejection - Subscriber Help

 The campaign submission has been reviewed and it was rejected because of provided Subscriber Help information.

#### Possible Causes
The campaign cannot be approved because it was indicated that you have a HELP message reply but it does not contain brand name, phone number, or email address.

#### Possible Solutions
Please verify that subscriber help message contains brand name, phone number, or email address. Message needs to guide customers on who them can contact after replying ""help"".
 
Additional help resources

* [A2P 10DLC campaign approval best practices](https://support.twilio.com/hc/en-us/articles/11847054539547-A2P-10DLC-Campaign-Approval-Best-Practices)"#,
            TwilioProgrammableMessagingError::ErrorCode11322 => r#"## WARNING - 11322

### Invalid webhook method

 You received a message to one of your Twilio numbers,  but we could not send a request to your web application using your webhook URL due to an incorrect HTTP method.

#### Possible Causes
Incorrect HTTP method

#### Possible Solutions
* Twilio expects specific HTTP methods (GET or POST) for making a request to your application.
* Check if the URL is configured to handle the correct HTTP method.
"#,
            TwilioProgrammableMessagingError::ErrorCode63047 => r#"## ERROR - 63047

### Link to a sample media file returned an invalid Content-Type

 The link to the media file does not have a Content-Type header set to an accepted MIME type. As a result, the template was not created.

#### Possible Causes
- The web server hosting the media file is not sending Content-Type headers
- The web server hosting the media file is sending an invalid Content-Type header

#### Possible Solutions
- Make sure the web server hosting the sample file returns either `image/jpeg`, `image/png`, `application/pdf`, or `video/mp4` for the `Content-Type` header."#,
            TwilioProgrammableMessagingError::ErrorCode30473 => r#"## ERROR - 30473

### Toll Free verification rejection - Cannot validate business website URL

The Toll-Free phone number cannot be verified because of issues related to your business website. This could be due to: 1. The business website does not load or is not operational; 2. The business website is in a private state that requires a login/password; 3. The business website URL doesn't pertain to the website the consumer is engaging with. Please try to access your business website via incognito mode and see if it is viewable, and confirm the business website URL in the submission is correct and resubmit The Toll-Free phone number verification submission has been reviewed and it was rejected.Effective Jan 31st, 2024, messaging traffic on this Toll-Free number is blocked until it’s verified.

Verification is one component of ensuring A2P SMS is used by businesses in a legal and compliant manner. The review process looks at the business sending messaging (not the software powering it), the use case, sample content, the web presence, the opt-in/consent collected and associated website terms and privacy policies.


#### Possible Causes
The toll-free phone number cannot be verified because the business website submitted is not accessible or available. The business website URL must pertain to the website the consumer is engaging with. In any of the following cases, the business website won’t be reviewable:
* The business website does not load or is not operational
* The business website is in a private state that requires a login/password

#### Possible Solutions
* Check the business website URL in the submission is correct and resubmit if it’s incorrect
* Check that the business website URL is a live website. Resubmit once it is live. 
* Make sure that the business website URL is not in a private state that requires a login/password.. Resubmit after it is corrected.

[Why was my Toll Free Verification Rejected?](https://support.twilio.com/hc/en-us/articles/9321443984155-Why-Was-My-Toll-Free-Verification-Rejected) 

[Required information for Toll-free verification](https://support.twilio.com/hc/en-us/articles/13264118705051-Required-Information-for-Toll-Free-Verification)

[Toll-Free best practices](https://support.twilio.com/hc/en-us/articles/360038172934-Information-and-best-practices-for-using-Toll-Free-SMS-and-MMS-in-the-US-and-Canada)

[Twilio’s Messaging Policy](https://www.twilio.com/en-us/legal/messaging-policy)"#,
            TwilioProgrammableMessagingError::ErrorCode30647 => r#"## ERROR - 30647

### Failed to Upsert Contact

 This error occurs when a contact object fails validation during upsert operations. The error can result from one or more of the following issues: invalid correlation ID, incorrect phone number format, invalid country ISO code, or an improperly formatted postal code. Additionally, this error may also indicate an internal server issue during the upsert process.

#### Possible Causes
**INVALID_CORRELATION_ID:** The correlation_id field is not a valid 32-character UUID. Each correlation_id should be a unique identifier, exactly 32 characters long, to map the response to the original request.

**INVALID_COUNTRY_ISO_CODE:** The country_iso_code is not a valid ISO 3166-1 alpha-2 country code. This field should be a 2-character uppercase string (e.g., "US" for the United States).

**INVALID_CONTACT_ID:** The contact_id field is not in the correct [E.164 format.](http://localhost:4200/docs/glossary/what-e164)

**INVALID_ZIP_CODE:** The zip_code field is not a valid postal code. The zip code should conform to the format used in the corresponding country as indicated by the country_iso_code.

**INTERNAL_SERVER_ERROR:** This error indicates an internal server issue that caused the upsert process to fail. Please retry the request.

#### Possible Solutions
- Ensure the correlation_id is a valid 32-character UUID. Each correlation_id must be unique across all contact objects. You can use a UUID generator if necessary.

- Ensure the contact_id is a valid phone number in [E.164 format.](http://localhost:4200/docs/glossary/what-e164)

- Confirm the country_iso_code: Ensure the country_iso_code follows the ISO 3166-1 alpha-2 standard (e.g., "US").

- Ensure the zip_code matches the postal code format for the specified country.

- If an internal server error occurs, retry the request after some time.


#### Continued issues with error 30647
Twilio's support team can help investigate what went wrong with upserting your contacts. Please collect 3 or more correlation_id from the last 24 hours that were flagged with Error 30647, and [open a support request.](https://www.twilio.com/console/support/tickets/create)"#,
            TwilioProgrammableMessagingError::ErrorCode216602 => r#"## ERROR - 216602

### Content Type is not supported on this channel

 The content types in the template cannot be used on the channel specified

#### Possible Causes
The content type you are using is not available for the channel you are using

#### Possible Solutions
Try to change the content type you are using with a one that it's supported on the channel. Please see this page of our docs to determine what content types are supported on what channels: https://www.twilio.com/docs/content/content-types-overview#channel-support"#,
            TwilioProgrammableMessagingError::ErrorCode30446 => r#"## ERROR - 30446

### Toll Free verification rejection - Opt-in not sufficient: express consent required

The Toll-Free phone number cannot be verified because you're sending marketing messages and the opt-in does not have express consent. Express consent is a clear and direct agreement from the end consumers, which may be given over text, on a written form, via a website, or verbally. The express consent should leave no ambiguity for why the consumer gave you their phone number, and for what type of communication they should expect from you. You must have express consent from each recipient you send a message to. Listing “SMS” in your Terms of Service or Privacy Policy is not Express Consent. The Toll-Free phone number verification submission has been reviewed and it was rejected.Effective Jan 31st, 2024, messaging traffic on this Toll-Free number is blocked until it’s verified.

Verification is one component of ensuring A2P SMS is used by businesses in a legal and compliant manner. The review process looks at the business sending messaging (not the software powering it), the use case, sample content, the web presence, the opt-in/consent collected and associated website terms and privacy policies.

#### Possible Causes
The Opt-in workflow provided in the toll-free verification is not sufficient. Express Consent is required. Express Consent is more than collecting a phone number - it is a voluntary, informed end user choice for a specific purpose - it requires upfront disclosure and intentional action. Explicit opt-in leaves no ambiguity for why the customer gave the phone number, and for how they expect the company will communicate with them.  Every end business must have consent from each recipient they send a message to. Consumers may give permission over text, on a form, on a website, or verbally. Consumers may also give written permission. Listing “SMS” in your Terms of Service or Privacy Policy is not Express Consent. 

Note: If there is promotional messaging on this toll-free phone number, it requires express written consent.

Opt-in refers to the process of getting a consumer’s permission to send them text messages. According to TCPA law, businesses must have "express written consent" from the consumer before texting them. NonConsumer (A2P) Message Senders should: 

* Obtain a Consumer’s consent to receive messages generally; 
* Obtain a Consumer’s express written consent to specifically receive marketing messages; 
* Ensure that Consumers have the ability to revoke consent

#### Possible Solutions
* Correct your opt-in process to collect a consumer’s express consent via CTIA guidelines and resubmit the toll free verification
* Note: An opt-in call-to-action (CTA) with a checkbox must be unchecked by default

[CITIA guidelines](
https://www.ctia.org/the-wireless-industry/industry-commitments/messaging-interoperability-sms-mms)

[Forbidden Message Categories](https://support.twilio.com/hc/en-us/articles/360045004974-Forbidden-message-categories-for-SMS-and-MMS-in-the-US-and-Canada)

[Why was my Toll Free Verification Rejected?](https://support.twilio.com/hc/en-us/articles/9321443984155-Why-Was-My-Toll-Free-Verification-Rejected) 

[Required information for Toll-free verification](https://support.twilio.com/hc/en-us/articles/13264118705051-Required-Information-for-Toll-Free-Verification)

[Toll-Free best practices](https://support.twilio.com/hc/en-us/articles/360038172934-Information-and-best-practices-for-using-Toll-Free-SMS-and-MMS-in-the-US-and-Canada)

[Twilio’s Messaging Policy](https://www.twilio.com/en-us/legal/messaging-policy)"#,
            TwilioProgrammableMessagingError::ErrorCode21737 => r#"## ERROR - 21737

### Campaign Registration Blocked for Non-Compliant Brand

 Campaign registration failed due to the Brand missing a Brand Contact Email. 

#### Possible Causes
* Brand is missing a Brand Contact Email and has not completed the Brand 2FA process which will prevent the creation of new campaigns. 

#### Possible Solutions
* Update the brand registration's BrandContactEmail with a valid email address and complete the Brand 2FA process."#,
            TwilioProgrammableMessagingError::ErrorCode30471 => r#"## ERROR - 30471

### Toll Free verification rejection - High Risk - Non-secured URL in sample message

The Toll-Free phone number cannot be verified because the campaign provided an unsecure URL in the sample message. URLs sent in message content are required to support the "https:" protocol and the sample message needs to reflect that. Please correct the URL with https:// in your sample message and resubmit the verification. The Toll-Free phone number verification submission has been reviewed and it was rejected. Effective Jan 31st, 2024, messaging traffic on this Toll-Free number is blocked until it’s verified.

Verification is one component of ensuring A2P SMS is used by businesses in a legal and compliant manner. The review process looks at the business sending messaging (not the software powering it), the use case, sample content, the web presence, the opt-in/consent collected and associated website terms and privacy policies.

#### Possible Causes
Toll-Free phone number cannot be verified because the campaign provided an unsecure URL in the sample message. URLs sent in message content are required to support the https: protocol and the sample message needs to reflect that.

#### Possible Solutions
* Correct the URL with https:// in your sample message and resubmit the verification
* Ensure that any URL’s sent in messages will be secured with https://

[Why was my Toll Free Verification Rejected?](https://support.twilio.com/hc/en-us/articles/9321443984155-Why-Was-My-Toll-Free-Verification-Rejected) 

[Required information for Toll-free verification](https://support.twilio.com/hc/en-us/articles/13264118705051-Required-Information-for-Toll-Free-Verification)

[Toll-Free best practices](https://support.twilio.com/hc/en-us/articles/360038172934-Information-and-best-practices-for-using-Toll-Free-SMS-and-MMS-in-the-US-and-Canada)

[Twilio’s Messaging Policy](https://www.twilio.com/en-us/legal/messaging-policy)"#,
            TwilioProgrammableMessagingError::ErrorCode21731 => r#"## ERROR - 21731

### Cannot perform operation on suspended brand

 When an A2P 10DLC Brand is in a suspended state, the following operations are not allowed:
• Create a new campaign - when you try to create a new campaign using a suspended brand, this error code is returned
• Update brand - when you try to update a suspended brand, this error code is returned


#### Possible Causes
Your Brand has been suspended because of one or more violations against the US A2P 10DLC policies.

#### Possible Solutions
The Twilio team should be reaching out to you to provide guidance on how to fix the suspended Brand. Please check your email or the Twilio Support Center. If you don’t see anything, please raise a ticket with the Twilio Support team."#,
            TwilioProgrammableMessagingError::ErrorCode30460 => r#"## ERROR - 30460

### Toll Free verification rejection - Disallowed: Third Party Debt Collection

 The Toll-Free phone number verification submission has been reviewed and it was rejected. Effective Jan 31st, 2024, messaging traffic on this Toll-Free number is blocked until it’s verified.

Verification is one component of ensuring A2P SMS is used by businesses in a legal and compliant manner. The review process looks at the business sending messaging (not the software powering it), the use case, sample content, the web presence, the opt-in/consent collected and associated website terms and privacy policies.

#### Possible Causes
The toll-free phone number cannot be verified because the submission indicated that the business is associated with or the toll-free verification submission indicated a forbidden messaging category (Debt collection or forgiveness: Third party debt collection). Third party debt collection is not permitted, consent must be obtained directly from end-users. 

"Third-party debt collection" means originating from any party other than the one who is owed the debt. For example, a hospital could send messages regarding bills for its own patients, assuming they provided opt-in to receive that messaging.

Debt consolidation, debt reduction and credit repair programs are prohibited regardless if there is first-party consent. 

Note: While third party debt collection is not permitted, a debt collection business that has direct consent from a consumer to send related content may do so. 

#### Possible Solutions
If you are confident that your business and messaging use case does not fit into forbidden message categories, contact Twilio Support to provide Additional Information that explains your use case.

[Forbidden Message Categories](https://support.twilio.com/hc/en-us/articles/360045004974-Forbidden-message-categories-for-SMS-and-MMS-in-the-US-and-Canada)

[Why was my Toll Free Verification Rejected?](https://support.twilio.com/hc/en-us/articles/9321443984155-Why-Was-My-Toll-Free-Verification-Rejected) 

[Required information for Toll-free verification](https://support.twilio.com/hc/en-us/articles/13264118705051-Required-Information-for-Toll-Free-Verification)

[Toll-Free best practices](https://support.twilio.com/hc/en-us/articles/360038172934-Information-and-best-practices-for-using-Toll-Free-SMS-and-MMS-in-the-US-and-Canada)

[Twilio’s Messaging Policy](https://www.twilio.com/en-us/legal/messaging-policy)"#,
            TwilioProgrammableMessagingError::ErrorCode36001 => r#"## ERROR - 36001

### Broadcast SID Not Found

 The Broadcast SID entered was not found

#### Possible Causes
* You provided an empty Broadcast SID
* You provided a Broadcast SID that does not exist

#### Possible Solutions
* Provide a non-empty, existing Broadcast SID"#,
            TwilioProgrammableMessagingError::ErrorCode30754 => r#"## ERROR - 30754

### Brand Registration Failure: Invalid postal address

 You have used an invalid postal address for brand registration.

#### Possible Causes
You have used an invalid address for brand registration.

#### Possible Solutions
Please ensure 'street', 'region', 'state', 'postal_code', and 'iso_country' collectively forms a valid postal address.

Additional help resources

* [Troubleshooting US A2P Brand registration failures]"#,
            TwilioProgrammableMessagingError::ErrorCode20410 => r#"## ERROR - 20410

### Gone

 The API is no longer available

#### Possible Causes
* You are  attempting to send messages via a discontinued API
* You are sending messages via the deprecated “SMS/Messages” endpoint


#### Possible Solutions
* Ensure you are using the most up to date Messaging Resources. See the [Programmable Messaging API docs](https://www.twilio.com/docs/messaging/api) for up to date endpoints.
* Migrate from the “SMS/Messages” endpoint to “Messages” endpoint. See [here](https://support.twilio.com/hc/en-us/articles/223181028-Switching-from-SMS-Messages-resource-URI-to-Messages-resource-URI) for more information."#,
            TwilioProgrammableMessagingError::ErrorCode30646 => r#"## ERROR - 30646

### Failed to Upsert Consent

 This error occurs when one or more fields in a consent object are invalid during the upsert process, or due to an internal server issue. The contact's consent status may fail validation due to issues with the sender ID, opt-in status, or the source from which the consent was collected.

#### Possible Causes
**INVALID_CORRELATION_ID:** The correlation_id field is not a valid 32-character UUID. Each correlation_id should be a unique identifier, exactly 32 characters long, to map the response to the original request.

**INVALID_SENDER_ID:** The sender_id field is invalid. It must be either a valid Messaging Service SID or a from phone number. Ensure the sender_id is correctly formatted.

**INVALID_CONTACT_ID:** The contact_id field is not in the correct [E.164 format.](http://localhost:4200/docs/glossary/what-e164)

**INVALID_STATUS:** The status field is invalid. It must be one of the following values: opt-in or opt-out.

**INVALID_SOURCE:** The source field is invalid. It must be one of the following values: website, offline, opt-in-message, opt-out-message, or others.

**INTERNAL_SERVER_ERROR:** The request failed due to an internal server error during the upsert process. Retry the request.

#### Possible Solutions
- Ensure the correlation_id is a valid 32-character UUID. Each correlation_id must be unique across all contact objects. You can use a UUID generator if necessary.

- Ensure the sender_id is either a valid Messaging Service SID or a from phone number.

- Ensure the contact_id is a valid phone number in [E.164 format.](http://localhost:4200/docs/glossary/what-e164)

- Ensure the status field contains a valid value (opt-in or opt-out).

- Ensure the source field is one of the accepted values (website, offline, opt-in-message, opt-out-message, others).

- If an internal server error occurs, retry the request after some time.

#### Continued issues with error 30646
Twilio's support team can help investigate what went wrong with upserting your contacts. Please collect 3 or more correlation_id from the last 24 hours that were flagged with Error 30646, and [open a support request.](https://www.twilio.com/console/support/tickets/create)"#,
            TwilioProgrammableMessagingError::ErrorCode36004 => r#"## ERROR - 36004

### Error Retrieving File

 File couldn't be retrieved, make sure it has 'file' key

#### Possible Causes
* You did not specify a key with the name "file" in the Broadcast Upload request body

#### Possible Solutions
* Attach a .json.gz file as the value to the "file" key in the Broadcast Upload request body"#,
            TwilioProgrammableMessagingError::ErrorCode30610 => r#"## ERROR - 30610

### Message couldn't be delivered

 Message couldn’t be delivered to the recipients number you are trying to reach at this time due to local regulatory restrictions.

#### Possible Causes
Marketing, promotional & other non essential messages are restricted under the local regulatory requirement (e.g.TCPA in US) to certain hours of the day. Specifically, these communications cannot be made before 8 a.m. or after 9 p.m. local time at the recipient's location. This restriction is intended to protect consumers from intrusive communications during times they are likely to be resting or engaged in personal activities.

#### Possible Solutions
**Verify Local Time:** Implement a system to accurately determine the local time zone of each recipient based on their area code and address data.

**Schedule Adjustments:** Configure your messaging campaign schedules to operate only within the permitted hours of 8 a.m. to 9 p.m. local time for each recipient.

**Update recipients location:** Configure your recipients location using this [API](https://www.twilio.com/docs/messaging/features/bulk-upsert-contacts-api), incase recipients location is different & doesn’t currently fall under 9 p.m. to 8 a.m.


#### Continued issues with error 30610

Twilio's support team can help investigate what went wrong with delivering your message. Please collect 3 or more message SIDs in your SMS [logs](https://www.twilio.com/console/sms/logs) from the last 24 hours that were flagged with Error 30610, and [open a support request.](https://www.twilio.com/console/support/tickets/create)"#,
            TwilioProgrammableMessagingError::ErrorCode21659 => r#"## ERROR - 21659

### 'From' is not a Twilio phone number or Short Code country mismatch

 You can only send SMS messages from a phone number, Alphanumeric Sender ID or [short code](https://www.twilio.com/docs/glossary/what-is-a-short-code) provided by or ported to Twilio. 

For Short Codes, the `From` number must be in the same country as the `To` number.


#### Possible Causes
* The number you are using is in the process of porting/hosting.
* You have provided an incorrect `From` number. 
* The number may be formatted incorrectly. Twilio accepts numbers in [E.164 format](https://www.twilio.com/docs/glossary/what-e164)
* If the number is a Short Code, it must be associated with the same country as the destination address.



#### Possible Solutions
* If you have ported/hosted the `From` number, ensure that the process is complete. You can follow up on this in the Twilio console for [porting](https://console.twilio.com/us1/develop/phone-numbers/port-host/porting-requests?frameUrl=%2Fconsole%2Fphone-numbers%2Fporting-requests%3Fx-target-region%3Dus1) and [hosting](https://console.twilio.com/us1/develop/phone-numbers/port-host/hosted-numbers?frameUrl=%2Fconsole%2Fphone-numbers%2Fhosted%3Fx-target-region%3Dus1).
* If you are sending from a Short Code, verify that the country you are sending to matches the country of the Short Code.
* Ensure the number you are using is in [E.164 format](https://www.twilio.com/docs/glossary/what-e164).

"#,
            TwilioProgrammableMessagingError::ErrorCode30734 => r#"## ERROR - 30734

### Brand Registration Failure: Sole Proprietor brands are not enabled

 Sole proprietor brand registration is not enabled for your account.

#### Possible Causes
Sole proprietor brand registration is not enabled for your account.

#### Possible Solutions
Please contact Twilio Support.

Additional help resources

* [Troubleshooting US A2P Brand registration failures]"#,
            TwilioProgrammableMessagingError::ErrorCode30475 => r#"## ERROR - 30475

### Toll Free verification rejection - Opt-in not sufficient: Cannot combine consent for messaging with requirement for service

The Toll-Free phone number cannot be verified because your opt-in mechanism requires that the consumer opt-in to messaging as a requirement of using your service, which is not allowed. Consent cannot be a requirement for your business's service. A business may not combine collecting a consumer's phone number with opting-in to receive marketing messaging. Please ensure your opt-in mechanism is a distinct action during a consumer's sign up for your service. The Toll-Free phone number verification submission has been reviewed and it was rejected. Effective Jan 31st, 2024, messaging traffic on this Toll-Free number is blocked until it’s verified.

Verification is one component of ensuring A2P SMS is used by businesses in a legal and compliant manner. The review process looks at the business sending messaging (not the software powering it), the use case, sample content, the web presence, the opt-in/consent collected and associated website terms and privacy policies.


#### Possible Causes
Your opt-in mechanism requires that the consumer opt-in to messaging as a requirement of using your service, which is not allowed.

Consent cannot be a requirement for your business’s service. A business may not combine collecting a consumer’s phone number with opting-in to receive marketing messaging. An explicit opt-in leaves no ambiguity for why the customer gave the phone number, and for how they expect the company will communicate with them. 

Opt-in refers to the process of getting a consumer’s permission to send them text messages. According to TCPA law, businesses must have "express written consent" from the consumer before texting them. NonConsumer (A2P) Message Senders should: 
* Obtain a Consumer’s consent to receive messages generally; 
* Obtain a Consumer’s express written consent to specifically receive marketing messages; and 
* Ensure that Consumers have the ability to revoke consent.

Note: If there is promotional messaging on this toll-free phone number, it requires express written consent.

#### Possible Solutions
* Ensure your opt-in mechanism is a distinct action during a consumer’s sign up for your service. 
* An opt-in call-to-action (CTA) with a checkbox must be unchecked by default.

Resources: 

[CTIA guidelines](https://www.ctia.org/the-wireless-industry/industry-commitments/messaging-interoperability-sms-mms)

[Consent/Opt-In](https://www.twilio.com/en-us/legal/messaging-policy#:~:text=with%20Twilio%20policies.-,Consent%20/%20Opt%2Din,-What%20Is%20Proper) in [Twilio’s Messaging Policy](https://www.twilio.com/en-us/legal/messaging-policy)

[Why was my Toll Free Verification Rejected?](https://support.twilio.com/hc/en-us/articles/9321443984155-Why-Was-My-Toll-Free-Verification-Rejected) 

[Required information for Toll-free verification](https://support.twilio.com/hc/en-us/articles/13264118705051-Required-Information-for-Toll-Free-Verification)

[Toll-Free best practices](https://support.twilio.com/hc/en-us/articles/360038172934-Information-and-best-practices-for-using-Toll-Free-SMS-and-MMS-in-the-US-and-Canada)
"#,
            TwilioProgrammableMessagingError::ErrorCode30993 => r#"## ERROR - 30993

### Campaign Registration Failed

 Campaign's CNP Migration did not complete in expected time!

#### Possible Causes
It is possible that all the necessary elections for the Campaign's CNP migration process to complete, were not done in time. 

#### Possible Solutions
Please trigger the migration process again and register the campaign with Twilio's ERC api. If the issue persists, please contact support."#,
            TwilioProgrammableMessagingError::ErrorCode30903 => r#"## ERROR - 30903

### Campaign rejection - Incorrect Sole Prop Brand Registration

 The campaign for a Sole Proprietorship Brand has been rejected due to incorrect registration and failure to meet the small business Sole Proprietor (EIN) criteria set by TCR and mobile carriers.

#### Possible Causes
Reasons for this failure can be:

* Failure to meet the specified criteria for Sole Proprietor registration, entities with EINs should be registered as a Standard Brand.

* Incorrect or incomplete registration information provided during the brand registration process.

* Inconsistencies or discrepancies in the provided information.

#### Possible Solutions
* Verify the accuracy and completeness of the registration information for the Sole Proprietorship Brand.
* Ensure that the brand meets the criteria defined by TCR and mobile carriers for Sole Proprietor (EIN) registration.
* If the brand does not qualify as a Sole Proprietorship, consider registering it as a standard brand according to the appropriate guidelines and requirements.
* Register a standard or acceptable campaign use case that aligns with the registered brand's classification."#,
            TwilioProgrammableMessagingError::ErrorCode30701 => r#"## ERROR - 30701

### Brand Registration Failure: Invalid input parameters

 Brand registration request contained one or more invalid inputs. 

#### Possible Causes
The fields provided in the error message failed validations. Most string parameters are subject to min and/or max length validations. Some parameters are subject to additional country-specific validation rules, including business_registration_number, state, and postal_code. Please refer to the following table for validation details.

#### Validation Rules

|Property|Description|Validation|
|--- |--- |--- |
|friendly_name|Brand/Marketing/DBA name of the business|Max length 255|
|business_name|The legal name of the business|Max length 255|
|business_registration_number|Tax ID of the business|Max length 21|
|first_name|First name of business contact|Max length 100|
|last_name|Last name of business contact|Max length 100|
|phone_number|The support contact telephone in e.164 format. E.g. +12023339999|Max length 20|
|mobile_phone_number|Valid wireless telephone in e.164 format. E.g. +12023339999.|Max length 20|
|street|Street name and house number. E.g. 1000 Sunset Hill Road|Max length 100|
|city|City name|Max length 100|
|region|State, region or province. For the United States, please use 2 character codes. E.g. 'CA' for California.|Max length 20|
|postal_code|Zipcode or postal code. E.g. 21012|Max length 10|
|iso_country|2 letter ISO-2 country code. E.g. US, CA Length 2||
|email|The email address of support contact|Max length 100|
|stock_ticker|The stock symbol of the brand|Max length 10|
|stock_exchange|The stock exchange of the brand|Enumerated values: "NONE", "NASDAQ", "NYSE", "AMEX", "AMX", "ASX", "B3", "BME", "BSE", "FRA", "ICEX", "JPX", "JSE", "KRX", "LON", "NSE", "OMX", "SEHK", "SGX", "SSE", "STO", "SWX", "SZSE", "TSX", "TWSE", "VSE", "OTHER"|
|website_url|The website/online presence of the business|Max length 100|
|business_industry|The segment in which the business operates in|Enumerated values: "AUTOMOTIVE", "AGRICULTURE", "BANKING", "CONSUMER", "EDUCATION", "ENGINEERING", "ENERGY", "OIL_AND_GAS", "FAST_MOVING_CONSUMER_GOODS", "FINANCIAL", "FINTECH", "FOOD_AND_BEVERAGE", "GOVERNMENT", "HEALTHCARE", "HOSPITALITY", "INSURANCE", "LEGAL", "MANUFACTURING", "MEDIA", "ONLINE", "RAW_MATERIALS", "REAL_ESTATE", "RELIGION", "RETAIL", "JEWELRY", "TECHNOLOGY", "TELECOMMUNICATIONS", "TRANSPORTATION", "TRAVEL", "ELECTRONICS", "NOT_FOR_PROFIT"|


#### Possible Solutions
Please check your inputs for the fields in the error message and confirm that they meet the validations listed in possible causes.

Additional help resources

* [Troubleshooting US A2P Brand registration failures](https://www.twilio.com/docs/messaging/compliance/a2p-10dlc/troubleshooting-a2p-brands)"#,
            TwilioProgrammableMessagingError::ErrorCode21267 => r#"## ERROR - 21267

### Alphanumeric Sender ID cannot be used as the 'From' number on trial accounts

 You attempted to send an SMS from a Trial Account using an Alphanumeric Sender ID as the `From` parameter.


#### Possible Causes
You are trying to use an Alphanumeric Sender ID on a Trial Account. Trial accounts [cannot send from Alphanumeric Sender IDs](https://www.twilio.com/docs/messaging/guides/how-to-use-your-free-trial-account#sending-sms-and-mms-messages)



#### Possible Solutions
If you are using an Alphanumeric Sender ID on a Trial Account, [upgrade your account](https://www.twilio.com/docs/messaging/guides/how-to-use-your-free-trial-account#how-to-upgrade-your-account).
"#,
            TwilioProgrammableMessagingError::ErrorCode21732 => r#"## ERROR - 21732

### Campaign limit reached on the Brand

 You can only set up one Sole Proprietor Campaign per Sole Proprietor Brand. Please note that campaigns in all statuses (such as Pending, Registered, Failed) count towards this limit.

#### Possible Causes
Your request to create a Campaign has failed because there’s already an existing Campaign associated with this Sole Proprietor Brand.

#### Possible Solutions
Please delete the existing Campaign first and then retry."#,
            TwilioProgrammableMessagingError::ErrorCode30445 => r#"## ERROR - 30445

### Toll Free verification rejection - Could not validate business information

The Toll-Free phone number cannot be verified because of invalid information. Please check and confirm that all the business information submitted  accurately represent the business that the end consumer has opted in to receive messages from. Please make necessary edits and resubmit. The Toll-Free phone number verification submission has been reviewed and it was rejected. Effective Jan 31st, 2024, messaging traffic on this Toll-Free number is blocked until it’s verified.

#### Possible Causes
The Toll-Free phone number cannot be verified because of invalid information. The Business Information such as business name or address that was submitted could not be verified or the website URL could be not validated because it is not accessible or available. Or the ISV contact information was provided instead of its customer’s contact information. 

#### Possible Solutions
* Ensure all business information fields are valid and accurately represent the business who manages the opt-in and has the relationship with the consumer:

  -- BusinessName

  -- BusinessStreetAddress

  -- BusinessCity

  -- BusinessStateProvinceRegion

  -- BusinessPostalCode

  -- BusinessCountry

  -- BusinessContactFirstName

  -- BusinessContactLastName

  -- BusinessContactEmail

  -- BusinessContactPhone

  -- BusinessWebsite

- Correct the business information fields on toll-free verification submission by editing it and then resubmit it. 


[Forbidden Message Categories](https://support.twilio.com/hc/en-us/articles/360045004974-Forbidden-message-categories-for-SMS-and-MMS-in-the-US-and-Canada)

[Why was my Toll Free Verification Rejected?](https://support.twilio.com/hc/en-us/articles/9321443984155-Why-Was-My-Toll-Free-Verification-Rejected) 

[Required information for Toll-free verification](https://support.twilio.com/hc/en-us/articles/13264118705051-Required-Information-for-Toll-Free-Verification)

[Toll-Free best practices](https://support.twilio.com/hc/en-us/articles/360038172934-Information-and-best-practices-for-using-Toll-Free-SMS-and-MMS-in-the-US-and-Canada)

[Twilio’s Messaging Policy](https://www.twilio.com/en-us/legal/messaging-policy)"#,
            TwilioProgrammableMessagingError::ErrorCode21266 => r#"## ERROR - 21266

### 'To' and 'From' numbers cannot be the same

 You attempted to send an SMS, but the `To` phone number you supplied is the same as the `From` number.


#### Possible Causes
You attempted to send an SMS from a Twilio number to itself (i.e. putting the same Twilio number in the `To` and `From` parameters).

#### Possible Solutions
Ensure you are not attempting to send a message from a Twilio number to itself.
"#,
            TwilioProgrammableMessagingError::ErrorCode30757 => r#"## ERROR - 30757

### Brand Registration Failure: Missing Business Registration Number 

 Business registration number was not provided for brand registration to complete.

#### Possible Causes
Missing Business Registration Number

#### Possible Solutions
Update the Business Registration Number field. "#,
            TwilioProgrammableMessagingError::ErrorCode30456 => r#"## ERROR - 30456

### Toll Free verification rejection - Disallowed: SHAFT - Alcohol

 The Toll-Free phone number verification submission has been reviewed and it was rejected. Effective Jan 31st, 2024, messaging traffic on this Toll-Free number is blocked until it’s verified.

Verification is one component of ensuring A2P SMS is used by businesses in a legal and compliant manner. The review process looks at the business sending messaging (not the software powering it), the use case, sample content, the web presence, the opt-in/consent collected and associated website terms and privacy policies.

#### Possible Causes
The toll-free phone number cannot be verified because the submission indicated that the business is associated with or the toll-free verification submission indicated a forbidden messaging category (SHAFT: Alcohol).

Alcohol traffic is allowed on Toll Free, Short Code, and Long Code in the US, as long as proper age gating procedures are in place. Age gating means that website users must input their date of birth. It cannot be a yes or no question.

All age-gated content into Canada must be blocked across Toll Free, Short Code, and Long Code. The only way to send age-gated traffic into Canada (even with proper age-gating) is to receive a special carrier exemption. Allowed age gated content in Canada include: pocket knives, lighters, and non-alcoholic beverages.

#### Possible Solutions
Add an Age Gate for your US business website. Resubmit the verification and provide proof in your opt-in workflow documentation that you have measures in place to ensure compliance.

[Forbidden Message Categories](https://support.twilio.com/hc/en-us/articles/360045004974-Forbidden-message-categories-for-SMS-and-MMS-in-the-US-and-Canada)

[Why was my Toll Free Verification Rejected?](https://support.twilio.com/hc/en-us/articles/9321443984155-Why-Was-My-Toll-Free-Verification-Rejected) 

[Required information for Toll-free verification](https://support.twilio.com/hc/en-us/articles/13264118705051-Required-Information-for-Toll-Free-Verification)

[Toll-Free best practices](https://support.twilio.com/hc/en-us/articles/360038172934-Information-and-best-practices-for-using-Toll-Free-SMS-and-MMS-in-the-US-and-Canada)

[Twilio’s Messaging Policy](https://www.twilio.com/en-us/legal/messaging-policy)"#,
            TwilioProgrammableMessagingError::ErrorCode63051 => r#"## ERROR - 63051

### WhatsApp Business Account is Locked

 Twilio cannot send the message because WhatsApp has locked the WhatsApp Business Account that contains this Sender due to an integrity policy violation

#### Possible Causes
Violating the WhatsApp Terms of Service or Commerce Policy

#### Possible Solutions
If you believe this is in error, please open a support ticket for us to appeal to WhatsApp on your behalf."#,
            TwilioProgrammableMessagingError::ErrorCode11100 => r#"## ERROR - 11100

### Invalid URL format

 The format of the provided URL is invalid.


#### Possible Causes
*   Bad URL for phone number configuration
*   Bad URL passed to outgoing call REST request
*   Bad URL in Play or Redirect Verb Body
*   Bad URL provided for a verb's action attribute
*   No URL provided in Record verb action attribute when modifying a live call
*   Unsupported characters in auth portion of URL

#### Possible Solutions
* Make sure you submit a fully qualified URL including: protocol (http:// or https://), hostname, file path, properly url-encoded query parameters.
* Twilio must be able to reach this URL over the public Internet."#,
            TwilioProgrammableMessagingError::ErrorCode63046 => r#"## WARNING - 63046

### Template approved

 WhatsApp template approval status has changed to approved.

#### Possible Causes
WhatsApp has reported the template is now approved.

#### Possible Solutions
No further action needed."#,
            TwilioProgrammableMessagingError::ErrorCode30908 => r#"## ERROR - 30908

### Campaign vetting rejection - Compliant Privacy Policy Required

 The campaign submission has been reviewed and rejected because a compliant privacy policy can not be verified.

#### Possible Causes
* *Missing Privacy Policy*: A compliant privacy policy was not located on the website provided or Message Flow during the review process.
* *Policy Inconsistencies*: Discrepancies in the provided privacy policy such as multiple privacy policies.
* *Mobile Information Sharing*: Privacy policy indicates end-user mobile information is shared with third parties/affiliates.


#### Possible Solutions
1. Verify privacy policy is accessible to end-users and meets the compliance standards for the collection and use of mobile phone numbers for messaging. 
2. Include a direct link to the privacy policy within the Message Flow.
Once you have made a change to address the privacy policy error, please resubmit the campaign for review.
3. Ensure that Privacy Policy states that no mobile information will be shared with third parties/affiliates for marketing/promotional purposes. 

Additional help resources
* [A2P 10DLC campaign approval best practices](https://support.twilio.com/hc/en-us/articles/11847054539547-A2P-10DLC-Campaign-Approval-Best-Practices)

"#,
            TwilioProgrammableMessagingError::ErrorCode21703 => r#"## ERROR - 21703

### The Messaging Service does not have a phone number available to send a message

 You have phone numbers or short codes in your Messaging Service, but none of them are capable of sending the requested message or media to this recipient.
Possible causes vary depending on the type of message and the country of the destination number. Below are several common causes of this error.

#### Possible Causes
• You attempted to send a message to a United States or Canada mobile number, but you do not have any US/Canada numbers or short codes in your Messaging Service. US/Canada mobile numbers are not reachable from Twilio numbers from outside the US/Canada, due to limitations imposed by carriers.**

• You only have a short code number in your Messaging Service, and the recipient is not reachable from your short code (for example, the To number is from a different country than your short code, or is on a [carrier that does not support short code messaging](https://support.twilio.com/hc/en-us/articles/223182088-What-carriers-are-supported-on-Twilio-short-codes-)).

• You attempted to send an MMS message, but you do not have any US/Canada long code numbers or an [MMS-enabled short code](https://support.twilio.com/hc/en-us/articles/223134667-Sending-receiving-MMS-messages-with-short-codes) in your Messaging Service.**

• You only have an [Alphanumeric Sender ID](https://support.twilio.com/hc/en-us/articles/223181348-Getting-started-with-Alphanumeric-Sender-ID) in your Messaging Service, and the To number is in a [country where Alphanumeric Sender ID is unsupported](https://support.twilio.com/hc/en-us/articles/223133767-International-support-for-Alphanumeric-Sender-ID).

• You attempted to send a message to a United States mobile number, but you do not have any 10DLC numbers that are registered with a verified A2P Campaign in the Messaging Service. Only numbers registered with a verified A2P Campaign can be allowed to send to US based numbers. 

** If you have recently added the number to a Messaging Service and while the number has a “Pending Registration” status, you may be experiencing a known issue which disables the ability to benefit from a temporary exception Twilio has made available to sending messages from “Pending Registration” numbers.  You can reattempt to send from the impacted number by including the number in the "From" parameter of your API request instead of using the Messaging Service SID. However, this exception will sunset in the coming weeks.  For more information about the 21703 error in relation to numbers “Pending Registration”, and processing times to complete the Number Registration going forward, please review updated [A2P 10DLC Number Registration Best Practices](https://support.twilio.com/hc/en-us/articles/19622397178139-A2P-10DLC-Number-Registration-Best-Practices). To check the status of your numbers at any given time, you can use the Number Registration CSV report as described in [How do I check that I have completed US A2P 10DLC registration](https://support.twilio.com/hc/en-us/articles/4418081745179-How-do-I-check-that-I-have-completed-US-A2P-10DLC-registration-).

#### Possible Solutions
Add a sender to your Messaging Service with the required capabilities, using the [Twilio Console](https://www.twilio.com/console/sms/services) or the [REST API](https://www.twilio.com/docs/messaging/services/api/phonenumber-resource#create-a-phonenumber-resource-add-a-phone-number-to-a-messaging-service)."#,
            TwilioProgrammableMessagingError::ErrorCode30463 => r#"## ERROR - 30463

### Toll Free verification rejection - Disallowed: Stock Alerts/Platforms

 The Toll-Free phone number verification submission has been reviewed and it was rejected. Effective Jan 31st, 2024, messaging traffic on this Toll-Free number is blocked until it’s verified.

Verification is one component of ensuring A2P SMS is used by businesses in a legal and compliant manner. The review process looks at the business sending messaging (not the software powering it), the use case, sample content, the web presence, the opt-in/consent collected and associated website terms and privacy policies.

#### Possible Causes
The toll-free phone number cannot be verified because the submission indicated that the business is associated with or the toll-free verification submission indicated a forbidden messaging category (High-risk financial services: Stock alerts/stock platforms). 

No businesses that solely operate in stocks, investing, or cryptocurrency are allowed to send SMS traffic. If there is a mixed use case where that is a partial aspect of the business it may be approved based on the other use case content.

Note, however, that stock platforms are allowed a two-factor authentication (2FA) messaging use case. 

#### Possible Solutions
If you are confident that your business and messaging use case does not fit into forbidden message categories, contact Twilio Support to provide Additional Information that explains your use case.

[Forbidden Message Categories](https://support.twilio.com/hc/en-us/articles/360045004974-Forbidden-message-categories-for-SMS-and-MMS-in-the-US-and-Canada)

[Why was my Toll Free Verification Rejected?](https://support.twilio.com/hc/en-us/articles/9321443984155-Why-Was-My-Toll-Free-Verification-Rejected) 

[Required information for Toll-free verification](https://support.twilio.com/hc/en-us/articles/13264118705051-Required-Information-for-Toll-Free-Verification)

[Toll-Free best practices](https://support.twilio.com/hc/en-us/articles/360038172934-Information-and-best-practices-for-using-Toll-Free-SMS-and-MMS-in-the-US-and-Canada)

[Twilio’s Messaging Policy](https://www.twilio.com/en-us/legal/messaging-policy)"#,
            TwilioProgrammableMessagingError::ErrorCode30895 => r#"## ERROR - 30895

### Campaign vetting rejection - Direct Lending - Campaign and Content Attribute Error

 The campaign cannot be approved because it appears to be associated with direct lending or loan arrangement, and the necessary content attribute was not indicated

#### Possible Causes
The campaign may involve direct lending or loan arrangements, and this was not accurately reflected in the content attribute during registration.

#### Possible Solutions
1. Verify Direct Lending Indicated in Campaign Description: If the campaign involves direct lending or loan arrangement or is associated with a financial services organization, ensure "DIRECT LENDING" is mentioned in campaign description. Please mention this even if the specified campaign use case does not directly relate to loan offering (e.g. 2FA).
2. Update Campaign Description if Needed: If the campaign is not related to direct lending or loan arrangement, update the campaign description to reflect the actual content and purpose. If the campaign is related to a financial services organization, update the campaign descripton to contain "DIRECT LENDING".

Once you have made a change to address the issue, please resubmit the campaign for review.

 
Additional help resources

* [Forbidden message categories](https://support.twilio.com/hc/en-us/articles/360045004974-Forbidden-message-categories-for-SMS-and-MMS-in-the-US-and-Canada)
* [A2P 10DLC campaign approval best practices](https://support.twilio.com/hc/en-us/articles/11847054539547-A2P-10DLC-Campaign-Approval-Best-Practices)"#,
            TwilioProgrammableMessagingError::ErrorCode21635 => r#"## ERROR - 21635

### 'To' number cannot be a landline

 You have attempted to send an SMS with a `To` number that is a landline. 

#### Possible Causes
The `To` number you provided is a landline number.

#### Possible Solutions
* Confirm that the number you are sending to is not a landline, using the [Lookup API](https://www.twilio.com/docs/lookup/v2-api).
* Include `ForceDelivery` [API parameter](https://www.twilio.com/docs/messaging/api/message-resource#request-body-parameters) if you still want to send the message to this phone number. Please note that this does not guarantee the recipient number will be capable of receiving the message. Some international carriers may have a text-to-speech feature that converts the [SMS to a voice call](https://help.twilio.com/articles/223133647). 

"#,
            TwilioProgrammableMessagingError::ErrorCode30729 => r#"## ERROR - 30729

### Brand Registration Failure: Country code not allowed

 Provided country code has been blacklisted and is not allowed for US A2P Brand registrations.

#### Possible Causes
Provided country code has been blacklisted and is not allowed for US A2P Brand registrations. Supported list of country codes are : PR,PS,PT,PW,PY,QA,AD,AE,AF,AG,AI,AL,AM,AO,AQ,AR,AS,AT,RE,AU,AW,AX,AZ,RO,BA,BB,RS,BD,RU,BE,BF,BG,RW,BH,BI,BJ,BL,BM,BN,BO,SA,SB,BQ,SC,BR,BS,SD,SE,BT,SG,BV,SH,BW,SI,SJ,BY,BZ,SK,SL,SM,SN,SO,CA,SR,CC,SS,ST,CD,CF,SV,CG,SX,CH,CI,SZ,CK,CL,CM,CN,CO,CR,TC,TD,TF,TG,CV,CW,TH,CX,TJ,CY,TK,CZ,TL,TM,TN,TO,TR,TT,DE,TV,TW,DJ,TZ,DK,DM,DO,UA,UG,DZ,UM,EC,US,EE,EG,EH,UY,UZ,VA,ER,VC,ES,VE,ET,VG,VI,VN,VU,FI,FJ,FK,FM,FO,FR,WF,GA,GB,WS,GD,GE,GF,GG,GH,GI,GL,GM,GN,GP,GQ,GR,GS,GT,XE,GU,GW,GY,XK,HK,HM,HN,HR,YE,HT,HU,YT,ID,IE,IL,IM,IN,IO,ZA,IQ,IS,IT,ZM,JE,ZW,JM,JO,JP,KE,KG,KH,KI,KM,KN,KR,KW,KY,KZ,LA,LB,LC,LI,LK,LR,LS,LT,LU,LV,LY,MA,MC,MD,ME,MF,MG,MH,MK,ML,MM,MN,MO,MP,MQ,MR,MS,MT,MU,MV,MW,MX,MY,MZ,NA,NC,NE,NF,NG,NI,NL,NO,NP,NR,NU,NZ,OM,PA,PE,PF,PG,PH,PK,PL,PM,PN

#### Possible Solutions
Please use a supported country for brand registrations.

Additional help resources

* [Troubleshooting US A2P Brand registration failures](https://www.twilio.com/docs/messaging/compliance/a2p-10dlc/troubleshooting-a2p-brands)"#,
            TwilioProgrammableMessagingError::ErrorCode30896 => r#"## ERROR - 30896

### Campaign vetting rejection - Opt-in Error

 The campaign submission has been reviewed and it was rejected because of provided Opt-in information.

#### Possible Causes
* Opt-in message workflow does not meet the requirements for the specific campaign type.
* Consent is required but not adequately provided or maintained.
* Opt-in information is shared with third-party entities.

#### Possible Solutions
1. Ensure compliance with Twilio Messaging Policy relating to opt-in
2. Detail All Opt-in Methods: Include all methods of opt-in, whether electronic, paper form, in-person verbal opt-in, or other means.
3. Provide Necessary Links and Documentation: If opt-in is collected through a paper form or behind a login, supply a hosted link to an image of the opt-in. If the opt-in occurs on a website, provide the relevant link.
4. Include Privacy Policy and Terms of Service: The website where opt-in occurs must contain a privacy policy and terms of service.
5. Avoid Third-Party Sharing: Make sure that opt-in information is not shared with unauthorized third parties.
6. Ensure Opt-in is Verifable: Each campaign is manually reviewed and needs to be verifable by a human.

Once you have made a change to address the issue, please resubmit the campaign for review.

 
Additional help resources

* [CTIA Guidelines]( https://www.ctia.org/the-wireless-industry/industry-commitments/messaging-interoperability-sms-mms) 

* [A2P 10DLC campaign approval best practices](https://support.twilio.com/hc/en-us/articles/11847054539547-A2P-10DLC-Campaign-Approval-Best-Practices)"#,
            TwilioProgrammableMessagingError::ErrorCode30906 => r#"## ERROR - 30906

### Campaign Rejected by Twilio

 Messaging Service could not be attached to the Campaign because Twilio's DCA declined the Campaign due to compliance violations.

#### Possible Causes
* Twilio identified compliance violations during the review process. The specific reason for rejection is provided in the CAMPAIGN_SHARE_DELETE webhook from TCR.

#### Possible Solutions
* Revise the Campaign details to address the compliance issues and re-share the Campaign with Twilio's DCA for another review.
* For further clarification and resolution, contact Twilio Customer Support.
 
Additional help resources

* [A2P 10DLC campaign approval best practices](https://support.twilio.com/hc/en-us/articles/11847054539547-A2P-10DLC-Campaign-Approval-Best-Practices)
* [Changes to Externally Registered Campaign Submission](https://support.twilio.com/hc/en-us/articles/13097615242395-Changes-to-Externally-Registered-Campaign-Submission)"#,
            TwilioProgrammableMessagingError::ErrorCode30907 => r#"## ERROR - 30907

### Campaign vetting rejection - Website URL Validation Issue

 The campaign submission has been reviewed and rejected because the provided website URL does not match the Brand and Campaign registered.

#### Possible Causes
* *Mismatched Website Information*: The website URL submitted does not correspond to the Brand information or the campaign's intended use as registered.
* *Brand-Campaign Discrepancy*: There is a discrepancy between the Brand details and the campaign details on provided website.


#### Possible Solutions
1. Confirm website content aligns with the registered Brand and Campaign details.
2. Update Website URL submission on Brand details.
Once you have made a change to address the website validation error, please resubmit the campaign for review.
 
Additional help resources

* [A2P 10DLC campaign approval best practices](https://support.twilio.com/hc/en-us/articles/11847054539547-A2P-10DLC-Campaign-Approval-Best-Practices)
"#,
            TwilioProgrammableMessagingError::ErrorCode30902 => r#"## ERROR - 30902

### Campaign rejection - A DCA2 rejected this campaign registration request.

 The campaign registration request failed due to a DCA2 rejecting this campaign.

#### Possible Causes
Reasons for this failure can be:

* Third parties involved in campaign registration rejected the campaign registration request.

#### Possible Solutions
Please reach out to customer support with campaign and account details for resolution. "#,
            TwilioProgrammableMessagingError::ErrorCode30441 => r#"## ERROR - 30441

### Toll-Free phone number verification rejection - Disallowed: SHAFT - Sex

 The Toll-Free phone number verification submission has been reviewed and it was rejected. Effective Jan 31st, 2024, messaging traffic on this Toll-Free number is blocked until it’s verified.

Verification is one component of ensuring A2P SMS is used by businesses in a legal and compliant manner. The review process looks at the business sending messaging (not the software powering it), the use case, sample content, the web presence, the opt-in/consent collected and associated website terms and privacy policies.

#### Possible Causes
The toll-free phone number cannot be verified because the submission indicated that the business is associated with or the toll-free verification submission indicated a forbidden messaging category - (SHAFT: Sex). 

Twilio does not allow Hate speech, harassment, exploitative, abusive, or any communications that originate from a hate group.

#### Possible Solutions
If you are confident that your business and messaging use case does not fit into forbidden message categories, contact Twilio Support to provide Additional Information that explains your use case.


[Forbidden Message Categories](https://support.twilio.com/hc/en-us/articles/360045004974-Forbidden-message-categories-for-SMS-and-MMS-in-the-US-and-Canada)

[Why was my Toll Free Verification Rejected?](https://support.twilio.com/hc/en-us/articles/9321443984155-Why-Was-My-Toll-Free-Verification-Rejected) 

[Required information for Toll-free verification](https://support.twilio.com/hc/en-us/articles/13264118705051-Required-Information-for-Toll-Free-Verification)

[Toll-Free best practices](https://support.twilio.com/hc/en-us/articles/360038172934-Information-and-best-practices-for-using-Toll-Free-SMS-and-MMS-in-the-US-and-Canada)

[Twilio’s Messaging Policy](https://www.twilio.com/en-us/legal/messaging-policy)"#,
            TwilioProgrammableMessagingError::ErrorCode30465 => r#"## ERROR - 30465

### Toll Free verification rejection - Disallowed: Risk Investment/Get Rich Quick Schemes

 The Toll-Free phone number verification submission has been reviewed and it was rejected.Effective Jan 31st, 2024, messaging traffic on this Toll-Free number is blocked until it’s verified.

Verification is one component of ensuring A2P SMS is used by businesses in a legal and compliant manner. The review process looks at the business sending messaging (not the software powering it), the use case, sample content, the web presence, the opt-in/consent collected and associated website terms and privacy policies.


#### Possible Causes
The toll-free phone number cannot be verified because the submission indicated that the business is associated with or the toll-free verification submission indicated a forbidden messaging category (Get Rich Quick Schemes: Risk Investment). 


Use cases in this category pertain to minimal effort for maximum and/or guaranteed financial gains. These categories in the telecoms industry produce high consumer complaints and are not permissible on carrier routes.


#### Possible Solutions
If you are confident that your business and messaging use case does not fit into forbidden message categories, contact Twilio Support to provide Additional Information that explains your use case.


[Forbidden Message Categories](https://support.twilio.com/hc/en-us/articles/360045004974-Forbidden-message-categories-for-SMS-and-MMS-in-the-US-and-Canada)

[Why was my Toll Free Verification Rejected?](https://support.twilio.com/hc/en-us/articles/9321443984155-Why-Was-My-Toll-Free-Verification-Rejected) 

[Required information for Toll-free verification](https://support.twilio.com/hc/en-us/articles/13264118705051-Required-Information-for-Toll-Free-Verification)

[Toll-Free best practices](https://support.twilio.com/hc/en-us/articles/360038172934-Information-and-best-practices-for-using-Toll-Free-SMS-and-MMS-in-the-US-and-Canada)

[Twilio’s Messaging Policy](https://www.twilio.com/en-us/legal/messaging-policy)
"#,
            TwilioProgrammableMessagingError::ErrorCode35127 => r#"## ERROR - 35127

### 'Messages' Body and ContentVariables cannot be provided together

 You provided a 'Messages' Body and 'ContentVariables' parameters in your request, which would lead to a conflict. Please provide only one of these. 

#### Possible Causes
You provided both a 'Messages' Body and 'ContentVariables' parameters in your request

#### Possible Solutions
Specify only one of the both parameters in your request"#,
            TwilioProgrammableMessagingError::ErrorCode63045 => r#"## ERROR - 63045

### Link to a sample media file returned an unexpected error status

 The link returned an unexpected status code, making it impossible to retrieve the file.

#### Possible Causes
- The server that hosts the sample media file is not able to return the data

#### Possible Solutions
- Confirm that the link is correct
- See if the link can be accessed in your browser"#,
            TwilioProgrammableMessagingError::ErrorCode30797 => r#"## ERROR - 30797

### Brand Registration Feedback: Non government entity registered as a government entity. Must be a U.S. government entity.

 Your Brand has received feedback from The Campaign Registry upon registration. This Brand Registration can not be used for campaign creation until the provided feedback has been addressed. In this particular case, a non-government entity has been registered as a government entity. The entity registering the brand must be a U.S. government entity.

#### Possible Causes
* The submitted company is not a US entity. Only US entities qualify for the Government Entity Type. 
* The company’s submission as a Government Entity type could not be confirmed from independent sources. 


#### Possible Solutions
Please change the entity_type field value to something other than Government. If you are a US Entity, please update the customer profile address to indicate the same.

Additional help resources

* [Troubleshooting US A2P Brand registration failures](https://www.twilio.com/docs/messaging/compliance/a2p-10dlc/troubleshooting-a2p-brands)
"#,
            TwilioProgrammableMessagingError::ErrorCode30885 => r#"## ERROR - 30885

### Campaign vetting rejection - High Risk

 The campaign submission has been reviewed and it was rejected because it is considered High Risk.

#### Possible Causes
The campaign cannot be approved because it is considered Fraud or Deceptive Marketing.

#### Possible Solutions
The rejected campaign is ineligible for resubmission. If you feel that this was in error, please contact Twilio Customer Support.
 
Additional help resources

* [Forbidden message categories](https://support.twilio.com/hc/en-us/articles/360045004974-Forbidden-message-categories-for-SMS-and-MMS-in-the-US-and-Canada ) 

* [A2P 10DLC campaign approval best practices](https://support.twilio.com/hc/en-us/articles/11847054539547-A2P-10DLC-Campaign-Approval-Best-Practices)"#,
            TwilioProgrammableMessagingError::ErrorCode30894 => r#"## ERROR - 30894

### Campaign vetting rejection - Invalid Brand Information

 The campaign submission has been reviewed and it was rejected because invalid brand information.

#### Possible Causes
The campaign cannot be approved because registration needs to be associated with the brand behind the campaign. If you are an ISV registering a direct offering, campaign description needs to indicate a direct offering.

#### Possible Solutions
Please verify that the brand information is valid. Resubmit campaign for review.
 
Additional help resources

* [A2P 10DLC campaign approval best practices](https://support.twilio.com/hc/en-us/articles/11847054539547-A2P-10DLC-Campaign-Approval-Best-Practices)"#,
            TwilioProgrammableMessagingError::ErrorCode36009 => r#"## ERROR - 36009

### Stats for Broadcast SID Not Found

 Stats for the Broadcast SID entered were not found

#### Possible Causes
* You provided an empty Broadcast SID
* You provided a Broadcast SID that does not exist
* Stats for the Broadcast SID are not available yet

#### Possible Solutions
* Provide a non-empty, existing Broadcast SID
* If the Broadcast SID is valid, please retry after some time"#,
            TwilioProgrammableMessagingError::ErrorCode30620 => r#"## ERROR - 30620

### Message couldn't be delivered

 Message couldn’t be delivered to the recipients number you are trying to reach due to a potential mismatch message intent with the registered use case of the Sender ID used.

#### Possible Causes
This error occurs when a message is sent using a Sender ID that does not align with the registered use case for that particular Sender ID in your account. Ensure that the message content matches the use case registered with the Sender ID to comply with regulations and avoid failures.

#### Possible Solutions
**Use alternate sender ID:** Use a different Sender ID, whose registered use case matches to the intent of the message being sent.

**Register the sender ID:** Consider re-registering the Sender ID with a use case which is appropriate for your business use case

#### Continued issues with error 30620

Twilio's support team can help investigate what went wrong with delivering your message. Please collect 3 or more message SIDs in your SMS [logs](https://www.twilio.com/console/sms/logs) from the last 24 hours that were flagged with Error 30620, and [open a support request.](https://www.twilio.com/console/support/tickets/create)"#,
            TwilioProgrammableMessagingError::ErrorCode30702 => r#"## ERROR - 30702

### Brand Registration Failure: Registration not found

 There was an error while reconciling data with TCR.

#### Possible Causes
Following case can cause this error: 

* Brand is no longer active or has been deleted.

#### Possible Solutions
Please edit and resubmit the brand or re-register the brand. "#,
            TwilioProgrammableMessagingError::ErrorCode30991 => r#"## ERROR - 30991

### Campaign Registration Failed

 The Brand of this Campaign is in an unverified status.  Campaign registrations under unverified brands is not allowed.

#### Possible Causes
Reason(s) for this failure can be:

* Brand is in unverified state

#### Possible Solutions
Review the Brand associated with this Campaign you are trying to register."#,
            TwilioProgrammableMessagingError::ErrorCode36008 => r#"## ERROR - 36008

### Invalid Page Token

 Provided page token query parameter is invalid.

The provided page token query parameter must be a valid Broadcast SID, consisting of 34 alphanumeric characters, prefixed with "BC".

#### Possible Causes
* The provided page token has more or less than 34 characters
* The provided page token has non-alphanumeric characters.
* The prefix of the provided page token is something other than "BC"

#### Possible Solutions
* Make sure the page token has exactly 34 alphanumeric characters
* Make sure the page token is prefixed with "BC""#,
            TwilioProgrammableMessagingError::ErrorCode30883 => r#"## ERROR - 30883

### Campaign vetting rejection - Content Violation

 The campaign submission has been reviewed and it was rejected due to Content Violation.

#### Possible Causes
The campaign cannot be approved because the submission indicated that the business was part of a prohibited SHAFT category or the content was potentially sending SHAFT material. SHAFT (Sex, Hate, Alcohol, Firearms, Tobacco/Vape) and Marijuana/CBD are prohibited categories.

#### Possible Solutions
The rejected campaign is ineligible for resubmission. If you feel that this was in error, please contact Twilio Customer Support.
 
Additional help resources

* [Forbidden message categories](https://support.twilio.com/hc/en-us/articles/360045004974-Forbidden-message-categories-for-SMS-and-MMS-in-the-US-and-Canada ) 

* [A2P 10DLC campaign approval best practices](https://support.twilio.com/hc/en-us/articles/11847054539547-A2P-10DLC-Campaign-Approval-Best-Practices)"#,
            TwilioProgrammableMessagingError::ErrorCode30442 => r#"## ERROR - 30442

### Toll-Free phone number verification rejection - Disallowed: Spam

 The Toll-Free phone number verification submission has been reviewed and it was rejected. Effective Jan 31st, 2024, messaging traffic on this Toll-Free number is blocked until it’s verified.

Verification is one component of ensuring A2P SMS is used by businesses in a legal and compliant manner. The review process looks at the business sending messaging (not the software powering it), the use case, sample content, the web presence, the opt-in/consent collected and associated website terms and privacy policies.

#### Possible Causes
The toll-free phone number cannot be verified because the business or message content was found to be associated with a known spam or fraudulent campaign. 

#### Possible Solutions
If you are confident that your business and messaging use case does not fit into forbidden message categories, contact Twilio Support to provide Additional Information that explains your use case.


[Forbidden Message Categories](https://support.twilio.com/hc/en-us/articles/360045004974-Forbidden-message-categories-for-SMS-and-MMS-in-the-US-and-Canada)

[Why was my Toll Free Verification Rejected?](https://support.twilio.com/hc/en-us/articles/9321443984155-Why-Was-My-Toll-Free-Verification-Rejected) 

[Required information for Toll-free verification](https://support.twilio.com/hc/en-us/articles/13264118705051-Required-Information-for-Toll-Free-Verification)

[Toll-Free best practices](https://support.twilio.com/hc/en-us/articles/360038172934-Information-and-best-practices-for-using-Toll-Free-SMS-and-MMS-in-the-US-and-Canada)

[Twilio’s Messaging Policy](https://www.twilio.com/en-us/legal/messaging-policy)"#,
            TwilioProgrammableMessagingError::ErrorCode30892 => r#"## ERROR - 30892

### Campaign vetting rejection - Invalid Sample Message - Public URL Shorteners

 The campaign submission has been reviewed and it was rejected because of URL shortener in the sample message.

#### Possible Causes
The campaign cannot be approved because public URL shorteners are not accepted. The website URL included in the sample messages was either from a public URL shortener or is unsecured.

#### Possible Solutions
Verify URL in sample messages does not contain public URL shorteners. Resubmit campaign."#,
            TwilioProgrammableMessagingError::ErrorCode30751 => r#"## ERROR - 30751

### Brand Registration Failure: Unsupported mobile phone number

 You have provided an unsupported mobile phone number for brand registration.

#### Possible Causes
You have provided an unsupported mobile phone number for brand registration.

#### Possible Solutions
Please use a valid US Wireless mobile phone number which is capable of receiving verification SMS.

Additional help resources

* [Troubleshooting US A2P Brand registration failures]"#,
            TwilioProgrammableMessagingError::ErrorCode30891 => r#"## ERROR - 30891

### Campaign vetting rejection - Invalid Website URL

 The campaign submission has been reviewed and it was rejected because of unverifiable website.

#### Possible Causes
* An invalid URL was provided during the registration process.
* The website associated with the campaign is not functioning or inaccessible.
* Opt-in flow is not found on website provided.
* Lack of proper indication in the campaign description if the registration pertains to a pre-launch website.

#### Possible Solutions
* Verify that the provided website(s) are functioning correctly, and accessible. If the website(s) are geographic specific, please indicate this in the campaign description and provide a link to a screenshot.
* If the registration is for a pre-launch website, instead include a publically accessible URL to view a screenshot of the SMS opt-in flow that will appear.
* If opt-in occurs on website, provide the direct link in the Message Flow field.
* If opt-in is collected through a paper form or behind a login, supply a hosted link to an image of the opt-in. If the opt-in occurs on a website, provide the relevant link.

Once you have made a change to address the issue, please resubmit the campaign for review.

Additional help resources

* [A2P 10DLC campaign approval best practices](https://support.twilio.com/hc/en-us/articles/11847054539547-A2P-10DLC-Campaign-Approval-Best-Practices)"#,
            TwilioProgrammableMessagingError::ErrorCode30455 => r#"## ERROR - 30455

### Toll-Free phone number verification rejection - Disallowed: SHAFT - Hate

 The Toll-Free phone number verification submission has been reviewed and it was rejected. Effective Jan 31st, 2024, messaging traffic on this Toll-Free number is blocked until it’s verified.

Verification is one component of ensuring A2P SMS is used by businesses in a legal and compliant manner. The review process looks at the business sending messaging (not the software powering it), the use case, sample content, the web presence, the opt-in/consent collected and associated website terms and privacy policies.

#### Possible Causes
The toll-free phone number cannot be verified because the submission indicated that the business is associated with or the toll-free verification submission indicated a forbidden messaging category - (SHAFT: Hate). 

Twilio does not allow Hate speech, harassment, exploitative, abusive, or any communications that originate from a hate group.

#### Possible Solutions
If you are confident that your business and messaging use case does not fit into forbidden message categories, contact Twilio Support to provide Additional Information that explains your use case.


[Forbidden Message Categories](https://support.twilio.com/hc/en-us/articles/360045004974-Forbidden-message-categories-for-SMS-and-MMS-in-the-US-and-Canada)

[Why was my Toll Free Verification Rejected?](https://support.twilio.com/hc/en-us/articles/9321443984155-Why-Was-My-Toll-Free-Verification-Rejected) 

[Required information for Toll-free verification](https://support.twilio.com/hc/en-us/articles/13264118705051-Required-Information-for-Toll-Free-Verification)

[Toll-Free best practices](https://support.twilio.com/hc/en-us/articles/360038172934-Information-and-best-practices-for-using-Toll-Free-SMS-and-MMS-in-the-US-and-Canada)

[Twilio’s Messaging Policy](https://www.twilio.com/en-us/legal/messaging-policy)"#,
            TwilioProgrammableMessagingError::ErrorCode63043 => r#"## ERROR - 63043

### Link to a sample media file returns 403 Forbidden

 The link returned a status code of 403 Forbidden, making it impossible to retrieve the file.

#### Possible Causes
- Server requires authorization

#### Possible Solutions
- Use a publicly accessible URL
- Use a URL that includes an access token"#,
            TwilioProgrammableMessagingError::ErrorCode36003 => r#"## ERROR - 36003

### Broadcast Upload File Size Exceeded

 The file size is greater than the allowed 25 MB

#### Possible Causes
* You attempted to upload a file that is greater than 25 MB

#### Possible Solutions
* Upload a file that is less than 25 MB"#,
            TwilioProgrammableMessagingError::ErrorCode35134 => r#"## ERROR - 35134

### `Messages` array contains duplicate `To` numbers.

 `Messages` array contains duplicate `To` numbers. Please check the list for duplicates

#### Possible Causes
The provided `Messages` array has one or more duplicate `To` numbers

#### Possible Solutions
Check the `Messages` array and remove `To` number duplicates if any"#,
            TwilioProgrammableMessagingError::ErrorCode36007 => r#"## ERROR - 36007

### Broadcast Upload Status Check Failure

 The Broadcast cannot have a file uploaded to it because it is in a state other than PENDING_UPLOAD.

#### Possible Causes
* The Broadcast SID that you provided is for a Broadcast that is in a state other than PENDING_UPLOAD. Those other states include: UPLOADED, QUEUED, EXECUTING, EXECUTION_FAILURE, EXECUTION_COMPLETED, CANCELATION_REQUESTED, CANCELED

#### Possible Solutions
* Upload the file to a Broadcast SID that is in a PENDING_UPLOAD state"#,
            TwilioProgrammableMessagingError::ErrorCode30796 => r#"## ERROR - 30796

### Brand Registration Feedback: Non public entity registered as a public for profit entity or the stock information mismatch.

 Your Brand has received feedback from The Campaign Registry upon registration. This Brand Registration can not be used for campaign creation until the provided feedback has been addressed. In this particular case, either a non-public entity has been registered as a public for profit entity or the stock information provided during the registration process doesn’t match the public company records.


#### Possible Causes
* The submitted stock symbol and stock exchange could not be verified for the submitted legal company name. 
* The submitted stock exchange is not recognized.  


#### Possible Solutions
Please edit your A2P profile and update the stock_symbol and/or stock_exchange with correct information.

Additional help resources

* [Troubleshooting US A2P Brand registration failures](https://www.twilio.com/docs/messaging/compliance/a2p-10dlc/troubleshooting-a2p-brands)"#,
            TwilioProgrammableMessagingError::ErrorCode30470 => r#"## ERROR - 30470

### Toll Free verification rejection - High Risk: Deceptive Marketing

 The Toll-Free phone number verification submission has been reviewed and it was rejected.Effective Jan 31st, 2024, messaging traffic on this Toll-Free number is blocked until it’s verified.

Verification is one component of ensuring A2P SMS is used by businesses in a legal and compliant manner. The review process looks at the business sending messaging (not the software powering it), the use case, sample content, the web presence, the opt-in/consent collected and associated website terms and privacy policies.


#### Possible Causes
The toll-free phone number cannot be verified because the submission indicated that the business is associated with or the toll-free verification submission indicated a forbidden messaging category  (High-risk: Deceptive Marketing). 

Marketing messages must be truthful, not misleading, and, when appropriate, backed by scientific evidence in order to meet the standard held by the Federal Trade Commission’s (FTC) Truth In Advertising rules. The FTC prohibits unfair or deceptive advertising in any medium, including text messages.


#### Possible Solutions
If you are confident that your business and messaging use case does not fit into forbidden message categories, contact Twilio Support to provide Additional Information that explains your use case.

[Forbidden Message Categories](https://support.twilio.com/hc/en-us/articles/360045004974-Forbidden-message-categories-for-SMS-and-MMS-in-the-US-and-Canada)

[Why was my Toll Free Verification Rejected?](https://support.twilio.com/hc/en-us/articles/9321443984155-Why-Was-My-Toll-Free-Verification-Rejected) 

[Required information for Toll-free verification](https://support.twilio.com/hc/en-us/articles/13264118705051-Required-Information-for-Toll-Free-Verification)

[Toll-Free best practices](https://support.twilio.com/hc/en-us/articles/360038172934-Information-and-best-practices-for-using-Toll-Free-SMS-and-MMS-in-the-US-and-Canada)

[Twilio’s Messaging Policy](https://www.twilio.com/en-us/legal/messaging-policy)
"#,
            TwilioProgrammableMessagingError::ErrorCode30476 => r#"## ERROR - 30476

### Toll Free verification rejection - Opt-in not provided

The Toll-Free phone number cannot be verified because of one or more issues related to the proof of opt-in for consumer to provide consent: 1. You may have selected an opt-in method without description or providing evidence. Please note that every end business must have consent from each recipient they send a message to; 2. You may have submitted a URL for the opt-in workflow, but it did not demonstrate how the consumer opts-in to messaging; 3. You may have submitted a URL that is not reachable, resolvable and accessible to the general public. The URL can contain a link to the web form where the consumer opts-in, a hosted image file with a screenshot of how the consumer opts-in, or a link to a document that tells the story of the opt-in (such as a verbal opt-in script). Please do not submit a URL that requires the user to log into your website to see, and resubmit with a link that tells the story of your opt-in workflow. The Toll-Free phone number verification submission has been reviewed and it was rejected. Effective Jan 31st, 2024, messaging traffic on this Toll-Free number is blocked until it’s verified.

Verification is one component of ensuring A2P SMS is used by businesses in a legal and compliant manner. The review process looks at the business sending messaging (not the software powering it), the use case, sample content, the web presence, the opt-in/consent collected and associated website terms and privacy policies.

#### Possible Causes
Proof of opt-in for consumer to provide consent was not provided. Every end business must have consent from each recipient they send a message to. It’s insufficient to select an opt-in method without description or providing evidence.

You may have submitted a URL for the opt-in workflow, but it did not demonstrate how the consumer opts-in to messaging. The URL can contain a link to the web form where the consumer opts-in, a hosted image file with a screenshot of how the consumer opts-in, or a link to a document that tells the story of the opt-in (such as a verbal opt-in script). 

Any URL submitted must be reachable, resolvable and of access to the public. 

Opt-in refers to the process of getting a consumer’s permission to send them text messages. A2P message Senders should: 
* Obtain a Consumer’s consent to receive messages generally; 
* Obtain a Consumer’s express written consent to specifically receive marketing messages; and 
* Ensure that Consumers have the ability to revoke consent.

Note: Any business with a terms of service or privacy policy that mentions sharing or selling consumer data/opt-in information is considered noncompliant. 


#### Possible Solutions
Resubmit the toll-free verification with a link that tells the story of your opt-in workflow. 

Resources: 

[CTIA guidelines](https://www.ctia.org/the-wireless-industry/industry-commitments/messaging-interoperability-sms-mms)

[Consent/Opt-In](https://www.twilio.com/en-us/legal/messaging-policy#:~:text=with%20Twilio%20policies.-,Consent%20/%20Opt%2Din,-What%20Is%20Proper) in [Twilio’s Messaging Policy](https://www.twilio.com/en-us/legal/messaging-policy)

[Why was my Toll Free Verification Rejected?](https://support.twilio.com/hc/en-us/articles/9321443984155-Why-Was-My-Toll-Free-Verification-Rejected) 

[Required information for Toll-free verification](https://support.twilio.com/hc/en-us/articles/13264118705051-Required-Information-for-Toll-Free-Verification)

[Toll-Free best practices](https://support.twilio.com/hc/en-us/articles/360038172934-Information-and-best-practices-for-using-Toll-Free-SMS-and-MMS-in-the-US-and-Canada)"#,
            TwilioProgrammableMessagingError::ErrorCode30990 => r#"## ERROR - 30990

### Campaign Suspension

 The campaign has been suspended. Messaging has been halted and number registration/deregistration is not available until suspension has been lifted

#### Possible Causes
*Active Suspension*: The campaign has been suspended due to a violation of messaging policies or other related issues.

*Suspension Reason Category : Suspension Reason Description*
* Campaign-to-traffic mismatch: In-market traffic does not match with the registered campaign.
* Spam: Including but not limited to any kind of unwanted or unsolicited messaging.
* Controlled substance: Including but not limited to messaging pertaining to controlled substances.
* Phishing Messages: Including but not limited to messaging designed to gain access.
* Excessive Complaints: Including but not limited to unacceptable volumes of consumer complaints.
* Illicit Content: Including but not limited to messages relating to illegal activity.
* Fraudulent Messages: Including but not limited to counterfeit/fraudulent goods or activities.
* Affiliate Marketing: Including but not limited to sharing of opt-ins to affiliate companies.
* Promotional Gambling: Including but not limited to the act of gambling or promoting gambling.
* Lack of Age Gate: No age gate mechanism for messaging campaigns that require it.
* Illegal Sweepstakes: Including but not limited to sweepstakes that do not follow required laws.
* Other: As applicable.

#### Possible Solutions
1. Review the specific reasons for the campaign suspension and resolve any outstanding compliance or policy issues. 
2. Contact Twilio Support team to receive more details around why your campaign has been suspended, and how to resolve the issue."#,
            TwilioProgrammableMessagingError::ErrorCode30436 => r#"## ERROR - 30436

### Invalid Phone Number SID

 The phone number SID is malformed or belongs to another account.

#### Possible Causes
* The phone number SID does not exist or does not belong to your account.
* The phone number SID is not a 34 character string prefixed with “PN”


#### Possible Solutions
* Please specify a phone number SID for a toll free number you own."#,
            TwilioProgrammableMessagingError::ErrorCode30452 => r#"## ERROR - 30452

### Toll-Free phone number verification unable to process - email invalid

"The Toll-Free phone number cannot be verified because the email address is invalid. It could be due to: 1. Incorrectly formatted email address; 2. Unknown email domain; 3. Email is a suspected disposable address; 4. Email is invalid. Please correct the email address and resubmit. " Twilio is unable to process your Toll Free Verification request because it could not be validated by Twilio and could not be submitted for verification review. 


#### Possible Causes
The toll-free phone number cannot be verified because the email address is invalid. It could be due to:

* Incorrectly formatted email address
* Unknown email domain
* Email is a suspected disposable address
* Email is invalid


#### Possible Solutions
Correct the email address and resubmit the verification in Console via the Messaging Compliance API. It is not recommended to delete the verification to resolve email validation issue. 


Forbidden message categories

https://support.twilio.com/hc/en-us/articles/360045004974-Forbidden-message-categories-for-SMS-and-MMS-in-the-US-and-Canada 

Toll-Free best practices can be found at 

https://support.twilio.com/hc/en-us/articles/360038172934-Information-and-best-practices-for-using-Toll-Free-SMS-and-MMS-in-the-US-and-Canada
"#,
            TwilioProgrammableMessagingError::ErrorCode30793 => r#"## ERROR - 30793

### Brand Registration Failure: Validation problems with connected bundles

 There was an error while processing the provided customer and a2p information.

#### Possible Causes
Following cases can cause this error :

* We were not able to find A2P bundle with the bundle SID provided as A2PProfileBundleSid

* We were not able to find Customer Profile bundle with the bundle SID provided as CustomerProfileBundleSid

* Invalid/unsupported stock exchange information was provided as part of A2P bundle (applicable only for public entities registering Low Volume Standard or Standard Brands)

#### Possible Solutions
Please check that bundle SIDs provide as A2PProfileBundleSid/CustomerProfileBundleSid exist and contain valid information.


Please check that you have provided one the following support Stock Exchanges (applicable only for public entities registering Low Volume Standard or Standard Brands)
NONE, NASDAQ, NYSE, AMEX, AMX, ASX, B3, BME, BSE, FRA, ICEX, JPX, JSE, KRX, LON, NSE, OMX, SEHK, SGX, SSE, STO, SWX, SZSE, TSX, TWSE, VSE, OTHER"#,
            TwilioProgrammableMessagingError::ErrorCode30468 => r#"## ERROR - 30468

### Toll Free verification rejection - Disallowed: Third-party Lead Generation

 The Toll-Free phone number verification submission has been reviewed and it was rejected. Effective Jan 31st, 2024, messaging traffic on this Toll-Free number is blocked until it’s verified.

Verification is one component of ensuring A2P SMS is used by businesses in a legal and compliant manner. The review process looks at the business sending messaging (not the software powering it), the use case, sample content, the web presence, the opt-in/consent collected and associated website terms and privacy policies.


#### Possible Causes
The toll-free phone number cannot be verified because the submission indicated that the business is associated with or the toll-free verification submission indicated a forbidden messaging category (Third-party lead generation services and marketing). 

Any third-party use cases are strictly forbidden. Consent must be obtained by the end business directly from consumers. Consent cannot be given to one business and then transferred to another. Any business with a terms of service or privacy policy that mentions sharing or selling consumer data/opt-in information is considered noncompliant.


#### Possible Solutions
If you are confident that your business and messaging use case does not fit into forbidden message categories, contact Twilio Support to provide Additional Information that explains your use case.


[Forbidden Message Categories](https://support.twilio.com/hc/en-us/articles/360045004974-Forbidden-message-categories-for-SMS-and-MMS-in-the-US-and-Canada)

[Why was my Toll Free Verification Rejected?](https://support.twilio.com/hc/en-us/articles/9321443984155-Why-Was-My-Toll-Free-Verification-Rejected) 

[Required information for Toll-free verification](https://support.twilio.com/hc/en-us/articles/13264118705051-Required-Information-for-Toll-Free-Verification)

[Toll-Free best practices](https://support.twilio.com/hc/en-us/articles/360038172934-Information-and-best-practices-for-using-Toll-Free-SMS-and-MMS-in-the-US-and-Canada)

[Twilio’s Messaging Policy](https://www.twilio.com/en-us/legal/messaging-policy)
"#,
            TwilioProgrammableMessagingError::ErrorCode63052 => r#"## WARNING - 63052

### Legacy WhatsApp Template Used

 Twilio has detected that this WhatsApp message was sent using the currently EOL Legacy WhatsApp templates system. 

Changes are required to how you send templates over WhatsApp by April 1, 2025. Templates must be created and managed using the Content Template Builder or Content API. Templates must be sent using ContentSid and Content Variables. 

For more information please see this support article: https://help.twilio.com/articles/19816296822299




#### Possible Causes
Template sent using body field.

#### Possible Solutions
Templates must be sent using ContentSid and Content Variables"#,
            TwilioProgrammableMessagingError::ErrorCode30748 => r#"## ERROR - 30748

### Brand Registration Failure: Phone number duplicate threshold reached

 Brand can not be registered with provided phone number since duplicate threshold limit has been reached for it.

#### Possible Causes
A phone number can not be used multiple times for Sole Proprietor Brand registrations. It is possible that the Starter customer profile bundle used for Sole Proprietor brand registration contains phone number that has been used to register multiple Sole Proprietor brands already. 

#### Possible Solutions
Please edit the starter customer profile and use a different phone number. Once submitted for review, you can resubmit the brand registration request. 

Additional help resources

* [Troubleshooting US A2P Brand registration failures]"#,
            TwilioProgrammableMessagingError::ErrorCode30992 => r#"## ERROR - 30992

### Campaign Registration Failed

 Connectivity partner migration process was cancelled.

#### Possible Causes
The user cancelled the CNP migration process before it could complete.

#### Possible Solutions
Please retry the registration after starting the CNP migration process."#,
            TwilioProgrammableMessagingError::ErrorCode30451 => r#"## ERROR - 30451

### Toll-Free phone number verification unable to process - address invalid

The Toll-Free phone number cannot be verified because the address is invalid. The address must be a valid address in US or Canada. Please edit the address fields and resubmit. Twilio is unable to process your Toll Free Verification request because it could not be validated by Twilio and could not be submitted for verification review. 


#### Possible Causes
The toll-free phone number cannot be verified because the address is invalid. It could be due to:

* The provided address is not a valid US or Canada address
* Invalid State or Region
* Invalid Country
* Invalid House Number (or Unit/Apartment/Suite Number)
* Invalid City
* Invalid Postal Code
* Invalid Street
* Invalid Address - multiple issues


#### Possible Solutions
Correct the address and resubmit the verification in Console or via the Messaging Compliance API. It is not recommended to delete the verification to resolve address validation issue. 

Forbidden message categories

https://support.twilio.com/hc/en-us/articles/360045004974-Forbidden-message-categories-for-SMS-and-MMS-in-the-US-and-Canada 

Toll-Free best practices can be found at 

https://support.twilio.com/hc/en-us/articles/360038172934-Information-and-best-practices-for-using-Toll-Free-SMS-and-MMS-in-the-US-and-Canada
"#,
            TwilioProgrammableMessagingError::ErrorCode30457 => r#"## ERROR - 30457

### Toll Free verification rejection - Disallowed: SHAFT - Firearms

 The Toll-Free phone number verification submission has been reviewed and it was rejected. Effective Jan 31st, 2024, messaging traffic on this Toll-Free number is blocked until it’s verified.

#### Possible Causes
The toll-free phone number cannot be verified because the submission indicated that the business is associated with or the toll-free verification submission indicated a forbidden messaging category (SHAFT: Firearms and accessories). 

Firearms and accessories, Tobacco, Vape, and E-cigarettes are not allowed on Toll Free, Short Code, or Long Code regardless of age gating.

#### Possible Solutions
If you are confident that your business and messaging use case does not fit into forbidden message categories, contact Twilio Support to provide Additional Information that explains your use case.

[Forbidden Message Categories](https://support.twilio.com/hc/en-us/articles/360045004974-Forbidden-message-categories-for-SMS-and-MMS-in-the-US-and-Canada)

[Why was my Toll Free Verification Rejected?](https://support.twilio.com/hc/en-us/articles/9321443984155-Why-Was-My-Toll-Free-Verification-Rejected) 

[Required information for Toll-free verification](https://support.twilio.com/hc/en-us/articles/13264118705051-Required-Information-for-Toll-Free-Verification)

[Toll-Free best practices](https://support.twilio.com/hc/en-us/articles/360038172934-Information-and-best-practices-for-using-Toll-Free-SMS-and-MMS-in-the-US-and-Canada)

[Twilio’s Messaging Policy](https://www.twilio.com/en-us/legal/messaging-policy)"#,
            TwilioProgrammableMessagingError::ErrorCode30467 => r#"## ERROR - 30467

### Toll Free verification rejection - Disallowed: Credit Repair

 The Toll-Free phone number verification submission has been reviewed and it was rejected.Effective Jan 31st, 2024, messaging traffic on this Toll-Free number is blocked until it’s verified.

Verification is one component of ensuring A2P SMS is used by businesses in a legal and compliant manner. The review process looks at the business sending messaging (not the software powering it), the use case, sample content, the web presence, the opt-in/consent collected and associated website terms and privacy policies.

#### Possible Causes
The toll-free phone number cannot be verified because the submission indicated that the business is associated with or the toll-free verification submission indicated a forbidden messaging category (Debt collection or forgiveness: Credit/debt repair). 

Debt consolidation, debt reduction and credit repair programs are prohibited regardless if there is first-party consent.


#### Possible Solutions
If you are confident that your business and messaging use case does not fit into forbidden message categories, contact Twilio Support to provide additional information that explains your use case.

[Forbidden Message Categories](https://support.twilio.com/hc/en-us/articles/360045004974-Forbidden-message-categories-for-SMS-and-MMS-in-the-US-and-Canada)

[Why was my Toll Free Verification Rejected?](https://support.twilio.com/hc/en-us/articles/9321443984155-Why-Was-My-Toll-Free-Verification-Rejected) 

[Required information for Toll-free verification](https://support.twilio.com/hc/en-us/articles/13264118705051-Required-Information-for-Toll-Free-Verification)

[Toll-Free best practices](https://support.twilio.com/hc/en-us/articles/360038172934-Information-and-best-practices-for-using-Toll-Free-SMS-and-MMS-in-the-US-and-Canada)

[Twilio’s Messaging Policy](https://www.twilio.com/en-us/legal/messaging-policy)
"#,
            TwilioProgrammableMessagingError::ErrorCode30798 => r#"## ERROR - 30798

### Brand Registration Feedback: No IRS 501c tax-exempt status found.

 Your Brand has received feedback from The Campaign Registration upon registration. This brand registration can not be used for campaign creation till the provided feedback has been addressed. In this particular case,  no IRS 501c tax-exempt status was found for the entity being registered.

#### Possible Causes
* The submitted company is not a US entity. Only US entities qualify for the Non-Profit Entity Type. 
* The submitted company is not verified as a US IRS recognised non-profit organisation. 
* The submitted company is verified as a US IRS recognised non-profit organisation but the charitable subsection code was not reported.


#### Possible Solutions
Please change the entity_type field value to something other than Non-Profit. If you are a Non-Profit Entity, please update the customer profile to indicate the same.

Additional help resources

* [Troubleshooting US A2P Brand registration failures](https://www.twilio.com/docs/messaging/compliance/a2p-10dlc/troubleshooting-a2p-brands)
"#,
            TwilioProgrammableMessagingError::ErrorCode30461 => r#"## ERROR - 30461

### Toll Free verification rejection - Disallowed: Gambling

 The Toll-Free phone number verification submission has been reviewed and it was rejected. Effective Jan 31st, 2024, messaging traffic on this Toll-Free number is blocked until it’s verified.

Verification is one component of ensuring A2P SMS is used by businesses in a legal and compliant manner. The review process looks at the business sending messaging (not the software powering it), the use case, sample content, the web presence, the opt-in/consent collected and associated website terms and privacy policies.

#### Possible Causes
The toll-free phone number cannot be verified because the submission indicated that the business is associated with or the toll-free verification submission indicated a forbidden messaging category (Gambling). 

Gambling traffic is prohibited in the US and Canada on all number types (Toll Free, Short Code, Long Code). The gambling category includes, but is not limited to:
 
* Casino apps
* Websites that offer gambling
* Sweepstakes
* 50/50 Raffles
* Contests
* Betting/Sports picks

#### Possible Solutions
If you are confident that your business and messaging use case does not fit into forbidden message categories, contact Twilio Support to provide Additional Information that explains your use case.

[Forbidden Message Categories](https://support.twilio.com/hc/en-us/articles/360045004974-Forbidden-message-categories-for-SMS-and-MMS-in-the-US-and-Canada)

[Why was my Toll Free Verification Rejected?](https://support.twilio.com/hc/en-us/articles/9321443984155-Why-Was-My-Toll-Free-Verification-Rejected) 

[Required information for Toll-free verification](https://support.twilio.com/hc/en-us/articles/13264118705051-Required-Information-for-Toll-Free-Verification)

[Toll-Free best practices](https://support.twilio.com/hc/en-us/articles/360038172934-Information-and-best-practices-for-using-Toll-Free-SMS-and-MMS-in-the-US-and-Canada)

[Twilio’s Messaging Policy](https://www.twilio.com/en-us/legal/messaging-policy)"#,
            TwilioProgrammableMessagingError::ErrorCode30893 => r#"## ERROR - 30893

### Campaign vetting rejection - Inconsistency between Sample Message and Use-case

 The campaign submission has been reviewed and rejected due to inconsistencies between the sample message and the intended use-case of the campaign.

#### Possible Causes
* Sample messages are missing, unclear, or their content does not match the campaign's use-case.
* Invalid content within the sample messages.

#### Possible Solutions
* Verify that the sample messages are accurate, detailed, and reflective of the actual messages to be sent under the campaign.
* Indicate templated fields within sample messages using brackets.
* At least one of the sample messages should include your business name and opt-out message.
* Ensure at least two sample messages are provided.
* Ensure that the use-case and campaign description align with each other.

Once you have made a change to address the issue, please resubmit the campaign for review.

Additional help resources

* [A2P 10DLC campaign approval best practices]( https://support.twilio.com/hc/en-us/articles/11847054539547-A2P-10DLC-Campaign-Approval-Best-Practices)"#,
            TwilioProgrammableMessagingError::ErrorCode30469 => r#"## ERROR - 30469

### Toll Free verification rejection - Disallowed: Illegal substances/articles

 The Toll-Free phone number verification submission has been reviewed and it was rejected.Effective Jan 31st, 2024, messaging traffic on this Toll-Free number is blocked until it’s verified.

Verification is one component of ensuring A2P SMS is used by businesses in a legal and compliant manner. The review process looks at the business sending messaging (not the software powering it), the use case, sample content, the web presence, the opt-in/consent collected and associated website terms and privacy policies.

#### Possible Causes
The toll-free phone number cannot be verified because the submission indicated that the business is associated with or the toll-free verification submission indicated a forbidden messaging category (Illegal substances/articles).

Cannabis, CBD, Kratom, or drug paraphernalia product businesses are prohibited from utilizing SMS/MMS messaging on Twilio in the US and Canada, regardless of content. These restrictions apply regardless of the federal or state legality. All use cases for these are disallowed from sending SMS whether it contains cannabis content or not, even for 2FA purposes it is not permissible for such entities. Illegal substances/articles include, but are not limited to: 

* Cannabis
* CBD
* Kratom
* Paraphernalia products
* Vape/E-cigs
* Fireworks

#### Possible Solutions
If you are confident that your business and messaging use case does not fit into forbidden message categories, contact Twilio Support to provide Additional Information that explains your use case.

[Forbidden Message Categories](https://support.twilio.com/hc/en-us/articles/360045004974-Forbidden-message-categories-for-SMS-and-MMS-in-the-US-and-Canada)

[Why was my Toll Free Verification Rejected?](https://support.twilio.com/hc/en-us/articles/9321443984155-Why-Was-My-Toll-Free-Verification-Rejected) 

[Required information for Toll-free verification](https://support.twilio.com/hc/en-us/articles/13264118705051-Required-Information-for-Toll-Free-Verification)

[Toll-Free best practices](https://support.twilio.com/hc/en-us/articles/360038172934-Information-and-best-practices-for-using-Toll-Free-SMS-and-MMS-in-the-US-and-Canada)

[Twilio’s Messaging Policy](https://www.twilio.com/en-us/legal/messaging-policy)"#,
            TwilioProgrammableMessagingError::ErrorCode30753 => r#"## ERROR - 30753

### Brand Registration Failure: Unsupported email address

 You have used an unsupported email address for brand registration

#### Possible Causes
You have used an unsupported email address for brand registration

#### Possible Solutions
Please use a valid and supported email address.

Additional help resources

* [Troubleshooting US A2P Brand registration failures]"#,
            TwilioProgrammableMessagingError::ErrorCode30448 => r#"## ERROR - 30448

### Toll-Free phone number verification rejection - Age Gate

The Toll-Free phone number cannot be verified because an Age Gate is not present or is not acceptable on your website. Please add a robust age gate to your website or opt-in policy, and resubmit the toll free verification with the updated age gate information. The Toll-Free phone number verification submission has been reviewed and it was rejected. Effective Jan 31st, 2024, messaging traffic on this Toll-Free number is blocked until it’s verified.

Verification is one component of ensuring A2P SMS is used by businesses in a legal and compliant manner. The review process looks at the business sending messaging (not the software powering it), the use case, sample content, the web presence, the opt-in/consent collected and associated website terms and privacy policies.

#### Possible Causes
The Toll-Free phone number cannot be verified because an Age Gate is not present or is not acceptable on your website and/or opt-in policy.

#### Possible Solutions
* Add a robust age gate to your website or opt-in policy. Resubmit the toll free verification with the updated age gate information. 

[Why was my Toll Free Verification Rejected?](https://support.twilio.com/hc/en-us/articles/9321443984155-Why-Was-My-Toll-Free-Verification-Rejected) 

[Required information for Toll-free verification](https://support.twilio.com/hc/en-us/articles/13264118705051-Required-Information-for-Toll-Free-Verification)

[Toll-Free best practices](https://support.twilio.com/hc/en-us/articles/360038172934-Information-and-best-practices-for-using-Toll-Free-SMS-and-MMS-in-the-US-and-Canada)

[Twilio’s Messaging Policy](https://www.twilio.com/en-us/legal/messaging-policy)"#,
            TwilioProgrammableMessagingError::ErrorCode30449 => r#"## ERROR - 30449

### Toll Free verification rejection - URL issues in sample message

The Toll-Free phone number cannot be verified because of one or more issues related to the website URL in sample messages: 1. Links to websites embedded within a message cannot conceal or obscure the Message Sender's identity and are not intended to cause harm or deceive Consumers; 2. Public URL shortener cannot be used. A website URL used in messaging must be a dedicated custom domain that belongs to your business; 3. The domain must align with the message sender identified in the text message itself. Web addresses contained in messages as well as any websites to which they redirect should unambiguously identify the website owner and include contact information, such as an address. Please replace edit your sample message to include a branded URL and resubmit the toll free verification. The Toll-Free phone number verification submission has been reviewed and it was rejected. Effective Jan 31st, 2024, messaging traffic on this Toll-Free number is blocked until it’s verified.

Verification is one component of ensuring A2P SMS is used by businesses in a legal and compliant manner. The review process looks at the business sending messaging (not the software powering it), the use case, sample content, the web presence, the opt-in/consent collected and associated website terms and privacy policies.

#### Possible Causes
The toll-free phone number cannot be verified because the website URL in the sample message that you submitted was either from a public URL shortener or is non-secure.

Links to websites embedded within a message cannot conceal or obscure the Message Sender’s identity and are not intended to cause harm or deceive Consumers. A website URL used in messaging must be a dedicated custom domain that belongs to your business, not a free shared public link shortener.The domain must align with the message sender identified in the text message itself. Web addresses contained in messages as well as any websites to which they redirect should unambiguously identify the website owner (i.e., a person or legally registered business entity) and include contact information, such as a postal mailing address.

#### Possible Solutions
* Replace call to action (CTA) in sample message with a branded URL. Resubmit the toll free verification with an updated sample message.  
* Ensure that any URL's sent in messages are not sent from a public URL shortener.
* Consider utilizing Twilio’s Link Shortening to programmatically shorten long links with your own company branded domain. 

[Link Shortening](https://help.twilio.com/articles/1260804572090-How-can-I-send-shortened-URLs-links-in-my-messages)

[Why was my Toll Free Verification Rejected?](https://support.twilio.com/hc/en-us/articles/9321443984155-Why-Was-My-Toll-Free-Verification-Rejected) 

[Required information for Toll-free verification](https://support.twilio.com/hc/en-us/articles/13264118705051-Required-Information-for-Toll-Free-Verification)

[Toll-Free best practices](https://support.twilio.com/hc/en-us/articles/360038172934-Information-and-best-practices-for-using-Toll-Free-SMS-and-MMS-in-the-US-and-Canada)

[Twilio’s Messaging Policy](https://www.twilio.com/en-us/legal/messaging-policy)"#,
            TwilioProgrammableMessagingError::ErrorCode30790 => r#"## ERROR - 30790

### Brand Registration Failure: Internal system error

 There was an internal system error when processing brand registration request.

#### Possible Causes
There was an internal system error when processing brand registration request.

#### Possible Solutions
Please contact Twilio Support."#,
            TwilioProgrammableMessagingError::ErrorCode30459 => r#"## ERROR - 30459

### Toll Free verification rejection - Disallowed: Cannabis/CBD

 The Toll-Free phone number verification submission has been reviewed and it was rejected. Effective Jan 31st, 2024, messaging traffic on this Toll-Free number is blocked until it’s verified.

#### Possible Causes
The toll-free phone number cannot be verified because the submission indicated that the business is associated with or the toll-free verification submission indicated a forbidden messaging category (Illegal Substances/articles: Cannabis/CBD).

Messages related to cannabis are not allowed in the United States as federal laws prohibit its sale, even though some states have legalized it. Similarly, messages related to CBD are not permissible in the United States, as certain states prohibit its sale. Twilio defines a cannabis message as any message which relates to the marketing or sale of a cannabis product, regardless of whether or not those messages explicitly contain cannabis terms, images, or links to cannabis websites.

#### Possible Solutions
If you are confident that your business and messaging use case does not fit into forbidden message categories, contact Twilio Support to provide Additional Information that explains your use case.

[Forbidden Message Categories](https://support.twilio.com/hc/en-us/articles/360045004974-Forbidden-message-categories-for-SMS-and-MMS-in-the-US-and-Canada)

[Why was my Toll Free Verification Rejected?](https://support.twilio.com/hc/en-us/articles/9321443984155-Why-Was-My-Toll-Free-Verification-Rejected) 

[Required information for Toll-free verification](https://support.twilio.com/hc/en-us/articles/13264118705051-Required-Information-for-Toll-Free-Verification)

[Toll-Free best practices](https://support.twilio.com/hc/en-us/articles/360038172934-Information-and-best-practices-for-using-Toll-Free-SMS-and-MMS-in-the-US-and-Canada)

[Twilio’s Messaging Policy](https://www.twilio.com/en-us/legal/messaging-policy)"#,
            TwilioProgrammableMessagingError::ErrorCode30795 => r#"## ERROR - 30795

### Brand Registration Feedback: Data mismatch related to tax id and its associated properties.

 Your Brand has received feedback from The Campaign Registry upon registration. This Brand Registration can not be used for campaign creation until the provided feedback has been addressed. In this particular case, there is a data mismatch related to the Tax ID that was provided during the registration and its associated properties such as business legal name.

#### Possible Causes
* The submitted US EIN is invalid. 
* The submitted Canadian government ID cannot be verified.
* The submitted foreign government ID cannot be verified.
* The submitted legal company name does not match with US EIN. 
* The submitted legal company name does not match with the Canadian government ID. 
* The submitted legal company name does not match with foreign government ID. 
* The submitted US EIN appears to be a US SSN.


#### Possible Solutions
Please edit your customer profile and update the business registration identifier with correct information that matches your tax returns. Please also make sure that your legal company name also matches the tax documents.

Additional help resources:

* [Troubleshooting US A2P Brand registration failures](https://www.twilio.com/docs/messaging/compliance/a2p-10dlc/troubleshooting-a2p-brands)"#,
            TwilioProgrammableMessagingError::ErrorCode30437 => r#"## ERROR - 30437

### Toll Free verification rejection - Edit time expired

 The rejected toll-free verification time window has reached the 7 day expiration. An edit is accepted, but the toll-free number remains unverified and the submission is no longer prioritized, so time to process is similar to a new submission. Effective Nov 8, 2023, messaging traffic on this Toll-Free number is blocked until it’s verified.

#### Possible Causes
There were no edits to the verification within 7 days of its rejection.

#### Possible Solutions
Edit and re-submit the toll-free verification. The resubmission will still be received, but it is handled as a new submission, so the review will take slightly longer.

[Why was my Toll Free Verification Rejected?](https://support.twilio.com/hc/en-us/articles/9321443984155-Why-Was-My-Toll-Free-Verification-Rejected) 

[Required information for Toll-free verification](https://support.twilio.com/hc/en-us/articles/13264118705051-Required-Information-for-Toll-Free-Verification)

[Toll-Free best practices](https://support.twilio.com/hc/en-us/articles/360038172934-Information-and-best-practices-for-using-Toll-Free-SMS-and-MMS-in-the-US-and-Canada)

[Twilio’s Messaging Policy](https://www.twilio.com/en-us/legal/messaging-policy)"#,
            TwilioProgrammableMessagingError::ErrorCode30887 => r#"## ERROR - 30887

### Campaign vetting rejection - Opt-out Error

 The campaign submission has been reviewed and it was rejected because of provided Opt-out information.

#### Possible Causes
The campaign cannot be approved because it was indicated that you are collecting and processing consumer Opt-Outs, but the workflow is unclear, missing opt-out keywords or opt-message.

#### Possible Solutions
Please verify that opt-out workflow is accurate and update Call-to-Action/ Message Flow description with opt-out process.

If opt-out is managed, add opt-out keywords and update opt-out message to to include acknowledgement of opt-out request, confirmation that no further messages will be sent, and brand name.
 
Additional help resources

* [A2P 10DLC campaign approval best practices](https://support.twilio.com/hc/en-us/articles/11847054539547-A2P-10DLC-Campaign-Approval-Best-Practices)"#,
            TwilioProgrammableMessagingError::ErrorCode30880 => r#"## ERROR - 30880

### Campaign vetting rejection - Unknown Error

 The campaign submission has been reviewed and it was rejected because of an unknown reason.

#### Possible Causes
The campaign cannot be approved because of an unknown error and may stem from an issue raised by other vetting parties in the ecosystem.

#### Possible Solutions
Our Support team has the means to obtain more detailed information about this specific error. Please contact [Twilio Customer Support]( https://www.twilio.com/help/contact) for assistance in understanding the underlying problem and finding a resolution. "#,
            TwilioProgrammableMessagingError::ErrorCode30899 => r#"## ERROR - 30899

### Campaign rejection - The campaign registration failed because of carrier rejection(s).

 The campaign registration failed because of carrier rejection(s).

#### Possible Causes
One or more carriers rejected this campaign upon submission.

#### Possible Solutions
Please reach out to customer support with campaign and account details for resolution. "#,
            TwilioProgrammableMessagingError::ErrorCode30905 => r#"## ERROR - 30905

### Campaign Review Pending by Twilio

 Unable to attach the Messaging Service to the Campaign as the Campaign is still under review and has not been accepted by Twilio's DCA.

#### Possible Causes
* Messaging Service was attempted to be associated before receiving the CAMPAIGN_SHARE_ACCEPT webhook from TCR.

#### Possible Solutions
* Await the CAMPAIGN_SHARE_ACCEPT webhook from TCR before assigning the Messaging Service to the CampaignID.
* Note: Campaign will remain in a 'FAILED' state until the Messaging Service is successfully re-attached.

 
Additional help resources

* [Changes to Externally Registered Campaign Submission](https://support.twilio.com/hc/en-us/articles/13097615242395-Changes-to-Externally-Registered-Campaign-Submission)"#,
            TwilioProgrammableMessagingError::ErrorCode36006 => r#"## ERROR - 36006

### Broadcast Cannot Be Canceled

 The Broadcast is in a non-cancelable state. The broadcast cannot be canceled when in any of the following statuses: EXECUTION_COMPLETED, EXECUTION_FAILURE, CANCELED.

#### Possible Causes
* The Broadcast already completed execution, either successfully or with a failure
* The Broadcast was already canceled

#### Possible Solutions
* Ensure that you provide a Broadcast SID for a Broadcast that is still in progress"#,
            TwilioProgrammableMessagingError::ErrorCode30479 => r#"## ERROR - 30479

### Toll Free verification rejection - justification needed for more than five toll free phone numbers per businesses

The Toll-Free phone number cannot be verified because you're already using 5 Toll-Free numbers. In addition, sending to Canadian handsets qualify for 1 Toll-Free number per business, as outlined in the Canadian Code of Conduct guide. For businesses requesting 6 or more toll-free phone numbers, please include a detailed explanation in the “Additional Information” field as to the reason for additional toll-free phone numbers. The Toll-Free phone number verification submission has been reviewed and it was rejected.Effective Jan 31st, 2024, messaging traffic on this Toll-Free number is blocked until it’s verified.

Verification is one component of ensuring A2P SMS is used by businesses in a legal and compliant manner. The review process looks at the business sending messaging (not the software powering it), the use case, sample content, the web presence, the opt-in/consent collected and associated website terms and privacy policies.


#### Possible Causes
Verification supports up to 5 toll-free phone numbers for a single entity in a verification submission with no additional information required for traffic intended for US subscribers. For businesses requesting 6 or more toll-free phone numbers approved for verification, please include a detailed explanation in the “Additional Information” field as to the reason for additional toll-free phone numbers. 

For ISVs or aggregators who provide messaging service to businesses, it’s expected that the information provided represents the entity (your customer) that is engaging with the opted-in consumer. Therefore, a toll-free verification should not be associated with the ISV.  Exceptions may apply when the use case clearly showcases that the ISV manages opt-in mechanisms and is the sole message content creator.

Sending to Canadian handsets qualify for 1 toll-free number per business, as outlined in the Canadian Code of Conduct guide.

#### Possible Solutions
Resubmit and explain the reason for additional toll-free phone numbers within the Additional information field.

Resources: 

[CTIA guidelines](https://www.ctia.org/the-wireless-industry/industry-commitments/messaging-interoperability-sms-mms)

[Consent/Opt-In](https://www.twilio.com/en-us/legal/messaging-policy#:~:text=with%20Twilio%20policies.-,Consent%20/%20Opt%2Din,-What%20Is%20Proper) in [Twilio’s Messaging Policy](https://www.twilio.com/en-us/legal/messaging-policy)

[Why was my Toll Free Verification Rejected?](https://support.twilio.com/hc/en-us/articles/9321443984155-Why-Was-My-Toll-Free-Verification-Rejected) 

[Required information for Toll-free verification](https://support.twilio.com/hc/en-us/articles/13264118705051-Required-Information-for-Toll-Free-Verification)

[Toll-Free best practices](https://support.twilio.com/hc/en-us/articles/360038172934-Information-and-best-practices-for-using-Toll-Free-SMS-and-MMS-in-the-US-and-Canada)"#,
            TwilioProgrammableMessagingError::ErrorCode30886 => r#"## ERROR - 30886

### Campaign vetting rejection - Invalid Campaign Description

 The campaign submission has been reviewed and it was rejected because of invalid campaign description.

#### Possible Causes
The campaign does not thoroughly describe explain the purpose or description does not match the use case.

#### Possible Solutions
Verify that campaign description is accurate and detailed. If you are an ISV registering a direct offering to your customers, please indicate that in the campaign description. Resubmit campaign for review.
 
Additional help resources

* [A2P 10DLC campaign approval best practices](https://support.twilio.com/hc/en-us/articles/11847054539547-A2P-10DLC-Campaign-Approval-Best-Practices)"#,
            TwilioProgrammableMessagingError::ErrorCode30882 => r#"## ERROR - 30882

### Campaign vetting rejection - Terms & Conditions

 The campaign submission has been reviewed and it was rejected due to Terms and Conditions issues.

#### Possible Causes
The campaign cannot be approved because affiliated marketing is not a supported use case. Terms and conditions do not support this use case.

#### Possible Solutions
Ineligible for resubmission. If you feel that this was in error, contact Twilio Customer Support.

Additional help resources

* [Forbidden message categories](https://support.twilio.com/hc/en-us/articles/360045004974-Forbidden-message-categories-for-SMS-and-MMS-in-the-US-and-Canada)

* [A2P 10DLC campaign approval best practices](https://support.twilio.com/hc/en-us/articles/11847054539547-A2P-10DLC-Campaign-Approval-Best-Practices)"#,
            TwilioProgrammableMessagingError::ErrorCode21733 => r#"## WARNING - 21733

### Default Messaging Service Not Found

 Default messaging service not found

#### Possible Causes
A default messaging service was not created for the account. 

#### Possible Solutions
Contact Twilio messaging support with the account sid and error code to make the error code disappear, the outbound message itself should still be successful."#,
            TwilioProgrammableMessagingError::ErrorCode30464 => r#"## ERROR - 30464

### Toll Free verification rejection - Disallowed: Cryptocurrency

 The Toll-Free phone number verification submission has been reviewed and it was rejected.Effective Jan 31st, 2024, messaging traffic on this Toll-Free number is blocked until it’s verified.

Verification is one component of ensuring A2P SMS is used by businesses in a legal and compliant manner. The review process looks at the business sending messaging (not the software powering it), the use case, sample content, the web presence, the opt-in/consent collected and associated website terms and privacy policies.

#### Possible Causes
The toll-free phone number cannot be verified because the submission indicated that the business is associated with or the toll-free verification submission indicated a forbidden messaging category (High-risk financial services: Cryptocurrency). 

No businesses that solely operate in stocks, investing, or cryptocurrency are allowed to send SMS traffic. If there is a mixed use case where that is a partial aspect of the business it may be approved based on the other use case content.

#### Possible Solutions
If you are confident that your business and messaging use case does not fit into forbidden message categories, contact Twilio Support to provide Additional Information that explains your use case.

[Forbidden Message Categories](https://support.twilio.com/hc/en-us/articles/360045004974-Forbidden-message-categories-for-SMS-and-MMS-in-the-US-and-Canada)

[Why was my Toll Free Verification Rejected?](https://support.twilio.com/hc/en-us/articles/9321443984155-Why-Was-My-Toll-Free-Verification-Rejected) 

[Required information for Toll-free verification](https://support.twilio.com/hc/en-us/articles/13264118705051-Required-Information-for-Toll-Free-Verification)

[Toll-Free best practices](https://support.twilio.com/hc/en-us/articles/360038172934-Information-and-best-practices-for-using-Toll-Free-SMS-and-MMS-in-the-US-and-Canada)

[Twilio’s Messaging Policy](https://www.twilio.com/en-us/legal/messaging-policy)"#,
            TwilioProgrammableMessagingError::ErrorCode30752 => r#"## ERROR - 30752

### Brand Registration Failure: Invalid or expired OTP

 You have used either an invalid or an expired OTP for brand verification.

#### Possible Causes
You have used either an invalid or an expired OTP for brand verification.

#### Possible Solutions
Please use a valid OTP for verifying your Sole Proprietor brand. You can use our public api or console to request a latest OTP and use that for verification.

Additional help resources

* [Troubleshooting US A2P Brand registration failures]"#,
            TwilioProgrammableMessagingError::ErrorCode35132 => r#"## ERROR - 35132

### Bulk : Provided Attributes JSON is not valid

 Provided Attributes are not in the expected JSON format

#### Possible Causes
Provided attributes is not a valid JSON

#### Possible Solutions
Check if all the attributes are in expected format"#,
            TwilioProgrammableMessagingError::ErrorCode21268 => r#"## ERROR - 21268

### Sending to Premium rate or Information Service numbers is not allowed

 Twilio doesn't allow sending SMS messages to premium rate or information service numbers. 


#### Possible Causes
You attempted to send an SMS to premium rate or information service numbers.


#### Possible Solutions
Twilio doesn't allow sending SMS messages to premium rate or information service numbers. 
"#,
            TwilioProgrammableMessagingError::ErrorCode30472 => r#"## ERROR - 30472

### Toll Free verification rejection - Could not verify Business

The Toll-Free phone number cannot be verified because we are unable to verify the business. You may have errors in or multiple business information fields: Business Name, Business Street Address, Business City, Business State/Province/Region, Business Postal Code, Business Country, Business Contact First Name, Business Contact Last Name, Business Contact Email, Business Contact Phone, Business Website. Please correct the business information fields and resubmit The Toll-Free phone number verification submission has been reviewed and it was rejected.Effective Jan 31st, 2024, messaging traffic on this Toll-Free number is blocked until it’s verified.

Verification is one component of ensuring A2P SMS is used by businesses in a legal and compliant manner. The review process looks at the business sending messaging (not the software powering it), the use case, sample content, the web presence, the opt-in/consent collected and associated website terms and privacy policies.


#### Possible Causes
The toll-free phone number cannot be verified. The business could not be verified, this may not be a valid business. 

#### Possible Solutions
- Verify that the business is a valid business
- Ensure all business information fields are valid and accurately represent the business who manages the opt-in and has the relationship with the consumer:

  **BusinessName**, **BusinessStreetAddress**, **BusinessCity**, **BusinessStateProvinceRegion**, **BusinessPostalCode**, **BusinessCountry**, **BusinessContactFirstName**, **BusinessContactLastName**, **BusinessContactEmail**, **BusinessContactPhone**, **BusinessWebsite**
- Correct the business information fields on toll-free verification submission by editing it and then resubmit it.

[Why was my Toll Free Verification Rejected?](https://support.twilio.com/hc/en-us/articles/9321443984155-Why-Was-My-Toll-Free-Verification-Rejected) 

[Required information for Toll-free verification](https://support.twilio.com/hc/en-us/articles/13264118705051-Required-Information-for-Toll-Free-Verification)

[Toll-Free best practices](https://support.twilio.com/hc/en-us/articles/360038172934-Information-and-best-practices-for-using-Toll-Free-SMS-and-MMS-in-the-US-and-Canada)

[Twilio’s Messaging Policy](https://www.twilio.com/en-us/legal/messaging-policy)
"#,
            TwilioProgrammableMessagingError::ErrorCode30135 => r#"## ERROR - 30135

### Unable to issue certificate

 

#### Possible Causes
Domain DNS setup is not completed.
There is an issue with the certificate authority

#### Possible Solutions
Please verify your Domain DNS setup. If you continue seeing this error, please contact the Support team"#,
            TwilioProgrammableMessagingError::ErrorCode30898 => r#"## ERROR - 30898

### Campaign vetting rejection - Excessive EIN  

 The campaign submission has been reviewed and it was rejected because of repeated use of same for multiple brands.

#### Possible Causes
The campaign cannot be approved because it the same EIN is used for multiple brands. 

#### Possible Solutions
Only register minimum number of brands per EIN and do not resubmit until brand registration is updated.

Additional help resources

* [A2P 10DLC campaign approval best practices](https://support.twilio.com/hc/en-us/articles/11847054539547-A2P-10DLC-Campaign-Approval-Best-Practices)"#,
            TwilioProgrammableMessagingError::ErrorCode21265 => r#"## ERROR - 21265

### 'To' number cannot be a Short Code

 You attempted to send an SMS, but the `To` phone number you supplied was a [Short Code](https://www.twilio.com/docs/glossary/what-is-a-short-code). Twilio numbers cannot send messages to short codes at this time whether they are Twilio short codes or external short codes.

#### Possible Causes
You attempted to send an SMS to Short Code.


#### Possible Solutions
Ensure you are not attempting to send messages to Short Codes. These sender types cannot receive messages from Twilio numbers."#,
            TwilioProgrammableMessagingError::ErrorCode30904 => r#"## ERROR - 30904

### Campaign Not Shared with Twilio

 Messaging Service could not be associated with the Campaign because the specific Campaign was not found in Twilio's DCA records.

#### Possible Causes
* Campaign has not been shared with Twilio's DCA.
* Provided campaignID is incorrect.

#### Possible Solutions
* Ensure that the Campaign has been successfully shared with Twilio as the DCA and has received approval.
* Confirm that the entered CampaignID is accurate.
* If needed, re-share the Campaign through TCR, await the CAMPAIGN_SHARE_ACCEPT webhook, and then proceed to assign the Messaging Service.

Additional help resources
* [Changes to Externally Registered Campaign Submission](https://support.twilio.com/hc/en-us/articles/13097615242395-Changes-to-Externally-Registered-Campaign-Submission)
"#,
            TwilioProgrammableMessagingError::ErrorCode30791 => r#"## ERROR - 30791

### Brand Registration Failure: Temporary system error

 There was a temporary system error when processing brand registration request.



#### Possible Causes
There was a temporary system error when processing brand registration request.



#### Possible Solutions
Please try again after 30 seconds.

"#,
            TwilioProgrammableMessagingError::ErrorCode30794 => r#"## ERROR - 30794

### US A2P Registration: General error

 There was a general system error when processing your registration request.

#### Possible Causes
Your US A2P Brand or Campaign registration request was not processed successfully. While our systems were not able to identify the exact cause of failure, the error description provided as part of registration response should indicate the possible issue. 

#### Possible Solutions
Please fix the cause of error message and retry the registration request. If issue persists, please contact Twilio support.

Additional help resources

* [Troubleshooting US A2P Brand registration failures](https://www.twilio.com/docs/messaging/compliance/a2p-10dlc/troubleshooting-a2p-brands)

* [Troubleshooting and rectifying US A2P Campaigns](https://www.twilio.com/docs/messaging/compliance/a2p-10dlc/troubleshooting-a2p-brands/troubleshooting-and-rectifying-a2p-campaigns-1)

"#,
            TwilioProgrammableMessagingError::ErrorCode30889 => r#"## ERROR - 30889

### Campaign vetting rejection - Embedded Phone Number

 The campaign submission has been reviewed and it was rejected because Embedded Phone Number selection.

#### Possible Causes
The campaign cannot be approved because embedded phone number use is selected, but is not reflective in the sample message.

#### Possible Solutions
Please verify that embedded phone number selection is accurate. Update the Sample Messages with Embedded Phone Number or update Embedded Phone Number selection. Resubmit campaign for review.
 
Additional help resources

* [A2P 10DLC campaign approval best practices](https://support.twilio.com/hc/en-us/articles/11847054539547-A2P-10DLC-Campaign-Approval-Best-Practices)"#,
            TwilioProgrammableMessagingError::ErrorCode30478 => r#"## ERROR - 30478

### Toll Free verification rejection - single phone number used for multiple businesses

 The Toll-Free phone number verification submission has been reviewed and it was rejected. Effective Jan 31st, 2024, messaging traffic on this Toll-Free number is blocked until it’s verified.

Verification is one component of ensuring A2P SMS is used by businesses in a legal and compliant manner. The review process looks at the business sending messaging (not the software powering it), the use case, sample content, the web presence, the opt-in/consent collected and associated website terms and privacy policies.


#### Possible Causes
A toll-free verification has been received for a toll-free phone number that is assigned to multiple businesses. The toll-free phone number must be 1:1 with a single business, it cannot be used for multiple businesses. 

#### Possible Solutions
* Use the toll-free phone number for only one business.  
* Resubmit the verification and provide Additional Information that explains how you rectified this. 

Resources: 

[Why was my Toll Free Verification Rejected?](https://support.twilio.com/hc/en-us/articles/9321443984155-Why-Was-My-Toll-Free-Verification-Rejected) 

[Required information for Toll-free verification](https://support.twilio.com/hc/en-us/articles/13264118705051-Required-Information-for-Toll-Free-Verification)

[Toll-Free best practices](https://support.twilio.com/hc/en-us/articles/360038172934-Information-and-best-practices-for-using-Toll-Free-SMS-and-MMS-in-the-US-and-Canada)

[Twilio’s Messaging Policy](https://www.twilio.com/en-us/legal/messaging-policy)"#,
            TwilioProgrammableMessagingError::ErrorCode63040 => r#"## ERROR - 63040

### Template Rejected

 This message matches a template that was rejected by WhatsApp or failed to be submitted. For the most updated guidelines, please view WhatsApp’s documentation: https://developers.facebook.com/docs/whatsapp/message-templates/guidelines.

#### Possible Causes
* Template purpose is not clear, concise, or well-written. For example, "Hi, {{1 2 Thank you.}}" is vague and not clear how it would be used or what the placeholder values represent.
* Template placeholders are not sequential in order. For example, "{{1 and 2 and 4 }}" are included but the placeholder {{3}} does not exist in the template.
* Template placeholders cannot be right next to each other like "{}{{1 2{}}}".
* Template content is identical to the content of another existing template.
* Template body cannot start or end with a placeholder.
* Template body cannot have the \n newline character.
* Template footer cannot have emojis or the \n newline character.
* Templates with Text headers cannot have emojis, asterisks, formatting markup, or the \n newline character.
* Templates with Media headers (video, image, document) must include a sample with their submission.
* Template is abusive or threatening in nature, such as containing content threatening to take legal action against a user.

For the most updated guidelines, please view WhatsApp’s documentation: https://developers.facebook.com/docs/whatsapp/message-templates/guidelines.

This error may also occur if you have an approved template that has similar body to a rejected template.

#### Possible Solutions
Submit a new template that doesn’t violate the above causes. We also recommend the follow best practices when creating templates:
  * Templates should be as short as possible to get the necessary information across.
  * Templates must not have any grammatical or spelling mistakes. 
  * Templates that include major spelling or grammatical mistakes are likely to be rejected. Templates with minor punctuation errors or grammatical inconsistencies may be approved, but should be avoided.
  * When submitting your Template in Console, ensure you select the appropriate message type and language.
  * Get feedback from your teammates or coworkers to ensure the template you’ve written is clear and free of errors.
  * Think about a template message as a conversation starter, with the goal to convert this into a 2-way message by having the user reply. The two-way use case will keep your spend lower, as WhatsApp does not charge the Template message fee for two-way messages.
  * Make sure your customers understand why they received your message – you can remind them of the reason why they originally gave you permission to send them messages.
  * Avoid sending surveys or using the word “survey”. Instead, you may ask the customer for their feedback.

If you believe you have an approved template that matches the message you are attempting to send, then we recommend deleting any rejected templates that are similar to the approved template. You may also use the Twilio Content API, which lets you send templates by referencing a template Sid. To learn more, please see: https://www.twilio.com/docs/content-api"#,
            TwilioProgrammableMessagingError::ErrorCode30909 => r#"## ERROR - 30909

### Campaign vetting rejection - CTA Verification Issue

 The campaign submission has been reviewed and rejected due to issues verifying the Call to Action (CTA) provided for the campaign.

#### Possible Causes
* *Unverifiable CTA*: The Call to Action associated with the campaign could not be verified during the review process.  If opt-in occurs outside of the website,then provided information was incomplete or not fully detailed.


#### Possible Solutions
1. Provide complete CTA information and all possible methods of receiving consent. If opt-in is not conducted through the website, please detail and provide documentation how the CTA is presented to and actioned by end-users.
 Once you have made a change to address CTA verification issues, please resubmit the campaign for review.

Additional help resources

* [A2P 10DLC campaign approval best practices](https://support.twilio.com/hc/en-us/articles/11847054539547-A2P-10DLC-Campaign-Approval-Best-Practices)
"#,
            TwilioProgrammableMessagingError::ErrorCode30443 => r#"## ERROR - 30443

### Toll-Free verification rejection - Disallowed: Loan Marketing

 The Toll-Free phone number verification submission has been reviewed and it was rejected. Effective Jan 31st, 2024, messaging traffic on this Toll-Free number is blocked until it’s verified.

Verification is one component of ensuring A2P SMS is used by businesses in a legal and compliant manner. The review process looks at the business sending messaging (not the software powering it), the use case, sample content, the web presence, the opt-in/consent collected and associated website terms and privacy policies.

#### Possible Causes
The toll-free phone number cannot be verified because the submission indicated that the business is associated with or the toll-free verification submission indicated a forbidden messaging category (High-risk financial services: New loan soliciting). 

New loan soliciting, Payday loans, Short term high-interest loans, Third-party loans and Student loans are disallowed content. 

"Third-party" means originating from any party other than the one which will service the loan. Examples of third-party loans could include: auto, mortgage, personal, etc. First party loan content is acceptable if it is not promotional messaging.

Note: Any business with a terms of service or privacy policy that mentions sharing or selling consumer data/opt-in information is considered noncompliant.


#### Possible Solutions
If you are confident that your business and messaging use case does not fit into forbidden message categories, contact Twilio Support to provide Additional Information that explains your use case.


[Forbidden Message Categories](https://support.twilio.com/hc/en-us/articles/360045004974-Forbidden-message-categories-for-SMS-and-MMS-in-the-US-and-Canada)

[Why was my Toll Free Verification Rejected?](https://support.twilio.com/hc/en-us/articles/9321443984155-Why-Was-My-Toll-Free-Verification-Rejected) 

[Required information for Toll-free verification](https://support.twilio.com/hc/en-us/articles/13264118705051-Required-Information-for-Toll-Free-Verification)

[Toll-Free best practices](https://support.twilio.com/hc/en-us/articles/360038172934-Information-and-best-practices-for-using-Toll-Free-SMS-and-MMS-in-the-US-and-Canada)

[Twilio’s Messaging Policy](https://www.twilio.com/en-us/legal/messaging-policy)"#,
            TwilioProgrammableMessagingError::ErrorCode30462 => r#"## ERROR - 30462

### Toll Free verification rejection - Disallowed: Sweepstakes

 The Toll-Free phone number verification submission has been reviewed and it was rejected. Effective Jan 31st, 2024, messaging traffic on this Toll-Free number is blocked until it’s verified.

Verification is one component of ensuring A2P SMS is used by businesses in a legal and compliant manner. The review process looks at the business sending messaging (not the software powering it), the use case, sample content, the web presence, the opt-in/consent collected and associated website terms and privacy policies.

#### Possible Causes
The toll-free phone number cannot be verified because the submission indicated that the business is associated with or the toll-free verification submission indicated a forbidden messaging category (Gambling - Sweepstakes). 

Gambling traffic is prohibited in the US and Canada on all number types (Toll Free, Short Code, Long Code). The gambling category includes, but is not limited to:
 
* Casino apps
* Websites that offer gambling
* Sweepstakes
* 50/50 Raffles
* Contests
* Betting/Sports picks

#### Possible Solutions
If you are confident that your business and messaging use case does not fit into forbidden message categories, contact Twilio Support to provide Additional Information that explains your use case.


[Forbidden Message Categories](https://support.twilio.com/hc/en-us/articles/360045004974-Forbidden-message-categories-for-SMS-and-MMS-in-the-US-and-Canada)

[Why was my Toll Free Verification Rejected?](https://support.twilio.com/hc/en-us/articles/9321443984155-Why-Was-My-Toll-Free-Verification-Rejected) 

[Required information for Toll-free verification](https://support.twilio.com/hc/en-us/articles/13264118705051-Required-Information-for-Toll-Free-Verification)

[Toll-Free best practices](https://support.twilio.com/hc/en-us/articles/360038172934-Information-and-best-practices-for-using-Toll-Free-SMS-and-MMS-in-the-US-and-Canada)

[Twilio’s Messaging Policy](https://www.twilio.com/en-us/legal/messaging-policy)"#,
            TwilioProgrammableMessagingError::ErrorCode63024 => r#"## ERROR - 63024

### Invalid message recipient

 For WhatsApp, the message is undeliverable due to the recipient being an invalid user on WhatsApp or other Meta restrictions for WhatsApp.

#### Possible Causes
For WhatsApp, reasons can include:

* Recipient phone number is not a WhatsApp enabled phone number.
* Recipient has not accepted WhatsApp's Terms of Service and Privacy Policy.
* Recipient is using an old, unsupported version of the WhatsApp client for their phone.

#### Possible Solutions
* Confirm with the recipient that they have accepted Meta's latest Terms of Service for WhatsApp. If they have not, then send them this link https://wa.me/tos/20210210 to accept the latest WhatsApp Terms of Service.
* Confirm with the recipient that they have an active WhatsApp phone number on the latest client version for their phone."#,
            TwilioProgrammableMessagingError::ErrorCode30136 => r#"## ERROR - 30136

### Unable to renew certificate

 

#### Possible Causes
Domain DNS setup is not completed.
There is an issue with the certificate authority

#### Possible Solutions
Please verify your Domain DNS setup. If you continue seeing this error, please contact the Support team"#,
            TwilioProgrammableMessagingError::ErrorCode30703 => r#"## ERROR - 30703

### Brand Registration Failure: Duplicate record detected

 Brand registration failed due to duplicate record detection. Following attributes are expected to be unique for each brand and can fail with this duplication check.

* Mobile phone number
* Business registration identifier
* Business legal name
* Authorised representative's phone number
* Email address
* Website
* Stock symbol (applicable only for public entities)





#### Possible Causes
* The mobile phone number provided in the brand registration request has already been used to register a US A2P Brand with TCR. 

* An existing brand is using the provided business identification type and number.

* An existing brand is using the provided business legal name.

* Authorised representative's phone number provided for brand registration has already been registered. Please retry with a different phone number.

* Email address provided for brand registration has already been registered with another brand. Please try again with a different email address.

* Website provided for brand registration has already been registered. Please try again with a different website url.

* Business name provided for brand registration has already been registered. Business names are expected to be unique across all brands.

* Stock symbol provided for brand registration has already been registered

#### Possible Solutions
Depending on the fields you may have received in the error message, please refer to the following solutions:

* Use a unique US mobile phone number for the registration. 

* Use a unique business registration identifier for successful registration.

* Use a unique business name.

* Use a unique authorized representative.

* Use a unique email address.

* Use a unique website address.

* Use a unique business name.

* Use a unique stock symbol.


Additional help resources

* [Troubleshooting US A2P Brand registration failures](https://www.twilio.com/docs/messaging/compliance/a2p-10dlc/troubleshooting-a2p-brands)"#,
            TwilioProgrammableMessagingError::ErrorCode30799 => r#"## ERROR - 30799

### Brand Registration Feedback: We were unable to verify the details of the registration data.

 Your Brand has received feedback from The Campaign Registration upon registration. This brand registration can not be used for campaign creation till the provided feedback has been addressed. In this case, it wasn’t possible to determine the exact cause of the failure. Please check the possible causes section to identify if the provided details for brand registration don’t fall into any of the mentioned categories.

#### Possible Causes
#### If your company is publicly listed and has an EIN, then it is possible that:

* The submitted US EIN is invalid. 

* The submitted Canadian government ID cannot be verified.

* The submitted foreign government ID cannot be verified.

* The submitted legal company name does not match with US EIN. 

* The submitted legal company name does not match with the Canadian government ID. 

* The submitted legal company name does not match with foreign government ID. 

* The submitted US EIN appears to be a US SSN. 

#### If you provided a stock exchange and stock ticker symbol, then it is possible that:

* The submitted stock symbol and stock exchange could not be verified for the submitted legal company name. 

* The submitted stock exchange is not recognized.  

#### If your company is a registered government entity, then it is possible that

*  The submitted company is not a US entity. Only US entities qualify for the Government Entity Type. 

*  The company’s submission as a Government Entity type could not be confirmed from independent sources. 

#### If your company is a non-profit entity, then it is possible that

* The submitted company is not a US entity. Only US entities qualify for the Non-Profit Entity Type. 

* The submitted company is not verified as a US IRS recognized non-profit organization. 

* The submitted company is verified as a US IRS recognized non-profit organization but the charitable subsection code was not reported.


#### Possible Solutions
#### For public entities:

Please edit your customer profile and update the business registration identifier with correct information that matches your tax returns. Please also make sure that your legal company name also matches the tax documents.

#### For entities that are listed on a stock exchange:

Please edit your A2P profile and update the stock_symbol and/or stock_exchange with correct information. 

#### For government entities:

Please change the entity_type field value to something other than Government. If you are a US Entity, please update the customer profile address to indicate the same.

#### For non-profit organisations:

Please change the entity_type field value to something other than Non-Profit. If you are a Non-Profit Entity, please update the customer profile to indicate the same.

In case none of the above meet your case, please contact twilio support for further resolution.

Additional help resources

* [Troubleshooting US A2P Brand registration failures](https://www.twilio.com/docs/messaging/compliance/a2p-10dlc/troubleshooting-a2p-brands)
"#,
            TwilioProgrammableMessagingError::ErrorCode11206 => r#"## ERROR - 11206

### HTTP protocol violation

 There was an error speaking HTTP to the target URL.


#### Possible Causes
* Server misconfiguration
* Network disruption
* Making HTTP connections to an HTTPS port
* There is an invalid symbol in the TwiML Cookie

#### Possible Solutions
* Verify your web server is serving valid HTTP for the URL	
* Verify the network connection to your server is stable	
* Verify the URL is not directing you to an SSL port
* Remove the following characters from the TwiML Cookies: ‘\t’, ‘\r’, ‘\n’, ‘\v’, ‘\f’
* TwiML cookies cannot have an empty name
"#,
            TwilioProgrammableMessagingError::ErrorCode63042 => r#"## ERROR - 63042

### Template disabled

 WhatsApp has reported that the template has been **permanently disabled** due to recurring negative feedback from customers. Message templates with this status cannot be sent to customers. For the most updated guidelines, please view WhatsApp’s documentation: https://developers.facebook.com/docs/whatsapp/message-templates/guidelines/#template-pausing.


#### Possible Causes
Negative customer feedback (i.e. multiple customers reporting messages or blocking number)

#### Possible Solutions
- Use a different template
- Create a new template"#,
            TwilioProgrammableMessagingError::ErrorCode30712 => r#"## ERROR - 30712

### Brand Registration Failure: Max registration limit reached

 Your account has reached maximum number allowed brand registrations.

#### Possible Causes
Your account has reached maximum number allowed brand registrations.

#### Possible Solutions
Please contact Twilio Support with relevant error details.

Additional help resources

* [Troubleshooting US A2P Brand registration failures](https://www.twilio.com/docs/messaging/compliance/a2p-10dlc/troubleshooting-a2p-brands)"#,
            TwilioProgrammableMessagingError::ErrorCode30466 => r#"## ERROR - 30466

### Toll Free verification rejection - Disallowed - Debt Reduction

 The Toll-Free phone number verification submission has been reviewed and it was rejected.Effective Jan 31st, 2024, messaging traffic on this Toll-Free number is blocked until it’s verified.

Verification is one component of ensuring A2P SMS is used by businesses in a legal and compliant manner. The review process looks at the business sending messaging (not the software powering it), the use case, sample content, the web presence, the opt-in/consent collected and associated website terms and privacy policies.

#### Possible Causes
The toll-free phone number cannot be verified because the submission indicated that the business is associated with or the toll-free verification submission indicated a forbidden messaging category  (Debt collection or forgiveness: Debt reduction). 

Debt consolidation, debt reduction and credit repair programs are prohibited regardless if there is first-party consent. 


#### Possible Solutions
If you are confident that your business and messaging use case does not fit into forbidden message categories, contact Twilio Support to provide Additional Information that explains your use case.

[Forbidden Message Categories](https://support.twilio.com/hc/en-us/articles/360045004974-Forbidden-message-categories-for-SMS-and-MMS-in-the-US-and-Canada)

[Why was my Toll Free Verification Rejected?](https://support.twilio.com/hc/en-us/articles/9321443984155-Why-Was-My-Toll-Free-Verification-Rejected) 

[Required information for Toll-free verification](https://support.twilio.com/hc/en-us/articles/13264118705051-Required-Information-for-Toll-Free-Verification)

[Toll-Free best practices](https://support.twilio.com/hc/en-us/articles/360038172934-Information-and-best-practices-for-using-Toll-Free-SMS-and-MMS-in-the-US-and-Canada)

[Twilio’s Messaging Policy](https://www.twilio.com/en-us/legal/messaging-policy)
"#,
            TwilioProgrammableMessagingError::ErrorCode21661 => r#"## ERROR - 21661

### 'From' number is not SMS-capable

 Phone numbers must be SMS-capable. Visit the [phone numbers page](https://www.twilio.com/console/phone-numbers/incoming) of your account portal to view a list of phone numbers that you own and to determine whether they are SMS-capable.


#### Possible Causes
The number you are using is not capable of sending messages.

#### Possible Solutions
* Check that you are using a Twilio phone number with SMS capabilities. You can see your purchased phone numbers and their capabilities in the [Twilio console](https://www.twilio.com/console/phone-numbers/incoming).
* Purchase an SMS capable number from the [Twilio Console](https://console.twilio.com/us1/develop/phone-numbers/manage/search?isoCountry=US&types%5B%5D=Local&types%5B%5D=Tollfree&capabilities%5B%5D=Sms&searchTerm=&searchFilter=left&searchType=number)."#,
            TwilioProgrammableMessagingError::ErrorCode30756 => r#"## ERROR - 30756

### Brand Registration Failure: Obfuscation check failure

  Obfuscation check failed for brand registration inputs.

#### Possible Causes
 Obfuscation check failed for brand registration inputs.

#### Possible Solutions
Please use natural text inputs for business name, first and last name.

Additional help resources

* [Troubleshooting US A2P Brand registration failures]"#,
            TwilioProgrammableMessagingError::ErrorCode30747 => r#"## ERROR - 30747

### Brand Registration Failure: Address duplicate threshold reached

 Brand can not be registered with provided address since duplicate threshold limit has been reached for it.

#### Possible Causes
Reusing addresses for Sole Proprietor Brands is not allowed. Each Sole Proprietor Brand should have a unique address linked to it. It is possible that the Starter customer profile bundle associated with the brand you tried to register has a duplicate address linked to it. 

#### Possible Solutions
Please edit the connected starter customer profile and link a different (unique) address to it. Once updated and submitted for review, please resubmit the Sole Proprietor Brand registration request.

Additional help resources

* [Troubleshooting US A2P Brand registration failures]"#,
            TwilioProgrammableMessagingError::ErrorCode30881 => r#"## ERROR - 30881

### Campaign vetting rejection - Invalid Brand Support Email

 The campaign submission has been reviewed and it was rejected because invalid brand support email.

#### Possible Causes
The campaign cannot be approved because support email needs is not associated with the brand. The brand support email was either invalid or associated with a public domain email.

#### Possible Solutions
Please verify that brand support email is valid and not associated with a public domain email.
 
Additional help resources

* [A2P 10DLC campaign approval best practices](https://support.twilio.com/hc/en-us/articles/11847054539547-A2P-10DLC-Campaign-Approval-Best-Practices)"#,
            TwilioProgrammableMessagingError::ErrorCode30615 => r#"## ERROR - 30615

### Message couldn't be delivered

 Message couldn’t be delivered to the recipients number you are trying to reach at this time due to local regulatory restrictions.

#### Possible Causes
Marketing, promotional & other non essential messages are restricted under the local regulatory requirement (e.g.TCPA in US) to certain hours of the day. Specifically, these communications cannot be made before 8 a.m. or after 9 p.m. local time at the recipient's location. This restriction is intended to protect consumers from intrusive communications during times they are likely to be resting or engaged in personal activities.

#### Possible Solutions
**Verify Local Time:** Implement a system to accurately determine the local time zone of each recipient based on their area code and address data.

**Schedule Adjustments:** Configure your messaging campaign schedules to operate only within the permitted hours of 8 a.m. to 9 p.m. local time for each recipient.

#### Continued issues with error 30615

Twilio's support team can help investigate what went wrong with delivering your message. Please collect 3 or more message SIDs in your SMS [logs](https://www.twilio.com/console/sms/logs) from the last 24 hours that were flagged with Error 30615, and [open a support request.](https://www.twilio.com/console/support/tickets/create)"#,
            TwilioProgrammableMessagingError::ErrorCode30458 => r#"## ERROR - 30458

### Toll Free verification rejection - Disallowed: SHAFT - Tobacco / Vape

 The Toll-Free phone number verification submission has been reviewed and it was rejected. Effective Jan 31st, 2024, messaging traffic on this Toll-Free number is blocked until it’s verified.

Verification is one component of ensuring A2P SMS is used by businesses in a legal and compliant manner. The review process looks at the business sending messaging (not the software powering it), the use case, sample content, the web presence, the opt-in/consent collected and associated website terms and privacy policies.

#### Possible Causes
The toll-free phone number cannot be verified because the submission indicated that the business is associated with or the toll-free verification submission indicated a forbidden messaging category (SHAFT: Tobacco/Vape). 

Firearms, Tobacco, Vape, and E-cigarettes are not allowed on Toll Free, Short Code, or Long Code regardless of age gating.

#### Possible Solutions
If you are confident that your business and messaging use case does not fit into forbidden message categories, contact Twilio Support to provide Additional Information that explains your use case.

[Forbidden Message Categories](https://support.twilio.com/hc/en-us/articles/360045004974-Forbidden-message-categories-for-SMS-and-MMS-in-the-US-and-Canada)

[Why was my Toll Free Verification Rejected?](https://support.twilio.com/hc/en-us/articles/9321443984155-Why-Was-My-Toll-Free-Verification-Rejected) 

[Required information for Toll-free verification](https://support.twilio.com/hc/en-us/articles/13264118705051-Required-Information-for-Toll-Free-Verification)

[Toll-Free best practices](https://support.twilio.com/hc/en-us/articles/360038172934-Information-and-best-practices-for-using-Toll-Free-SMS-and-MMS-in-the-US-and-Canada)

[Twilio’s Messaging Policy](https://www.twilio.com/en-us/legal/messaging-policy)"#,
            TwilioProgrammableMessagingError::ErrorCode30755 => r#"## ERROR - 30755

### Brand Registration Failure: Unsupported country code

 You have used a country code which is not yet support for US A2P brand registrations.

#### Possible Causes
You have used an country code which is not yet support for US A2P brand registrations. Supported list of country codes are : PR,PS,PT,PW,PY,QA,AD,AE,AF,AG,AI,AL,AM,AO,AQ,AR,AS,AT,RE,AU,AW,AX,AZ,RO,BA,BB,RS,BD,RU,BE,BF,BG,RW,BH,BI,BJ,BL,BM,BN,BO,SA,SB,BQ,SC,BR,BS,SD,SE,BT,SG,BV,SH,BW,SI,SJ,BY,BZ,SK,SL,SM,SN,SO,CA,SR,CC,SS,ST,CD,CF,SV,CG,SX,CH,CI,SZ,CK,CL,CM,CN,CO,CR,TC,TD,TF,TG,CV,CW,TH,CX,TJ,CY,TK,CZ,TL,TM,TN,TO,TR,TT,DE,TV,TW,DJ,TZ,DK,DM,DO,UA,UG,DZ,UM,EC,US,EE,EG,EH,UY,UZ,VA,ER,VC,ES,VE,ET,VG,VI,VN,VU,FI,FJ,FK,FM,FO,FR,WF,GA,GB,WS,GD,GE,GF,GG,GH,GI,GL,GM,GN,GP,GQ,GR,GS,GT,XE,GU,GW,GY,XK,HK,HM,HN,HR,YE,HT,HU,YT,ID,IE,IL,IM,IN,IO,ZA,IQ,IS,IT,ZM,JE,ZW,JM,JO,JP,KE,KG,KH,KI,KM,KN,KR,KW,KY,KZ,LA,LB,LC,LI,LK,LR,LS,LT,LU,LV,LY,MA,MC,MD,ME,MF,MG,MH,MK,ML,MM,MN,MO,MP,MQ,MR,MS,MT,MU,MV,MW,MX,MY,MZ,NA,NC,NE,NF,NG,NI,NL,NO,NP,NR,NU,NZ,OM,PA,PE,PF,PG,PH,PK,PL,PM,PN

#### Possible Solutions
Please use a valid country code.


Additional help resources

* [Troubleshooting US A2P Brand registration failures]"#,
            TwilioProgrammableMessagingError::ErrorCode36002 => r#"## ERROR - 36002

### Broadcast Request Limit Reached

 The limit of the number of Broadcast requests for this account has been reached

#### Possible Causes
* You are at the limit of Broadcast requests for your account and cannot create any additional Broadcasts

#### Possible Solutions
* Wait for your existing Broadcasts to process before creating more Broadcasts
* If you have any Broadcasts with status PENDING_UPLOAD, upload a file to those Broadcasts first to resume processing on them"#,
            TwilioProgrammableMessagingError::ErrorCode63044 => r#"## ERROR - 63044

### Link to a sample media file returns 404 Not Found

 The link returned a status code of 404 Not Found, making it impossible to retrieve the file.

#### Possible Causes
- The file does not exist 
- The URL provided is incorrect

#### Possible Solutions
- Update the provided URL"#,
            TwilioProgrammableMessagingError::ErrorCode30792 => r#"## ERROR - 30792

### Brand Registration Failure: General error

 There was a general system error when processing brand registration request.

#### Possible Causes
There was a general system error when processing brand registration request.

#### Possible Solutions
Please contact Twilio Support."#,
            TwilioProgrammableMessagingError::ErrorCode30900 => r#"## ERROR - 30900

### Campaign rejection - The campaign use case is ineligible for registration.

 The campaign use case provided as part of registration request is ineligible for registration.

#### Possible Causes
Reasons for this failure can be: 

* Provided use case is not supported yet.
* Campaign use case not supported by one of the carriers.
* The brand is not qualified to run campaign of provide use case for one of the carriers.
* The brand with use case campaign is not within sub-usecase bounds.

#### Possible Solutions
Try to select another use case for campaign registration that reflects your A2P Messaging use case with Twilio, or reach out to support with campaign details for resolution. "#,
            TwilioProgrammableMessagingError::ErrorCode63049 => r#"## ERROR - 63049

### Meta chose not to deliver this WhatsApp marketing message

 Meta may limit the number of marketing template messages a person receives from any business in a given period of time, starting with a small number of conversations that are less likely to be read. 

WhatsApp has found that per-user marketing template limits maximize message engagement and improve the user experience, measured through improvements in user read rates and sentiment. This limit helps WhatsApp users find business messaging more valuable and feel less like they receive too many business messages.



#### Possible Causes
Meta blocked the delivery of this marketing template messages to the recipient.

#### Possible Solutions
The limit only applies to marketing template messages that would normally open a new marketing conversation. If a marketing conversation is already open between you and a WhatsApp user, you may send one additional marketing template message. Further marketing template messages can only be sent in an open marketing conversation if the person responds to any message.

Example:

- The first marketing template message is delivered and opens a new 24-hour marketing conversation customer service window. The per-user marketing template message limit applies

- A second marketing template message can be sent in an existing conversation.

- Each time the WhatsApp user responds in an existing conversation window, you can send one additional marketing template message. You can also send unlimited free-form messages."#,
            TwilioProgrammableMessagingError::ErrorCode30477 => r#"## ERROR - 30477

### Toll Free verification rejection - Opt-in - Third party information sharing not allowed

The Toll-Free phone number cannot be verified because your Terms of Service or Privacy Policy indicates that Opt-in is shared with third parties, affiliates, partners, etc. Any business with a terms of service or privacy policy that mentions sharing or selling consumer opt-in data is considered noncompliant as consent cannot be bought, sold, shared, transferred, or exchanged. Please do the following and then resubmit: 1. Revise your processes to not share or sell consumer data/opt-in information with anyone; 2. Revise Terms of Service and Privacy Policy on your website to remove all third party opt-in sharing language; 3. If you have a “reasons we can share your personal information matrix” in your privacy policy that is not modifiable, adding a note below it that Messaging opt-in consent will not be shared is acceptable. The Toll-Free phone number verification submission has been reviewed and it was rejected.Effective Jan 31st, 2024, messaging traffic on this Toll-Free number is blocked until it’s verified.

Verification is one component of ensuring A2P SMS is used by businesses in a legal and compliant manner. The review process looks at the business sending messaging (not the software powering it), the use case, sample content, the web presence, the opt-in/consent collected and associated website terms and privacy policies.


#### Possible Causes
The Terms of Service or Privacy Policy on the business’s website indicates that Opt-in is shared with third parties, affiliates, partners, etc. Any business with a terms of service or privacy policy that mentions sharing or selling consumer opt-in data is considered noncompliant. Consent cannot be bought, sold, shared, transferred, or exchanged. 

#### Possible Solutions
Revise your processes to not share or sell consumer data/opt-in information with anyone. Revise your Terms of Service and Privacy Policy to remove all third party opt-in sharing language. If you have a “reasons we can share your personal information matrix” in your privacy policy that is not modifiable, adding a note below it that Messaging opt-in consent will not be shared is acceptable. Once complete and live on the business website, resubmit the toll free verification. 

Resources: 

[CTIA guidelines](https://www.ctia.org/the-wireless-industry/industry-commitments/messaging-interoperability-sms-mms)

[Consent/Opt-In](https://www.twilio.com/en-us/legal/messaging-policy#:~:text=with%20Twilio%20policies.-,Consent%20/%20Opt%2Din,-What%20Is%20Proper) in [Twilio’s Messaging Policy](https://www.twilio.com/en-us/legal/messaging-policy)

[Why was my Toll Free Verification Rejected?](https://support.twilio.com/hc/en-us/articles/9321443984155-Why-Was-My-Toll-Free-Verification-Rejected) 

[Required information for Toll-free verification](https://support.twilio.com/hc/en-us/articles/13264118705051-Required-Information-for-Toll-Free-Verification)

[Toll-Free best practices](https://support.twilio.com/hc/en-us/articles/360038172934-Information-and-best-practices-for-using-Toll-Free-SMS-and-MMS-in-the-US-and-Canada)"#,
            TwilioProgrammableMessagingError::ErrorCode30630 => r#"## ERROR - 30630

### Attempt to send to unsubscribed recipient

 The person you are trying to message has opted out of receiving messages from your Twilio phone number, Channels sender, or Messaging Service.

You have attempted to message a 'To' number that has replied with "STOP" to one of your previous messages. You will not be able to send to the phone number specified in the 'To' parameter until the subscriber identified by the phone number has responded with "START".

Please see this [FAQ](https://help.twilio.com/articles/223134027-Twilio-support-for-opt-out-keywords-SMS-STOP-filtering) for more information about how Twilio handles opt-out and opt-in.

#### Possible Causes
The end user handset has responded with "STOP" or [another opt-out keyword.](https://help.twilio.com/articles/223134027-Twilio-support-for-opt-out-keywords-SMS-STOP-filtering)

#### Possible Solutions
- Consider removing this phone number from your list of recipients.
- Request the recipient to resubscribe to your messages by texting in "START" or [another opt-in keyword](https://help.twilio.com/articles/223134027-Twilio-support-for-opt-out-keywords-SMS-STOP-filtering) to your Twilio sender.
- Before sending messages to a recipient, ensure they have consented to receive messages from you. Please read these guidelines to understand [messaging opt-in requirements and best practices.](https://www.twilio.com/en-us/blog/ctia-messaging-principles-and-best-practices)"#
        }
    }
}
