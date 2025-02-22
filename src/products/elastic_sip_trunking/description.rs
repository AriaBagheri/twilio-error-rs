// AUTO-GENERATED. DO NOT MODIFY. YOUR CHANGES WILL BE OVER-WRITTEN
use crate::products::TwilioElasticSipTrunkingError;
use standard_error::traits::StandardErrorDescriptionTrait;

impl StandardErrorDescriptionTrait for TwilioElasticSipTrunkingError {
    fn description(&self) -> Option<&'static str> {
        match self {
            TwilioElasticSipTrunkingError::ErrorCode21244 => None,
            TwilioElasticSipTrunkingError::ErrorCode21257 => Some(r#"Operation could not be performed because the SIP Manipulation Policy SID does not exist."#),
            TwilioElasticSipTrunkingError::ErrorCode21253 => None,
            TwilioElasticSipTrunkingError::ErrorCode32208 => None,
            TwilioElasticSipTrunkingError::ErrorCode32219 => Some(r#"Twilio could not process the SIP 30x response from your SIP server.
"#),
            TwilioElasticSipTrunkingError::ErrorCode21248 => None,
            TwilioElasticSipTrunkingError::ErrorCode32202 => None,
            TwilioElasticSipTrunkingError::ErrorCode32205 => Some(r#"You attempted to initiate an outbound phone call to a phone number that is not enabled on your account."#),
            TwilioElasticSipTrunkingError::ErrorCode21252 => None,
            TwilioElasticSipTrunkingError::ErrorCode32207 => None,
            TwilioElasticSipTrunkingError::ErrorCode21249 => None,
            TwilioElasticSipTrunkingError::ErrorCode21256 => Some(r#"The ruleset is invalid"#),
            TwilioElasticSipTrunkingError::ErrorCode21259 => Some(r#"Your account already has the maximum number of Sip Manipulation Policies allowed."#),
            TwilioElasticSipTrunkingError::ErrorCode32218 => None,
            TwilioElasticSipTrunkingError::ErrorCode32001 => None,
            TwilioElasticSipTrunkingError::ErrorCode32005 => Some(r#"Voice calling has been disabled for this account. "#),
            TwilioElasticSipTrunkingError::ErrorCode21247 => None,
            TwilioElasticSipTrunkingError::ErrorCode32203 => Some(r#"The destination number is blocked by Twilio."#),
            TwilioElasticSipTrunkingError::ErrorCode32100 => None,
            TwilioElasticSipTrunkingError::ErrorCode32012 => Some(r#"#### Possible Causes 
*  Calls Per Second placed on a SIP trunk exceeded the Parent Account pool limit. 

#### Possible Solutions
*  You've exceeded the number of calls per second you are authorized to make on your Parent Account. "#),
            TwilioElasticSipTrunkingError::ErrorCode32011 => Some(r#"There was a problem communicating with a specific endpoint of your SIP communications infrastructure. This means there was either a lack of timely response, an error response or an invalid response from your SIP endpoint. This may result in increased call setup times or even failed call depending on the failover configuration for your Elastic SIP Trunk or SIP application. Twilio will make multiple attempts to deliver calls to your endpoint and each failed attempt will have its own notification.
The notification will have details about the specific error response and the SIP URI that causes the failure."#),
            TwilioElasticSipTrunkingError::ErrorCode32201 => None,
            TwilioElasticSipTrunkingError::ErrorCode21245 => None,
            TwilioElasticSipTrunkingError::ErrorCode32010 => Some(r#"Your call can't be completed because no valid/active origination URLs could be found for your Elastic SIP Trunk or BYOC Trunk."#),
            TwilioElasticSipTrunkingError::ErrorCode32206 => Some(r#"You attempted to initiate an outbound phone call or message, but the 'From' parameter you supplied was not a valid phone number or alphanumeric sender ID.

Twilio accepts phone numbers in [E.164 format](https://www.twilio.com/docs/glossary/what-e164) (i.e. "+1 format"), 10-digit US and Canadian numbers with any combination of non-digit separators, or Alphanumeric Sender IDs (SMS only) with up to 11 alphanumeric characters [a-zA-Z0-9]. Refer to [this page](https://support.twilio.com/hc/en-us/articles/223133867-Using-Alphanumeric-Sender-ID-with-Messaging-Services) for acceptable characters.

The number must not be on a do-not-originate (DNO) list, and Alpha Sender IDs may not be generic."#),
            TwilioElasticSipTrunkingError::ErrorCode21260 => Some(r#"This rule already has the maximum number of actions allowed."#),
            TwilioElasticSipTrunkingError::ErrorCode21261 => Some(r#"This rule already has the maximum number of conditions allowed."#),
            TwilioElasticSipTrunkingError::ErrorCode32115 => Some(r#"X-Branded-CallReason header contains an invalid value."#),
            TwilioElasticSipTrunkingError::ErrorCode21258 => Some(r#"There was a validation error for the Sip Manipulation Policy in question."#),
            TwilioElasticSipTrunkingError::ErrorCode21634 => Some(r#"Please disable emergency calling on these numbers before
proceeding."#),
            TwilioElasticSipTrunkingError::ErrorCode32222 => Some(r#"The TLS version used by your SIP endpoints is lower than the one configured for your account. It's highly recommended to upgrade your SIP endpoints to use TLSv1.2. Otherwise, you can opt in to use deprecated TLS versions in the console. By default, only TLSv1.2 is allowed."#),
            TwilioElasticSipTrunkingError::ErrorCode21254 => None,
            TwilioElasticSipTrunkingError::ErrorCode21251 => None,
            TwilioElasticSipTrunkingError::ErrorCode32204 => Some(r#"You attempted to initiate an outbound phone call, but the <span class='rest-attribute'>From</span> number you specified is not a verified Outgoing Caller ID for your account.  In order to use a phone number as the Caller ID on outgoing calls, you must first validate your ownership of that phone number."#),
            TwilioElasticSipTrunkingError::ErrorCode32200 => None,
            TwilioElasticSipTrunkingError::ErrorCode32020 => Some(r#"Twilio cannot verify incoming SHAKEN PASSporT from customer"#)
        }
    }
}
