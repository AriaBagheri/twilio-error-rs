// AUTO-GENERATED. DO NOT MODIFY. YOUR CHANGES WILL BE OVER-WRITTEN
use crate::products::TwilioVerifyError;
use standard_error::traits::StandardErrorDocsTrait;

impl StandardErrorDocsTrait for TwilioVerifyError {
    fn docs(&self) -> &'static str {
        match self {
            TwilioVerifyError::ErrorCode60367 => r#"## ERROR - 60367

### Invalid entity identity

 

#### Possible Causes
The Entity identity in the request is missing or incorrectly formatted.

#### Possible Solutions
Check that the Entity identity in the request body is correct."#,
            TwilioVerifyError::ErrorCode60215 => r#"## ERROR - 60215

### Max number of mailers per account reached

 Max number of mailers per account reached

HTTP status: 400

#### Possible Causes
Your account has reached the max number of Mailers

#### Possible Solutions
Delete/update Mailers that you are not using at the moment"#,
            TwilioVerifyError::ErrorCode60378 => r#"## ERROR - 60378

### Authenticator response invalid or not provided

 

#### Possible Causes
The Authenticator response in the request is missing or incorrectly formatted.

#### Possible Solutions
Check that the Authenticator response in the request body is correct."#,
            TwilioVerifyError::ErrorCode60374 => r#"## ERROR - 60374

### Invalid page token

 

#### Possible Causes
The Page token in the request is incorrectly formatted

#### Possible Solutions
Check that the Page token in the request body is correct."#,
            TwilioVerifyError::ErrorCode60373 => r#"## ERROR - 60373

### Invalid page size

 

#### Possible Causes
The Page size in the request is missing or incorrectly formatted.

#### Possible Solutions
Check that the Page size in the request body is correct."#,
            TwilioVerifyError::ErrorCode60213 => r#"## ERROR - 60213

### A Messaging Configuration already exists for the given country

 * You have already configured the given country.

HTTP status: 400


#### Possible Causes
* You have already configured the given country.

#### Possible Solutions
* Try updating the Messaging Configuration instead."#,
            TwilioVerifyError::ErrorCode60424 => r#"## ERROR - 60424

### Contact address not found

 

#### Possible Causes
The contact does not contain relevant address information.

#### Possible Solutions
Try updating the contact with the appropriate address information."#,
            TwilioVerifyError::ErrorCode60388 => r#"## ERROR - 60388

### User display name not provided

 User display name not provided

#### Possible Causes
The User display name in the request is missing.

#### Possible Solutions
Check that the User display name in the request body is provided."#,
            TwilioVerifyError::ErrorCode60365 => r#"## ERROR - 60365

### Entity SID invalid or not provided

 

#### Possible Causes
The Entity SID in the request is missing or incorrectly formatted.

#### Possible Solutions
Check that the Entity SID in the request parameter/body is correct."#,
            TwilioVerifyError::ErrorCode60237 => r#"## ERROR - 60237

### The given domain is on a deny list

 

#### Possible Causes
Some generic domains have been blocked for specific functionalities. The domains in the deny list are: Gmail, Yahoo, Hotmail, Outlook.

#### Possible Solutions
Please use a different email with a custom domain."#,
            TwilioVerifyError::ErrorCode60312 => r#"## ERROR - 60312

### Challenge creation limit reached

 The maximum number of Challenge creations for a Factor has been reached.

Max. number of attempts for a Factor with `Config.CodeLength` less or equal to 4: 10 every 24 hours.

Max. number of attempts for a Factor with `Config.CodeLength` greater or equal to 5: 50 every hour, and up to 100 every 24 hours.

HTTP status: 429

#### Possible Causes
Too many Challenges created within a time period.

#### Possible Solutions
- Create a new Factor and verify it. Then create new Challenges under the new Factor.
- Depending on the Factor's `Config.CodeLength` parameter, await rate limit cooldown period between 1 hour and 24 hours."#,
            TwilioVerifyError::ErrorCode60318 => r#"## ERROR - 60318

### Factor is unverified

 HTTP status: 403

#### Possible Causes
The Factor has not been verified.

#### Possible Solutions
Verify the Factor first."#,
            TwilioVerifyError::ErrorCode60234 => r#"## ERROR - 60234

### Sender id can not be set as not default

 Sender id can not be set as not default

#### Possible Causes
The sender id that's being updated is default and it's not allowed to set default as false

#### Possible Solutions
Set another sender id as default"#,
            TwilioVerifyError::ErrorCode68006 => r#"## ERROR - 68006

### Initialization Error

 An error occurred while initializing a class.

#### Possible Causes
Missing arguments to initialize a class.

#### Possible Solutions
Check parameters and retry."#,
            TwilioVerifyError::ErrorCode60235 => r#"## ERROR - 60235

### Sender Definitions does not match with requirements

 Sender definitions does not match with requirements

#### Possible Causes
The sender definitions contains invalid properties

#### Possible Solutions
Validate the object, it contains invalid requirements such as an invalid name. Sender id must have at least 3 characters and up to 11 characters."#,
            TwilioVerifyError::ErrorCode60407 => r#"## ERROR - 60407

### Authentication Token Error

 An error occurred while generating a token.

#### Possible Causes
Unsupported factor type.

#### Possible Solutions
Check parameters and retry."#,
            TwilioVerifyError::ErrorCode60306 => r#"## ERROR - 60306

### Invalid Request

 The request is invalid.

HTTP status: 400

#### Possible Causes
One or more parameters are in an invalid format:

- Invalid `AccountSid`.
- Invalid `ServiceSid`.
- Invalid `Identity`.
- Invalid Factor parameters: Invalid `FriendlyName`, `Binding`, etc.
- Invalid `AuthPayload` when verifying a factor or challenge.
- Invalid `FactorSid`.
- Invalid list parameters: Invalid `Page`, `PageSize` or `PageToken`.
- Invalid Challenge parameters: Invalid `FactorSid`, `ExpirationDate`, `Details` or `HiddenDetails`.
- Invalid `ChallengeSid`.
- Invalid Factor configuration.
- Invalid `JWT`.
- Invalid Webhooks parameters: Invalid `EventTypes`

#### Possible Solutions
Review the parameters you are passing from the list above and try again. Accepted parameters are detailed in the [API reference](https://www.twilio.com/docs/verify/api)."#,
            TwilioVerifyError::ErrorCode60236 => r#"## ERROR - 60236

### The domain could not be obtained from the given email

 The domain could not be obtained from the given email

#### Possible Causes
Could not obtain the domain from the email provided

#### Possible Solutions
Please review the email provided or the email registered of your account administrator users"#,
            TwilioVerifyError::ErrorCode60421 => r#"## ERROR - 60421

### Unexpected result when creating contact

 

#### Possible Causes
Some unexpected error was appeared during creating new Contact.

#### Possible Solutions
Check that the contact information in the request body is correct and try again."#,
            TwilioVerifyError::ErrorCode60218 => r#"## ERROR - 60218

### SendGrid Template is not active

 SendGrid Template is not active

HTTP status: 400

#### Possible Causes
The provided template is not active

#### Possible Solutions
Log into SendGrid's dashboard and activate the template, or select a different template"#,
            TwilioVerifyError::ErrorCode60310 => r#"## ERROR - 60310

### Factor verification attempts reached

 The maximum number of verification attempts for a Factor has been reached.

Max. Number of attempts for a TOTP Factor: 5.

HTTP status: 429

#### Possible Causes
Too many incorrect TOTP codes have been provided in the AuthPayload

#### Possible Solutions
- Create a new Factor and verify that one..
- For a TOTP Factor, ask the user to provide the correct TOTP code or create a new Factor if they are unable to generate the correct codes for the Factor they are verifying."#,
            TwilioVerifyError::ErrorCode60381 => r#"## ERROR - 60381

### Signature invalid or not provided

 

#### Possible Causes
The Signature in the request is missing or incorrectly formatted.

#### Possible Solutions
Check that the Signature in the request body is correct."#,
            TwilioVerifyError::ErrorCode60377 => r#"## ERROR - 60377

### Authenticator attachment invalid or not provided

 

#### Possible Causes
The Authenticator attachment in the request is missing or incorrectly formatted.

#### Possible Solutions
Check that the Authenticator attachment in the request body is correct."#,
            TwilioVerifyError::ErrorCode60382 => r#"## ERROR - 60382

### Factor does not match the relying party of the challenge

 

#### Possible Causes
The Relying party is specified in the request, but it is incorrect.

#### Possible Solutions
Check that the Relying party in the request body is correct."#,
            TwilioVerifyError::ErrorCode60440 => r#"## ERROR - 60440

### Unsupported verification content type

 

#### Possible Causes
The requested verification content type is not currently supported. 

#### Possible Solutions
Try using a different verification content type"#,
            TwilioVerifyError::ErrorCode60211 => r#"## ERROR - 60211

### Bucket with the given Interval already exists

 You have already configured that Max number of requests for this `Interval`

HTTP status: 400


#### Possible Causes
You have already configured that Max number of requests for this `Interval`

#### Possible Solutions
* Delete/Update Bucket that has this same `Interval`"#,
            TwilioVerifyError::ErrorCode60324 => r#"## ERROR - 60324

### Challenge verification failed

 Verification attempt was unsuccessful, AuthPayload is invalid.

HTTP status: 403

#### Possible Causes
- For a Push Challenge, the signature in AuthPayload is invalid.  
- For a TOTP Challenge, the OTP code in AuthPayload is invalid.

#### Possible Solutions
- For a Push Challenge, check the signature sent in AuthPayload and retry.  
- For a TOTP Challenge, check the OTP code sent in AuthPayload and retry."#,
            TwilioVerifyError::ErrorCode60240 => r#"## WARNING - 60240

### Templates not allowed

 Your account does not have enough permissions to use templates

#### Possible Causes
Some countries have special requirements that can override the message to be compliant with the country thus preventing the use of templates.

#### Possible Solutions
Please contact the support team to validate the requirements needed to be compliant with the country and avoid the override of the message. "#,
            TwilioVerifyError::ErrorCode60205 => r#"## ERROR - 60205

### SMS is not supported by landline phone number

 You have attempted to send the verification code to phone number that does not support SMS.

HTTP status: 403

#### Possible Causes
You have attempted to send the verification code to phone number that does not support SMS.

#### Possible Solutions
* Make sure the number you are sending a verification to is a valid mobile phone number
* Disable validating phone numbers using the Twilio API setting `SkipSmsToLandlines` on your Verification Service to `false`."#,
            TwilioVerifyError::ErrorCode60519 => r#"## ERROR - 60519

### SNA Verification Result Pending

 

#### Possible Causes
The Verification request was received and is still pending completion. 

This can happen when:

- A Verification Check is performed on a Verification that is still in progress.
- A Verification was created for a phone number whose carrier is not supported for the selected Twilio Region. For example, if an SNA Verification was created for a UK phone number using the US1 Region instead of the IE1 Region. In this case, the Verification request will never complete.

#### Possible Solutions
- Confirm the Verification Check is performed only after a HTTP 200 Response is received from invoking the SNA URL.
- Retry if the Verification is still in its validity window and has not expired.
- Confirm the Verification was created in the correct Twilio Region for the phone number, see [Using Verify Silent Network Auth with Twilio Regions
](https://www.twilio.com/docs/verify/using-verify-silent-network-auth-with-twilio-regions) for more detailed information. If it was not, retry Verification using correct Region."#,
            TwilioVerifyError::ErrorCode60247 => r#"## ERROR - 60247

### Message Length Exceeded

 The message body generated for your verification is too long. The maximum number of characters allowed is 500. 

#### Possible Causes
1. Template Substitutions: If you are using a template, ensure that it, including any variable substitutions, does not exceed the 500-character limit.
2. Friendly Name, Code, or Custom Substitutions: Review your message's friendly name, code, or custom substitutions. These elements might be increasing the message length and causing it to exceed the limit.

#### Possible Solutions
1. Template Substitutions: 
    * Review the length of dynamic variables or placeholders in your template, such as {{customer_name}} or {{order_number}}. These values might be longer than expected once replaced. 
    * Simplify or abbreviate the content of your template. For example, instead of "Dear {{customer_name}}, we are pleased to inform that your code is {{code}}", try "Hi {{customer_name}}, your code is {{code}}"
2. Friendly Name Substitutions:
    * If you're using a friendly name for identification, ensure it's concise. 
3. Code Substitutions:
    * Check for codes that are dynamically added to the message. Long codes may need to be shortened. For example, you could change the code length of your verify service.
4. Custom Substitutions:
    * Look for unnecessary custom substitutions that may be inflating the message length. 

These substitutions are focused on shortening the content without losing clarity, helping users keep their messages within the 500-character limit."#,
            TwilioVerifyError::ErrorCode60229 => r#"## ERROR - 60229

### Template translation was not found

 ## Error - 60229

### Invalid Parameter

The translation locale for the template was not found

#### Possible Causes
The user has provide a translation for a templated that not exists

#### Possible Solutions
Create a translation or use a existent translation for the template"#,
            TwilioVerifyError::ErrorCode60246 => r#"## ERROR - 60246

### Service already exists

 Service already exists

#### Possible Causes
* You tried to create a service with a UniqueName that already exists.

#### Possible Solutions
* Use a different UniqueName."#,
            TwilioVerifyError::ErrorCode60375 => r#"## ERROR - 60375

### Invalid id

 

#### Possible Causes
The ID in the request is incorrectly formatted.

#### Possible Solutions
Check that the ID in the request body is correct."#,
            TwilioVerifyError::ErrorCode60425 => r#"## ERROR - 60425

### Verification SID invalid or not provided

 

#### Possible Causes
The Verification ID in request has invalid format

#### Possible Solutions
Check that the Verification ID in the request parameters or body is correct."#,
            TwilioVerifyError::ErrorCode60431 => r#"## ERROR - 60431

### Verification not found

 

#### Possible Causes
The requested Verification is missing

#### Possible Solutions
Check that the Verification in the request parameter/body is correct."#,
            TwilioVerifyError::ErrorCode60500 => r#"## ERROR - 60500

### SNA Phone Number Mismatch

 

#### Possible Causes
The Verification request and processing was successful but the phone number provided in the request does not match the phone number detected via the carrier. This can happen when there is dual SIM usage or the end user is trying to verify a phone number that does not belong to the SIM.

#### Possible Solutions
- Check that the phone number provided is correct.
- Retry Verification using a channel other than Silent Network Auth for this end user."#,
            TwilioVerifyError::ErrorCode60371 => r#"## ERROR - 60371

### Relying party invalid or not provided

 

#### Possible Causes
The config for relying party in the request is missing or incorrectly formatted.

#### Possible Solutions
Check that the config for relying party in the request body is correct."#,
            TwilioVerifyError::ErrorCode60401 => r#"## ERROR - 60401

### Network Error

 An error occurred while calling the API.

#### Possible Causes
It was not possible to communicate properly with the API.

#### Possible Solutions
Check parameters and retry"#,
            TwilioVerifyError::ErrorCode60410 => r#"## ERROR - 60410

### Verification delivery attempt blocked

A verification delivery attempt was blocked.  This is because [Fraud Guard](/docs/verify/preventing-toll-fraud/sms-fraud-guard) has identified potential fraudulent messages being sent to one or more destination. A temporary block on SMS traffic has been placed for the next 12 hours on the prefix you used. This block will be lifted at the end of this 12-hour period. If further fraudulent activity is detected after this block, this pattern will continue with further temporary 12 hour blocks until the issue is no longer observed.

#### Possible Causes
We [identified SMS fraud](https://www.twilio.com/docs/verify/preventing-toll-fraud/sms-fraud-guard) occurring within your Verify product.

#### Possible Solutions
You do not need to take any specific action. The block is temporary and will resolve in 12 hours if we do not detect additional fraud."#,
            TwilioVerifyError::ErrorCode60604 => r#"## ERROR - 60604

### SendGrid Authenticated user is not authorized to send mail

SendGrid Authenticated user is not authorized to send mail SendGrid Authenticated user is not authorized to send mail

HTTP status: 401

#### Possible Causes
SendGrid API key have no send emails privileges

#### Possible Solutions
Update your API key permissions"#,
            TwilioVerifyError::ErrorCode60311 => r#"## ERROR - 60311

### Factor verification failed

 Factor verification failure.

HTTP status: 403

#### Possible Causes
It was not possible to verify a Factor.

#### Possible Solutions
Check parameters and retry."#,
            TwilioVerifyError::ErrorCode60517 => r#"## ERROR - 60517

### SNA User-Agent Mismatch Error

 

#### Possible Causes
The Verification request was received but failed during processing due to a User-Agent mismatch detected during redirects.

#### Possible Solutions
Retry Verification using a channel other than Silent Network Auth for this end user."#,
            TwilioVerifyError::ErrorCode60245 => r#"## ERROR - 60245

### You have exceeded your messaging limits

 You have exceeded your messaging limits

#### Possible Causes
This error is returned by SendGrid if you've sent more emails than your account can handle. This error can occur for a number of reasons, including daily or hourly limits, or limits on the number of messages that can be sent at once. SendGrid uses one credit for each email, and credits renew at the beginning of each month.

#### Possible Solutions
Contact SendGrid support to remove that limit or change your SendGrid plan. "#,
            TwilioVerifyError::ErrorCode60369 => r#"## ERROR - 60369

### Factor type invalid or not provided

 

#### Possible Causes
The Factor type in the request is missing or incorrectly formatted.

#### Possible Solutions
Check that the Factor type in the request body is correct."#,
            TwilioVerifyError::ErrorCode60441 => r#"## ERROR - 60441

### Illegal status

 

#### Possible Causes
An unexpected verification status returned.

#### Possible Solutions
Call Twilio support team"#,
            TwilioVerifyError::ErrorCode60602 => r#"## ERROR - 60602

### App hash can only be used with SMS channel

App hash can only be used with SMS channel App hash can only be used with SMS channel

HTTP status: 400

#### Possible Causes
The customer is using App Hash without SMS channel

#### Possible Solutions
do not use app Hash with channel other than SMS"#,
            TwilioVerifyError::ErrorCode60433 => r#"## ERROR - 60433

### Unsupported passkeys approval content type

 

#### Possible Causes
The requested approval content type is not currently supported.

#### Possible Solutions
Try using a different approval content type"#,
            TwilioVerifyError::ErrorCode60212 => r#"## ERROR - 60212

### Too many concurrent requests for phone number

 Your application is initiating many verifications in a short period of time for the same phone number.

HTTP status: 429


#### Possible Causes
Your application is initiating many verifications in a short period of time for the same phone number.

#### Possible Solutions
* Check if your application is being the target of fraudulent traffic.
* Review your implementation."#,
            TwilioVerifyError::ErrorCode60208 => r#"## ERROR - 60208

### Rate limit with that UniqueName already exists

 You tried to create a rate limit with a `UniqueName` that already exists for the specified service

HTTP status: 400


#### Possible Causes
You tried to create a rate limit with a `UniqueName` that already exists for the specified service

#### Possible Solutions
* Use a different `UniqueName`
* Create a new service and create a rate limit with the desired `UniqueName`"#,
            TwilioVerifyError::ErrorCode60385 => r#"## ERROR - 60385

### Public key invalid or not provided

 

#### Possible Causes
The Public key in the request is missing or incorrectly formatted.

#### Possible Solutions
Check that the Public key in the request body is correct."#,
            TwilioVerifyError::ErrorCode60532 => r#"## ERROR - 60532

### SNA Potential Dual SIM Detected

 

#### Possible Causes
The Verification request was received and failed due to the request potentially coming from a dual SIM device. In this case, the carrier identified via IP address does not match the carrier identified via phone number and Silent Network Auth cannot be completed. This can also happen when multiple people share one device.

#### Possible Solutions
Retry Verification using a channel other than Silent Network Auth for this end user."#,
            TwilioVerifyError::ErrorCode60386 => r#"## ERROR - 60386

### Attestation object invalid or not provided

 

#### Possible Causes
The Attestation object in the request is missing or incorrectly formatted.

#### Possible Solutions
Check that the Attestation object in the request body is correct."#,
            TwilioVerifyError::ErrorCode60390 => r#"## ERROR - 60390

### Factor not found

 

#### Possible Causes
The requested Factor is missing

#### Possible Solutions
Check that the Factor in the request parameter/body is correct."#,
            TwilioVerifyError::ErrorCode60361 => r#"## ERROR - 60361

### Account SID is invalid or not provided

 

#### Possible Causes
The Account SID in the request is missing or incorrectly formatted.

#### Possible Solutions
Check that the Account SID in the request parameter/header is correct."#,
            TwilioVerifyError::ErrorCode60408 => r#"## ERROR - 60408

### TemplateSid is only supported for the SMS channel

 TemplateSid is only supported for the SMS channel

#### Possible Causes
Creating a verification on CALL or EMAIL with TemplateSid parameter.

#### Possible Solutions
Stop sending the TemplateSid param with non SMS verifications"#,
            TwilioVerifyError::ErrorCode60323 => r#"## ERROR - 60323

### Challenge expired

 Challenge verification window expired.

HTTP status: 403

#### Possible Causes
The Challenge was not responded within the window of time.

#### Possible Solutions
Create a new Challenge."#,
            TwilioVerifyError::ErrorCode60411 => r#"## ERROR - 60411

### Phone Number already exists

 The phone number already exists in the SafeList.

#### Possible Causes
* Multiple requests were made to add the phone number to the SafeList.
* The phone number was already added at an earlier time.

#### Possible Solutions
If you know this phone number is present in the SafeList, do nothing."#,
            TwilioVerifyError::ErrorCode60209 => r#"## ERROR - 60209

### UniqueName format is invalid

 `UniqueName` only supports alphanumeric characters and `-`, `_` or `.` 

HTTP status: 400


#### Possible Causes
`UniqueName` only supports alphanumeric characters and `-`, `_` or `.` 


#### Possible Solutions
* Remove unsupported characters from `UniqueName`"#,
            TwilioVerifyError::ErrorCode68002 => r#"## ERROR - 68002

### Mapper Error

 An error occurred while mapping an entity.

#### Possible Causes
Unexpected entity format.

#### Possible Solutions
Create a new factor."#,
            TwilioVerifyError::ErrorCode60325 => r#"## WARNING - 60325

### Translation for locale not found, using default

 The template you are using does not have the translation with the locale you specified.

#### Possible Causes
The translation might not exist.

#### Possible Solutions
If you want to add more translations, please contact support."#,
            TwilioVerifyError::ErrorCode60221 => r#"## ERROR - 60221

### No target verification specified

 Ensure that either a `To` or `VerificationSid` is provided

HTTP status: 400

#### Possible Causes
No `To` nor `VerificationSid` is provided in the request

#### Possible Solutions
Ensure that either a `To` or `VerificationSid` is provided"#,
            TwilioVerifyError::ErrorCode60363 => r#"## ERROR - 60363

### Service SID invalid or not provided

 

#### Possible Causes
The Service SID in the request is missing or incorrectly formatted.

#### Possible Solutions
Check that the Service SID in the request parameter/header is correct."#,
            TwilioVerifyError::ErrorCode60383 => r#"## ERROR - 60383

### Illegal factor status

 

#### Possible Causes
The requested operation cannot be performed on this Factor because the Factor is in an inconsistent state or has expired.

#### Possible Solutions
Complete the Factor or re-register the identity."#,
            TwilioVerifyError::ErrorCode60315 => r#"## ERROR - 60315

### Reached max limit of 20 push Factors associated per Entity

 * Reached max limit of 20 Factors of `FactorType` = `push` per Entity.
* Reached max limit of 100 Factors of `FactorType` = `totp` per Entity.

HTTP status: 400

#### Possible Causes
* You already have 20 `push` Factors created for the Entity.
* You already have 100 `totp` Factors created for the Entity.

#### Possible Solutions
Delete one of the existing Factors of the same `FactorType` to create a new one."#,
            TwilioVerifyError::ErrorCode60404 => r#"## ERROR - 60404

### Input Error

 An error occurred while loading input.

#### Possible Causes
Some inputs are invalid.

#### Possible Solutions
Check parameters and retry."#,
            TwilioVerifyError::ErrorCode60437 => r#"## ERROR - 60437

### Recipient type unsupported

 

#### Possible Causes
The requested recipient type is not currently supported. 

#### Possible Solutions
Try using a different recipient type"#,
            TwilioVerifyError::ErrorCode60534 => r#"## ERROR - 60534

### SNA Downstream Carrier Error

 

#### Possible Causes
The Verification request was received but failed during processing due to a downstream carrier error.

#### Possible Solutions
Retry Verification using a channel other than Silent Network Auth for this end user."#,
            TwilioVerifyError::ErrorCode60434 => r#"## ERROR - 60434

### Unsupported passkeys verification content type

 

#### Possible Causes
The requested verification content type is not currently supported.

#### Possible Solutions
Try using a different verification content type"#,
            TwilioVerifyError::ErrorCode60223 => r#"## ERROR - 60223

### Delivery channel disabled

Create verification failed because delivery channel is currently disabled 

#### Possible Causes
The delivery channel specified is currently disabled in the Verify Service settings

#### Possible Solutions
 * Enable delivery channel in Verify Service settings
 * Change the delivery channel specified in the create verification request"#,
            TwilioVerifyError::ErrorCode60368 => r#"## ERROR - 60368

### Challenge details invalid or not provided

 

#### Possible Causes
The Challenge details section in the request is missing or incorrectly formatted.

#### Possible Solutions
Check that the Challenge details in the request body is correct."#,
            TwilioVerifyError::ErrorCode60510 => r#"## ERROR - 60510

### SNA Error

 

#### Possible Causes
The Verification request was received but failed during processing due to an error during the Silent Network Auth process.

#### Possible Solutions
- Check that the mobile client is using a mobile data connection to invoke the SNA URL, not WiFi.
- Retry Verification using a channel other than Silent Network Auth for this end user."#,
            TwilioVerifyError::ErrorCode60228 => r#"## ERROR - 60228

### Template was not found

 ## Error - 60228

### Invalid Parameter

The selected template was not found

#### Possible Causes
The user has provide a TemplateSid that was not found

#### Possible Solutions
Provide a TemplateSid that exists"#,
            TwilioVerifyError::ErrorCode60232 => r#"## ERROR - 60232

### Sender id does not exist

 Sender id does not exist

#### Possible Causes
The sender id used in the request does not exist

#### Possible Solutions
Use a sender id that exists for the given account"#,
            TwilioVerifyError::ErrorCode60326 => r#"## ERROR - 60326

### Too many requests to create factors for the entity

 * Too many requests to create factors for the entity

HTTP status: 429

#### Possible Causes
* This entity tried to create more than 100 factors per day

#### Possible Solutions
Try again 24h later"#,
            TwilioVerifyError::ErrorCode60308 => r#"## ERROR - 60308

### Challenge verification attempts limit reached

 The maximum number of verification attempts for a Challenge has been reached.

Max. number of attempts for TOTP: 5.

HTTP status: 429

#### Possible Causes
Too many incorrect TOTP codes have been provided in the AuthPayload

#### Possible Solutions
- Create a new Challenge and verify that one.
- For TOTP, ask the user to provide the correct TOTP code or re-register them with a new Factor if they are unable to generate correct codes for the existing Factor."#,
            TwilioVerifyError::ErrorCode60317 => r#"## ERROR - 60317

### Factor already exists

 You are creating a Factor that already exists.

HTTP status: 409

#### Possible Causes
You are trying to create a Factor with the same binding parameters as an existing one.

#### Possible Solutions
Check that the binding parameters are not the same as those in an existing Factor."#,
            TwilioVerifyError::ErrorCode60242 => r#"## ERROR - 60242

### WhatsApp template not found

An approved WhatsApp template was not found for the given account sid and language An approved WhatsApp template was not found for the given account sid and language

#### Possible Causes
Selected account does not have whatsApp template created or approved

#### Possible Solutions
Create and request approval for a whatsApp template in the needed language"#,
            TwilioVerifyError::ErrorCode60202 => r#"## ERROR - 60202

### Max check attempts reached

 Max (5) verification check attempts reached.

HTTP status: 429

#### Possible Causes
You have attempted to check a verification more than 5 times and have reached the limit.

#### Possible Solutions
You have 5 attempts to check a verification code, after that you will need to wait until the current verification expires (10 minutes) to create a new verification. Find more recommendations for avoiding rate limits while testing Verify in [this blog post](https://www.twilio.com/blog/test-verify-no-rate-limits)."#,
            TwilioVerifyError::ErrorCode60380 => r#"## ERROR - 60380

### Client data invalid or not provided

 

#### Possible Causes
The Client data JSON in the request is missing or incorrectly formatted.

#### Possible Solutions
Check that the Client data JSON in the request body is correct."#,
            TwilioVerifyError::ErrorCode60200 => r#"## ERROR - 60200

### Invalid parameter

Parameter value is incorrectly formatted or is not an accepted option. The response `message` contains the name of the invalid parameter. 

HTTP status: 400

Example where `To` is the invalid parameter:
```
{
    "code": 60200,
    "message": "Invalid parameter: To",
    "more_info": "https://www.twilio.com/docs/errors/60200",
    "status": 400
}
```

#### Possible Causes
When [starting a verification](https://www.twilio.com/docs/verify/api/verification#start-new-verification):

* `To` parameter is incorrectly formatted or missing the leading `+`.
* `Channel` parameter is not one of the [accepted options](https://www.twilio.com/docs/verify/api/verification#start-new-verification), or you might not have access to the requested channel.

When [checking a verification](https://www.twilio.com/docs/verify/api/verification-check#check-a-verification):

* `Code` parameter is not between 4-10 numbers long.

When [creating a service](https://www.twilio.com/docs/verify/api/service#create-a-verification-service):

* `FriendlyName` is too long (max 30 characters).

When [fetching a service](https://www.twilio.com/docs/verify/api/service#fetch-a-service):

* Neither `ServiceSid` nor `UniqueName` is present.

#### Possible Solutions
* Use strict [E.164 formatting](https://www.twilio.com/docs/glossary/what-e164), including the `+` sign, for phone numbers in the `To` parameter. Example: `+15017122661`.
* If using the `sna` channel, ensure you have requested and been granted access. See [SNA Overview](https://www.twilio.com/docs/verify/sna) for details."#,
            TwilioVerifyError::ErrorCode60333 => r#"## WARNING - 60333

### The SMS message was sent using a brandless template.

 The SMS message was sent using a brandless template, instead of the template that you configured, which contains a friendly name (brand).

#### Possible Causes
Due to mobile carrier regulations, Verify needs to remove the friendly name from the message body. 
This is due to compliance, and our best-effort to ensure delivery of your OTP to certain restricted destinations.

#### Possible Solutions
Configure a dedicated Sender ID or phone number to send messages, instead of using a shared one.

To learn more, see <a href="https://support.twilio.com/hc/en-us/articles/12387480513307-Why-was-my-friendly-name-not-included-in-the-Verify-SMS-">Why was my friendly name not included in the Verify SMS?</a>"#,
            TwilioVerifyError::ErrorCode60313 => r#"## ERROR - 60313

### Unauthorized factor creation

 You are not authorized to create a Factor.

HTTP status: 403

#### Possible Causes
You do not have the permissions required to create a Factor.

#### Possible Solutions
Request permissions."#,
            TwilioVerifyError::ErrorCode60427 => r#"## ERROR - 60427

### Factor ID invalid or not provided

 

#### Possible Causes
The Factor ID in the request is missing or incorrectly formatted.

#### Possible Solutions
Check that the Factor ID in the request parameter or body is correct."#,
            TwilioVerifyError::ErrorCode60224 => r#"## ERROR - 60224

### Missing substitutions for selected template

 The selected template has one or more expected substitutions that were not provided in the request.

#### Possible Causes
- The template requires extra fields for building the message that are not included in the request.
- The substitutions were not provided in the correct format (stringified JSON).


#### Possible Solutions
- Include all the substitutions for the place holder variables with the `TemplateCustomSubstitution` parameter.
- Format the variable object as stringified JSON.

For example, the following template requires a **`uuid`** substitution:

"**Your {{friendly_name}} login link: https://example.com/verify.html?uuid={{uuid}}&code={{code}}**"

The expected request in cURL:
```bash
curl -X POST https://verify.twilio.com/v2/Services/VAXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXX/Verifications \
--data-urlencode "To=+15017122661" \
--data-urlencode "Channel=sms" \
--data-urlencode "TemplateSid=HJXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXX" \
--data-urlencode "TemplateCustomSubstitutions={ \"uuid\": \"MY_UNIQUE_ID\" }" \
-u $TWILIO_ACCOUNT_SID:$TWILIO_AUTH_TOKEN
```

The expected request in Node.js:
```javascript
const verification = await client.verify.v2
  .services("VAXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXX")
  .verifications.create({
    channel: "sms",
    templateSid: "HJXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXX",
    to: "+15017122661",
    templateCustomSubstitutions: '{ "uuid": "MY_UNIQUE_ID" }'
  });
```

The following variables are auto-populated by Twilio and do **not** need to be included in your request:

* `{{friendly_name}}`
* `{{code}}`
* `{{ttl}}`"#,
            TwilioVerifyError::ErrorCode60366 => r#"## ERROR - 60366

### Entity invalid or not provided

 

#### Possible Causes
The Entity in the request is missing or incorrectly formatted.

#### Possible Solutions
Check that the Entity in the request body is correct."#,
            TwilioVerifyError::ErrorCode60550 => r#"## ERROR - 60550

### Auto Channel Failed: None of the available channels could be selected due to validation errors. Check your debugger messages in console.

 Due to validation errors, none of the available specific channels could be selected. Check your debugger messages in the console for more details.

#### Possible Causes
- Specific delivery channel(s) are disabled
- Your service does not have access to an specific feature.

#### Possible Solutions
Check your debugger messages in the console to get the specific validation errors"#,
            TwilioVerifyError::ErrorCode60362 => r#"## ERROR - 60362

### Factor SID invalid or not provided

 

#### Possible Causes
The Factor SID in the request is missing or incorrectly formatted.

#### Possible Solutions
Check that the Factor SID in the request parameter is correct."#,
            TwilioVerifyError::ErrorCode60220 => r#"## ERROR - 60220

### Messages to China require use case vetting

 Messages to China require use case vetting, please refer to this support article https://support.twilio.com/hc/en-us/articles/17024185400859-Use-Case-Vetting-for-Verify-Messages-to-China to understand this in detail.


HTTP status: 400

#### Possible Causes
Your account is not allowed to create verifications towards China

#### Possible Solutions
Please read this support article https://support.twilio.com/hc/en-us/articles/17024185400859-Use-Case-Vetting-for-Verify-Messages-to-China to get your more details on how to get your use case vetted for China."#,
            TwilioVerifyError::ErrorCode60372 => r#"## ERROR - 60372

### Relying party id invalid or not provided

 

#### Possible Causes
The Relying party ID in the request is missing or incorrectly formatted.

#### Possible Solutions
Check that the Relying party ID in the request body is correct."#,
            TwilioVerifyError::ErrorCode60511 => r#"## ERROR - 60511

### SNA Downstream Error

 

#### Possible Causes
The Verification request was received but failed during processing due to a downstream error during the Silent Network Auth process. This can happen when the carrier was expecting a missing header, the phone number was invalid, the SNA URL was invoked on a carrier that does not support HTTPS, or there is an invalid redirect.

#### Possible Solutions
Retry Verification using a channel other than Silent Network Auth for this end user."#,
            TwilioVerifyError::ErrorCode60201 => r#"## ERROR - 60201

### Selected template translation is not approved

 Selected template translation is not approved.

#### Possible Causes
- A custom template was created but has not been approved.

#### Possible Solutions
Approve your template[s] in the Twilio Console: [console.twilio.com/us1/develop/verify/settings/templates](https://console.twilio.com/us1/develop/verify/settings/templates)

If your custom template is not listed, please reach out to Twilio Support to create a new template. Learn more about verification templates in the documentation: [twilio.com/docs/verify/api/templates](https://twilio.com/docs/verify/api/templates)"#,
            TwilioVerifyError::ErrorCode60436 => r#"## ERROR - 60436

### Recipient invalid or not provided

 

#### Possible Causes
The Recipient in the request is missing or incorrectly formatted.

#### Possible Solutions
Check that the Recipient in the request body is correct."#,
            TwilioVerifyError::ErrorCode60307 => r#"## ERROR - 60307

### Cannot resend push notifications to 'none' notification platform

 This error is returned when the resend push notification endpoint was called for a factor using "none" as notification platform in its configuration.

#### Possible Causes
You called the resend push notification endpoint for a factor with the "none" notification platform configured.

#### Possible Solutions
For factor configured with "none" as notification platform, it is not necessary to call the resend push notification endpoint."#,
            TwilioVerifyError::ErrorCode60330 => r#"## ERROR - 60330

### Failed to invoke the webhook

 HTTP status: 400

#### Possible Causes
- Unable to access the `webhook` **URL**
- The `webhook` is allowing **HTTP** requests other than **POST**
- The `webhook` is responding with a status code other than **200** (OK)

#### Possible Solutions
- Make sure the `webhook` **URL** is reachable
- Remember the `webhook` must allow **HTTP POST** requests
- Make sure the  `webhook` is responding with status code **200** when everything is going well"#,
            TwilioVerifyError::ErrorCode60376 => r#"## ERROR - 60376

### Invalid rawId

 

#### Possible Causes
The RawID in the request is incorrectly formatted.

#### Possible Solutions
Check that the RawID in the request body is correct."#,
            TwilioVerifyError::ErrorCode60520 => r#"## ERROR - 60520

### SNA URL Failed

 

#### Possible Causes
The Verification request was received and failed due to an error related to invoking the SNA URL.

#### Possible Solutions
- Retry Verification using a channel other than Silent Network Auth for this end user.
- [Contact Twilio Support](https://www.twilio.com/help/contact) to rule out integration issues."#,
            TwilioVerifyError::ErrorCode60300 => r#"## ERROR - 60300

### Invalid Param

 The request contained malformed or semantically incorrect parameters.

#### Is the request safe to retry?

A 60300 error request is never processed and is always safe to retry.

HTTP status: 400

#### Possible Causes
* Missing parameter.
* Invalid parameter type, value, or length.

#### Possible Solutions
* Send the required parameter.
* Review the parameter type or value.
* Review the parameter length."#,
            TwilioVerifyError::ErrorCode60406 => r#"## ERROR - 60406

### Initialization Error

 An error occurred while initializing a class.

#### Possible Causes
Missing arguments to initialize a class.

#### Possible Solutions
Check parameters and retry."#,
            TwilioVerifyError::ErrorCode60392 => r#"## ERROR - 60392

### Entity not found

 

#### Possible Causes
The requested Entity is missing

#### Possible Solutions
Check that the Entity in the request parameter/body is correct."#,
            TwilioVerifyError::ErrorCode60329 => r#"## ERROR - 60329

### Verify SNA does not work with `psd2_enabled`

 The Silent Network Auth channel does not work when `psd2_nabled` is set to `true` for a Verify `Service`.

#### Possible Causes
- Attempting to create a `Verification` with `channel` set to `sna` under a Verify `Service` with `psd2_enabled` set to `true`.

#### Possible Solutions
- Do not use `sna` channel for a Verify `Service` with `psd2_enabled` set to `true`."#,
            TwilioVerifyError::ErrorCode60420 => r#"## ERROR - 60420

### Invalid Contact ID format

 

#### Possible Causes
The Contact ID in the request is incorrectly formatted.

#### Possible Solutions
Check that the Contact ID in the request body is correct."#,
            TwilioVerifyError::ErrorCode60370 => r#"## ERROR - 60370

### Factor config invalid or not provided

 

#### Possible Causes
The Factor config section in the request is missing or incorrectly formatted.

#### Possible Solutions
Check that the Factor config in the request body is correct."#,
            TwilioVerifyError::ErrorCode60533 => r#"## ERROR - 60533

### SNA Carrier Header Error

 

#### Possible Causes
The Verification request was received but failed during processing due to a missing header from the carrier.

#### Possible Solutions
Retry Verification using a channel other than Silent Network Auth for this end user."#,
            TwilioVerifyError::ErrorCode60238 => r#"## ERROR - 60238

### Verification Creation Attempt blocked by Twilio

 Verification creation attempt is blocked by Twilio

#### Possible Causes
Your request has been blocked because your upgrade request is going through a review process

#### Possible Solutions
If you are still getting this error after 72 hours, please reach out to our support team for assistance."#,
            TwilioVerifyError::ErrorCode60403 => r#"## ERROR - 60403

### Storage Error

 An error occurred while storing/loading an entity.

#### Possible Causes
* Storage may be corrupted.
* Factor not found.

#### Possible Solutions
Create a new factor."#,
            TwilioVerifyError::ErrorCode60387 => r#"## ERROR - 60387

### Attested credential data invalid or not provided

 

#### Possible Causes
The Attested credential data in the request is missing or incorrectly formatted.

#### Possible Solutions
Check that the Attested credential data in the request body is correct."#,
            TwilioVerifyError::ErrorCode60531 => r#"## ERROR - 60531

### SNA Carrier Not Detected

 

#### Possible Causes
The Verification request was received and failed due to being unable to identify the carrier from the detected IP address. This can happen if the end user is connected to WiFi during the Verification process.

#### Possible Solutions
- Check that the mobile client is using a mobile data connection to invoke the SNA URL, not WiFi.
- Retry Verification using a channel other than Silent Network Auth for this end user."#,
            TwilioVerifyError::ErrorCode60239 => r#"## WARNING - 60239

### BYOT feature is not enabled

 Your account doesn't have access to use Bring your Own templates (BYOT). The verification was sent using default templates.

#### Possible Causes
The BYOT feature is not enabled on your account.

#### Possible Solutions
If you need access to the BYOT feature please contact Twilio Support."#,
            TwilioVerifyError::ErrorCode60402 => r#"## ERROR - 60402

### Mapper Error

 An error occurred while mapping an entity.

#### Possible Causes
Unexpected entity format.

#### Possible Solutions
Create a new factor."#,
            TwilioVerifyError::ErrorCode68007 => r#"## ERROR - 68007

### Authentication Token Error

 An error occurred while generating a token.

#### Possible Causes
Unsupported factor type.

#### Possible Solutions
Check parameters and retry."#,
            TwilioVerifyError::ErrorCode60364 => r#"## ERROR - 60364

### Challenge SID invalid or not provided

 

#### Possible Causes
The Challenge SID in the request is missing or incorrectly formatted.

#### Possible Solutions
Check that the Challenge SID in the request parameter/body is correct."#,
            TwilioVerifyError::ErrorCode60244 => r#"## WARNING - 60244

### Custom messages are not supported

 Your verification messages are upgraded to default templates.

#### Possible Causes
Verifications created using custom messages are no longer supported and upgraded to default templates.

#### Possible Solutions
If you would like to continue using custom message text migrate to [BYOT templates](https://www.twilio.com/docs/verify/api/templates)."#,
            TwilioVerifyError::ErrorCode60225 => r#"## ERROR - 60225

### Translation already exists for the provided template

 Translation already exists for the provided template.

HTTP status: 409

#### Possible Causes
The user has provided an existent translation for the template.

#### Possible Solutions
Try to create another translation with a different locale."#,
            TwilioVerifyError::ErrorCode60540 => r#"## ERROR - 60540

### SNA Carrier Identified Invalid Phone Number

 

#### Possible Causes
The Verification request was received and failed during processing because the carrier indicated that the phone number is invalid, although the phone number formatting was valid. This can happen when the end user provides an invalid phone number in error, the phone number is deactivated, suspended, in the grace period before reassignment, or a fraudulent user purposefully uses an invalid phone number.

#### Possible Solutions
- Check that the phone number provided is correct. 
- Retry Verification using a channel not based on phone number such as email. 

If fraud is suspected:
- Do not retry Verification using a channel other than Silent Network Auth for this end user.
- [Report potential fraud attempt](https://www.twilio.com/docs/verify/preventing-toll-fraud#what-to-do-if-you-suspect-fraud-on-your-twilio-account)."#,
            TwilioVerifyError::ErrorCode60423 => r#"## ERROR - 60423

### Multiple contacts were found

 

#### Possible Causes
Several contacts were found based on the request criteria.

#### Possible Solutions
Try sending a request with more accurate contact information."#,
            TwilioVerifyError::ErrorCode60226 => r#"## ERROR - 60226

### Messages sent to china require friendly_name

 Messages sent to china require friendly_name

#### Possible Causes
You are trying to send a verification to a chinese phone number but didn't specify the friendly name

#### Possible Solutions
Send the friendly name if the verification is done to a chinese phone number"#,
            TwilioVerifyError::ErrorCode60222 => r#"## ERROR - 60222

### SendGrid The from address does not match a verified Sender Identity

The from address does not match a verified Sender Identity. Mail cannot be sent until this error is resolved. Now SendGrid needs that the account has been verified before it can send messages.

HTTP status: 403

#### Possible Causes
The SendGrid account from address the message has not been verified.

#### Possible Solutions
Visit https://sendgrid.com/docs/for-developers/sending-email/sender-identity/ to see the Sender Identity requirements"#,
            TwilioVerifyError::ErrorCode60217 => r#"## ERROR - 60217

### Invalid Service configuration

 Add an email integration before using the Email channel.

HTTP status: 400

#### Possible Causes
No email integration has been set up with the Verify Service.

#### Possible Solutions
Create a SendGrid email integration for the Service before using the email channel. [Detailed instructions available here.](https://www.twilio.com/docs/verify/email)"#,
            TwilioVerifyError::ErrorCode68005 => r#"## ERROR - 68005

### Key Storage Error

 An error occurred while storing/loading keypairs.

#### Possible Causes
* Keypair not set.
* Keypair could not be created.
* Signature failed.

#### Possible Solutions
Create a new factor."#,
            TwilioVerifyError::ErrorCode60432 => r#"## ERROR - 60432

### Unsupported passkeys relying party

 

#### Possible Causes
The requested relying party type is not currently supported. 

#### Possible Solutions
Try using a different relying party type"#,
            TwilioVerifyError::ErrorCode60334 => r#"## WARNING - 60334

### The SMS message was sent using a brandful template.

 The SMS message was sent using a brandful template instead of the configured template, which does not contain a friendly name (brand).

#### Possible Causes
Due to mobile carrier regulations, Verify needs to add the friendly name from the message body.

#### Possible Solutions
Configure or use a dedicated template that includes the friendly name."#,
            TwilioVerifyError::ErrorCode60207 => r#"## ERROR - 60207

### Max rate limits per service reached 

 Your service has reached the max number of rate limits

HTTP status: 400

#### Possible Causes
Your service has reached the max number of rate limits


#### Possible Solutions
* Delete/update rate limits that you are not using at the moment
* Create a new service and add the desired rate limit to the new service"#,
            TwilioVerifyError::ErrorCode60605 => r#"## ERROR - 60605

### Verification delivery attempt blocked

Verification delivery attempt blocked The destination phone number has been blocked by Verify Geo-Permissions

HTTP status: 403

#### Possible Causes
The destination has been disabled due to suspected fraudulent activity or for possible security reasons.

#### Possible Solutions
Investigate your application for potential abuse. If you have a legitimate need to call this number, please check the following: https://support.twilio.com/hc/en-us/articles/5054044369179-Error-60605-Verification-delivery-attempt-blocked-"#,
            TwilioVerifyError::ErrorCode60428 => r#"## ERROR - 60428

### Unsupported channel

 

#### Possible Causes
The requested channel is not currently supported.

#### Possible Solutions
Try using a different channel type"#,
            TwilioVerifyError::ErrorCode60204 => r#"## ERROR - 60204

### Service does not support this feature

Your service does not have access to this feature. Your service does not have access to this feature.

HTTP status: 403

#### Possible Causes
You attempted to use a premium feature, but this feature is not enabled in your Service. Further authorization required.

#### Possible Solutions
If you are looking to use [custom verification codes](https://www.twilio.com/docs/verify/api/customization-options#custom-verification-codes), you must first select the **Enable Custom Verification Code** option on your [Verify service](https://console.twilio.com/us1/develop/verify/services) via the Twilio Console. [Learn more](https://www.twilio.com/docs/verify/api/customization-options#custom-verification-codes).

If you are looking to use a [custom friendly name](https://www.twilio.com/docs/verify/api/customization-options#custom-company-name), contact [Twilio Sales](https://www.twilio.com/help/sales) to have this feature enabled."#,
            TwilioVerifyError::ErrorCode60231 => r#"## ERROR - 60231

### Sender id already exists for account

 Sender id already exists for account

#### Possible Causes
The created sender id already exists in the given account sid

#### Possible Solutions
Use a different sender id"#,
            TwilioVerifyError::ErrorCode60227 => r#"## ERROR - 60227

### The selected channel for template is not supported

 ## Error - 60227

### Invalid Parameter

The selected template for translation is not supported

#### Possible Causes
The user have provided a channel to be used that is not supported for a template.

#### Possible Solutions
Provide a supported channel for the template."#,
            TwilioVerifyError::ErrorCode60603 => r#"## ERROR - 60603

### SendGrid maximum credits exceeded

SendGrid maximum credits exceeded SendGrid maximum credits exceeded

HTTP status: 401

#### Possible Causes
Sent too many emails verifications

#### Possible Solutions
Upgrade your Sendgrid subscription "#,
            TwilioVerifyError::ErrorCode68004 => r#"## ERROR - 68004

### Input Error

 An error occurred while loading input.

#### Possible Causes
Some inputs are invalid.

#### Possible Solutions
Check parameters and retry."#,
            TwilioVerifyError::ErrorCode60391 => r#"## ERROR - 60391

### Challenge not found

 

#### Possible Causes
The requested Challenge is missing

#### Possible Solutions
Check that the Challenge in the request parameter/body is correct."#,
            TwilioVerifyError::ErrorCode68003 => r#"## ERROR - 68003

### Storage Error

 An error occurred while storing/loading an entity.

#### Possible Causes
* Storage may be corrupted.
* Factor not found.

#### Possible Solutions
Create a new factor."#,
            TwilioVerifyError::ErrorCode60206 => r#"## ERROR - 60206

### 'Amount' & 'Payee' params are required

 Your service has the `Psd2Enabled ` flag which requires you to send the 'Amount' & 'Payee' params when creating a verification

HTTP status: 400

#### Possible Causes
Your service has the `Psd2Enabled ` flag which requires you to send the 'Amount' & 'Payee' params when creating a verification

#### Possible Solutions
* Make sure that you send the 'Amount' & 'Payee' params
* Set the `Psd2Enabled ` flag on your Verification Service to `false`."#,
            TwilioVerifyError::ErrorCode60219 => r#"## ERROR - 60219

### SendGrid Template does not contain required placeholders

 SendGrid Template does not contain required placeholders

HTTP status: 400

#### Possible Causes
Missing placeholders {{twilio_code}} or {{twilio_message}}

#### Possible Solutions
Add the required placeholders"#,
            TwilioVerifyError::ErrorCode60233 => r#"## ERROR - 60233

### Default sender id can't be deleted

 Default sender id can't be deleted

#### Possible Causes
The sender id that was requested to be deleted is default

#### Possible Solutions
Set another sender id as default and try again"#,
            TwilioVerifyError::ErrorCode60728 => r#"## ERROR - 60728

### Account exceeded the hourly messages limit

Account exceeded the maximum amount of messages per hour 

#### Possible Causes
Account has sent too many messages.

#### Possible Solutions
* Wait for sometime and try again.
* Check your [support center](https://help.twilio.com/tickets) or email inbox/spam folder for outreach from the Twilio team on how to remove/raise your hourly limit."#,
            TwilioVerifyError::ErrorCode60409 => r#"## WARNING - 60409

### Custom message did not match any template

 The custom message did not match any of our approved templates for your account.  Usage of approved message templates are strongly recommended as they increase the deliverability and conversion rate.

#### Possible Causes
You have changed your custom message and it does not match any of our approved templates.

#### Possible Solutions
In case you have changed the custom message, please return to the old one and contact support before changing it again. In case you have not changed the custom message, please contact support to provide the correct template for you.
Note: Consider using templates instead of custom messages, if none of the available  templates work for you, please contact support and we can create it for you."#,
            TwilioVerifyError::ErrorCode60331 => r#"## ERROR - 60331

### Locale requested is not supported by Verify Text-To-Speech conversion

 ## Error - 60331

### Invalid Parameter

Locale requested is not supported by Verify Text-To-Speech conversion.

#### Possible Causes
The user have provided a Locale to be used that is not supported for Text-to-Speech.

#### Possible Solutions
Provide a supported Text-To-Speech using this documentation. https://support.twilio.com/hc/en-us/articles/223132827-What-Languages-can-the-Say-TwiML-Verb-Speak-"#,
            TwilioVerifyError::ErrorCode60405 => r#"## ERROR - 60405

### Key Storage Error

 An error occurred while storing/loading keypairs.

#### Possible Causes
* Keypair not set.
* Keypair could not be created.
* Signature failed.

#### Possible Solutions
Create a new factor."#,
            TwilioVerifyError::ErrorCode60322 => r#"## ERROR - 60322

### Challenge already responded

 The Challenge was already responded before.

HTTP status: 403

#### Possible Causes
This Challenge already have a previous response.

#### Possible Solutions
Create a new Challenge."#,
            TwilioVerifyError::ErrorCode60379 => r#"## ERROR - 60379

### Authenticator data invalid or not provided

 

#### Possible Causes
The Authenticator data in the request is missing or incorrectly formatted.

#### Possible Solutions
Check that the Authenticator data in the request body is correct."#,
            TwilioVerifyError::ErrorCode60328 => r#"## ERROR - 60328

### Entities rate limit exceeded

 Exceeded the rate limit of entities created per account per second

#### Possible Causes
* More than 40 entities were created within the last second

#### Possible Solutions
* Lower the rate at which entities are being created
* Keep in mind that this could also happen while using the create factor endpoint
* Verify that your application is not misbehaving and the load is expected. If everything is as expected then contact Customer Support to review your use case and limits"#,
            TwilioVerifyError::ErrorCode60314 => r#"## ERROR - 60314

### Factors binding format is invalid

 The format of the Factor binding is not valid.

HTTP status: 400

#### Possible Causes
The `Public Key` might be invalid.

#### Possible Solutions
Make sure that the `Public Key` and `Algorithm` are correct."#,
            TwilioVerifyError::ErrorCode60302 => r#"## ERROR - 60302

### FactorType already exists

 The Entity has already a `FactorType` of the same type.

HTTP status: 400

#### Possible Causes
Each Entity can have multiples Factor types, but each `FactorType` has to be unique for the given Entity.

#### Possible Solutions
Make sure you are assigning the new `FactorType` to the right Entity."#,
            TwilioVerifyError::ErrorCode60301 => r#"## ERROR - 60301

### Entity already exists

 HTTP status: 409

#### Possible Causes
You are trying to create an Entity with an existing `Identity` in the same Service.

#### Possible Solutions
Make sure that there is not any other Entity with the same `Identity` associated to the same Service. "#,
            TwilioVerifyError::ErrorCode60305 => r#"## ERROR - 60305

### Access Token parameters are invalid

 Access Token parameters are invalid.

HTTP status: 400

#### Possible Causes
Any of the required parameters are missing in the request params:
- **Identity:** External unique identifier for the Entity.
- **FactorType:** The factor type that the Access Token will be used to create.

#### Possible Solutions
Check Access Token parameters."#,
            TwilioVerifyError::ErrorCode60203 => r#"## ERROR - 60203

### Max send attempts reached.

 This rate limit is triggered when the verification lifecycle (sending and checking) is not completed.

HTTP status: 429

#### Possible Causes
* You have attempted to send the verification code more than 5 times and have reached the limit.
* You have not [checked the verification](https://www.twilio.com/docs/verify/api/verification-check).

#### Possible Solutions
* Complete a verification lifecycle by calling the [Verification Check endpoint](https://www.twilio.com/docs/verify/api/verification-check).
* Wait for the verification to [expire](https://www.twilio.com/docs/verify/api/rate-limits-and-timeouts) (10 minutes).
* For use with [custom codes](https://www.twilio.com/docs/verify/api/customization-options) you can manually approve or cancel the verification by calling the [Verification Update endpoint](https://www.twilio.com/docs/verify/api/verification#update-a-verification-status).
* **For testing** you can manually cancel the verification by calling the [Verification Update endpoint](https://www.twilio.com/docs/verify/api/verification#update-a-verification-status).

If you identify a delivery issue, please reach out to Twilio Support through the [Console](https://console.twilio.com) or [Help Center](https://support.twilio.com)."#,
            TwilioVerifyError::ErrorCode60422 => r#"## ERROR - 60422

### Contact not found

 

#### Possible Causes
The requested Contact is missing

#### Possible Solutions
Check that the Contact in the request body is correct."#,
            TwilioVerifyError::ErrorCode60384 => r#"## ERROR - 60384

### Invalid challenge timeout

 

#### Possible Causes
The Challenge timeout in the request is negative.

#### Possible Solutions
Check that the Challenge timeout in the request has positive value"#,
            TwilioVerifyError::ErrorCode60309 => r#"## ERROR - 60309

### Push notifications limit reached for a Challenge

 The maximum number of push notifications for the Challenge has been reached

Max number of notifications: 3

HTTP status: 429

#### Possible Causes
Too many calls to the Resend Notification endpoint for a given Challenge

#### Possible Solutions
Create a new Challenge and send notifications for it."#,
            TwilioVerifyError::ErrorCode60243 => r#"## ERROR - 60243

### Custom messages are not supported

 Your verification is using custom messages.

#### Possible Causes
Verification created using custom message are not supported.

#### Possible Solutions
Ensure that you pass the OTP code via [custom code](https://www.twilio.com/docs/verify/api/customization-options) parameter in the API request, following which it will be automatically map your incoming message to Verify default templates or migrate your custom messages  to use [Verify Custom Templates](https://www.twilio.com/docs/verify/api/templates)"#,
            TwilioVerifyError::ErrorCode60214 => r#"## ERROR - 60214

### Call channel is not supported when using PSD2

 Call channel is not supported when using PSD2

HTTP status: 400

#### Possible Causes
* Verify Service has PSD2 enabled

#### Possible Solutions
* Disable PSD2 for this Verify Service
* Use SMS channel"#,
            TwilioVerifyError::ErrorCode68001 => r#"## ERROR - 68001

### Network Error

 An error occurred while calling the API.

#### Possible Causes
It was not possible to communicate properly with the API.

#### Possible Solutions
Check parameters and retry"#,
            TwilioVerifyError::ErrorCode60241 => r#"## WARNING - 60241

### Static message required

 Your verification requires use static message

#### Possible Causes
Some countries have special requirements that can override the message to be compliant with the country thus forcing the use of static messages.

#### Possible Solutions
Please contact the support team to validate the requirements needed to be compliant with the country and avoid the override of the message."#,
            TwilioVerifyError::ErrorCode60426 => r#"## ERROR - 60426

### Verification ID invalid or not provided

 

#### Possible Causes
The Verification ID in the request is incorrectly formatted.

#### Possible Solutions
Check that the Verification ID in the request parameter or body is correct."#,
            TwilioVerifyError::ErrorCode60210 => r#"## ERROR - 60210

### Max Buckets per Rate limit reached

 Your Rate limit has reached the max number of Buckets

HTTP status: 400

#### Possible Causes
Your Rate limit has reached the max number of Buckets

#### Possible Solutions
* Delete/update Buckets that you are not using at the moment
* Create a new Rate limit and add the desired Buckets to the new Rate limit"#,
            TwilioVerifyError::ErrorCode60327 => r#"## WARNING - 60327

### The provided channel is not supported by the given template. Verify is falling back to the static message.

 The template you are using does not support the channel you specified. The static message fallback was used.

#### Possible Causes
The template provided doesn't support the specified channel.

#### Possible Solutions
If you want to add the provided channel for the given template, please contact support."#
        }
    }
}
