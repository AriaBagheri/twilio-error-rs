// AUTO-GENERATED. DO NOT MODIFY. YOUR CHANGES WILL BE OVER-WRITTEN
use crate::products::TwilioProgrammableMessagingError;
use standard_error::traits::StandardErrorDescriptionTrait;

impl StandardErrorDescriptionTrait for TwilioProgrammableMessagingError {
    fn description(&self) -> Option<&'static str> {
        match self {
            TwilioProgrammableMessagingError::ErrorCode20504 => Some(r#"Twilio's platform encountered an internal error while processing this message."#),
            TwilioProgrammableMessagingError::ErrorCode30884 => Some(r#"The campaign submission has been reviewed and it was rejected because it was found to be a known Spam or Phishing campaign."#),
            TwilioProgrammableMessagingError::ErrorCode35133 => Some(r#"The 'Messages' array provided in the request in invalid. It is either empty or contains too many items"#),
            TwilioProgrammableMessagingError::ErrorCode30758 => Some(r#"Registration Authority was not provided for brand registration to complete."#),
            TwilioProgrammableMessagingError::ErrorCode21660 => Some(r#"The `From` number you are using to send an SMS is not associated with your account. 
"#),
            TwilioProgrammableMessagingError::ErrorCode30897 => Some(r#"The campaign submission has been reviewed and it was rejected due to Disallowed Content."#),
            TwilioProgrammableMessagingError::ErrorCode63036 => Some(r#"The specified phone number cannot be reached using RCS at this time."#),
            TwilioProgrammableMessagingError::ErrorCode30447 => Some(r#"The Toll-Free phone number verification submission has been reviewed and it was rejected. Effective Jan 31st, 2024, messaging traffic on this Toll-Free number is blocked until it’s verified.

Verification is one component of ensuring A2P SMS is used by businesses in a legal and compliant manner. The review process looks at the business sending messaging (not the software powering it), the use case, sample content, the web presence, the opt-in/consent collected and associated website terms and privacy policies."#),
            TwilioProgrammableMessagingError::ErrorCode30444 => Some(r#"The Toll-Free phone number verification submission has been reviewed and it was rejected.Effective Jan 31st, 2024, messaging traffic on this Toll-Free number is blocked until it’s verified.

Verification is one component of ensuring A2P SMS is used by businesses in a legal and compliant manner. The review process looks at the business sending messaging (not the software powering it), the use case, sample content, the web presence, the opt-in/consent collected and associated website terms and privacy policies."#),
            TwilioProgrammableMessagingError::ErrorCode30474 => Some(r#"The Toll-Free phone number verification submission has been reviewed and it was rejected.Effective Jan 31st, 2024, messaging traffic on this Toll-Free number is blocked until it’s verified.

Verification is one component of ensuring A2P SMS is used by businesses in a legal and compliant manner. The review process looks at the business sending messaging (not the software powering it), the use case, sample content, the web presence, the opt-in/consent collected and associated website terms and privacy policies."#),
            TwilioProgrammableMessagingError::ErrorCode36005 => Some(r#"Uploaded file is of an invalid type"#),
            TwilioProgrammableMessagingError::ErrorCode30440 => Some(r#"The toll-free phone number cannot be verified due to an unknown error. Messaging traffic on this Toll-Free phone number is blocked."#),
            TwilioProgrammableMessagingError::ErrorCode30901 => Some(r#"The campaign registration request timed out due to an error."#),
            TwilioProgrammableMessagingError::ErrorCode30749 => Some(r#"Brand can not be registered with provided email address since duplicate threshold limit has been reached for it."#),
            TwilioProgrammableMessagingError::ErrorCode30888 => Some(r#"The campaign submission has been reviewed and it was rejected due to Age Gate issues."#),
            TwilioProgrammableMessagingError::ErrorCode21736 => Some(r#"The Brand 2FA attempt failed to validate the Brand Contact Email provided."#),
            TwilioProgrammableMessagingError::ErrorCode11321 => Some(r#"You received a message to one of your Twilio numbers, but we could not send a request to your web application using your provided webhook URL because its format is invalid.
"#),
            TwilioProgrammableMessagingError::ErrorCode30480 => Some(r#"The Toll-Free phone number verification submission has been reviewed and it was rejected. Messaging traffic on this Toll-Free phone number is blocked.

Verification is one component of ensuring A2P SMS is used by businesses in a legal and compliant manner. The review process looks at the business sending messaging (not the software powering it), the use case, sample content, the web presence, the opt-in/consent collected and associated website terms and privacy policies."#),
            TwilioProgrammableMessagingError::ErrorCode30750 => Some(r#"Brand can not be registered with provided mobile phone number since duplicate threshold limit has been reached for it."#),
            TwilioProgrammableMessagingError::ErrorCode30994 => Some(r#"Campaign Registration failed due to it's brand being in a pending state. Additional information may be required for the brand registration to proceed."#),
            TwilioProgrammableMessagingError::ErrorCode63041 => Some(r#"WhatsApp has reported that the template has been temporarily paused due to recurring negative feedback from customers. Message templates with this status cannot be sent to customers. For the most updated guidelines, please view [WhatsApp’s documentation](https://developers.facebook.com/docs/whatsapp/message-templates/guidelines/#template-pausing). "#),
            TwilioProgrammableMessagingError::ErrorCode30890 => Some(r#"The campaign submission has been reviewed and it was rejected because of provided Subscriber Help information."#),
            TwilioProgrammableMessagingError::ErrorCode11322 => Some(r#"You received a message to one of your Twilio numbers,  but we could not send a request to your web application using your webhook URL due to an incorrect HTTP method."#),
            TwilioProgrammableMessagingError::ErrorCode63047 => Some(r#"The link to the media file does not have a Content-Type header set to an accepted MIME type. As a result, the template was not created."#),
            TwilioProgrammableMessagingError::ErrorCode30473 => Some(r#"The Toll-Free phone number verification submission has been reviewed and it was rejected.Effective Jan 31st, 2024, messaging traffic on this Toll-Free number is blocked until it’s verified.

Verification is one component of ensuring A2P SMS is used by businesses in a legal and compliant manner. The review process looks at the business sending messaging (not the software powering it), the use case, sample content, the web presence, the opt-in/consent collected and associated website terms and privacy policies.
"#),
            TwilioProgrammableMessagingError::ErrorCode30647 => Some(r#"This error occurs when a contact object fails validation during upsert operations. The error can result from one or more of the following issues: invalid correlation ID, incorrect phone number format, invalid country ISO code, or an improperly formatted postal code. Additionally, this error may also indicate an internal server issue during the upsert process."#),
            TwilioProgrammableMessagingError::ErrorCode216602 => Some(r#"The content types in the template cannot be used on the channel specified"#),
            TwilioProgrammableMessagingError::ErrorCode30446 => Some(r#"The Toll-Free phone number verification submission has been reviewed and it was rejected.Effective Jan 31st, 2024, messaging traffic on this Toll-Free number is blocked until it’s verified.

Verification is one component of ensuring A2P SMS is used by businesses in a legal and compliant manner. The review process looks at the business sending messaging (not the software powering it), the use case, sample content, the web presence, the opt-in/consent collected and associated website terms and privacy policies."#),
            TwilioProgrammableMessagingError::ErrorCode21737 => Some(r#"Campaign registration failed due to the Brand missing a Brand Contact Email. "#),
            TwilioProgrammableMessagingError::ErrorCode30471 => Some(r#"The Toll-Free phone number verification submission has been reviewed and it was rejected. Effective Jan 31st, 2024, messaging traffic on this Toll-Free number is blocked until it’s verified.

Verification is one component of ensuring A2P SMS is used by businesses in a legal and compliant manner. The review process looks at the business sending messaging (not the software powering it), the use case, sample content, the web presence, the opt-in/consent collected and associated website terms and privacy policies."#),
            TwilioProgrammableMessagingError::ErrorCode21731 => Some(r#"When an A2P 10DLC Brand is in a suspended state, the following operations are not allowed:
• Create a new campaign - when you try to create a new campaign using a suspended brand, this error code is returned
• Update brand - when you try to update a suspended brand, this error code is returned
"#),
            TwilioProgrammableMessagingError::ErrorCode30460 => Some(r#"The Toll-Free phone number verification submission has been reviewed and it was rejected. Effective Jan 31st, 2024, messaging traffic on this Toll-Free number is blocked until it’s verified.

Verification is one component of ensuring A2P SMS is used by businesses in a legal and compliant manner. The review process looks at the business sending messaging (not the software powering it), the use case, sample content, the web presence, the opt-in/consent collected and associated website terms and privacy policies."#),
            TwilioProgrammableMessagingError::ErrorCode36001 => Some(r#"The Broadcast SID entered was not found"#),
            TwilioProgrammableMessagingError::ErrorCode30754 => Some(r#"You have used an invalid postal address for brand registration."#),
            TwilioProgrammableMessagingError::ErrorCode20410 => Some(r#"The API is no longer available"#),
            TwilioProgrammableMessagingError::ErrorCode30646 => Some(r#"This error occurs when one or more fields in a consent object are invalid during the upsert process, or due to an internal server issue. The contact's consent status may fail validation due to issues with the sender ID, opt-in status, or the source from which the consent was collected."#),
            TwilioProgrammableMessagingError::ErrorCode36004 => Some(r#"File couldn't be retrieved, make sure it has 'file' key"#),
            TwilioProgrammableMessagingError::ErrorCode30610 => Some(r#"Message couldn’t be delivered to the recipients number you are trying to reach at this time due to local regulatory restrictions."#),
            TwilioProgrammableMessagingError::ErrorCode21659 => Some(r#"You can only send SMS messages from a phone number, Alphanumeric Sender ID or [short code](https://www.twilio.com/docs/glossary/what-is-a-short-code) provided by or ported to Twilio. 

For Short Codes, the `From` number must be in the same country as the `To` number.
"#),
            TwilioProgrammableMessagingError::ErrorCode30734 => Some(r#"Sole proprietor brand registration is not enabled for your account."#),
            TwilioProgrammableMessagingError::ErrorCode30475 => Some(r#"The Toll-Free phone number verification submission has been reviewed and it was rejected. Effective Jan 31st, 2024, messaging traffic on this Toll-Free number is blocked until it’s verified.

Verification is one component of ensuring A2P SMS is used by businesses in a legal and compliant manner. The review process looks at the business sending messaging (not the software powering it), the use case, sample content, the web presence, the opt-in/consent collected and associated website terms and privacy policies.
"#),
            TwilioProgrammableMessagingError::ErrorCode30993 => Some(r#"Campaign's CNP Migration did not complete in expected time!"#),
            TwilioProgrammableMessagingError::ErrorCode30903 => Some(r#"The campaign for a Sole Proprietorship Brand has been rejected due to incorrect registration and failure to meet the small business Sole Proprietor (EIN) criteria set by TCR and mobile carriers."#),
            TwilioProgrammableMessagingError::ErrorCode30701 => Some(r#"Brand registration request contained one or more invalid inputs. "#),
            TwilioProgrammableMessagingError::ErrorCode21267 => Some(r#"You attempted to send an SMS from a Trial Account using an Alphanumeric Sender ID as the `From` parameter.
"#),
            TwilioProgrammableMessagingError::ErrorCode21732 => Some(r#"You can only set up one Sole Proprietor Campaign per Sole Proprietor Brand. Please note that campaigns in all statuses (such as Pending, Registered, Failed) count towards this limit."#),
            TwilioProgrammableMessagingError::ErrorCode30445 => Some(r#"The Toll-Free phone number verification submission has been reviewed and it was rejected. Effective Jan 31st, 2024, messaging traffic on this Toll-Free number is blocked until it’s verified."#),
            TwilioProgrammableMessagingError::ErrorCode21266 => Some(r#"You attempted to send an SMS, but the `To` phone number you supplied is the same as the `From` number.
"#),
            TwilioProgrammableMessagingError::ErrorCode30757 => Some(r#"Business registration number was not provided for brand registration to complete."#),
            TwilioProgrammableMessagingError::ErrorCode30456 => Some(r#"The Toll-Free phone number verification submission has been reviewed and it was rejected. Effective Jan 31st, 2024, messaging traffic on this Toll-Free number is blocked until it’s verified.

Verification is one component of ensuring A2P SMS is used by businesses in a legal and compliant manner. The review process looks at the business sending messaging (not the software powering it), the use case, sample content, the web presence, the opt-in/consent collected and associated website terms and privacy policies."#),
            TwilioProgrammableMessagingError::ErrorCode63051 => Some(r#"Twilio cannot send the message because WhatsApp has locked the WhatsApp Business Account that contains this Sender due to an integrity policy violation"#),
            TwilioProgrammableMessagingError::ErrorCode11100 => Some(r#"The format of the provided URL is invalid.
"#),
            TwilioProgrammableMessagingError::ErrorCode63046 => Some(r#"WhatsApp template approval status has changed to approved."#),
            TwilioProgrammableMessagingError::ErrorCode30908 => Some(r#"The campaign submission has been reviewed and rejected because a compliant privacy policy can not be verified."#),
            TwilioProgrammableMessagingError::ErrorCode21703 => Some(r#"You have phone numbers or short codes in your Messaging Service, but none of them are capable of sending the requested message or media to this recipient.
Possible causes vary depending on the type of message and the country of the destination number. Below are several common causes of this error."#),
            TwilioProgrammableMessagingError::ErrorCode30463 => Some(r#"The Toll-Free phone number verification submission has been reviewed and it was rejected. Effective Jan 31st, 2024, messaging traffic on this Toll-Free number is blocked until it’s verified.

Verification is one component of ensuring A2P SMS is used by businesses in a legal and compliant manner. The review process looks at the business sending messaging (not the software powering it), the use case, sample content, the web presence, the opt-in/consent collected and associated website terms and privacy policies."#),
            TwilioProgrammableMessagingError::ErrorCode30895 => Some(r#"The campaign cannot be approved because it appears to be associated with direct lending or loan arrangement, and the necessary content attribute was not indicated"#),
            TwilioProgrammableMessagingError::ErrorCode21635 => Some(r#"You have attempted to send an SMS with a `To` number that is a landline. "#),
            TwilioProgrammableMessagingError::ErrorCode30729 => Some(r#"Provided country code has been blacklisted and is not allowed for US A2P Brand registrations."#),
            TwilioProgrammableMessagingError::ErrorCode30896 => Some(r#"The campaign submission has been reviewed and it was rejected because of provided Opt-in information."#),
            TwilioProgrammableMessagingError::ErrorCode30906 => Some(r#"Messaging Service could not be attached to the Campaign because Twilio's DCA declined the Campaign due to compliance violations."#),
            TwilioProgrammableMessagingError::ErrorCode30907 => Some(r#"The campaign submission has been reviewed and rejected because the provided website URL does not match the Brand and Campaign registered."#),
            TwilioProgrammableMessagingError::ErrorCode30902 => Some(r#"The campaign registration request failed due to a DCA2 rejecting this campaign."#),
            TwilioProgrammableMessagingError::ErrorCode30441 => Some(r#"The Toll-Free phone number verification submission has been reviewed and it was rejected. Effective Jan 31st, 2024, messaging traffic on this Toll-Free number is blocked until it’s verified.

Verification is one component of ensuring A2P SMS is used by businesses in a legal and compliant manner. The review process looks at the business sending messaging (not the software powering it), the use case, sample content, the web presence, the opt-in/consent collected and associated website terms and privacy policies."#),
            TwilioProgrammableMessagingError::ErrorCode30465 => Some(r#"The Toll-Free phone number verification submission has been reviewed and it was rejected.Effective Jan 31st, 2024, messaging traffic on this Toll-Free number is blocked until it’s verified.

Verification is one component of ensuring A2P SMS is used by businesses in a legal and compliant manner. The review process looks at the business sending messaging (not the software powering it), the use case, sample content, the web presence, the opt-in/consent collected and associated website terms and privacy policies.
"#),
            TwilioProgrammableMessagingError::ErrorCode35127 => Some(r#"You provided a 'Messages' Body and 'ContentVariables' parameters in your request, which would lead to a conflict. Please provide only one of these. "#),
            TwilioProgrammableMessagingError::ErrorCode63045 => Some(r#"The link returned an unexpected status code, making it impossible to retrieve the file."#),
            TwilioProgrammableMessagingError::ErrorCode30797 => Some(r#"Your Brand has received feedback from The Campaign Registry upon registration. This Brand Registration can not be used for campaign creation until the provided feedback has been addressed. In this particular case, a non-government entity has been registered as a government entity. The entity registering the brand must be a U.S. government entity."#),
            TwilioProgrammableMessagingError::ErrorCode30885 => Some(r#"The campaign submission has been reviewed and it was rejected because it is considered High Risk."#),
            TwilioProgrammableMessagingError::ErrorCode30894 => Some(r#"The campaign submission has been reviewed and it was rejected because invalid brand information."#),
            TwilioProgrammableMessagingError::ErrorCode36009 => Some(r#"Stats for the Broadcast SID entered were not found"#),
            TwilioProgrammableMessagingError::ErrorCode30620 => Some(r#"Message couldn’t be delivered to the recipients number you are trying to reach due to a potential mismatch message intent with the registered use case of the Sender ID used."#),
            TwilioProgrammableMessagingError::ErrorCode30702 => Some(r#"There was an error while reconciling data with TCR."#),
            TwilioProgrammableMessagingError::ErrorCode30991 => Some(r#"The Brand of this Campaign is in an unverified status.  Campaign registrations under unverified brands is not allowed."#),
            TwilioProgrammableMessagingError::ErrorCode36008 => Some(r#"Provided page token query parameter is invalid.

The provided page token query parameter must be a valid Broadcast SID, consisting of 34 alphanumeric characters, prefixed with "BC"."#),
            TwilioProgrammableMessagingError::ErrorCode30883 => Some(r#"The campaign submission has been reviewed and it was rejected due to Content Violation."#),
            TwilioProgrammableMessagingError::ErrorCode30442 => Some(r#"The Toll-Free phone number verification submission has been reviewed and it was rejected. Effective Jan 31st, 2024, messaging traffic on this Toll-Free number is blocked until it’s verified.

Verification is one component of ensuring A2P SMS is used by businesses in a legal and compliant manner. The review process looks at the business sending messaging (not the software powering it), the use case, sample content, the web presence, the opt-in/consent collected and associated website terms and privacy policies."#),
            TwilioProgrammableMessagingError::ErrorCode30892 => Some(r#"The campaign submission has been reviewed and it was rejected because of URL shortener in the sample message."#),
            TwilioProgrammableMessagingError::ErrorCode30751 => Some(r#"You have provided an unsupported mobile phone number for brand registration."#),
            TwilioProgrammableMessagingError::ErrorCode30891 => Some(r#"The campaign submission has been reviewed and it was rejected because of unverifiable website."#),
            TwilioProgrammableMessagingError::ErrorCode30455 => Some(r#"The Toll-Free phone number verification submission has been reviewed and it was rejected. Effective Jan 31st, 2024, messaging traffic on this Toll-Free number is blocked until it’s verified.

Verification is one component of ensuring A2P SMS is used by businesses in a legal and compliant manner. The review process looks at the business sending messaging (not the software powering it), the use case, sample content, the web presence, the opt-in/consent collected and associated website terms and privacy policies."#),
            TwilioProgrammableMessagingError::ErrorCode63043 => Some(r#"The link returned a status code of 403 Forbidden, making it impossible to retrieve the file."#),
            TwilioProgrammableMessagingError::ErrorCode36003 => Some(r#"The file size is greater than the allowed 25 MB"#),
            TwilioProgrammableMessagingError::ErrorCode35134 => Some(r#"`Messages` array contains duplicate `To` numbers. Please check the list for duplicates"#),
            TwilioProgrammableMessagingError::ErrorCode36007 => Some(r#"The Broadcast cannot have a file uploaded to it because it is in a state other than PENDING_UPLOAD."#),
            TwilioProgrammableMessagingError::ErrorCode30796 => Some(r#"Your Brand has received feedback from The Campaign Registry upon registration. This Brand Registration can not be used for campaign creation until the provided feedback has been addressed. In this particular case, either a non-public entity has been registered as a public for profit entity or the stock information provided during the registration process doesn’t match the public company records.
"#),
            TwilioProgrammableMessagingError::ErrorCode30470 => Some(r#"The Toll-Free phone number verification submission has been reviewed and it was rejected.Effective Jan 31st, 2024, messaging traffic on this Toll-Free number is blocked until it’s verified.

Verification is one component of ensuring A2P SMS is used by businesses in a legal and compliant manner. The review process looks at the business sending messaging (not the software powering it), the use case, sample content, the web presence, the opt-in/consent collected and associated website terms and privacy policies.
"#),
            TwilioProgrammableMessagingError::ErrorCode30476 => Some(r#"The Toll-Free phone number verification submission has been reviewed and it was rejected. Effective Jan 31st, 2024, messaging traffic on this Toll-Free number is blocked until it’s verified.

Verification is one component of ensuring A2P SMS is used by businesses in a legal and compliant manner. The review process looks at the business sending messaging (not the software powering it), the use case, sample content, the web presence, the opt-in/consent collected and associated website terms and privacy policies."#),
            TwilioProgrammableMessagingError::ErrorCode30990 => Some(r#"The campaign has been suspended. Messaging has been halted and number registration/deregistration is not available until suspension has been lifted"#),
            TwilioProgrammableMessagingError::ErrorCode30436 => Some(r#"The phone number SID is malformed or belongs to another account."#),
            TwilioProgrammableMessagingError::ErrorCode30452 => Some(r#"Twilio is unable to process your Toll Free Verification request because it could not be validated by Twilio and could not be submitted for verification review. 
"#),
            TwilioProgrammableMessagingError::ErrorCode30793 => Some(r#"There was an error while processing the provided customer and a2p information."#),
            TwilioProgrammableMessagingError::ErrorCode30468 => Some(r#"The Toll-Free phone number verification submission has been reviewed and it was rejected. Effective Jan 31st, 2024, messaging traffic on this Toll-Free number is blocked until it’s verified.

Verification is one component of ensuring A2P SMS is used by businesses in a legal and compliant manner. The review process looks at the business sending messaging (not the software powering it), the use case, sample content, the web presence, the opt-in/consent collected and associated website terms and privacy policies.
"#),
            TwilioProgrammableMessagingError::ErrorCode63052 => Some(r#"Twilio has detected that this WhatsApp message was sent using the currently EOL Legacy WhatsApp templates system. 

Changes are required to how you send templates over WhatsApp by April 1, 2025. Templates must be created and managed using the Content Template Builder or Content API. Templates must be sent using ContentSid and Content Variables. 

For more information please see this support article: https://help.twilio.com/articles/19816296822299


"#),
            TwilioProgrammableMessagingError::ErrorCode30748 => Some(r#"Brand can not be registered with provided phone number since duplicate threshold limit has been reached for it."#),
            TwilioProgrammableMessagingError::ErrorCode30992 => Some(r#"Connectivity partner migration process was cancelled."#),
            TwilioProgrammableMessagingError::ErrorCode30451 => Some(r#"Twilio is unable to process your Toll Free Verification request because it could not be validated by Twilio and could not be submitted for verification review. 
"#),
            TwilioProgrammableMessagingError::ErrorCode30457 => Some(r#"The Toll-Free phone number verification submission has been reviewed and it was rejected. Effective Jan 31st, 2024, messaging traffic on this Toll-Free number is blocked until it’s verified."#),
            TwilioProgrammableMessagingError::ErrorCode30467 => Some(r#"The Toll-Free phone number verification submission has been reviewed and it was rejected.Effective Jan 31st, 2024, messaging traffic on this Toll-Free number is blocked until it’s verified.

Verification is one component of ensuring A2P SMS is used by businesses in a legal and compliant manner. The review process looks at the business sending messaging (not the software powering it), the use case, sample content, the web presence, the opt-in/consent collected and associated website terms and privacy policies."#),
            TwilioProgrammableMessagingError::ErrorCode30798 => Some(r#"Your Brand has received feedback from The Campaign Registration upon registration. This brand registration can not be used for campaign creation till the provided feedback has been addressed. In this particular case,  no IRS 501c tax-exempt status was found for the entity being registered."#),
            TwilioProgrammableMessagingError::ErrorCode30461 => Some(r#"The Toll-Free phone number verification submission has been reviewed and it was rejected. Effective Jan 31st, 2024, messaging traffic on this Toll-Free number is blocked until it’s verified.

Verification is one component of ensuring A2P SMS is used by businesses in a legal and compliant manner. The review process looks at the business sending messaging (not the software powering it), the use case, sample content, the web presence, the opt-in/consent collected and associated website terms and privacy policies."#),
            TwilioProgrammableMessagingError::ErrorCode30893 => Some(r#"The campaign submission has been reviewed and rejected due to inconsistencies between the sample message and the intended use-case of the campaign."#),
            TwilioProgrammableMessagingError::ErrorCode30469 => Some(r#"The Toll-Free phone number verification submission has been reviewed and it was rejected.Effective Jan 31st, 2024, messaging traffic on this Toll-Free number is blocked until it’s verified.

Verification is one component of ensuring A2P SMS is used by businesses in a legal and compliant manner. The review process looks at the business sending messaging (not the software powering it), the use case, sample content, the web presence, the opt-in/consent collected and associated website terms and privacy policies."#),
            TwilioProgrammableMessagingError::ErrorCode30753 => Some(r#"You have used an unsupported email address for brand registration"#),
            TwilioProgrammableMessagingError::ErrorCode30448 => Some(r#"The Toll-Free phone number verification submission has been reviewed and it was rejected. Effective Jan 31st, 2024, messaging traffic on this Toll-Free number is blocked until it’s verified.

Verification is one component of ensuring A2P SMS is used by businesses in a legal and compliant manner. The review process looks at the business sending messaging (not the software powering it), the use case, sample content, the web presence, the opt-in/consent collected and associated website terms and privacy policies."#),
            TwilioProgrammableMessagingError::ErrorCode30449 => Some(r#"The Toll-Free phone number verification submission has been reviewed and it was rejected. Effective Jan 31st, 2024, messaging traffic on this Toll-Free number is blocked until it’s verified.

Verification is one component of ensuring A2P SMS is used by businesses in a legal and compliant manner. The review process looks at the business sending messaging (not the software powering it), the use case, sample content, the web presence, the opt-in/consent collected and associated website terms and privacy policies."#),
            TwilioProgrammableMessagingError::ErrorCode30790 => Some(r#"There was an internal system error when processing brand registration request."#),
            TwilioProgrammableMessagingError::ErrorCode30459 => Some(r#"The Toll-Free phone number verification submission has been reviewed and it was rejected. Effective Jan 31st, 2024, messaging traffic on this Toll-Free number is blocked until it’s verified."#),
            TwilioProgrammableMessagingError::ErrorCode30795 => Some(r#"Your Brand has received feedback from The Campaign Registry upon registration. This Brand Registration can not be used for campaign creation until the provided feedback has been addressed. In this particular case, there is a data mismatch related to the Tax ID that was provided during the registration and its associated properties such as business legal name."#),
            TwilioProgrammableMessagingError::ErrorCode30437 => Some(r#"The rejected toll-free verification time window has reached the 7 day expiration. An edit is accepted, but the toll-free number remains unverified and the submission is no longer prioritized, so time to process is similar to a new submission. Effective Nov 8, 2023, messaging traffic on this Toll-Free number is blocked until it’s verified."#),
            TwilioProgrammableMessagingError::ErrorCode30887 => Some(r#"The campaign submission has been reviewed and it was rejected because of provided Opt-out information."#),
            TwilioProgrammableMessagingError::ErrorCode30880 => Some(r#"The campaign submission has been reviewed and it was rejected because of an unknown reason."#),
            TwilioProgrammableMessagingError::ErrorCode30899 => Some(r#"The campaign registration failed because of carrier rejection(s)."#),
            TwilioProgrammableMessagingError::ErrorCode30905 => Some(r#"Unable to attach the Messaging Service to the Campaign as the Campaign is still under review and has not been accepted by Twilio's DCA."#),
            TwilioProgrammableMessagingError::ErrorCode36006 => Some(r#"The Broadcast is in a non-cancelable state. The broadcast cannot be canceled when in any of the following statuses: EXECUTION_COMPLETED, EXECUTION_FAILURE, CANCELED."#),
            TwilioProgrammableMessagingError::ErrorCode30479 => Some(r#"The Toll-Free phone number verification submission has been reviewed and it was rejected.Effective Jan 31st, 2024, messaging traffic on this Toll-Free number is blocked until it’s verified.

Verification is one component of ensuring A2P SMS is used by businesses in a legal and compliant manner. The review process looks at the business sending messaging (not the software powering it), the use case, sample content, the web presence, the opt-in/consent collected and associated website terms and privacy policies.
"#),
            TwilioProgrammableMessagingError::ErrorCode30886 => Some(r#"The campaign submission has been reviewed and it was rejected because of invalid campaign description."#),
            TwilioProgrammableMessagingError::ErrorCode30882 => Some(r#"The campaign submission has been reviewed and it was rejected due to Terms and Conditions issues."#),
            TwilioProgrammableMessagingError::ErrorCode21733 => Some(r#"Default messaging service not found"#),
            TwilioProgrammableMessagingError::ErrorCode30464 => Some(r#"The Toll-Free phone number verification submission has been reviewed and it was rejected.Effective Jan 31st, 2024, messaging traffic on this Toll-Free number is blocked until it’s verified.

Verification is one component of ensuring A2P SMS is used by businesses in a legal and compliant manner. The review process looks at the business sending messaging (not the software powering it), the use case, sample content, the web presence, the opt-in/consent collected and associated website terms and privacy policies."#),
            TwilioProgrammableMessagingError::ErrorCode30752 => Some(r#"You have used either an invalid or an expired OTP for brand verification."#),
            TwilioProgrammableMessagingError::ErrorCode35132 => Some(r#"Provided Attributes are not in the expected JSON format"#),
            TwilioProgrammableMessagingError::ErrorCode21268 => Some(r#"Twilio doesn't allow sending SMS messages to premium rate or information service numbers. 
"#),
            TwilioProgrammableMessagingError::ErrorCode30472 => Some(r#"The Toll-Free phone number verification submission has been reviewed and it was rejected.Effective Jan 31st, 2024, messaging traffic on this Toll-Free number is blocked until it’s verified.

Verification is one component of ensuring A2P SMS is used by businesses in a legal and compliant manner. The review process looks at the business sending messaging (not the software powering it), the use case, sample content, the web presence, the opt-in/consent collected and associated website terms and privacy policies.
"#),
            TwilioProgrammableMessagingError::ErrorCode30135 => None,
            TwilioProgrammableMessagingError::ErrorCode30898 => Some(r#"The campaign submission has been reviewed and it was rejected because of repeated use of same for multiple brands."#),
            TwilioProgrammableMessagingError::ErrorCode21265 => Some(r#"You attempted to send an SMS, but the `To` phone number you supplied was a [Short Code](https://www.twilio.com/docs/glossary/what-is-a-short-code). Twilio numbers cannot send messages to short codes at this time whether they are Twilio short codes or external short codes."#),
            TwilioProgrammableMessagingError::ErrorCode30904 => Some(r#"Messaging Service could not be associated with the Campaign because the specific Campaign was not found in Twilio's DCA records."#),
            TwilioProgrammableMessagingError::ErrorCode30791 => Some(r#"There was a temporary system error when processing brand registration request.

"#),
            TwilioProgrammableMessagingError::ErrorCode30794 => Some(r#"There was a general system error when processing your registration request."#),
            TwilioProgrammableMessagingError::ErrorCode30889 => Some(r#"The campaign submission has been reviewed and it was rejected because Embedded Phone Number selection."#),
            TwilioProgrammableMessagingError::ErrorCode30478 => Some(r#"The Toll-Free phone number verification submission has been reviewed and it was rejected. Effective Jan 31st, 2024, messaging traffic on this Toll-Free number is blocked until it’s verified.

Verification is one component of ensuring A2P SMS is used by businesses in a legal and compliant manner. The review process looks at the business sending messaging (not the software powering it), the use case, sample content, the web presence, the opt-in/consent collected and associated website terms and privacy policies.
"#),
            TwilioProgrammableMessagingError::ErrorCode63040 => Some(r#"This message matches a template that was rejected by WhatsApp or failed to be submitted. For the most updated guidelines, please view WhatsApp’s documentation: https://developers.facebook.com/docs/whatsapp/message-templates/guidelines."#),
            TwilioProgrammableMessagingError::ErrorCode30909 => Some(r#"The campaign submission has been reviewed and rejected due to issues verifying the Call to Action (CTA) provided for the campaign."#),
            TwilioProgrammableMessagingError::ErrorCode30443 => Some(r#"The Toll-Free phone number verification submission has been reviewed and it was rejected. Effective Jan 31st, 2024, messaging traffic on this Toll-Free number is blocked until it’s verified.

Verification is one component of ensuring A2P SMS is used by businesses in a legal and compliant manner. The review process looks at the business sending messaging (not the software powering it), the use case, sample content, the web presence, the opt-in/consent collected and associated website terms and privacy policies."#),
            TwilioProgrammableMessagingError::ErrorCode30462 => Some(r#"The Toll-Free phone number verification submission has been reviewed and it was rejected. Effective Jan 31st, 2024, messaging traffic on this Toll-Free number is blocked until it’s verified.

Verification is one component of ensuring A2P SMS is used by businesses in a legal and compliant manner. The review process looks at the business sending messaging (not the software powering it), the use case, sample content, the web presence, the opt-in/consent collected and associated website terms and privacy policies."#),
            TwilioProgrammableMessagingError::ErrorCode63024 => Some(r#"For WhatsApp, the message is undeliverable due to the recipient being an invalid user on WhatsApp or other Meta restrictions for WhatsApp."#),
            TwilioProgrammableMessagingError::ErrorCode30136 => None,
            TwilioProgrammableMessagingError::ErrorCode30703 => Some(r#"Brand registration failed due to duplicate record detection. Following attributes are expected to be unique for each brand and can fail with this duplication check.

* Mobile phone number
* Business registration identifier
* Business legal name
* Authorised representative's phone number
* Email address
* Website
* Stock symbol (applicable only for public entities)



"#),
            TwilioProgrammableMessagingError::ErrorCode30799 => Some(r#"Your Brand has received feedback from The Campaign Registration upon registration. This brand registration can not be used for campaign creation till the provided feedback has been addressed. In this case, it wasn’t possible to determine the exact cause of the failure. Please check the possible causes section to identify if the provided details for brand registration don’t fall into any of the mentioned categories."#),
            TwilioProgrammableMessagingError::ErrorCode11206 => Some(r#"There was an error speaking HTTP to the target URL.
"#),
            TwilioProgrammableMessagingError::ErrorCode63042 => Some(r#"WhatsApp has reported that the template has been **permanently disabled** due to recurring negative feedback from customers. Message templates with this status cannot be sent to customers. For the most updated guidelines, please view WhatsApp’s documentation: https://developers.facebook.com/docs/whatsapp/message-templates/guidelines/#template-pausing.
"#),
            TwilioProgrammableMessagingError::ErrorCode30712 => Some(r#"Your account has reached maximum number allowed brand registrations."#),
            TwilioProgrammableMessagingError::ErrorCode30466 => Some(r#"The Toll-Free phone number verification submission has been reviewed and it was rejected.Effective Jan 31st, 2024, messaging traffic on this Toll-Free number is blocked until it’s verified.

Verification is one component of ensuring A2P SMS is used by businesses in a legal and compliant manner. The review process looks at the business sending messaging (not the software powering it), the use case, sample content, the web presence, the opt-in/consent collected and associated website terms and privacy policies."#),
            TwilioProgrammableMessagingError::ErrorCode21661 => Some(r#"Phone numbers must be SMS-capable. Visit the [phone numbers page](https://www.twilio.com/console/phone-numbers/incoming) of your account portal to view a list of phone numbers that you own and to determine whether they are SMS-capable.
"#),
            TwilioProgrammableMessagingError::ErrorCode30756 => Some(r#" Obfuscation check failed for brand registration inputs."#),
            TwilioProgrammableMessagingError::ErrorCode30747 => Some(r#"Brand can not be registered with provided address since duplicate threshold limit has been reached for it."#),
            TwilioProgrammableMessagingError::ErrorCode30881 => Some(r#"The campaign submission has been reviewed and it was rejected because invalid brand support email."#),
            TwilioProgrammableMessagingError::ErrorCode30615 => Some(r#"Message couldn’t be delivered to the recipients number you are trying to reach at this time due to local regulatory restrictions."#),
            TwilioProgrammableMessagingError::ErrorCode30458 => Some(r#"The Toll-Free phone number verification submission has been reviewed and it was rejected. Effective Jan 31st, 2024, messaging traffic on this Toll-Free number is blocked until it’s verified.

Verification is one component of ensuring A2P SMS is used by businesses in a legal and compliant manner. The review process looks at the business sending messaging (not the software powering it), the use case, sample content, the web presence, the opt-in/consent collected and associated website terms and privacy policies."#),
            TwilioProgrammableMessagingError::ErrorCode30755 => Some(r#"You have used a country code which is not yet support for US A2P brand registrations."#),
            TwilioProgrammableMessagingError::ErrorCode36002 => Some(r#"The limit of the number of Broadcast requests for this account has been reached"#),
            TwilioProgrammableMessagingError::ErrorCode63044 => Some(r#"The link returned a status code of 404 Not Found, making it impossible to retrieve the file."#),
            TwilioProgrammableMessagingError::ErrorCode30792 => Some(r#"There was a general system error when processing brand registration request."#),
            TwilioProgrammableMessagingError::ErrorCode30900 => Some(r#"The campaign use case provided as part of registration request is ineligible for registration."#),
            TwilioProgrammableMessagingError::ErrorCode63049 => Some(r#"Meta may limit the number of marketing template messages a person receives from any business in a given period of time, starting with a small number of conversations that are less likely to be read. 

WhatsApp has found that per-user marketing template limits maximize message engagement and improve the user experience, measured through improvements in user read rates and sentiment. This limit helps WhatsApp users find business messaging more valuable and feel less like they receive too many business messages.

"#),
            TwilioProgrammableMessagingError::ErrorCode30477 => Some(r#"The Toll-Free phone number verification submission has been reviewed and it was rejected.Effective Jan 31st, 2024, messaging traffic on this Toll-Free number is blocked until it’s verified.

Verification is one component of ensuring A2P SMS is used by businesses in a legal and compliant manner. The review process looks at the business sending messaging (not the software powering it), the use case, sample content, the web presence, the opt-in/consent collected and associated website terms and privacy policies.
"#),
            TwilioProgrammableMessagingError::ErrorCode30630 => Some(r#"The person you are trying to message has opted out of receiving messages from your Twilio phone number, Channels sender, or Messaging Service.

You have attempted to message a 'To' number that has replied with "STOP" to one of your previous messages. You will not be able to send to the phone number specified in the 'To' parameter until the subscriber identified by the phone number has responded with "START".

Please see this [FAQ](https://help.twilio.com/articles/223134027-Twilio-support-for-opt-out-keywords-SMS-STOP-filtering) for more information about how Twilio handles opt-out and opt-in."#)
        }
    }
}
