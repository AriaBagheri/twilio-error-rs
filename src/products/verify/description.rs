// AUTO-GENERATED. DO NOT MODIFY. YOUR CHANGES WILL BE OVER-WRITTEN
use crate::products::TwilioVerifyError;
use standard_error::traits::StandardErrorDescriptionTrait;

impl StandardErrorDescriptionTrait for TwilioVerifyError {
    fn description(&self) -> Option<&'static str> {
        match self {
            TwilioVerifyError::ErrorCode60367 => None,
            TwilioVerifyError::ErrorCode60215 => Some(r#"Max number of mailers per account reached

HTTP status: 400"#),
            TwilioVerifyError::ErrorCode60378 => None,
            TwilioVerifyError::ErrorCode60374 => None,
            TwilioVerifyError::ErrorCode60373 => None,
            TwilioVerifyError::ErrorCode60213 => Some(r#"* You have already configured the given country.

HTTP status: 400
"#),
            TwilioVerifyError::ErrorCode60424 => None,
            TwilioVerifyError::ErrorCode60388 => Some(r#"User display name not provided"#),
            TwilioVerifyError::ErrorCode60365 => None,
            TwilioVerifyError::ErrorCode60237 => None,
            TwilioVerifyError::ErrorCode60312 => Some(r#"The maximum number of Challenge creations for a Factor has been reached.

Max. number of attempts for a Factor with `Config.CodeLength` less or equal to 4: 10 every 24 hours.

Max. number of attempts for a Factor with `Config.CodeLength` greater or equal to 5: 50 every hour, and up to 100 every 24 hours.

HTTP status: 429"#),
            TwilioVerifyError::ErrorCode60318 => Some(r#"HTTP status: 403"#),
            TwilioVerifyError::ErrorCode60234 => Some(r#"Sender id can not be set as not default"#),
            TwilioVerifyError::ErrorCode68006 => Some(r#"An error occurred while initializing a class."#),
            TwilioVerifyError::ErrorCode60235 => Some(r#"Sender definitions does not match with requirements"#),
            TwilioVerifyError::ErrorCode60407 => Some(r#"An error occurred while generating a token."#),
            TwilioVerifyError::ErrorCode60306 => Some(r#"The request is invalid.

HTTP status: 400"#),
            TwilioVerifyError::ErrorCode60236 => Some(r#"The domain could not be obtained from the given email"#),
            TwilioVerifyError::ErrorCode60421 => None,
            TwilioVerifyError::ErrorCode60218 => Some(r#"SendGrid Template is not active

HTTP status: 400"#),
            TwilioVerifyError::ErrorCode60310 => Some(r#"The maximum number of verification attempts for a Factor has been reached.

Max. Number of attempts for a TOTP Factor: 5.

HTTP status: 429"#),
            TwilioVerifyError::ErrorCode60381 => None,
            TwilioVerifyError::ErrorCode60377 => None,
            TwilioVerifyError::ErrorCode60382 => None,
            TwilioVerifyError::ErrorCode60440 => None,
            TwilioVerifyError::ErrorCode60211 => Some(r#"You have already configured that Max number of requests for this `Interval`

HTTP status: 400
"#),
            TwilioVerifyError::ErrorCode60324 => Some(r#"Verification attempt was unsuccessful, AuthPayload is invalid.

HTTP status: 403"#),
            TwilioVerifyError::ErrorCode60240 => Some(r#"Your account does not have enough permissions to use templates"#),
            TwilioVerifyError::ErrorCode60205 => Some(r#"You have attempted to send the verification code to phone number that does not support SMS.

HTTP status: 403"#),
            TwilioVerifyError::ErrorCode60519 => None,
            TwilioVerifyError::ErrorCode60247 => Some(r#"The message body generated for your verification is too long. The maximum number of characters allowed is 500. "#),
            TwilioVerifyError::ErrorCode60229 => Some(r#"## Error - 60229

### Invalid Parameter

The translation locale for the template was not found"#),
            TwilioVerifyError::ErrorCode60246 => Some(r#"Service already exists"#),
            TwilioVerifyError::ErrorCode60375 => None,
            TwilioVerifyError::ErrorCode60425 => None,
            TwilioVerifyError::ErrorCode60431 => None,
            TwilioVerifyError::ErrorCode60500 => None,
            TwilioVerifyError::ErrorCode60371 => None,
            TwilioVerifyError::ErrorCode60401 => Some(r#"An error occurred while calling the API."#),
            TwilioVerifyError::ErrorCode60410 => Some(r#"This is because [Fraud Guard](/docs/verify/preventing-toll-fraud/sms-fraud-guard) has identified potential fraudulent messages being sent to one or more destination. A temporary block on SMS traffic has been placed for the next 12 hours on the prefix you used. This block will be lifted at the end of this 12-hour period. If further fraudulent activity is detected after this block, this pattern will continue with further temporary 12 hour blocks until the issue is no longer observed."#),
            TwilioVerifyError::ErrorCode60604 => Some(r#"SendGrid Authenticated user is not authorized to send mail

HTTP status: 401"#),
            TwilioVerifyError::ErrorCode60311 => Some(r#"Factor verification failure.

HTTP status: 403"#),
            TwilioVerifyError::ErrorCode60517 => None,
            TwilioVerifyError::ErrorCode60245 => Some(r#"You have exceeded your messaging limits"#),
            TwilioVerifyError::ErrorCode60369 => None,
            TwilioVerifyError::ErrorCode60441 => None,
            TwilioVerifyError::ErrorCode60602 => Some(r#"App hash can only be used with SMS channel

HTTP status: 400"#),
            TwilioVerifyError::ErrorCode60433 => None,
            TwilioVerifyError::ErrorCode60212 => Some(r#"Your application is initiating many verifications in a short period of time for the same phone number.

HTTP status: 429
"#),
            TwilioVerifyError::ErrorCode60208 => Some(r#"You tried to create a rate limit with a `UniqueName` that already exists for the specified service

HTTP status: 400
"#),
            TwilioVerifyError::ErrorCode60385 => None,
            TwilioVerifyError::ErrorCode60532 => None,
            TwilioVerifyError::ErrorCode60386 => None,
            TwilioVerifyError::ErrorCode60390 => None,
            TwilioVerifyError::ErrorCode60361 => None,
            TwilioVerifyError::ErrorCode60408 => Some(r#"TemplateSid is only supported for the SMS channel"#),
            TwilioVerifyError::ErrorCode60323 => Some(r#"Challenge verification window expired.

HTTP status: 403"#),
            TwilioVerifyError::ErrorCode60411 => Some(r#"The phone number already exists in the SafeList."#),
            TwilioVerifyError::ErrorCode60209 => Some(r#"`UniqueName` only supports alphanumeric characters and `-`, `_` or `.` 

HTTP status: 400
"#),
            TwilioVerifyError::ErrorCode68002 => Some(r#"An error occurred while mapping an entity."#),
            TwilioVerifyError::ErrorCode60325 => Some(r#"The template you are using does not have the translation with the locale you specified."#),
            TwilioVerifyError::ErrorCode60221 => Some(r#"Ensure that either a `To` or `VerificationSid` is provided

HTTP status: 400"#),
            TwilioVerifyError::ErrorCode60363 => None,
            TwilioVerifyError::ErrorCode60383 => None,
            TwilioVerifyError::ErrorCode60315 => Some(r#"* Reached max limit of 20 Factors of `FactorType` = `push` per Entity.
* Reached max limit of 100 Factors of `FactorType` = `totp` per Entity.

HTTP status: 400"#),
            TwilioVerifyError::ErrorCode60404 => Some(r#"An error occurred while loading input."#),
            TwilioVerifyError::ErrorCode60437 => None,
            TwilioVerifyError::ErrorCode60534 => None,
            TwilioVerifyError::ErrorCode60434 => None,
            TwilioVerifyError::ErrorCode60223 => Some(r#"Create verification failed because delivery channel is currently disabled

HTTP status: 400"#),
            TwilioVerifyError::ErrorCode60368 => None,
            TwilioVerifyError::ErrorCode60510 => None,
            TwilioVerifyError::ErrorCode60228 => Some(r#"## Error - 60228

### Invalid Parameter

The selected template was not found"#),
            TwilioVerifyError::ErrorCode60232 => Some(r#"Sender id does not exist"#),
            TwilioVerifyError::ErrorCode60326 => Some(r#"* Too many requests to create factors for the entity

HTTP status: 429"#),
            TwilioVerifyError::ErrorCode60308 => Some(r#"The maximum number of verification attempts for a Challenge has been reached.

Max. number of attempts for TOTP: 5.

HTTP status: 429"#),
            TwilioVerifyError::ErrorCode60317 => Some(r#"You are creating a Factor that already exists.

HTTP status: 409"#),
            TwilioVerifyError::ErrorCode60242 => Some(r#"An approved WhatsApp template was not found for the given account sid and language"#),
            TwilioVerifyError::ErrorCode60202 => Some(r#"Max (5) verification check attempts reached.

HTTP status: 429"#),
            TwilioVerifyError::ErrorCode60380 => None,
            TwilioVerifyError::ErrorCode60200 => Some(r#"The response `message` contains the name of the invalid parameter. 

HTTP status: 400

Example where `To` is the invalid parameter:
```
{
    "code": 60200,
    "message": "Invalid parameter: To",
    "more_info": "https://www.twilio.com/docs/errors/60200",
    "status": 400
}
```"#),
            TwilioVerifyError::ErrorCode60333 => Some(r#"The SMS message was sent using a brandless template, instead of the template that you configured, which contains a friendly name (brand)."#),
            TwilioVerifyError::ErrorCode60313 => Some(r#"You are not authorized to create a Factor.

HTTP status: 403"#),
            TwilioVerifyError::ErrorCode60427 => None,
            TwilioVerifyError::ErrorCode60224 => Some(r#"The selected template has one or more expected substitutions that were not provided in the request."#),
            TwilioVerifyError::ErrorCode60366 => None,
            TwilioVerifyError::ErrorCode60550 => Some(r#"Due to validation errors, none of the available specific channels could be selected. Check your debugger messages in the console for more details."#),
            TwilioVerifyError::ErrorCode60362 => None,
            TwilioVerifyError::ErrorCode60220 => Some(r#"Messages to China require use case vetting, please refer to this support article https://support.twilio.com/hc/en-us/articles/17024185400859-Use-Case-Vetting-for-Verify-Messages-to-China to understand this in detail.


HTTP status: 400"#),
            TwilioVerifyError::ErrorCode60372 => None,
            TwilioVerifyError::ErrorCode60511 => None,
            TwilioVerifyError::ErrorCode60201 => Some(r#"Selected template translation is not approved."#),
            TwilioVerifyError::ErrorCode60436 => None,
            TwilioVerifyError::ErrorCode60307 => Some(r#"This error is returned when the resend push notification endpoint was called for a factor using "none" as notification platform in its configuration."#),
            TwilioVerifyError::ErrorCode60330 => Some(r#"HTTP status: 400"#),
            TwilioVerifyError::ErrorCode60376 => None,
            TwilioVerifyError::ErrorCode60520 => None,
            TwilioVerifyError::ErrorCode60300 => Some(r#"The request contained malformed or semantically incorrect parameters.

#### Is the request safe to retry?

A 60300 error request is never processed and is always safe to retry.

HTTP status: 400"#),
            TwilioVerifyError::ErrorCode60406 => Some(r#"An error occurred while initializing a class."#),
            TwilioVerifyError::ErrorCode60392 => None,
            TwilioVerifyError::ErrorCode60329 => Some(r#"The Silent Network Auth channel does not work when `psd2_nabled` is set to `true` for a Verify `Service`."#),
            TwilioVerifyError::ErrorCode60420 => None,
            TwilioVerifyError::ErrorCode60370 => None,
            TwilioVerifyError::ErrorCode60533 => None,
            TwilioVerifyError::ErrorCode60238 => Some(r#"Verification creation attempt is blocked by Twilio"#),
            TwilioVerifyError::ErrorCode60403 => Some(r#"An error occurred while storing/loading an entity."#),
            TwilioVerifyError::ErrorCode60387 => None,
            TwilioVerifyError::ErrorCode60531 => None,
            TwilioVerifyError::ErrorCode60239 => Some(r#"Your account doesn't have access to use Bring your Own templates (BYOT). The verification was sent using default templates."#),
            TwilioVerifyError::ErrorCode60402 => Some(r#"An error occurred while mapping an entity."#),
            TwilioVerifyError::ErrorCode68007 => Some(r#"An error occurred while generating a token."#),
            TwilioVerifyError::ErrorCode60364 => None,
            TwilioVerifyError::ErrorCode60244 => Some(r#"Your verification messages are upgraded to default templates."#),
            TwilioVerifyError::ErrorCode60225 => Some(r#"Translation already exists for the provided template.

HTTP status: 409"#),
            TwilioVerifyError::ErrorCode60540 => None,
            TwilioVerifyError::ErrorCode60423 => None,
            TwilioVerifyError::ErrorCode60226 => Some(r#"Messages sent to china require friendly_name"#),
            TwilioVerifyError::ErrorCode60222 => Some(r#"Now SendGrid needs that the account has been verified before it can send messages.

HTTP status: 403"#),
            TwilioVerifyError::ErrorCode60217 => Some(r#"Add an email integration before using the Email channel.

HTTP status: 400"#),
            TwilioVerifyError::ErrorCode68005 => Some(r#"An error occurred while storing/loading keypairs."#),
            TwilioVerifyError::ErrorCode60432 => None,
            TwilioVerifyError::ErrorCode60334 => Some(r#"The SMS message was sent using a brandful template instead of the configured template, which does not contain a friendly name (brand)."#),
            TwilioVerifyError::ErrorCode60207 => Some(r#"Your service has reached the max number of rate limits

HTTP status: 400"#),
            TwilioVerifyError::ErrorCode60605 => Some(r#"The destination phone number has been blocked by Verify Geo-Permissions

HTTP status: 403"#),
            TwilioVerifyError::ErrorCode60428 => None,
            TwilioVerifyError::ErrorCode60204 => Some(r#"Your service does not have access to this feature.

HTTP status: 403"#),
            TwilioVerifyError::ErrorCode60231 => Some(r#"Sender id already exists for account"#),
            TwilioVerifyError::ErrorCode60227 => Some(r#"## Error - 60227

### Invalid Parameter

The selected template for translation is not supported"#),
            TwilioVerifyError::ErrorCode60603 => Some(r#"SendGrid maximum credits exceeded

HTTP status: 401"#),
            TwilioVerifyError::ErrorCode68004 => Some(r#"An error occurred while loading input."#),
            TwilioVerifyError::ErrorCode60391 => None,
            TwilioVerifyError::ErrorCode68003 => Some(r#"An error occurred while storing/loading an entity."#),
            TwilioVerifyError::ErrorCode60206 => Some(r#"Your service has the `Psd2Enabled ` flag which requires you to send the 'Amount' & 'Payee' params when creating a verification

HTTP status: 400"#),
            TwilioVerifyError::ErrorCode60219 => Some(r#"SendGrid Template does not contain required placeholders

HTTP status: 400"#),
            TwilioVerifyError::ErrorCode60233 => Some(r#"Default sender id can't be deleted"#),
            TwilioVerifyError::ErrorCode60728 => None,
            TwilioVerifyError::ErrorCode60409 => Some(r#"The custom message did not match any of our approved templates for your account.  Usage of approved message templates are strongly recommended as they increase the deliverability and conversion rate."#),
            TwilioVerifyError::ErrorCode60331 => Some(r#"## Error - 60331

### Invalid Parameter

Locale requested is not supported by Verify Text-To-Speech conversion."#),
            TwilioVerifyError::ErrorCode60405 => Some(r#"An error occurred while storing/loading keypairs."#),
            TwilioVerifyError::ErrorCode60322 => Some(r#"The Challenge was already responded before.

HTTP status: 403"#),
            TwilioVerifyError::ErrorCode60379 => None,
            TwilioVerifyError::ErrorCode60328 => Some(r#"Exceeded the rate limit of entities created per account per second"#),
            TwilioVerifyError::ErrorCode60314 => Some(r#"The format of the Factor binding is not valid.

HTTP status: 400"#),
            TwilioVerifyError::ErrorCode60302 => Some(r#"The Entity has already a `FactorType` of the same type.

HTTP status: 400"#),
            TwilioVerifyError::ErrorCode60301 => Some(r#"HTTP status: 409"#),
            TwilioVerifyError::ErrorCode60305 => Some(r#"Access Token parameters are invalid.

HTTP status: 400"#),
            TwilioVerifyError::ErrorCode60203 => Some(r#"This rate limit is triggered when the verification lifecycle (sending and checking) is not completed.

HTTP status: 429"#),
            TwilioVerifyError::ErrorCode60422 => None,
            TwilioVerifyError::ErrorCode60384 => None,
            TwilioVerifyError::ErrorCode60309 => Some(r#"The maximum number of push notifications for the Challenge has been reached

Max number of notifications: 3

HTTP status: 429"#),
            TwilioVerifyError::ErrorCode60243 => Some(r#"Your verification is using custom messages."#),
            TwilioVerifyError::ErrorCode60214 => Some(r#"Call channel is not supported when using PSD2

HTTP status: 400"#),
            TwilioVerifyError::ErrorCode68001 => Some(r#"An error occurred while calling the API."#),
            TwilioVerifyError::ErrorCode60241 => Some(r#"Your verification requires use static message"#),
            TwilioVerifyError::ErrorCode60426 => None,
            TwilioVerifyError::ErrorCode60210 => Some(r#"Your Rate limit has reached the max number of Buckets

HTTP status: 400"#),
            TwilioVerifyError::ErrorCode60327 => Some(r#"The template you are using does not support the channel you specified. The static message fallback was used."#)
        }
    }
}
