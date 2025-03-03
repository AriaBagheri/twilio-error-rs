// AUTO-GENERATED. DO NOT MODIFY. YOUR CHANGES WILL BE OVER-WRITTEN
use crate::products::TwilioVerifyError;
use standard_error::traits::StandardErrorMessageTrait;
use std::borrow::Cow;

impl StandardErrorMessageTrait for TwilioVerifyError {
    fn message(&self) -> Cow<'static, str> {
        match self {
            TwilioVerifyError::ErrorCode60367 => r#"Invalid entity identity"#.into(),
            TwilioVerifyError::ErrorCode60215 => r#"Max number of mailers per account reached"#.into(),
            TwilioVerifyError::ErrorCode60378 => r#"Authenticator response invalid or not provided"#.into(),
            TwilioVerifyError::ErrorCode60374 => r#"Invalid page token"#.into(),
            TwilioVerifyError::ErrorCode60373 => r#"Invalid page size"#.into(),
            TwilioVerifyError::ErrorCode60213 => r#"A Messaging Configuration already exists for the given country"#.into(),
            TwilioVerifyError::ErrorCode60424 => r#"Contact address not found"#.into(),
            TwilioVerifyError::ErrorCode60388 => r#"User display name not provided"#.into(),
            TwilioVerifyError::ErrorCode60365 => r#"Entity SID invalid or not provided"#.into(),
            TwilioVerifyError::ErrorCode60237 => r#"The given domain is on a deny list"#.into(),
            TwilioVerifyError::ErrorCode60312 => r#"Challenge creation limit reached"#.into(),
            TwilioVerifyError::ErrorCode60318 => r#"Factor is unverified"#.into(),
            TwilioVerifyError::ErrorCode60234 => r#"Sender id can not be set as not default"#.into(),
            TwilioVerifyError::ErrorCode68006 => r#"Initialization Error"#.into(),
            TwilioVerifyError::ErrorCode60235 => r#"Sender Definitions does not match with requirements"#.into(),
            TwilioVerifyError::ErrorCode60407 => r#"Authentication Token Error"#.into(),
            TwilioVerifyError::ErrorCode60306 => r#"Invalid Request"#.into(),
            TwilioVerifyError::ErrorCode60236 => r#"The domain could not be obtained from the given email"#.into(),
            TwilioVerifyError::ErrorCode60421 => r#"Unexpected result when creating contact"#.into(),
            TwilioVerifyError::ErrorCode60218 => r#"SendGrid Template is not active"#.into(),
            TwilioVerifyError::ErrorCode60310 => r#"Factor verification attempts reached"#.into(),
            TwilioVerifyError::ErrorCode60381 => r#"Signature invalid or not provided"#.into(),
            TwilioVerifyError::ErrorCode60377 => r#"Authenticator attachment invalid or not provided"#.into(),
            TwilioVerifyError::ErrorCode60382 => r#"Factor does not match the relying party of the challenge"#.into(),
            TwilioVerifyError::ErrorCode60440 => r#"Unsupported verification content type"#.into(),
            TwilioVerifyError::ErrorCode60211 => r#"Bucket with the given Interval already exists"#.into(),
            TwilioVerifyError::ErrorCode60324 => r#"Challenge verification failed"#.into(),
            TwilioVerifyError::ErrorCode60240 => r#"Templates not allowed"#.into(),
            TwilioVerifyError::ErrorCode60205 => r#"SMS is not supported by landline phone number"#.into(),
            TwilioVerifyError::ErrorCode60519 => r#"SNA Verification Result Pending"#.into(),
            TwilioVerifyError::ErrorCode60247 => r#"Message Length Exceeded"#.into(),
            TwilioVerifyError::ErrorCode60229 => r#"Template translation was not found"#.into(),
            TwilioVerifyError::ErrorCode60246 => r#"Service already exists"#.into(),
            TwilioVerifyError::ErrorCode60375 => r#"Invalid id"#.into(),
            TwilioVerifyError::ErrorCode60425 => r#"Verification SID invalid or not provided"#.into(),
            TwilioVerifyError::ErrorCode60431 => r#"Verification not found"#.into(),
            TwilioVerifyError::ErrorCode60500 => r#"SNA Phone Number Mismatch"#.into(),
            TwilioVerifyError::ErrorCode60371 => r#"Relying party invalid or not provided"#.into(),
            TwilioVerifyError::ErrorCode60401 => r#"Network Error"#.into(),
            TwilioVerifyError::ErrorCode60410 => r#"Verification delivery attempt blocked"#.into(),
            TwilioVerifyError::ErrorCode60604 => r#"SendGrid Authenticated user is not authorized to send mail"#.into(),
            TwilioVerifyError::ErrorCode60311 => r#"Factor verification failed"#.into(),
            TwilioVerifyError::ErrorCode60517 => r#"SNA User-Agent Mismatch Error"#.into(),
            TwilioVerifyError::ErrorCode60245 => r#"You have exceeded your messaging limits"#.into(),
            TwilioVerifyError::ErrorCode60369 => r#"Factor type invalid or not provided"#.into(),
            TwilioVerifyError::ErrorCode60441 => r#"Illegal status"#.into(),
            TwilioVerifyError::ErrorCode60602 => r#"App hash can only be used with SMS channel"#.into(),
            TwilioVerifyError::ErrorCode60433 => r#"Unsupported passkeys approval content type"#.into(),
            TwilioVerifyError::ErrorCode60212 => r#"Too many concurrent requests for phone number"#.into(),
            TwilioVerifyError::ErrorCode60208 => r#"Rate limit with that UniqueName already exists"#.into(),
            TwilioVerifyError::ErrorCode60385 => r#"Public key invalid or not provided"#.into(),
            TwilioVerifyError::ErrorCode60532 => r#"SNA Potential Dual SIM Detected"#.into(),
            TwilioVerifyError::ErrorCode60386 => r#"Attestation object invalid or not provided"#.into(),
            TwilioVerifyError::ErrorCode60390 => r#"Factor not found"#.into(),
            TwilioVerifyError::ErrorCode60361 => r#"Account SID is invalid or not provided"#.into(),
            TwilioVerifyError::ErrorCode60408 => r#"TemplateSid is only supported for the SMS channel"#.into(),
            TwilioVerifyError::ErrorCode60323 => r#"Challenge expired"#.into(),
            TwilioVerifyError::ErrorCode60411 => r#"Phone Number already exists"#.into(),
            TwilioVerifyError::ErrorCode60209 => r#"UniqueName format is invalid"#.into(),
            TwilioVerifyError::ErrorCode68002 => r#"Mapper Error"#.into(),
            TwilioVerifyError::ErrorCode60325 => r#"Translation for locale not found, using default"#.into(),
            TwilioVerifyError::ErrorCode60221 => r#"No target verification specified"#.into(),
            TwilioVerifyError::ErrorCode60363 => r#"Service SID invalid or not provided"#.into(),
            TwilioVerifyError::ErrorCode60383 => r#"Illegal factor status"#.into(),
            TwilioVerifyError::ErrorCode60315 => r#"Reached max limit of 20 push Factors associated per Entity"#.into(),
            TwilioVerifyError::ErrorCode60404 => r#"Input Error"#.into(),
            TwilioVerifyError::ErrorCode60437 => r#"Recipient type unsupported"#.into(),
            TwilioVerifyError::ErrorCode60534 => r#"SNA Downstream Carrier Error"#.into(),
            TwilioVerifyError::ErrorCode60434 => r#"Unsupported passkeys verification content type"#.into(),
            TwilioVerifyError::ErrorCode60223 => r#"Delivery channel disabled"#.into(),
            TwilioVerifyError::ErrorCode60368 => r#"Challenge details invalid or not provided"#.into(),
            TwilioVerifyError::ErrorCode60510 => r#"SNA Error"#.into(),
            TwilioVerifyError::ErrorCode60228 => r#"Template was not found"#.into(),
            TwilioVerifyError::ErrorCode60232 => r#"Sender id does not exist"#.into(),
            TwilioVerifyError::ErrorCode60326 => r#"Too many requests to create factors for the entity"#.into(),
            TwilioVerifyError::ErrorCode60308 => r#"Challenge verification attempts limit reached"#.into(),
            TwilioVerifyError::ErrorCode60317 => r#"Factor already exists"#.into(),
            TwilioVerifyError::ErrorCode60242 => r#"WhatsApp template not found"#.into(),
            TwilioVerifyError::ErrorCode60202 => r#"Max check attempts reached"#.into(),
            TwilioVerifyError::ErrorCode60380 => r#"Client data invalid or not provided"#.into(),
            TwilioVerifyError::ErrorCode60200 => r#"Invalid parameter"#.into(),
            TwilioVerifyError::ErrorCode60333 => r#"The SMS message was sent using a brandless template."#.into(),
            TwilioVerifyError::ErrorCode60313 => r#"Unauthorized factor creation"#.into(),
            TwilioVerifyError::ErrorCode60427 => r#"Factor ID invalid or not provided"#.into(),
            TwilioVerifyError::ErrorCode60224 => r#"Missing substitutions for selected template"#.into(),
            TwilioVerifyError::ErrorCode60366 => r#"Entity invalid or not provided"#.into(),
            TwilioVerifyError::ErrorCode60550 => r#"Auto Channel Failed: None of the available channels could be selected due to validation errors. Check your debugger messages in console."#.into(),
            TwilioVerifyError::ErrorCode60362 => r#"Factor SID invalid or not provided"#.into(),
            TwilioVerifyError::ErrorCode60220 => r#"Messages to China require use case vetting"#.into(),
            TwilioVerifyError::ErrorCode60372 => r#"Relying party id invalid or not provided"#.into(),
            TwilioVerifyError::ErrorCode60511 => r#"SNA Downstream Error"#.into(),
            TwilioVerifyError::ErrorCode60201 => r#"Selected template translation is not approved"#.into(),
            TwilioVerifyError::ErrorCode60436 => r#"Recipient invalid or not provided"#.into(),
            TwilioVerifyError::ErrorCode60307 => r#"Cannot resend push notifications to 'none' notification platform"#.into(),
            TwilioVerifyError::ErrorCode60330 => r#"Failed to invoke the webhook"#.into(),
            TwilioVerifyError::ErrorCode60376 => r#"Invalid rawId"#.into(),
            TwilioVerifyError::ErrorCode60520 => r#"SNA URL Failed"#.into(),
            TwilioVerifyError::ErrorCode60300 => r#"Invalid Param"#.into(),
            TwilioVerifyError::ErrorCode60406 => r#"Initialization Error"#.into(),
            TwilioVerifyError::ErrorCode60392 => r#"Entity not found"#.into(),
            TwilioVerifyError::ErrorCode60329 => r#"Verify SNA does not work with `psd2_enabled`"#.into(),
            TwilioVerifyError::ErrorCode60420 => r#"Invalid Contact ID format"#.into(),
            TwilioVerifyError::ErrorCode60370 => r#"Factor config invalid or not provided"#.into(),
            TwilioVerifyError::ErrorCode60533 => r#"SNA Carrier Header Error"#.into(),
            TwilioVerifyError::ErrorCode60238 => r#"Verification Creation Attempt blocked by Twilio"#.into(),
            TwilioVerifyError::ErrorCode60403 => r#"Storage Error"#.into(),
            TwilioVerifyError::ErrorCode60387 => r#"Attested credential data invalid or not provided"#.into(),
            TwilioVerifyError::ErrorCode60531 => r#"SNA Carrier Not Detected"#.into(),
            TwilioVerifyError::ErrorCode60239 => r#"BYOT feature is not enabled"#.into(),
            TwilioVerifyError::ErrorCode60402 => r#"Mapper Error"#.into(),
            TwilioVerifyError::ErrorCode68007 => r#"Authentication Token Error"#.into(),
            TwilioVerifyError::ErrorCode60364 => r#"Challenge SID invalid or not provided"#.into(),
            TwilioVerifyError::ErrorCode60244 => r#"Custom messages are not supported"#.into(),
            TwilioVerifyError::ErrorCode60225 => r#"Translation already exists for the provided template"#.into(),
            TwilioVerifyError::ErrorCode60540 => r#"SNA Carrier Identified Invalid Phone Number"#.into(),
            TwilioVerifyError::ErrorCode60423 => r#"Multiple contacts were found"#.into(),
            TwilioVerifyError::ErrorCode60226 => r#"Messages sent to china require friendly_name"#.into(),
            TwilioVerifyError::ErrorCode60222 => r#"SendGrid The from address does not match a verified Sender Identity"#.into(),
            TwilioVerifyError::ErrorCode60217 => r#"Invalid Service configuration"#.into(),
            TwilioVerifyError::ErrorCode68005 => r#"Key Storage Error"#.into(),
            TwilioVerifyError::ErrorCode60432 => r#"Unsupported passkeys relying party"#.into(),
            TwilioVerifyError::ErrorCode60334 => r#"The SMS message was sent using a brandful template."#.into(),
            TwilioVerifyError::ErrorCode60207 => r#"Max rate limits per service reached "#.into(),
            TwilioVerifyError::ErrorCode60605 => r#"Verification delivery attempt blocked"#.into(),
            TwilioVerifyError::ErrorCode60428 => r#"Unsupported channel"#.into(),
            TwilioVerifyError::ErrorCode60204 => r#"Service does not support this feature"#.into(),
            TwilioVerifyError::ErrorCode60231 => r#"Sender id already exists for account"#.into(),
            TwilioVerifyError::ErrorCode60227 => r#"The selected channel for template is not supported"#.into(),
            TwilioVerifyError::ErrorCode60603 => r#"SendGrid maximum credits exceeded"#.into(),
            TwilioVerifyError::ErrorCode68004 => r#"Input Error"#.into(),
            TwilioVerifyError::ErrorCode60391 => r#"Challenge not found"#.into(),
            TwilioVerifyError::ErrorCode68003 => r#"Storage Error"#.into(),
            TwilioVerifyError::ErrorCode60206 => r#"'Amount' & 'Payee' params are required"#.into(),
            TwilioVerifyError::ErrorCode60219 => r#"SendGrid Template does not contain required placeholders"#.into(),
            TwilioVerifyError::ErrorCode60233 => r#"Default sender id can't be deleted"#.into(),
            TwilioVerifyError::ErrorCode60728 => r#"Account exceeded the hourly messages limit"#.into(),
            TwilioVerifyError::ErrorCode60409 => r#"Custom message did not match any template"#.into(),
            TwilioVerifyError::ErrorCode60331 => r#"Locale requested is not supported by Verify Text-To-Speech conversion"#.into(),
            TwilioVerifyError::ErrorCode60405 => r#"Key Storage Error"#.into(),
            TwilioVerifyError::ErrorCode60322 => r#"Challenge already responded"#.into(),
            TwilioVerifyError::ErrorCode60379 => r#"Authenticator data invalid or not provided"#.into(),
            TwilioVerifyError::ErrorCode60328 => r#"Entities rate limit exceeded"#.into(),
            TwilioVerifyError::ErrorCode60314 => r#"Factors binding format is invalid"#.into(),
            TwilioVerifyError::ErrorCode60302 => r#"FactorType already exists"#.into(),
            TwilioVerifyError::ErrorCode60301 => r#"Entity already exists"#.into(),
            TwilioVerifyError::ErrorCode60305 => r#"Access Token parameters are invalid"#.into(),
            TwilioVerifyError::ErrorCode60203 => r#"Max send attempts reached."#.into(),
            TwilioVerifyError::ErrorCode60422 => r#"Contact not found"#.into(),
            TwilioVerifyError::ErrorCode60384 => r#"Invalid challenge timeout"#.into(),
            TwilioVerifyError::ErrorCode60309 => r#"Push notifications limit reached for a Challenge"#.into(),
            TwilioVerifyError::ErrorCode60243 => r#"Custom messages are not supported"#.into(),
            TwilioVerifyError::ErrorCode60214 => r#"Call channel is not supported when using PSD2"#.into(),
            TwilioVerifyError::ErrorCode68001 => r#"Network Error"#.into(),
            TwilioVerifyError::ErrorCode60241 => r#"Static message required"#.into(),
            TwilioVerifyError::ErrorCode60426 => r#"Verification ID invalid or not provided"#.into(),
            TwilioVerifyError::ErrorCode60210 => r#"Max Buckets per Rate limit reached"#.into(),
            TwilioVerifyError::ErrorCode60327 => r#"The provided channel is not supported by the given template. Verify is falling back to the static message."#.into()
        }
    }
}
