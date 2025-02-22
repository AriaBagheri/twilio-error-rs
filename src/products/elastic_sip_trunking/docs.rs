// AUTO-GENERATED. DO NOT MODIFY. YOUR CHANGES WILL BE OVER-WRITTEN
use crate::products::TwilioElasticSipTrunkingError;
use standard_error::traits::StandardErrorDocsTrait;

impl StandardErrorDocsTrait for TwilioElasticSipTrunkingError {
    fn docs(&self) -> &'static str {
        match self {
            TwilioElasticSipTrunkingError::ErrorCode21244 => r#"## ERROR - 21244

### Maximum Number of Trunks reached

 

#### Possible Causes
*   Maximum Number of Trunks reached.

#### Possible Solutions
*  Please see limits allowed, visit [Trunking Key Concepts](/docs/sip-trunking/scale-and-limits)."#,
            TwilioElasticSipTrunkingError::ErrorCode21257 => r#"## ERROR - 21257

### Invalid SIP Manipulation Policy SID

 Operation could not be performed because the SIP Manipulation Policy SID does not exist.

#### Possible Causes
SIP Manipulation Policy SID is incorrect.

#### Possible Solutions
Verify the SIP Manipulation Policy SID is valid."#,
            TwilioElasticSipTrunkingError::ErrorCode21253 => r#"## ERROR - 21253

### Max Connection Policies Reached

An account cannot have more than 10000 Connection Policies. 

#### Possible Causes
* An account cannot have more than 10000 Connection Policies.

#### Possible Solutions
* Delete an existing Connection Policy if you must create more."#,
            TwilioElasticSipTrunkingError::ErrorCode32208 => r#"## Error - 32208

###  SIP: Secure media required

#### Possible Causes 
*    This secure SIP trunk cannot accept insecure media (RTP). 

#### Possible Solutions
*  Please use SRTP. See more on [Secure Trunking](/docs/api/sip-trunking/getting-started#securetrunking)."#,
            TwilioElasticSipTrunkingError::ErrorCode32219 => r#"## ERROR - 32219

### SIP: Redirect failed

 Twilio could not process the SIP 30x response from your SIP server.


#### Possible Causes
*    Max redirect limit reached.
*    Redirection is not allowed in the call to the registered SIP endpoint.
*    Redirect target in the Contact header of the SIP 30x response is not a valid SIP URI.
*    Redirect target is a private IP address.
*    Redirect target is a Twilio domain such as \*.sip.twilio.com or \*.pstn.twilio.com.
*    Redirect target is an IP in a deny list.

#### Possible Solutions
Review the Possible Causes and update the redirect target as needed."#,
            TwilioElasticSipTrunkingError::ErrorCode21248 => r#"## ERROR - 21248

### Trunk Domain already taken

 

#### Possible Causes
*   This Termination URI (Domain) is not available.

#### Possible Solutions
*  Please review your Termination URI (Domain name) and select a different one."#,
            TwilioElasticSipTrunkingError::ErrorCode32202 => r#"## Error - 32202

###  SIP: Bad user credentials

#### Possible Causes 
*  There is a Credentials List on your trunk, and your INVITE’s Authentication Digest is incorrect due to wrong username/password

#### Possible Solutions
*  Confirm that your username/password matches one in your Credentials List. "#,
            TwilioElasticSipTrunkingError::ErrorCode32205 => r#"## ERROR - 32205

### SIP Trunking: Geo Permission configuration is not permitting call

 You attempted to initiate an outbound phone call to a phone number that is not enabled on your account.

#### Possible Causes
User dialed a destination your application is not enabled to support calling to.

#### Possible Solutions
Please check your [Voice Dialing Geographic Permissions](https://www.twilio.com/console/voice/calls/geo-permissions), fix it, and try again."#,
            TwilioElasticSipTrunkingError::ErrorCode21252 => r#"## ERROR - 21252

### Invalid Region

 

#### Possible Causes
*   The Region requested is not valid.

#### Possible Solutions
*  Please see [Localized Termination URIs](/docs/sip-trunking#localized-termination-uris) for more information on the valid Elastic SIP Trunking Regions."#,
            TwilioElasticSipTrunkingError::ErrorCode32207 => r#"## Error - 32207

###  SIP: Secure media not accepted

#### Possible Causes
*    A SIP Call is made to a Twilio SIP Domain using SRTP, which it is not supported
*    A termination Call is made to a Twilio SIP Trunk and the Twilio SIP Trunk is not configured to accept secure media (SRTP). 

#### Possible Solutions
*  Please use RTP.
*  For Twilio Elastic SIP Trunking, if you wish to use SRTP kindly enable [Secure Trunking](/docs/api/sip-trunking/getting-started#securetrunking).
"#,
            TwilioElasticSipTrunkingError::ErrorCode21249 => r#"## ERROR - 21249

### Maximum Origination URIs reached  

 

#### Possible Causes
*   You've configured the maximum number of Origination URIs allowed for your Trunk.

#### Possible Solutions
* Please see [Multiple Origination URIs](/docs/sip-trunking#multiple-orig-uris) for more information."#,
            TwilioElasticSipTrunkingError::ErrorCode21256 => r#"## ERROR - 21256

### Invalid ruleset

 The ruleset is invalid

#### Possible Causes
The json format of the ruleset does not match the spec or duplicate rule name.

#### Possible Solutions
Follow the json format for the ruleset found in the spec and use a valid ruleset."#,
            TwilioElasticSipTrunkingError::ErrorCode21259 => r#"## ERROR - 21259

### Maximum number of SIP Manipulation Polies per account reached

 Your account already has the maximum number of Sip Manipulation Policies allowed.

#### Possible Causes
The number of Sip Manipulation Policies per account cannot exceed the limit. See https://www.twilio.com/docs/sip-trunking/sip-header-manipulation 

#### Possible Solutions
Remove unused Sip Manipulation Policies."#,
            TwilioElasticSipTrunkingError::ErrorCode32218 => r#"## Error - 32218

### SIP: Transfer not allowed

#### Possible Causes 
*    Transfer enable mode is in DISABLE_ALL
*    Transferring to PSTN when transfer enable mode is in `SIP_ONLY`
*    Transferring to SIP when transfer enable mode is in `PSTN_ONLY`

#### Possible Solutions
*   There is a configurable transfer enable mode on your trunk. Change this mode to suit your needs"#,
            TwilioElasticSipTrunkingError::ErrorCode32001 => r#"## Error - 32001

### SIP: Trunk CPS limit exceeded

#### Possible Causes 
*  Calls Per Second placed against a SIP trunk exceeded its set limit. 

#### Possible Solutions
*  You've exceeded the number of calls per second you are authorized to make on this SIP Trunk. Visit [Elastic SIP Trunks](/user/account/sip-trunking/trunks) and within a specific Trunk, under the `Termination` tab you may see your CPS limits."#,
            TwilioElasticSipTrunkingError::ErrorCode32005 => r#"## ERROR - 32005

### Voice calling has been disabled for this account

 Voice calling has been disabled for this account. 

#### Possible Causes
Your account may have been flagged for review.

#### Possible Solutions
Please reach out to Support for assistance. "#,
            TwilioElasticSipTrunkingError::ErrorCode21247 => r#"## ERROR - 21247

### Trunk Dependencies

 

#### Possible Causes
*  Cannot create subdomain because parent domain does not exist/account does not own it
* Cannot delete trunk while it has subdomains
* Cannot rename subdomain because parent domain does not exist/account doesn't own it
* Cannot rename domain while it has subdomains

#### Possible Solutions
*  One of these constraints in the "Possible Causes" has been be violated, please review your Termination URI (Domain name)."#,
            TwilioElasticSipTrunkingError::ErrorCode32203 => r#"## ERROR - 32203

### SIP: Call blocked by Twilio

 The destination number is blocked by Twilio.

#### Possible Causes
The outbound call to the destination number was blocked by Twilio. Potential causes may be:
- The destination has a high-risk of fraud
- Due to regulatory reasons, the destination cannot be reached
- You are placing a call to a +1 destination and your account is missing a Primary Customer Profile

#### Possible Solutions
Potential solutions may be:
- If your destination is being incorrectly identified as high-risk of fraud and you have a legitimate need to call this number, please reach out to Twilio Support through the [Console](https://console.twilio.com) or [Help Center](https://support.twilio.com/hc/en-us).
- If you are placing a call to a +1 destination and your account is missing a Primary Customer Profile, create one in [TrustHub](https://console.twilio.com/us1/account/trust-hub/customer-profiles). "#,
            TwilioElasticSipTrunkingError::ErrorCode32100 => r#"You must verify your caller ID on the Console or via the REST API. You must use a verified callerID for both the from and to number while in Trial status. Once you upgrade you can use any caller ID for the from and to number.
You can verify you caller ID in the Console (https://www.twilio.com/console/phone-numbers/verified)"#,
            TwilioElasticSipTrunkingError::ErrorCode32012 => r#"## WARNING - 32012

### SIP: Parent account pooled Trunking CPS limit exceeded

 #### Possible Causes 
*  Calls Per Second placed on a SIP trunk exceeded the Parent Account pool limit. 

#### Possible Solutions
*  You've exceeded the number of calls per second you are authorized to make on your Parent Account. 

#### Possible Causes
*  Calls Per Second placed on a SIP trunk exceeded the Parent Account pool limit. 

#### Possible Solutions
*  You've exceeded the number of calls per second you are authorized to make on your Parent Account. "#,
            TwilioElasticSipTrunkingError::ErrorCode32011 => r#"## WARNING - 32011

### Error communicating with your SIP communications infrastructure

Twilio received a SIP error while communicating with your communications infrastructure specified in either your Trunking Origination URI(s), or Dial SIP URI. There was a problem communicating with a specific endpoint of your SIP communications infrastructure. This means there was either a lack of timely response, an error response or an invalid response from your SIP endpoint. This may result in increased call setup times or even failed call depending on the failover configuration for your Elastic SIP Trunk or SIP application. Twilio will make multiple attempts to deliver calls to your endpoint and each failed attempt will have its own notification.
The notification will have details about the specific error response and the SIP URI that causes the failure.

#### Possible Causes
* Your SIP endpoint is not reachable due to network connectivity issue between Twilio and your system
* Your SIP endpoint is not responding (service down or maintenance) 
* There is a firewall in your network that is blocking SIP traffic from Twilio
* Your SIP endpoint is sending an error response, such as SIP 500 response.
* The SIP URI specified in your Trunking Origination URI, <Dial> <SIP> TwiML or REST API call is invalid
* Your SIP endpoint does not support TLSv1.2

#### Possible Solutions
* Ensure the SIP URI used in the request is correctly configured.
 * For details on how to configure your Origination URIs for SIP trunking, please see [Origination settings](/docs/sip-trunking#origination).
 * For SIP interfaces verify that your SIP URI in your <Dial> <SIP> TwiML or REST API call, see [Receiving SIP from Twilio](/docs/voice/api/receiving-sip).
* Ensure you've allowed Twilio's IP address ranges and ports on your firewall for SIP signalling and RTP media traffic, see [IP addresses - Trunking](/docs/sip-trunking#ip-addresses) or [IP addresses - SIP Interface](/docs/voice/api/sip-interface#ip-addresses)
* Ensure that your SIP servers are running properly and is properly processing calls from Twilio.
* Ensure that your SIP servers support TLSv1.2, or configure your account to allow TLSv1.0+"#,
            TwilioElasticSipTrunkingError::ErrorCode32201 => r#"## Error - 32201

###  SIP: Source IP address not in ACL

#### Possible Causes 
*  Authorization failure - source IP Address not in ACL (Access Control List).

#### Possible Solutions
*  There is an ACL on your trunk and you are sending us INVITE requests from an IP address not on that ACL. Either fix local routing so that you are sending us SIP from an address already in your ACL or add this other address to your ACL. "#,
            TwilioElasticSipTrunkingError::ErrorCode21245 => r#"## ERROR - 21245

### Trunk Validation Error

 

#### Possible Causes
* Cannot use another account's SID as the Termination URI for this Trunk
* Cannot create more than one Trunk with the same Termination URI
* Credential List or IP-ACL already associated with Trunk

#### Possible Solutions
*  Please update your Trunk settings to avoid the conditions stated above. Learn more: [Configure Trunks](/docs/sip-trunking#configure-trunks)."#,
            TwilioElasticSipTrunkingError::ErrorCode32010 => r#"## ERROR - 32010

### SIP: No valid Origination URIs configured

 Your call can't be completed because no valid/active origination URLs could be found for your Elastic SIP Trunk or BYOC Trunk.

#### Possible Causes
Your Elastic SIP Trunk or BYOC Trunk does not have any valid active origination URLs

#### Possible Solutions
Please verify your Origination settings of your [Elastic SIP Trunk](/docs/sip-trunking#origination) or [BYOC Trunk](/docs/voice/bring-your-own-carrier-byoc#origination-connection-policy-settings)."#,
            TwilioElasticSipTrunkingError::ErrorCode32206 => r#"## ERROR - 32206

### SIP: Invalid From number (caller ID)

 You attempted to initiate an outbound phone call or message, but the 'From' parameter you supplied was not a valid phone number or alphanumeric sender ID.

Twilio accepts phone numbers in [E.164 format](https://www.twilio.com/docs/glossary/what-e164) (i.e. "+1 format"), 10-digit US and Canadian numbers with any combination of non-digit separators, or Alphanumeric Sender IDs (SMS only) with up to 11 alphanumeric characters [a-zA-Z0-9]. Refer to [this page](https://support.twilio.com/hc/en-us/articles/223133867-Using-Alphanumeric-Sender-ID-with-Messaging-Services) for acceptable characters.

The number must not be on a do-not-originate (DNO) list, and Alpha Sender IDs may not be generic.

#### Possible Causes
* You have supplied a phone number that was not in [E.164 format](https://www.twilio.com/docs/glossary/what-e164)
* Your `From` phone number is on a do-not-originate (DNO) list

#### Possible Solutions
* Ensure your number is formatted in [E.164 format](https://www.twilio.com/docs/glossary/what-e164)
* Ensure that your `From` number is assigned and not on a do-not-originate (DNO) list."#,
            TwilioElasticSipTrunkingError::ErrorCode21260 => r#"## ERROR - 21260

### Maximum number of actions per rule reached

 This rule already has the maximum number of actions allowed.

#### Possible Causes
The number of actions in a given rule cannot exceed the limit. See https://www.twilio.com/docs/sip-trunking/sip-header-manipulation 

#### Possible Solutions
Optimize your rule by removing unused actions."#,
            TwilioElasticSipTrunkingError::ErrorCode21261 => r#"## ERROR - 21261

### Maximum number of conditions per rule reached

 This rule already has the maximum number of conditions allowed.

#### Possible Causes
The number of conditions in a given rule cannot exceed the limit. See https://www.twilio.com/docs/sip-trunking/sip-header-manipulation 

#### Possible Solutions
Optimize your rule by removing unused conditions."#,
            TwilioElasticSipTrunkingError::ErrorCode32115 => r#"## ERROR - 32115

### X-Branded-CallReason header contains an invalid value.

 X-Branded-CallReason header contains an invalid value.

#### Possible Causes
X-Branded-CallReason header contains an invalid value exceeding 50 character limit. 

#### Possible Solutions
Verify that the call reason supplied does not exceed more than 50 characters"#,
            TwilioElasticSipTrunkingError::ErrorCode21258 => r#"## ERROR - 21258

### Invalid SIP Manipulation Policy

 There was a validation error for the Sip Manipulation Policy in question.

#### Possible Causes
Reference the docs for configuration information - https://www.twilio.com/docs/sip-trunking/sip-header-manipulation 

#### Possible Solutions
Reference the docs for a possible configuation solition - https://www.twilio.com/docs/sip-trunking/sip-header-manipulation"#,
            TwilioElasticSipTrunkingError::ErrorCode21634 => r#"## ERROR - 21634

### SIP Trunk is in use for emergency calling

One or more numbers associated with this SIP trunk are provisioned for emergency calling. Please disable emergency calling on these numbers before
proceeding.

#### Possible Causes
* One or more numbers associated with this SIP Trunk are provisioned for emergency calling.

#### Possible Solutions
* Refer to the docs on <a href="/docs/sip-trunking/emergency-calling">
Emergency Calling</a> over SIP trunking for more details."#,
            TwilioElasticSipTrunkingError::ErrorCode32222 => r#"## ERROR - 32222

### TLS version not allowed

 The TLS version used by your SIP endpoints is lower than the one configured for your account. It's highly recommended to upgrade your SIP endpoints to use TLSv1.2. Otherwise, you can opt in to use deprecated TLS versions in the console. By default, only TLSv1.2 is allowed.

#### Possible Causes
SIP/TLS version is lower than the allowed

#### Possible Solutions
Upgrade TLS version or configure your account to allow all TLSv1+ in console (Voice → Settings → General → Allow Deprecated SIP/TLS versions)"#,
            TwilioElasticSipTrunkingError::ErrorCode21254 => r#"## ERROR - 21254

### Max Connection Policy Entries Reached

A Connection Policy cannot have more than 10 entries. 

#### Possible Causes
* A Connection Policy cannot have more than 10 entries.

#### Possible Solutions
* Delete an existing entry from the Connection Policy if you must create more."#,
            TwilioElasticSipTrunkingError::ErrorCode21251 => r#"## ERROR - 21251

### Trunking CPS change not allowed

 

#### Possible Causes
*   This CPS change is not allowed.

#### Possible Solutions
*  This CPS change is not allowed, kindly contact Twilio Support through the [Console](https://console.twilio.com) or [Help Center](https://support.twilio.com)."#,
            TwilioElasticSipTrunkingError::ErrorCode32204 => r#"## ERROR - 32204

### SIP: 'From' phone number not verified

 You attempted to initiate an outbound phone call, but the <span class='rest-attribute'>From</span> number you specified is not a verified Outgoing Caller ID for your account.  In order to use a phone number as the Caller ID on outgoing calls, you must first validate your ownership of that phone number.

#### Possible Causes
The `From` number used to make outbound call not a Twilio number or not a verified caller ID

#### Possible Solutions
Change the callerID used in the From field of the SIP INVITE to a Twilio number on your account or verify your number by visiting the Console [Verified Caller IDs](/user/account/phone-numbers/verified) or REST API."#,
            TwilioElasticSipTrunkingError::ErrorCode32200 => r#"## Error - 32200

###  SIP: Insufficient permissions

#### Possible Causes 
*  Request failed due to insufficient permissions.

#### Possible Solutions
*  Please check that your account is active. Ensure that you have [Authentication details](/docs/api/sip-trunking/getting-started#authentication) configured on your Trunk."#,
            TwilioElasticSipTrunkingError::ErrorCode32020 => r#"## ERROR - 32020

### SHAKEN/STIR call verification failed due to invalid passport from customer

 Twilio cannot verify incoming SHAKEN PASSporT from customer

#### Possible Causes
See the error message for details

#### Possible Solutions
The call is rejected. Reattempt the call with a new passport."#
        }
    }
}
