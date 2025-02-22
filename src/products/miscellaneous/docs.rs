// AUTO-GENERATED. DO NOT MODIFY. YOUR CHANGES WILL BE OVER-WRITTEN
use crate::products::TwilioMiscellaneousError;
use standard_error::traits::StandardErrorDocsTrait;

impl StandardErrorDocsTrait for TwilioMiscellaneousError {
    fn docs(&self) -> &'static str {
        match self {
            TwilioMiscellaneousError::ErrorCode13000 => r#"None"#,
            TwilioMiscellaneousError::ErrorCode21616 => r#"## Error - 21616

### The 'From' number matches multiple numbers for your account

You have attempted to send a SMS with a 'From' number that matches more
than one number belonging to your account. Please submit the 'From'
number in E.164 format to resolve the ambiguity.
"#,
            TwilioMiscellaneousError::ErrorCode13612 => r#"## Warning - 13612 

### Record: Invalid maxLength value

The maxLength attribute must be a positive integer in seconds. See the <a href='/docs/api/twiml/record#maxLength'>Record Verb</a> API Reference for more information.





"#,
            TwilioMiscellaneousError::ErrorCode19023 => r#"## ERROR - 19023

### Invalid channel type

 Request with an invalid channel type

#### Possible Causes
Channel type other than: work, home, other

#### Possible Solutions
Only work, home or other are valid channels types"#,
            TwilioMiscellaneousError::ErrorCode51109 => r#"## Error - 51109

### Service instance is under legal hold

#### Possible Causes 
Specified service instance is under legal hold.

#### Possible Solutions
Please use a different service instance."#,
            TwilioMiscellaneousError::ErrorCode21227 => r#"## Warning - 21227

### Headers portion of SIP URI must be fewer than 1024 chars

The headers you are sending must be fewer than 1024 characters."#,
            TwilioMiscellaneousError::ErrorCode21504 => r#"## Error - 21504 

### RecordingSid is required.

You POSTed to the /Accounts/{AccountSid}/Transcriptions resource, but you did not include a RecordingSid.





"#,
            TwilioMiscellaneousError::ErrorCode21406 => r#"## Error - 21406 

### Cannot set SmsFallbackUrl without setting SmsUrl

You cannot set a fallback URL without specifying a primary URL




"#,
            TwilioMiscellaneousError::ErrorCode51106 => r#"## Error - 51106

### Active product doesn't match with service instance product

#### Possible Causes 
Passed service instance belongs to another product.

#### Possible Solutions
Verify matching product and service instance is selected."#,
            TwilioMiscellaneousError::ErrorCode13241 => r#"## Warning - 13241 

### Dial->SIP: Invalid method value

The method attribute of Number must be GET or POST





"#,
            TwilioMiscellaneousError::ErrorCode21602 => r#"## ERROR - 21602

### Message body is required

You must specify content (text or media) for your message. The `Body` OR `MediaURL` parameter is required to send a Message with Twilio.

#### Possible Causes
You may have attempted to send a message without a `Body` or `MediaURL` parameter.

#### Possible Solutions
 Retry your message including a body or a link to some media. For sample code that shows how to send an SMS message with Twilio's supported helper libraries, cURL, or the Twilio CLI, please see [this short tutorial](https://www.twilio.com/docs/sms/send-messages)"#,
            TwilioMiscellaneousError::ErrorCode84004 => r#"## ERROR - 84004

### Error while creating execution via REST API.

 Required values to create execution might not have been provided.

#### Possible Causes
- The Create Execution endpoint was called with parameters that could not be processed.

#### Possible Solutions
- Double-check the provided parameters for syntax errors."#,
            TwilioMiscellaneousError::ErrorCode21477 => r#"## Error - 21477

### Unable to update Status for parent accounts

Unable to update Status for parent accounts, status updates restricted to subaccounts only

"#,
            TwilioMiscellaneousError::ErrorCode70156 => r#"## Error - 70156

### Request Hash Is Invalid
The request hash that is declared in the JWT does not match the given request’s hash calculated by Twilio. This is a Public Key Client Validation Error.

#### Possible Solution:
* Make sure you are using the latest Twilio helper library. If not using the library, make sure the request hash is calculated on the canonicalized HTTP request and the same canonicalization is used.
* For the details on JWTs used as part of Public Key Client Validation, see [the documentation](/docs/api/credentials/public-key-client-validation-getting-started)"#,
            TwilioMiscellaneousError::ErrorCode94601 => r#"## ERROR - 94601

### Date range is not valid

 The action could not be performed because the date range provided is not valid.

#### Possible Causes
 * UntilDate is equal or more recent than FromDate.

#### Possible Solutions
 * FromDate should be a recent date than UntilDate."#,
            TwilioMiscellaneousError::ErrorCode19041 => r#"## ERROR - 19041

### Invalid or missing Custom Field input

 Request body is not valid

#### Possible Causes
Custom field value is missing, Custom Field value is invalid and/or Custom Field is duplicated

#### Possible Solutions
Review the request body and the API documentation for more information"#,
            TwilioMiscellaneousError::ErrorCode21235 => r#"## ERROR - 21235

### IP Access Control List Validation Error

One of the parameters passed when creating or updating an IP Access Control List was incorrect. 

#### Possible Causes
* An IP Access Control List or IP Address with the same FriendlyName may already exist.
* The IP Access Control List may already contain that IP Address

#### Possible Solutions
* Use a different FriendlyName
* Make sure the IP Access Control List does not already contain that IP Address."#,
            TwilioMiscellaneousError::ErrorCode21222 => r#"## Warning - 21222

### Invalid SipAuthUsername. Illegal chars

The SipAuthUsername you specified contained characters not allowed in SIP.
"#,
            TwilioMiscellaneousError::ErrorCode91203 => r#"## ERROR - 91203

### Method Not Allowed

 This operation is not allowed against the resource

#### Possible Causes
* Deleting a schema is not allowed

#### Possible Solutions
* Avoid executing a DELETE action against the Schema resource"#,
            TwilioMiscellaneousError::ErrorCode21237 => r#"## ERROR - 21237

### Maximum IP Addresses Reached for List

IP Access control lists cannot contain more than 25 Addresses. 

#### Possible Causes
* IP Access control lists cannot contain more than 25 Addresses.

#### Possible Solutions
* Remove IP Addresses from your IP Access Control List if you must create new ones."#,
            TwilioMiscellaneousError::ErrorCode21229 => r#"## Warning - 21229

### Invalid SIP Header. Illegal chars in header value

The header value you passed has characters not allowed in SIP."#,
            TwilioMiscellaneousError::ErrorCode13234 => r#"## Warning - 13234

### Dial->Conference: Invalid waitMethod value

waitMethod must be either GET or POST
"#,
            TwilioMiscellaneousError::ErrorCode15002 => r#"## Error - 15002

### Call Progress: Queue Timeout

A callback request timed out in the callback queue.

#### Possible Causes

*  Your application is not responding to Call Progress Events in a timely manner leading to queued callbacks expiring before they are sent.
*  There was a network disruption between Twilio and your web server.

#### Possible Solutions

*   Please ensure your application is responding to callbacks in a timely manner."#,
            TwilioMiscellaneousError::ErrorCode51108 => r#"## Error - 51108

### Service instance disabled

#### Possible Causes 
Service instance is disabled.

#### Possible Solutions
Please use a different service instance."#,
            TwilioMiscellaneousError::ErrorCode21264 => r#"## ERROR - 21264

### ‘From’ phone number not verified

 You received an inbound phone call to a Twilio phone number owned by your trial account, but the 'From' number is not a [Verified Caller ID](https://support.twilio.com/hc/en-us/articles/223180048-How-to-Add-and-Remove-a-Verified-Phone-Number-or-Caller-ID-with-Twilio) on your account. In order to receive calls to a Twilio number during a free trial, you must first verify your ownership of the phone number that is placing the call.

#### Possible Causes
The ‘From’ number of the inbound call is not a [Verified Caller ID](https://support.twilio.com/hc/en-us/articles/223180048-How-to-Add-and-Remove-a-Verified-Phone-Number-or-Caller-ID-with-Twilio). Trial accounts can only receive inbound calls from verified phone numbers.

#### Possible Solutions
You can verify phone numbers from the [phone numbers section](https://www.twilio.com/user/account/phone-numbers) of your account portal."#,
            TwilioMiscellaneousError::ErrorCode94403 => r#"## WARNING - 94403

### Transcriptions: Invalid encryption credentials

 Public key credentials to encrypt transcription results are invalid. Twilio will attempt operation again shortly. 

#### Possible Causes
* Public key specified in Transcription Settings is not found in Twilio Credentials Storage
* Public key defined in credentials resource is invalid and can’t be used to encrypt transcription files
* Public key defined in credentials resource is expired and can’t be used to encrypt transcription files

#### Possible Solutions
* Ensure public key is correct in credentials resource
* Set a different public key that can be used to encrypt transcription results or disable encryption"#,
            TwilioMiscellaneousError::ErrorCode20011 => r#"## Error Code 20011

### Invalid TLS version

The request was rejected because it was not correctly configured to use TLSv1.2. All new accounts are [required to use TLSv1.2 with supported ciphers suites](https://support.twilio.com/hc/en-us/articles/360007724794). 

#### Possible Solutions:
* Reconfigure request to use a supported cipher suite for TLSv1.2
* Review these [tips for upgrading your environment to TLSv1.2](https://support.twilio.com/hc/en-us/articles/360006751753)"#,
            TwilioMiscellaneousError::ErrorCode51113 => r#"## Error - 51113

### Product usage is not enabled

#### Possible Causes 
Product usage is not enabled."#,
            TwilioMiscellaneousError::ErrorCode21402 => r#"## Error - 21402 

### Invalid URL

You attempted to update the URL handler associated with a phone number, but the URL you specified was not valid. 

#### Possible Solutions

Make sure you submit a fully qualified URL including:

*   protocol (http:// or https://)
*   hostname
*   file path
*   properly url-encoded query parameters

Twilio must be able to reach this URL over the public Internet.
"#,
            TwilioMiscellaneousError::ErrorCode13616 => r#"## Warning - 13616 

### Record: playBeep must be true or false

The playBeep attribute must have a value of "true" or "false"

"#,
            TwilioMiscellaneousError::ErrorCode70252 => r#"## Error - 70252

### Bad Saml Response
The Request tries to authenticate through SSO. The request is well-formed, but the response from the Identity Provider is missing necessary attributes. 

#### Possible Causes 
* Misconfigured Identity Provider settings

#### Possible Solution
* Reconfigure the Identity Provider settings and retry."#,
            TwilioMiscellaneousError::ErrorCode14220 => r#"## Error - 14220

### Enqueue: Provided Workflow was not a valid sid

Provided Workflow is invalid.
"#,
            TwilioMiscellaneousError::ErrorCode70105 => r#"## Error - 70105

### Invalid Type Specified in the Request
When using a Twilio Resource that supports working with different types of Twilio Credentials (Public Key Credentials, Push Credentials) the request must contain type(s) that is recognized by the resource.

#### Possible Solution:
* Check the API documents to verify the correct credential type(s) were used in the request.

"#,
            TwilioMiscellaneousError::ErrorCode13618 => r#"## Warning - 13618

### Record: Recording not available for transcription

#### Possible Causes 
* The recording was deleted before before Twilio was able to fetch the recording for transcription.

#### Possible Solutions
* Make a request to delete the recording once you receive the transcribe callback instead of the record callback. This ensures that the recording is available for Twilio to transcribe and still allows you to delete recordings from Twilio.
"#,
            TwilioMiscellaneousError::ErrorCode94400 => r#"## ERROR - 94400

### Transcriptions: transcription internal error

 Twilio failed to transcribe audio due to an internal issue. The transcription files have been deleted and the resource has been marked as “failed”

#### Possible Causes
* Twilio failed to process transcription request and all retries attempts were exhausted
* Twilio failed to normalize and/or store transcription results and all retries attempts were exhausted

#### Possible Solutions
 "#,
            TwilioMiscellaneousError::ErrorCode21475 => r#"## Error - 21475

### Unable to update Status, invalid Status.

The Status you specified must be one of the following strings: 'active', 'suspended', or 'closed'.




"#,
            TwilioMiscellaneousError::ErrorCode13310 => r#"## Warning - 13310 

### Gather: Invalid finishOnKey value

The value of the finishOnKey attribute must be one of the following characters "0123456789#*". See the <a href='/docs/api/twiml/gather#finishOnKey'>Gather Verb</a> API Reference for more information.





"#,
            TwilioMiscellaneousError::ErrorCode19035 => r#"## ERROR - 19035

### Invalid page size, it must be between 1 and 25 if specified

 The page size provided in the request is not valid

#### Possible Causes
Page size specified greater than 25 or less than 1

#### Possible Solutions
Page size must be between 1 and 25"#,
            TwilioMiscellaneousError::ErrorCode21702 => r#"## Error - 21702

### The Messaging Service is not available to send new messages.

The Messaging Service is not available to send new messages.

#### Possible Solutions
*  Retry your API call
"#,
            TwilioMiscellaneousError::ErrorCode19026 => r#"## ERROR - 19026

### Maximum number of channels allowed reached

 Maximum number of channels of the same type  allowed reached

#### Possible Causes
Attempt to create more than 5 channels of the same type

#### Possible Solutions
Only 5 channels of the same type are allowed"#,
            TwilioMiscellaneousError::ErrorCode21456 => r#"## Error - 21456 

### Invalid CallbackUrl

The callback URL given to callback after validation isn't a valid URL.

#### Possible Causes 






"#,
            TwilioMiscellaneousError::ErrorCode32017 => r#"## ERROR - 32017

### PSTN: Carrier blocked call due to calling number (caller ID)

 Intermediary / Carrier Analytics blocked call due to poor reputation score on the telephone number

#### Possible Causes
* The calling number has a low answer rate from end users
* The calling number generates low duration calls
* The calling number is responsible for large volumes of unwanted calls

#### Possible Solutions
Register the telephone numbers with the analytic providers. To reach the major mobile carrier's analytic providers, please register the telephone numbers at www.freecallerregistry.com . To reach T-Mobile's analytic provider directly to register numbers, please go to https://portal.firstorion.com to register numbers. Going direct to the analytic provider has a faster turnaround of being registered / call being delivered to end user"#,
            TwilioMiscellaneousError::ErrorCode51107 => r#"## Error - 51107

### Service can't be used"#,
            TwilioMiscellaneousError::ErrorCode13111 => r#"## Error - 13111 

### Annotate: Annotate must contain only one of element X

Annotate must contain only one of element X





"#,
            TwilioMiscellaneousError::ErrorCode21233 => r#"# Error - 21233

### Domain still has subdomains  

You cannot delete a domain while subdomains of it exist.  
"#,
            TwilioMiscellaneousError::ErrorCode19033 => r#"## ERROR - 19033

### Location validation error

 The request body has invalid inputs

#### Possible Causes
Invalid length, invalid data type

#### Possible Solutions
Review the request body and the API documentation for more information"#,
            TwilioMiscellaneousError::ErrorCode94402 => r#"## ERROR - 94402

### Transcriptions: Encryption failed and transcription result files deleted

 Twilio failed to encrypt and securely store transcription results. The transcription files have been deleted and the resource has been marked as “failed”

#### Possible Causes
* Twilio failed to encrypt transcription results and all retries attempts were exhausted

#### Possible Solutions
* Set a public key that can be used to encrypt transcription results or disable encryption so future transcriptions can be successfully processed and stored"#,
            TwilioMiscellaneousError::ErrorCode80403 => r#"## Warning

### Interaction Sid Invalid

Example Message: Invalid Interaction Sid"#,
            TwilioMiscellaneousError::ErrorCode19028 => r#"## ERROR - 19028

### Channel value can not be updated

 Channel value can not be updated

#### Possible Causes
The channel value provided in the request is different from the existing one

#### Possible Solutions
Remove channel value from the request or Delete the existing channel and create a then new one with the new value"#,
            TwilioMiscellaneousError::ErrorCode21623 => r#"## Error - 21623

### Number of media files exceeds allowed limit

The number of media URLs exceeds the maximum allowed. You may send up to 10 media files in a single message. 
"#,
            TwilioMiscellaneousError::ErrorCode19012 => r#"## ERROR - 19012

### When updating a contact at least one field should be updated

 No contact updates provided in the request

#### Possible Causes
User has not provided contact updates

#### Possible Solutions
When updating a contact at least one field should be updated"#,
            TwilioMiscellaneousError::ErrorCode60002 => r#"## ERROR - 60002

### End User Identification Timeout

 End User Identification Timeout

#### Possible Causes
The request was received but failed during processing due to a timeout. If using Silent Network Auth, this can happen when the device does not invoke the SNA URL within the timeout period. The default timeout period is 10 minutes.

#### Possible Solutions
Resend request for this phone number to restart timeout period."#,
            TwilioMiscellaneousError::ErrorCode61001 => r#"## Error - 61001

### Add-ons: Request timed out

#### Possible Causes 
*   The Add-on Provider failed to respond in the allotted time. The request was terminated and no further processing was attempted.

#### Possible Solutions
*  Contact Add-on Provider if this error persists"#,
            TwilioMiscellaneousError::ErrorCode21242 => r#"## ERROR - 21242

### Maximum Credentials Reached for List

Credential Lists cannot contain more than 1000 Credentials. 

#### Possible Causes
* Credential Lists cannot contain more than 1000 Credentials.

#### Possible Solutions
* Delete Credentials from your Credential List if you must create more."#,
            TwilioMiscellaneousError::ErrorCode32102 => r#"## Error - 32102

###  SIP: Bad SDP

#### Possible Causes 
*  The SDP is not correctly formatted.

#### Possible Solutions
*  Please see [RFC 2327 - SDP: Session Description Protocol](https://www.ietf.org/rfc/rfc2327.txt)."#,
            TwilioMiscellaneousError::ErrorCode90015 => r#"## ERROR - 90015

### Body and Template (Body, Sid, Language, Args) are provided

 Body and Template (Body, Sid, Language, Args) are provided together.

#### Possible Causes
Single create message request has Body and one of Template's field: Template or TemplateSid or TemplateLanguage or TemplateArgs

#### Possible Solutions
Choose which type of message would you like to use: templated message or plain message."#,
            TwilioMiscellaneousError::ErrorCode60004 => r#"## ERROR - 60004

### Invalid Configuration 

 

#### Possible Causes
The request was received but failed due to an invalid configuration.

#### Possible Solutions
- Check that the Account SID provided is correct.
- [Contact Twilio Support](https://www.twilio.com/help/contact) to confirm that access to this endpoint is enabled for the given Account SID."#,
            TwilioMiscellaneousError::ErrorCode51110 => r#"## Error - 51110

### Token contains multiple grants of same product

#### Possible Causes 
Duplication of grants for the same product.

#### Possible Solutions
Remove duplications of products from grants section in the token. 

To verify the token being passed, you can use the tools available at [jwt.io](https://jwt.io)."#,
            TwilioMiscellaneousError::ErrorCode22223 => r#"## ERROR - 22223

### Regulatory Bundle is not eligible to be Copied

 The Regulatory Bundle requested is not eligible to be Copied.

#### Possible Causes
The Regulatory Bundle is either in not in a twilio-approved status or does not have a valid_until date

#### Possible Solutions
Select a Regulatory Bundle that is eligible to be Copied by filtering the Regulatory Bundle list with a valid_until date and in the twilio-approved status."#,
            TwilioMiscellaneousError::ErrorCode13420 => r#"## Error - 13420 

### Play: Invalid Content-Type

Play requires an audio Content-Type

#### Possible Causes 
*  Play references a URL that does not return audio or is returning an invalid Content-Type.


#### Possible Solutions
*  Make certain the URL returns an audio file	
*   See the <a href='/docs/api/twiml/play'>Play Verb</a> API Reference for more information on valid Content-Types.	




"#,
            TwilioMiscellaneousError::ErrorCode30112 => r#"## ERROR - 30112

### Account is not found

 

#### Possible Causes
Wrong account is given

#### Possible Solutions
Specify the correct account"#,
            TwilioMiscellaneousError::ErrorCode51114 => r#"## Error - 51114

### Invalid access token header

#### Possible Causes 
The token has an invalid header.

#### Possible Solutions
Confirm a valid token is being passed in the request.

To check whether the access token is structurally correct, you can use the tools available at [jwt.io](https://jwt.io)."#,
            TwilioMiscellaneousError::ErrorCode21407 => r#"## Error - 21407 

### This Phone Number type does not support SMS

This phone number type does not support SMS.  You cannot set SmsUrl, SmsMethod, SmsFallbackUrl, or SmsFallbackMethod

"#,
            TwilioMiscellaneousError::ErrorCode21457 => r#"## Error - 21457

### AreaCode Parameter not Supported

The AreaCode parameter for provisioning numbers is not supported on the IncomingPhoneNumbers/Local & IncomingPhoneNumbers/TollFree sub-resources.





"#,
            TwilioMiscellaneousError::ErrorCode51004 => r#"## Error - 51004

### Client Connection: `endpoint_id` string too long

#### Possible Causes 

`endpoint_id` field in the Access Token is too long.

#### Possible Solutions

Consult the product documentation for maximum allowed length of `endpoint_id` field.
"#,
            TwilioMiscellaneousError::ErrorCode94303 => r#"## ERROR - 94303

### Transcriptions Configurations: Language is required

 A language is required to create a new configuration

#### Possible Causes
* "Language" value is blank
* "Language" value is null

#### Possible Solutions
* Specify a valid "language" as a BCP-47 code"#,
            TwilioMiscellaneousError::ErrorCode51118 => r#"## Error - 51118

### Invalid claim set

#### Possible Causes 
Claim set provided in token is invalid.

#### Possible Solutions
Verify the token has a valid claim set.

To check whether the token is structurally correct, you can use the tools available at [jwt.io](https://jwt.io)."#,
            TwilioMiscellaneousError::ErrorCode21607 => r#"## Error - 21607

### The 'From' number is not a valid, SMS-capable Twilio number

While your account is in Trial mode, you can only send SMS messages from your
free trial number, or the Twilio Sandbox Number. To remove this restriction,
[upgrade to a full account](/user/billing/upgrade).

"#,
            TwilioMiscellaneousError::ErrorCode21204 => r#"## Error - 21204 

### Call already initiated

You should never receive this error





"#,
            TwilioMiscellaneousError::ErrorCode19029 => r#"## ERROR - 19029

### When updating a channel at least one field should be updated

 No channel updates provided in the request

#### Possible Causes
User has not provided channel updates

#### Possible Solutions
When updating a channel at least one field should be updated"#,
            TwilioMiscellaneousError::ErrorCode13230 => r#"## Warning - 13230 

### Dial->Conference: Invalid muted value

muted must be either "true" or false".
"#,
            TwilioMiscellaneousError::ErrorCode13222 => r#"## Warning - 13222 

### Dial->Number: Invalid sendDigits value

sendDigits attribute may only contain the following characters "0123456789*#w". See the <a href='/docs/api/twiml/dial'>Dial Verb</a> API Reference for more information.





"#,
            TwilioMiscellaneousError::ErrorCode32112 => r#"## WARNING - 32112

### SIP: Invalid header value

 The header you asked Twilio to stamp was omitted because it contains illegal characters in its _value_. Including this header would have resulting in Twilio generating a message that runs contrary to the SIP specification.

#### Possible Causes
* If you specified headers in a SIP URI, invalid characters were most likely introduced by an escaped character. Escaped characters in SIP URIs are defined by a percent sign (%) followed by two hexadecimal digits specifying a character code.

#### Possible Solutions
* If you are generating SIP headers from user input, ensure your application properly accounts for non-alphanumeric characters such as punctuation and whitespace, as well as international characters."#,
            TwilioMiscellaneousError::ErrorCode20004 => r#"## Error - 20004 

### Method not allowed

The resource you accessed is valid, but the HTTP method you attempted to use is not supported for the resource.  For example, you cannot DELETE your own account.





"#,
            TwilioMiscellaneousError::ErrorCode13617 => r#"## Warning - 13617

### Record: Recording length is out of range for transcription

Twilio could not process the transcription because the duration of the recording is either too short or too long. The recording itself will not be affected. 


#### Possible Solutions
*  Transcription is currently limited to recordings with a duration greater than 2 seconds and less than 120 seconds. Consider using the maxLength parameter for `<Record>` to limit the duration of the recording. See <a href='/docs/api/twiml/record#attributes-transcribe'>transcription TwiML reference</a> for more information. 

"#,
            TwilioMiscellaneousError::ErrorCode10003 => r#"## ERROR - 10003

### Incoming call rejected due to inactive account

An incoming call was made to your application, but was not answered because your account was not active at the time. 

#### Possible Causes
*  Lack of funds
*  Violation of the Terms of Service or Acceptable Use Policy

#### Possible Solutions
*  Check your account balance.  Refill funds if necessary.	
*  Contact Twilio <a href="/help/contact">customer support</a> to inquire further."#,
            TwilioMiscellaneousError::ErrorCode19014 => r#"## ERROR - 19014

### Can fetch contact either by unique_customer_provided_id or channel 

 Contact fetch request should contain unique_customer_provided_id or channel, not both

#### Possible Causes
Contact fetch request contains both unique_customer_provided_id and channel

#### Possible Solutions
Contact can be fetched either by unique_customer_provided_id or by channel"#,
            TwilioMiscellaneousError::ErrorCode21403 => r#"## Error - 21403 

### Invalid Method

You attempted to update the HTTP method that Twilio uses when somebody calls your phone number, but the <span class='rest-attribute'>Method</span> you supplied was not valid.  Valid <span class='rest-attribute'>Method</span> values are GET or POST.





"#,
            TwilioMiscellaneousError::ErrorCode21473 => r#"## Error - 21473

### AccountSid you are transferring to is not related to the originating owner of the phone number

The AccountSid you specified is neither parent nor child of the originating Account owner.  Please make sure that the subaccount or parent account is related to the current owner you are transferring from or to.





"#,
            TwilioMiscellaneousError::ErrorCode90030 => r#"## ERROR - 90030

### Broadcast 'CorrelationId' is too long

 Broadcast 'CorrelationId' is too long

#### Possible Causes
'CorrelationId' field has too long length

#### Possible Solutions
Use shorter 'CorrelationId' valuer"#,
            TwilioMiscellaneousError::ErrorCode19022 => r#"## ERROR - 19022

### Invalid channel

 Request with a channel that is not supported

#### Possible Causes
Channel other than: email, sms, voice, messenger, whatsapp

#### Possible Solutions
Only email, sms, voice, messenger or whatsapp are valid channels"#,
            TwilioMiscellaneousError::ErrorCode14213 => r#"## Error - 14213 

### Dial->Queue: queue name too long

The name of the queue must be shorter than 64 characters.
"#,
            TwilioMiscellaneousError::ErrorCode21478 => r#"## Error - 21478

### Unable to update Status for subaccount, subaccount has been suspended by Twilio

Unable to update Status for subaccounts that have been suspended externally by Twilio.  Please contact customer support: http://www.twilio.com/help

"#,
            TwilioMiscellaneousError::ErrorCode30016 => r#"## Error - 30016

### 'To' and 'From' channel types are incompatible

The To or From attributes have channel prefixes what are incompatible

#### Possible Causes 
*  Attempt to send a message to one channel but use 'From' address for another channel

#### Possible Solutions
*  Check the To and From attributes channel prefixes"#,
            TwilioMiscellaneousError::ErrorCode20422 => r#"## Error - 20422

### Invalid Parameter

The request contained malformed or semantically incorrect parameters.

Common occurrences:

* Lacking a specified `Content-Type` header field
* Invalid or missing XML data
* Invalid parameter type or value

#### Is the request safe to retry?

A 422 request is never processed and is always safe to retry."#,
            TwilioMiscellaneousError::ErrorCode60005 => r#"## ERROR - 60005

### Downstream Carrier Error

 

#### Possible Causes
The request was received but failed during processing due to an error while communicating with the carrier.

#### Possible Solutions
- Retry request to rule out transient carrier connection issues. 
- If error persists, [contact Twilio Support](https://www.twilio.com/help/contact) to rule out integration issues."#,
            TwilioMiscellaneousError::ErrorCode21218 => r#"## Error - 21218

### Invalid ApplicationSid

You attempted to initiate an outbound phone call with an invalid ApplicationSid.
The application may not exist anymore or may not be available within your
account.
"#,
            TwilioMiscellaneousError::ErrorCode20021 => r#"## ERROR - 20021

### Phone number rejected by T-Mobile SDG Service Provisioning API

 Exception received while creating application address in SDG Service Provisioning API: Invalid short code And enum server status is: FAILURE

#### Possible Causes
* The phone number was rejected by T-Mobile SDG Service Provisioning API

#### Possible Solutions
* Check the phone number"#,
            TwilioMiscellaneousError::ErrorCode90027 => r#"## ERROR - 90027

### Broadcast 'FriendlyName' is too long

 Broadcast 'FriendlyName' is too long

#### Possible Causes
'FriendlyName' field has too long length

#### Possible Solutions
Use shorter string as Broadcast 'FriendlyName'"#,
            TwilioMiscellaneousError::ErrorCode80606 => r#"## Warning

### Proxy Identifier Not Owned By Account

Example Message: Proxy identifier not owned by account."#,
            TwilioMiscellaneousError::ErrorCode14204 => r#"## Error - 14204

### Enqueue: Queue name too short

The queue name must be at least one character long.
"#,
            TwilioMiscellaneousError::ErrorCode94304 => r#"## ERROR - 94304

### Transcriptions Configurations: Language is invalid

 Invalid language value, either to create a new configuration or update existing one. Language should be a BCP-47 code

#### Possible Causes
* "Language" value is not a BCP-47 code

#### Possible Solutions
* Specify a valid "language" as a BCP-47 code"#,
            TwilioMiscellaneousError::ErrorCode13252 => r#"## Warning - 13252

### Dial: Invalid header name

You named your header something invalid.

"#,
            TwilioMiscellaneousError::ErrorCode21207 => r#"## Error - 21207 

### Invalid IfMachine

You attempted to initiate an outbound phone call, but you sent an invalid <span class='rest-attribute'>IfMachine</span> parameter.  Twilio can determine if an answering machine or voicemail has answered the call, and behave differently if it has.  Valid options for <span class='rest-attribute'>IfMachine</span> are "Continue" (continue with the call, progressing after the beep), "Hangup" (immediately hangup the call) or "False" (don't determine if a machine has answered).





"#,
            TwilioMiscellaneousError::ErrorCode21503 => r#"## Error - 21503 

### Invalid transcription type

The transcription value you provided is invalid.  You may use one of: "true" or "false".





"#,
            TwilioMiscellaneousError::ErrorCode70004 => r#"## Error - 70004

### Unauthorized
The Request does not contain any authorization information (most cases: credentials) or the credentials and the resource have no relation.

#### Possible Causes 
* When omitting the Auth header from HTTP request
* Passing an Auth header to access a resource that belongs to some other account.
"#,
            TwilioMiscellaneousError::ErrorCode13212 => r#"## Warning - 13212 

### Dial: Invalid timeout value

timeout must be a positive integer, in seconds.  See the <a href='/docs/api/twiml/dial#timeout'>Dial Verb</a> API Reference for more information.





"#,
            TwilioMiscellaneousError::ErrorCode21472 => r#"## Error - 21472

### Account is not active

The AccountSid you specified is not an active Twilio Account.  To transfer the account, you must supply an Active 34 character AccountSid .





"#,
            TwilioMiscellaneousError::ErrorCode21203 => r#"## Error - 21203 

### International calling not enabled

You attempted to initiate an outbound phone call to an international phone number, which is not currently supported.





"#,
            TwilioMiscellaneousError::ErrorCode51105 => r#"## Error - 51105

### Token doesn't contain identity

#### Possible Causes 
The token doesn't have an identity specified.

#### Possible Solutions
Confirm that there is an identity field in the token. To check whether the access token is structurally correct, you can use the tools available at [jwt.io](https://jwt.io)."#,
            TwilioMiscellaneousError::ErrorCode21701 => r#"## ERROR - 21701

### The Messaging Service does not exist

 The Messaging Service resource you are referencing does not exist or does not match the `Account SID` in the API Request.


#### Possible Causes
* The Messaging Service resource you are referencing does not exist.
* The Messaging Service resource you are referencing does not belong to the Account SID in your API Request.


#### Possible Solutions
*  Double check the `Messaging Service SID` and `Account SID` parameters that you are using. You can find the Messaging Service SID in the [Twilio Console](https://console.twilio.com/us1/develop/sms/services) (beginning with MG)."#,
            TwilioMiscellaneousError::ErrorCode21239 => r#"## ERROR - 21239

### Maximum Credential Lists Reached

An account cannot have more than 100 IP Access Control Lists. 

#### Possible Causes
* An account cannot have more than 100 IP Access Control Lists.

#### Possible Solutions
* Delete an existing IP Access Control List if you must create more."#,
            TwilioMiscellaneousError::ErrorCode51201 => r#"Too many connection requests for this account."#,
            TwilioMiscellaneousError::ErrorCode52116 => r#"## WARNING - 52116

### Legacy FCM server key credential used to send notifications

 Google will deprecate the legacy FCM API in June 2024: https://help.twilio.com/articles/20768292997147-Updating-Twilio-Push-for-FCM-HTTP-v1-API

#### Possible Causes
Detected the use of a legacy FCM server key credential. This credential will stop working after Google deprecates the legacy FCM API in June 2024. Your action is required in order for FCM push notifications to continue to work after the legacy FCM API is deprecated.

#### Possible Solutions
Use a new FCMv1 credential before June 2024 to keep sending push notifications. See instructions linked in the description above."#,
            TwilioMiscellaneousError::ErrorCode20423 => r#"## Error - 20423

### Invalid SID

The request contained at least one parameter what is incorrect SID but has to be.

#### Is the request safe to retry?

A 422 request is never processed and is always safe to retry."#,
            TwilioMiscellaneousError::ErrorCode14221 => r#"## Error - 14221

### Enqueue: Provided Attributes JSON was not valid

Provided Attributes is not valid JSON.
"#,
            TwilioMiscellaneousError::ErrorCode11243 => r#"## Error - 11243

### HTTP retry policy is invalid
One or more retry policies specified in the ‘rp’ parameter of your connection settings are invalid.

#### Possible Solutions
*  Correct the 'rp' (retry policy) parameter in the fragment part of the URL, the one after '#'."#,
            TwilioMiscellaneousError::ErrorCode13611 => r#"## Warning - 13611 

### Record: Invalid timeout value

timeout must be a positive integer, in seconds.  See the <a href='/docs/api/twiml/record#timeout'>Record Verb</a> API Reference for more information.





"#,
            TwilioMiscellaneousError::ErrorCode20012 => r#"## ERROR - 20012

### Invalid TLSv1.2 Cipher Suite

Request is not configured with a PCI compliant cipher suite ## Error Code 20012

### Invalid TLSv1.2 Cipher Suite

The request was rejected because it was not correctly configured to use TLSv1.2. All new accounts are [required to use TLSv1.2 with supported ciphers suites](https://support.twilio.com/hc/en-us/articles/360007724794). 

#### Possible Solutions:
* Reconfigure request to use a supported cipher suite for TLSv1.2
* Review these [tips for upgrading your environment to TLSv1.2](https://support.twilio.com/hc/en-us/articles/360006751753)

#### Supported Cipher Suites:
* TLS_ECDHE_RSA_WITH_AES_128_GCM_SHA256 (ecdh_x25519) - A
* TLS_ECDHE_RSA_WITH_AES_128_CBC_SHA256 (ecdh_x25519) - A
* TLS_ECDHE_RSA_WITH_AES_128_CBC_SHA (ecdh_x25519) - A
* TLS_ECDHE_RSA_WITH_AES_256_GCM_SHA384 (ecdh_x25519) - A
* TLS_ECDHE_RSA_WITH_AES_256_CBC_SHA384 (ecdh_x25519) - A
* TLS_ECDHE_RSA_WITH_AES_256_CBC_SHA (ecdh_x25519) - A
* TLS_RSA_WITH_AES_128_GCM_SHA256 (rsa 2048) - A
* TLS_RSA_WITH_AES_128_CBC_SHA256 (rsa 2048) - A
* TLS_RSA_WITH_AES_128_CBC_SHA (rsa 2048) - A
* TLS_RSA_WITH_AES_256_GCM_SHA384 (rsa 2048) - A
* TLS_RSA_WITH_AES_256_CBC_SHA256 (rsa 2048) - A
* TLS_RSA_WITH_AES_256_CBC_SHA (rsa 2048) - A
* TLS_RSA_WITH_3DES_EDE_CBC_SHA (rsa 2048) - C

#### Possible Causes
Supported Cipher Suites list is not reflecting the reality.

#### Possible Solutions
Changed with the real list which is checked "#,
            TwilioMiscellaneousError::ErrorCode90016 => r#"## ERROR - 90016

### 'Template' or 'TemplateSid'/'TemplateLanguage' is required to send a Template Message

 'Template' or 'TemplateSid'/'TemplateLanguage' is required to send a Template Message

#### Possible Causes
Some template parameters are provided but Template (or TemplateSid) itself is not provide.

#### Possible Solutions
Provide Template or TemplateSid&TemplateLanguage in the create message request"#,
            TwilioMiscellaneousError::ErrorCode21501 => r#"## Error - 21501 

### Resource not available

The resource you requested is not (yet) available.

#### Possible Causes 
*  Most likely you're fetching a Recording resource that is temporarily performing some post-processing, and will be available momentarily.


#### Possible Solutions
*  Retry the HTTP request again.  The Retry-After header will tell you when to retry.  Within a couple seconds should be ample.	




"#,
            TwilioMiscellaneousError::ErrorCode19010 => r#"## ERROR - 19010

### Invalid contact search request

 

#### Possible Causes
The search parameter provided is not valid

#### Possible Solutions
Review the API documentation for more information"#,
            TwilioMiscellaneousError::ErrorCode32209 => r#"## ERROR - 32209

### SIP: Secure transport required

When secure calling is enabled, Twilio cannot accept calls made using UDP or TCP. Please use TLS. 

#### Possible Causes
* Media security is enabled on your SIP domain or trunk, but Twilio received SIP traffic over an insecure protocol, such as UDP or TCP.
* You are using SIP registration, and are attempting to place a secure call to an endpoint where all of the available bindings were registered with an insecure protocol, such as UDP or TCP.

#### Possible Solutions
* Please ensure your PBX or end device is configured to send SIP traffic over the TLS transport.
* If you are using SIP registration, your device must be configured to *register* over the TLS protocol. Devices registered over insecure protocols will not be attempted for secure calls."#,
            TwilioMiscellaneousError::ErrorCode12301 => r#"## Error - 12301

### Invalid Upload Content-Type

Twilio is unable to process the Content-Type of the uploaded file. Please make sure the uploaded file is valid and supported.

#### Possible Causes
* Unsupported file format
* File is malformed

"#,
            TwilioMiscellaneousError::ErrorCode80602 => r#"## Warning

### Non Unique Service Name

Example Message: Service UniqueName must be unique."#,
            TwilioMiscellaneousError::ErrorCode20500 => r#"## Error - 20500 

### Internal Server Error

The Twilio API encountered an error when processing your request. This generally indicates an error in the server handling logic or a timeout in the API. We apologize for the inconvenience.

#### I'm seeing a lot of these

If you are seeing a consistent pattern of 500 server errors coming from the Twilio API, check our [status page](http://status.twilio.com/) for more details. If our status page has no information, [contact support](https://www.twilio.com/help/contact) with details about the requests that are failing, and we will investigate.

#### Is the request safe to retry?

GET and DELETE requests are always safe for you to retry as they are idempotent. Some POST requests are idempotent by nature - purchasing a specific phone number, or hanging up a call - and while these may return different HTTP status codes on each attempt, the end state of the system will be the same whether you make the request one time or ten.

Other POST requests - sending an SMS or triggering an outbound call - are not idempotent. If you get a 500 Server Error on these requests, and you retry the request, it is possible for a customer to receive multiple messages or calls from your application."#,
            TwilioMiscellaneousError::ErrorCode90013 => r#"## ERROR - 90013

### 'Template' or 'TemplateSid'&'TemplateLanguage' or 'MediaUrls' is required

 'Template' or 'TemplateSid'&'TemplateLanguage' or 'MediaUrls' is required to send Broadcast

#### Possible Causes
All next attributes are missing: 'Template', 'TemplateSid', 'TemplateLanguage', 'MediaUrls'

#### Possible Solutions
Use one of the attributes 'Template', 'TemplateSid', 'TemplateLanguage', 'MediaUrls' to specify Broadcast content"#,
            TwilioMiscellaneousError::ErrorCode21620 => r#"## ERROR - 21620

### Invalid media URL(s)

One or more media URL you provided is invalid. Please use only valid HTTP and HTTPS URLs. One or more media URLs that you provided is invalid — they are malformed in some way. 

Typically, this error is issued when a URL lacks a protocol specifier prefix, i.e., `htttp://` or `https://`.

#### Possible Causes
The URL(s) lack a protocol specifier: `http://` or `https://`

#### Possible Solutions
Add a correct protocol specifier prefix: `http://` or `https://`"#,
            TwilioMiscellaneousError::ErrorCode51003 => r#"## Error - 51003

### Client Connection: `identity` too long

#### Possible Causes 

`identity` field in the Access Token is too long.

#### Possible Solutions

Consult the product documentation for maximum allowed length of `identity` field.
"#,
            TwilioMiscellaneousError::ErrorCode80903 => r#"## Warning

### Unknown Participant

Example Message: Unknown Participant"#,
            TwilioMiscellaneousError::ErrorCode21608 => r#"## Error - 21608

### This number can send messages only to verified numbers

You have attempted to send a message to an unverified phone number while using a trial account.

To prevent spam, trial accounts can only send messages to phone numbers you've verified with Twilio. You must upgrade your account to remove this restriction.

### Possible solutions
* Verify the phone number you wish to reach via the [Twilio console](https://www.twilio.com/console/phone-numbers/verified). See step-by-step instructions in our [guide to working with your trial account](https://www.twilio.com/docs/usage/tutorials/how-to-use-your-free-trial-account#verify-your-personal-phone-number). _Note that you will need access to this device to receive the call or text with your verification code._
*  Upgrade your Twilio trial account to a full paid account to remove this restriction by providing your payment information via credit card or Paypal on the [console billing page](https://www.twilio.com/console/billing). 
"#,
            TwilioMiscellaneousError::ErrorCode94500 => r#"## ERROR - 94500

### Transcriptions: sourceSid invalid

 The action could not be performed because the sourceSid provided is not valid

#### Possible Causes
* sourceSid value is invalid
* sourceSid is not a call recording sid

#### Possible Solutions
 * Specify a sourceSid of a valid call recording"#,
            TwilioMiscellaneousError::ErrorCode21220 => r#"## Error - 21220 

### Invalid call state

You attempted a realtime operation on a Call that was not in-progress.  Calls that have completed cannot be recorded or redirected.  





"#,
            TwilioMiscellaneousError::ErrorCode503 => r#"## ERROR - 503

### Internal Error

  An internal error has occurred while processing the request.


#### Possible Causes
* The server is currently unable to handle the request.


#### Possible Solutions
* Retry the request at a later time."#,
            TwilioMiscellaneousError::ErrorCode70052 => r#"## Error - 70052 

### Public Key Client Validation Required For Account
The account is only allowed to perform Public Key Client Validation requests at Twilio. All other types of authentication are disabled for the account.

#### Possible Causes 
* The Twilio-Client-Validation is not present in the request.

#### Possible Solution:
* Ensure the Twilio-Client-Validation header is present in the request and valid.
* Disable forcing Client Validation feature at the Console if not needed.
"#,
            TwilioMiscellaneousError::ErrorCode13233 => r#"## Error - 13233

### Dial->Conference: Invalid waitUrl value

waitUrl must be either a well formed URL or the empty string ""
"#,
            TwilioMiscellaneousError::ErrorCode21613 => r#"## Error - 21613

### PhoneNumber Requires an Address

Local regulation requires Twilio to have an address on file for your account to
purchase this phone number.  You must add an address that specifies the end
user’s location using our [API][addresses-api] or [account
portal][addresses-portal]. Alternatively, you can provision a phone number that
does not require an address by using Basic List Filters on the
[AvailablePhoneNumbers][phone-numbers] resource. If you received this error
when trying to transfer a phone number to a subaccount or between subaccounts,
please ensure that the subaccount has the correct address on file.

[addresses-api]: /docs/api/rest/addresses
[addresses-portal]: /user/account
[phone-numbers]: /docs/api/rest/available-phone-numbers
"#,
            TwilioMiscellaneousError::ErrorCode21502 => r#"## Error - 21502 

### Invalid callback url

The callback url provided is not a valid URL.

#### Possible Solutions

Make sure you submit a fully qualified URL including:

*   protocol (http:// or https://)
*   hostname
*   file path
*   properly url-encoded query parameters

Twilio must be able to reach this URL over the public Internet.


"#,
            TwilioMiscellaneousError::ErrorCode21231 => r#"# Error - 21231

### Domain Validation Error

One of the parameters passed when creating or updating a SIP domain was incorrect.  This may be caused by the IP Access Control List SID already being associated with the Domain.  
"#,
            TwilioMiscellaneousError::ErrorCode51130 => r#"## Error - 51130

### Token is invalid!"#,
            TwilioMiscellaneousError::ErrorCode51116 => r#"## Error - 51116

### Invalid access token grants

#### Possible Causes 
Provided token has invalid grants.

#### Possible Solutions
Confirm valid grants are being passed in the token.

To check whether the token is structurally correct, you can use the tools available at [jwt.io](https://jwt.io)."#,
            TwilioMiscellaneousError::ErrorCode51006 => r#"## Warning - 51006

### Client Connection: Connection expired

#### Possible Causes 

The connection has expired. The connection expiration is set by the `exp` field of the Access Token.

#### Possible Solutions

Make sure clients refresh Access Tokens before they expire. Please, refer to the Twilio SDK documentation for further details.

"#,
            TwilioMiscellaneousError::ErrorCode21101 => r#"None"#,
            TwilioMiscellaneousError::ErrorCode70104 => r#"## Error - 70104

### Invalid Public Key
The Request tries to store a Public Key in Twilio. This public key cannot be parsed. Only supported key format is X.509 PEM. The prefix and suffix (-----BEGIN PUBLIC KEY-----) of the key can be present (but not mandatory).

#### Possible Solution:
Make sure the public key is correct in the request:
* The key is in X.509 PEM format
* There are no copy paste errors
* All the parts of the key are present in the request."#,
            TwilioMiscellaneousError::ErrorCode13313 => r#"## Warning - 13313 

### Gather: Invalid timeout value

timeout must be a positive integer, in seconds.  See the <a href='/docs/api/twiml/gather#timeout'>Gather Verb</a> API Reference for more information.





"#,
            TwilioMiscellaneousError::ErrorCode20007 => r#"## Error - 20007

### Page size too large

You've requested a page size that is too large. The maximum page size is 1000.

#### Possible solutions

* Decrease the page size in your request.
"#,
            TwilioMiscellaneousError::ErrorCode90017 => r#"## ERROR - 90017

### 'Template' and 'TemplateSid'/'TemplateLanguage' must not be specified together

 'Template' and 'TemplateSid'/'TemplateLanguage' must not be specified together for template message

#### Possible Causes
'Template' and 'TemplateSid'/'TemplateLanguage' are specified together for template message

#### Possible Solutions
Choose which template message to use Direcy('Template') or Indirect('TemplateSid')"#,
            TwilioMiscellaneousError::ErrorCode11202 => r#"## Error - 11202
### TCP connection refused
Twilio cannot establish a TCP connection to your web server. Your web server rejected our two connection attempts.

#### Possible Causes
* invalid port number in the HTTP URL
* your web server process is down so no process is listening on the port
* your web server is in a degraded state and is not accepting new connections (e.g., overloaded)

#### Possible Solutions
* verify the HTTP URL provided has the correct host and port number of your web server
* verify your web server is running and listening on the correct port
* verify your web server is properly provisioned to handle all requests"#,
            TwilioMiscellaneousError::ErrorCode14101 => r#"## Warning - 14101 

### "To" Attribute is Invalid

The To attribute does not appear to be a phone number that you can SMS

#### Possible Causes 
*  Invalid formatted number
*  SMSing an un-validated number while using a Trial Account


#### Possible Solutions
*  Check the number and make sure it is a valid 10 digit domestic phone number  
*  Confirm you have validated this number, if you are still using a Trial account, or upgrade your account
*  Contact Twilio <a href="/help/contact">customer support</a> to inquire further.
"#,
            TwilioMiscellaneousError::ErrorCode30012 => r#"## Error - 30012

## Message Time To Live is too small

### Possible Causes 
TTL specified in the request is too small

### Possible Solutions
Specify a bigger TTL
"#,
            TwilioMiscellaneousError::ErrorCode94200 => r#"## ERROR - 94200

### Transcriptions Settings: Invalid encryptionEnabled value

 Invalid "encryptionEnabled" value. It should be either true or false

#### Possible Causes
* "encryptionEnabled" value is invalid

#### Possible Solutions
* Set "encryptionEnabled" value to true to enable encryption for transcriptions
* Set "encryptionEnabled" value to false to disable encryption for transcriptions"#,
            TwilioMiscellaneousError::ErrorCode21201 => r#"## Error - 21201 

### No 'To' number specified

You attempted to initiate an outbound phone call, but you did not specify what number to call in the 'To' POST parameter
"#,
            TwilioMiscellaneousError::ErrorCode13510 => r#"## Warning - 13510 

### Say: Invalid loop value

The value of the loop attribute must be an integer greater or equal to zero. See the <a href='/docs/api/twiml/say#loop'>Play Verb</a> API Reference for more information.





"#,
            TwilioMiscellaneousError::ErrorCode90010 => r#"This account is currently not in an active state

#### Possible causes

* Account is suspended
"#,
            TwilioMiscellaneousError::ErrorCode60003 => r#"## ERROR - 60003

### End User Data is Not Available

 

#### Possible Causes
The request was received and processed successfully but data is not available for the requested end user.

#### Possible Solutions
Do not retry request for this end user."#,
            TwilioMiscellaneousError::ErrorCode13614 => r#"## Warning - 13614 

### Record: Invalid transcribe value

The transcribe attribute of Record must be TRUE or FALSE. See the <a href='/docs/api/twiml/record#transcribe'>Record Verb</a> API Reference for more information.





"#,
            TwilioMiscellaneousError::ErrorCode70155 => r#"## Error - 70155

### Request Is Missing Required HTTP Headers
The JWT received by Twilio along with the request has defined a set of HTTP headers that must be validated. The actual request is missing at least one of those headers. This is a Public Key Client Validation Error.

#### Possible Solution:
* Make sure you are using the latest Twilio helper library.
* Make sure the JWT does not declare unnecessary headers.
* For the details on JWTs used as part of Public Key Client Validation, see [the documentation](/docs/api/credentials/public-key-client-validation-getting-started)"#,
            TwilioMiscellaneousError::ErrorCode90037 => r#"## ERROR - 90037

### Broadcast has too many 'CorrelationId' items

 Broadcast has too many 'CorrelationId' items

#### Possible Causes
To many 'CorrelationId' items in the request

#### Possible Solutions
Use no more than allowed number of 'CorrelationId' fields"#,
            TwilioMiscellaneousError::ErrorCode21226 => r#"## Warning - 21226

### SIP calling not enabled for this account

Your account is not enabled for SIP calling."#,
            TwilioMiscellaneousError::ErrorCode21255 => r#"## ERROR - 21255

### Maximum IP Access Control Lists reached

You cannot have more than 1000 IP Access Control Lists. 

#### Possible Causes
* You cannot have more than 1000 IP Access Control Lists.

#### Possible Solutions
* Remove current IP Access Control Lists if you must create new ones."#,
            TwilioMiscellaneousError::ErrorCode19011 => r#"## ERROR - 19011

### When updating a contact, invalid JSON syntax or invalid field that cannot be updated by this endpoint

 Invalid JSON or fields other than first_name, middle_name, last_name, legal_name, preferred_name, unique_customer_provided_id or custom_fields exist in update contact request

#### Possible Causes
User provided an invalid JSON or trying to update fields other than first_name, middle_name, last_name, legal_name, preferred_name, unique_customer_provided_id or custom_fields of a contact

#### Possible Solutions
Fix JSON input or change update contact request to update only first_name, middle_name, last_name, legal_name, preferred_name, unique_customer_provided_id or custom_fields"#,
            TwilioMiscellaneousError::ErrorCode19031 => r#"## ERROR - 19031

### Maximum number of locations allowed reached 

 Maximum number of locations allowed reached

#### Possible Causes
Attempt to create more than 5 locations

#### Possible Solutions
Remove an existing location to add a new one"#,
            TwilioMiscellaneousError::ErrorCode94401 => r#"## ERROR - 94401

### Transcriptions: audio processing error

 Twilio failed to transcribe audio due to an issue with the transcription provider.  The transcription files have been deleted and the resource has been marked as “failed”

#### Possible Causes
* The speech-to-text provider failed to transcribe audio

#### Possible Solutions
 "#,
            TwilioMiscellaneousError::ErrorCode70001 => r#"## Error - 70001

### Validation Failed
The Request body is syntactically correct, the body was successfully parsed, but the data in the body violates the constraints.

#### Possible Causes 
* Performing an update to the “FriendlyName” with a string longer than the allowed 64 characters.
* Updating a field that cannot have a null value to a null."#,
            TwilioMiscellaneousError::ErrorCode90041 => r#"## ERROR - 90041

### Broadcast 'MediaUrl' field is too long

 Broadcast 'MediaUrl' field is too long

#### Possible Causes
One of 'MediaUrl' fields is too long

#### Possible Solutions
Use shorter 'MediaUrl' value"#,
            TwilioMiscellaneousError::ErrorCode80906 => r#"## Warning

### Interaction Not Open

Example Message: Received interaction completion event for an Interaction that is already complete"#,
            TwilioMiscellaneousError::ErrorCode51125 => r#"## Error - 51125

### Too many updates

#### Possible Causes 
Number of updates for this account is higher than expected!

#### Possible Solutions
Verify that your application is not misbehaving and the load is expected. If everything is as expected then contact Customer Support to increase your account's limits."#,
            TwilioMiscellaneousError::ErrorCode13226 => r#"## Warning - 13226 

### Dial: Invalid country code

The country code of the provided phone number is unknown





"#,
            TwilioMiscellaneousError::ErrorCode70251 => r#"## Error - 70251

### Bad SSO Settings
The Request tries to authenticate through SSO. The request is well-formed, but the configured settings for the SSO is not valid.

#### Possible Causes 
* Malformed urls
* Missing certificates

#### Possible Solution
* Reconfigure the SSO settings and retry."#,
            TwilioMiscellaneousError::ErrorCode21409 => r#"## Error - 21409

### VoiceCallerIdLookup cannot be set for this phone number

You attempted to set the VoiceCallerIdLookup for a phone number using the REST API, but the number does not support this feature.
"#,
            TwilioMiscellaneousError::ErrorCode21422 => r#"## Error - 21422

### PhoneNumber is not available

You tried to purchase a phone number that is not available for purchase. 

This error can be triggered in a few ways:

- Twilio does not own the phone number in question.
- Twilio owns the number, but it is reserved for another account.

"#,
            TwilioMiscellaneousError::ErrorCode51126 => r#"## Error - 51126

### DNC limit has been reached"#,
            TwilioMiscellaneousError::ErrorCode32006 => r#"## Error - 32006

### SIP: Too many hops

#### Possible Causes 
*  The request was rejected because its `Max-Forward` header had reached zero.

#### Possible Solutions
*  Ensure your infrastructure will send a minimum value of 20 for `Max-Forward` to ensure your call is processed successfully."#,
            TwilioMiscellaneousError::ErrorCode19027 => r#"## ERROR - 19027

### Invalid Channel Description

 The channel description provided in the request is invalid

#### Possible Causes
Channel description is over 100 chars

#### Possible Solutions
Channel description must be under 100 chars"#,
            TwilioMiscellaneousError::ErrorCode13615 => r#"## Warning - 13615 

### Record: transcribeCallback URL is invalid

The transcribeCallback URL provided in the `<Record>` verb is not valid.


#### Possible Solutions
Make sure you submit a fully qualified URL including:

*   protocol (http:// or https://)
*   hostname
*   file path
*   properly url-encoded query parameters

Twilio must be able to reach this URL over the public Internet.
 	




"#,
            TwilioMiscellaneousError::ErrorCode11242 => r#"## Error - 11242

### HTTP connection over Twilio Interconnect is not allowed

Your account has not been enabled for HTTP connections over Twilio Interconnect.

#### Possible Solutions
* Correct the 'r' (region) parameter in the fragment part of the URL, the one after '#'. Remove those with the '-ix' suffix.
* Contact Twilio <a href="/help/contact">customer support</a> to inquire further.
"#,
            TwilioMiscellaneousError::ErrorCode19021 => r#"## ERROR - 19021

### Only one channel can be set as primary

 is_primary can be set to true only for one channel

#### Possible Causes
Request with is_primary set to True for more than one channel 

#### Possible Solutions
Ensure only each channel has only 1 is_primary set to true"#,
            TwilioMiscellaneousError::ErrorCode21211 => r#"## ERROR - 21211

### Invalid 'To' Phone Number

You attempted to initiate an outbound phone call or send a message, but the `To` phone number you supplied was not a valid phone number or was incorrectly formatted. Twilio accepts phone numbers in [E164 format](http://en.wikipedia.org/wiki/E.164): `[+] [country code] [subscriber number including area code]`. You attempted to initiate an outbound phone call or send a message, but the `To` phone number you supplied was not a valid phone number or was incorrectly formatted. Twilio accepts phone numbers in [E164 format](http://en.wikipedia.org/wiki/E.164): `[+] [country code] [subscriber number including area code]`.

#### Possible Causes
* The formatting of the `To` number you supplied was invalid.
* You attempted to send a message or place a call from a Twilio number to itself (i.e. putting the same Twilio number in the `To` and `From` parameters).
* You attempted to send a message to Short Code.
* You attempted to send a message to an Alphanumeric Sender ID.
* You attempted to send a message to a `To` number with an invalid prefix using a Messaging Service.

#### Possible Solutions
* Ensure you have formatted your phone numbers in [E164
format](http://en.wikipedia.org/wiki/E.164): `[+] [country code] [subscriber number including area code]`.
* Ensure you have used the correct [country calling code](https://en.wikipedia.org/wiki/List_of_country_calling_codes#Alphabetical_listing_by_country_or_region) for the phone number you are calling.
* Ensure you are not attempting to call or message from a Twilio number to itself.
* Ensure you are not attempting to send messages to Short Codes or Alphanumeric Sender IDs. These sender types cannot receive messages from Twilio numbers.
* Review the 'To' number for any typos or incorrect input methods.
* If you are using a third party integration, test your API request in the [Try it out](https://console.twilio.com/us1/develop/sms/try-it-out/send-an-sms) section of the Twilio console. If this is successful, you may need to contact the third party to ensure they are not making unexpected changes to your `To` parameter.
* When sending messages using a Messaging Service, instead of formatting a `To` number with an invalid prefix, remove the prefix and send directly to the `To` number using [E164
format](http://en.wikipedia.org/wiki/E.164): `[+] [country code] [subscriber number including area code]`. If the `To` number is able to accept RCS messages and there is an RCS Agent available in the Messaging Service Sender Pool, it will be selected to send to the given `To` number.
"#,
            TwilioMiscellaneousError::ErrorCode21705 => r#"## ERROR - 21705

### The Messaging Service is invalid

The Messaging Service Sid you are using is not valid 





#### Possible Causes
* The MessagingServiceSid provided with the request does not match a valid Sid 
   pattern(MGxxx).

#### Possible Solutions
* Double check the Messaging Service Sid parameter that you are using.
* Find a specific Messaging Service Sid and manage your Messaging Services in the 
  [Twilio Console](https://www.twilio.com/console/sms/services)."#,
            TwilioMiscellaneousError::ErrorCode52004 => r#"## ERROR - 52004

### Credential SID not specified

Credential SID not specified in binding, token or service Credential SIDs are used to determine which Credential resource to use to send a given push notification (APNS, FCM or GCM). For each delivery attempt the system tries to select a Credential SID depending on the API request and configuration. This error indicates that the system did not find any available Credential SID to use and hence failed to send the delivery attempt.

#### Possible Causes
Configuration is slightly different depending on which Twilio API and product you are using.

*  Notify: Credential SID can be specified in the Notify Service or in the Binding. This error indicates neither of those were specified.

*  Verify Push: Credential SID can be specified in the Verify Service in the `push` configuration. This error indicates the service doesn't have this configuration.

*  Chat, Voice, Verify, Lookup: Credential SID can be specified in the Twilio Access Token used to register for push notifications. This error indicates that the token missed that information.


#### Possible Solutions
*  Notify: Set the Credential SID either in the Notify Service or in the Create Binding request
*  Verify Push: Set the FCM/APN Credential SID in the Verify Service
*  Chat, Voice, Verify, Lookup: Set the Credential SID in the Access Token"#,
            TwilioMiscellaneousError::ErrorCode13610 => r#"## Warning - 13610 

### Record: Invalid method value

The method attribute of Record must be GET or POST. See the <a href='/docs/api/twiml/record#method'>Record Verb</a> API Reference for more information.





"#,
            TwilioMiscellaneousError::ErrorCode21236 => r#"## ERROR - 21236

### IP Access Control List Dependencies Violation

You cannot delete an IP Access Control List that is still mapped to a SIP Domain. 

#### Possible Causes
* Your IP Access Control List is still mapped to a SIP Domain.

#### Possible Solutions
* Remove the IP Access Control list from your SIP Domain before trying to delete it."#,
            TwilioMiscellaneousError::ErrorCode21604 => r#"## ERROR - 21604

### The destination 'To' phone number is required to send an SMS

Make sure to specify a valid phone number as the recipient of the SMS. ## Error - 21604

### 'To' phone number is required to send a Message

Make sure to specify a valid phone number as the recipient of the message.



#### Possible Causes
- 'To' phone number is not specified
- Your request is not formatted as `application/x-www-form-urlencoded`

#### Possible Solutions
- Make sure to include a 'To' phone number
- Check that you are submitting your request as `application/x-www-form-urlencoded` and not as `json`."#,
            TwilioMiscellaneousError::ErrorCode19030 => r#"## ERROR - 19030

### Invalid location type

 Request with an invalid location type

#### Possible Causes
Location type other than: work, home, other

#### Possible Solutions
Only use work, home, other as location type"#,
            TwilioMiscellaneousError::ErrorCode94002 => r#"## ERROR - 94002

### Transcriptions: configuration not found

 Twilio failed to process the request to transcribe audio because configuration was not found. No transcription resource has been created

#### Possible Causes
* Transcription Configuration was not specified
* Transcription Configuration was specified but not found in account

#### Possible Solutions
* Specify unique name or Sid of an existent Transcription Configuration associated to your account"#,
            TwilioMiscellaneousError::ErrorCode32216 => r#"## ERROR - 32216

### SIP: SIP Address is on a deny list

This address is on a deny list. This address is on a deny list.

#### Possible Causes
The SIP Address exists on a deny list.

#### Possible Solutions
Use a SIP Address that is not on a deny list."#,
            TwilioMiscellaneousError::ErrorCode19032 => r#"## ERROR - 19032

### Invalid location input

 Request body is not valid

#### Possible Causes
A Required field is missing or field is invalid or duplicated 

#### Possible Solutions
Review the request body and the API documentation for more information"#,
            TwilioMiscellaneousError::ErrorCode80105 => r#"## Warning

### Short Code Already In Service

Example Message: ShortCode has already been added to Service"#,
            TwilioMiscellaneousError::ErrorCode32016 => r#"## ERROR - 32016

### PSTN PDD timeout

 ## PSTN Post-Dial Delay timeout

#### Possible Causes
PSTN Post-Dial Delay timer (60s) expired due to lack of response from multiple Carriers in route.

#### Possible Solutions
* Check the dialed Number to ensure it is currently reachable.
* If you believe the Number is currently reachable, please reach out to Twilio Support through the [Console](https://console.twilio.com) or [Help Center](https://support.twilio.com) and be sure to mention this Error code."#,
            TwilioMiscellaneousError::ErrorCode13910 => r#"## Warning - 13910 

### Pause: Invalid length value

The length attribute must be a positive integer, in seconds. See the <a href='/docs/api/twiml/pause#length'>Pause Verb</a> API Reference for more information.





"#,
            TwilioMiscellaneousError::ErrorCode80407 => r#"## Warning

### Phone Number Sid Invalid

Example Message: Invalid Phone Number Sid"#,
            TwilioMiscellaneousError::ErrorCode13246 => r#"## Warning - 13246

### Dial: SIP dialing not enabled for this account.

Your account is not enabled for SIP dialing.




"#,
            TwilioMiscellaneousError::ErrorCode32103 => r#"## Error - 32103

###  SIP: Empty body

#### Possible Causes 
*  The body of the request was empty, which is not allowed in this context.
"#,
            TwilioMiscellaneousError::ErrorCode70102 => r#"## Error - 70102

### Unsupported Public Key Length
The Request tries to store a Public Key in Twilio. This public key can be parsed, has a correct encryption algorithm, but the request fails because the key has an invalid length.

#### Possible Causes 
* Public key lenght different than 2048.

#### Possible Solution
* Regenerate a new keypair with length 2048 and retry."#,
            TwilioMiscellaneousError::ErrorCode90032 => r#"## ERROR - 90032

### Broadcast recipient's 'to' is invalid

 Broadcast recipient's 'to' is invalid

#### Possible Causes
Broadcast recipient's 'to' has invalid format. For instance, colon is missing.

#### Possible Solutions
Use correct 'to' ("channel:address") for recipient object"#,
            TwilioMiscellaneousError::ErrorCode11236 => r#"## Error - 11236

### Certificate Invalid - Certificate Expired

Twilio tried to validate your SSL certificate but your certificate has expired.

#### Possible Causes
*  Your SSL certificate has expired.

#### Possible Solutions
*  Renew your SSL certificate.
"#,
            TwilioMiscellaneousError::ErrorCode14202 => r#"## Error - 14202

### Enqueue: Invalid waitUrl

The waitUrl for the Enqueue verb is invalid.

#### Possible Solutions

Make sure you submit a fully qualified URL including:

*   protocol (http:// or https://)
*   hostname
*   file path
*   properly url-encoded query parameters

Twilio must be able to reach this URL over the public Internet.
"#,
            TwilioMiscellaneousError::ErrorCode11216 => r#"## ERROR - 11216

### HTTP invalid redirect

The response from your server contained an invalid redirect. Twilio sent a webhook to your server and your server responded with a 3xx status code that was not a valid redirect.

#### Possible Causes
* Your server responded with an non-redirect HTTP status code in the 3xx series. Twilio will only follow redirects for status codes 301, 302, 303, and 307.
* Your server did not include a location header in the redirect response

#### Possible Solutions
* Update your webhook configuration specifying the final URL you would like to be called, removing the need for redirects.
* Ensure you are using a standard redirect HTTP status code (301, 302, 303, or 307)
* Ensure that you are including a valid url in the location header of your response."#,
            TwilioMiscellaneousError::ErrorCode14111 => r#"## Warning - 14111

### Invalid To phone number for Trial mode

'To' field must be a validated caller ID while in Trial mode
"#,
            TwilioMiscellaneousError::ErrorCode70051 => r#"## ERROR - 70051

### Authorization Failed

 Twilio could not authorize the given request.

#### Possible Causes
* The Account is not found or is suspended
* The API key is not found or is suspended
* The Credentials used for authorization do not belong to the given account
* If using a Restricted API Key - the key may be missing a permission to access the resource
* If using a Restricted API Key - permission to access the resource may not be supported

#### Possible Solutions
* Make sure the account or the API key are valid and active
* If using a Restricted API Key - add the missing permission to the key
* If using a Restricted API Key - use a Main/Standard type API key"#,
            TwilioMiscellaneousError::ErrorCode21210 => r#"## ERROR - 21210

### 'From' phone number not verified

 You attempted to initiate an outbound phone call, but the
'From' number you specified is not a verified
number for your account. In order to use a phone number as the Caller ID on
outgoing calls, you must first verify your ownership of that phone number.

You can verify phone numbers in the Console [here](https://www.twilio.com/console/phone-numbers/verified).

### Test Credentials

If you received this error while trying to authenticate with your Test
Credentials, you probably tried to make a test call with a `From` number from
your live account. The only number that can be used to queue `successful`
calls with your Test Credentials is `+15005550006`. For more information,
read our documentation on [the `From` number for making calls with your test
credentials][from-test-credentials].

[from-test-credentials]: /docs/iam/test-credentials#test-sms-messages-parameters-From


#### Possible Causes
The From number used to make outbound call not a Twilio number or not a verified caller ID

#### Possible Solutions
 [phone number verification on Console](https://www.twilio.com/console/phone-numbers/verified)"#,
            TwilioMiscellaneousError::ErrorCode80303 => r#"## Warning

### Not Found Short Code Sid

Example Message: Short Code Sid is not associated with your Account."#,
            TwilioMiscellaneousError::ErrorCode80905 => r#"## Warning

### Unknown Scenario

Example Message: No valid proxy scenario for handling inbound call or message"#,
            TwilioMiscellaneousError::ErrorCode61004 => r#"## Error - 61004

### Add-ons: No results

#### Possible Causes 
*   The Add-on Provider returned a successful response but the Response Validation Schema indicated there was no usable data.
"#,
            TwilioMiscellaneousError::ErrorCode13217 => r#"## Warning - 13217 

### Dial: Invalid record value

record must be "true" or "false".

"#,
            TwilioMiscellaneousError::ErrorCode80503 => r#"## Error

### No Records Updated

Example Message: Record to be updated was not found. It may have been deleted."#,
            TwilioMiscellaneousError::ErrorCode11320 => r#"## Error - 11320 

### Invalid template unclosed brackets

The provided URL template has an invalid format. Please check to see if you have unclosed brackets.





"#,
            TwilioMiscellaneousError::ErrorCode13242 => r#"## Warning - 13242 

### Dial->SIP: Invalid sendDigits value

sendDigits attribute may only contain the following characters "0123456789*#w"





"#,
            TwilioMiscellaneousError::ErrorCode13245 => r#"## Warning - 13245 

### Dial: Not allowed in this API version.

Your version of the API does not support this feature.




"#,
            TwilioMiscellaneousError::ErrorCode51104 => r#"## Error - 51104

### Token doesn't contain service instance

#### Possible Causes 
The token doesn't have a service instance.

#### Possible Solutions
Confirm a service instance is being passed in the token. 
To check whether the access token is structurally correct, you can use the tools available at [jwt.io](https://jwt.io)."#,
            TwilioMiscellaneousError::ErrorCode51122 => r#"## Error - 51122

### Authentication failed

#### Possible Causes 
Missing or wrong information provided as credentials.

#### Possible Solutions
Confirm your credentials are correct and being passed correctly."#,
            TwilioMiscellaneousError::ErrorCode20020 => r#"## ERROR - 20020

### No update/state change is observed for the data entered

 No update/state change is observed for the data entered

#### Possible Causes
* The data already exist in T-Mobile SDG Service Provisioning API

#### Possible Solutions
* If data is needed to be updated in T-Mobile SDG Service Provisioning API the request must have updated fields"#,
            TwilioMiscellaneousError::ErrorCode19042 => r#"## ERROR - 19042

### Custom Field validation error

 The request body has invalid inputs

#### Possible Causes
Invalid length for custom field, data type mismatch and/or date in invalid format

#### Possible Solutions
Review the request body and the API documentation for more information"#,
            TwilioMiscellaneousError::ErrorCode12101 => r#"## Error - 12101 

### Invalid Twilio Markup XML version

The version attribute of the Response root element is not valid.

#### Possible Causes 
*  an invalid value was provided for the "version" attribute of the <a href='/docs/api/twiml/response'>Response</a> root element.


#### Possible Solutions
*  refer to the <a href="/docs/api/twiml/response">versions</a> documentation for information on valid version values.	




"#,
            TwilioMiscellaneousError::ErrorCode14211 => r#"## Error - 14211

### Dial->Queue: Invalid whisper url

The whisper url is invalid.

#### Possible Solutions

Make sure you submit a fully qualified URL including:

*   protocol (http:// or https://)
*   hostname
*   file path
*   properly url-encoded query parameters

Twilio must be able to reach this URL over the public Internet.
"#,
            TwilioMiscellaneousError::ErrorCode21452 => r#"## Error - 21452 

### No phone numbers found in area code

You attempted to get a phone number for your account, but Twilio couldn't find any numbers in the area you specified.  You may try again with a different <span class='rest-attribute'>AreaCode</span> or contact us to request a number in a certain area.  We can usually accommodate such requests within a few days.





"#,
            TwilioMiscellaneousError::ErrorCode61000 => r#"## Error - 61000

### Add-ons Internal server error

#### Possible Causes 
*  This is generic catch all error for Add-on related conditions that were not caught by a more specific error.
"#,
            TwilioMiscellaneousError::ErrorCode21421 => r#"## ERROR - 21421

### Phone Number is invalid

You tried to purchase a phone number for your account that was an invalid number. Please specify a valid phone number in E.164 format. You tried to purchase or lookup a phone number that is invalid or add a phone number to a Messaging Service with an invalid Phone Number SID.

#### Possible Causes
* You tried to purchase or lookup a phone number that is invalid.
* You tried to add an invalid Phone Number SID or Short Code SID to a Messaging Service.

#### Possible Solutions
* For phone number lookup or purchase, please specify a valid phone number in E.164 format.
* For adding a Phone Number to a Messaging Service, verify that the Phone Number SID or Short Code SID is correct and formatted properly, then retry the request. "#,
            TwilioMiscellaneousError::ErrorCode13312 => r#"## Warning - 13312 

### Gather: Invalid method value

The method attribute of Number must be GET or POST. See the <a href='/docs/api/twiml/gather#method'>Gather Verb</a> API Reference for more information.





"#,
            TwilioMiscellaneousError::ErrorCode20429 => r#"## ERROR - 20429

### Too Many Requests

Your account is sending too many concurrent Rest API requests to Twilio servers. 

#### Possible Causes
Your account is sending too many concurrent Rest API requests to Twilio servers.

#### Possible Solutions
An API Request that receives an error 429 response is never processed and is always safe to retry.  Please wait for a short period of time and make the request again, or alter your client's settings to issue fewer concurrent requests to the Twilio API.

For more information, please review the 429 Error Response knowledge base article at [https://support.twilio.com/hc/en-us/articles/360044308153-Twilio-API-response-Error-429-Too-Many-Requests-](https://support.twilio.com/hc/en-us/articles/360044308153-Twilio-API-response-Error-429-Too-Many-Requests-)"#,
            TwilioMiscellaneousError::ErrorCode50068 => r#"## Error - 50068

### Programmable Chat: Service instance unique name invalid

#### Possible Causes 
*  Request does not contain correctly formatted service instance unique name

#### Possible Solutions
*  Confirm a valid service instance unique name is being passed in request. "#,
            TwilioMiscellaneousError::ErrorCode51202 => r#"Too many requests"#,
            TwilioMiscellaneousError::ErrorCode21471 => r#"## Error - 21471

### Account does not exist

The AccountSid you specified does not exist.  AccountSids are 34 character strings that represent the Twilio Account.  For example, AC44CE4123947237834573457345347567.





"#,
            TwilioMiscellaneousError::ErrorCode70253 => r#"## Error - 70253

### Invalid User Grants
The Request tries to authenticate through SSO. The request is well-formed, but the user grants stored is not a valid grants form.

#### Possible Causes 
* Invalid User Grants.

#### Possible Solution
* Reconfigure the user grants and retry."#,
            TwilioMiscellaneousError::ErrorCode90012 => r#"## ERROR - 90012

### 'Recipients' list has too many items [deprecated] 

 'Recipients' list has too many items

#### Possible Causes
Too many recipients are provided for a single Broadcast

#### Possible Solutions
Split the broadcast to several parts where each part has smaller number of recipients than allowed"#,
            TwilioMiscellaneousError::ErrorCode21601 => r#"## Error - 21601

### Phone number is not a valid SMS-capable/MMS-capable inbound phone number 

The phone number you provided as the FROM field to initiate an outbound message is not an incoming phone number assigned to your account. Only phone numbers that you buy from Twilio can be used for outgoing messages. In particular, 3rd party phone numbers such as validated CallerIDs cannot be used to send messages."#,
            TwilioMiscellaneousError::ErrorCode13221 => r#"## Warning - 13221 

### Dial->Number: Invalid method value

The method attribute of Number must be GET or POST. See the <a href='/docs/api/twiml/dial'>Dial Verb</a> API Reference for more information.





"#,
            TwilioMiscellaneousError::ErrorCode14212 => r#"## Error - 14212

### Dial->Queue: queue name too short

The name of the queue must be at least 1 character long.
"#,
            TwilioMiscellaneousError::ErrorCode13251 => r#"## Warning - 13251 

### Dial: Too many headers passed

You sent too many header elements.


"#,
            TwilioMiscellaneousError::ErrorCode51127 => r#"## Error - 51127

### Twilsock: PNC limit hit"#,
            TwilioMiscellaneousError::ErrorCode13299 => r#"## ERROR - 13299

### 2010 Conference API feature requested using 2008 API

A feature added to the 2010 Conference API was requested via the 2008 Twilio API. Please upgrade to the latest version of the conference API. A feature added to the 2010 Conference API was requested via the 2008 Twilio API. Please upgrade to the latest version of the conference API.

#### Possible Causes
- 

#### Possible Solutions
- "#,
            TwilioMiscellaneousError::ErrorCode403 => r#"## ERROR - 403

### Forbidden

 The request is not authorized

#### Possible Causes
Incorrect credentials

#### Possible Solutions
Verify credentials"#,
            TwilioMiscellaneousError::ErrorCode13231 => r#"## Error - 13231

### Dial->Conference: Invalid endConferenceOnExit value

endConferenceOnExit must be either "true" or false".
"#,
            TwilioMiscellaneousError::ErrorCode13120 => r#"## Error - 13120 

### Annotate->BillingReferenceTag cannot be over 128 characters

BillingReferenceTag cannot be over 128 characters





"#,
            TwilioMiscellaneousError::ErrorCode21653 => r#"## ERROR - 21653

### There are more recipient addresses than allowed

 There are more recipient addresses than allowed for a channel. For instance for SMS we allow only one recipient but for Group MMS we allow multiple recipients

#### Possible Causes
Too many recipients were provided in the request or channel selection is incorrect

#### Possible Solutions
Fix the list of recipients according a channel restrictions"#,
            TwilioMiscellaneousError::ErrorCode12300 => r#"## ERROR - 12300

### Invalid Content-Type

 Twilio is unable to process the Content-Type of the provided URL. Please see Twilio's <a href="https://www.twilio.com/docs/voice/twiml#twilio-understands-mime-types"> documentation on accepted content types </a> for more information on valid Content-Types.

You must return a Content-Type for all requests. Requests without a
Content-Type will appear in the [Debugger](/console/runtime/debugger) as a
502 Bad Gateway error.



#### Possible Causes
* Having a phone number, outgoing call request or action attribute refer to a non-XML or audio resource.
* Having a Play verb attempt to play non-audio content, such as XML or text.

#### Possible Solutions
* Verify that that your web server is returning a Content-Type and it is the expected value
* Make sure the URL noted refers to a valid resource
* Make sure messages to or from your Twilio phone number are using a supported content-type. See the [full list of supported content types](https://www.twilio.com/docs/sms/accepted-mime-types).
* Use one of the [supported request types](https://www.twilio.com/docs/runtime/functions/request-flow#supported-requests) to call a Twilio function."#,
            TwilioMiscellaneousError::ErrorCode51215 => r#"Auth failure"#,
            TwilioMiscellaneousError::ErrorCode80604 => r#"## Warning

### Proxy Identifier In Use

Example Message: Proxy identifier already in use with requested identifier."#,
            TwilioMiscellaneousError::ErrorCode32113 => r#"## WARNING - 32113

### SIP: Header name is not allowed

 The SIP header you asked Twilio to stamp was omitted because it is not supported on the Twilio platform.

Twilio supports extension headers beginning with an X- prefix. Your extension header name cannot begin with the prefix `X-Twilio`.

#### Possible Causes
* SIP header name not supported on the Twilio platform.

#### Possible Solutions
* If your application can use a different header name, use an X- prefixed header name, e.g. if you want to use the name `Customer-Info`, see if your application can be configured to use `X-Customer-Info` instead."#,
            TwilioMiscellaneousError::ErrorCode21453 => r#"## Error - 21453 

### Phone number already verified for another account

You attempted to verify your phone number to activate your Twilio Free Trial Account, but that phone number is already associated with another Free Trial Account.  You can only open one Free Trial Account per phone number.





"#,
            TwilioMiscellaneousError::ErrorCode19024 => r#"## ERROR - 19024

### Invalid channel input

 Request body is not valid

#### Possible Causes
Required field is missing or field provided is invalid or duplicated

#### Possible Solutions
Review the request body and the API documentation for more information"#,
            TwilioMiscellaneousError::ErrorCode11235 => r#"## Error - 11235

### Certificate Invalid - Domain Mismatch

Twilio tried to validate your SSL certificate but your certificate has a domain name that does not match the domain we requested.

#### Possible Causes
*  Your Certificate Name or Certificate Alternate Name on your SSL certificate does not match the domain Twilio requested.


#### Possible Solutions
*  Obtain a SSL certificate with a Certificate Name or Certificate Alternate Name that matches this domain.
"#,
            TwilioMiscellaneousError::ErrorCode90025 => r#"## ERROR - 90025

### Template body has unsupported tag type

 Template body has unsupported Mustache tag type

#### Possible Causes
Currently, Twilio supports only data tags ("{{<tag name>}}"). All other Mustache tags/commands are disabled.

#### Possible Solutions
Check your template body has only data tags (placeholders)"#,
            TwilioMiscellaneousError::ErrorCode70106 => r#"## Error - 70106

### Invalid AWS credentials specified in the request
When uploading AWS credentials to Twilio make sure:
* AWS key matches the regex: (?<![A-Z0-9])[A-Z0-9]{20}(?![A-Z0-9])
* AWS secret matches the regex: (?<![A-Za-z0-9/+=])[A-Za-z0-9/+=]{40}(?![A-Za-z0-9/+=])

#### Possible Solution:
* Validate the credentials and upload them again.

"#,
            TwilioMiscellaneousError::ErrorCode10001 => r#"## ERROR - 10001

### Account is not active

This account has been disabled and may not be used until it is reactived. This account has been disabled and may not be used until it is reactivated.

#### Possible Causes
*  Lack of funds
*   Violation of the Terms of Service or Acceptable Use Policy

#### Possible Solutions
*  Check your account balance.  Refill funds if necessary.	
*  Please reach out to Twilio Support through the [Console](https://console.twilio.com) or [Help Center](https://support.twilio.com)."#,
            TwilioMiscellaneousError::ErrorCode80605 => r#"## Warning

### Proxy Identifier Not In Service

Example Message: Proxy identifier not associated with service."#,
            TwilioMiscellaneousError::ErrorCode13112 => r#"## Warning - 13112 

### Annotate: Invalid nested element

You may only nest BillingReferenceTag elements in Annotate





"#,
            TwilioMiscellaneousError::ErrorCode94100 => r#"## WARNING - 94100

### Transcriptions: status callback response error

 Twilio attempted to send a transcription event to the callback URL specified and your application returned a 4xx or 5xx or other HTTP response error

#### Possible Causes
* The callback URL specified is incorrect
* Your application has an issue while handling callback requests or suffering and outage

#### Possible Solutions
* Ensure that the callback URL specified is correct
* Ensure that your callback server can handle the request and responds with a 2xx HTTP status code"#,
            TwilioMiscellaneousError::ErrorCode90039 => r#"## ERROR - 90039

### Broadcast 'MessageStatusCallbackUrl' is invalid

 Broadcast 'MessageStatusCallbackUrl' is invalid

#### Possible Causes
'MessageStatusCallbackUrl' has invalid URL format or schema. Messaging allows only HTTP or HTTPS schemas

#### Possible Solutions
Use correct http or https URL"#,
            TwilioMiscellaneousError::ErrorCode21209 => r#"## Error - 21209 

### Invalid Method

You attempted to initiate an outbound phone call, but you sent a <span class='rest-attribute'>Method</span> parameter that was invalid.  <span class='rest-attribute'>Method</span> specifies what HTTP method Twilio should use in contacting your web server.  Possible values are GET and POST.





"#,
            TwilioMiscellaneousError::ErrorCode11203 => r#"## Error - 11203

### HTTP communication total time out triggered
All attempts to communicate with your web server were timed out. You set the total timeout using the tt parameter in the URL fragment identifier.

#### Possible Solutions
*  Adjust the 'tt' (total timeout) parameter in the fragment part of the URL, the one after '#'."#,
            TwilioMiscellaneousError::ErrorCode12100 => r#"## Error - 12100

### Document parse failure

Twilio was unable to parse the provided XML Document.

Your TwiML document must be a valid XML Document, or Twilio will not
be able to read your document. You can debug XML parsing errors by
getting the response body in the [debugger](/user/account/debugger),
and then using an online validation tool like the
[W3C Validation Service](http://validator.w3.org/#validate_by_input).

#### Possible Causes
*   There is a leading space, or an empty line, before the XML type header (i.e. ` <?xml version="1.0" encoding="UTF-8"?>`)
*   The root `<Response>` element is missing
*   There is an unclosed element
*   There is an unquoted attribute
*   There is an improperly nested element

#### Possible Solutions
*   Make sure there is no extra space or line at the beginning of the file before the type header
*   Make sure your root element is &lt;Response&gt;
*   XML is case sensitive, make sure your start and end elements match case. (Twilio elements begin with a capital letter)
*   Make sure characters such as &lt; &gt; and &amp; are escaped properly, as `&lt; &gt; and &amp;`.

[redirect]:/docs/api/twiml/redirect
"#,
            TwilioMiscellaneousError::ErrorCode61008 => r#"## Error - 61008

### Add-ons:  HTTP too many redirects

#### Possible Causes 
*  Add-on Provider returned too many HTTP redirects. To prevent getting stuck in a redirect loop only 5 redirects will be followed. After that the request will be terminated and this error code will be returned."#,
            TwilioMiscellaneousError::ErrorCode61005 => r#"## Error - 61005

### Add-ons: Bad request

#### Possible Causes 
*   The Add-on Provider provided an HTTP Status Code in the 400 range. This indicates that they received the request but were unable to processes it due to validation or authorization issue.
"#,
            TwilioMiscellaneousError::ErrorCode14106 => r#"## Warning - 14106 

### Message Reply TwiML document retrieval limit reached

Your Message Reply session has requested too many documents. The limit is 50

#### Possible Causes 
*  infinite loop caused by self referencing Message verb action URL
*  infinite loop caused by self referencing Redirect verb URL

#### Possible Solutions
*  make certain any action or Redirect URLs do not loop back to the same TwiML document.
"#,
            TwilioMiscellaneousError::ErrorCode11217 => r#"## ERROR - 11217

### HTTP error response code

Your server responded to the webhook request with an error response (4xx or 5xx) Twilio sent a webhook to your server and your server responded with a 4xx or 5xx status code.

#### Possible Causes
* There was an error in your server (500).
* There was a problem between your proxy and your server (502, 503, 504).
* The URL specified in your webhook configuration is incorrect (404, 410).
* Your server is attempting to rate limit Twilio's webhook requests (429). Twilio's webhooks do not support rate limiting.
* Your webhook configuration does not have appropriate permissions to call the configured URL (403).

#### Possible Solutions
* Review the configured webhook URL to ensure that it is correct.
* Look for and fix errors in your server logs.
* Ensure that you can get 2xx status codes by calling your webhook URL.
* Review your proxy and authentication configurations.
* For rate limiting issues, consider shifting to Event Streams HTTP Sink which [support rate limiting](https://www.twilio.com/docs/events/faq/problems-on-delivery#what-does-throttle-sink-means-for-eventstreams)."#,
            TwilioMiscellaneousError::ErrorCode13248 => r#"## Warning - 13248 

### Dial: Invalid callerID, too long

Your callerID contains too many characters.



"#,
            TwilioMiscellaneousError::ErrorCode94600 => r#"## ERROR - 94600

### Filter limit exceeded

 The action could not be performed because the filter limit was exceeded.

#### Possible Causes
 * There were too many filters applied.

#### Possible Solutions
 * Use date filtering freely plus at most one other filter."#,
            TwilioMiscellaneousError::ErrorCode13244 => r#"## Warning - 13244 

### Dial: No SIP Authorization

Your account is not authorized to make SIP calls





"#,
            TwilioMiscellaneousError::ErrorCode51131 => r#"## Error - 51131

### Authentication failed"#,
            TwilioMiscellaneousError::ErrorCode80902 => r#"## Warning

### No Active Session

You received a call to your Proxy Number from a caller who is not a participant in an active session. "#,
            TwilioMiscellaneousError::ErrorCode21420 => r#"## Error - 21420

### ApplicationSid is not accessible

You specified a VoiceApplicationSid or SmsApplicationSid for a new phone number
that is not controlled by your account, or is invalid. Please specify an
ApplicationSid that you've created in your [Account
Dashboard](/user/account/apps).
"#,
            TwilioMiscellaneousError::ErrorCode11237 => r#"## Error - 11237

### Certificate Invalid - Could not find path to certificate

Twilio tried to validate your SSL certificate but was unable to find it in our certificate store.

#### Possible Causes
*  You are using a self signed certificate.
*  The certificate authority you are using is not on our list of approved certificate authorities.
* Your certificate chain is incomplete and requires an additional download.

#### Possible Solutions
*  Do not use a self signed certificate.
* Concatenate your certificate chain so that no additional download is required.
*  Twilio uses CAs that are approved by Mozilla, you can find the full list [here](https://ccadb-public.secure.force.com/mozilla/IncludedCACertificateReport).
* For testing purposes you can [disable SSL Certificate Validation in Console](https://www.twilio.com/console/project/settings).
"#,
            TwilioMiscellaneousError::ErrorCode61003 => r#"## Error - 61003

### Add-ons: Requirements to invoke AddOns have not been met

#### Possible Causes 
*  A parameter required to complete the request was not provided at run time. As a result the Add-on Request can not be completed.
*  The Add-on does not support the data that was provided. For example the Add-on may not support looking up phone numbers in specific countries. 

#### Possible Solutions
*  Refer to Add-on documentation in Console to determine if the Add-on supports the country or format of the data you are providing
* Check Add-on documentation to see if you may be missing some data for Lookup Add-ons"#,
            TwilioMiscellaneousError::ErrorCode90022 => r#"## ERROR - 90022

### One of 'TemplateArgs' dictionary value is too long

 One of 'TemplateArgs' dictionary value is too long

#### Possible Causes
One of 'TemplateArgs' dictionary value is too long

#### Possible Solutions
Use shorter 'TemplateArgs' dictionary values"#,
            TwilioMiscellaneousError::ErrorCode14108 => r#"## Warning - 14108 

### Message "From" Phone Number not message capable

The From phone number does not appear to be a phone number that you can Message from

#### Possible Causes 
*  Sending from a number that is not SMS or MMS capable


#### Possible Solutions
*  Check to make sure the number is correct
*  Check to make sure the number is an SMS or MMS capable Twilio Incoming number assigned to your account
"#,
            TwilioMiscellaneousError::ErrorCode20002 => r#"## Error - 20002 

### Invalid FriendlyName

Many Twilio resources take descriptive names to help you remember what they are.  However, such "FriendlyNames" must be between 1 and 64 characters long.





"#,
            TwilioMiscellaneousError::ErrorCode80406 => r#"## Warning

### Phone Number Did Invalid

Example Message: Invalid Phone Number Did"#,
            TwilioMiscellaneousError::ErrorCode21618 => r#"## Error - 21618

### The message body cannot be sent

You have attempted to send a message with content that is being filtered by all
of our carriers.

We have logged the carrier you are attempting to reach and will monitor these
failures when adding new carriers. If we add a new carrier in the future that
does not filter the content you are trying to send, Twilio will automatically
route your message through the new carrier.
"#,
            TwilioMiscellaneousError::ErrorCode61007 => r#"## Error - 61007

### Add-ons: Response body too large

#### Possible Causes 
*   The Add-on Provider returned a response payload that exceeded 64 kilobytes. As a result the payload was removed and this error was returned instead."#,
            TwilioMiscellaneousError::ErrorCode19034 => r#"## ERROR - 19034

### Invalid country code

 Country code provided in the request is not valid

#### Possible Causes
Country code is not in the ISO 3166-1 alpha-2 country code of the address format

#### Possible Solutions
Update the country code to match  ISO 3166-1 alpha-2 country code of the address format"#,
            TwilioMiscellaneousError::ErrorCode11770 => r#"## Error - 11770

### Empty response body

Twilio received a HTTP response with Transfer-encoding: chunked but an empty body.

#### Possible Causes 

*   Server misconfiguration
*   Network disruptions


#### Possible Solutions

*   Verify that the server is serving a non-empty HTTP response
*   Verify that the network connection is stable

"#,
            TwilioMiscellaneousError::ErrorCode60007 => r#"## ERROR - 60007

### Downstream Verification Failed

 Security Token validation failed

#### Possible Causes
- Missing App Security Token.
- App Security Token mismatch.

#### Possible Solutions
- Ensure you have correctly enabled additional token validation at verification creation.
- Check App Security Token is being passed correctly as part of SNA URL validation."#,
            TwilioMiscellaneousError::ErrorCode11300 => r#"## Error - 11300 

### Invalid template URL

The provided URL template has an invalid format. Please check to see if you have unclosed brackets.





"#,
            TwilioMiscellaneousError::ErrorCode80307 => r#"## WARNING - 80307

### Record to be updated was not found in database.

 Record to be updated was not found in database.

#### Possible Causes
Record to be updated is already deleted.

#### Possible Solutions
No action required."#,
            TwilioMiscellaneousError::ErrorCode21241 => r#"## ERROR - 21241

### Credential List Dependencies Violation

You cannot delete a Credential List that is still mapped to a SIP Domain. 

#### Possible Causes
* You cannot delete a Credential List that is still mapped to a SIP Domain.

#### Possible Solutions
* Remove the Credential List from any mapped SIP Domains before deleting."#,
            TwilioMiscellaneousError::ErrorCode19050 => r#"## ERROR - 19050

### Internal Server Error

 

#### Possible Causes
We're having trouble completing your request 

#### Possible Solutions
Reach Customer Support or Try again later"#,
            TwilioMiscellaneousError::ErrorCode14105 => r#"## Warning - 14105 

### Message statusCallback attribute Invalid

The Message Verb's statusCallback attribute must be a valid URL

"#,
            TwilioMiscellaneousError::ErrorCode80409 => r#"## Warning

### Session Sid Invalid

Example Message: Invalid Session Sid"#,
            TwilioMiscellaneousError::ErrorCode19025 => r#"## ERROR - 19025

### Channel validation error

 The request body has invalid inputs

#### Possible Causes
Invalid length, email or phone in invalid format

#### Possible Solutions
Review the request body and the API documentation for more information"#,
            TwilioMiscellaneousError::ErrorCode404 => r#"## ERROR - 404

### Not Found

 Failed to find a match for the request

#### Possible Causes
* The specified resource is no longer available.


#### Possible Solutions
* Check that the specified resource still exists.
"#,
            TwilioMiscellaneousError::ErrorCode13110 => r#"## Error - 13110 

### Annotate: Annotate must contain one valid nested element

Annotate must contain one valid nested element





"#,
            TwilioMiscellaneousError::ErrorCode11200 => r#"## ERROR - 11200

### HTTP retrieval failure

Twilio’s servers were unable to fetch a non-error response from the designated URL. We try to provide specific webhook errors whenever possible, however in this instance we are unable to identify the exact cause of the webhook delivery/response issue. To prevent 11200 errors, ensure that your webhook receiving infrastructure can quickly respond to inbound Twilio requests with a 2xx status code.

#### Possible Causes
* Web server returned a 4xx or 5xx HTTP response to Twilio
* Network disruptions between Twilio and your web server; these can be within your or Twilio’s infrastructure or wider disruptions to the internet at large.
* Firewalls (network or application level) between Twilio and your infrastructure
* Misconfigured Web Server
* No Content-Type header attached to response
* Content-Type doesn't match actual content, e.g. an MP3 file that is being served with Content-Type: audio/x-wav, instead of Content-Type: audio/mpeg


#### Possible Solutions
* Double check that your TwiML URL does not return a 4xx or 5xx error
* Make certain that the URL does not perform a 302 redirect to an invalid URL
* Confirm the URL requested is not protected by HTTP Auth
* Make sure your web server allows HTTP POST requests to static resources (if the URL refers to .xml or .html files)
* Verify your web server is up and responsive
* Check to see that the URL host is not a private or local IP address
* Verify the ping times and packet loss between your web server and api.twilio.com
* Review firewall rules; modern web application firewalls can trigger on only a small subset of inbound connections leading to inconsistent behavior.
* Check logs at the very edge of your infrastructure to verify if TCP connection attempts are arriving at your edge. If you have confirmed these requests are not reaching your infrastructure, please reach out to [support](https://www.twilio.com/console/support/tickets/create).
* Use more robust options for webhook receipt. For inbound messages and calls be sure to provide a fallback URL (ideally on separate infrastructure). Transition integrations to [Event Streams](https://www.twilio.com/docs/events) which has various event sink types including webhooks with built in retries.
* Review your use of any [webhook connection overrides](https://www.twilio.com/docs/usage/webhooks/webhooks-connection-overrides). In general the defaults provide the best results, start by reconfiguring your webhook URL without any fragment overrides and see if the problem persists.
* If synchronously processing a webhook requires significant time, we recommend that you simply acknowledge the event by quickly responding with an empty 202 (Accepted) and then processing the message on your own timeline. Replies to inbound message events can be done at any time by making a call to the REST API.
"#,
            TwilioMiscellaneousError::ErrorCode51111 => r#"## Error - 51111

### Service instance limit has been reached

#### Possible Causes 
Service instance limit has been reached for the specified account.

#### Possible Solutions
Verify that your application is not misbehaving and the load is expected. If everything is as expected then contact Customer Support to increase your account's service Instance limits."#,
            TwilioMiscellaneousError::ErrorCode21224 => r#"## Warning - 21224

### Invalid SipAuthPassword. Illegal chars

The SipAuthUsername you specified contained characters not allowed in SIP.
"#,
            TwilioMiscellaneousError::ErrorCode14203 => r#"## Error - 14203

### Enqueue: Invalid Enqueue action url

The action URL for the Enqueue verb is invalid.

#### Possible Solutions

Make sure you submit a fully qualified URL including:

*   protocol (http:// or https://)
*   hostname
*   file path
*   properly url-encoded query parameters

Twilio must be able to reach this URL over the public Internet.
"#,
            TwilioMiscellaneousError::ErrorCode60006 => r#"## ERROR - 60006

### Invalid Phone Number

 

#### Possible Causes
The request was received and failed during processing because a downstream provider indicated that the phone number is invalid, although the phone number formatting was valid. This can happen when the end user provides an invalid phone number in error, the phone number is deactivated, suspended, or in the grace period before reassignment, or a fraudulent user purposefully uses an invalid phone number.

#### Possible Solutions
- Check that the phone number provided is correct.
- If using Silent Network Auth, retry Verification using a channel not based on phone number such as email."#,
            TwilioMiscellaneousError::ErrorCode21479 => r#"## Error - 21479

### Unable to update Status for subaccount, subaccount has been closed.

Unable to update Status for subaccount, subaccount is closed and can not be re-opened.
"#,
            TwilioMiscellaneousError::ErrorCode21451 => r#"## Error - 21451 

### Invalid area code

You attempted to get a phone number for your account, but the <span class='rest-attribute'>AreaCode</span> parameter you specified was not a valid North American area code.  <span class='rest-attribute'>AreaCode</span> is optional, and for Local phone numbers, may be any US or Canadian area code, or for TollFree numbers, may be one of 800,888,877 or 866.





"#,
            TwilioMiscellaneousError::ErrorCode90011 => r#"## ERROR - 90011

### MessageSid is invalid

 MessageSid is invalid

#### Possible Causes
MessageSid has incorrect sid format or prefix (SM and MM allowed)

#### Possible Solutions
Use correct MessageSid. Example `SMec8e12b0a5eb4fc68e173f1c5db95b2a`"#,
            TwilioMiscellaneousError::ErrorCode21480 => r#"## Error - 21480

### Reached maximum number of subaccounts

Reached maximum number of subaccounts, please contact [Twilio](/help) if you want to create more subaccounts.
"#,
            TwilioMiscellaneousError::ErrorCode13322 => r#"## Warning - 13322 

### Gather->Say: Invalid loop value

The value of the loop attribute must be an integer greater or equal to zero.  See the <a href='/docs/api/twiml/say#loop'>Say Verb</a> API Reference for more information.





"#,
            TwilioMiscellaneousError::ErrorCode13254 => r#"## Warning - 13254

### Dial->Sip: SIP URI DNS does not resolve or resolves to an non-public IP address

Please check your DNS record to ensure it is properly configured. Alternatively, you can use a specific public IP address in your URI rather than specifying a domain.
"#,
            TwilioMiscellaneousError::ErrorCode11251 => r#"None"#,
            TwilioMiscellaneousError::ErrorCode70103 => r#"## Error - 70103

### Unsupported Public Key Exponent
The Request tries to store a Public Key in Twilio. This public key can be parsed, has a correct encryption algorithm, but the request fails because the key exponent has an invalid length.

#### Possible Causes 
* Public key key exponent different than 65537.

#### Possible Solution
* Regenerate a new keypair with exponent 65537 and retry."#,
            TwilioMiscellaneousError::ErrorCode90034 => r#"## ERROR - 90034

### 'BroadcastStatusCallbackUrl' is too long

 'BroadcastStatusCallbackUrl' is too long

#### Possible Causes
'BroadcastStatusCallbackUrl' is too long

#### Possible Solutions
Use shorter url in 'BroadcastStatusCallbackUrl' field"#,
            TwilioMiscellaneousError::ErrorCode21212 => r#"## ERROR - 21212

### Invalid 'From' Number

 You attempted to initiate an outbound phone call or message, but the `From` parameter you supplied was not a valid phone number, Alphanumeric Sender ID or approved WhatsApp Sender.

Twilio accepts phone numbers in [E.164 format](https://www.twilio.com/docs/glossary/what-e164) (i.e. "+1 format"), 10-digit US and Canadian numbers with any combination of non-digit separators, or Alphanumeric Sender IDs (SMS only) with up to 11 alphanumeric characters [a-zA-Z0-9]. Refer to [this page](https://support.twilio.com/hc/en-us/articles/223133867-Using-Alphanumeric-Sender-ID-with-Messaging-Services) for acceptable characters.

The number must not be on a do-not-originate (DNO) list, and Alphanumeric Sender IDs may not be generic.


#### Possible Causes
* You have supplied a phone number that was not in [E.164 format](https://www.twilio.com/docs/glossary/what-e164).
* Your `From` phone number is on a do-not-originate (DNO) list.
* Your [Alphanumeric Sender ID](https://www.twilio.com/docs/glossary/what-alphanumeric-sender-id) is blocked or too generic.
* You are trying to use an Alphanumeric Sender ID on Trial Account. Trial accounts [cannot send from Alphanumeric Sender IDs](https://www.twilio.com/docs/messaging/guides/how-to-use-your-free-trial-account#sending-sms-and-mms-messages).
* Your WhatsApp Sender is not approved.
* You attempted to send a message with a `From` number that has an invalid prefix, using a Messaging Service.


#### Possible Solutions
* Ensure your number is formatted in [E.164 format](https://www.twilio.com/docs/glossary/what-e164).
* Ensure that your `From` number is assigned and not on a do-not-originate (DNO) list.
* Ensure you are not using an [alphanumeric sender ID](https://www.twilio.com/docs/glossary/what-alphanumeric-sender-id) that is too generic (for example, "SMS" or "INFO").
* If you are getting this error when using a non-generic alphanumeric sender ID, please [submit a support ticket](https://www.twilio.com/console/support/tickets/create) for further assistance.
* If you are using an Alphanumeric Sender ID on a Trial Account, upgrade your account.
* If you are using a WhatsApp Sender, check that your [sender is approved](https://console.twilio.com/us1/develop/sms/senders/whatsapp-senders) and that you have followed the [onboarding instructions](https://www.twilio.com/docs/whatsapp/tutorial/requesting-access-to-whatsapp).
* If you are sending messages using a Messaging Service, and you wish to utilize an RCS Agent in your Sender Pool that falls back to SMS, [send the message from a MessagingServiceSid](https://www.twilio.com/docs/messaging/services#send-a-message-with-a-messaging-service-1). "#,
            TwilioMiscellaneousError::ErrorCode13210 => r#"## Warning - 13210 

### Dial: Invalid method value

You have provided an invalid value for the "method" attribute.  Valid values are GET or POST.  See the <a href='/docs/api/twiml/dial#method'>Dial Verb</a> API Reference for more information.





"#,
            TwilioMiscellaneousError::ErrorCode11201 => r#"## Error - 11201
### TCP connection timed out
Twilio cannot establish a TCP connection to your web server. The two attempts to connect to your web server were timed out. Each connection attempt has a 5 second timeout.

#### Possible Causes
* firewall in your network blocking incoming TCP traffic
* network outage between Twilio and your web server
* invalid host part in the HTTP URL pointing to non-existing host or private IP address

#### Possible Solutions
* verify firewall configuration in your network or on the web server
* verify network connectivity to your web server
* verify the HTTP URL provided is accessible to the public Internet"#,
            TwilioMiscellaneousError::ErrorCode20413 => r#"## Request Entity Too Large

### Request entity is larger than limits defined

Request entity is larger than limits defined by the server. "#,
            TwilioMiscellaneousError::ErrorCode12102 => r#"## Error - 12102 

### The root element must be Response

The root element of a Twilio Markup XML document *must* be <a href="/docs/api/twiml/response">Response</a>





"#,
            TwilioMiscellaneousError::ErrorCode51001 => r#"## Error - 51001

### Client Connection: Connections resource limit exceeded

#### Possible Causes 

The rate of client connection requests or the number of concurrent connections exceeded the limit.

#### Possible Solutions

Verify that your application is not misbehaving and the load is expected. If everything is as expected then contact Customer Support to increase your Account's limits.

"#,
            TwilioMiscellaneousError::ErrorCode84001 => r#"## ERROR - 84001

### Workflow execution timed out.

 Execution timed out before completing the execution of all the blocks.

#### Possible Causes
- Long await on any async await events.

#### Possible Solutions
- Define a timeout for async await events. "#,
            TwilioMiscellaneousError::ErrorCode90029 => r#"## ERROR - 90029

### Broadcast 'CorrelationId' is empty

 Broadcast 'CorrelationId' is empty

#### Possible Causes
Empty string value is used in 'CorrelationId' field

#### Possible Solutions
Use not empty string as 'CorrelationId' field"#,
            TwilioMiscellaneousError::ErrorCode19040 => r#"## ERROR - 19040

### Custom Field provided is not defined

 The custom field provided in the request does not exist

#### Possible Causes
Request with a custom_field that is not defined

#### Possible Solutions
Use custom field definition API to define the custom field first"#,
            TwilioMiscellaneousError::ErrorCode32111 => r#"## WARNING - 32111

### SIP: Invalid header name

 The header you asked Twilio to stamp was omitted because it contains illegal characters in its _name_. Including this header would have resulted in Twilio generating a message that runs contrary to the SIP specification.

The set of legal characters for a header name is defined in [RFC 3261](https://tools.ietf.org/html/rfc3261).

#### Possible Causes
* If you specified headers in a SIP URI, invalid characters were most likely introduced by an escaped character. Escaped characters in SIP URIs are defined by a percent sign (%) followed by two hexadecimal digits specifying a character code.

#### Possible Solutions
* Ensure that your SIP URI is formed correctly. Header names should generally only consist of alphanumeric characters and hyphens."#,
            TwilioMiscellaneousError::ErrorCode90018 => r#"## ERROR - 90018

### 'Template' field is too long

 'Template' field is too long

#### Possible Causes
'Template' field is too long

#### Possible Solutions
Use shorter 'Template'"#,
            TwilioMiscellaneousError::ErrorCode94001 => r#"## ERROR - 94001

### Transcriptions: Invalid transcribe value

 Invalid “transcribe” value. It should be either true or false. No transcription resource has been created

#### Possible Causes
 

#### Possible Solutions
* Ensure that the transcribe value is set to “true” to request a transcription via REST API or TwiML"#,
            TwilioMiscellaneousError::ErrorCode94201 => r#"## ERROR - 94201

### Transcriptions Settings: encryptionKeySid invalid or not found

 Request to enable encryption for transcriptions failed because encryptionKeySid is invalid or not defined

#### Possible Causes
* "encryptionKeySid" value is invalid
* "encryptionKeySid" not found in Transcription Settings
* "encryptionKeySid" not found in Credentials Storage
* "encryptionKeySid" is not a credential resource of type Public Key

#### Possible Solutions
* Specify a encryptionKeySid of a valid credential resource of type Public Key that can be used to encrypt transcription results or disable encryption"#,
            TwilioMiscellaneousError::ErrorCode13253 => r#"## Warning - 13253

### Dial: Header is too long

You sent a header that is too long.

"#,
            TwilioMiscellaneousError::ErrorCode19004 => r#"## ERROR - 19004

### Invalid or missing Contact input

 The request body provided has some errors

#### Possible Causes
A required field is missing, duplicate field or invalid field name.

#### Possible Solutions
Review the request body and the API documentation for more information"#,
            TwilioMiscellaneousError::ErrorCode19005 => r#"## ERROR - 19005

### Contact validation error

 The data provided in the request body is invalid

#### Possible Causes
Invalid string length, invalid data type

#### Possible Solutions
Review the request body and the API documentation for more information"#,
            TwilioMiscellaneousError::ErrorCode90019 => r#"## ERROR - 90019

### 'TemplateArgs' dictionary size is too large

 'TemplateArgs' dictionary size is too large

#### Possible Causes
Too many items in 'TemplateArgs' dictionary

#### Possible Solutions
Use shorter 'TemplateArgs' dictionary"#,
            TwilioMiscellaneousError::ErrorCode400 => r#"## ERROR - 400

### Bad Request

 Failed to complete request due to a bad request

#### Possible Causes
* The resource to be modified has moved into a state that is no longer valid.
* Input on the request did not pass validation.

#### Possible Solutions
* Retry the request after confirming the request is valid.
* Verify if the resource to be modified exists and is in a valid state."#,
            TwilioMiscellaneousError::ErrorCode13520 => r#"## Warning - 13520 

### Say: Invalid text

The text of the Say verb was empty or un-parsable. See the <a href='/docs/api/twiml/say'>Say Verb</a> API Reference for more information.





"#,
            TwilioMiscellaneousError::ErrorCode21609 => r#"## Error - 21609

### Invalid StatusCallback URL

The `StatusCallback` URL provided is not a valid URL. 

#### Possible Solutions

Make sure you submit a fully qualified URL including:

*   protocol (http:// or https://)
*   hostname
*   file path
*   properly url-encoded query parameters

Twilio must be able to reach this URL over the public Internet.
"#,
            TwilioMiscellaneousError::ErrorCode51121 => r#"## Error - 51121

### Token expiration time exceeds maximum

#### Possible Causes 
Token expiration time exceeds max allowed amount.

#### Possible Solutions
Use a proper expiration time to fit into the allowed range.

To check whether the access token is structurally correct, you can use the tools available at [jwt.io](https://jwt.io)."#,
            TwilioMiscellaneousError::ErrorCode84002 => r#"## ERROR - 84002

### Workflow execution failed. 

 Workflow execution failed.

#### Possible Causes
- Possible downstream exceptions.

#### Possible Solutions
- Check error payload for the error message."#,
            TwilioMiscellaneousError::ErrorCode11310 => r#"## Error - 11310 

### Invalid template token

The provided URL template references a nonexistent token.





"#,
            TwilioMiscellaneousError::ErrorCode60008 => r#"## ERROR - 60008

### Unsupported Carrier

 

#### Possible Causes
The request was received and failed due to the phone number’s carrier not supporting this endpoint.

#### Possible Solutions
- Complete the [Account Security Registration](https://interactive.twilio.com/account-security-registration-form?_ga=2.40477419.894088021.1660748067-1879743973.1650468405) form to use this carrier.
- If you have completed the Account Security Registration form, [contact Twilio Support](https://www.twilio.com/help/contact) to confirm that you are approved.
- If you have completed the Account Security Registration form and it has been approved, [contact Twilio Support](https://www.twilio.com/help/contact) to rule out integration issues.
- If using Silent Network Auth, retry Verification using a channel other than Silent Network Auth for this end user."#,
            TwilioMiscellaneousError::ErrorCode94300 => r#"## ERROR - 94300

### Transcriptions Configurations: Invalid callback configuration

 Invalid callback configuration, either to create a new configuration or update existing one. A valid Url and Method must be specified to enable callbacks via HTTP request (webhook) and receive transcriptions status updates.

#### Possible Causes
* Invalid Callback "Url" value
* Invalid Callback "Method" value

#### Possible Solutions
* Callback "Url" value should be valid Http(s) url format 
* Callback "Method" value should be either POST or GET 
* Disable callback by setting “Url” to ‘’ value (empty string)"#,
            TwilioMiscellaneousError::ErrorCode51112 => r#"## Error - 51112

### Product usage disabled

#### Possible Causes 
Product usage has been disabled for this instance.

#### Possible Solutions
If you are sure the service instance and product are correct, please contact with Customer Support."#,
            TwilioMiscellaneousError::ErrorCode94301 => r#"## ERROR - 94301

### Transcriptions Configurations: UniqueName is required

 A UniqueName is required to create a new configuration

#### Possible Causes
* "UniqueName" is blank
* "UniqueName" is null

#### Possible Solutions
* Specify a valid "UniqueName" for transcription configuration"#,
            TwilioMiscellaneousError::ErrorCode20005 => r#"## Error - 20005 

### Account not active

Your Twilio account is not active, potentially because you ran out of funds, or you have not yet activated your free trial.
"#,
            TwilioMiscellaneousError::ErrorCode32110 => r#"## WARNING - 32110

### SIP: URI is formatted incorrectly

Your SIP URI cannot be parsed unambiguously. Twilio cannot understand parts of your SIP URI. This may be due to invalid characters appearing in the user, params, or headers portion. While Twilio may still attempt to complete your call, the parts of the URI that are invalid will be ignored.

The format for a valid SIP URI is defined in [RFC 3261](https://tools.ietf.org/html/rfc3261).

#### Possible Causes
* The user-portion of the URI contains invalid characters. The user-info portion allows alphanumeric characters, hyphens, underscores, and a small number of additional punctuation characters.
* A URI header or parameter value contains punctuation or whitespace that is not escaped.

#### Possible Solutions
* If you specify SIP headers in your URI, ensure any non-alphanumeric characters in the header values are escaped. For example, to add a User-to-User header with value `0A1B2C3D;encoding=hex`, you must encode this into the URI as `User-to-User=0A1B2C3D%3Bencoding%3Dhex`."#,
            TwilioMiscellaneousError::ErrorCode51120 => r#"## Error - 51120

### Token is not valid yet"#,
            TwilioMiscellaneousError::ErrorCode21230 => r#"# Error - 21230

### Maximum Domains Reached

An account cannot have more than 100 domains."#,
            TwilioMiscellaneousError::ErrorCode90038 => r#"## ERROR - 90038

### 'BroadcastStatusCallbackUrl' is invalid

 'BroadcastStatusCallbackUrl' is invalid

#### Possible Causes
'BroadcastStatusCallbackUrl' has no Url format or schema is incorrect. Broadcast allows only http and https schemas

#### Possible Solutions
Use valid http or https url"#,
            TwilioMiscellaneousError::ErrorCode21214 => r#"## Error - 21214 

### 'To' phone number cannot be reached

You attempted to initiate an outbound phone call to a phone number that cannot
be reached. Please check the number and try again.  
"#,
            TwilioMiscellaneousError::ErrorCode51007 => r#"## ERROR - 51007

### Client Connection: Token authentication is rejected

error ## Error - 51007

### Client Connection: Token authentication is rejected by authentication service





#### Possible Causes
Authentication service has rejected provided Access Token.

To check whether the Access Token is structurally correct, you can use the tools available at [jwt.io](https://jwt.io/).

For the details of Twilio's specific Access Token implementation including the grant format, see [the documentation](/docs/api/rest/access-tokens).

#### Possible Solutions
For further assistance, please reach out to Twilio Support through the [Console](https://console.twilio.com) or [Help Center](https://support.twilio.com)."#,
            TwilioMiscellaneousError::ErrorCode80603 => r#"## ERROR - 80603

### Non Unique Session Name

 Session UniqueName must be unique.

#### Possible Causes
A Session exists with the proposed UniqueName.

#### Possible Solutions
Retry with a UniqueName that is not currently in use by any other Sessions for this Service."#,
            TwilioMiscellaneousError::ErrorCode90026 => r#"## ERROR - 90026

### Template body could not be parsed

 Template body could not be parsed by Mustache compiler

#### Possible Causes
Template body uses incorrect grammar (we support Mustache grammar). For instance empty Mustache placeholder ("{{}}") is not allowed by the grammar.

#### Possible Solutions
Check your template body is correct Mustache grammar."#,
            TwilioMiscellaneousError::ErrorCode13224 => r#"## ERROR - 13224

### Dial: Twilio does not support calling this number or the number is invalid

 Twilio does not support calling this number or the number is invalid

#### Possible Causes
This error occurs when the provided number is not in E.164 format; for example using local dialing pattern 01234567890 instead of the full E.164 address +441234567890. This can also occur when calls are attempted to non-existent country codes, area codes, or exchanges. For example, there is no +693 country code, no +1238 US area code, and no 555 US exchange.

This can also occur with properly-formatted, valid destinations if none of Twilio’s carrier partners are able to deliver the call; for example due to a telecom network outage in the phone number’s local area.

#### Possible Solutions
Use E.164 formatting and ensure the destination is a valid, in-service destination."#,
            TwilioMiscellaneousError::ErrorCode14206 => r#"## Error - 14206

### Enqueue: Invalid waitUrlMethod value

The waitUrlMethod value is invalid. You must specify one of 'GET' or 'POST'.
"#,
            TwilioMiscellaneousError::ErrorCode94604 => r#"## ERROR - 94604

### Account Sid on Legal Hold.

 The Account is on Legal Hold, the operation could not be performed.

#### Possible Causes
 * The account has a Legal Hold

#### Possible Solutions
 "#,
            TwilioMiscellaneousError::ErrorCode11241 => r#"## ERROR - 11241

### HTTP connection edge location is not supported

 There was an error connecting to the URL due to an unsupported edge location parameter.

#### Possible Causes
* The edge location(s) specified in the 'e' parameter of your connection settings are not currently supported by Twilio.

#### Possible Solutions
* Correct the 'e' parameter in the fragment part of the URL, the one after '#'."#,
            TwilioMiscellaneousError::ErrorCode13237 => r#"## Error - 13237

### Dial->Conference: Invalid conference name 

The conference name you attempted to Dial is invalid.  Conference names must be between 1 and 128 characters. 
"#,
            TwilioMiscellaneousError::ErrorCode80405 => r#"## Warning

### Participant Sid Invalid

Example Message: Invalid Participant Sid"#,
            TwilioMiscellaneousError::ErrorCode21474 => r#"## Error - 21474

### API User must be the parent account to transfer phone numbers.

The API user can not be a subaccount, must be a parent account to transfer phone numbers between parent and child accounts.




"#,
            TwilioMiscellaneousError::ErrorCode13250 => r#"## Warning - 13250 

### Dial: Too many URIs passed

You specified too many URI elements.


"#,
            TwilioMiscellaneousError::ErrorCode13314 => r#"## Warning - 13314 

### Gather: Invalid numDigits value

The numDigits attribute must be a positive integer. See the <a href='/docs/api/twiml/gather#numDigits'>Gather Verb</a> API Reference for more information.





"#,
            TwilioMiscellaneousError::ErrorCode50069 => r#"## Error - 50069

### Programmable Chat: Service instance with provided unique name already exists

#### Possible Causes 
*  Request contains already existing service instance unique name

#### Possible Solutions
*  Provide non-existent service instance unique name in the request"#,
            TwilioMiscellaneousError::ErrorCode51119 => r#"## Error - 51119

### Token expired

#### Possible Causes 
The token has been expired.

#### Possible Solutions
Please use a valid token.

To check whether the access token is structurally correct, you can use the tools available at [jwt.io](https://jwt.io)."#,
            TwilioMiscellaneousError::ErrorCode80610 => r#"## Warning

### Unauthorized Operation

Example Message: Unauthorized. You may be trying to use an invalid combination of sids."#,
            TwilioMiscellaneousError::ErrorCode21213 => r#"## Error - 21213 

### 'From' phone number is required

You attempted to initiate an outbound phone call, but you forgot to include a
'From' parameter. The 'From' parameter is required, and tells Twilio what Caller
ID to use for the phone call.





"#,
            TwilioMiscellaneousError::ErrorCode90036 => r#"## ERROR - 90036

### Broadcast recipient's 'body' is too long

 Broadcast Recipient's 'body' is too long

#### Possible Causes
Recipient's 'body' is too long

#### Possible Solutions
Use shorter 'body' value for recipient object"#,
            TwilioMiscellaneousError::ErrorCode13211 => r#"## Warning - 13211 

### Dial: Invalid ifMachine value

ifMachine must be either "hangup","continue", or "false".





"#,
            TwilioMiscellaneousError::ErrorCode51005 => r#"## Warning - 51005

### Client Connection: Command or keepalive acknowledgement not received

#### Possible Causes 

The client did not acknowledge a message or a keep-alive request, which is required by the connection protocol therefore the connection has been dropped. This is most often caused by network connectivity issues.

#### Possible Solutions

Twilio SDKs automatically recover and reestablish the connection if possible. 
For best user experience, ensure reliable network connectivity.
"#,
            TwilioMiscellaneousError::ErrorCode12400 => r#"## Error - 12400 

### Internal Failure

An internal error has occurred that prevented Twilio from processing your response.

#### Possible Causes 
*  We screwed up. Sorry!


#### Possible Solutions
*   If the error persists, please <a href="/help/contact">contact us</a> to figure out what has happened and how to fix it.
*   Note the time of the error and what you were trying to do when it occurred.




"#,
            TwilioMiscellaneousError::ErrorCode32114 => r#"## WARNING - 32114

### SIP: Unsupported parameter value

A URI parameter contains an unsupported value and will be ignored Twilio parses certain standard URI parameters, such as the "transport" parameter, as well as Twilio-specific parameters such as "secure".

#### Possible Causes
* If using the "secure" parameter, allowed values are "true" and "false". Any other value will be treated as "false" and trigger this warning.

#### Possible Solutions
* Check the other details of the debugger alert. The alert may indicate which parameter contains the unsupported value.
* Update your application/configuration to ensure a URI with appropriate parameters is sent."#,
            TwilioMiscellaneousError::ErrorCode21450 => r#"## Error - 21450

### Phone number already verified for your account

You attempted to verify a phone number for your account, but that number is
already verified for your account.
"#,
            TwilioMiscellaneousError::ErrorCode21454 => r#"## Error - 21454 

### Invalid CallDelay

Call delay specified must be an integer between 0 and 60, specifying the number of seconds to delay before initiating the validation call.

#### Possible Causes 
*  Make sure to enter an integer between 0 and 60.





"#,
            TwilioMiscellaneousError::ErrorCode21626 => r#"## Error - 21626

### Invalid 'StatusCallbackEvent'

One or more of the StatusCallbackEvents provided were invalid.
"#,
            TwilioMiscellaneousError::ErrorCode13410 => r#"## Warning - 13410 

### Play: Invalid loop value

The value of the loop attribute must be an integer greater or equal to zero. See the <a href='/docs/api/twiml/play#loop'>Play Verb</a> API Reference for more information.





"#,
            TwilioMiscellaneousError::ErrorCode21401 => r#"## ERROR - 21401

### Invalid Phone Number

The phone number you specified was not a valid North American (US or Canadian) number.  Twilio accepts only 10 digit US domestic and Canadian phone numbers, with or without a leading "1" or "+1", with any combination of character separators.  For example, these are all valid: 415-867-5309, 4158675309, +1 (415) 867 5309. ## Error - 21401

### Invalid Phone Number

The phone number you specified was not a valid SMS-enabled phone number or alphanumeric sender ID.

If a phone number was used, this number must be one you have purchased from or ported to Twilio in [E.164 format][e164].

Alphanumeric sender IDs can only be used with [accepted characters][alpha-sender-id] when messaging [countries where this feature is supported][alphanumeric-countries].

[e164]: http://en.wikipedia.org/wiki/E.164
[alphanumeric-countries]: https://support.twilio.com/hc/en-us/articles/223133767-International-support-for-Alphanumeric-Sender-ID
[alpha-sender-id]: /docs/api/rest/sending-messages#alpha-sender-id

#### Possible Causes
The phone number is not supported in North America.

#### Possible Solutions
Try using a different phone number: https://www.twilio.com/phone-numbers"#,
            TwilioMiscellaneousError::ErrorCode70153 => r#"## Error - 70153 

### Public Key Specified Does Not Exist
The Public Key with the SID provided in the JWT header does exist. This is a Public Key Client Validation Error.

#### Possible Solution:
* Update the kid value provided in the JWT header.
* For the details on JWTs used as part of Public Key Client Validation, see [the documentation](/docs/api/credentials/public-key-client-validation-getting-started)"#,
            TwilioMiscellaneousError::ErrorCode70152 => r#"## Error - 70152

### Request Contains Invalid Flags
One or more of the provided flags in the API key request are invalid in the given context.

#### Possible Causes 
* An API key is attempting to create a new key with greater permissions than what it is assigned.

#### Possible Solution:
* Remove the invalid flag(s) that would expand permissions."#,
            TwilioMiscellaneousError::ErrorCode80202 => r#"## Warning

### No Available Message Proxy

Example Message: This Service has no compatible Proxy numbers for this Participant. This Service has no available Proxy numbers having sms capabilities. This can happen if you attempted to create a participant in a session for which you did not specify a Mode in a country that does not support combined voice and sms capabilities, or if your only available proxy numbers do not have messaging capabilities."#,
            TwilioMiscellaneousError::ErrorCode20503 => r#"## Error 20503

### Service unavailable

The requested Twilio Service is temporarily unavailable.

### Possible Solutions
- Try again a few times with exponential back-off
- If the problem persists, please [contact support](https://www.twilio.com/help/contact)."#,
            TwilioMiscellaneousError::ErrorCode94000 => r#"## ERROR - 94000

### Transcriptions: request to transcribe audio error

 Twilio failed to process the request to transcribe audio due to an internal issue. No transcription resource has been created

#### Possible Causes
 

#### Possible Solutions
 "#,
            TwilioMiscellaneousError::ErrorCode13232 => r#"## Warning - 13232

### Dial->Conference: Invalid startConferenceOnEnter value

startConferenceOnEnter must be either "true" or false".
"#,
            TwilioMiscellaneousError::ErrorCode21901 => r#"## ERROR - 21901

### DltTemplateId is invalid

 DltTemplateId is invalid

#### Possible Causes
Too long DltTemplateId or its format is invalid

#### Possible Solutions
Use correct DltTemplateId"#,
            TwilioMiscellaneousError::ErrorCode19013 => r#"## ERROR - 19013

### At least one of the following fields is required for a contact: first_name, middle_name, last_name, legal_name, preferred_name, unique_customer_provided_id or channel

 Contact should contain at least one required field

#### Possible Causes
Contact does not possess even one required field

#### Possible Solutions
Contact should contain at least one of the following fields - first_name, middle_name, last_name, legal_name, preferred_name, unique_customer_provided_id or channel"#,
            TwilioMiscellaneousError::ErrorCode10004 => r#"## ERROR - 10004

### Call concurrency limit exceeded

 ## Error - 10004

### Call concurrency limit exceeded

The call was rejected because you reached the maximum limit of concurrent calls available for your account.

#### Possible Causes
Trial accounts and newly upgraded accounts without an approved Primary Customer Profile have limited concurrent calls.

#### Possible Solutions
[Upgrade your account](https://www.twilio.com/docs/usage/tutorials/how-to-use-your-free-trial-account-namer#how-to-upgrade-your-account) and [create a Primary Business Profile](https://www.twilio.com/docs/trust-hub/trusthub-rest-api/console-create-a-primary-customer-profile) to remove concurrent call limitations."#,
            TwilioMiscellaneousError::ErrorCode19037 => r#"## ERROR - 19037

### When updating a location at least one field should be updated

 No location updates provided in the request

#### Possible Causes
User has not provided location updates

#### Possible Solutions
When updating a location at least one field should be updated"#,
            TwilioMiscellaneousError::ErrorCode13213 => r#"## Warning - 13213 

### Dial: Invalid hangupOnStar value

hangupOnStar must be "true" or "false". See the <a href='/docs/api/twiml/dial#hangupOnStar'>Dial Verb</a> API Reference for more information.





"#,
            TwilioMiscellaneousError::ErrorCode11205 => r#"## ERROR - 11205

### HTTP connection failure

There was a network failure attempting to connect to this URL ### HTTP connection failure

There was a network failure attempting to connect to this URL. Twilio will try for 10 seconds to establish a TCP connection to your URL (two 5-second attempts). Twilio will wait for a total of 15 seconds to receive an HTTP response. This includes the TCP connection time, so if it took six seconds to establish a TCP connection, your server would have 9 seconds to deliver an HTTP response. If either of those timers expire, we fail the request, fire a notification, and try your fallback URL, if one is specified.

#### Possible Causes
*  Network outages between Twilio and your web server.
*  URLs that reference private IP addresses or localhost.
*  Twilio’s TCP timeout is 10 seconds. If you see an error in Request Inspector after more than 10000 milliseconds, Twilio could locate your server, but could not establish a TCP connection with it. 
*  Twilio’s HTTP timeout is 15 seconds. If you see an error in Request Inspector returned after more than 15000 milliseconds, Twilio was able to establish a TCP connection with your server, but the HTTP response from the server failed.

#### Possible Solutions
*  Verify the URL provided is accessible to the public internet.	
*  Verify network connectivity to your web server.
*  Check your TCP logs to verify inbound connection attempts are successful.
*  Check HTTP server logs to verify a response was generated and returned within the timeout.
*  Check your edge element logs to verify responses are making it out of your infrastructure."#,
            TwilioMiscellaneousError::ErrorCode51102 => r#"## Error - 51102

### Service instance SID not specified

#### Possible Causes 
No service instance SID specified and couldn't be generated.

#### Possible Solutions
Confirm a service instance SID is being passed in the request. "#,
            TwilioMiscellaneousError::ErrorCode51129 => r#"## Error - 51129

### Too many messages per account

#### Possible Causes 
Number of messages for this account is higher than expected!

#### Possible Solutions
Verify that your application is not misbehaving and the load is expected. If everything is as expected then contact Customer Support to increase your account's limits."#,
            TwilioMiscellaneousError::ErrorCode14223 => r#"## Error - 14223

### Enqueue: Unable to cleanup task

Unable to cancel the task created by Enqueue upon a TwimL error or task hangup.
"#,
            TwilioMiscellaneousError::ErrorCode410 => r#"## ERROR - 410

### Unknown Error Code

 Access to this resource is no longer available and should not be retried.


#### Possible Causes
* The resource has reached a terminal state and is no longer available.


#### Possible Solutions
* Do not retry the request since the condition is likely permanent.
"#,
            TwilioMiscellaneousError::ErrorCode94602 => r#"## ERROR - 94602

### Invalid FromDate

 The action could not be performed because the FromDate is invalid.

#### Possible Causes
* FromDate is a future date.

#### Possible Solutions
 * FromDate should be earlier than the present date."#,
            TwilioMiscellaneousError::ErrorCode20003 => r#"## ERROR - 20003

### Permission Denied

You lack permission to the resource and method you requested. #### Authenticating requests to the Twilio API

All requests to sensitive areas of the Twilio API must use [HTTP Basic
Authentication][auth]. Authenticate using your Account SID as the username,
and your Auth Token as the password. Both can be found in the [Twilio
Console](/console/account/settings).

You can also generate revocable [API Keys](/docs/iam/keys/api-key) to authenticate.

You may want to consider using a [Twilio helper library](/docs/libraries)
(available in PHP, .NET, Java, Ruby, Python and Node.js) as they will take care of
authentication for you.

[auth]: http://en.wikipedia.org/wiki/Basic_access_authentication

#### Possible Causes
* Using the wrong combination of Account SID and Auth Token 
* Using Test Credentials to access your Live Account
* Using subaccount credentials to access master account
* Previous Auth Token has been deleted
* Attempted API Key has been deleted
* Account is suspended or closed
* Extra characters or spaces in the supplied credentials
* Web proxy is stripping out credentials before forwarding to Twilio
* Using JWT with Auth Token rather than API Key
* Attempted API Key is for the incorrect Twilio Region

#### Possible Solutions
* Verify the Account SID and Auth Token are correct
* Verify the correct Account is being accessed
* Ensure the Account is active, not suspended or closed
* Ensure no extra characters or spaces are being included
* Ensure JWT is only used with API Key
* Ensure the API Key's Twilio Region matches the request's specified Twilio Region"#,
            TwilioMiscellaneousError::ErrorCode80701 => r#"## Warning

### Parameter Validation Failed

Example Message: '<parameter name>' is invalid. <parameter is invalid>"#,
            TwilioMiscellaneousError::ErrorCode21217 => r#"## Error - 21217

### Phone number does not appear to be valid 

You attempted to initiate an outbound phone call to an invalid phone number.  
Please check the phone number and try again.  
"#,
            TwilioMiscellaneousError::ErrorCode21621 => r#"## ERROR - 21621

### The 'From' number has not been enabled for MMS

 The `From` number is not enabled for MMS messaging.


#### Possible Causes
The `From` phone number is not capable of sending MMS.

#### Possible Solutions
* Buy an MMS Capable phone number from the [Twilio Console](https://console.twilio.com/us1/develop/phone-numbers/manage/search?isoCountry=US&types[]=Local&types[]=Tollfree&capabilities[]=Sms&capabilities[]=Mms&capabilities[]=Voice&capabilities[]=Fax&searchTerm=&searchFilter=left&searchType=number) by filtering for “MMS” under “Capabilities”.
* Check if you already have MMS capable numbers in the [Twilio Console](https://console.twilio.com/us1/develop/phone-numbers/manage/incoming) and use that number in your `From` parameter or Messaging Service."#,
            TwilioMiscellaneousError::ErrorCode13249 => r#"## Warning - 13249 

### Dial: Invalid username or password attribute

Your username or password are invalid.


"#,
            TwilioMiscellaneousError::ErrorCode13215 => r#"## Warning - 13215 

### Dial: Invalid nested element

The only valid nested element for Dial is Number. See the <a href='/docs/api/twiml/dial'>Dial Verb</a> API Reference for more information.





"#,
            TwilioMiscellaneousError::ErrorCode21202 => r#"## Error - 21202 

### 'To' number is a premium number

You attempted to initiate an outbound phone call to a "premium" phone number, such as a toll 976 line.
"#,
            TwilioMiscellaneousError::ErrorCode21225 => r#"## Warning - 21225

### SipAuthPassword is required when providing SipAuthUsername

You must specify a SipAuthPassword if you are also using SipAuthUsername."#,
            TwilioMiscellaneousError::ErrorCode21455 => r#"## Error - 21455 

### Invalid PlayUrl

The URL given to play during validation isn't a valid URL.

#### Possible Causes 

"#,
            TwilioMiscellaneousError::ErrorCode94101 => r#"## WARNING - 94101

### Transcriptions: status callback response timed out

 Twilio attempted to send a transcription event to the callback URL specified and your application didn’t respond before time out

#### Possible Causes
* The server receiving status callbacks sent by Twilio takes too long to respond

#### Possible Solutions
* Ensure that your callback server can handle the request and responds within a reasonable time"#,
            TwilioMiscellaneousError::ErrorCode13235 => r#"## Warning - 13235

### Dial->Conference: Invalid beep value

beep must be either "true" or false".
"#,
            TwilioMiscellaneousError::ErrorCode51128 => r#"## Error - 51128

### Too many messages per connection

#### Possible Causes 
Number of messages for one connection is higher than expected!

#### Possible Solutions
Verify that your application is not misbehaving and the load is expected. If everything is as expected then contact Customer Support to increase your account's limits."#,
            TwilioMiscellaneousError::ErrorCode13236 => r#"## Error - 13236

### Dial->Conference: Invalid Conference Sid value

You attempted to dial a Conference Sid that does not exist or has already been completed. 
"#,
            TwilioMiscellaneousError::ErrorCode21405 => r#"## Error - 21405 

### Cannot set VoiceFallbackUrl without setting Url

You cannot set a fallback URL without specifying a primary URL




"#,
            TwilioMiscellaneousError::ErrorCode70002 => r#"## Error - 70002

### Bad request
The data in the request is unparsable.

#### Possible Causes 
* When sending a string field “abc” when an intValue is expected
* When sending invalid JSON in the request body"#,
            TwilioMiscellaneousError::ErrorCode21223 => r#"## Warning - 21223

### Invalid SipAuthPassword. Must be fewer than 256 chars

The SipAuthPassword you specified contained too many characters.
"#,
            TwilioMiscellaneousError::ErrorCode21900 => r#"## ERROR - 21900

### DltPEId is invalid

 DltPEId is invalid

#### Possible Causes
The format or size of DltPEId is invalid

#### Possible Solutions
Use correct DltPEId"#,
            TwilioMiscellaneousError::ErrorCode15003 => r#"## WARNING - 15003

### Call Progress: Warning Response to Callback URL

Got an HTTP warning in response to the callback URL. 

#### Possible Causes
*  Your application returned a 4xx or 5xx HTTP response to Twilio.

#### Possible Solutions
Ensure your application responds to callbacks with: 

*  200 OK  
*  Content-Type: text/xml 
*  Valid TwiML

Or respond with 204 No Content and an empty response body."#,
            TwilioMiscellaneousError::ErrorCode19003 => r#"## ERROR - 19003

### Contact with the unique_customer_provided_id provided already exists

 unique_customer_provided_id already exists

#### Possible Causes
An existing contact has the same unique_customer_provided_id value provided in the request

#### Possible Solutions
Provide different unique_customer_provided_id or remove the existing one"#,
            TwilioMiscellaneousError::ErrorCode15000 => r#"## Error - 15000

### Call Progress: Internal Twilio Error

An internal error has occurred that prevented Twilio from processing your Call Progress Event.

#### Possible Solutions

*   If the error persists, please <a href="/help/contact">contact us</a> to figure out what has happened and how to fix it!
*   Note the time of the error and what you were trying to do when it occurred!"#,
            TwilioMiscellaneousError::ErrorCode21240 => r#"## ERROR - 21240

### Credential List Validation Error

One of the parameters passed when creating or updating a Credential List was incorrect. 

#### Possible Causes
* A Credential List with the same FriendlyName may already exist
* A Username was not provided. A Username must be provided and it must have 32 characters or less
* The password was not provided. A password is required that meets the following criteria:
    * Minimum 12 characters
    * At least one mixed case
    * At least one digit

#### Possible Solutions
* Ensure that your Credential List meets the validation requirements."#,
            TwilioMiscellaneousError::ErrorCode21206 => r#"## Error - 21206 

### Invalid SendDigits

You attempted to initiate an outbound phone call, and you included a <span class='rest-attribute'>SendDigits</span> parameter that wasn't valid. <span class='rest-attribute'>SendDigits</span> is optional, and specifies that Twilio should simulate sending keypad digits when the party answers.  It is useful for navigating a phone tree, reaching an extension or entering a password in a voicemail system.  <span class='rest-attribute'>SendDigits</span> may contain numbers, the # and * characters, as well as the "w" character to "wait" for half a second.





"#,
            TwilioMiscellaneousError::ErrorCode70053 => r#"
## Error - 70053

### Public Key Client Validation Not Enabled For Account
The request attempted to use the Public Key Client Validation but the functionality is not enabled for this account.

#### Possible Solution:
* Ensure your Account is enabled for this feature.
* Remove the Twilio-Client-Validation header from your request"#,
            TwilioMiscellaneousError::ErrorCode80205 => r#"## Warning

### No Proxies For Service

Example Message: This Service has no compatible Proxy numbers for this Participant. This Service has no Proxy numbers configured."#,
            TwilioMiscellaneousError::ErrorCode21228 => r#"## Warning - 21228

### Invalid SIP Header. Illegal chars in header name

The header you passed has characters not allowed in SIP."#,
            TwilioMiscellaneousError::ErrorCode21232 => r#"# Error - 21232

### Invalid Domain

The domain name requested is invalid, already in use or a subdomain of a domain you do not control.  This is likely because the parent domain does not exist or because neither the account sid or its parent account controls the parent domain.
"#,
            TwilioMiscellaneousError::ErrorCode91202 => r#"## ERROR - 91202

### Not Found

 Resource is not found

#### Possible Causes
* Schema not found
* Event Type Not found

#### Possible Solutions
* Recheck to see if the correct information is provided else
* Recheck to verify is the resource actually exists"#,
            TwilioMiscellaneousError::ErrorCode51103 => r#"## Error - 51103

### Token doesn't contain required grants section

#### Possible Causes 
No grants section is specified in the token and grants section is required.

#### Possible Solutions
Confirm grants section exists and valid in the token. 
To check whether the access token is structurally correct, you can use the tools available at [jwt.io](https://jwt.io)."#,
            TwilioMiscellaneousError::ErrorCode80908 => r#"## Warning

### Callback Error

Example Message: Error on Callback for http://www.yourdomain.com/callback"#,
            TwilioMiscellaneousError::ErrorCode80601 => r#"## Warning

### Phone Number Not Available

Example Message: PhoneNumber is not available.  It may be bound to a number pool."#,
            TwilioMiscellaneousError::ErrorCode13710 => r#"## Warning - 13710 

### Redirect: Invalid method value

The method attribute of Number must be GET or POST. See the <a href='/docs/api/twiml/redirect#method'>Redirect Verb</a> API Reference for more information.





"#,
            TwilioMiscellaneousError::ErrorCode14104 => r#"## Warning - 14104 

### Message method attribute invalid

Method attribute may only be "GET" or "POST"

"#,
            TwilioMiscellaneousError::ErrorCode80402 => r#"## Warning

### Identifier Proxy Pair Invalid

Example Message: Invalid Identifier and Proxy Identifier pair"#,
            TwilioMiscellaneousError::ErrorCode90020 => r#"## ERROR - 90020

### One of 'TemplateArgs' dictionary key is blank

 One of 'TemplateArgs' dictionary key is blank

#### Possible Causes
One of 'TemplateArgs' dictionary key is blank

#### Possible Solutions
Do not use empty or blank keys in 'TemplateArgs' dictionary"#,
            TwilioMiscellaneousError::ErrorCode21221 => r#"## Warning - 21221

### Invalid SipAuthUsername. Must be fewer than 256 chars

The SipAuthUsername you specified was longer than 255 characters.
"#,
            TwilioMiscellaneousError::ErrorCode20001 => r#"## Warning - 20001 

### Unknown parameters

One or more parameters you sent the REST API were not recognized and were ignored.  No harm done, but make sure that you didn't mis-type the parameter you really wanted to send.





"#,
            TwilioMiscellaneousError::ErrorCode90028 => r#"## ERROR - 90028

### Broadcast 'IdempotencyToken' is too long

 Broadcast 'IdempotencyToken' is too long

#### Possible Causes
'IdempotencyToken' field value has too long length

#### Possible Solutions
Use shorter string lenght for 'IdempotencyToken' field"#,
            TwilioMiscellaneousError::ErrorCode30014 => r#"## Error - 30014 

### "To" attributes are Invalid

The To attribute list is invalid or empty

#### Possible Causes 
*  empty list
*  to long list

#### Possible Solutions
*  Check the number of To attributes 
"#,
            TwilioMiscellaneousError::ErrorCode21100 => r#"## Error - 21100 

### Accounts Resource

Errors related to the Accounts resource





"#,
            TwilioMiscellaneousError::ErrorCode21476 => r#"## Error - 21476

### Unable to update Status for subaccount, parent account is suspended.

Unable to update Status for subaccount, parent account is suspended.  The account needs to be in good standing to continue using the subaccount feature.



"#,
            TwilioMiscellaneousError::ErrorCode20426 => r#"# Error - 20426

## Upgrade Required
The server refuses to perform the request using the current protocol but might be willing to do so after the client upgrades to a different protocol.

### TLS and cipher suite upgrade requirements
In moving towards a more secure posture with regards to [TLS](https://en.wikipedia.org/wiki/Transport_Layer_Security), we require that all new accounts must use TLS v1.2 or later. All customers must use TLS > 1.2 starting June 26, 2019.

Customers running older operating systems or legacy network software may need to upgrade their systems to be compatible with these changes. If you've received Error 20426, you likely need to upgrade your system.

To learn more about these changes, see our support site's FAQ on [Twilio REST API’s TLS and Cipher Suite Security Changes](https://support.twilio.com/hc/en-us/articles/360007820133-FAQ-Twilio-REST-API-s-TLS-and-Cipher-Suite-Security-Changes-for-June-2019)

### Tips for upgrading your environment
To learn more about Twilio's TLS and cipher requirements, please reference [this support article](https://support.twilio.com/hc/en-us/articles/360006751753-Tips-for-Upgrading-Your-Environment-to-Support-Twilio-REST-API-s-TLS-and-Strong-Cipher-Suite-Changes) where you can learn more about understanding failure signatures, which components to upgrade, and verifying your upgrade."#,
            TwilioMiscellaneousError::ErrorCode90033 => r#"## ERROR - 90033

### Broadcast recipient's 'to' is too long

 Broadcast recipient's 'to' is too long

#### Possible Causes
Too long address and channel are used in Recipient's 'to'

#### Possible Solutions
Use correct channel type (sms|mms|etc) and appropriate address"#,
            TwilioMiscellaneousError::ErrorCode20009 => r#"## ERROR - 20009

### Cannot delete this resource before it is complete

 You attempted to delete a call or message resource before it was completed. If you are attempting to delete a call, wait until the call is completed and try your request again. If you are attempting to delete a message, wait several minutes and try again.

If you're seeing a message in the logs that you have already deleted, please allow up to 14 days for our systems to completely remove this record before reaching out to support.

#### Possible Causes
* You requested the call or message be deleted before all of Twilio's systems finished processing it.
* In rare cases, a message or call record may take up to 14 days to be finalized and able to be deleted.

#### Possible Solutions
* If you are attempting to delete a call, verify that the call is not still in-progress.
* If you are attempting to delete a call or message shortly after sending or receiving it, please wait at least 1 minute to make your DELETE request. If you receive this error after waiting 1 minute, wait several minutes and retry deletion again.
* If you encounter this issue frequently even when waiting several minutes before deleting a message or a completed call, please [contact Twilio support](https://www.twilio.com/console/support/tickets/create) with examples of affected Call or Message SIDs.
* For the highest level of privacy for Twilio SMS messages, apply for [message redaction](https://ahoy.twilio.com/message-redaction) which can automatically redact message bodies or phone numbers immediately after a message is sent or received."#,
            TwilioMiscellaneousError::ErrorCode94302 => r#"## ERROR - 94302

### Transcriptions Configurations: UniqueName is invalid

 Invalid UniqueName value, either to create a new configuration or update existing one

#### Possible Causes
* "UniqueName" already associated to another configuration
* "UniqueName" is blank or null
* Invalid "UniqueName" value: Length should be between 1 and 50 characters, can’t contain blank spaces or special characters and can’t be a Transcription Configuration Sid

#### Possible Solutions
* Specify a valid "UniqueName" for transcription configuration"#,
            TwilioMiscellaneousError::ErrorCode13613 => r#"## Warning - 13613 

### Record: Invalid finishOnKey value

The value of the finishOnKey attribute must be one of the following characters "0123456789#*".  See the <a href='/docs/api/twiml/record#maxLength'>Record Verb</a> API Reference for more information.





"#,
            TwilioMiscellaneousError::ErrorCode11210 => r#"## ERROR - 11210

### HTTP bad host name

 The DNS entry for the URL’s host cannot be resolved.


#### Possible Causes
* URL references a locally defined hostname (check your `/etc/hosts`)
* URL references an undefined hostname

#### Possible Solutions
* Verify the hostname of the URL has a valid DNS record the can be resolved by the public"#,
            TwilioMiscellaneousError::ErrorCode21219 => r#"## Error - 21219

### 'To' phone number not verified

You attempted to initiate an outbound phone call from a trial account, but the
'To' number you specified is not a verified number for your account. In order
to make calls during a free trial, you must first verify your ownership of the
phone number being called.

You can verify phone numbers from the [phone numbers
section](/user/account/phone-numbers) of your account portal.
"#,
            TwilioMiscellaneousError::ErrorCode21404 => r#"## Error - 21404

### Inbound phone numbers not available to trial accounts

Purchasing phone numbers is a feature that is only available for upgraded
accounts. Please upgrade to a Full Twilio Account in order to purchase numbers.

You can upgrade from the [billing section](/user/billing) of your account
portal. 
"#,
            TwilioMiscellaneousError::ErrorCode21622 => r#"## Error - 21622

### MMS has not been enabled for your account

Your account has not been enabled for picture messaging.  Please [contact support](/help/contact) to send picture messages
"#,
            TwilioMiscellaneousError::ErrorCode61002 => r#"## Error - 61002

### Add-ons: Provider could not complete request

#### Possible Causes 
*  The Add-on Provider  provided a HTTP Status Code in the 500 range. This indicates an error processing the Add-on Request on the Vendors side. "#,
            TwilioMiscellaneousError::ErrorCode20008 => r#"## Error - 20008

#### Cannot access this resource with Test Credentials

Currently only a handful of API resources can be tested with your test
credentials. Trying to authenticate with your test credentials to other
resources will return a Forbidden error message. For a full list of supported
resources, please see our [Test Credentials](/docs/api/rest/test-credentials)
page.
"#,
            TwilioMiscellaneousError::ErrorCode51115 => r#"## Error - 51115

### Invalid access token subject

#### Possible Causes 
Subject provided in the token is invalid.

#### Possible Solutions
Confirm a valid token is being passed with a valid subject. 

To check whether the token is structurally correct, you can use the tools available at [jwt.io](https://jwt.io)."#,
            TwilioMiscellaneousError::ErrorCode61006 => r#"## Error - 61006

### Add-ons: Add-ons unavailable for GET callbacks

#### Possible Causes 
*  Due to the size limitations of GET callbacks we do not include Add-on results in GET callbacks"#,
            TwilioMiscellaneousError::ErrorCode20010 => r#"## Error - 20010

#### Action disabled for account

 The action you attempted to perform has been disabled for your account. Please contact Customer Support for assistance."#,
            TwilioMiscellaneousError::ErrorCode84003 => r#"## ERROR - 84003

### Unable to render block.

 A block could not be rendered. 

#### Possible Causes
- Liquid values used in the block might not be correct.

#### Possible Solutions
- Check the liquid expressions in the block evaluate to a proper value."#,
            TwilioMiscellaneousError::ErrorCode21208 => r#"## Error - 21208 

### Invalid Timeout

You attempted to initiate an outbound phone call, but you sent an invalid <span class='rest-attribute'>Timeout</span> parameter. The timeout specifes how long Twilio should let the phone ring before giving up, and is useful if you want to have Twilio hangup before an answering machine picks up. You may specify any positive, whole integer for the <span class='rest-attribute'>Timeout</span> parameter.

"#,
            TwilioMiscellaneousError::ErrorCode21200 => r#"## Error - 21200 

### Calls Resource

Errors related to the Calls resource





"#,
            TwilioMiscellaneousError::ErrorCode90023 => r#"## ERROR - 90023

### One of 'TemplateArgs' dictionary value is null

 One of 'TemplateArgs' dictionary value is null

#### Possible Causes
One of 'TemplateArgs' dictionary value is null

#### Possible Solutions
Do not use null 'TemplateArgs' dictionary values"#,
            TwilioMiscellaneousError::ErrorCode90035 => r#"## ERROR - 90035

### Broadcast 'MessageStatusCallbackUrl' is too long

 Broadcast 'MessageStatusCallbackUrl' is too long

#### Possible Causes
'MessageStatusCallbackUrl' is too long

#### Possible Solutions
Use shorter url in 'MessageStatusCallbackUrl' field"#,
            TwilioMiscellaneousError::ErrorCode80609 => r#"## Warning

### Too Many Added Participants

Example Message: A Session may have at most 2 participants"#,
            TwilioMiscellaneousError::ErrorCode51124 => r#"## Error - 51124

### Too many connections

#### Possible Causes 
Number of connections for this account is higher than expected!

#### Possible Solutions
Verify that your application is not misbehaving and the load is expected. If everything is as expected then contact Customer Support to increase your account's limits."#,
            TwilioMiscellaneousError::ErrorCode70005 => r#"## Error - 70005 

### Failure Threshold Exceeded
Too many unsuccessful requests against Twilio API.

#### Possible Causes 
* The requests try to repeatedly access a deleted resource
* The requests try to repeatedly access an invalid resource

#### Possible Solution:
* Slow down the requests
* Validate whether the given resource exists"#,
            TwilioMiscellaneousError::ErrorCode14205 => r#"## Error - 14205

### Enqueue: Queue name too long

The queue name must be less than 64 characters in length.
"#,
            TwilioMiscellaneousError::ErrorCode80408 => r#"## Warning

### Service Sid Invalid

Example Message: Invalid Service Sid"#,
            TwilioMiscellaneousError::ErrorCode13243 => r#"## Warning - 13243 

### Dial->SIP: Invalid SIP URI

The provided SIP URI is not formatted properly.





"#,
            TwilioMiscellaneousError::ErrorCode21238 => r#"## ERROR - 21238

### Address Validation Error

One of the parameters passed when creating or updating an Address was incorrect. 

#### Possible Causes
* One of the parameters passed when creating or updating an Address was incorrect.

#### Possible Solutions
* Ensure that you are creating a valid Address."#,
            TwilioMiscellaneousError::ErrorCode13320 => r#"## Warning - 13320 

### Gather: Invalid nested verb

The only valid nested elements of the Gather Verb are Say and Play. See the <a href='/docs/api/twiml/gather'>Gather Verb</a> API Reference for more information.





"#,
            TwilioMiscellaneousError::ErrorCode30015 => r#"## Error - 30015

### Non-supported channel type is used

The To or From attributes have an unsupported prefix for channel

#### Possible Causes 
*  empty channel prefix for an attribute that requires a prefix
*  non-existing prefix for an attribute that requires a prefix

#### Possible Solutions
*  Check the To/From attributes channel prefix
"#,
            TwilioMiscellaneousError::ErrorCode50353 => r#"## ERROR - 50353

### Conversation with provided unique name already exists

 

#### Possible Causes
Request contains already existing Conversation unique name.

#### Possible Solutions
Provide non-existing Conversation unique name in the request."#,
            TwilioMiscellaneousError::ErrorCode21458 => r#"## Error - 21458

### PhoneNumber Provisioning Type Mismatch

The type of a PhoneNumber (toll-free or not) did not match the type of the IncomingPhoneNumbers sub-resource (Local or TollFree).

#### Possible Causes 
*  Make sure to provision toll-free numbers from the IncomingPhoneNumbers/TollFree resource or directly from the IncomingPhoneNumbers/ resource.
*  Make sure to provision non-toll-free numbers from the IncomingPhoneNumbers/Local resource or directly from the IncomingPhoneNumbers/ resource.
"#,
            TwilioMiscellaneousError::ErrorCode14201 => r#"## Error - 14201

### Enqueue: Invalid method value

The method is invalid. You must specify one of 'GET' or 'POST'.
"#,
            TwilioMiscellaneousError::ErrorCode70101 => r#"## Error - 70101

### Unsupported Public Key Algorithm
The Request tries to store a Public Key in Twilio. This public key can be parsed, but the request fails, because the key uses an unsupported algorithm.

#### Possible Causes 
* Wrong public key algorithm. Only supported algorithm is RSA.

#### Possible Solution
* Regenerate a new keypair with RSA encryption and retry."#,
            TwilioMiscellaneousError::ErrorCode21243 => r#"## ERROR - 21243

### Credential Validation Error

One of the parameters passed when creating or updating a Credential was incorrect.  Please see the response body for more details. 

#### Possible Causes
* One of the parameters passed when creating or updating a Credential was incorrect.

#### Possible Solutions
* Please see the response body for more details."#,
            TwilioMiscellaneousError::ErrorCode51123 => r#"## Error - 51123

### Upstream not resolved

#### Possible Causes 
Specified upstream couldn't be resolved.

#### Possible Solutions
Verify that upstream is valid and passed correctly. "#,
            TwilioMiscellaneousError::ErrorCode20023 => r#"## ERROR - 20023

### Phone number is not correct: it cannot be null or have non-decimal symbols

 Phone number is not correct: it cannot be null and have non-decimal symbols

#### Possible Causes
* The phone number entered is not correct: either it is null or it has non-decimal symbols.

#### Possible Solutions
* Check the phone number"#,
            TwilioMiscellaneousError::ErrorCode70151 => r#"## Error - 70151

### Maximum Number Of API Keys Exceeded
The account reached the maximum limit of API Keys allowed. 

#### Possible Solution:
* Delete unused API Keys before creating additional ones."#,
            TwilioMiscellaneousError::ErrorCode51002 => r#"## Error - 51002

### Client Connection: Request rate limit exceeded

#### Possible Causes 

The rate of client requests exceeded the limit.

#### Possible Solutions

Verify that your application is not misbehaving and the load is expected. If everything is as expected then contact Customer Support to increase your Account's client request limit.
"#,
            TwilioMiscellaneousError::ErrorCode80102 => r#"## Warning

### Participant Already In Interaction

Example Message: Participant has already been added to Interaction"#,
            TwilioMiscellaneousError::ErrorCode70003 => r#"## Error - 70003

### Outdated Entity
The request tries to update a resource that has been already updated (by someone else). Only applies if the resource uses a version field.

#### Possible Causes 
* When sending an update request that contains field: “version: 1” and the server recognizes that the actual version of the resource is already 2.

#### Possible Solution:
* Reload the resource with a GET request to get the latest version."#,
            TwilioMiscellaneousError::ErrorCode90024 => r#"## ERROR - 90024

### Template body has tag which is not provided in 'TemplateArgs'

 Template body has tag which is not provided in 'TemplateArgs'

#### Possible Causes
Template body has wider tag's set than specified in 'TemplateArgs' dictionary

#### Possible Solutions
Check all tags from template body are specified in 'TemplateArgs' dictionary as keys"#,
            TwilioMiscellaneousError::ErrorCode13255 => r#"# Warning - 13255

### Dial->Sip: Dialing .sip.twilio.com addresses is not currently allowed

At this time internal routing of SIP calls within Twilio is not supported i.e. dialing .sip.twilio.com addresses from a Twilio number."#,
            TwilioMiscellaneousError::ErrorCode14215 => r#"## Error - 14215

### Dial->Queue: Invalid ReservationSid. Unable to dequeue

Provided ReservationSid is invalid.
"#,
            TwilioMiscellaneousError::ErrorCode14102 => r#"## Warning - 14102 

### Message "From" Attribute is Invalid

The From phone number does not appear to be a phone number that you can message from

#### Possible Causes 
*  Using a ‘From’ number that is not assigned to your account
*  Using a ‘From’ number that is not SMS/MMS enabled


#### Possible Solutions
*  Check to make sure the number is correct
*  Check to make sure the number is an SMS/MMS enabled Twilio Incoming number assigned to your account

"#,
            TwilioMiscellaneousError::ErrorCode12200 => r#"## WARNING - 12200

### Schema validation warning

The provided XML does not conform to the Twilio Markup XML schema.  Please refer to the specific error and correct the problem. ## Warning - 12200 

### Schema validation warning

The provided XML does not conform to the Twilio Markup XML schema.  Please refer to the specific error and correct the problem.

#### Possible Causes
* Misspelled verbs
* Incorrect case for verbs; TwiML is case-sensitive, so &lt;Say&gt; works, but &lt;say&gt; will not
* Misspelled or unknown attributes
* Unknown or unexpected nested elements.

#### Possible Solutions
* Check the line and column reported by the warning to see what part of your XML response caused the complaint
* Verify that your application server is accepting and properly responding to any [custom parameters](https://support.twilio.com/hc/en-us/articles/115011213347) you may be passing"#,
            TwilioMiscellaneousError::ErrorCode94501 => r#"## ERROR - 94501

### Transcriptions: Status invalid

 The action could not be performed because the Status provided is not valid.

#### Possible Causes
 * Status value is invalid.

#### Possible Solutions
 * Value must be enqueued, processing, completed, failed or deleted."#,
            TwilioMiscellaneousError::ErrorCode80907 => r#"## Warning

### Open Interaction Not Found

Example Message: Received a call with no open interaction"#,
            TwilioMiscellaneousError::ErrorCode61009 => r#"## Error - 61009

### Add-ons: Could not fulfill request with available data

#### Possible Causes 
*   One of the request fields in the Add-ons specification could not be interpolated with the provided data. The result is an invalid URL, Header value or other parameter that makes the request impossible to execute."#,
            TwilioMiscellaneousError::ErrorCode20006 => r#"## Error - 20006

### Access Denied

You don't have sufficient privileges to access the requested resource. The user
on whose behalf you are attempting to access the API may have revoked your
privileges, or you may need to request greater privileges from the user.

"#,
            TwilioMiscellaneousError::ErrorCode14110 => r#"## Warning - 14110 

### Invalid Verb for Message Reply

An Invalid Verb has been passed back in a Message reply.  Valid Verbs are Message, Sms and Redirect

"#,
            TwilioMiscellaneousError::ErrorCode80104 => r#"## Warning

### Phone Number Already In Service

Example Message: PhoneNumber has already been added to Service"#,
            TwilioMiscellaneousError::ErrorCode11215 => r#"## Error - 11215 

### HTTP too many redirects

this request has been redirected too many times and may be in a loop.

#### Possible Causes 
*  Twilio received too many HTTP redirect messages in a row for this request.


#### Possible Solutions
*  Make certain your code is not stuck in an infinite loop.

"#,
            TwilioMiscellaneousError::ErrorCode14210 => r#"## Error - 14210

### Dial->Queue: Invalid whisper method

The whisper method is invalid. You must specify one of 'GET' or 'POST'.

"#,
            TwilioMiscellaneousError::ErrorCode21205 => r#"## Error - 21205

### Invalid URL

You attempted to initiate an outbound phone call, but the URL you specified to
handle the call was not a valid URL.

If you specified an Application Sid for your outbound phone call, the
application must have a valid VoiceUrl or the call will fail.

#### Possible Solutions

Make sure you submit a fully qualified URL including:

*   protocol (http:// or https://)
*   hostname
*   file path
*   properly url-encoded query parameters

Twilio must be able to reach this URL over the public Internet.
"#,
            TwilioMiscellaneousError::ErrorCode19036 => r#"## ERROR - 19036

### Invalid page token

 The page token provided in the request is not valid

#### Possible Causes
Page token provided is not an integer

#### Possible Solutions
Page token must be an integer"#,
            TwilioMiscellaneousError::ErrorCode60001 => r#"## ERROR - 60001

### Downstream Authentication Failed

 Downstream Authentication Failed

#### Possible Causes
The request was received and passed Twilio API authentication, but failed due to a downstream authentication error for the Account SID provided.

#### Possible Solutions
- Check that the Account SID provided is correct.
- [Contact Twilio Support](https://www.twilio.com/help/contact) to confirm that access to this endpoint is enabled for the given Account SID."#,
            TwilioMiscellaneousError::ErrorCode70154 => r#"## Error - 70154

### Public Key Is Invalid
The Public Key provided to Twilio is invalid. This is a Public Key Client Validation Error.

#### Possible Solution:
* Upload a new, valid Public Key.
* For details on the Credentials Endpoint, see [the documentation](/docs/api/credentials/rest)"#,
            TwilioMiscellaneousError::ErrorCode14103 => r#"## Warning - 14103 

### Message Invalid Body

The Message body does not contain valid text or a media URL

#### Possible Causes 
*  Empty SMS body with no media URL specified
*  SMS Body exceeding 1600 characters 


#### Possible Solutions
*  Make sure you're not sending a blank message
*  Make sure the text of the message you are sending is 1600 characters or less.

"#,
            TwilioMiscellaneousError::ErrorCode13201 => r#"## Error - 13201 

### Dial: Cannot Dial out from a Dial Call Segment

TwiML documents that execute on the called party's end (via the Dial->Number URL attribute, before the parties are bridged) cannot issue a new Dial.





"#,
            TwilioMiscellaneousError::ErrorCode13311 => r#"## Warning - 13311 

### Gather: Invalid finishOnKey value

The value of the finishOnKey attribute must be one of the following characters "0123456789#*". See the <a href='/docs/api/twiml/gather#finishOnKey'>Gather Verb</a> API Reference for more information.





"#,
            TwilioMiscellaneousError::ErrorCode11240 => r#"## ERROR - 11240

### HTTP connection edge location is invalid

 There was an error connecting to the URL due to an invalid edge location parameter.

#### Possible Causes
* The edge location(s) specified in the 'e' parameter of your connection settings are not valid

#### Possible Solutions
* Correct the 'e' parameter in the fragment part of the URL, the one after '#'."#,
            TwilioMiscellaneousError::ErrorCode11220 => r#"## ERROR - 11220

### SSL/TLS Handshake Error

During SSL/TLS negotiation, Twilio experienced a connection reset. 

#### Possible Causes
*  Incompatible cipher suites in use by the client and the server. This would require the client to use (or enable) a cipher suite that is supported by the server.

#### Possible Solutions
*  Verify cipher suites in-use are up to date. Twilio-supported ciphers can be found [here](https://support.twilio.com/hc/en-us/articles/360007724794-Notice-Twilio-REST-API-s-TLS-and-Cipher-Suite-Security-Changes-for-June-2019)
*  Use compatible version of TLS, Twilio supports TLS 1.2."#,
            TwilioMiscellaneousError::ErrorCode21470 => r#"## Error - 21470

### Invalid AccountSid

The AccountSid you specified was not a valid AccountSid identifier.  AccountSid's are 34 character strings that represent the Twilio Account .  For example, AC44CE4123947237834573457345347567.





"#,
            TwilioMiscellaneousError::ErrorCode19020 => r#"## ERROR - 19020

### Contact with the provided channel value already exists

 Channel value already exists 

#### Possible Causes
An existing contact has the same channel value provided in the request 

#### Possible Solutions
Channel value has to be unique for contact, use different channel value or update the existing one"#,
            TwilioMiscellaneousError::ErrorCode30013 => r#"## Error - 30013

## Message Time To Live is too big

### Possible Causes 
TTL specified in the request is too big

### Possible Solutions
Specify a smaller TTL
"#,
            TwilioMiscellaneousError::ErrorCode51101 => r#"## Error - 51101

### Service instance not found

#### Possible Causes 
Service instance specified in the request does not exist.

#### Possible Solutions
Confirm a valid service instance is being passed in the request. "#,
            TwilioMiscellaneousError::ErrorCode94603 => r#"## ERROR - 94603

### Ttl is out of range

 The action could not be performed because the TTL provided is out of range.

#### Possible Causes
 * Ttl is smaller than 1 seconds
 * Ttl is bigger  than 3600 seconds

#### Possible Solutions
 * Ttl value must be between 1 and 3600 seconds"#,
            TwilioMiscellaneousError::ErrorCode51117 => r#"## Error - 51117

### Invalid access token signature

#### Possible Causes 
The token has an invalid signature.

#### Possible Solutions
Confirm a valid and signed token is being passed.

To check whether signed correctly, you can use the tools available at [jwt.io](https://jwt.io)."#,
            TwilioMiscellaneousError::ErrorCode21617 => r#"## ERROR - 21617

### The concatenated message body exceeds the 1600 character limit

 


#### Possible Causes
The maximum allowable body text length is generally 1600 characters, but some special characters such as emojis, emoticons or non-GSM characters will be counted as multiple characters if they have a higher byte count. 

#### Possible Solutions
An online text to byte calculator can help you understand if certain characters are taking up more character space than others."#,
            TwilioMiscellaneousError::ErrorCode90040 => r#"## ERROR - 90040

### Broadcast 'MediaUrls' list has too many items

 Broadcast 'MediaUrls' list has too many items

#### Possible Causes
Broadcast request has too many 'MediaUrl' fields 

#### Possible Solutions
Use fewer 'MediaUrl' fields in Broadcast request"#,
            TwilioMiscellaneousError::ErrorCode90021 => r#"## ERROR - 90021

### One of 'TemplateArgs' dictionary key is too long

 One of 'TemplateArgs' dictionary key is too long

#### Possible Causes
One of 'TemplateArgs' dictionary key is too long

#### Possible Solutions
Use shorter 'TemplateArgs' dictionary keys"#,
            TwilioMiscellaneousError::ErrorCode84000 => r#"## WARNING - 84000

### Execution has exceeded max steps allowed for a flow.

 This limitation is enforced to prevent infinite loops. Please try to design your flows such that they terminate.

#### Possible Causes
There are too many steps in the flow

#### Possible Solutions
Reduce the number of steps in your flow."#,
            TwilioMiscellaneousError::ErrorCode80401 => r#"## Warning

### Account Sid Invalid

Example Message: Invalid Account Sid"#,
            TwilioMiscellaneousError::ErrorCode13325 => r#"## Error - 13325 

### Gather->Play: Invalid Content-Type

Play requires an audio Content-Type

#### Possible Causes 
*  Play references a URL that does not return audio or is returning an invalid Content-Type.


#### Possible Solutions
*  Make certain the URL returns an audio file	
*   See the <a href='/docs/api/twiml/play'>Play Verb</a> API Reference for more information on valid Content-Types.	




"#
        }
    }
}
