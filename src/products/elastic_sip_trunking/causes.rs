// AUTO-GENERATED. DO NOT MODIFY. YOUR CHANGES WILL BE OVER-WRITTEN
use crate::products::TwilioElasticSipTrunkingError;
use standard_error::traits::StandardErrorCausesTrait;

impl StandardErrorCausesTrait for TwilioElasticSipTrunkingError {
    fn causes(&self) -> Option<&'static str> {
        match self {
            TwilioElasticSipTrunkingError::ErrorCode21244 => Some(r#"*   Maximum Number of Trunks reached."#),
            TwilioElasticSipTrunkingError::ErrorCode21257 => Some(r#"SIP Manipulation Policy SID is incorrect."#),
            TwilioElasticSipTrunkingError::ErrorCode21253 => Some(r#"* An account cannot have more than 10000 Connection Policies."#),
            TwilioElasticSipTrunkingError::ErrorCode32208 => None,
            TwilioElasticSipTrunkingError::ErrorCode32219 => Some(r#"*    Max redirect limit reached.
*    Redirection is not allowed in the call to the registered SIP endpoint.
*    Redirect target in the Contact header of the SIP 30x response is not a valid SIP URI.
*    Redirect target is a private IP address.
*    Redirect target is a Twilio domain such as \*.sip.twilio.com or \*.pstn.twilio.com.
*    Redirect target is an IP in a deny list."#),
            TwilioElasticSipTrunkingError::ErrorCode21248 => Some(r#"*   This Termination URI (Domain) is not available."#),
            TwilioElasticSipTrunkingError::ErrorCode32202 => None,
            TwilioElasticSipTrunkingError::ErrorCode32205 => Some(r#"User dialed a destination your application is not enabled to support calling to."#),
            TwilioElasticSipTrunkingError::ErrorCode21252 => Some(r#"*   The Region requested is not valid."#),
            TwilioElasticSipTrunkingError::ErrorCode32207 => None,
            TwilioElasticSipTrunkingError::ErrorCode21249 => Some(r#"*   You've configured the maximum number of Origination URIs allowed for your Trunk."#),
            TwilioElasticSipTrunkingError::ErrorCode21256 => Some(r#"The json format of the ruleset does not match the spec or duplicate rule name."#),
            TwilioElasticSipTrunkingError::ErrorCode21259 => Some(r#"The number of Sip Manipulation Policies per account cannot exceed the limit. See https://www.twilio.com/docs/sip-trunking/sip-header-manipulation "#),
            TwilioElasticSipTrunkingError::ErrorCode32218 => None,
            TwilioElasticSipTrunkingError::ErrorCode32001 => None,
            TwilioElasticSipTrunkingError::ErrorCode32005 => Some(r#"Your account may have been flagged for review."#),
            TwilioElasticSipTrunkingError::ErrorCode21247 => Some(r#"*  Cannot create subdomain because parent domain does not exist/account does not own it
* Cannot delete trunk while it has subdomains
* Cannot rename subdomain because parent domain does not exist/account doesn't own it
* Cannot rename domain while it has subdomains"#),
            TwilioElasticSipTrunkingError::ErrorCode32203 => Some(r#"The outbound call to the destination number was blocked by Twilio. Potential causes may be:
- The destination has a high-risk of fraud
- Due to regulatory reasons, the destination cannot be reached
- You are placing a call to a +1 destination and your account is missing a Primary Customer Profile"#),
            TwilioElasticSipTrunkingError::ErrorCode32100 => None,
            TwilioElasticSipTrunkingError::ErrorCode32012 => Some(r#"*  Calls Per Second placed on a SIP trunk exceeded the Parent Account pool limit. "#),
            TwilioElasticSipTrunkingError::ErrorCode32011 => Some(r#"* Your SIP endpoint is not reachable due to network connectivity issue between Twilio and your system
* Your SIP endpoint is not responding (service down or maintenance) 
* There is a firewall in your network that is blocking SIP traffic from Twilio
* Your SIP endpoint is sending an error response, such as SIP 500 response.
* The SIP URI specified in your Trunking Origination URI, <Dial> <SIP> TwiML or REST API call is invalid
* Your SIP endpoint does not support TLSv1.2"#),
            TwilioElasticSipTrunkingError::ErrorCode32201 => None,
            TwilioElasticSipTrunkingError::ErrorCode21245 => Some(r#"* Cannot use another account's SID as the Termination URI for this Trunk
* Cannot create more than one Trunk with the same Termination URI
* Credential List or IP-ACL already associated with Trunk"#),
            TwilioElasticSipTrunkingError::ErrorCode32010 => Some(r#"Your Elastic SIP Trunk or BYOC Trunk does not have any valid active origination URLs"#),
            TwilioElasticSipTrunkingError::ErrorCode32206 => Some(r#"* You have supplied a phone number that was not in [E.164 format](https://www.twilio.com/docs/glossary/what-e164)
* Your `From` phone number is on a do-not-originate (DNO) list"#),
            TwilioElasticSipTrunkingError::ErrorCode21260 => Some(r#"The number of actions in a given rule cannot exceed the limit. See https://www.twilio.com/docs/sip-trunking/sip-header-manipulation "#),
            TwilioElasticSipTrunkingError::ErrorCode21261 => Some(r#"The number of conditions in a given rule cannot exceed the limit. See https://www.twilio.com/docs/sip-trunking/sip-header-manipulation "#),
            TwilioElasticSipTrunkingError::ErrorCode32115 => Some(r#"X-Branded-CallReason header contains an invalid value exceeding 50 character limit. "#),
            TwilioElasticSipTrunkingError::ErrorCode21258 => Some(r#"Reference the docs for configuration information - https://www.twilio.com/docs/sip-trunking/sip-header-manipulation "#),
            TwilioElasticSipTrunkingError::ErrorCode21634 => Some(r#"* One or more numbers associated with this SIP Trunk are provisioned for emergency calling."#),
            TwilioElasticSipTrunkingError::ErrorCode32222 => Some(r#"SIP/TLS version is lower than the allowed"#),
            TwilioElasticSipTrunkingError::ErrorCode21254 => Some(r#"* A Connection Policy cannot have more than 10 entries."#),
            TwilioElasticSipTrunkingError::ErrorCode21251 => Some(r#"*   This CPS change is not allowed."#),
            TwilioElasticSipTrunkingError::ErrorCode32204 => Some(r#"The `From` number used to make outbound call not a Twilio number or not a verified caller ID"#),
            TwilioElasticSipTrunkingError::ErrorCode32200 => None,
            TwilioElasticSipTrunkingError::ErrorCode32020 => Some(r#"See the error message for details"#)
        }
    }
}
