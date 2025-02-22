// AUTO-GENERATED. DO NOT MODIFY. YOUR CHANGES WILL BE OVER-WRITTEN
use crate::products::TwilioVerifyError;
use standard_error::traits::StandardErrorCausesTrait;

impl StandardErrorCausesTrait for TwilioVerifyError {
    fn causes(&self) -> Option<&'static str> {
        match self {
            TwilioVerifyError::ErrorCode60367 => Some(r#"The Entity identity in the request is missing or incorrectly formatted."#),
            TwilioVerifyError::ErrorCode60215 => Some(r#"Your account has reached the max number of Mailers"#),
            TwilioVerifyError::ErrorCode60378 => Some(r#"The Authenticator response in the request is missing or incorrectly formatted."#),
            TwilioVerifyError::ErrorCode60374 => Some(r#"The Page token in the request is incorrectly formatted"#),
            TwilioVerifyError::ErrorCode60373 => Some(r#"The Page size in the request is missing or incorrectly formatted."#),
            TwilioVerifyError::ErrorCode60213 => Some(r#"* You have already configured the given country."#),
            TwilioVerifyError::ErrorCode60424 => Some(r#"The contact does not contain relevant address information."#),
            TwilioVerifyError::ErrorCode60388 => Some(r#"The User display name in the request is missing."#),
            TwilioVerifyError::ErrorCode60365 => Some(r#"The Entity SID in the request is missing or incorrectly formatted."#),
            TwilioVerifyError::ErrorCode60237 => Some(r#"Some generic domains have been blocked for specific functionalities. The domains in the deny list are: Gmail, Yahoo, Hotmail, Outlook."#),
            TwilioVerifyError::ErrorCode60312 => Some(r#"Too many Challenges created within a time period."#),
            TwilioVerifyError::ErrorCode60318 => Some(r#"The Factor has not been verified."#),
            TwilioVerifyError::ErrorCode60234 => Some(r#"The sender id that's being updated is default and it's not allowed to set default as false"#),
            TwilioVerifyError::ErrorCode68006 => Some(r#"Missing arguments to initialize a class."#),
            TwilioVerifyError::ErrorCode60235 => Some(r#"The sender definitions contains invalid properties"#),
            TwilioVerifyError::ErrorCode60407 => Some(r#"Unsupported factor type."#),
            TwilioVerifyError::ErrorCode60306 => Some(r#"One or more parameters are in an invalid format:

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
- Invalid Webhooks parameters: Invalid `EventTypes`"#),
            TwilioVerifyError::ErrorCode60236 => Some(r#"Could not obtain the domain from the email provided"#),
            TwilioVerifyError::ErrorCode60421 => Some(r#"Some unexpected error was appeared during creating new Contact."#),
            TwilioVerifyError::ErrorCode60218 => Some(r#"The provided template is not active"#),
            TwilioVerifyError::ErrorCode60310 => Some(r#"Too many incorrect TOTP codes have been provided in the AuthPayload"#),
            TwilioVerifyError::ErrorCode60381 => Some(r#"The Signature in the request is missing or incorrectly formatted."#),
            TwilioVerifyError::ErrorCode60377 => Some(r#"The Authenticator attachment in the request is missing or incorrectly formatted."#),
            TwilioVerifyError::ErrorCode60382 => Some(r#"The Relying party is specified in the request, but it is incorrect."#),
            TwilioVerifyError::ErrorCode60440 => Some(r#"The requested verification content type is not currently supported. "#),
            TwilioVerifyError::ErrorCode60211 => Some(r#"You have already configured that Max number of requests for this `Interval`"#),
            TwilioVerifyError::ErrorCode60324 => Some(r#"- For a Push Challenge, the signature in AuthPayload is invalid.  
- For a TOTP Challenge, the OTP code in AuthPayload is invalid."#),
            TwilioVerifyError::ErrorCode60240 => Some(r#"Some countries have special requirements that can override the message to be compliant with the country thus preventing the use of templates."#),
            TwilioVerifyError::ErrorCode60205 => Some(r#"You have attempted to send the verification code to phone number that does not support SMS."#),
            TwilioVerifyError::ErrorCode60519 => Some(r#"The Verification request was received and is still pending completion. 

This can happen when:

- A Verification Check is performed on a Verification that is still in progress.
- A Verification was created for a phone number whose carrier is not supported for the selected Twilio Region. For example, if an SNA Verification was created for a UK phone number using the US1 Region instead of the IE1 Region. In this case, the Verification request will never complete."#),
            TwilioVerifyError::ErrorCode60247 => Some(r#"1. Template Substitutions: If you are using a template, ensure that it, including any variable substitutions, does not exceed the 500-character limit.
2. Friendly Name, Code, or Custom Substitutions: Review your message's friendly name, code, or custom substitutions. These elements might be increasing the message length and causing it to exceed the limit."#),
            TwilioVerifyError::ErrorCode60229 => Some(r#"The user has provide a translation for a templated that not exists"#),
            TwilioVerifyError::ErrorCode60246 => Some(r#"* You tried to create a service with a UniqueName that already exists."#),
            TwilioVerifyError::ErrorCode60375 => Some(r#"The ID in the request is incorrectly formatted."#),
            TwilioVerifyError::ErrorCode60425 => Some(r#"The Verification ID in request has invalid format"#),
            TwilioVerifyError::ErrorCode60431 => Some(r#"The requested Verification is missing"#),
            TwilioVerifyError::ErrorCode60500 => Some(r#"The Verification request and processing was successful but the phone number provided in the request does not match the phone number detected via the carrier. This can happen when there is dual SIM usage or the end user is trying to verify a phone number that does not belong to the SIM."#),
            TwilioVerifyError::ErrorCode60371 => Some(r#"The config for relying party in the request is missing or incorrectly formatted."#),
            TwilioVerifyError::ErrorCode60401 => Some(r#"It was not possible to communicate properly with the API."#),
            TwilioVerifyError::ErrorCode60410 => Some(r#"We [identified SMS fraud](https://www.twilio.com/docs/verify/preventing-toll-fraud/sms-fraud-guard) occurring within your Verify product."#),
            TwilioVerifyError::ErrorCode60604 => Some(r#"SendGrid API key have no send emails privileges"#),
            TwilioVerifyError::ErrorCode60311 => Some(r#"It was not possible to verify a Factor."#),
            TwilioVerifyError::ErrorCode60517 => Some(r#"The Verification request was received but failed during processing due to a User-Agent mismatch detected during redirects."#),
            TwilioVerifyError::ErrorCode60245 => Some(r#"This error is returned by SendGrid if you've sent more emails than your account can handle. This error can occur for a number of reasons, including daily or hourly limits, or limits on the number of messages that can be sent at once. SendGrid uses one credit for each email, and credits renew at the beginning of each month."#),
            TwilioVerifyError::ErrorCode60369 => Some(r#"The Factor type in the request is missing or incorrectly formatted."#),
            TwilioVerifyError::ErrorCode60441 => Some(r#"An unexpected verification status returned."#),
            TwilioVerifyError::ErrorCode60602 => Some(r#"The customer is using App Hash without SMS channel"#),
            TwilioVerifyError::ErrorCode60433 => Some(r#"The requested approval content type is not currently supported."#),
            TwilioVerifyError::ErrorCode60212 => Some(r#"Your application is initiating many verifications in a short period of time for the same phone number."#),
            TwilioVerifyError::ErrorCode60208 => Some(r#"You tried to create a rate limit with a `UniqueName` that already exists for the specified service"#),
            TwilioVerifyError::ErrorCode60385 => Some(r#"The Public key in the request is missing or incorrectly formatted."#),
            TwilioVerifyError::ErrorCode60532 => Some(r#"The Verification request was received and failed due to the request potentially coming from a dual SIM device. In this case, the carrier identified via IP address does not match the carrier identified via phone number and Silent Network Auth cannot be completed. This can also happen when multiple people share one device."#),
            TwilioVerifyError::ErrorCode60386 => Some(r#"The Attestation object in the request is missing or incorrectly formatted."#),
            TwilioVerifyError::ErrorCode60390 => Some(r#"The requested Factor is missing"#),
            TwilioVerifyError::ErrorCode60361 => Some(r#"The Account SID in the request is missing or incorrectly formatted."#),
            TwilioVerifyError::ErrorCode60408 => Some(r#"Creating a verification on CALL or EMAIL with TemplateSid parameter."#),
            TwilioVerifyError::ErrorCode60323 => Some(r#"The Challenge was not responded within the window of time."#),
            TwilioVerifyError::ErrorCode60411 => Some(r#"* Multiple requests were made to add the phone number to the SafeList.
* The phone number was already added at an earlier time."#),
            TwilioVerifyError::ErrorCode60209 => Some(r#"`UniqueName` only supports alphanumeric characters and `-`, `_` or `.` 
"#),
            TwilioVerifyError::ErrorCode68002 => Some(r#"Unexpected entity format."#),
            TwilioVerifyError::ErrorCode60325 => Some(r#"The translation might not exist."#),
            TwilioVerifyError::ErrorCode60221 => Some(r#"No `To` nor `VerificationSid` is provided in the request"#),
            TwilioVerifyError::ErrorCode60363 => Some(r#"The Service SID in the request is missing or incorrectly formatted."#),
            TwilioVerifyError::ErrorCode60383 => Some(r#"The requested operation cannot be performed on this Factor because the Factor is in an inconsistent state or has expired."#),
            TwilioVerifyError::ErrorCode60315 => Some(r#"* You already have 20 `push` Factors created for the Entity.
* You already have 100 `totp` Factors created for the Entity."#),
            TwilioVerifyError::ErrorCode60404 => Some(r#"Some inputs are invalid."#),
            TwilioVerifyError::ErrorCode60437 => Some(r#"The requested recipient type is not currently supported. "#),
            TwilioVerifyError::ErrorCode60534 => Some(r#"The Verification request was received but failed during processing due to a downstream carrier error."#),
            TwilioVerifyError::ErrorCode60434 => Some(r#"The requested verification content type is not currently supported."#),
            TwilioVerifyError::ErrorCode60223 => Some(r#"The delivery channel specified is currently disabled in the Verify Service settings"#),
            TwilioVerifyError::ErrorCode60368 => Some(r#"The Challenge details section in the request is missing or incorrectly formatted."#),
            TwilioVerifyError::ErrorCode60510 => Some(r#"The Verification request was received but failed during processing due to an error during the Silent Network Auth process."#),
            TwilioVerifyError::ErrorCode60228 => Some(r#"The user has provide a TemplateSid that was not found"#),
            TwilioVerifyError::ErrorCode60232 => Some(r#"The sender id used in the request does not exist"#),
            TwilioVerifyError::ErrorCode60326 => Some(r#"* This entity tried to create more than 100 factors per day"#),
            TwilioVerifyError::ErrorCode60308 => Some(r#"Too many incorrect TOTP codes have been provided in the AuthPayload"#),
            TwilioVerifyError::ErrorCode60317 => Some(r#"You are trying to create a Factor with the same binding parameters as an existing one."#),
            TwilioVerifyError::ErrorCode60242 => Some(r#"Selected account does not have whatsApp template created or approved"#),
            TwilioVerifyError::ErrorCode60202 => Some(r#"You have attempted to check a verification more than 5 times and have reached the limit."#),
            TwilioVerifyError::ErrorCode60380 => Some(r#"The Client data JSON in the request is missing or incorrectly formatted."#),
            TwilioVerifyError::ErrorCode60200 => Some(r#"When [starting a verification](https://www.twilio.com/docs/verify/api/verification#start-new-verification):

* `To` parameter is incorrectly formatted or missing the leading `+`.
* `Channel` parameter is not one of the [accepted options](https://www.twilio.com/docs/verify/api/verification#start-new-verification), or you might not have access to the requested channel.

When [checking a verification](https://www.twilio.com/docs/verify/api/verification-check#check-a-verification):

* `Code` parameter is not between 4-10 numbers long.

When [creating a service](https://www.twilio.com/docs/verify/api/service#create-a-verification-service):

* `FriendlyName` is too long (max 30 characters).

When [fetching a service](https://www.twilio.com/docs/verify/api/service#fetch-a-service):

* Neither `ServiceSid` nor `UniqueName` is present."#),
            TwilioVerifyError::ErrorCode60333 => Some(r#"Due to mobile carrier regulations, Verify needs to remove the friendly name from the message body. 
This is due to compliance, and our best-effort to ensure delivery of your OTP to certain restricted destinations."#),
            TwilioVerifyError::ErrorCode60313 => Some(r#"You do not have the permissions required to create a Factor."#),
            TwilioVerifyError::ErrorCode60427 => Some(r#"The Factor ID in the request is missing or incorrectly formatted."#),
            TwilioVerifyError::ErrorCode60224 => Some(r#"- The template requires extra fields for building the message that are not included in the request.
- The substitutions were not provided in the correct format (stringified JSON).
"#),
            TwilioVerifyError::ErrorCode60366 => Some(r#"The Entity in the request is missing or incorrectly formatted."#),
            TwilioVerifyError::ErrorCode60550 => Some(r#"- Specific delivery channel(s) are disabled
- Your service does not have access to an specific feature."#),
            TwilioVerifyError::ErrorCode60362 => Some(r#"The Factor SID in the request is missing or incorrectly formatted."#),
            TwilioVerifyError::ErrorCode60220 => Some(r#"Your account is not allowed to create verifications towards China"#),
            TwilioVerifyError::ErrorCode60372 => Some(r#"The Relying party ID in the request is missing or incorrectly formatted."#),
            TwilioVerifyError::ErrorCode60511 => Some(r#"The Verification request was received but failed during processing due to a downstream error during the Silent Network Auth process. This can happen when the carrier was expecting a missing header, the phone number was invalid, the SNA URL was invoked on a carrier that does not support HTTPS, or there is an invalid redirect."#),
            TwilioVerifyError::ErrorCode60201 => Some(r#"- A custom template was created but has not been approved."#),
            TwilioVerifyError::ErrorCode60436 => Some(r#"The Recipient in the request is missing or incorrectly formatted."#),
            TwilioVerifyError::ErrorCode60307 => Some(r#"You called the resend push notification endpoint for a factor with the "none" notification platform configured."#),
            TwilioVerifyError::ErrorCode60330 => Some(r#"- Unable to access the `webhook` **URL**
- The `webhook` is allowing **HTTP** requests other than **POST**
- The `webhook` is responding with a status code other than **200** (OK)"#),
            TwilioVerifyError::ErrorCode60376 => Some(r#"The RawID in the request is incorrectly formatted."#),
            TwilioVerifyError::ErrorCode60520 => Some(r#"The Verification request was received and failed due to an error related to invoking the SNA URL."#),
            TwilioVerifyError::ErrorCode60300 => Some(r#"* Missing parameter.
* Invalid parameter type, value, or length."#),
            TwilioVerifyError::ErrorCode60406 => Some(r#"Missing arguments to initialize a class."#),
            TwilioVerifyError::ErrorCode60392 => Some(r#"The requested Entity is missing"#),
            TwilioVerifyError::ErrorCode60329 => Some(r#"- Attempting to create a `Verification` with `channel` set to `sna` under a Verify `Service` with `psd2_enabled` set to `true`."#),
            TwilioVerifyError::ErrorCode60420 => Some(r#"The Contact ID in the request is incorrectly formatted."#),
            TwilioVerifyError::ErrorCode60370 => Some(r#"The Factor config section in the request is missing or incorrectly formatted."#),
            TwilioVerifyError::ErrorCode60533 => Some(r#"The Verification request was received but failed during processing due to a missing header from the carrier."#),
            TwilioVerifyError::ErrorCode60238 => Some(r#"Your request has been blocked because your upgrade request is going through a review process"#),
            TwilioVerifyError::ErrorCode60403 => Some(r#"* Storage may be corrupted.
* Factor not found."#),
            TwilioVerifyError::ErrorCode60387 => Some(r#"The Attested credential data in the request is missing or incorrectly formatted."#),
            TwilioVerifyError::ErrorCode60531 => Some(r#"The Verification request was received and failed due to being unable to identify the carrier from the detected IP address. This can happen if the end user is connected to WiFi during the Verification process."#),
            TwilioVerifyError::ErrorCode60239 => Some(r#"The BYOT feature is not enabled on your account."#),
            TwilioVerifyError::ErrorCode60402 => Some(r#"Unexpected entity format."#),
            TwilioVerifyError::ErrorCode68007 => Some(r#"Unsupported factor type."#),
            TwilioVerifyError::ErrorCode60364 => Some(r#"The Challenge SID in the request is missing or incorrectly formatted."#),
            TwilioVerifyError::ErrorCode60244 => Some(r#"Verifications created using custom messages are no longer supported and upgraded to default templates."#),
            TwilioVerifyError::ErrorCode60225 => Some(r#"The user has provided an existent translation for the template."#),
            TwilioVerifyError::ErrorCode60540 => Some(r#"The Verification request was received and failed during processing because the carrier indicated that the phone number is invalid, although the phone number formatting was valid. This can happen when the end user provides an invalid phone number in error, the phone number is deactivated, suspended, in the grace period before reassignment, or a fraudulent user purposefully uses an invalid phone number."#),
            TwilioVerifyError::ErrorCode60423 => Some(r#"Several contacts were found based on the request criteria."#),
            TwilioVerifyError::ErrorCode60226 => Some(r#"You are trying to send a verification to a chinese phone number but didn't specify the friendly name"#),
            TwilioVerifyError::ErrorCode60222 => Some(r#"The SendGrid account from address the message has not been verified."#),
            TwilioVerifyError::ErrorCode60217 => Some(r#"No email integration has been set up with the Verify Service."#),
            TwilioVerifyError::ErrorCode68005 => Some(r#"* Keypair not set.
* Keypair could not be created.
* Signature failed."#),
            TwilioVerifyError::ErrorCode60432 => Some(r#"The requested relying party type is not currently supported. "#),
            TwilioVerifyError::ErrorCode60334 => Some(r#"Due to mobile carrier regulations, Verify needs to add the friendly name from the message body."#),
            TwilioVerifyError::ErrorCode60207 => Some(r#"Your service has reached the max number of rate limits
"#),
            TwilioVerifyError::ErrorCode60605 => Some(r#"The destination has been disabled due to suspected fraudulent activity or for possible security reasons."#),
            TwilioVerifyError::ErrorCode60428 => Some(r#"The requested channel is not currently supported."#),
            TwilioVerifyError::ErrorCode60204 => Some(r#"You attempted to use a premium feature, but this feature is not enabled in your Service. Further authorization required."#),
            TwilioVerifyError::ErrorCode60231 => Some(r#"The created sender id already exists in the given account sid"#),
            TwilioVerifyError::ErrorCode60227 => Some(r#"The user have provided a channel to be used that is not supported for a template."#),
            TwilioVerifyError::ErrorCode60603 => Some(r#"Sent too many emails verifications"#),
            TwilioVerifyError::ErrorCode68004 => Some(r#"Some inputs are invalid."#),
            TwilioVerifyError::ErrorCode60391 => Some(r#"The requested Challenge is missing"#),
            TwilioVerifyError::ErrorCode68003 => Some(r#"* Storage may be corrupted.
* Factor not found."#),
            TwilioVerifyError::ErrorCode60206 => Some(r#"Your service has the `Psd2Enabled ` flag which requires you to send the 'Amount' & 'Payee' params when creating a verification"#),
            TwilioVerifyError::ErrorCode60219 => Some(r#"Missing placeholders {{twilio_code}} or {{twilio_message}}"#),
            TwilioVerifyError::ErrorCode60233 => Some(r#"The sender id that was requested to be deleted is default"#),
            TwilioVerifyError::ErrorCode60728 => Some(r#"Account has sent too many messages."#),
            TwilioVerifyError::ErrorCode60409 => Some(r#"You have changed your custom message and it does not match any of our approved templates."#),
            TwilioVerifyError::ErrorCode60331 => Some(r#"The user have provided a Locale to be used that is not supported for Text-to-Speech."#),
            TwilioVerifyError::ErrorCode60405 => Some(r#"* Keypair not set.
* Keypair could not be created.
* Signature failed."#),
            TwilioVerifyError::ErrorCode60322 => Some(r#"This Challenge already have a previous response."#),
            TwilioVerifyError::ErrorCode60379 => Some(r#"The Authenticator data in the request is missing or incorrectly formatted."#),
            TwilioVerifyError::ErrorCode60328 => Some(r#"* More than 40 entities were created within the last second"#),
            TwilioVerifyError::ErrorCode60314 => Some(r#"The `Public Key` might be invalid."#),
            TwilioVerifyError::ErrorCode60302 => Some(r#"Each Entity can have multiples Factor types, but each `FactorType` has to be unique for the given Entity."#),
            TwilioVerifyError::ErrorCode60301 => Some(r#"You are trying to create an Entity with an existing `Identity` in the same Service."#),
            TwilioVerifyError::ErrorCode60305 => Some(r#"Any of the required parameters are missing in the request params:
- **Identity:** External unique identifier for the Entity.
- **FactorType:** The factor type that the Access Token will be used to create."#),
            TwilioVerifyError::ErrorCode60203 => Some(r#"* You have attempted to send the verification code more than 5 times and have reached the limit.
* You have not [checked the verification](https://www.twilio.com/docs/verify/api/verification-check)."#),
            TwilioVerifyError::ErrorCode60422 => Some(r#"The requested Contact is missing"#),
            TwilioVerifyError::ErrorCode60384 => Some(r#"The Challenge timeout in the request is negative."#),
            TwilioVerifyError::ErrorCode60309 => Some(r#"Too many calls to the Resend Notification endpoint for a given Challenge"#),
            TwilioVerifyError::ErrorCode60243 => Some(r#"Verification created using custom message are not supported."#),
            TwilioVerifyError::ErrorCode60214 => Some(r#"* Verify Service has PSD2 enabled"#),
            TwilioVerifyError::ErrorCode68001 => Some(r#"It was not possible to communicate properly with the API."#),
            TwilioVerifyError::ErrorCode60241 => Some(r#"Some countries have special requirements that can override the message to be compliant with the country thus forcing the use of static messages."#),
            TwilioVerifyError::ErrorCode60426 => Some(r#"The Verification ID in the request is incorrectly formatted."#),
            TwilioVerifyError::ErrorCode60210 => Some(r#"Your Rate limit has reached the max number of Buckets"#),
            TwilioVerifyError::ErrorCode60327 => Some(r#"The template provided doesn't support the specified channel."#)
        }
    }
}
