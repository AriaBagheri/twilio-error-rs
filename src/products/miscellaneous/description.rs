// AUTO-GENERATED. DO NOT MODIFY. YOUR CHANGES WILL BE OVER-WRITTEN
use crate::products::TwilioMiscellaneousError;
use standard_error::traits::StandardErrorDescriptionTrait;

impl StandardErrorDescriptionTrait for TwilioMiscellaneousError {
    fn description(&self) -> Option<&'static str> {
        match self {
            TwilioMiscellaneousError::ErrorCode13000 => None,
            TwilioMiscellaneousError::ErrorCode21616 => None,
            TwilioMiscellaneousError::ErrorCode13612 => None,
            TwilioMiscellaneousError::ErrorCode19023 => Some(r#"Request with an invalid channel type"#),
            TwilioMiscellaneousError::ErrorCode51109 => None,
            TwilioMiscellaneousError::ErrorCode21227 => None,
            TwilioMiscellaneousError::ErrorCode21504 => None,
            TwilioMiscellaneousError::ErrorCode21406 => None,
            TwilioMiscellaneousError::ErrorCode51106 => None,
            TwilioMiscellaneousError::ErrorCode13241 => None,
            TwilioMiscellaneousError::ErrorCode21602 => Some(r#"The `Body` OR `MediaURL` parameter is required to send a Message with Twilio."#),
            TwilioMiscellaneousError::ErrorCode84004 => Some(r#"Required values to create execution might not have been provided."#),
            TwilioMiscellaneousError::ErrorCode21477 => None,
            TwilioMiscellaneousError::ErrorCode70156 => None,
            TwilioMiscellaneousError::ErrorCode94601 => Some(r#"The action could not be performed because the date range provided is not valid."#),
            TwilioMiscellaneousError::ErrorCode19041 => Some(r#"Request body is not valid"#),
            TwilioMiscellaneousError::ErrorCode21235 => None,
            TwilioMiscellaneousError::ErrorCode21222 => None,
            TwilioMiscellaneousError::ErrorCode91203 => Some(r#"This operation is not allowed against the resource"#),
            TwilioMiscellaneousError::ErrorCode21237 => None,
            TwilioMiscellaneousError::ErrorCode21229 => None,
            TwilioMiscellaneousError::ErrorCode13234 => None,
            TwilioMiscellaneousError::ErrorCode15002 => None,
            TwilioMiscellaneousError::ErrorCode51108 => None,
            TwilioMiscellaneousError::ErrorCode21264 => Some(r#"You received an inbound phone call to a Twilio phone number owned by your trial account, but the 'From' number is not a [Verified Caller ID](https://support.twilio.com/hc/en-us/articles/223180048-How-to-Add-and-Remove-a-Verified-Phone-Number-or-Caller-ID-with-Twilio) on your account. In order to receive calls to a Twilio number during a free trial, you must first verify your ownership of the phone number that is placing the call."#),
            TwilioMiscellaneousError::ErrorCode94403 => Some(r#"Public key credentials to encrypt transcription results are invalid. Twilio will attempt operation again shortly. "#),
            TwilioMiscellaneousError::ErrorCode20011 => None,
            TwilioMiscellaneousError::ErrorCode51113 => None,
            TwilioMiscellaneousError::ErrorCode21402 => None,
            TwilioMiscellaneousError::ErrorCode13616 => None,
            TwilioMiscellaneousError::ErrorCode70252 => None,
            TwilioMiscellaneousError::ErrorCode14220 => None,
            TwilioMiscellaneousError::ErrorCode70105 => None,
            TwilioMiscellaneousError::ErrorCode13618 => None,
            TwilioMiscellaneousError::ErrorCode94400 => Some(r#"Twilio failed to transcribe audio due to an internal issue. The transcription files have been deleted and the resource has been marked as “failed”"#),
            TwilioMiscellaneousError::ErrorCode21475 => None,
            TwilioMiscellaneousError::ErrorCode13310 => None,
            TwilioMiscellaneousError::ErrorCode19035 => Some(r#"The page size provided in the request is not valid"#),
            TwilioMiscellaneousError::ErrorCode21702 => None,
            TwilioMiscellaneousError::ErrorCode19026 => Some(r#"Maximum number of channels of the same type  allowed reached"#),
            TwilioMiscellaneousError::ErrorCode21456 => None,
            TwilioMiscellaneousError::ErrorCode32017 => Some(r#"Intermediary / Carrier Analytics blocked call due to poor reputation score on the telephone number"#),
            TwilioMiscellaneousError::ErrorCode51107 => None,
            TwilioMiscellaneousError::ErrorCode13111 => None,
            TwilioMiscellaneousError::ErrorCode21233 => None,
            TwilioMiscellaneousError::ErrorCode19033 => Some(r#"The request body has invalid inputs"#),
            TwilioMiscellaneousError::ErrorCode94402 => Some(r#"Twilio failed to encrypt and securely store transcription results. The transcription files have been deleted and the resource has been marked as “failed”"#),
            TwilioMiscellaneousError::ErrorCode80403 => None,
            TwilioMiscellaneousError::ErrorCode19028 => Some(r#"Channel value can not be updated"#),
            TwilioMiscellaneousError::ErrorCode21623 => None,
            TwilioMiscellaneousError::ErrorCode19012 => Some(r#"No contact updates provided in the request"#),
            TwilioMiscellaneousError::ErrorCode60002 => Some(r#"End User Identification Timeout"#),
            TwilioMiscellaneousError::ErrorCode61001 => None,
            TwilioMiscellaneousError::ErrorCode21242 => None,
            TwilioMiscellaneousError::ErrorCode32102 => None,
            TwilioMiscellaneousError::ErrorCode90015 => Some(r#"Body and Template (Body, Sid, Language, Args) are provided together."#),
            TwilioMiscellaneousError::ErrorCode60004 => None,
            TwilioMiscellaneousError::ErrorCode51110 => None,
            TwilioMiscellaneousError::ErrorCode22223 => Some(r#"The Regulatory Bundle requested is not eligible to be Copied."#),
            TwilioMiscellaneousError::ErrorCode13420 => None,
            TwilioMiscellaneousError::ErrorCode30112 => None,
            TwilioMiscellaneousError::ErrorCode51114 => None,
            TwilioMiscellaneousError::ErrorCode21407 => None,
            TwilioMiscellaneousError::ErrorCode21457 => None,
            TwilioMiscellaneousError::ErrorCode51004 => None,
            TwilioMiscellaneousError::ErrorCode94303 => Some(r#"A language is required to create a new configuration"#),
            TwilioMiscellaneousError::ErrorCode51118 => None,
            TwilioMiscellaneousError::ErrorCode21607 => None,
            TwilioMiscellaneousError::ErrorCode21204 => None,
            TwilioMiscellaneousError::ErrorCode19029 => Some(r#"No channel updates provided in the request"#),
            TwilioMiscellaneousError::ErrorCode13230 => None,
            TwilioMiscellaneousError::ErrorCode13222 => None,
            TwilioMiscellaneousError::ErrorCode32112 => Some(r#"The header you asked Twilio to stamp was omitted because it contains illegal characters in its _value_. Including this header would have resulting in Twilio generating a message that runs contrary to the SIP specification."#),
            TwilioMiscellaneousError::ErrorCode20004 => None,
            TwilioMiscellaneousError::ErrorCode13617 => None,
            TwilioMiscellaneousError::ErrorCode10003 => None,
            TwilioMiscellaneousError::ErrorCode19014 => Some(r#"Contact fetch request should contain unique_customer_provided_id or channel, not both"#),
            TwilioMiscellaneousError::ErrorCode21403 => None,
            TwilioMiscellaneousError::ErrorCode21473 => None,
            TwilioMiscellaneousError::ErrorCode90030 => Some(r#"Broadcast 'CorrelationId' is too long"#),
            TwilioMiscellaneousError::ErrorCode19022 => Some(r#"Request with a channel that is not supported"#),
            TwilioMiscellaneousError::ErrorCode14213 => None,
            TwilioMiscellaneousError::ErrorCode21478 => None,
            TwilioMiscellaneousError::ErrorCode30016 => None,
            TwilioMiscellaneousError::ErrorCode20422 => None,
            TwilioMiscellaneousError::ErrorCode60005 => None,
            TwilioMiscellaneousError::ErrorCode21218 => None,
            TwilioMiscellaneousError::ErrorCode20021 => Some(r#"Exception received while creating application address in SDG Service Provisioning API: Invalid short code And enum server status is: FAILURE"#),
            TwilioMiscellaneousError::ErrorCode90027 => Some(r#"Broadcast 'FriendlyName' is too long"#),
            TwilioMiscellaneousError::ErrorCode80606 => None,
            TwilioMiscellaneousError::ErrorCode14204 => None,
            TwilioMiscellaneousError::ErrorCode94304 => Some(r#"Invalid language value, either to create a new configuration or update existing one. Language should be a BCP-47 code"#),
            TwilioMiscellaneousError::ErrorCode13252 => None,
            TwilioMiscellaneousError::ErrorCode21207 => None,
            TwilioMiscellaneousError::ErrorCode21503 => None,
            TwilioMiscellaneousError::ErrorCode70004 => None,
            TwilioMiscellaneousError::ErrorCode13212 => None,
            TwilioMiscellaneousError::ErrorCode21472 => None,
            TwilioMiscellaneousError::ErrorCode21203 => None,
            TwilioMiscellaneousError::ErrorCode51105 => None,
            TwilioMiscellaneousError::ErrorCode21701 => Some(r#"The Messaging Service resource you are referencing does not exist or does not match the `Account SID` in the API Request.
"#),
            TwilioMiscellaneousError::ErrorCode21239 => None,
            TwilioMiscellaneousError::ErrorCode51201 => None,
            TwilioMiscellaneousError::ErrorCode52116 => Some(r#"Google will deprecate the legacy FCM API in June 2024: https://help.twilio.com/articles/20768292997147-Updating-Twilio-Push-for-FCM-HTTP-v1-API"#),
            TwilioMiscellaneousError::ErrorCode20423 => None,
            TwilioMiscellaneousError::ErrorCode14221 => None,
            TwilioMiscellaneousError::ErrorCode11243 => None,
            TwilioMiscellaneousError::ErrorCode13611 => None,
            TwilioMiscellaneousError::ErrorCode20012 => Some(r#"## Error Code 20012

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
* TLS_RSA_WITH_3DES_EDE_CBC_SHA (rsa 2048) - C"#),
            TwilioMiscellaneousError::ErrorCode90016 => Some(r#"'Template' or 'TemplateSid'/'TemplateLanguage' is required to send a Template Message"#),
            TwilioMiscellaneousError::ErrorCode21501 => None,
            TwilioMiscellaneousError::ErrorCode19010 => None,
            TwilioMiscellaneousError::ErrorCode32209 => None,
            TwilioMiscellaneousError::ErrorCode12301 => None,
            TwilioMiscellaneousError::ErrorCode80602 => None,
            TwilioMiscellaneousError::ErrorCode20500 => None,
            TwilioMiscellaneousError::ErrorCode90013 => Some(r#"'Template' or 'TemplateSid'&'TemplateLanguage' or 'MediaUrls' is required to send Broadcast"#),
            TwilioMiscellaneousError::ErrorCode21620 => Some(r#"One or more media URLs that you provided is invalid — they are malformed in some way. 

Typically, this error is issued when a URL lacks a protocol specifier prefix, i.e., `htttp://` or `https://`."#),
            TwilioMiscellaneousError::ErrorCode51003 => None,
            TwilioMiscellaneousError::ErrorCode80903 => None,
            TwilioMiscellaneousError::ErrorCode21608 => None,
            TwilioMiscellaneousError::ErrorCode94500 => Some(r#"The action could not be performed because the sourceSid provided is not valid"#),
            TwilioMiscellaneousError::ErrorCode21220 => None,
            TwilioMiscellaneousError::ErrorCode503 => Some(r#" An internal error has occurred while processing the request.
"#),
            TwilioMiscellaneousError::ErrorCode70052 => None,
            TwilioMiscellaneousError::ErrorCode13233 => None,
            TwilioMiscellaneousError::ErrorCode21613 => None,
            TwilioMiscellaneousError::ErrorCode21502 => None,
            TwilioMiscellaneousError::ErrorCode21231 => None,
            TwilioMiscellaneousError::ErrorCode51130 => None,
            TwilioMiscellaneousError::ErrorCode51116 => None,
            TwilioMiscellaneousError::ErrorCode51006 => None,
            TwilioMiscellaneousError::ErrorCode21101 => None,
            TwilioMiscellaneousError::ErrorCode70104 => None,
            TwilioMiscellaneousError::ErrorCode13313 => None,
            TwilioMiscellaneousError::ErrorCode20007 => None,
            TwilioMiscellaneousError::ErrorCode90017 => Some(r#"'Template' and 'TemplateSid'/'TemplateLanguage' must not be specified together for template message"#),
            TwilioMiscellaneousError::ErrorCode11202 => None,
            TwilioMiscellaneousError::ErrorCode14101 => None,
            TwilioMiscellaneousError::ErrorCode30012 => None,
            TwilioMiscellaneousError::ErrorCode94200 => Some(r#"Invalid "encryptionEnabled" value. It should be either true or false"#),
            TwilioMiscellaneousError::ErrorCode21201 => None,
            TwilioMiscellaneousError::ErrorCode13510 => None,
            TwilioMiscellaneousError::ErrorCode90010 => None,
            TwilioMiscellaneousError::ErrorCode60003 => None,
            TwilioMiscellaneousError::ErrorCode13614 => None,
            TwilioMiscellaneousError::ErrorCode70155 => None,
            TwilioMiscellaneousError::ErrorCode90037 => Some(r#"Broadcast has too many 'CorrelationId' items"#),
            TwilioMiscellaneousError::ErrorCode21226 => None,
            TwilioMiscellaneousError::ErrorCode21255 => None,
            TwilioMiscellaneousError::ErrorCode19011 => Some(r#"Invalid JSON or fields other than first_name, middle_name, last_name, legal_name, preferred_name, unique_customer_provided_id or custom_fields exist in update contact request"#),
            TwilioMiscellaneousError::ErrorCode19031 => Some(r#"Maximum number of locations allowed reached"#),
            TwilioMiscellaneousError::ErrorCode94401 => Some(r#"Twilio failed to transcribe audio due to an issue with the transcription provider.  The transcription files have been deleted and the resource has been marked as “failed”"#),
            TwilioMiscellaneousError::ErrorCode70001 => None,
            TwilioMiscellaneousError::ErrorCode90041 => Some(r#"Broadcast 'MediaUrl' field is too long"#),
            TwilioMiscellaneousError::ErrorCode80906 => None,
            TwilioMiscellaneousError::ErrorCode51125 => None,
            TwilioMiscellaneousError::ErrorCode13226 => None,
            TwilioMiscellaneousError::ErrorCode70251 => None,
            TwilioMiscellaneousError::ErrorCode21409 => None,
            TwilioMiscellaneousError::ErrorCode21422 => None,
            TwilioMiscellaneousError::ErrorCode51126 => None,
            TwilioMiscellaneousError::ErrorCode32006 => None,
            TwilioMiscellaneousError::ErrorCode19027 => Some(r#"The channel description provided in the request is invalid"#),
            TwilioMiscellaneousError::ErrorCode13615 => None,
            TwilioMiscellaneousError::ErrorCode11242 => None,
            TwilioMiscellaneousError::ErrorCode19021 => Some(r#"is_primary can be set to true only for one channel"#),
            TwilioMiscellaneousError::ErrorCode21211 => Some(r#"You attempted to initiate an outbound phone call or send a message, but the `To` phone number you supplied was not a valid phone number or was incorrectly formatted. Twilio accepts phone numbers in [E164 format](http://en.wikipedia.org/wiki/E.164): `[+] [country code] [subscriber number including area code]`."#),
            TwilioMiscellaneousError::ErrorCode21705 => Some(r#"



"#),
            TwilioMiscellaneousError::ErrorCode52004 => Some(r#"Credential SIDs are used to determine which Credential resource to use to send a given push notification (APNS, FCM or GCM). For each delivery attempt the system tries to select a Credential SID depending on the API request and configuration. This error indicates that the system did not find any available Credential SID to use and hence failed to send the delivery attempt."#),
            TwilioMiscellaneousError::ErrorCode13610 => None,
            TwilioMiscellaneousError::ErrorCode21236 => None,
            TwilioMiscellaneousError::ErrorCode21604 => Some(r#"## Error - 21604

### 'To' phone number is required to send a Message

Make sure to specify a valid phone number as the recipient of the message.

"#),
            TwilioMiscellaneousError::ErrorCode19030 => Some(r#"Request with an invalid location type"#),
            TwilioMiscellaneousError::ErrorCode94002 => Some(r#"Twilio failed to process the request to transcribe audio because configuration was not found. No transcription resource has been created"#),
            TwilioMiscellaneousError::ErrorCode32216 => Some(r#"This address is on a deny list."#),
            TwilioMiscellaneousError::ErrorCode19032 => Some(r#"Request body is not valid"#),
            TwilioMiscellaneousError::ErrorCode80105 => None,
            TwilioMiscellaneousError::ErrorCode32016 => Some(r#"## PSTN Post-Dial Delay timeout"#),
            TwilioMiscellaneousError::ErrorCode13910 => None,
            TwilioMiscellaneousError::ErrorCode80407 => None,
            TwilioMiscellaneousError::ErrorCode13246 => None,
            TwilioMiscellaneousError::ErrorCode32103 => None,
            TwilioMiscellaneousError::ErrorCode70102 => None,
            TwilioMiscellaneousError::ErrorCode90032 => Some(r#"Broadcast recipient's 'to' is invalid"#),
            TwilioMiscellaneousError::ErrorCode11236 => None,
            TwilioMiscellaneousError::ErrorCode14202 => None,
            TwilioMiscellaneousError::ErrorCode11216 => Some(r#"Twilio sent a webhook to your server and your server responded with a 3xx status code that was not a valid redirect."#),
            TwilioMiscellaneousError::ErrorCode14111 => None,
            TwilioMiscellaneousError::ErrorCode70051 => Some(r#"Twilio could not authorize the given request."#),
            TwilioMiscellaneousError::ErrorCode21210 => Some(r#"You attempted to initiate an outbound phone call, but the
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
"#),
            TwilioMiscellaneousError::ErrorCode80303 => None,
            TwilioMiscellaneousError::ErrorCode80905 => None,
            TwilioMiscellaneousError::ErrorCode61004 => None,
            TwilioMiscellaneousError::ErrorCode13217 => None,
            TwilioMiscellaneousError::ErrorCode80503 => None,
            TwilioMiscellaneousError::ErrorCode11320 => None,
            TwilioMiscellaneousError::ErrorCode13242 => None,
            TwilioMiscellaneousError::ErrorCode13245 => None,
            TwilioMiscellaneousError::ErrorCode51104 => None,
            TwilioMiscellaneousError::ErrorCode51122 => None,
            TwilioMiscellaneousError::ErrorCode20020 => Some(r#"No update/state change is observed for the data entered"#),
            TwilioMiscellaneousError::ErrorCode19042 => Some(r#"The request body has invalid inputs"#),
            TwilioMiscellaneousError::ErrorCode12101 => None,
            TwilioMiscellaneousError::ErrorCode14211 => None,
            TwilioMiscellaneousError::ErrorCode21452 => None,
            TwilioMiscellaneousError::ErrorCode61000 => None,
            TwilioMiscellaneousError::ErrorCode21421 => Some(r#"You tried to purchase or lookup a phone number that is invalid or add a phone number to a Messaging Service with an invalid Phone Number SID."#),
            TwilioMiscellaneousError::ErrorCode13312 => None,
            TwilioMiscellaneousError::ErrorCode20429 => None,
            TwilioMiscellaneousError::ErrorCode50068 => None,
            TwilioMiscellaneousError::ErrorCode51202 => None,
            TwilioMiscellaneousError::ErrorCode21471 => None,
            TwilioMiscellaneousError::ErrorCode70253 => None,
            TwilioMiscellaneousError::ErrorCode90012 => Some(r#"'Recipients' list has too many items"#),
            TwilioMiscellaneousError::ErrorCode21601 => None,
            TwilioMiscellaneousError::ErrorCode13221 => None,
            TwilioMiscellaneousError::ErrorCode14212 => None,
            TwilioMiscellaneousError::ErrorCode13251 => None,
            TwilioMiscellaneousError::ErrorCode51127 => None,
            TwilioMiscellaneousError::ErrorCode13299 => Some(r#"A feature added to the 2010 Conference API was requested via the 2008 Twilio API. Please upgrade to the latest version of the conference API."#),
            TwilioMiscellaneousError::ErrorCode403 => Some(r#"The request is not authorized"#),
            TwilioMiscellaneousError::ErrorCode13231 => None,
            TwilioMiscellaneousError::ErrorCode13120 => None,
            TwilioMiscellaneousError::ErrorCode21653 => Some(r#"There are more recipient addresses than allowed for a channel. For instance for SMS we allow only one recipient but for Group MMS we allow multiple recipients"#),
            TwilioMiscellaneousError::ErrorCode12300 => Some(r#"Twilio is unable to process the Content-Type of the provided URL. Please see Twilio's <a href="https://www.twilio.com/docs/voice/twiml#twilio-understands-mime-types"> documentation on accepted content types </a> for more information on valid Content-Types.

You must return a Content-Type for all requests. Requests without a
Content-Type will appear in the [Debugger](/console/runtime/debugger) as a
502 Bad Gateway error.

"#),
            TwilioMiscellaneousError::ErrorCode51215 => None,
            TwilioMiscellaneousError::ErrorCode80604 => None,
            TwilioMiscellaneousError::ErrorCode32113 => Some(r#"The SIP header you asked Twilio to stamp was omitted because it is not supported on the Twilio platform.

Twilio supports extension headers beginning with an X- prefix. Your extension header name cannot begin with the prefix `X-Twilio`."#),
            TwilioMiscellaneousError::ErrorCode21453 => None,
            TwilioMiscellaneousError::ErrorCode19024 => Some(r#"Request body is not valid"#),
            TwilioMiscellaneousError::ErrorCode11235 => None,
            TwilioMiscellaneousError::ErrorCode90025 => Some(r#"Template body has unsupported Mustache tag type"#),
            TwilioMiscellaneousError::ErrorCode70106 => None,
            TwilioMiscellaneousError::ErrorCode10001 => Some(r#"This account has been disabled and may not be used until it is reactivated."#),
            TwilioMiscellaneousError::ErrorCode80605 => None,
            TwilioMiscellaneousError::ErrorCode13112 => None,
            TwilioMiscellaneousError::ErrorCode94100 => Some(r#"Twilio attempted to send a transcription event to the callback URL specified and your application returned a 4xx or 5xx or other HTTP response error"#),
            TwilioMiscellaneousError::ErrorCode90039 => Some(r#"Broadcast 'MessageStatusCallbackUrl' is invalid"#),
            TwilioMiscellaneousError::ErrorCode21209 => None,
            TwilioMiscellaneousError::ErrorCode11203 => None,
            TwilioMiscellaneousError::ErrorCode12100 => None,
            TwilioMiscellaneousError::ErrorCode61008 => None,
            TwilioMiscellaneousError::ErrorCode61005 => None,
            TwilioMiscellaneousError::ErrorCode14106 => None,
            TwilioMiscellaneousError::ErrorCode11217 => Some(r#"Twilio sent a webhook to your server and your server responded with a 4xx or 5xx status code."#),
            TwilioMiscellaneousError::ErrorCode13248 => None,
            TwilioMiscellaneousError::ErrorCode94600 => Some(r#"The action could not be performed because the filter limit was exceeded."#),
            TwilioMiscellaneousError::ErrorCode13244 => None,
            TwilioMiscellaneousError::ErrorCode51131 => None,
            TwilioMiscellaneousError::ErrorCode80902 => None,
            TwilioMiscellaneousError::ErrorCode21420 => None,
            TwilioMiscellaneousError::ErrorCode11237 => None,
            TwilioMiscellaneousError::ErrorCode61003 => None,
            TwilioMiscellaneousError::ErrorCode90022 => Some(r#"One of 'TemplateArgs' dictionary value is too long"#),
            TwilioMiscellaneousError::ErrorCode14108 => None,
            TwilioMiscellaneousError::ErrorCode20002 => None,
            TwilioMiscellaneousError::ErrorCode80406 => None,
            TwilioMiscellaneousError::ErrorCode21618 => None,
            TwilioMiscellaneousError::ErrorCode61007 => None,
            TwilioMiscellaneousError::ErrorCode19034 => Some(r#"Country code provided in the request is not valid"#),
            TwilioMiscellaneousError::ErrorCode11770 => None,
            TwilioMiscellaneousError::ErrorCode60007 => Some(r#"Security Token validation failed"#),
            TwilioMiscellaneousError::ErrorCode11300 => None,
            TwilioMiscellaneousError::ErrorCode80307 => Some(r#"Record to be updated was not found in database."#),
            TwilioMiscellaneousError::ErrorCode21241 => None,
            TwilioMiscellaneousError::ErrorCode19050 => None,
            TwilioMiscellaneousError::ErrorCode14105 => None,
            TwilioMiscellaneousError::ErrorCode80409 => None,
            TwilioMiscellaneousError::ErrorCode19025 => Some(r#"The request body has invalid inputs"#),
            TwilioMiscellaneousError::ErrorCode404 => Some(r#"Failed to find a match for the request"#),
            TwilioMiscellaneousError::ErrorCode13110 => None,
            TwilioMiscellaneousError::ErrorCode11200 => Some(r#"We try to provide specific webhook errors whenever possible, however in this instance we are unable to identify the exact cause of the webhook delivery/response issue. To prevent 11200 errors, ensure that your webhook receiving infrastructure can quickly respond to inbound Twilio requests with a 2xx status code."#),
            TwilioMiscellaneousError::ErrorCode51111 => None,
            TwilioMiscellaneousError::ErrorCode21224 => None,
            TwilioMiscellaneousError::ErrorCode14203 => None,
            TwilioMiscellaneousError::ErrorCode60006 => None,
            TwilioMiscellaneousError::ErrorCode21479 => None,
            TwilioMiscellaneousError::ErrorCode21451 => None,
            TwilioMiscellaneousError::ErrorCode90011 => Some(r#"MessageSid is invalid"#),
            TwilioMiscellaneousError::ErrorCode21480 => None,
            TwilioMiscellaneousError::ErrorCode13322 => None,
            TwilioMiscellaneousError::ErrorCode13254 => None,
            TwilioMiscellaneousError::ErrorCode11251 => None,
            TwilioMiscellaneousError::ErrorCode70103 => None,
            TwilioMiscellaneousError::ErrorCode90034 => Some(r#"'BroadcastStatusCallbackUrl' is too long"#),
            TwilioMiscellaneousError::ErrorCode21212 => Some(r#"You attempted to initiate an outbound phone call or message, but the `From` parameter you supplied was not a valid phone number, Alphanumeric Sender ID or approved WhatsApp Sender.

Twilio accepts phone numbers in [E.164 format](https://www.twilio.com/docs/glossary/what-e164) (i.e. "+1 format"), 10-digit US and Canadian numbers with any combination of non-digit separators, or Alphanumeric Sender IDs (SMS only) with up to 11 alphanumeric characters [a-zA-Z0-9]. Refer to [this page](https://support.twilio.com/hc/en-us/articles/223133867-Using-Alphanumeric-Sender-ID-with-Messaging-Services) for acceptable characters.

The number must not be on a do-not-originate (DNO) list, and Alphanumeric Sender IDs may not be generic.
"#),
            TwilioMiscellaneousError::ErrorCode13210 => None,
            TwilioMiscellaneousError::ErrorCode11201 => None,
            TwilioMiscellaneousError::ErrorCode20413 => None,
            TwilioMiscellaneousError::ErrorCode12102 => None,
            TwilioMiscellaneousError::ErrorCode51001 => None,
            TwilioMiscellaneousError::ErrorCode84001 => Some(r#"Execution timed out before completing the execution of all the blocks."#),
            TwilioMiscellaneousError::ErrorCode90029 => Some(r#"Broadcast 'CorrelationId' is empty"#),
            TwilioMiscellaneousError::ErrorCode19040 => Some(r#"The custom field provided in the request does not exist"#),
            TwilioMiscellaneousError::ErrorCode32111 => Some(r#"The header you asked Twilio to stamp was omitted because it contains illegal characters in its _name_. Including this header would have resulted in Twilio generating a message that runs contrary to the SIP specification.

The set of legal characters for a header name is defined in [RFC 3261](https://tools.ietf.org/html/rfc3261)."#),
            TwilioMiscellaneousError::ErrorCode90018 => Some(r#"'Template' field is too long"#),
            TwilioMiscellaneousError::ErrorCode94001 => Some(r#"Invalid “transcribe” value. It should be either true or false. No transcription resource has been created"#),
            TwilioMiscellaneousError::ErrorCode94201 => Some(r#"Request to enable encryption for transcriptions failed because encryptionKeySid is invalid or not defined"#),
            TwilioMiscellaneousError::ErrorCode13253 => None,
            TwilioMiscellaneousError::ErrorCode19004 => Some(r#"The request body provided has some errors"#),
            TwilioMiscellaneousError::ErrorCode19005 => Some(r#"The data provided in the request body is invalid"#),
            TwilioMiscellaneousError::ErrorCode90019 => Some(r#"'TemplateArgs' dictionary size is too large"#),
            TwilioMiscellaneousError::ErrorCode400 => Some(r#"Failed to complete request due to a bad request"#),
            TwilioMiscellaneousError::ErrorCode13520 => None,
            TwilioMiscellaneousError::ErrorCode21609 => None,
            TwilioMiscellaneousError::ErrorCode51121 => None,
            TwilioMiscellaneousError::ErrorCode84002 => Some(r#"Workflow execution failed."#),
            TwilioMiscellaneousError::ErrorCode11310 => None,
            TwilioMiscellaneousError::ErrorCode60008 => None,
            TwilioMiscellaneousError::ErrorCode94300 => Some(r#"Invalid callback configuration, either to create a new configuration or update existing one. A valid Url and Method must be specified to enable callbacks via HTTP request (webhook) and receive transcriptions status updates."#),
            TwilioMiscellaneousError::ErrorCode51112 => None,
            TwilioMiscellaneousError::ErrorCode94301 => Some(r#"A UniqueName is required to create a new configuration"#),
            TwilioMiscellaneousError::ErrorCode20005 => None,
            TwilioMiscellaneousError::ErrorCode32110 => Some(r#"Twilio cannot understand parts of your SIP URI. This may be due to invalid characters appearing in the user, params, or headers portion. While Twilio may still attempt to complete your call, the parts of the URI that are invalid will be ignored.

The format for a valid SIP URI is defined in [RFC 3261](https://tools.ietf.org/html/rfc3261)."#),
            TwilioMiscellaneousError::ErrorCode51120 => None,
            TwilioMiscellaneousError::ErrorCode21230 => None,
            TwilioMiscellaneousError::ErrorCode90038 => Some(r#"'BroadcastStatusCallbackUrl' is invalid"#),
            TwilioMiscellaneousError::ErrorCode21214 => None,
            TwilioMiscellaneousError::ErrorCode51007 => Some(r#"## Error - 51007

### Client Connection: Token authentication is rejected by authentication service



"#),
            TwilioMiscellaneousError::ErrorCode80603 => Some(r#"Session UniqueName must be unique."#),
            TwilioMiscellaneousError::ErrorCode90026 => Some(r#"Template body could not be parsed by Mustache compiler"#),
            TwilioMiscellaneousError::ErrorCode13224 => Some(r#"Twilio does not support calling this number or the number is invalid"#),
            TwilioMiscellaneousError::ErrorCode14206 => None,
            TwilioMiscellaneousError::ErrorCode94604 => Some(r#"The Account is on Legal Hold, the operation could not be performed."#),
            TwilioMiscellaneousError::ErrorCode11241 => Some(r#"There was an error connecting to the URL due to an unsupported edge location parameter."#),
            TwilioMiscellaneousError::ErrorCode13237 => None,
            TwilioMiscellaneousError::ErrorCode80405 => None,
            TwilioMiscellaneousError::ErrorCode21474 => None,
            TwilioMiscellaneousError::ErrorCode13250 => None,
            TwilioMiscellaneousError::ErrorCode13314 => None,
            TwilioMiscellaneousError::ErrorCode50069 => None,
            TwilioMiscellaneousError::ErrorCode51119 => None,
            TwilioMiscellaneousError::ErrorCode80610 => None,
            TwilioMiscellaneousError::ErrorCode21213 => None,
            TwilioMiscellaneousError::ErrorCode90036 => Some(r#"Broadcast Recipient's 'body' is too long"#),
            TwilioMiscellaneousError::ErrorCode13211 => None,
            TwilioMiscellaneousError::ErrorCode51005 => None,
            TwilioMiscellaneousError::ErrorCode12400 => None,
            TwilioMiscellaneousError::ErrorCode32114 => Some(r#"Twilio parses certain standard URI parameters, such as the "transport" parameter, as well as Twilio-specific parameters such as "secure"."#),
            TwilioMiscellaneousError::ErrorCode21450 => None,
            TwilioMiscellaneousError::ErrorCode21454 => None,
            TwilioMiscellaneousError::ErrorCode21626 => None,
            TwilioMiscellaneousError::ErrorCode13410 => None,
            TwilioMiscellaneousError::ErrorCode21401 => Some(r#"## Error - 21401

### Invalid Phone Number

The phone number you specified was not a valid SMS-enabled phone number or alphanumeric sender ID.

If a phone number was used, this number must be one you have purchased from or ported to Twilio in [E.164 format][e164].

Alphanumeric sender IDs can only be used with [accepted characters][alpha-sender-id] when messaging [countries where this feature is supported][alphanumeric-countries].

[e164]: http://en.wikipedia.org/wiki/E.164
[alphanumeric-countries]: https://support.twilio.com/hc/en-us/articles/223133767-International-support-for-Alphanumeric-Sender-ID
[alpha-sender-id]: /docs/api/rest/sending-messages#alpha-sender-id"#),
            TwilioMiscellaneousError::ErrorCode70153 => None,
            TwilioMiscellaneousError::ErrorCode70152 => None,
            TwilioMiscellaneousError::ErrorCode80202 => None,
            TwilioMiscellaneousError::ErrorCode20503 => None,
            TwilioMiscellaneousError::ErrorCode94000 => Some(r#"Twilio failed to process the request to transcribe audio due to an internal issue. No transcription resource has been created"#),
            TwilioMiscellaneousError::ErrorCode13232 => None,
            TwilioMiscellaneousError::ErrorCode21901 => Some(r#"DltTemplateId is invalid"#),
            TwilioMiscellaneousError::ErrorCode19013 => Some(r#"Contact should contain at least one required field"#),
            TwilioMiscellaneousError::ErrorCode10004 => Some(r#"## Error - 10004

### Call concurrency limit exceeded

The call was rejected because you reached the maximum limit of concurrent calls available for your account."#),
            TwilioMiscellaneousError::ErrorCode19037 => Some(r#"No location updates provided in the request"#),
            TwilioMiscellaneousError::ErrorCode13213 => None,
            TwilioMiscellaneousError::ErrorCode11205 => Some(r#"### HTTP connection failure

There was a network failure attempting to connect to this URL. Twilio will try for 10 seconds to establish a TCP connection to your URL (two 5-second attempts). Twilio will wait for a total of 15 seconds to receive an HTTP response. This includes the TCP connection time, so if it took six seconds to establish a TCP connection, your server would have 9 seconds to deliver an HTTP response. If either of those timers expire, we fail the request, fire a notification, and try your fallback URL, if one is specified."#),
            TwilioMiscellaneousError::ErrorCode51102 => None,
            TwilioMiscellaneousError::ErrorCode51129 => None,
            TwilioMiscellaneousError::ErrorCode14223 => None,
            TwilioMiscellaneousError::ErrorCode410 => Some(r#"Access to this resource is no longer available and should not be retried.
"#),
            TwilioMiscellaneousError::ErrorCode94602 => Some(r#"The action could not be performed because the FromDate is invalid."#),
            TwilioMiscellaneousError::ErrorCode20003 => Some(r#"#### Authenticating requests to the Twilio API

All requests to sensitive areas of the Twilio API must use [HTTP Basic
Authentication][auth]. Authenticate using your Account SID as the username,
and your Auth Token as the password. Both can be found in the [Twilio
Console](/console/account/settings).

You can also generate revocable [API Keys](/docs/iam/keys/api-key) to authenticate.

You may want to consider using a [Twilio helper library](/docs/libraries)
(available in PHP, .NET, Java, Ruby, Python and Node.js) as they will take care of
authentication for you.

[auth]: http://en.wikipedia.org/wiki/Basic_access_authentication"#),
            TwilioMiscellaneousError::ErrorCode80701 => None,
            TwilioMiscellaneousError::ErrorCode21217 => None,
            TwilioMiscellaneousError::ErrorCode21621 => Some(r#"The `From` number is not enabled for MMS messaging.
"#),
            TwilioMiscellaneousError::ErrorCode13249 => None,
            TwilioMiscellaneousError::ErrorCode13215 => None,
            TwilioMiscellaneousError::ErrorCode21202 => None,
            TwilioMiscellaneousError::ErrorCode21225 => None,
            TwilioMiscellaneousError::ErrorCode21455 => None,
            TwilioMiscellaneousError::ErrorCode94101 => Some(r#"Twilio attempted to send a transcription event to the callback URL specified and your application didn’t respond before time out"#),
            TwilioMiscellaneousError::ErrorCode13235 => None,
            TwilioMiscellaneousError::ErrorCode51128 => None,
            TwilioMiscellaneousError::ErrorCode13236 => None,
            TwilioMiscellaneousError::ErrorCode21405 => None,
            TwilioMiscellaneousError::ErrorCode70002 => None,
            TwilioMiscellaneousError::ErrorCode21223 => None,
            TwilioMiscellaneousError::ErrorCode21900 => Some(r#"DltPEId is invalid"#),
            TwilioMiscellaneousError::ErrorCode15003 => None,
            TwilioMiscellaneousError::ErrorCode19003 => Some(r#"unique_customer_provided_id already exists"#),
            TwilioMiscellaneousError::ErrorCode15000 => None,
            TwilioMiscellaneousError::ErrorCode21240 => None,
            TwilioMiscellaneousError::ErrorCode21206 => None,
            TwilioMiscellaneousError::ErrorCode70053 => None,
            TwilioMiscellaneousError::ErrorCode80205 => None,
            TwilioMiscellaneousError::ErrorCode21228 => None,
            TwilioMiscellaneousError::ErrorCode21232 => None,
            TwilioMiscellaneousError::ErrorCode91202 => Some(r#"Resource is not found"#),
            TwilioMiscellaneousError::ErrorCode51103 => None,
            TwilioMiscellaneousError::ErrorCode80908 => None,
            TwilioMiscellaneousError::ErrorCode80601 => None,
            TwilioMiscellaneousError::ErrorCode13710 => None,
            TwilioMiscellaneousError::ErrorCode14104 => None,
            TwilioMiscellaneousError::ErrorCode80402 => None,
            TwilioMiscellaneousError::ErrorCode90020 => Some(r#"One of 'TemplateArgs' dictionary key is blank"#),
            TwilioMiscellaneousError::ErrorCode21221 => None,
            TwilioMiscellaneousError::ErrorCode20001 => None,
            TwilioMiscellaneousError::ErrorCode90028 => Some(r#"Broadcast 'IdempotencyToken' is too long"#),
            TwilioMiscellaneousError::ErrorCode30014 => None,
            TwilioMiscellaneousError::ErrorCode21100 => None,
            TwilioMiscellaneousError::ErrorCode21476 => None,
            TwilioMiscellaneousError::ErrorCode20426 => None,
            TwilioMiscellaneousError::ErrorCode90033 => Some(r#"Broadcast recipient's 'to' is too long"#),
            TwilioMiscellaneousError::ErrorCode20009 => Some(r#"You attempted to delete a call or message resource before it was completed. If you are attempting to delete a call, wait until the call is completed and try your request again. If you are attempting to delete a message, wait several minutes and try again.

If you're seeing a message in the logs that you have already deleted, please allow up to 14 days for our systems to completely remove this record before reaching out to support."#),
            TwilioMiscellaneousError::ErrorCode94302 => Some(r#"Invalid UniqueName value, either to create a new configuration or update existing one"#),
            TwilioMiscellaneousError::ErrorCode13613 => None,
            TwilioMiscellaneousError::ErrorCode11210 => Some(r#"The DNS entry for the URL’s host cannot be resolved.
"#),
            TwilioMiscellaneousError::ErrorCode21219 => None,
            TwilioMiscellaneousError::ErrorCode21404 => None,
            TwilioMiscellaneousError::ErrorCode21622 => None,
            TwilioMiscellaneousError::ErrorCode61002 => None,
            TwilioMiscellaneousError::ErrorCode20008 => None,
            TwilioMiscellaneousError::ErrorCode51115 => None,
            TwilioMiscellaneousError::ErrorCode61006 => None,
            TwilioMiscellaneousError::ErrorCode20010 => None,
            TwilioMiscellaneousError::ErrorCode84003 => Some(r#"A block could not be rendered. "#),
            TwilioMiscellaneousError::ErrorCode21208 => None,
            TwilioMiscellaneousError::ErrorCode21200 => None,
            TwilioMiscellaneousError::ErrorCode90023 => Some(r#"One of 'TemplateArgs' dictionary value is null"#),
            TwilioMiscellaneousError::ErrorCode90035 => Some(r#"Broadcast 'MessageStatusCallbackUrl' is too long"#),
            TwilioMiscellaneousError::ErrorCode80609 => None,
            TwilioMiscellaneousError::ErrorCode51124 => None,
            TwilioMiscellaneousError::ErrorCode70005 => None,
            TwilioMiscellaneousError::ErrorCode14205 => None,
            TwilioMiscellaneousError::ErrorCode80408 => None,
            TwilioMiscellaneousError::ErrorCode13243 => None,
            TwilioMiscellaneousError::ErrorCode21238 => None,
            TwilioMiscellaneousError::ErrorCode13320 => None,
            TwilioMiscellaneousError::ErrorCode30015 => None,
            TwilioMiscellaneousError::ErrorCode50353 => None,
            TwilioMiscellaneousError::ErrorCode21458 => None,
            TwilioMiscellaneousError::ErrorCode14201 => None,
            TwilioMiscellaneousError::ErrorCode70101 => None,
            TwilioMiscellaneousError::ErrorCode21243 => None,
            TwilioMiscellaneousError::ErrorCode51123 => None,
            TwilioMiscellaneousError::ErrorCode20023 => Some(r#"Phone number is not correct: it cannot be null and have non-decimal symbols"#),
            TwilioMiscellaneousError::ErrorCode70151 => None,
            TwilioMiscellaneousError::ErrorCode51002 => None,
            TwilioMiscellaneousError::ErrorCode80102 => None,
            TwilioMiscellaneousError::ErrorCode70003 => None,
            TwilioMiscellaneousError::ErrorCode90024 => Some(r#"Template body has tag which is not provided in 'TemplateArgs'"#),
            TwilioMiscellaneousError::ErrorCode13255 => None,
            TwilioMiscellaneousError::ErrorCode14215 => None,
            TwilioMiscellaneousError::ErrorCode14102 => None,
            TwilioMiscellaneousError::ErrorCode12200 => Some(r#"## Warning - 12200 

### Schema validation warning

The provided XML does not conform to the Twilio Markup XML schema.  Please refer to the specific error and correct the problem."#),
            TwilioMiscellaneousError::ErrorCode94501 => Some(r#"The action could not be performed because the Status provided is not valid."#),
            TwilioMiscellaneousError::ErrorCode80907 => None,
            TwilioMiscellaneousError::ErrorCode61009 => None,
            TwilioMiscellaneousError::ErrorCode20006 => None,
            TwilioMiscellaneousError::ErrorCode14110 => None,
            TwilioMiscellaneousError::ErrorCode80104 => None,
            TwilioMiscellaneousError::ErrorCode11215 => None,
            TwilioMiscellaneousError::ErrorCode14210 => None,
            TwilioMiscellaneousError::ErrorCode21205 => None,
            TwilioMiscellaneousError::ErrorCode19036 => Some(r#"The page token provided in the request is not valid"#),
            TwilioMiscellaneousError::ErrorCode60001 => Some(r#"Downstream Authentication Failed"#),
            TwilioMiscellaneousError::ErrorCode70154 => None,
            TwilioMiscellaneousError::ErrorCode14103 => None,
            TwilioMiscellaneousError::ErrorCode13201 => None,
            TwilioMiscellaneousError::ErrorCode13311 => None,
            TwilioMiscellaneousError::ErrorCode11240 => Some(r#"There was an error connecting to the URL due to an invalid edge location parameter."#),
            TwilioMiscellaneousError::ErrorCode11220 => None,
            TwilioMiscellaneousError::ErrorCode21470 => None,
            TwilioMiscellaneousError::ErrorCode19020 => Some(r#"Channel value already exists "#),
            TwilioMiscellaneousError::ErrorCode30013 => None,
            TwilioMiscellaneousError::ErrorCode51101 => None,
            TwilioMiscellaneousError::ErrorCode94603 => Some(r#"The action could not be performed because the TTL provided is out of range."#),
            TwilioMiscellaneousError::ErrorCode51117 => None,
            TwilioMiscellaneousError::ErrorCode21617 => Some(r#"
"#),
            TwilioMiscellaneousError::ErrorCode90040 => Some(r#"Broadcast 'MediaUrls' list has too many items"#),
            TwilioMiscellaneousError::ErrorCode90021 => Some(r#"One of 'TemplateArgs' dictionary key is too long"#),
            TwilioMiscellaneousError::ErrorCode84000 => Some(r#"This limitation is enforced to prevent infinite loops. Please try to design your flows such that they terminate."#),
            TwilioMiscellaneousError::ErrorCode80401 => None,
            TwilioMiscellaneousError::ErrorCode13325 => None
        }
    }
}
