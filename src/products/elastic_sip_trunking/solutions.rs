// AUTO-GENERATED. DO NOT MODIFY. YOUR CHANGES WILL BE OVER-WRITTEN
use crate::products::TwilioElasticSipTrunkingError;
use standard_error::traits::StandardErrorSolutionsTrait;

impl StandardErrorSolutionsTrait for TwilioElasticSipTrunkingError {
    fn solutions(&self) -> Option<&'static str> {
        match self {
            TwilioElasticSipTrunkingError::ErrorCode21244 => Some(r#"*  Please see limits allowed, visit [Trunking Key Concepts](/docs/sip-trunking/scale-and-limits)."#),
            TwilioElasticSipTrunkingError::ErrorCode21257 => Some(r#"Verify the SIP Manipulation Policy SID is valid."#),
            TwilioElasticSipTrunkingError::ErrorCode21253 => Some(r#"* Delete an existing Connection Policy if you must create more."#),
            TwilioElasticSipTrunkingError::ErrorCode32208 => None,
            TwilioElasticSipTrunkingError::ErrorCode32219 => Some(r#"Review the Possible Causes and update the redirect target as needed."#),
            TwilioElasticSipTrunkingError::ErrorCode21248 => Some(r#"*  Please review your Termination URI (Domain name) and select a different one."#),
            TwilioElasticSipTrunkingError::ErrorCode32202 => None,
            TwilioElasticSipTrunkingError::ErrorCode32205 => Some(r#"Please check your [Voice Dialing Geographic Permissions](https://www.twilio.com/console/voice/calls/geo-permissions), fix it, and try again."#),
            TwilioElasticSipTrunkingError::ErrorCode21252 => Some(r#"*  Please see [Localized Termination URIs](/docs/sip-trunking#localized-termination-uris) for more information on the valid Elastic SIP Trunking Regions."#),
            TwilioElasticSipTrunkingError::ErrorCode32207 => None,
            TwilioElasticSipTrunkingError::ErrorCode21249 => Some(r#"* Please see [Multiple Origination URIs](/docs/sip-trunking#multiple-orig-uris) for more information."#),
            TwilioElasticSipTrunkingError::ErrorCode21256 => Some(r#"Follow the json format for the ruleset found in the spec and use a valid ruleset."#),
            TwilioElasticSipTrunkingError::ErrorCode21259 => Some(r#"Remove unused Sip Manipulation Policies."#),
            TwilioElasticSipTrunkingError::ErrorCode32218 => None,
            TwilioElasticSipTrunkingError::ErrorCode32001 => None,
            TwilioElasticSipTrunkingError::ErrorCode32005 => Some(r#"Please reach out to Support for assistance. "#),
            TwilioElasticSipTrunkingError::ErrorCode21247 => Some(r#"*  One of these constraints in the "Possible Causes" has been be violated, please review your Termination URI (Domain name)."#),
            TwilioElasticSipTrunkingError::ErrorCode32203 => Some(r#"Potential solutions may be:
- If your destination is being incorrectly identified as high-risk of fraud and you have a legitimate need to call this number, please reach out to Twilio Support through the [Console](https://console.twilio.com) or [Help Center](https://support.twilio.com/hc/en-us).
- If you are placing a call to a +1 destination and your account is missing a Primary Customer Profile, create one in [TrustHub](https://console.twilio.com/us1/account/trust-hub/customer-profiles). "#),
            TwilioElasticSipTrunkingError::ErrorCode32100 => None,
            TwilioElasticSipTrunkingError::ErrorCode32012 => Some(r#"*  You've exceeded the number of calls per second you are authorized to make on your Parent Account. "#),
            TwilioElasticSipTrunkingError::ErrorCode32011 => Some(r#"* Ensure the SIP URI used in the request is correctly configured.
 * For details on how to configure your Origination URIs for SIP trunking, please see [Origination settings](/docs/sip-trunking#origination).
 * For SIP interfaces verify that your SIP URI in your <Dial> <SIP> TwiML or REST API call, see [Receiving SIP from Twilio](/docs/voice/api/receiving-sip).
* Ensure you've allowed Twilio's IP address ranges and ports on your firewall for SIP signalling and RTP media traffic, see [IP addresses - Trunking](/docs/sip-trunking#ip-addresses) or [IP addresses - SIP Interface](/docs/voice/api/sip-interface#ip-addresses)
* Ensure that your SIP servers are running properly and is properly processing calls from Twilio.
* Ensure that your SIP servers support TLSv1.2, or configure your account to allow TLSv1.0+"#),
            TwilioElasticSipTrunkingError::ErrorCode32201 => None,
            TwilioElasticSipTrunkingError::ErrorCode21245 => Some(r#"*  Please update your Trunk settings to avoid the conditions stated above. Learn more: [Configure Trunks](/docs/sip-trunking#configure-trunks)."#),
            TwilioElasticSipTrunkingError::ErrorCode32010 => Some(r#"Please verify your Origination settings of your [Elastic SIP Trunk](/docs/sip-trunking#origination) or [BYOC Trunk](/docs/voice/bring-your-own-carrier-byoc#origination-connection-policy-settings)."#),
            TwilioElasticSipTrunkingError::ErrorCode32206 => Some(r#"* Ensure your number is formatted in [E.164 format](https://www.twilio.com/docs/glossary/what-e164)
* Ensure that your `From` number is assigned and not on a do-not-originate (DNO) list."#),
            TwilioElasticSipTrunkingError::ErrorCode21260 => Some(r#"Optimize your rule by removing unused actions."#),
            TwilioElasticSipTrunkingError::ErrorCode21261 => Some(r#"Optimize your rule by removing unused conditions."#),
            TwilioElasticSipTrunkingError::ErrorCode32115 => Some(r#"Verify that the call reason supplied does not exceed more than 50 characters"#),
            TwilioElasticSipTrunkingError::ErrorCode21258 => Some(r#"Reference the docs for a possible configuation solition - https://www.twilio.com/docs/sip-trunking/sip-header-manipulation"#),
            TwilioElasticSipTrunkingError::ErrorCode21634 => Some(r#"* Refer to the docs on <a href="/docs/sip-trunking/emergency-calling">
Emergency Calling</a> over SIP trunking for more details."#),
            TwilioElasticSipTrunkingError::ErrorCode32222 => Some(r#"Upgrade TLS version or configure your account to allow all TLSv1+ in console (Voice → Settings → General → Allow Deprecated SIP/TLS versions)"#),
            TwilioElasticSipTrunkingError::ErrorCode21254 => Some(r#"* Delete an existing entry from the Connection Policy if you must create more."#),
            TwilioElasticSipTrunkingError::ErrorCode21251 => Some(r#"*  This CPS change is not allowed, kindly contact Twilio Support through the [Console](https://console.twilio.com) or [Help Center](https://support.twilio.com)."#),
            TwilioElasticSipTrunkingError::ErrorCode32204 => Some(r#"Change the callerID used in the From field of the SIP INVITE to a Twilio number on your account or verify your number by visiting the Console [Verified Caller IDs](/user/account/phone-numbers/verified) or REST API."#),
            TwilioElasticSipTrunkingError::ErrorCode32200 => None,
            TwilioElasticSipTrunkingError::ErrorCode32020 => Some(r#"The call is rejected. Reattempt the call with a new passport."#)
        }
    }
}
