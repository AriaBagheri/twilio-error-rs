// AUTO-GENERATED. DO NOT MODIFY. YOUR CHANGES WILL BE OVER-WRITTEN
use crate::products::TwilioPhoneNumbersError;
use standard_error::traits::StandardErrorDocsTrait;

impl StandardErrorDocsTrait for TwilioPhoneNumbersError {
    fn docs(&self) -> &'static str {
        match self {
            TwilioPhoneNumbersError::ErrorCode22225 => r#"## ERROR - 22225

### From Bundle to Replace Items does not exist

 The FromBundleSid specified does not exist.

#### Possible Causes
The FromBundleSid specified does not exist.

#### Possible Solutions
Please review the FromBundleSid specified to ensure it is correctly referencing the account's BundleSid and not another account."#,
            TwilioPhoneNumbersError::ErrorCode22152 => r#"## ERROR - 22152

### Port In Error - Phone number is inactive or disconnected

 The phone number is inactive or disconnected on the losing carrier and so cannot be ported. 

#### Possible Causes
The phone number is inactive or disconnected on the losing carrier and so cannot be ported. 

#### Possible Solutions
Contact the phone number's current carrier to reactivate the phone number, so that you can port it to Twilio."#,
            TwilioPhoneNumbersError::ErrorCode22155 => r#"## ERROR - 22155

### Port In Error - Invalid Pin

 The PIN submitted in the port in request for this phone number does not match the losing carrier's information. 

#### Possible Causes
An invalid or no PIN for this phone number was submitted as part of the port in request.

#### Possible Solutions
Update PIN in the port in request to one that matches the losing carrier's information. 

PINs are unique to a single phone numbers, so you should not need a new request to change the PIN for just this phone number. Instead update the port in request with the correct PIN."#,
            TwilioPhoneNumbersError::ErrorCode22205 => r#"## ERROR - 22205

### Attempting to assign invalid object_sid to Bundle

 You are attempting to add an invalid object_sid to the Bundle.

#### Possible Causes
The object_sid may be malformed or belonging to a different account.

#### Possible Solutions
Please refer to the Supporting Documents LIST or End-User LIST resources to find the correct Supporting Document SID or End-User SID to assign as an item to a Bundle."#,
            TwilioPhoneNumbersError::ErrorCode22212 => r#"## ERROR - 22212

### Invalid End-User Type in request

 An invalid End-User Type is included in the request to filter the Regulations LIST resource.

#### Possible Causes
The End-User Type is malformed or is not an acceptable value.

#### Possible Solutions
Ensure that the End-User Type value is spelled correctly or is present in the End-User Type LIST resource."#,
            TwilioPhoneNumbersError::ErrorCode22112 => r#"## Error - 22112

#### Unable to Update Hosted Number Order Status

The status provided in a request to update Hosted Number Order instance cannot be applied to the current Hosted Number Order state.

#### Possible Causes

* Trying to update a Hosted Number Order that has been completed or failed.

* Trying to update the status of a Hosted Number Order to a status that cannot be user-initiated.

#### Possible Solutions

* If a Hosted Number Order is in a terminal state (failed or completed), create a new instance.

* If a Hosted Number Order is in "action-required" state, reach out to support to remediate."#,
            TwilioPhoneNumbersError::ErrorCode18056 => r#"## ERROR - 18056

### Missing Work Email for Authorized Representative

We require the work email of the Authorized Representative. Please resubmit with a work email of the Authorized Representative. Missing Work Email for Authorized Representative.

#### Possible Causes
The work email of the Authorized Representative was not provided.

#### Possible Solutions

Please ensure you provide the work email of the Authorized Representative to proceed."#,
            TwilioPhoneNumbersError::ErrorCode22213 => r#"## ERROR - 22213

### Invalid Number Type in request

 An invalid End-User Type is included in the request to filter the Regulations LIST resource.

#### Possible Causes
The Number Type is malformed or is not an acceptable value.

#### Possible Solutions
Ensure that the Number Type value is spelled correctly or is an acceptable Number Type in the Regulations LIST resource"#,
            TwilioPhoneNumbersError::ErrorCode22210 => r#"## ERROR - 22210

### Cannot create a Supporting Document with no Type

 A Supporting Document instance was requested to be created with no `Type` specified

#### Possible Causes
Either the Type string was malformed or the Supporting Document Type name does not exist.

#### Possible Solutions
Refer to the Supporting Document Types LIST resource to find the correct name type for your request."#,
            TwilioPhoneNumbersError::ErrorCode18060 => r#"## ERROR - 18060

### Commercial registration copy not submitted

Excerpt from the commercial register is missing, which is a required supporting document. Please re-submit after uploading the excerpt from the commercial register Excerpt from the commercial register is missing, which is a required supporting document.

#### Possible Causes
Missed uploading or upload failure

#### Possible Solutions
Please re-submit after uploading the excerpt from the commercial register."#,
            TwilioPhoneNumbersError::ErrorCode18605 => r#"## ERROR - 18605

### Unable to verify Authorized representative #2. 

 Unable to verify Authorized representative #2

#### Possible Causes
Unable to verify Authorized representative #2’s identity or their affiliation with the business.

#### Possible Solutions
Please feel free to edit the Business Profile & update the missing information. You can also reach out to our support team at trusthub-verify@twilio.com for any additional clarifications."#,
            TwilioPhoneNumbersError::ErrorCode18609 => r#"## ERROR - 18609

### Ineligible Business Registration Authority

 The provided business registration authority is not an EIN (Employer Identification Number) or DUN (Data Universal Numbering System) number.

#### Possible Causes
The submitted business registration authority does not meet the eligibility requirements for the US Voice Integrity/CNAM trust product.

#### Possible Solutions
Please resubmit your registration using either an EIN or a DUN number."#,
            TwilioPhoneNumbersError::ErrorCode22171 => r#"## ERROR - 22171

### Port In Error - Missing required fields

 Missing required fields, please review which fields are required for each country.

#### Possible Causes
There are missing required fields for the country your phone numbers are related to.

#### Possible Solutions
Check what fields are required for each country and update those fields with valid values."#,
            TwilioPhoneNumbersError::ErrorCode22122 => r#"## Error - 22122

#### Invalid Authorization Document Status

Status must be one of "opened" or "signing.""#,
            TwilioPhoneNumbersError::ErrorCode21628 => r#"## Error - 21628

### Address Validation Error

The address you have provided cannot be validated. 

#### Possible Causes
- Address misspelling
- Address cannot be found in third-party data source 

#### Possible Solutions
- Ensure the address is spelled correctly 
- Contact Twilio Support"#,
            TwilioPhoneNumbersError::ErrorCode22118 => r#"## Error - 22118

#### Invalid Verification Document SID

The Verification Document SID specified is not a valid identity document identifier. 

#### Possible Causes

* The Verification Document SID is not a 34-character string that is prefixed with "RI".

* The identity document represented by the Verification DocumentSID is not of type "phone-bill".

* The identity document represented by the Verification Document SID does not exist anymore or does not belong to your account.

#### Possible Solutions

Please specify a Verification Document SID you’ve created in your [Account Dashboard](https://www.twilio.com/console/phone-numbers/identities)."#,
            TwilioPhoneNumbersError::ErrorCode22217 => r#"## ERROR - 22217

### Missing Supporting Document field

A field is missing for the supporting document. A Supporting Document field is missing from the regulatory bundle.

#### Possible Causes
The Regulation requires a Supporting Document field which is missing from your Regulatory Bundle.

#### Possible Solutions
Please refer to the Regulations API resource to ensure that all required fields are present in the Supporting Document."#,
            TwilioPhoneNumbersError::ErrorCode18008 => r#"## ERROR - 18008

### Name mismatch (Proof of Identity)

The name on the Proof of Identity document does not match what's submitted in the first name / last name fields. Please ensure the document and field information match. The individual’s name on the Proof of Identity document does not match the information entered in the Bundle.

#### Possible Causes
The name on the Proof of Identity document does not match what's submitted in the first name / last name fields.

#### Possible Solutions
Please ensure the names match exactly and that all the individual's last names are provided. Please amend firstly the name in the Business & legal representative information / Individual information section of the bundle, and then in the Bundle fields under the uploaded supporting document."#,
            TwilioPhoneNumbersError::ErrorCode22133 => r#"## ERROR - 22133

### Not Portable API - Manual porting available

 

#### Possible Causes
The phone number cannot be ported using the Twilio porting API. 

#### Possible Solutions
If the number is an US phone number, please open a porting request through the Console. See this [support page](https://support.twilio.com/hc/en-us/articles/223179348-Porting-a-Phone-Number-to-Twilio) for more information on US porting.

Otherwise create a support ticket to port in a phone number from another country. See this [support page](https://support.twilio.com/hc/en-us/articles/115000781088-International-Porting-Process) for more information on international porting."#,
            TwilioPhoneNumbersError::ErrorCode22130 => r#"## ERROR - 22130

### Not Portable - Unsupported

 

#### Possible Causes
Phone number is in a country or rate center that is not supported by Twilio

#### Possible Solutions
Not Action Required"#,
            TwilioPhoneNumbersError::ErrorCode22114 => r#"## Error - 22114

#### Unable to Verify Code

A verification code was provided but the Hosted Number Order instance is not in a valid state for handling caller ID verification.

#### Possible Solutions

* Update the status of the Hosted Number Order instance to "pending-verification".

* Remove VerificationCode from the request body if verification call has already been completed."#,
            TwilioPhoneNumbersError::ErrorCode22111 => r#"## Error - 22111

#### Invalid Hosted Number Order Status

The status you specified is not a valid Hosted Number Order status.

#### Possible Causes

* Attempting to filter a list of Hosted Number Orders on a status that does not exist.

* Attempting to update a Hosted Number Order instance with a status that cannot be applied to the current Hosted Number Order state."#,
            TwilioPhoneNumbersError::ErrorCode22151 => r#"## ERROR - 22151

### Port In Error - Number with carrier restrictions

 Phone number has carrier restrictions with the losing carrier that must be resolved before porting the phone number.

#### Possible Causes
A carrier may place a restriction on a phone number to prevent porting for many reasons, common ones include service freezes to prevent unauthorized transfers or billing problems.

#### Possible Solutions
Contact the phone number's current carrier and resolve any outstanding issues that is preventing them from approving the port."#,
            TwilioPhoneNumbersError::ErrorCode21650 => r#"## Error - 21650

### Phone Number Requires a Verified Identity Document

To purchase this number you must have a verified identity document on your account. 

#### Possible Causes
- There is no verified identity document on your account or subaccount. Due to local regulations, some phone numbers require a verified identity document associated with your Twilio account.

#### Possible Solutions
- In the Phone Numbers > Documents section in Console, provide the name of the individual or business registering the phone numbers and a scanned copy of supporting identification documents.
- Ensure that the verification status of the identity document record is “Passed”. If “Pending”, wait 48 business hours for Twilio to verify the document. If “Failed”, please refer to the failure reason to address the issue. 
"#,
            TwilioPhoneNumbersError::ErrorCode18036 => r#"## ERROR - 18036

### One or more of the required information is missing.

 One or more of the following required information is missing: Birth Date, Birth Place, Tax Number, Document Number, Issue Date.

#### Possible Causes
You may have missed to input the required information.

#### Possible Solutions
Please fill out the required fields in the form."#,
            TwilioPhoneNumbersError::ErrorCode18608 => r#"## ERROR - 18608

### Customer Profile or Trust Product is already copied

 You already copied the customer profile or trust product

#### Possible Causes
You already copied.

#### Possible Solutions
Since the customer profile or trust product is already copied, please move forward without any additional action"#,
            TwilioPhoneNumbersError::ErrorCode18053 => r#"## ERROR - 18053

### Unable to verify association between business name and website

The business name does not match the website SSL certificate, or the website URL, or the association cannot be verified. The association between business name and website cannot be verified.

#### Possible Causes
The business name does not match the website SSL certificate, or the website URL.

#### Possible Solutions
Please reach out to us via email numbers-regulatory-review@twilio.com and share the details of how the company and website are connected."#,
            TwilioPhoneNumbersError::ErrorCode22153 => r#"## ERROR - 22153

### Port In Error - Invalid end user name

 The name of the Authorized Representative submitted in the port in request does not the losing carrier's information. 

#### Possible Causes
The name of the Authorized Representative submitted in the port in request does not the losing carrier's information. 

#### Possible Solutions
Update the port in request to include an authorized representative that is on record with the losing carrier for this phone number.

Note if this is the only phone number in a port in request that got rejected you may need to start a new port in request with this phone number so that you can correct the authorized representative name as it may be different from the other phone numbers in this request. "#,
            TwilioPhoneNumbersError::ErrorCode18051 => r#"## ERROR - 18051

### Issue with the inputs you provided

There is an issue with the inputs you provided, more details will be sent to the Authorized Representative's email. There is an issue with the inputs you provided, please check the email sent to the Authorized Representative for the details.

#### Possible Causes
One of the inputs (information or documents) you provided has issues.

#### Possible Solutions
Please check the Authorized Representative's email for the details on the issue, please resubmit the bundle after rectifying the issue mentioned in the email."#,
            TwilioPhoneNumbersError::ErrorCode18604 => r#"## ERROR - 18604

### Unable to verify Authorized representative #1

 Unable to verify Authorized representative #1

#### Possible Causes
Unable to verify Authorized representative #1’s identity or their affiliation with the business.

#### Possible Solutions
Please feel free to edit the Business Profile & update the missing information. You can also reach out to our support team at trusthub-verify@twilio.com for any additional clarifications."#,
            TwilioPhoneNumbersError::ErrorCode22401 => r#"## ERROR - 22401

### Phone Number Instance fields are not supported within region

 The fields you are attempting to update are not supported within the region specified.

#### Possible Causes
The feature is not supported within the region.

#### Possible Solutions
Please refer to documentation to understand which fields are supported."#,
            TwilioPhoneNumbersError::ErrorCode22200 => r#"## ERROR - 22200

### Invalid End-User Type or Number Type

 An invalid End-User Type or Number Type is included in the request to create a Bundle.

#### Possible Causes
The End-User Type or Number Type is malformed or is not an acceptable value.

#### Possible Solutions
Ensure that the End-User Type value or Number Type value is spelled correctly or is an acceptable value from the End-User Type LIST resource or Number Type LIST resource."#,
            TwilioPhoneNumbersError::ErrorCode18606 => r#"## ERROR - 18606

### The Email domain doesn't match the website domain

 The Email domain doesn't match the website domain

#### Possible Causes
The domain of the email address you provided doesn't match the website domain.

#### Possible Solutions
Please feel free to edit the Business Profile & update the missing information. You can also reach out to our support team at trusthub-verify@twilio.com for any additional clarifications."#,
            TwilioPhoneNumbersError::ErrorCode22102 => r#"## ERROR - 22102

### Invalid Phone Number

 The phone number provided is not a valid number.

#### Possible Causes
* The phone number provided is formatted incorrectly.

* The phone number provided includes a non-existent country code, area code or exchange.

#### Possible Solutions
Please specify a valid phone number in [E164 format](http://en.wikipedia.org/wiki/E.164) (i.e. "+1 format")."#,
            TwilioPhoneNumbersError::ErrorCode18607 => r#"## ERROR - 18607

### Customer Profile or Trust Product is not eligible to be copied

 You can not copy the customer profile or trust product

#### Possible Causes
The customer profile is not eligible to be copied

#### Possible Solutions
Please move forward without copying the customer profile or trust product"#,
            TwilioPhoneNumbersError::ErrorCode18005 => r#"## ERROR - 18005

### Missing information

There is missing information. Please fill out the required form fields. One or more of the required criteria is missing. 

#### Possible Causes
You may have missed sharing the required information.

#### Possible Solutions
Please fill out the required form fields and resubmit the bundle."#,
            TwilioPhoneNumbersError::ErrorCode18017 => r#"## ERROR - 18017

### Address not found

The Proof of Identity document does not show a visible address. Please upload a Supporting Document with a visible address. The supporting document does not provide an address. 

#### Possible Causes
The identity document that you submitted for address document does not have the address

#### Possible Solutions
Please upload a Supporting Document that shows an address."#,
            TwilioPhoneNumbersError::ErrorCode21648 => r#"## ERROR - 21648

### Regulatory Bundle cannot be deleted due to active number assignment

 The Regulatory Bundle cannot be deleted due to an active number assignment.a

#### Possible Causes
The Regulatory Bundle cannot be deleted due to an active number assignment.a

#### Possible Solutions
Re-assign the current number(s) to another twilio-approved Regulatory Bundle."#,
            TwilioPhoneNumbersError::ErrorCode18052 => r#"## ERROR - 18052

### Under age Individual

  We do not permit underage individuals on our platform. Please resubmit for an individual with acceptable age (over the age of 13 in the US and UK or 16, in the EEA). We do not permit underage individuals on our platform.

#### Possible Causes
The Authorized Representative's age is under 18 years, which is considered underage to use Twilio's products.

#### Possible Solutions
Please resubmit for an individual with acceptable age. Please make sure to take necessary steps to remove underaged person's personal information from the Bundle."#,
            TwilioPhoneNumbersError::ErrorCode22402 => r#"## ERROR - 22402

### Phone Number Operation not permitted within Region

 Phone Number update operation is not permitted within the Region.

#### Possible Causes
Certain Phone Number operations are not permitted within this Region such as restore, transfer, emergency services, and release.

#### Possible Solutions
Please refer to documentation to understand which operations are permitted within each region."#,
            TwilioPhoneNumbersError::ErrorCode21630 => r#"## Error - 21630

### Cannot mutate Address that is linked to a verified Document.
"#,
            TwilioPhoneNumbersError::ErrorCode21449 => r#"## Error - 21449

### Number already can be used for outgoing calls and messages

The number you attempted to add as an Outgoing Caller ID to your account already can be used for outgoing calls and messages.
"#,
            TwilioPhoneNumbersError::ErrorCode18055 => r#"## ERROR - 18055

### Invalid email

The email you provided is not valid. Please resubmit the bundle with a valid email address. The email you provided is not valid.

#### Possible Causes
The email format is incorrect or contains errors.

#### Possible Solutions
Please check and resubmit the bundle with a valid email address and continue."#,
            TwilioPhoneNumbersError::ErrorCode22215 => r#"## ERROR - 22215

### Missing End-User field

User information is missing. An End-User field is missing from the Regulatory Bundle

#### Possible Causes
The Regulation requires an End-User field which is missing from your Regulatory Bundle.

#### Possible Solutions
Please refer to the Regulations API to ensure that all required fields are present in the End-User."#,
            TwilioPhoneNumbersError::ErrorCode18054 => r#"## ERROR - 18054

### Invalid phone number

The phone number you provided is not valid. Please review and provide a valid phone number in E164 format. The phone number you provided is not valid.

#### Possible Causes
The phone number format is incorrect or or contains errors.

#### Possible Solutions
Please review and provide a valid phone number in E164 format."#,
            TwilioPhoneNumbersError::ErrorCode18603 => r#"## ERROR - 18603

### The address could not be verified

 The address you provided could not be verified.

#### Possible Causes
The address is not found in an authoritative address database.

#### Possible Solutions
Please feel free to edit the Business Profile & update the missing information. You can also reach out to our support team at trusthub-verify@twilio.com for any additional clarifications."#,
            TwilioPhoneNumbersError::ErrorCode18058 => r#"## ERROR - 18058

### Missing Tax ID

Authorized Representative's Tax ID is missing. Please amend the Tax ID Number field in the Business & Legal Representative Information section to the Authorized Representative´s personal tax ID. An Authorized Representative's Tax ID is missing.

#### Possible Causes
An Authorized Representative's Tax ID is missing.

#### Possible Solutions
Please amend the Tax ID Number field in the Business & Legal Representative Information section to the Authorized Representative´s personal tax ID."#,
            TwilioPhoneNumbersError::ErrorCode18012 => r#"## ERROR - 18012

### Require domestic address.

The phone number type you selected requires a valid domestic address on the document. Please resubmit a document with a valid address and ensure you've selected the desired phone number type. The phone number type you selected requires a valid domestic address.

#### Possible Causes
- You may have submitted an international address.
- Your address is not a valid address.

#### Possible Solutions
Please resubmit with a valid address or create a new bundle selecting the appropriate phone number type."#,
            TwilioPhoneNumbersError::ErrorCode22403 => r#"## ERROR - 22403

### Phone Number Operation not permitted within Region

 Phone Number update operation is not permitted within the Region.

#### Possible Causes
Certain Phone Number operations are not permitted within this Region emergency services

#### Possible Solutions
Please refer to documentation to understand which operations are permitted within each region."#,
            TwilioPhoneNumbersError::ErrorCode22400 => r#"## ERROR - 22400

### Phone Number linked to Active Route Configuration

 Phone Number linked to Active Route Configuration.

#### Possible Causes
Phone Number linked to Active Route Configuration.

#### Possible Solutions
Please contact Twilio Support either via the [Console](https://console.twilio.com) or the [Help Center](https://support.twilio.com) to request assistance with deleting the Active Route Configuration for an inactive phone number reference."#,
            TwilioPhoneNumbersError::ErrorCode18061 => r#"## ERROR - 18061

### Invalid excerpt from the commercial register

We are unable to validate your company registration from the government database, please resubmit with a current and accurate excerpt from the commercial register. We are unable to validate your company registration from the government database, please resubmit with a current and accurate excerpt from the commercial register.

#### Possible Causes
- There is a mistake in the registration number, either on the document or in the bundle information
- Your registration may have expired
- Registration database might have a technical glitch

#### Possible Solutions
Please upload a business registration or excerpt from the commercial registry as a Supporting Document and ensure it matches the information provided in the Bundle."#,
            TwilioPhoneNumbersError::ErrorCode18024 => r#"## ERROR - 18024

### Failed to upload an unprocessable document

A document could not be uploaded either because it is corrupted or could not be parsed or processed. Please resubmit with a valid/uncorrupted file. A document could not be uploaded either because it is corrupted or could not be parsed or processed.

#### Possible Causes
The file type does not match the file content or the file corrupt/invalid.

#### Possible Solutions
Please resubmit with a valid/uncorrupted file."#,
            TwilioPhoneNumbersError::ErrorCode18602 => r#"## ERROR - 18602

### The Business ID you provided could not be verified

 The Business ID you provided could not be verified.

#### Possible Causes
You may have provided an inaccurate business ID or the Business ID is invalid / expired.

#### Possible Solutions
Please feel free to edit the Business Profile & update the missing information. You can also reach out to our support team at trusthub-verify@twilio.com for any additional clarifications.

Please be sure to use the legal company name and EIN number found in your Tax records."#,
            TwilioPhoneNumbersError::ErrorCode22224 => r#"## ERROR - 22224

### Regulatory Bundle cannot transfer Item Assignments

 The Regulatory Bundle cannot transfer the Item Assignments from the destination Bundle.

#### Possible Causes
The status is either not eligible as all Bundles need to be in a twilio-approved state or the destination Bundle is not a Copy of the source Bundle.

#### Possible Solutions
Ensure both Bundles are in the correct twilio-approved status and that the destination Bundle is a Copy of the source Bundle."#,
            TwilioPhoneNumbersError::ErrorCode22207 => r#"## ERROR - 22207

### Unable to parse attributes JSON

 The attributes JSON string provided was not valid or parsing otherwise failed.

#### Possible Causes
The JSON was invalid.

#### Possible Solutions
Check to see if the JSON is valid and resend the request."#,
            TwilioPhoneNumbersError::ErrorCode22230 => r#"## ERROR - 22230

### Regulatory Bundle cannot be copied due to a deleted Address

 The Regulatory Bundle requested cannot be copied due to a deleted Address.

#### Possible Causes
The Address linked to the Regulatory Bundle was deleted.

#### Possible Solutions
Create a new Address on your Account or Sub-Account (if you don’t already have one) and reach out to [Twilio Support](https://support.twilio.com/hc/en-us) to link it to the Regulatory Bundle that you wish to copy."#,
            TwilioPhoneNumbersError::ErrorCode22159 => r#"## ERROR - 22159

### Port In Error - Not Portable

 The losing carrier has rejected the port request for the number stating that the phone number cannot be ported.

#### Possible Causes
The most likely case here is that the phone number uses services on the losing carrier that prevent it from being ported. For example, in Japan both rental phone numbers & public phone numbers cannot be ported.

In rare cases, the number may have been incorrectly typed by Twilio's Portability API and the requested number is not actually portable.

#### Possible Solutions
Check with your current carrier to determine if you are using any services that would prevent the phone number from being ported to Twilio. If you can remove those services you may still be able to attempt a port. 

Otherwise, please reach out to Support at porting@twilio.com and they can help to troubleshoot the rejection further and find alternatives for you."#,
            TwilioPhoneNumbersError::ErrorCode22204 => r#"## ERROR - 22204

### Bundle status and properties cannot be updated in the same request

 Bundle status and properties cannot be updated in the same request

#### Possible Causes
Status update and a property update are in the same request

#### Possible Solutions
Remove the status update parameter or remove the property update(s) in the request."#,
            TwilioPhoneNumbersError::ErrorCode22113 => r#"## Error - 22113

#### Phone Verification Incorrect

You attempted to complete caller verification but the code you provided was incorrect. 

#### Possible Causes

* You entered the code incorrectly in the request

* The code you entered has expired

#### Possible Solutions

* Make another request to update Hosted Number Order instance to Status pending-verification to receive another verification code."#,
            TwilioPhoneNumbersError::ErrorCode22203 => r#"## ERROR - 22203

### Unable to parse bundle status

 Bundle Status provided by user didn't parse into the accepted enum valuesf

#### Possible Causes
Bundle status is not a valid option from the enum values

#### Possible Solutions
Refer to the Bundle REST API status section for which statuses you are able to choose from."#,
            TwilioPhoneNumbersError::ErrorCode18099 => r#"## ERROR - 18099

### Rejected test bundle

The information was rejected since API request marked it as a test. Resubmit with the IsTest parameter to False or omitted if it should be considered for approval. The information was rejected since API request marked it as a test.

#### Possible Causes
The API request to submit this bundle set the IsTest parameter to true.

#### Possible Solutions
Resubmit the bundle with the IsTest parameter to False or omitted if it should be considered for approval"#,
            TwilioPhoneNumbersError::ErrorCode18022 => r#"## ERROR - 18022

### Outdated Proof of Address document

The address document you shared is outdated. If you are an individual, please share the document issued in the last three months. If you are a business, please share the document issued within the last year. The address document you shared is outdated.

#### Possible Causes
The address document contains a date that is in the past.

#### Possible Solutions
If you are an individual, please share the document issued in the last three months. If you are a business, please share the document issued within the last year."#,
            TwilioPhoneNumbersError::ErrorCode22214 => r#"## ERROR - 22214

### Missing End User

User is missing. An End User is missing from the regulatory bundle

#### Possible Causes
The Regulation requires an end-user which is missing from the regulatory bundle you are attempting to evaluate.

#### Possible Solutions
Please refer to the Regulations API to ensure that the End User satisfying the requirement is in the regulatory bundle."#,
            TwilioPhoneNumbersError::ErrorCode22209 => r#"## ERROR - 22209

### Invalid status enum in Supporting Document update request

 An invalid status enum was sent in an update Supporting Document reques

#### Possible Causes
The status enum may be malformed or incorrect.

#### Possible Solutions
Please refer to the Supporting Documents REST Public API documentation for the update instance resource to pick from an allowed status enum list."#,
            TwilioPhoneNumbersError::ErrorCode22404 => r#"## ERROR - 22404

### Starter profile creation and updates are temporarily disabled

 Starter profiles cannot be created or updated at this time. Contact support for help.

#### Possible Causes
The profile you are trying to create/update is a Starter profile. See [New Requirements for A2P 10DLC Registrations](https://www.twilio.com/blog/new-requirements-for-a2p-10dlc-registrations) for more info.

#### Possible Solutions
Contact Twilio support."#,
            TwilioPhoneNumbersError::ErrorCode22119 => r#"## Error - 22119

#### Invalid Capabilities

The SMS Capability you specified for your Hosted Number Order was not valid.

#### Possible Solutions

Please explicitly specify SMS as the capability to host on Twilio's platform by setting SMS Capability to true."#,
            TwilioPhoneNumbersError::ErrorCode18038 => r#"## ERROR - 18038

### Information does not match the supporting document

 The information you shared does not match the supporting document.

#### Possible Causes
You may have attached the wrong document or shared inaccurate information.

#### Possible Solutions
Please amend the information and resubmit the bundle."#,
            TwilioPhoneNumbersError::ErrorCode22105 => r#"## Error - 22105

#### Invalid URL Format

You attempted to configure your hosted number, but the URL you specified was not valid.

#### Possible Causes

* Invalid URL provided for phone number configuration (SmsUrl, SmsFallbackUrl, VoiceUrl, VoiceFallbackUrl). 

* Invalid URL provided for status callback configuration (StatusCallbackUrl).

#### Possible Solutions

Make sure you submit a fully qualified URL including:

*   protocol (http:// or https://)

*   hostname

*   file path

*   properly url-encoded query parameters

Twilio must be able to reach this URL over the public Internet."#,
            TwilioPhoneNumbersError::ErrorCode22117 => r#"## Error - 22117

#### Invalid Extension

The extension provided for the verification call contains characters that aren't allowed. 

#### Possible Solutions

* Confirm that your Extension is a non-empty string. Extension may contain numbers, the # and * characters, as well as the "w" character to "wait" for half a second. "#,
            TwilioPhoneNumbersError::ErrorCode22108 => r#"## Error - 22108

#### Invalid Application SID

You attempted to configure a phone number with an invalid Application SID. 

#### Possible Causes

* The SID is not a 34-character string that is prefixed with "AP".

* Invalid SMS Application SID provided in request

* Invalid Voice Application SID provided in request

* The Application resource identified by the SID does not exist anymore or does not belong to your account.

#### Possible Solutions

Please specify an Application SID you’ve created in your [Account Dashboard](https://www.twilio.com/user/account/apps)."#,
            TwilioPhoneNumbersError::ErrorCode10002 => r#"## ERROR - 10002

### Trial accounts do not support the feature you tried to use

Your account is currently in trial mode and therefore does not have access to the premium feature you attempted to use. Premium features are limited to full accounts, so you must upgrade your account if you wish to use them.

For guidance on updating your account, please see [How to Work with your Free Twilio Trial Account](/docs/usage/tutorials/how-to-use-your-free-trial-account#how-to-upgrade-your-account). 


#### Possible Causes
You attempted to use a premium feature with a trial account.

#### Possible Solutions
Upgrade to a full account and try the feature again."#,
            TwilioPhoneNumbersError::ErrorCode18004 => r#"## ERROR - 18004

### Redactions in document.

The submitted Supporting Document had redactions or covered information. Re-upload the Supporting Document without any redactions or covered portions. The supporting document you submitted had redactions or covered information that is required for the regulatory requirement. 

#### Possible Causes
Uploading a document with obscured or redacted information.

#### Possible Solutions
Re-upload the Supporting Document without any redactions or covered portions. For any ID document, make sure to provide a color copy of the ID document without any reductions. Make sure that all the edges of the ID document and MRZ data for passports are visible."#,
            TwilioPhoneNumbersError::ErrorCode18014 => r#"## ERROR - 18014

### Invalid or incomplete address provided.

The address you have provided is not valid. Please resubmit the bundle with the correct address. The address you have provided is not valid.

#### Possible Causes
The submitted address does not meet our requirements.

#### Possible Solutions
Please resubmit the bundle with the correct address."#,
            TwilioPhoneNumbersError::ErrorCode18001 => r#"## ERROR - 18001

### Invalid document submission

The uploaded document does not match the Regulatory Requirements criteria. Please upload a Supporting Document that matches the necessary Regulatory Requirements. The document you have uploaded does not match our Regulatory Requirements criteria.

#### Possible Causes
The document you have uploaded does not match our acceptable documentation for the Regulatory Requirements.

#### Possible Solutions
Please re-submit after uploading a Supporting Document according to our Guidelines. 
https://www.twilio.com/en-us/guidelines/regulatory"#,
            TwilioPhoneNumbersError::ErrorCode22222 => r#"## WARNING - 22222

### Emergency Status cannot be updated

 Emergency Status cannot be updated

#### Possible Causes
You sent a *POST* request to update the Emergency Status

#### Possible Solutions
Remove Emergency Status from the request"#,
            TwilioPhoneNumbersError::ErrorCode18011 => r#"## ERROR - 18011

### Business name mismatch

Business name provided does not match with the domestic company register or excerpt from the commercial register that you may have submitted. Please resubmit with the accurate business name. The business name provided does not match with the supporting document. 

#### Possible Causes
The business name and registration information do not align with the local company register or excerpt from the commercial register.

#### Possible Solutions
Please resubmit with the accurate business name and the corresponding business registration number."#,
            TwilioPhoneNumbersError::ErrorCode22218 => r#"## ERROR - 22218

### The Supporting Document field does not match the field in the End-User

Document and user fields mismatch. The Supporting Document field does not match the field in the End-User.

#### Possible Causes
The Regulation requires the fields on the Supporting Document and End-User to match.

#### Possible Solutions
Please ensure that the field in the Supporting Document matches the field in the End-User."#,
            TwilioPhoneNumbersError::ErrorCode22208 => r#"## ERROR - 22208

### Supporting Document status and attributes cannot be updated in the same request

 Both the Supporting Documents status and the attributes were present in the update request.

#### Possible Causes
The Supporting Documents status cannot have both the status and the attributes in the same update request.

#### Possible Solutions
Send the attributes in the update request followed with a status update in another request."#,
            TwilioPhoneNumbersError::ErrorCode22120 => r#"## Error - 22120

#### Invalid Verification Type

Verification Type must be either "phone-call" or "phone-bill"."#,
            TwilioPhoneNumbersError::ErrorCode22206 => r#"## ERROR - 22206

### Attempting to add invalid object type to bundle

 The object_sid provided was not a Supporting Document Sid or an End-User Sid

#### Possible Causes
The SID belongs to a different object group that is not allowed to be assigned as an item to a Bundle.

#### Possible Solutions
Please refer to the Supporting Documents LIST or End-User LIST resources to find the correct Supporting Document SID or End-User SID to assign as an item to a Bundle."#,
            TwilioPhoneNumbersError::ErrorCode22135 => r#"## ERROR - 22135

### Error - Internal Server Error

 

#### Possible Causes
Internal error determining the portability of this number, please try again

#### Possible Solutions
Run the Portability Check again"#,
            TwilioPhoneNumbersError::ErrorCode22500 => r#"## ERROR - 22500

### Twilio phone number using deprecated API version

 Phone Numbers no longer supports 2008-08-01 API.  The Twilio phone number is configured for the deprecated 2008-08-01 API, which is not compatible. Update the phone number API version to 2010-04-01

#### Possible Causes
The Twilio phone number is configured for the 2008-08-01 API

#### Possible Solutions
Update the phone number API version to 2010-04-01 or later version"#,
            TwilioPhoneNumbersError::ErrorCode18007 => r#"## ERROR - 18007

### Incomplete document submission.

The Proof of Identity document needs to show the front and back of the ID. Please upload a copy of both sides in a single document. The Proof of Identity needs to show the front and back of the ID. Your uploaded proof of Identity does not have both sides.

#### Possible Causes
Your uploaded proof of Identity does not have both sides.

#### Possible Solutions
Please upload a copy of both sides and make sure to upload them in one document."#,
            TwilioPhoneNumbersError::ErrorCode22121 => r#"## Error - 22121

#### Unable to Transfer Hosted Number 


#### Possible Causes


* The Hosted Number you are trying to transfer is not in status "in-use."

#### Possible Solutions

See [Exchanging Numbers Between Subaccounts](https://www.twilio.com/docs/api/rest/subaccounts#exchanging-numbers) for more details about transferring numbers."#,
            TwilioPhoneNumbersError::ErrorCode22211 => r#"## ERROR - 22211

### Cannot create a Supporting Document with no FriendlyName

 The request to create a new Supporting Document instance did not contain a FriendlyName

#### Possible Causes
The FriendlyName is missing from the POST LIST request to create a new Supporting Document.

#### Possible Solutions
Resend the request including the FriendlyName"#,
            TwilioPhoneNumbersError::ErrorCode22202 => r#"## ERROR - 22202

### No regulation sid or phone number country and type was provided

 No regulation sid or phone number country and type was provided

#### Possible Causes
The regulation sid or phone number country and type are missing from the request.

#### Possible Solutions
* Please ensure you are passing a regulation sid or phone number country and type
* If you need to know which regulation sid or phone number country and type to use, please refer to the Regulations REST API."#,
            TwilioPhoneNumbersError::ErrorCode22115 => r#"## Error - 22115

#### Invalid Unique Name

The unique name must be a string that does not exceed a length of 128 characters.

#### Possible Solutions

* Confirm that your unique name is a string that does not exceed 128 characters."#,
            TwilioPhoneNumbersError::ErrorCode18010 => r#"## ERROR - 18010

### Business registration number mismatch.

Business registration number does not match with the local company register or the excerpt from the commercial register that you may have submitted. Please resubmit with the accurate Business Registration number. The business registration number provided does not match the supporting document. 

#### Possible Causes
The submitted business registration number does not align with the local company register or excerpt from the commercial register.

#### Possible Solutions
Please resubmit with the accurate business registration number to ensure it matches the local company register or commercial register."#,
            TwilioPhoneNumbersError::ErrorCode22170 => r#"## ERROR - 22170

### Port In Error - Invalid Bundle

 Given Bundle SID does not exist for your account or is not approved.

#### Possible Causes
The Bundle SID does not exist or is not approved yet.

#### Possible Solutions
Update the Bundle SID to one that exists or is approved."#,
            TwilioPhoneNumbersError::ErrorCode22158 => r#"## ERROR - 22158

### Port In Error - Port Date Rejected

 The requested target port date was rejected by the losing carrier

#### Possible Causes
The requested target port date was rejected by the losing carrier. This can happen for a lot of reasons, some common ones include: 
- If the date is on a public holiday
- If the date is on a scheduled maintenance window of the losing carrier

#### Possible Solutions
Resubmit the port in request with a different target port date. Please review the new date to ensure it does not fall on a public holiday. 

Contact support at porting@twilio.com if you need help selecting a target port date."#,
            TwilioPhoneNumbersError::ErrorCode22157 => r#"## ERROR - 22157

### Port In Error - Invalid Subscription Right

 The subscription right provided as part of the porting request is invalid

#### Possible Causes
The subscription right provided as part of the porting request was not accepted by NTT. Certain NTT plans only allow one subscription right, see the solution section for which one you should use. 

#### Possible Solutions
Some NTT plans only allowed to request specific subscription rights. 

If your current NTT plan is the "Light Plan", then only the Termination subscription right will be accepted by NTT.

Otherwise you should request the subscription right "Suspension"."#,
            TwilioPhoneNumbersError::ErrorCode18020 => r#"## ERROR - 18020

### Proof of authorized representative’s association with the business required.

The Authorized Representative is not shown on the Business Registration. Please upload a Supporting Document that shows the Authorized Representative of the business. Need document to verify authorized representative’s association with the business.

#### Possible Causes
The authorized representative is not on the business registration.

#### Possible Solutions
Please upload a Supporting Document that shows the Authorized Representative as part of the business."#,
            TwilioPhoneNumbersError::ErrorCode22136 => r#"## ERROR - 22136

### Not Portable - Already in one of your Twilio Accounts

 

#### Possible Causes
Phone number already exists in one of your accounts or is currently being ported into one of your Twilio accounts.

#### Possible Solutions
If needed, see this [support page](https://support.twilio.com/hc/en-us/articles/223135327-Moving-Twilio-Phone-Numbers-to-another-Twilio-Account) for more information on how to move the line to a different subaccount."#,
            TwilioPhoneNumbersError::ErrorCode22104 => r#"## Error - 22104

#### Invalid Email Format

The format of a provided email was invalid.

#### Possible Causes

* Invalid email provided in request for email or cc emails.

#### Possible Solutions

Make sure you submit emails in the format local-part@domain"#,
            TwilioPhoneNumbersError::ErrorCode22106 => r#"## Error - 22106

#### Invalid Method

The method you provided for configuring your hosted number was invalid.

Valid values are GET and POST..

#### Possible Causes

* Invalid method passed for number configuration (SmsMethod, SmsFallbackMethod, VoiceMethod, VoiceFallbackMethod).

* Invalid method passed for status callback configuration (StatusCallbackMethod)."#,
            TwilioPhoneNumbersError::ErrorCode18002 => r#"## ERROR - 18002

### Illegible or blurry document submission

The document you have uploaded is not legible. Please re-submit a clear copy of the supporting document. The document you have uploaded is not legible or blurry.

#### Possible Causes
Blur or bad quality image uploaded.

#### Possible Solutions
Please re-submit after uploading a clear copy of the supporting document"#,
            TwilioPhoneNumbersError::ErrorCode18018 => r#"## ERROR - 18018

### PO Box not allowed.

The address you shared is a PO Box address, which is not acceptable. Please share a full physical address. A PO Box address is not allowed.

#### Possible Causes
PO Box address is not acceptable.

#### Possible Solutions
Please share a full physical address."#,
            TwilioPhoneNumbersError::ErrorCode22221 => r#"## WARNING - 22221

### Emergency address is not registered

 The emergency address is missing when an emergency call is made. The emergency call might have an additional cost.

#### Possible Causes
From number does not have a registered emergency address when an emergency call is made.

#### Possible Solutions
Please register an emergency address with the from number."#,
            TwilioPhoneNumbersError::ErrorCode21631 => r#"## ERROR - 21631

### Phone Number Requires an Address

 To purchase this number you must supply the Address of the user of this phone number.  Previously the IncomingPhoneNumbers API would look at your account for an appropriate address, however, you are now required to pass in an AddressSid that satisfies the requirement for the phone number.  This is because Twilio needs to know which address will be operating the phone number to meet regulatory requirements for that country and type of phone number.

#### Possible Causes
- No address was provided 
- No address was that satisfies the locality requirement for this phone number type.
- Depending on the country and number type, this may be an address in the same country or same geographic area as the number being purchased.

#### Possible Solutions
- Add an address that satisfies the locality requirement.
- Contact Twilio Support."#,
            TwilioPhoneNumbersError::ErrorCode21615 => r#"## ERROR - 21615

### Phone Number Requires a Local Address

 To purchase this number you must have an Address on your account which satisfies the local address requirements. 

#### Possible Causes
- No address on your account or subaccount that satisfies the locality requirement. Depending on the country, this can be either same country or same geographic area as the number being purchased.

#### Possible Solutions
- Add an address that satisfies the locality requirement
- Contact Twilio Support"#,
            TwilioPhoneNumbersError::ErrorCode18003 => r#"## ERROR - 18003

### Expired or invalid document submission

The submitted Supporting Document is expired or is invalid. Please resubmit a valid supporting document. The supporting document you submitted is either expired or invalid.

#### Possible Causes
The Supporting Document you uploaded has expired or it does not show an expiration date.

#### Possible Solutions
Please resubmit with a valid supporting document."#,
            TwilioPhoneNumbersError::ErrorCode18062 => r#"## ERROR - 18062

### Business Details contain an inactive business

The status of your entity is listed as inactive in your local business registry. Please amend the business details to include an active business registration. The status of your entity is listed as inactive in your local business registry.

#### Possible Causes
The status of your entity provided in the business details is listed as inactive in your local business registry.

#### Possible Solutions
Please amend the business details to include an active business registration."#,
            TwilioPhoneNumbersError::ErrorCode22132 => r#"## ERROR - 22132

### Not Portable - Already in Twilio different owner

 

#### Possible Causes
Phone number already exists on another Twilio account. 

#### Possible Solutions
See this [support page](https://support.twilio.com/hc/en-us/articles/223135327-Moving-Twilio-Phone-Numbers-to-another-Twilio-Account) for more information on how to move the line to your account."#,
            TwilioPhoneNumbersError::ErrorCode22101 => r#"## Error - 22101

### Invalid Hosted Number Order SIDs

You attempted to create or update an Authorization Document with one or more invalid Hosted Number Order SIDs. 

#### Possible Causes

* No Hosted Number Order SIDs were provided when creating the Authorization Document.

* One or more of the SIDs provided is not a 34-character string that is prefixed with "HR".



#### Possible Solutions
Please specify Hosted Number Order SIDs you've created in your [Account Dashboard](https://www.twilio.com/console/phone-numbers/hosted)."#,
            TwilioPhoneNumbersError::ErrorCode22201 => r#"## ERROR - 22201

### No regulation sid found for the given number group

 No regulation sid found for the given number group

#### Possible Causes
The regulation sid may be misspelled or belong to another account.

#### Possible Solutions
Use the Regulations LIST resource with filters to find the correct Number Group."#,
            TwilioPhoneNumbersError::ErrorCode22100 => r#"## Error - 22100

### Reached Maximum Verification Attempts

You have reached maximum verification attempts on this Hosted Number Order. Please reach out to hostedsms@twilio.com to reset the verification attempts."#,
            TwilioPhoneNumbersError::ErrorCode22150 => r#"## ERROR - 22150

### Port In Error - Contact support required

 There was an error during the port in process. 

#### Possible Causes
There was an error during the port in process.

#### Possible Solutions
Please contact Twilio's Porting Operations team at porting@twilio.com for support."#,
            TwilioPhoneNumbersError::ErrorCode22173 => r#"## ERROR - 22173

### Port In Error - Invalid Address

 Given Address SID does not exist for your account or is invalid.

#### Possible Causes
Given Address SID does not exist for your account or is invalid.

#### Possible Solutions
Update the Address SID to one that exists or check the account you are using."#,
            TwilioPhoneNumbersError::ErrorCode22103 => r#"## Error - 22103

### Unsupported Iso Country

You attempted to host a number, but Twilio does not currently support hosting numbers in the Iso Country provided.

"#,
            TwilioPhoneNumbersError::ErrorCode18009 => r#"## ERROR - 18009

### Nationality mismatch (Proof of Identity)

There is a mismatch between the nationality provided in the form and the Proof of Identity document. Please ensure that the information matches between form fields and Proof of Identity document. The information about the nationality provided in the Bundle does not match the information on the upload Proof of Identity.

#### Possible Causes
You may have uploaded a wrong document.

#### Possible Solutions
Please resubmit with the appropriate identity document. Please ensure the document uploaded and the information you provided match."#,
            TwilioPhoneNumbersError::ErrorCode22131 => r#"## ERROR - 22131

### Not Portable - Already in your Twilio Account

 

#### Possible Causes
Phone number already exists on your Twilio account or is currently being ported into your Twilio account

#### Possible Solutions
If needed, see this [support page](https://support.twilio.com/hc/en-us/articles/223135327-Moving-Twilio-Phone-Numbers-to-another-Twilio-Account) for more information on how to move the line to a different subaccount."#,
            TwilioPhoneNumbersError::ErrorCode22109 => r#"## Error - 22109

#### Invalid Address SID

You attempted to host a phone number with an invalid Address SID. 

#### Possible Causes

* The SID is not a 34-character string that is prefixed with "AD". 

* The Address resource identified by the SID does not exist anymore or does not belong to your account.

#### Possible Solutions

* Please specify an Address SID you’ve created in your [Account Dashboard](https://www.twilio.com/console/phone-numbers/addresses)."#,
            TwilioPhoneNumbersError::ErrorCode22156 => r#"## ERROR - 22156

### Port In Error - Invalid Account Number

 The Account Number submitted in the port in request for this phone number does not match the losing carrier's information. 

#### Possible Causes
The Account Number submitted for the port in request does not match the losing carrier's information. 

#### Possible Solutions
Update Account Number in the port in request to one that matches the losing carrier's information. 

Note if this is the only phone number in a port in request that got rejected you may need to start a new port in request with this phone number so that you can correct the account number as it may be different from the other phone numbers in this request."#,
            TwilioPhoneNumbersError::ErrorCode21645 => r#"## ERROR - 21645

### Supporting Document cannot be deleted due to active Regulatory Bundle assignment

 The Supporting Document cannot be deleted as it is an active item assignment in one or more Regulatory Bundles.

#### Possible Causes
The Supporting Document cannot be deleted as it is an active item assignment in one or more Regulatory Bundles.

#### Possible Solutions
Unassign the Supporting Document from the Regulatory Bundle(s)"#,
            TwilioPhoneNumbersError::ErrorCode21651 => r#"## Error - 21651

### Document does not satisfy regulatory requirement

The document being mapped to the number does not satisfy the regulatory requirements. 

#### Possible Causes
- The document is the incorrect type for the number you are trying to purchase or update.
- The document does not have the correct metadata fields required for the number you are trying to purchase or update.
- The document sid is not valid. 

#### Possible Solutions
- Ensure that the document you are using satisfies the regulatory requirement.
- Ensure the document sid refers to a document on your account or subaccount. 
- Contact support for assistance."#,
            TwilioPhoneNumbersError::ErrorCode18015 => r#"## ERROR - 18015

### Invalid or incomplete emergency address provided.

The emergency address you have provided is not valid. Please resubmit the bundle with the correct address. The emergency address you have provided is not valid.

#### Possible Causes
The submitted emergency address does not meet our requirements.

#### Possible Solutions
Please resubmit the bundle with the correct address."#,
            TwilioPhoneNumbersError::ErrorCode18023 => r#"## ERROR - 18023

### Document contains an inactive business

The status of your entity is listed as inactive in your local business registry. Please upload a Business Document showing a currently active business registration. The status of your entity is listed as inactive in your local business registry. Please upload a Business Document showing a currently active business registration.

#### Possible Causes
The status of your entity provided in the Business Document is listed as inactive in your local business registry.

#### Possible Solutions
Please upload a Business Document showing a currently active business registration."#,
            TwilioPhoneNumbersError::ErrorCode22199 => r#"## ERROR - 22199

### Configuration Retrieval Error

The Phone Numbers Configuration is Missing The Phone Numbers Configuration is Missing.  Phone Numbers have a configuration for each region.  This Error shows if you are attempting to perform an action that requires region specific configuration.  Please create or add a configuration to this Phone Number for the Region in question.

#### Possible Causes
Performing an Action that Requires a Configuration in Region in Question

#### Possible Solutions
Double Check that the phone number has a configuration for each region you are working with.
"#,
            TwilioPhoneNumbersError::ErrorCode21649 => r#"## ERROR - 21649

### Phone Number Requires a Bundle

 To provision this number you must supply the Bundle with End-User information and Supporting Documents to comply with local telecom regulations. 

Previously the IncomingPhoneNumbers API would let you provision the phone number without providing a Bundle, however, you are now required to pass in a BundleSid that satisfies the requirement for the phone number. This is because Twilio needs to know which Bundle containing End-User information and Supporting Documents will be assigned to the phone number to meet regulatory requirements for that country and type of phone number.

#### Possible Causes
- No Bundle was provided
- No Bundle was that satisfies the locality requirement for this phone number type
- Depending on the country, number type, and end-user type, the regulations will be different compared to other phone numbers

#### Possible Solutions
- Build a new Bundle that satisfies the regulations of the phone number.
- Contact Twilio Support."#,
            TwilioPhoneNumbersError::ErrorCode22116 => r#"## Error - 22116

#### Invalid Friendly Name

Friendly name is a descriptive name to help you remember your resource. The friendly name cannot exceed a length of 128 characters. 

#### Possible Solutions

* Confirm that your friendly name is a string that does not exceed 128 characters."#,
            TwilioPhoneNumbersError::ErrorCode22350 => r#"## ERROR - 22350

### The Phone Number cannot be released because it is being ported out of Twilio

 The Phone Number cannot be released because it is being ported out of Twilio.

#### Possible Causes
The Phone Number is being ported out of Twilio.

#### Possible Solutions
You cannot release a Phone Number that is being ported out. Please, reach out to porting@twilio.com for assistance."#,
            TwilioPhoneNumbersError::ErrorCode22110 => r#"## Error - 22110

#### Phone Number Not Hostable

You attempted to host a number that is not hostable on Twilio.

#### Possible Causes

* The number is already hosted. 

* The number is owned by Twilio.

* The number cannot be enabled by carrier for requested capabilities.

* The number is type mobile or voip."#,
            TwilioPhoneNumbersError::ErrorCode18013 => r#"## ERROR - 18013

### Require domestic emergency address.

This phone number type requires a valid domestic emergency address on the document. Please resubmit a document with a valid domestic address and ensure you've selected the desired phone number type. This phone number type requires a valid domestic emergency address.

#### Possible Causes
- You may have submitted an international address.                                                               - Your address is not a valid address.

#### Possible Solutions
Please resubmit with a valid domestic address or create a new bundle selecting the appropriate phone number type."#,
            TwilioPhoneNumbersError::ErrorCode18037 => r#"## ERROR - 18037

### Missing information in the form

 One of more of the information in the forms are missing.

#### Possible Causes
You may have missed sharing the required information.

#### Possible Solutions
Please fill out the required form fields and resubmit the bundle."#,
            TwilioPhoneNumbersError::ErrorCode22172 => r#"## ERROR - 22172

### Port In Error - Contains numbers for multiple countries

 Port in request contains numbers for multiple countries or that are not in E.164 format. 

#### Possible Causes
There are phone numbers in the port in request from multiple countries.

There are phone numbers in the port in request that are not in E.164 format and so the country cannot be identified. 

#### Possible Solutions
Please review the list of phone numbers and submit one port in request for all numbers from each country. A port in request can only contain phone numbers from one country. 

Also double check that all phone numbers are in E.164 format so that Twilio can correct identify the country code. 
https://www.twilio.com/docs/glossary/what-e164"#,
            TwilioPhoneNumbersError::ErrorCode21646 => r#"## ERROR - 21646

### Supporting Document is not eligible for deletion

 The Supporting Document is in a non-deletable status and is not eligible to be deleted until the Supporting Document transitions to a deletable status.

#### Possible Causes
The Supporting Document is in a non-deletable status (PENDING_REVIEW).

#### Possible Solutions
Wait for the Supporting Document to transition to a deletable status (DRAFT, TWILIO_REJECTED, TWILIO_APPROVED). If your Supporting Document is idle for more than 3 business days, please reach out to Twilio Support through the [Console](https://console.twilio.com) or [Help Center](https://support.twilio.com)."#,
            TwilioPhoneNumbersError::ErrorCode18021 => r#"## ERROR - 18021

### Name mismatch (Proof of Address)

The name on the Proof of Address document does not match what's submitted in the first name / last name fields. Please ensure the document and field information match. The individual’s name on the Proof of Address document does not match the information entered in the Bundle.

#### Possible Causes
The name on the Proof of Address document does not match what's submitted in the first name / last name fields. Please ensure the document and field information match.

#### Possible Solutions
Please ensure the names match exactly and that all the individual's last names are provided. Please amend firstly the name in the Business & legal representative information / Individual information section of the bundle, and then in the Bundle fields under the uploaded supporting document."#,
            TwilioPhoneNumbersError::ErrorCode22216 => r#"## ERROR - 22216

### Missing Supporting Document

A document is missing. A Supporting Document satisfying the requirement is missing from the regulatory bundle.

#### Possible Causes
The Regulation requires a Supporting Document which is missing from your Regulatory Bundle.

#### Possible Solutions
Please refer to the Regulations API resource to ensure that a Supporting Document satisfying the requirement is in the regulatory bundle."#,
            TwilioPhoneNumbersError::ErrorCode22123 => r#"## Error - 22123

#### Unable to Initiate Verification Call

#### Possible Causes

* The number you are trying to verify cannot be reached.

* Your account is not allowed to call the number you are trying to verify.

* The number you are attempting to verify has been blocked.

* You are attempting to verify a number that is not a valid phone number.

#### Possible Solutions

Please reach out to HostedSMS@twilio.com."#,
            TwilioPhoneNumbersError::ErrorCode22154 => r#"## ERROR - 22154

### Port In Error - Invalid Address

 The address submitted in the port in request does not match the losing carrier's information for this phone number. 

#### Possible Causes
The address submitted in the port in request does not match the losing carrier's information for this phone number. 

#### Possible Solutions
Check the address submitted in the port in request for typos or change it to an address that matches the losing carrier's information for this phone number. 

Note if this is the only phone number in a port in request that got rejected you may need to start a new port in request with this phone number so that you can correct the address as it may be different from the other phone numbers in this request. "#,
            TwilioPhoneNumbersError::ErrorCode22228 => r#"## ERROR - 22228

### From Bundle does not have latest Regulation requirements that matches destination Bundle

 The from Bundle does not have the latest Regulation requirements which match the destination Bundle

#### Possible Causes
Incompatible Regulation requirements.

#### Possible Solutions
Please reach out to Twilio Support through the [Console](https://console.twilio.com) or [Help Center](https://support.twilio.com) by referencing this API error code with the BundleSid requesting assistance with fixing the Bundle."#,
            TwilioPhoneNumbersError::ErrorCode21629 => r#"## Error - 21629

### Address Validation Error - Check Suggested Address

The address you have provided cannot be validated. A similar address has been found to be valid. The suggested address is included in the error message body. 

#### Possible Causes
- Address misspelling
- Alternate format or spelling in third-party data source

#### Possible Solutions
- Ensure the address is spelled correctly
- If a correct suggestion, submit the suggested address for validation
- Contact Twilio Support"#,
            TwilioPhoneNumbersError::ErrorCode18016 => r#"## ERROR - 18016

### Address mismatch.

The address does not match exactly with the Supporting Document. Please amend the Address fields to match the document's address. The address you have provided does not match with the supporting document.

#### Possible Causes
There may be a typo when submitting the address information.

#### Possible Solutions
Please amend the data entered in the Address fields to match exactly the address in the documents you have provided."#,
            TwilioPhoneNumbersError::ErrorCode18050 => r#"## ERROR - 18050

### Issue with the Supporting Document(s)

There is an issue with one or more of your Supporting Documents. Please review each rejection reason for the Supporting Document(s). There is an issue with your Supporting Document(s).

#### Possible Causes
One or more of your Supporting Documents has been reviewed and was found to have issues.

#### Possible Solutions
Please review the Supporting Documents for any rejection reason(s), and resubmit with correct Supporting Documents."#,
            TwilioPhoneNumbersError::ErrorCode22107 => r#"## Error - 22107

#### Unable to Update Authorization Document

You attempted to make a change that cannot be applied to the Authorization Document resource.

#### Possible Causes

* Trying to update or delete an Authorization Document resource that is in status "signed."


* Trying to update an Authorization Document that is in status "signing."

#### Possible Solutions


Update the Authorization Document to status "opened" if it is not already in a terminal state."#,
            TwilioPhoneNumbersError::ErrorCode18601 => r#"## ERROR - 18601

### The association between business name and website cannot be verified

 The association between business name and website cannot be verified

#### Possible Causes
The business name does not match the website SSL certificate, or the website URL

#### Possible Solutions
Please edit the Business Profile & update the missing information. You can also reach out to our support team at trusthub-verify@twilio.com for any additional clarifications."#,
            TwilioPhoneNumbersError::ErrorCode22300 => r#"## ERROR - 22300

### This account is restricted from provisioning new long code phone numbers

 The account is temporarily restricted from provisioning new long code phone numbers.

#### Possible Causes
Twilio Compliance has temporarily restricted the account from provisioning new long code phone numbers.

#### Possible Solutions
Please reach out to verifymyaccount@twilio.com to have this restriction lifted from your account. Be sure to include your Twilio Account SID, the name of your business or project, and a brief explanation of your use case."#,
            TwilioPhoneNumbersError::ErrorCode22229 => r#"## ERROR - 22229

### Supporting Document Bundle Assignment cannot be removed

 The destination Bundle must have the same address as the Bundle Copy due to lack of feature support for Phone Number address assignment.

#### Possible Causes
The Address in the Bundle Copy must be the same as in the destination Bundle and cannot be removed.

#### Possible Solutions
Metadata of the Supporting Document can be updated but the Address is immutable. Either a new Supporting Document that validates the same information can be supplied or a new Regulatory Bundle can be created."#,
            TwilioPhoneNumbersError::ErrorCode18610 => r#"## ERROR - 18610

### Customer Profile or Trust Product cannot be copied due to a deleted Address

 Customer Profile or Trust Product requested cannot be copied due to a deleted Address.

#### Possible Causes
The Address linked to the Customer Profile or Trust Product was deleted.

#### Possible Solutions
Create a new Address on your Account or Sub-Account (if you don’t already have one) and reach out to [Twilio Support](https://support.twilio.com/hc/en-us) to link it to the Customer Profile or Trust Product that you wish to copy."#,
            TwilioPhoneNumbersError::ErrorCode18059 => r#"## ERROR - 18059

### Missing/Invalid Photo ID

An Authorized Representative's government-issued photo ID document missing or invalid. Please resubmit with a valid supporting document. An Authorized Representative's government-issued photo ID document missing or invalid.

#### Possible Causes
- Missing government-issued photo ID
- Invalid Photo ID (ex: expired or incorrect name)

#### Possible Solutions
Ensure the dates are current and the information on the Photo ID matches the Authorized Individual's information and resubmit with a valid government-issued Photo ID."#,
            TwilioPhoneNumbersError::ErrorCode180017 => r#"## ERROR - 180017

### Require domestic emergency address

 This phone number type requires a valid domestic emergency address.

#### Possible Causes
- You may have submitted an international address.
- Your address is not a valid address.

#### Possible Solutions
Please resubmit with a valid domestic address or create a new bundle selecting the appropriate phone number type."#,
            TwilioPhoneNumbersError::ErrorCode18006 => r#"## ERROR - 18006

### Information does not match the supporting document

The information does not match the regulatory requirement. The information you shared does not match the supporting document.

#### Possible Causes
The information you shared does not match the supporting document.

#### Possible Solutions
You may have attached the wrong document or shared inaccurate information."#,
            TwilioPhoneNumbersError::ErrorCode21647 => r#"## ERROR - 21647

### Regulatory Bundle is not eligible for deletion

 Regulatory Bundle is in a non-deletable status (PENDING_REVIEW & IN_REVIEW)

#### Possible Causes
Regulatory Bundle is in a non-deletable status.

#### Possible Solutions
Wait for the Regulatory Bundle to transition to a deletable status (DRAFT, TWILIO_REJECTED, TWILIO_APPROVED).

If the Regulatory Bundle is idle for 3 or more business days, please reach out to Twilio Support through the [Console](https://console.twilio.com) or [Help Center](https://support.twilio.com)."#,
            TwilioPhoneNumbersError::ErrorCode22219 => r#"## ERROR - 22219

### An Address is missing

An address is missing. An Address is missing from the regulatory Bundle

#### Possible Causes
The Regulation requires an Address to be assigned to a Supporting Document in the Regulatory Bundle.

#### Possible Solutions
Please refer to the Regulations API to ensure that an AddressSid is assigned to a Supporting Document."#,
            TwilioPhoneNumbersError::ErrorCode21644 => r#"## ERROR - 21644

### End-User cannot be deleted due to an active assignment to a Bundle

 The End-User cannot be deleted due to active assignments

#### Possible Causes
The End User cannot be deleted due to one or more active assignments to a Bundle

#### Possible Solutions
Reassign a new End-User to all Bundle(s) and then perform the DELETE operation."#,
            TwilioPhoneNumbersError::ErrorCode18019 => r#"## ERROR - 18019

### Proof of Identity Required for Authorized Representative.

We require Proof of Identity for the Authorized Representative. Please provide a government-issued ID or passport for the Authorized Representative. We require proof of identity for the Authorized Representative.

#### Possible Causes
We could not verify the identity of the Authorized Representative.

#### Possible Solutions
Please provide a government-issued ID or passport for the Authorized Representative to fulfill this requirement."#,
            TwilioPhoneNumbersError::ErrorCode18039 => r#"## ERROR - 18039

### The phone number type you selected requires a valid domestic address.

 The phone number type you selected requires a valid local address. Please resubmit a local address or create a new bundle selecting the appropriate phone number type.

#### Possible Causes
x

#### Possible Solutions
x"#,
            TwilioPhoneNumbersError::ErrorCode18057 => r#"## ERROR - 18057

### Validation Issue for Authorized Representative

The digital validation for the authorized representative has failed. Please resubmit with an identity document for the Authorized Representative. The digital validation for the authorized representative has failed.

#### Possible Causes
There was an issue verifying the identity of the authorized representative. 

#### Possible Solutions
Please share a different authorized representative or reach out to us via email numbers-regulatory-review@twilio.com and share the details of how the company and website are connected."#,
            TwilioPhoneNumbersError::ErrorCode22226 => r#"## ERROR - 22226

### Cannot replace Items from Bundle to same Bundle

 From FromBundleSid cannot be the same Bundle as in URI request.

#### Possible Causes
The same FromBundleSid is being used in both the URI and the From Bundle.

#### Possible Solutions
Please ensure From FromBundleSid is Copy of the provisioning Bundle."#
        }
    }
}
