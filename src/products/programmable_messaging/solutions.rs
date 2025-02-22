// AUTO-GENERATED. DO NOT MODIFY. YOUR CHANGES WILL BE OVER-WRITTEN
use crate::products::TwilioProgrammableMessagingError;
use standard_error::traits::StandardErrorSolutionsTrait;

impl StandardErrorSolutionsTrait for TwilioProgrammableMessagingError {
    fn solutions(&self) -> Option<&'static str> {
        match self {
            TwilioProgrammableMessagingError::ErrorCode20504 => Some(r#"* Try sending the message again.
* If the subsequent attempt also fails with this error, [contact support](https://support.twilio.com/hc/en-us/articles/360048500694-Contacting-Twilio-Support) referencing the affected Message Sids for further troubleshooting."#),
            TwilioProgrammableMessagingError::ErrorCode30884 => Some(r#"The rejected campaign is ineligible for resubmission. If you feel that this was in error, please contact Twilio Customer Support.
 
Additional help resources

* [A2P 10DLC campaign approval best practices](https://support.twilio.com/hc/en-us/articles/11847054539547-A2P-10DLC-Campaign-Approval-Best-Practices)"#),
            TwilioProgrammableMessagingError::ErrorCode35133 => Some(r#"Check if the Messages array is empty. If not, please ensure it doesn't contain too many items"#),
            TwilioProgrammableMessagingError::ErrorCode30758 => Some(r#"Update the Registration Authority field. "#),
            TwilioProgrammableMessagingError::ErrorCode21660 => Some(r#"Check that you are using account credentials that correspond with the account that owns the phone number."#),
            TwilioProgrammableMessagingError::ErrorCode30897 => Some(r#"The rejected campaign is ineligible for resubmission. If you feel that this was in error, please contact Twilio Customer Support.
 
Additional help resources

* [Forbidden message categories](https://support.twilio.com/hc/en-us/articles/360045004974-Forbidden-message-categories-for-SMS-and-MMS-in-the-US-and-Canada ) 

* [A2P 10DLC campaign approval best practices](https://support.twilio.com/hc/en-us/articles/11847054539547-A2P-10DLC-Campaign-Approval-Best-Practices)"#),
            TwilioProgrammableMessagingError::ErrorCode63036 => Some(r#"Turn on the device, connect to internet, and enable RCS and RCS Business Messaging if the device is running iOS ([more details](https://help.twilio.com/articles/29076535334043-RCS-Messaging-Best-Practices-and-FAQ))"#),
            TwilioProgrammableMessagingError::ErrorCode30447 => Some(r#"* If the number is/was with another messaging provider at the time of verification, please ensure the porting or hosting process has been initiated and is complete. Then Delete the rejected toll-free verification and submit a new request after the port completes.
* If it’s not a porting scenario, contact Twilio Support.

[Toll-Free best practices](https://support.twilio.com/hc/en-us/articles/360038172934-Information-and-best-practices-for-using-Toll-Free-SMS-and-MMS-in-the-US-and-Canada)

[Why was my Toll Free Verification Rejected?](https://support.twilio.com/hc/en-us/articles/9321443984155-Why-Was-My-Toll-Free-Verification-Rejected) "#),
            TwilioProgrammableMessagingError::ErrorCode30444 => Some(r#"If you are confident that your business and messaging use case does not fit into forbidden message categories, contact Twilio Support to provide Additional Information that explains your use case.


[Forbidden Message Categories](https://support.twilio.com/hc/en-us/articles/360045004974-Forbidden-message-categories-for-SMS-and-MMS-in-the-US-and-Canada)

[Why was my Toll Free Verification Rejected?](https://support.twilio.com/hc/en-us/articles/9321443984155-Why-Was-My-Toll-Free-Verification-Rejected) 

[Required information for Toll-free verification](https://support.twilio.com/hc/en-us/articles/13264118705051-Required-Information-for-Toll-Free-Verification)

[Toll-Free best practices](https://support.twilio.com/hc/en-us/articles/360038172934-Information-and-best-practices-for-using-Toll-Free-SMS-and-MMS-in-the-US-and-Canada)

[Twilio’s Messaging Policy](https://www.twilio.com/en-us/legal/messaging-policy)"#),
            TwilioProgrammableMessagingError::ErrorCode30474 => Some(r#"Re-submit the toll-free verification with your end-business’s information (address, website, etc)


Resources:

[Toll-free verification for ISVs]( https://support.twilio.com/hc/en-us/articles/13263383206299-Toll-Free-Verification-for-ISVs) 

[Why was my Toll Free Verification Rejected?](https://support.twilio.com/hc/en-us/articles/9321443984155-Why-Was-My-Toll-Free-Verification-Rejected) 

[Required information for Toll-free verification](https://support.twilio.com/hc/en-us/articles/13264118705051-Required-Information-for-Toll-Free-Verification)

[Toll-Free best practices](https://support.twilio.com/hc/en-us/articles/360038172934-Information-and-best-practices-for-using-Toll-Free-SMS-and-MMS-in-the-US-and-Canada)

[Twilio’s Messaging Policy](https://www.twilio.com/en-us/legal/messaging-policy)"#),
            TwilioProgrammableMessagingError::ErrorCode36005 => Some(r#"* Upload a file of type .json.gz (G-Zipped JSON)"#),
            TwilioProgrammableMessagingError::ErrorCode30440 => Some(r#"* Contact [Twilio Customer Support](https://www.twilio.com/console/support/tickets/create)

See [Toll-Free best practices](https://support.twilio.com/hc/en-us/articles/360038172934-Information-and-best-practices-for-using-Toll-Free-SMS-and-MMS-in-the-US-and-Canada) for resources on how to implement Toll-Free messaging.

See [what happens to traffic](https://support.twilio.com/hc/en-us/articles/9321443984155-Why-Was-My-Toll-Free-Verification-Rejected-#:~:text=deceptive%20marketing%20tactics-,What%20happens%20to%20my%20traffic%20if%20my%20Toll%2DFree%20number%20is%20rejected%3F,-If%20your%20Toll) if a Toll-Free verification is rejected"#),
            TwilioProgrammableMessagingError::ErrorCode30901 => Some(r#"Please retry with a new campaign registration."#),
            TwilioProgrammableMessagingError::ErrorCode30749 => Some(r#"Please edit the starter customer profile and use a different email address. Once submitted for review, you can resubmit the brand registration request. 

Additional help resources

* [Troubleshooting US A2P Brand registration failures]"#),
            TwilioProgrammableMessagingError::ErrorCode30888 => Some(r#"Please verify and add a robust age gate to your website or opt-in policy. Resubmit the campaign with the updated age gate information for review.
 
Additional help resources

* [Age Gating](https://www.twilio.com/en-us/legal/messaging-policy#:~:text=Age%20and%20Geographic%20Gating&text=In%20addition%20to%20obtaining%20consent,where%20the%20recipient%20is%20located)

* [A2P 10DLC campaign approval best practices](https://support.twilio.com/hc/en-us/articles/11847054539547-A2P-10DLC-Campaign-Approval-Best-Practices)"#),
            TwilioProgrammableMessagingError::ErrorCode21736 => Some(r#"* Verify that the Brand Contact Email provided is correct. If not, update or add the email via the Brand API or Console.
* Verify if the Brand 2FA email was actioned upon and completed within 7 days. If not, use the following API or Console to re-send the Brand 2FA email."#),
            TwilioProgrammableMessagingError::ErrorCode11321 => Some(r#"* Verify the URL in the Twilio Console to ensure it is correct and accessible
* Make sure you submit a fully qualified URL including: protocol (http:// or https://), hostname, file path, and properly url-encoded query parameters. 
* Twilio must be able to reach this URL over the public Internet.
"#),
            TwilioProgrammableMessagingError::ErrorCode30480 => Some(r#"Contact Twilio Support to provide Additional Information that explains your use case.

Resources: 

[Why was my Toll Free Verification Rejected?](https://support.twilio.com/hc/en-us/articles/9321443984155-Why-Was-My-Toll-Free-Verification-Rejected) 

[Required information for Toll-free verification](https://support.twilio.com/hc/en-us/articles/13264118705051-Required-Information-for-Toll-Free-Verification)

[Toll-Free best practices](https://support.twilio.com/hc/en-us/articles/360038172934-Information-and-best-practices-for-using-Toll-Free-SMS-and-MMS-in-the-US-and-Canada)"#),
            TwilioProgrammableMessagingError::ErrorCode30750 => Some(r#"Please use a different mobile phone number.
Please edit the Sole Proprietor A2P profile bundle and use a different active wireless mobile number for OTP verification. Once submitted for review, you can resubmit the brand registration request. 

Additional help resources

* [Troubleshooting US A2P Brand registration failures]"#),
            TwilioProgrammableMessagingError::ErrorCode30994 => Some(r#"Check your inbox for a 2FA email verification. If issue persists, please contact support."#),
            TwilioProgrammableMessagingError::ErrorCode63041 => Some(r#"* Use a different template
* Edit this template and re-submit for review"#),
            TwilioProgrammableMessagingError::ErrorCode30890 => Some(r#"Please verify that subscriber help message contains brand name, phone number, or email address. Message needs to guide customers on who them can contact after replying ""help"".
 
Additional help resources

* [A2P 10DLC campaign approval best practices](https://support.twilio.com/hc/en-us/articles/11847054539547-A2P-10DLC-Campaign-Approval-Best-Practices)"#),
            TwilioProgrammableMessagingError::ErrorCode11322 => Some(r#"* Twilio expects specific HTTP methods (GET or POST) for making a request to your application.
* Check if the URL is configured to handle the correct HTTP method.
"#),
            TwilioProgrammableMessagingError::ErrorCode63047 => Some(r#"- Make sure the web server hosting the sample file returns either `image/jpeg`, `image/png`, `application/pdf`, or `video/mp4` for the `Content-Type` header."#),
            TwilioProgrammableMessagingError::ErrorCode30473 => Some(r#"* Check the business website URL in the submission is correct and resubmit if it’s incorrect
* Check that the business website URL is a live website. Resubmit once it is live. 
* Make sure that the business website URL is not in a private state that requires a login/password.. Resubmit after it is corrected.

[Why was my Toll Free Verification Rejected?](https://support.twilio.com/hc/en-us/articles/9321443984155-Why-Was-My-Toll-Free-Verification-Rejected) 

[Required information for Toll-free verification](https://support.twilio.com/hc/en-us/articles/13264118705051-Required-Information-for-Toll-Free-Verification)

[Toll-Free best practices](https://support.twilio.com/hc/en-us/articles/360038172934-Information-and-best-practices-for-using-Toll-Free-SMS-and-MMS-in-the-US-and-Canada)

[Twilio’s Messaging Policy](https://www.twilio.com/en-us/legal/messaging-policy)"#),
            TwilioProgrammableMessagingError::ErrorCode30647 => Some(r#"- Ensure the correlation_id is a valid 32-character UUID. Each correlation_id must be unique across all contact objects. You can use a UUID generator if necessary.

- Ensure the contact_id is a valid phone number in [E.164 format.](http://localhost:4200/docs/glossary/what-e164)

- Confirm the country_iso_code: Ensure the country_iso_code follows the ISO 3166-1 alpha-2 standard (e.g., "US").

- Ensure the zip_code matches the postal code format for the specified country.

- If an internal server error occurs, retry the request after some time.


#### Continued issues with error 30647
Twilio's support team can help investigate what went wrong with upserting your contacts. Please collect 3 or more correlation_id from the last 24 hours that were flagged with Error 30647, and [open a support request.](https://www.twilio.com/console/support/tickets/create)"#),
            TwilioProgrammableMessagingError::ErrorCode216602 => Some(r#"Try to change the content type you are using with a one that it's supported on the channel. Please see this page of our docs to determine what content types are supported on what channels: https://www.twilio.com/docs/content/content-types-overview#channel-support"#),
            TwilioProgrammableMessagingError::ErrorCode30446 => Some(r#"* Correct your opt-in process to collect a consumer’s express consent via CTIA guidelines and resubmit the toll free verification
* Note: An opt-in call-to-action (CTA) with a checkbox must be unchecked by default

[CITIA guidelines](
https://www.ctia.org/the-wireless-industry/industry-commitments/messaging-interoperability-sms-mms)

[Forbidden Message Categories](https://support.twilio.com/hc/en-us/articles/360045004974-Forbidden-message-categories-for-SMS-and-MMS-in-the-US-and-Canada)

[Why was my Toll Free Verification Rejected?](https://support.twilio.com/hc/en-us/articles/9321443984155-Why-Was-My-Toll-Free-Verification-Rejected) 

[Required information for Toll-free verification](https://support.twilio.com/hc/en-us/articles/13264118705051-Required-Information-for-Toll-Free-Verification)

[Toll-Free best practices](https://support.twilio.com/hc/en-us/articles/360038172934-Information-and-best-practices-for-using-Toll-Free-SMS-and-MMS-in-the-US-and-Canada)

[Twilio’s Messaging Policy](https://www.twilio.com/en-us/legal/messaging-policy)"#),
            TwilioProgrammableMessagingError::ErrorCode21737 => Some(r#"* Update the brand registration's BrandContactEmail with a valid email address and complete the Brand 2FA process."#),
            TwilioProgrammableMessagingError::ErrorCode30471 => Some(r#"* Correct the URL with https:// in your sample message and resubmit the verification
* Ensure that any URL’s sent in messages will be secured with https://

[Why was my Toll Free Verification Rejected?](https://support.twilio.com/hc/en-us/articles/9321443984155-Why-Was-My-Toll-Free-Verification-Rejected) 

[Required information for Toll-free verification](https://support.twilio.com/hc/en-us/articles/13264118705051-Required-Information-for-Toll-Free-Verification)

[Toll-Free best practices](https://support.twilio.com/hc/en-us/articles/360038172934-Information-and-best-practices-for-using-Toll-Free-SMS-and-MMS-in-the-US-and-Canada)

[Twilio’s Messaging Policy](https://www.twilio.com/en-us/legal/messaging-policy)"#),
            TwilioProgrammableMessagingError::ErrorCode21731 => Some(r#"The Twilio team should be reaching out to you to provide guidance on how to fix the suspended Brand. Please check your email or the Twilio Support Center. If you don’t see anything, please raise a ticket with the Twilio Support team."#),
            TwilioProgrammableMessagingError::ErrorCode30460 => Some(r#"If you are confident that your business and messaging use case does not fit into forbidden message categories, contact Twilio Support to provide Additional Information that explains your use case.

[Forbidden Message Categories](https://support.twilio.com/hc/en-us/articles/360045004974-Forbidden-message-categories-for-SMS-and-MMS-in-the-US-and-Canada)

[Why was my Toll Free Verification Rejected?](https://support.twilio.com/hc/en-us/articles/9321443984155-Why-Was-My-Toll-Free-Verification-Rejected) 

[Required information for Toll-free verification](https://support.twilio.com/hc/en-us/articles/13264118705051-Required-Information-for-Toll-Free-Verification)

[Toll-Free best practices](https://support.twilio.com/hc/en-us/articles/360038172934-Information-and-best-practices-for-using-Toll-Free-SMS-and-MMS-in-the-US-and-Canada)

[Twilio’s Messaging Policy](https://www.twilio.com/en-us/legal/messaging-policy)"#),
            TwilioProgrammableMessagingError::ErrorCode36001 => Some(r#"* Provide a non-empty, existing Broadcast SID"#),
            TwilioProgrammableMessagingError::ErrorCode30754 => Some(r#"Please ensure 'street', 'region', 'state', 'postal_code', and 'iso_country' collectively forms a valid postal address.

Additional help resources

* [Troubleshooting US A2P Brand registration failures]"#),
            TwilioProgrammableMessagingError::ErrorCode20410 => Some(r#"* Ensure you are using the most up to date Messaging Resources. See the [Programmable Messaging API docs](https://www.twilio.com/docs/messaging/api) for up to date endpoints.
* Migrate from the “SMS/Messages” endpoint to “Messages” endpoint. See [here](https://support.twilio.com/hc/en-us/articles/223181028-Switching-from-SMS-Messages-resource-URI-to-Messages-resource-URI) for more information."#),
            TwilioProgrammableMessagingError::ErrorCode30646 => Some(r#"- Ensure the correlation_id is a valid 32-character UUID. Each correlation_id must be unique across all contact objects. You can use a UUID generator if necessary.

- Ensure the sender_id is either a valid Messaging Service SID or a from phone number.

- Ensure the contact_id is a valid phone number in [E.164 format.](http://localhost:4200/docs/glossary/what-e164)

- Ensure the status field contains a valid value (opt-in or opt-out).

- Ensure the source field is one of the accepted values (website, offline, opt-in-message, opt-out-message, others).

- If an internal server error occurs, retry the request after some time.

#### Continued issues with error 30646
Twilio's support team can help investigate what went wrong with upserting your contacts. Please collect 3 or more correlation_id from the last 24 hours that were flagged with Error 30646, and [open a support request.](https://www.twilio.com/console/support/tickets/create)"#),
            TwilioProgrammableMessagingError::ErrorCode36004 => Some(r#"* Attach a .json.gz file as the value to the "file" key in the Broadcast Upload request body"#),
            TwilioProgrammableMessagingError::ErrorCode30610 => Some(r#"**Verify Local Time:** Implement a system to accurately determine the local time zone of each recipient based on their area code and address data.

**Schedule Adjustments:** Configure your messaging campaign schedules to operate only within the permitted hours of 8 a.m. to 9 p.m. local time for each recipient.

**Update recipients location:** Configure your recipients location using this [API](https://www.twilio.com/docs/messaging/features/bulk-upsert-contacts-api), incase recipients location is different & doesn’t currently fall under 9 p.m. to 8 a.m.


#### Continued issues with error 30610

Twilio's support team can help investigate what went wrong with delivering your message. Please collect 3 or more message SIDs in your SMS [logs](https://www.twilio.com/console/sms/logs) from the last 24 hours that were flagged with Error 30610, and [open a support request.](https://www.twilio.com/console/support/tickets/create)"#),
            TwilioProgrammableMessagingError::ErrorCode21659 => Some(r#"* If you have ported/hosted the `From` number, ensure that the process is complete. You can follow up on this in the Twilio console for [porting](https://console.twilio.com/us1/develop/phone-numbers/port-host/porting-requests?frameUrl=%2Fconsole%2Fphone-numbers%2Fporting-requests%3Fx-target-region%3Dus1) and [hosting](https://console.twilio.com/us1/develop/phone-numbers/port-host/hosted-numbers?frameUrl=%2Fconsole%2Fphone-numbers%2Fhosted%3Fx-target-region%3Dus1).
* If you are sending from a Short Code, verify that the country you are sending to matches the country of the Short Code.
* Ensure the number you are using is in [E.164 format](https://www.twilio.com/docs/glossary/what-e164).

"#),
            TwilioProgrammableMessagingError::ErrorCode30734 => Some(r#"Please contact Twilio Support.

Additional help resources

* [Troubleshooting US A2P Brand registration failures]"#),
            TwilioProgrammableMessagingError::ErrorCode30475 => Some(r#"* Ensure your opt-in mechanism is a distinct action during a consumer’s sign up for your service. 
* An opt-in call-to-action (CTA) with a checkbox must be unchecked by default.

Resources: 

[CTIA guidelines](https://www.ctia.org/the-wireless-industry/industry-commitments/messaging-interoperability-sms-mms)

[Consent/Opt-In](https://www.twilio.com/en-us/legal/messaging-policy#:~:text=with%20Twilio%20policies.-,Consent%20/%20Opt%2Din,-What%20Is%20Proper) in [Twilio’s Messaging Policy](https://www.twilio.com/en-us/legal/messaging-policy)

[Why was my Toll Free Verification Rejected?](https://support.twilio.com/hc/en-us/articles/9321443984155-Why-Was-My-Toll-Free-Verification-Rejected) 

[Required information for Toll-free verification](https://support.twilio.com/hc/en-us/articles/13264118705051-Required-Information-for-Toll-Free-Verification)

[Toll-Free best practices](https://support.twilio.com/hc/en-us/articles/360038172934-Information-and-best-practices-for-using-Toll-Free-SMS-and-MMS-in-the-US-and-Canada)
"#),
            TwilioProgrammableMessagingError::ErrorCode30993 => Some(r#"Please trigger the migration process again and register the campaign with Twilio's ERC api. If the issue persists, please contact support."#),
            TwilioProgrammableMessagingError::ErrorCode30903 => Some(r#"* Verify the accuracy and completeness of the registration information for the Sole Proprietorship Brand.
* Ensure that the brand meets the criteria defined by TCR and mobile carriers for Sole Proprietor (EIN) registration.
* If the brand does not qualify as a Sole Proprietorship, consider registering it as a standard brand according to the appropriate guidelines and requirements.
* Register a standard or acceptable campaign use case that aligns with the registered brand's classification."#),
            TwilioProgrammableMessagingError::ErrorCode30701 => Some(r#"Please check your inputs for the fields in the error message and confirm that they meet the validations listed in possible causes.

Additional help resources

* [Troubleshooting US A2P Brand registration failures](https://www.twilio.com/docs/messaging/compliance/a2p-10dlc/troubleshooting-a2p-brands)"#),
            TwilioProgrammableMessagingError::ErrorCode21267 => Some(r#"If you are using an Alphanumeric Sender ID on a Trial Account, [upgrade your account](https://www.twilio.com/docs/messaging/guides/how-to-use-your-free-trial-account#how-to-upgrade-your-account).
"#),
            TwilioProgrammableMessagingError::ErrorCode21732 => Some(r#"Please delete the existing Campaign first and then retry."#),
            TwilioProgrammableMessagingError::ErrorCode30445 => Some(r#"* Ensure all business information fields are valid and accurately represent the business who manages the opt-in and has the relationship with the consumer:

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

[Twilio’s Messaging Policy](https://www.twilio.com/en-us/legal/messaging-policy)"#),
            TwilioProgrammableMessagingError::ErrorCode21266 => Some(r#"Ensure you are not attempting to send a message from a Twilio number to itself.
"#),
            TwilioProgrammableMessagingError::ErrorCode30757 => Some(r#"Update the Business Registration Number field. "#),
            TwilioProgrammableMessagingError::ErrorCode30456 => Some(r#"Add an Age Gate for your US business website. Resubmit the verification and provide proof in your opt-in workflow documentation that you have measures in place to ensure compliance.

[Forbidden Message Categories](https://support.twilio.com/hc/en-us/articles/360045004974-Forbidden-message-categories-for-SMS-and-MMS-in-the-US-and-Canada)

[Why was my Toll Free Verification Rejected?](https://support.twilio.com/hc/en-us/articles/9321443984155-Why-Was-My-Toll-Free-Verification-Rejected) 

[Required information for Toll-free verification](https://support.twilio.com/hc/en-us/articles/13264118705051-Required-Information-for-Toll-Free-Verification)

[Toll-Free best practices](https://support.twilio.com/hc/en-us/articles/360038172934-Information-and-best-practices-for-using-Toll-Free-SMS-and-MMS-in-the-US-and-Canada)

[Twilio’s Messaging Policy](https://www.twilio.com/en-us/legal/messaging-policy)"#),
            TwilioProgrammableMessagingError::ErrorCode63051 => Some(r#"If you believe this is in error, please open a support ticket for us to appeal to WhatsApp on your behalf."#),
            TwilioProgrammableMessagingError::ErrorCode11100 => Some(r#"* Make sure you submit a fully qualified URL including: protocol (http:// or https://), hostname, file path, properly url-encoded query parameters.
* Twilio must be able to reach this URL over the public Internet."#),
            TwilioProgrammableMessagingError::ErrorCode63046 => Some(r#"No further action needed."#),
            TwilioProgrammableMessagingError::ErrorCode30908 => Some(r#"1. Verify privacy policy is accessible to end-users and meets the compliance standards for the collection and use of mobile phone numbers for messaging. 
2. Include a direct link to the privacy policy within the Message Flow.
Once you have made a change to address the privacy policy error, please resubmit the campaign for review.
3. Ensure that Privacy Policy states that no mobile information will be shared with third parties/affiliates for marketing/promotional purposes. 

Additional help resources
* [A2P 10DLC campaign approval best practices](https://support.twilio.com/hc/en-us/articles/11847054539547-A2P-10DLC-Campaign-Approval-Best-Practices)

"#),
            TwilioProgrammableMessagingError::ErrorCode21703 => Some(r#"Add a sender to your Messaging Service with the required capabilities, using the [Twilio Console](https://www.twilio.com/console/sms/services) or the [REST API](https://www.twilio.com/docs/messaging/services/api/phonenumber-resource#create-a-phonenumber-resource-add-a-phone-number-to-a-messaging-service)."#),
            TwilioProgrammableMessagingError::ErrorCode30463 => Some(r#"If you are confident that your business and messaging use case does not fit into forbidden message categories, contact Twilio Support to provide Additional Information that explains your use case.

[Forbidden Message Categories](https://support.twilio.com/hc/en-us/articles/360045004974-Forbidden-message-categories-for-SMS-and-MMS-in-the-US-and-Canada)

[Why was my Toll Free Verification Rejected?](https://support.twilio.com/hc/en-us/articles/9321443984155-Why-Was-My-Toll-Free-Verification-Rejected) 

[Required information for Toll-free verification](https://support.twilio.com/hc/en-us/articles/13264118705051-Required-Information-for-Toll-Free-Verification)

[Toll-Free best practices](https://support.twilio.com/hc/en-us/articles/360038172934-Information-and-best-practices-for-using-Toll-Free-SMS-and-MMS-in-the-US-and-Canada)

[Twilio’s Messaging Policy](https://www.twilio.com/en-us/legal/messaging-policy)"#),
            TwilioProgrammableMessagingError::ErrorCode30895 => Some(r#"1. Verify Direct Lending Indicated in Campaign Description: If the campaign involves direct lending or loan arrangement or is associated with a financial services organization, ensure "DIRECT LENDING" is mentioned in campaign description. Please mention this even if the specified campaign use case does not directly relate to loan offering (e.g. 2FA).
2. Update Campaign Description if Needed: If the campaign is not related to direct lending or loan arrangement, update the campaign description to reflect the actual content and purpose. If the campaign is related to a financial services organization, update the campaign descripton to contain "DIRECT LENDING".

Once you have made a change to address the issue, please resubmit the campaign for review.

 
Additional help resources

* [Forbidden message categories](https://support.twilio.com/hc/en-us/articles/360045004974-Forbidden-message-categories-for-SMS-and-MMS-in-the-US-and-Canada)
* [A2P 10DLC campaign approval best practices](https://support.twilio.com/hc/en-us/articles/11847054539547-A2P-10DLC-Campaign-Approval-Best-Practices)"#),
            TwilioProgrammableMessagingError::ErrorCode21635 => Some(r#"* Confirm that the number you are sending to is not a landline, using the [Lookup API](https://www.twilio.com/docs/lookup/v2-api).
* Include `ForceDelivery` [API parameter](https://www.twilio.com/docs/messaging/api/message-resource#request-body-parameters) if you still want to send the message to this phone number. Please note that this does not guarantee the recipient number will be capable of receiving the message. Some international carriers may have a text-to-speech feature that converts the [SMS to a voice call](https://help.twilio.com/articles/223133647). 

"#),
            TwilioProgrammableMessagingError::ErrorCode30729 => Some(r#"Please use a supported country for brand registrations.

Additional help resources

* [Troubleshooting US A2P Brand registration failures](https://www.twilio.com/docs/messaging/compliance/a2p-10dlc/troubleshooting-a2p-brands)"#),
            TwilioProgrammableMessagingError::ErrorCode30896 => Some(r#"1. Ensure compliance with Twilio Messaging Policy relating to opt-in
2. Detail All Opt-in Methods: Include all methods of opt-in, whether electronic, paper form, in-person verbal opt-in, or other means.
3. Provide Necessary Links and Documentation: If opt-in is collected through a paper form or behind a login, supply a hosted link to an image of the opt-in. If the opt-in occurs on a website, provide the relevant link.
4. Include Privacy Policy and Terms of Service: The website where opt-in occurs must contain a privacy policy and terms of service.
5. Avoid Third-Party Sharing: Make sure that opt-in information is not shared with unauthorized third parties.
6. Ensure Opt-in is Verifable: Each campaign is manually reviewed and needs to be verifable by a human.

Once you have made a change to address the issue, please resubmit the campaign for review.

 
Additional help resources

* [CTIA Guidelines]( https://www.ctia.org/the-wireless-industry/industry-commitments/messaging-interoperability-sms-mms) 

* [A2P 10DLC campaign approval best practices](https://support.twilio.com/hc/en-us/articles/11847054539547-A2P-10DLC-Campaign-Approval-Best-Practices)"#),
            TwilioProgrammableMessagingError::ErrorCode30906 => Some(r#"* Revise the Campaign details to address the compliance issues and re-share the Campaign with Twilio's DCA for another review.
* For further clarification and resolution, contact Twilio Customer Support.
 
Additional help resources

* [A2P 10DLC campaign approval best practices](https://support.twilio.com/hc/en-us/articles/11847054539547-A2P-10DLC-Campaign-Approval-Best-Practices)
* [Changes to Externally Registered Campaign Submission](https://support.twilio.com/hc/en-us/articles/13097615242395-Changes-to-Externally-Registered-Campaign-Submission)"#),
            TwilioProgrammableMessagingError::ErrorCode30907 => Some(r#"1. Confirm website content aligns with the registered Brand and Campaign details.
2. Update Website URL submission on Brand details.
Once you have made a change to address the website validation error, please resubmit the campaign for review.
 
Additional help resources

* [A2P 10DLC campaign approval best practices](https://support.twilio.com/hc/en-us/articles/11847054539547-A2P-10DLC-Campaign-Approval-Best-Practices)
"#),
            TwilioProgrammableMessagingError::ErrorCode30902 => Some(r#"Please reach out to customer support with campaign and account details for resolution. "#),
            TwilioProgrammableMessagingError::ErrorCode30441 => Some(r#"If you are confident that your business and messaging use case does not fit into forbidden message categories, contact Twilio Support to provide Additional Information that explains your use case.


[Forbidden Message Categories](https://support.twilio.com/hc/en-us/articles/360045004974-Forbidden-message-categories-for-SMS-and-MMS-in-the-US-and-Canada)

[Why was my Toll Free Verification Rejected?](https://support.twilio.com/hc/en-us/articles/9321443984155-Why-Was-My-Toll-Free-Verification-Rejected) 

[Required information for Toll-free verification](https://support.twilio.com/hc/en-us/articles/13264118705051-Required-Information-for-Toll-Free-Verification)

[Toll-Free best practices](https://support.twilio.com/hc/en-us/articles/360038172934-Information-and-best-practices-for-using-Toll-Free-SMS-and-MMS-in-the-US-and-Canada)

[Twilio’s Messaging Policy](https://www.twilio.com/en-us/legal/messaging-policy)"#),
            TwilioProgrammableMessagingError::ErrorCode30465 => Some(r#"If you are confident that your business and messaging use case does not fit into forbidden message categories, contact Twilio Support to provide Additional Information that explains your use case.


[Forbidden Message Categories](https://support.twilio.com/hc/en-us/articles/360045004974-Forbidden-message-categories-for-SMS-and-MMS-in-the-US-and-Canada)

[Why was my Toll Free Verification Rejected?](https://support.twilio.com/hc/en-us/articles/9321443984155-Why-Was-My-Toll-Free-Verification-Rejected) 

[Required information for Toll-free verification](https://support.twilio.com/hc/en-us/articles/13264118705051-Required-Information-for-Toll-Free-Verification)

[Toll-Free best practices](https://support.twilio.com/hc/en-us/articles/360038172934-Information-and-best-practices-for-using-Toll-Free-SMS-and-MMS-in-the-US-and-Canada)

[Twilio’s Messaging Policy](https://www.twilio.com/en-us/legal/messaging-policy)
"#),
            TwilioProgrammableMessagingError::ErrorCode35127 => Some(r#"Specify only one of the both parameters in your request"#),
            TwilioProgrammableMessagingError::ErrorCode63045 => Some(r#"- Confirm that the link is correct
- See if the link can be accessed in your browser"#),
            TwilioProgrammableMessagingError::ErrorCode30797 => Some(r#"Please change the entity_type field value to something other than Government. If you are a US Entity, please update the customer profile address to indicate the same.

Additional help resources

* [Troubleshooting US A2P Brand registration failures](https://www.twilio.com/docs/messaging/compliance/a2p-10dlc/troubleshooting-a2p-brands)
"#),
            TwilioProgrammableMessagingError::ErrorCode30885 => Some(r#"The rejected campaign is ineligible for resubmission. If you feel that this was in error, please contact Twilio Customer Support.
 
Additional help resources

* [Forbidden message categories](https://support.twilio.com/hc/en-us/articles/360045004974-Forbidden-message-categories-for-SMS-and-MMS-in-the-US-and-Canada ) 

* [A2P 10DLC campaign approval best practices](https://support.twilio.com/hc/en-us/articles/11847054539547-A2P-10DLC-Campaign-Approval-Best-Practices)"#),
            TwilioProgrammableMessagingError::ErrorCode30894 => Some(r#"Please verify that the brand information is valid. Resubmit campaign for review.
 
Additional help resources

* [A2P 10DLC campaign approval best practices](https://support.twilio.com/hc/en-us/articles/11847054539547-A2P-10DLC-Campaign-Approval-Best-Practices)"#),
            TwilioProgrammableMessagingError::ErrorCode36009 => Some(r#"* Provide a non-empty, existing Broadcast SID
* If the Broadcast SID is valid, please retry after some time"#),
            TwilioProgrammableMessagingError::ErrorCode30620 => Some(r#"**Use alternate sender ID:** Use a different Sender ID, whose registered use case matches to the intent of the message being sent.

**Register the sender ID:** Consider re-registering the Sender ID with a use case which is appropriate for your business use case

#### Continued issues with error 30620

Twilio's support team can help investigate what went wrong with delivering your message. Please collect 3 or more message SIDs in your SMS [logs](https://www.twilio.com/console/sms/logs) from the last 24 hours that were flagged with Error 30620, and [open a support request.](https://www.twilio.com/console/support/tickets/create)"#),
            TwilioProgrammableMessagingError::ErrorCode30702 => Some(r#"Please edit and resubmit the brand or re-register the brand. "#),
            TwilioProgrammableMessagingError::ErrorCode30991 => Some(r#"Review the Brand associated with this Campaign you are trying to register."#),
            TwilioProgrammableMessagingError::ErrorCode36008 => Some(r#"* Make sure the page token has exactly 34 alphanumeric characters
* Make sure the page token is prefixed with "BC""#),
            TwilioProgrammableMessagingError::ErrorCode30883 => Some(r#"The rejected campaign is ineligible for resubmission. If you feel that this was in error, please contact Twilio Customer Support.
 
Additional help resources

* [Forbidden message categories](https://support.twilio.com/hc/en-us/articles/360045004974-Forbidden-message-categories-for-SMS-and-MMS-in-the-US-and-Canada ) 

* [A2P 10DLC campaign approval best practices](https://support.twilio.com/hc/en-us/articles/11847054539547-A2P-10DLC-Campaign-Approval-Best-Practices)"#),
            TwilioProgrammableMessagingError::ErrorCode30442 => Some(r#"If you are confident that your business and messaging use case does not fit into forbidden message categories, contact Twilio Support to provide Additional Information that explains your use case.


[Forbidden Message Categories](https://support.twilio.com/hc/en-us/articles/360045004974-Forbidden-message-categories-for-SMS-and-MMS-in-the-US-and-Canada)

[Why was my Toll Free Verification Rejected?](https://support.twilio.com/hc/en-us/articles/9321443984155-Why-Was-My-Toll-Free-Verification-Rejected) 

[Required information for Toll-free verification](https://support.twilio.com/hc/en-us/articles/13264118705051-Required-Information-for-Toll-Free-Verification)

[Toll-Free best practices](https://support.twilio.com/hc/en-us/articles/360038172934-Information-and-best-practices-for-using-Toll-Free-SMS-and-MMS-in-the-US-and-Canada)

[Twilio’s Messaging Policy](https://www.twilio.com/en-us/legal/messaging-policy)"#),
            TwilioProgrammableMessagingError::ErrorCode30892 => Some(r#"Verify URL in sample messages does not contain public URL shorteners. Resubmit campaign."#),
            TwilioProgrammableMessagingError::ErrorCode30751 => Some(r#"Please use a valid US Wireless mobile phone number which is capable of receiving verification SMS.

Additional help resources

* [Troubleshooting US A2P Brand registration failures]"#),
            TwilioProgrammableMessagingError::ErrorCode30891 => Some(r#"* Verify that the provided website(s) are functioning correctly, and accessible. If the website(s) are geographic specific, please indicate this in the campaign description and provide a link to a screenshot.
* If the registration is for a pre-launch website, instead include a publically accessible URL to view a screenshot of the SMS opt-in flow that will appear.
* If opt-in occurs on website, provide the direct link in the Message Flow field.
* If opt-in is collected through a paper form or behind a login, supply a hosted link to an image of the opt-in. If the opt-in occurs on a website, provide the relevant link.

Once you have made a change to address the issue, please resubmit the campaign for review.

Additional help resources

* [A2P 10DLC campaign approval best practices](https://support.twilio.com/hc/en-us/articles/11847054539547-A2P-10DLC-Campaign-Approval-Best-Practices)"#),
            TwilioProgrammableMessagingError::ErrorCode30455 => Some(r#"If you are confident that your business and messaging use case does not fit into forbidden message categories, contact Twilio Support to provide Additional Information that explains your use case.


[Forbidden Message Categories](https://support.twilio.com/hc/en-us/articles/360045004974-Forbidden-message-categories-for-SMS-and-MMS-in-the-US-and-Canada)

[Why was my Toll Free Verification Rejected?](https://support.twilio.com/hc/en-us/articles/9321443984155-Why-Was-My-Toll-Free-Verification-Rejected) 

[Required information for Toll-free verification](https://support.twilio.com/hc/en-us/articles/13264118705051-Required-Information-for-Toll-Free-Verification)

[Toll-Free best practices](https://support.twilio.com/hc/en-us/articles/360038172934-Information-and-best-practices-for-using-Toll-Free-SMS-and-MMS-in-the-US-and-Canada)

[Twilio’s Messaging Policy](https://www.twilio.com/en-us/legal/messaging-policy)"#),
            TwilioProgrammableMessagingError::ErrorCode63043 => Some(r#"- Use a publicly accessible URL
- Use a URL that includes an access token"#),
            TwilioProgrammableMessagingError::ErrorCode36003 => Some(r#"* Upload a file that is less than 25 MB"#),
            TwilioProgrammableMessagingError::ErrorCode35134 => Some(r#"Check the `Messages` array and remove `To` number duplicates if any"#),
            TwilioProgrammableMessagingError::ErrorCode36007 => Some(r#"* Upload the file to a Broadcast SID that is in a PENDING_UPLOAD state"#),
            TwilioProgrammableMessagingError::ErrorCode30796 => Some(r#"Please edit your A2P profile and update the stock_symbol and/or stock_exchange with correct information.

Additional help resources

* [Troubleshooting US A2P Brand registration failures](https://www.twilio.com/docs/messaging/compliance/a2p-10dlc/troubleshooting-a2p-brands)"#),
            TwilioProgrammableMessagingError::ErrorCode30470 => Some(r#"If you are confident that your business and messaging use case does not fit into forbidden message categories, contact Twilio Support to provide Additional Information that explains your use case.

[Forbidden Message Categories](https://support.twilio.com/hc/en-us/articles/360045004974-Forbidden-message-categories-for-SMS-and-MMS-in-the-US-and-Canada)

[Why was my Toll Free Verification Rejected?](https://support.twilio.com/hc/en-us/articles/9321443984155-Why-Was-My-Toll-Free-Verification-Rejected) 

[Required information for Toll-free verification](https://support.twilio.com/hc/en-us/articles/13264118705051-Required-Information-for-Toll-Free-Verification)

[Toll-Free best practices](https://support.twilio.com/hc/en-us/articles/360038172934-Information-and-best-practices-for-using-Toll-Free-SMS-and-MMS-in-the-US-and-Canada)

[Twilio’s Messaging Policy](https://www.twilio.com/en-us/legal/messaging-policy)
"#),
            TwilioProgrammableMessagingError::ErrorCode30476 => Some(r#"Resubmit the toll-free verification with a link that tells the story of your opt-in workflow. 

Resources: 

[CTIA guidelines](https://www.ctia.org/the-wireless-industry/industry-commitments/messaging-interoperability-sms-mms)

[Consent/Opt-In](https://www.twilio.com/en-us/legal/messaging-policy#:~:text=with%20Twilio%20policies.-,Consent%20/%20Opt%2Din,-What%20Is%20Proper) in [Twilio’s Messaging Policy](https://www.twilio.com/en-us/legal/messaging-policy)

[Why was my Toll Free Verification Rejected?](https://support.twilio.com/hc/en-us/articles/9321443984155-Why-Was-My-Toll-Free-Verification-Rejected) 

[Required information for Toll-free verification](https://support.twilio.com/hc/en-us/articles/13264118705051-Required-Information-for-Toll-Free-Verification)

[Toll-Free best practices](https://support.twilio.com/hc/en-us/articles/360038172934-Information-and-best-practices-for-using-Toll-Free-SMS-and-MMS-in-the-US-and-Canada)"#),
            TwilioProgrammableMessagingError::ErrorCode30990 => Some(r#"1. Review the specific reasons for the campaign suspension and resolve any outstanding compliance or policy issues. 
2. Contact Twilio Support team to receive more details around why your campaign has been suspended, and how to resolve the issue."#),
            TwilioProgrammableMessagingError::ErrorCode30436 => Some(r#"* Please specify a phone number SID for a toll free number you own."#),
            TwilioProgrammableMessagingError::ErrorCode30452 => Some(r#"Correct the email address and resubmit the verification in Console via the Messaging Compliance API. It is not recommended to delete the verification to resolve email validation issue. 


Forbidden message categories

https://support.twilio.com/hc/en-us/articles/360045004974-Forbidden-message-categories-for-SMS-and-MMS-in-the-US-and-Canada 

Toll-Free best practices can be found at 

https://support.twilio.com/hc/en-us/articles/360038172934-Information-and-best-practices-for-using-Toll-Free-SMS-and-MMS-in-the-US-and-Canada
"#),
            TwilioProgrammableMessagingError::ErrorCode30793 => Some(r#"Please check that bundle SIDs provide as A2PProfileBundleSid/CustomerProfileBundleSid exist and contain valid information.


Please check that you have provided one the following support Stock Exchanges (applicable only for public entities registering Low Volume Standard or Standard Brands)
NONE, NASDAQ, NYSE, AMEX, AMX, ASX, B3, BME, BSE, FRA, ICEX, JPX, JSE, KRX, LON, NSE, OMX, SEHK, SGX, SSE, STO, SWX, SZSE, TSX, TWSE, VSE, OTHER"#),
            TwilioProgrammableMessagingError::ErrorCode30468 => Some(r#"If you are confident that your business and messaging use case does not fit into forbidden message categories, contact Twilio Support to provide Additional Information that explains your use case.


[Forbidden Message Categories](https://support.twilio.com/hc/en-us/articles/360045004974-Forbidden-message-categories-for-SMS-and-MMS-in-the-US-and-Canada)

[Why was my Toll Free Verification Rejected?](https://support.twilio.com/hc/en-us/articles/9321443984155-Why-Was-My-Toll-Free-Verification-Rejected) 

[Required information for Toll-free verification](https://support.twilio.com/hc/en-us/articles/13264118705051-Required-Information-for-Toll-Free-Verification)

[Toll-Free best practices](https://support.twilio.com/hc/en-us/articles/360038172934-Information-and-best-practices-for-using-Toll-Free-SMS-and-MMS-in-the-US-and-Canada)

[Twilio’s Messaging Policy](https://www.twilio.com/en-us/legal/messaging-policy)
"#),
            TwilioProgrammableMessagingError::ErrorCode63052 => Some(r#"Templates must be sent using ContentSid and Content Variables"#),
            TwilioProgrammableMessagingError::ErrorCode30748 => Some(r#"Please edit the starter customer profile and use a different phone number. Once submitted for review, you can resubmit the brand registration request. 

Additional help resources

* [Troubleshooting US A2P Brand registration failures]"#),
            TwilioProgrammableMessagingError::ErrorCode30992 => Some(r#"Please retry the registration after starting the CNP migration process."#),
            TwilioProgrammableMessagingError::ErrorCode30451 => Some(r#"Correct the address and resubmit the verification in Console or via the Messaging Compliance API. It is not recommended to delete the verification to resolve address validation issue. 

Forbidden message categories

https://support.twilio.com/hc/en-us/articles/360045004974-Forbidden-message-categories-for-SMS-and-MMS-in-the-US-and-Canada 

Toll-Free best practices can be found at 

https://support.twilio.com/hc/en-us/articles/360038172934-Information-and-best-practices-for-using-Toll-Free-SMS-and-MMS-in-the-US-and-Canada
"#),
            TwilioProgrammableMessagingError::ErrorCode30457 => Some(r#"If you are confident that your business and messaging use case does not fit into forbidden message categories, contact Twilio Support to provide Additional Information that explains your use case.

[Forbidden Message Categories](https://support.twilio.com/hc/en-us/articles/360045004974-Forbidden-message-categories-for-SMS-and-MMS-in-the-US-and-Canada)

[Why was my Toll Free Verification Rejected?](https://support.twilio.com/hc/en-us/articles/9321443984155-Why-Was-My-Toll-Free-Verification-Rejected) 

[Required information for Toll-free verification](https://support.twilio.com/hc/en-us/articles/13264118705051-Required-Information-for-Toll-Free-Verification)

[Toll-Free best practices](https://support.twilio.com/hc/en-us/articles/360038172934-Information-and-best-practices-for-using-Toll-Free-SMS-and-MMS-in-the-US-and-Canada)

[Twilio’s Messaging Policy](https://www.twilio.com/en-us/legal/messaging-policy)"#),
            TwilioProgrammableMessagingError::ErrorCode30467 => Some(r#"If you are confident that your business and messaging use case does not fit into forbidden message categories, contact Twilio Support to provide additional information that explains your use case.

[Forbidden Message Categories](https://support.twilio.com/hc/en-us/articles/360045004974-Forbidden-message-categories-for-SMS-and-MMS-in-the-US-and-Canada)

[Why was my Toll Free Verification Rejected?](https://support.twilio.com/hc/en-us/articles/9321443984155-Why-Was-My-Toll-Free-Verification-Rejected) 

[Required information for Toll-free verification](https://support.twilio.com/hc/en-us/articles/13264118705051-Required-Information-for-Toll-Free-Verification)

[Toll-Free best practices](https://support.twilio.com/hc/en-us/articles/360038172934-Information-and-best-practices-for-using-Toll-Free-SMS-and-MMS-in-the-US-and-Canada)

[Twilio’s Messaging Policy](https://www.twilio.com/en-us/legal/messaging-policy)
"#),
            TwilioProgrammableMessagingError::ErrorCode30798 => Some(r#"Please change the entity_type field value to something other than Non-Profit. If you are a Non-Profit Entity, please update the customer profile to indicate the same.

Additional help resources

* [Troubleshooting US A2P Brand registration failures](https://www.twilio.com/docs/messaging/compliance/a2p-10dlc/troubleshooting-a2p-brands)
"#),
            TwilioProgrammableMessagingError::ErrorCode30461 => Some(r#"If you are confident that your business and messaging use case does not fit into forbidden message categories, contact Twilio Support to provide Additional Information that explains your use case.

[Forbidden Message Categories](https://support.twilio.com/hc/en-us/articles/360045004974-Forbidden-message-categories-for-SMS-and-MMS-in-the-US-and-Canada)

[Why was my Toll Free Verification Rejected?](https://support.twilio.com/hc/en-us/articles/9321443984155-Why-Was-My-Toll-Free-Verification-Rejected) 

[Required information for Toll-free verification](https://support.twilio.com/hc/en-us/articles/13264118705051-Required-Information-for-Toll-Free-Verification)

[Toll-Free best practices](https://support.twilio.com/hc/en-us/articles/360038172934-Information-and-best-practices-for-using-Toll-Free-SMS-and-MMS-in-the-US-and-Canada)

[Twilio’s Messaging Policy](https://www.twilio.com/en-us/legal/messaging-policy)"#),
            TwilioProgrammableMessagingError::ErrorCode30893 => Some(r#"* Verify that the sample messages are accurate, detailed, and reflective of the actual messages to be sent under the campaign.
* Indicate templated fields within sample messages using brackets.
* At least one of the sample messages should include your business name and opt-out message.
* Ensure at least two sample messages are provided.
* Ensure that the use-case and campaign description align with each other.

Once you have made a change to address the issue, please resubmit the campaign for review.

Additional help resources

* [A2P 10DLC campaign approval best practices]( https://support.twilio.com/hc/en-us/articles/11847054539547-A2P-10DLC-Campaign-Approval-Best-Practices)"#),
            TwilioProgrammableMessagingError::ErrorCode30469 => Some(r#"If you are confident that your business and messaging use case does not fit into forbidden message categories, contact Twilio Support to provide Additional Information that explains your use case.

[Forbidden Message Categories](https://support.twilio.com/hc/en-us/articles/360045004974-Forbidden-message-categories-for-SMS-and-MMS-in-the-US-and-Canada)

[Why was my Toll Free Verification Rejected?](https://support.twilio.com/hc/en-us/articles/9321443984155-Why-Was-My-Toll-Free-Verification-Rejected) 

[Required information for Toll-free verification](https://support.twilio.com/hc/en-us/articles/13264118705051-Required-Information-for-Toll-Free-Verification)

[Toll-Free best practices](https://support.twilio.com/hc/en-us/articles/360038172934-Information-and-best-practices-for-using-Toll-Free-SMS-and-MMS-in-the-US-and-Canada)

[Twilio’s Messaging Policy](https://www.twilio.com/en-us/legal/messaging-policy)"#),
            TwilioProgrammableMessagingError::ErrorCode30753 => Some(r#"Please use a valid and supported email address.

Additional help resources

* [Troubleshooting US A2P Brand registration failures]"#),
            TwilioProgrammableMessagingError::ErrorCode30448 => Some(r#"* Add a robust age gate to your website or opt-in policy. Resubmit the toll free verification with the updated age gate information. 

[Why was my Toll Free Verification Rejected?](https://support.twilio.com/hc/en-us/articles/9321443984155-Why-Was-My-Toll-Free-Verification-Rejected) 

[Required information for Toll-free verification](https://support.twilio.com/hc/en-us/articles/13264118705051-Required-Information-for-Toll-Free-Verification)

[Toll-Free best practices](https://support.twilio.com/hc/en-us/articles/360038172934-Information-and-best-practices-for-using-Toll-Free-SMS-and-MMS-in-the-US-and-Canada)

[Twilio’s Messaging Policy](https://www.twilio.com/en-us/legal/messaging-policy)"#),
            TwilioProgrammableMessagingError::ErrorCode30449 => Some(r#"* Replace call to action (CTA) in sample message with a branded URL. Resubmit the toll free verification with an updated sample message.  
* Ensure that any URL's sent in messages are not sent from a public URL shortener.
* Consider utilizing Twilio’s Link Shortening to programmatically shorten long links with your own company branded domain. 

[Link Shortening](https://help.twilio.com/articles/1260804572090-How-can-I-send-shortened-URLs-links-in-my-messages)

[Why was my Toll Free Verification Rejected?](https://support.twilio.com/hc/en-us/articles/9321443984155-Why-Was-My-Toll-Free-Verification-Rejected) 

[Required information for Toll-free verification](https://support.twilio.com/hc/en-us/articles/13264118705051-Required-Information-for-Toll-Free-Verification)

[Toll-Free best practices](https://support.twilio.com/hc/en-us/articles/360038172934-Information-and-best-practices-for-using-Toll-Free-SMS-and-MMS-in-the-US-and-Canada)

[Twilio’s Messaging Policy](https://www.twilio.com/en-us/legal/messaging-policy)"#),
            TwilioProgrammableMessagingError::ErrorCode30790 => Some(r#"Please contact Twilio Support."#),
            TwilioProgrammableMessagingError::ErrorCode30459 => Some(r#"If you are confident that your business and messaging use case does not fit into forbidden message categories, contact Twilio Support to provide Additional Information that explains your use case.

[Forbidden Message Categories](https://support.twilio.com/hc/en-us/articles/360045004974-Forbidden-message-categories-for-SMS-and-MMS-in-the-US-and-Canada)

[Why was my Toll Free Verification Rejected?](https://support.twilio.com/hc/en-us/articles/9321443984155-Why-Was-My-Toll-Free-Verification-Rejected) 

[Required information for Toll-free verification](https://support.twilio.com/hc/en-us/articles/13264118705051-Required-Information-for-Toll-Free-Verification)

[Toll-Free best practices](https://support.twilio.com/hc/en-us/articles/360038172934-Information-and-best-practices-for-using-Toll-Free-SMS-and-MMS-in-the-US-and-Canada)

[Twilio’s Messaging Policy](https://www.twilio.com/en-us/legal/messaging-policy)"#),
            TwilioProgrammableMessagingError::ErrorCode30795 => Some(r#"Please edit your customer profile and update the business registration identifier with correct information that matches your tax returns. Please also make sure that your legal company name also matches the tax documents.

Additional help resources:

* [Troubleshooting US A2P Brand registration failures](https://www.twilio.com/docs/messaging/compliance/a2p-10dlc/troubleshooting-a2p-brands)"#),
            TwilioProgrammableMessagingError::ErrorCode30437 => Some(r#"Edit and re-submit the toll-free verification. The resubmission will still be received, but it is handled as a new submission, so the review will take slightly longer.

[Why was my Toll Free Verification Rejected?](https://support.twilio.com/hc/en-us/articles/9321443984155-Why-Was-My-Toll-Free-Verification-Rejected) 

[Required information for Toll-free verification](https://support.twilio.com/hc/en-us/articles/13264118705051-Required-Information-for-Toll-Free-Verification)

[Toll-Free best practices](https://support.twilio.com/hc/en-us/articles/360038172934-Information-and-best-practices-for-using-Toll-Free-SMS-and-MMS-in-the-US-and-Canada)

[Twilio’s Messaging Policy](https://www.twilio.com/en-us/legal/messaging-policy)"#),
            TwilioProgrammableMessagingError::ErrorCode30887 => Some(r#"Please verify that opt-out workflow is accurate and update Call-to-Action/ Message Flow description with opt-out process.

If opt-out is managed, add opt-out keywords and update opt-out message to to include acknowledgement of opt-out request, confirmation that no further messages will be sent, and brand name.
 
Additional help resources

* [A2P 10DLC campaign approval best practices](https://support.twilio.com/hc/en-us/articles/11847054539547-A2P-10DLC-Campaign-Approval-Best-Practices)"#),
            TwilioProgrammableMessagingError::ErrorCode30880 => Some(r#"Our Support team has the means to obtain more detailed information about this specific error. Please contact [Twilio Customer Support]( https://www.twilio.com/help/contact) for assistance in understanding the underlying problem and finding a resolution. "#),
            TwilioProgrammableMessagingError::ErrorCode30899 => Some(r#"Please reach out to customer support with campaign and account details for resolution. "#),
            TwilioProgrammableMessagingError::ErrorCode30905 => Some(r#"* Await the CAMPAIGN_SHARE_ACCEPT webhook from TCR before assigning the Messaging Service to the CampaignID.
* Note: Campaign will remain in a 'FAILED' state until the Messaging Service is successfully re-attached.

 
Additional help resources

* [Changes to Externally Registered Campaign Submission](https://support.twilio.com/hc/en-us/articles/13097615242395-Changes-to-Externally-Registered-Campaign-Submission)"#),
            TwilioProgrammableMessagingError::ErrorCode36006 => Some(r#"* Ensure that you provide a Broadcast SID for a Broadcast that is still in progress"#),
            TwilioProgrammableMessagingError::ErrorCode30479 => Some(r#"Resubmit and explain the reason for additional toll-free phone numbers within the Additional information field.

Resources: 

[CTIA guidelines](https://www.ctia.org/the-wireless-industry/industry-commitments/messaging-interoperability-sms-mms)

[Consent/Opt-In](https://www.twilio.com/en-us/legal/messaging-policy#:~:text=with%20Twilio%20policies.-,Consent%20/%20Opt%2Din,-What%20Is%20Proper) in [Twilio’s Messaging Policy](https://www.twilio.com/en-us/legal/messaging-policy)

[Why was my Toll Free Verification Rejected?](https://support.twilio.com/hc/en-us/articles/9321443984155-Why-Was-My-Toll-Free-Verification-Rejected) 

[Required information for Toll-free verification](https://support.twilio.com/hc/en-us/articles/13264118705051-Required-Information-for-Toll-Free-Verification)

[Toll-Free best practices](https://support.twilio.com/hc/en-us/articles/360038172934-Information-and-best-practices-for-using-Toll-Free-SMS-and-MMS-in-the-US-and-Canada)"#),
            TwilioProgrammableMessagingError::ErrorCode30886 => Some(r#"Verify that campaign description is accurate and detailed. If you are an ISV registering a direct offering to your customers, please indicate that in the campaign description. Resubmit campaign for review.
 
Additional help resources

* [A2P 10DLC campaign approval best practices](https://support.twilio.com/hc/en-us/articles/11847054539547-A2P-10DLC-Campaign-Approval-Best-Practices)"#),
            TwilioProgrammableMessagingError::ErrorCode30882 => Some(r#"Ineligible for resubmission. If you feel that this was in error, contact Twilio Customer Support.

Additional help resources

* [Forbidden message categories](https://support.twilio.com/hc/en-us/articles/360045004974-Forbidden-message-categories-for-SMS-and-MMS-in-the-US-and-Canada)

* [A2P 10DLC campaign approval best practices](https://support.twilio.com/hc/en-us/articles/11847054539547-A2P-10DLC-Campaign-Approval-Best-Practices)"#),
            TwilioProgrammableMessagingError::ErrorCode21733 => Some(r#"Contact Twilio messaging support with the account sid and error code to make the error code disappear, the outbound message itself should still be successful."#),
            TwilioProgrammableMessagingError::ErrorCode30464 => Some(r#"If you are confident that your business and messaging use case does not fit into forbidden message categories, contact Twilio Support to provide Additional Information that explains your use case.

[Forbidden Message Categories](https://support.twilio.com/hc/en-us/articles/360045004974-Forbidden-message-categories-for-SMS-and-MMS-in-the-US-and-Canada)

[Why was my Toll Free Verification Rejected?](https://support.twilio.com/hc/en-us/articles/9321443984155-Why-Was-My-Toll-Free-Verification-Rejected) 

[Required information for Toll-free verification](https://support.twilio.com/hc/en-us/articles/13264118705051-Required-Information-for-Toll-Free-Verification)

[Toll-Free best practices](https://support.twilio.com/hc/en-us/articles/360038172934-Information-and-best-practices-for-using-Toll-Free-SMS-and-MMS-in-the-US-and-Canada)

[Twilio’s Messaging Policy](https://www.twilio.com/en-us/legal/messaging-policy)"#),
            TwilioProgrammableMessagingError::ErrorCode30752 => Some(r#"Please use a valid OTP for verifying your Sole Proprietor brand. You can use our public api or console to request a latest OTP and use that for verification.

Additional help resources

* [Troubleshooting US A2P Brand registration failures]"#),
            TwilioProgrammableMessagingError::ErrorCode35132 => Some(r#"Check if all the attributes are in expected format"#),
            TwilioProgrammableMessagingError::ErrorCode21268 => Some(r#"Twilio doesn't allow sending SMS messages to premium rate or information service numbers. 
"#),
            TwilioProgrammableMessagingError::ErrorCode30472 => Some(r#"- Verify that the business is a valid business
- Ensure all business information fields are valid and accurately represent the business who manages the opt-in and has the relationship with the consumer:

  **BusinessName**, **BusinessStreetAddress**, **BusinessCity**, **BusinessStateProvinceRegion**, **BusinessPostalCode**, **BusinessCountry**, **BusinessContactFirstName**, **BusinessContactLastName**, **BusinessContactEmail**, **BusinessContactPhone**, **BusinessWebsite**
- Correct the business information fields on toll-free verification submission by editing it and then resubmit it.

[Why was my Toll Free Verification Rejected?](https://support.twilio.com/hc/en-us/articles/9321443984155-Why-Was-My-Toll-Free-Verification-Rejected) 

[Required information for Toll-free verification](https://support.twilio.com/hc/en-us/articles/13264118705051-Required-Information-for-Toll-Free-Verification)

[Toll-Free best practices](https://support.twilio.com/hc/en-us/articles/360038172934-Information-and-best-practices-for-using-Toll-Free-SMS-and-MMS-in-the-US-and-Canada)

[Twilio’s Messaging Policy](https://www.twilio.com/en-us/legal/messaging-policy)
"#),
            TwilioProgrammableMessagingError::ErrorCode30135 => Some(r#"Please verify your Domain DNS setup. If you continue seeing this error, please contact the Support team"#),
            TwilioProgrammableMessagingError::ErrorCode30898 => Some(r#"Only register minimum number of brands per EIN and do not resubmit until brand registration is updated.

Additional help resources

* [A2P 10DLC campaign approval best practices](https://support.twilio.com/hc/en-us/articles/11847054539547-A2P-10DLC-Campaign-Approval-Best-Practices)"#),
            TwilioProgrammableMessagingError::ErrorCode21265 => Some(r#"Ensure you are not attempting to send messages to Short Codes. These sender types cannot receive messages from Twilio numbers."#),
            TwilioProgrammableMessagingError::ErrorCode30904 => Some(r#"* Ensure that the Campaign has been successfully shared with Twilio as the DCA and has received approval.
* Confirm that the entered CampaignID is accurate.
* If needed, re-share the Campaign through TCR, await the CAMPAIGN_SHARE_ACCEPT webhook, and then proceed to assign the Messaging Service.

Additional help resources
* [Changes to Externally Registered Campaign Submission](https://support.twilio.com/hc/en-us/articles/13097615242395-Changes-to-Externally-Registered-Campaign-Submission)
"#),
            TwilioProgrammableMessagingError::ErrorCode30791 => Some(r#"Please try again after 30 seconds.

"#),
            TwilioProgrammableMessagingError::ErrorCode30794 => Some(r#"Please fix the cause of error message and retry the registration request. If issue persists, please contact Twilio support.

Additional help resources

* [Troubleshooting US A2P Brand registration failures](https://www.twilio.com/docs/messaging/compliance/a2p-10dlc/troubleshooting-a2p-brands)

* [Troubleshooting and rectifying US A2P Campaigns](https://www.twilio.com/docs/messaging/compliance/a2p-10dlc/troubleshooting-a2p-brands/troubleshooting-and-rectifying-a2p-campaigns-1)

"#),
            TwilioProgrammableMessagingError::ErrorCode30889 => Some(r#"Please verify that embedded phone number selection is accurate. Update the Sample Messages with Embedded Phone Number or update Embedded Phone Number selection. Resubmit campaign for review.
 
Additional help resources

* [A2P 10DLC campaign approval best practices](https://support.twilio.com/hc/en-us/articles/11847054539547-A2P-10DLC-Campaign-Approval-Best-Practices)"#),
            TwilioProgrammableMessagingError::ErrorCode30478 => Some(r#"* Use the toll-free phone number for only one business.  
* Resubmit the verification and provide Additional Information that explains how you rectified this. 

Resources: 

[Why was my Toll Free Verification Rejected?](https://support.twilio.com/hc/en-us/articles/9321443984155-Why-Was-My-Toll-Free-Verification-Rejected) 

[Required information for Toll-free verification](https://support.twilio.com/hc/en-us/articles/13264118705051-Required-Information-for-Toll-Free-Verification)

[Toll-Free best practices](https://support.twilio.com/hc/en-us/articles/360038172934-Information-and-best-practices-for-using-Toll-Free-SMS-and-MMS-in-the-US-and-Canada)

[Twilio’s Messaging Policy](https://www.twilio.com/en-us/legal/messaging-policy)"#),
            TwilioProgrammableMessagingError::ErrorCode63040 => Some(r#"Submit a new template that doesn’t violate the above causes. We also recommend the follow best practices when creating templates:
  * Templates should be as short as possible to get the necessary information across.
  * Templates must not have any grammatical or spelling mistakes. 
  * Templates that include major spelling or grammatical mistakes are likely to be rejected. Templates with minor punctuation errors or grammatical inconsistencies may be approved, but should be avoided.
  * When submitting your Template in Console, ensure you select the appropriate message type and language.
  * Get feedback from your teammates or coworkers to ensure the template you’ve written is clear and free of errors.
  * Think about a template message as a conversation starter, with the goal to convert this into a 2-way message by having the user reply. The two-way use case will keep your spend lower, as WhatsApp does not charge the Template message fee for two-way messages.
  * Make sure your customers understand why they received your message – you can remind them of the reason why they originally gave you permission to send them messages.
  * Avoid sending surveys or using the word “survey”. Instead, you may ask the customer for their feedback.

If you believe you have an approved template that matches the message you are attempting to send, then we recommend deleting any rejected templates that are similar to the approved template. You may also use the Twilio Content API, which lets you send templates by referencing a template Sid. To learn more, please see: https://www.twilio.com/docs/content-api"#),
            TwilioProgrammableMessagingError::ErrorCode30909 => Some(r#"1. Provide complete CTA information and all possible methods of receiving consent. If opt-in is not conducted through the website, please detail and provide documentation how the CTA is presented to and actioned by end-users.
 Once you have made a change to address CTA verification issues, please resubmit the campaign for review.

Additional help resources

* [A2P 10DLC campaign approval best practices](https://support.twilio.com/hc/en-us/articles/11847054539547-A2P-10DLC-Campaign-Approval-Best-Practices)
"#),
            TwilioProgrammableMessagingError::ErrorCode30443 => Some(r#"If you are confident that your business and messaging use case does not fit into forbidden message categories, contact Twilio Support to provide Additional Information that explains your use case.


[Forbidden Message Categories](https://support.twilio.com/hc/en-us/articles/360045004974-Forbidden-message-categories-for-SMS-and-MMS-in-the-US-and-Canada)

[Why was my Toll Free Verification Rejected?](https://support.twilio.com/hc/en-us/articles/9321443984155-Why-Was-My-Toll-Free-Verification-Rejected) 

[Required information for Toll-free verification](https://support.twilio.com/hc/en-us/articles/13264118705051-Required-Information-for-Toll-Free-Verification)

[Toll-Free best practices](https://support.twilio.com/hc/en-us/articles/360038172934-Information-and-best-practices-for-using-Toll-Free-SMS-and-MMS-in-the-US-and-Canada)

[Twilio’s Messaging Policy](https://www.twilio.com/en-us/legal/messaging-policy)"#),
            TwilioProgrammableMessagingError::ErrorCode30462 => Some(r#"If you are confident that your business and messaging use case does not fit into forbidden message categories, contact Twilio Support to provide Additional Information that explains your use case.


[Forbidden Message Categories](https://support.twilio.com/hc/en-us/articles/360045004974-Forbidden-message-categories-for-SMS-and-MMS-in-the-US-and-Canada)

[Why was my Toll Free Verification Rejected?](https://support.twilio.com/hc/en-us/articles/9321443984155-Why-Was-My-Toll-Free-Verification-Rejected) 

[Required information for Toll-free verification](https://support.twilio.com/hc/en-us/articles/13264118705051-Required-Information-for-Toll-Free-Verification)

[Toll-Free best practices](https://support.twilio.com/hc/en-us/articles/360038172934-Information-and-best-practices-for-using-Toll-Free-SMS-and-MMS-in-the-US-and-Canada)

[Twilio’s Messaging Policy](https://www.twilio.com/en-us/legal/messaging-policy)"#),
            TwilioProgrammableMessagingError::ErrorCode63024 => Some(r#"* Confirm with the recipient that they have accepted Meta's latest Terms of Service for WhatsApp. If they have not, then send them this link https://wa.me/tos/20210210 to accept the latest WhatsApp Terms of Service.
* Confirm with the recipient that they have an active WhatsApp phone number on the latest client version for their phone."#),
            TwilioProgrammableMessagingError::ErrorCode30136 => Some(r#"Please verify your Domain DNS setup. If you continue seeing this error, please contact the Support team"#),
            TwilioProgrammableMessagingError::ErrorCode30703 => Some(r#"Depending on the fields you may have received in the error message, please refer to the following solutions:

* Use a unique US mobile phone number for the registration. 

* Use a unique business registration identifier for successful registration.

* Use a unique business name.

* Use a unique authorized representative.

* Use a unique email address.

* Use a unique website address.

* Use a unique business name.

* Use a unique stock symbol.


Additional help resources

* [Troubleshooting US A2P Brand registration failures](https://www.twilio.com/docs/messaging/compliance/a2p-10dlc/troubleshooting-a2p-brands)"#),
            TwilioProgrammableMessagingError::ErrorCode30799 => Some(r#"#### For public entities:

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
"#),
            TwilioProgrammableMessagingError::ErrorCode11206 => Some(r#"* Verify your web server is serving valid HTTP for the URL	
* Verify the network connection to your server is stable	
* Verify the URL is not directing you to an SSL port
* Remove the following characters from the TwiML Cookies: ‘\t’, ‘\r’, ‘\n’, ‘\v’, ‘\f’
* TwiML cookies cannot have an empty name
"#),
            TwilioProgrammableMessagingError::ErrorCode63042 => Some(r#"- Use a different template
- Create a new template"#),
            TwilioProgrammableMessagingError::ErrorCode30712 => Some(r#"Please contact Twilio Support with relevant error details.

Additional help resources

* [Troubleshooting US A2P Brand registration failures](https://www.twilio.com/docs/messaging/compliance/a2p-10dlc/troubleshooting-a2p-brands)"#),
            TwilioProgrammableMessagingError::ErrorCode30466 => Some(r#"If you are confident that your business and messaging use case does not fit into forbidden message categories, contact Twilio Support to provide Additional Information that explains your use case.

[Forbidden Message Categories](https://support.twilio.com/hc/en-us/articles/360045004974-Forbidden-message-categories-for-SMS-and-MMS-in-the-US-and-Canada)

[Why was my Toll Free Verification Rejected?](https://support.twilio.com/hc/en-us/articles/9321443984155-Why-Was-My-Toll-Free-Verification-Rejected) 

[Required information for Toll-free verification](https://support.twilio.com/hc/en-us/articles/13264118705051-Required-Information-for-Toll-Free-Verification)

[Toll-Free best practices](https://support.twilio.com/hc/en-us/articles/360038172934-Information-and-best-practices-for-using-Toll-Free-SMS-and-MMS-in-the-US-and-Canada)

[Twilio’s Messaging Policy](https://www.twilio.com/en-us/legal/messaging-policy)
"#),
            TwilioProgrammableMessagingError::ErrorCode21661 => Some(r#"* Check that you are using a Twilio phone number with SMS capabilities. You can see your purchased phone numbers and their capabilities in the [Twilio console](https://www.twilio.com/console/phone-numbers/incoming).
* Purchase an SMS capable number from the [Twilio Console](https://console.twilio.com/us1/develop/phone-numbers/manage/search?isoCountry=US&types%5B%5D=Local&types%5B%5D=Tollfree&capabilities%5B%5D=Sms&searchTerm=&searchFilter=left&searchType=number)."#),
            TwilioProgrammableMessagingError::ErrorCode30756 => Some(r#"Please use natural text inputs for business name, first and last name.

Additional help resources

* [Troubleshooting US A2P Brand registration failures]"#),
            TwilioProgrammableMessagingError::ErrorCode30747 => Some(r#"Please edit the connected starter customer profile and link a different (unique) address to it. Once updated and submitted for review, please resubmit the Sole Proprietor Brand registration request.

Additional help resources

* [Troubleshooting US A2P Brand registration failures]"#),
            TwilioProgrammableMessagingError::ErrorCode30881 => Some(r#"Please verify that brand support email is valid and not associated with a public domain email.
 
Additional help resources

* [A2P 10DLC campaign approval best practices](https://support.twilio.com/hc/en-us/articles/11847054539547-A2P-10DLC-Campaign-Approval-Best-Practices)"#),
            TwilioProgrammableMessagingError::ErrorCode30615 => Some(r#"**Verify Local Time:** Implement a system to accurately determine the local time zone of each recipient based on their area code and address data.

**Schedule Adjustments:** Configure your messaging campaign schedules to operate only within the permitted hours of 8 a.m. to 9 p.m. local time for each recipient.

#### Continued issues with error 30615

Twilio's support team can help investigate what went wrong with delivering your message. Please collect 3 or more message SIDs in your SMS [logs](https://www.twilio.com/console/sms/logs) from the last 24 hours that were flagged with Error 30615, and [open a support request.](https://www.twilio.com/console/support/tickets/create)"#),
            TwilioProgrammableMessagingError::ErrorCode30458 => Some(r#"If you are confident that your business and messaging use case does not fit into forbidden message categories, contact Twilio Support to provide Additional Information that explains your use case.

[Forbidden Message Categories](https://support.twilio.com/hc/en-us/articles/360045004974-Forbidden-message-categories-for-SMS-and-MMS-in-the-US-and-Canada)

[Why was my Toll Free Verification Rejected?](https://support.twilio.com/hc/en-us/articles/9321443984155-Why-Was-My-Toll-Free-Verification-Rejected) 

[Required information for Toll-free verification](https://support.twilio.com/hc/en-us/articles/13264118705051-Required-Information-for-Toll-Free-Verification)

[Toll-Free best practices](https://support.twilio.com/hc/en-us/articles/360038172934-Information-and-best-practices-for-using-Toll-Free-SMS-and-MMS-in-the-US-and-Canada)

[Twilio’s Messaging Policy](https://www.twilio.com/en-us/legal/messaging-policy)"#),
            TwilioProgrammableMessagingError::ErrorCode30755 => Some(r#"Please use a valid country code.


Additional help resources

* [Troubleshooting US A2P Brand registration failures]"#),
            TwilioProgrammableMessagingError::ErrorCode36002 => Some(r#"* Wait for your existing Broadcasts to process before creating more Broadcasts
* If you have any Broadcasts with status PENDING_UPLOAD, upload a file to those Broadcasts first to resume processing on them"#),
            TwilioProgrammableMessagingError::ErrorCode63044 => Some(r#"- Update the provided URL"#),
            TwilioProgrammableMessagingError::ErrorCode30792 => Some(r#"Please contact Twilio Support."#),
            TwilioProgrammableMessagingError::ErrorCode30900 => Some(r#"Try to select another use case for campaign registration that reflects your A2P Messaging use case with Twilio, or reach out to support with campaign details for resolution. "#),
            TwilioProgrammableMessagingError::ErrorCode63049 => Some(r#"The limit only applies to marketing template messages that would normally open a new marketing conversation. If a marketing conversation is already open between you and a WhatsApp user, you may send one additional marketing template message. Further marketing template messages can only be sent in an open marketing conversation if the person responds to any message.

Example:

- The first marketing template message is delivered and opens a new 24-hour marketing conversation customer service window. The per-user marketing template message limit applies

- A second marketing template message can be sent in an existing conversation.

- Each time the WhatsApp user responds in an existing conversation window, you can send one additional marketing template message. You can also send unlimited free-form messages."#),
            TwilioProgrammableMessagingError::ErrorCode30477 => Some(r#"Revise your processes to not share or sell consumer data/opt-in information with anyone. Revise your Terms of Service and Privacy Policy to remove all third party opt-in sharing language. If you have a “reasons we can share your personal information matrix” in your privacy policy that is not modifiable, adding a note below it that Messaging opt-in consent will not be shared is acceptable. Once complete and live on the business website, resubmit the toll free verification. 

Resources: 

[CTIA guidelines](https://www.ctia.org/the-wireless-industry/industry-commitments/messaging-interoperability-sms-mms)

[Consent/Opt-In](https://www.twilio.com/en-us/legal/messaging-policy#:~:text=with%20Twilio%20policies.-,Consent%20/%20Opt%2Din,-What%20Is%20Proper) in [Twilio’s Messaging Policy](https://www.twilio.com/en-us/legal/messaging-policy)

[Why was my Toll Free Verification Rejected?](https://support.twilio.com/hc/en-us/articles/9321443984155-Why-Was-My-Toll-Free-Verification-Rejected) 

[Required information for Toll-free verification](https://support.twilio.com/hc/en-us/articles/13264118705051-Required-Information-for-Toll-Free-Verification)

[Toll-Free best practices](https://support.twilio.com/hc/en-us/articles/360038172934-Information-and-best-practices-for-using-Toll-Free-SMS-and-MMS-in-the-US-and-Canada)"#),
            TwilioProgrammableMessagingError::ErrorCode30630 => Some(r#"- Consider removing this phone number from your list of recipients.
- Request the recipient to resubscribe to your messages by texting in "START" or [another opt-in keyword](https://help.twilio.com/articles/223134027-Twilio-support-for-opt-out-keywords-SMS-STOP-filtering) to your Twilio sender.
- Before sending messages to a recipient, ensure they have consented to receive messages from you. Please read these guidelines to understand [messaging opt-in requirements and best practices.](https://www.twilio.com/en-us/blog/ctia-messaging-principles-and-best-practices)"#)
        }
    }
}
