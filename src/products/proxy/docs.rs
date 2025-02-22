// AUTO-GENERATED. DO NOT MODIFY. YOUR CHANGES WILL BE OVER-WRITTEN
use crate::products::TwilioProxyError;
use standard_error::traits::StandardErrorDocsTrait;

impl StandardErrorDocsTrait for TwilioProxyError {
    fn docs(&self) -> &'static str {
        match self {
            TwilioProxyError::ErrorCode80612 => r#"## ERROR - 80612

### Duplicate Entry

 

#### Possible Causes
You tried to add a participant with an identifier that already exists, or existed for a participant that was deleted.

#### Possible Solutions
You may not add a participant with the same identifier as a previously removed participant."#,
            TwilioProxyError::ErrorCode80611 => r#"## ERROR - 80611

### Proxy Number In Active Sessions

 The Proxy Number cannot be removed from Service. 

#### Possible Causes
It is in use by one or more Sessions.

#### Possible Solutions
The number can be removed from the Service once it's not in use."#,
            TwilioProxyError::ErrorCode80911 => r#"## Warning

### Call To Message Only Session Rejected

Example Message: Inbound call to message-only session rejected."#,
            TwilioProxyError::ErrorCode80308 => r#"## ERROR - 80308

### Session with the unique name not found.

 Session with the given unique name not found in database.

#### Possible Causes
Wrong unique name.

#### Possible Solutions
Retry operation with the correct unique name."#,
            TwilioProxyError::ErrorCode80623 => r#"## ERROR - 80623

### Duplicate Participant Request

Possible duplicate Participant created in multiple Sessions. Possible duplicate Participant created in multiple Sessions.

#### Possible Causes
A request to create a Participant for an Identifier was made while another ongoing request was in progress for the same Identifier in a different Session.  This can result in the same proxy identifier being assigned in multiple sessions for the same Identifier, causing incorrect behavior.

#### Possible Solutions
If you need to add the same person to multiple Sessions, ensure that the Participant Create requests are made one at a time.  To allow Proxy to enforce this, please pass the ReturnErrorOnDuplicateRequest=true parameter in your Participant Create requests and/or Session Create requests (if participant parameters are included).  This will be the default behavior for all requests in the future, and will enable us to use more efficient logic to find ProxyIdentifiers for your participants, thus reducing Participant Create latency."#,
            TwilioProxyError::ErrorCode80621 => r#"## WARNING - 80621

### Requests Rate Limited on Endpoint

 You have reached the limit on the number of requests for this endpoint

#### Possible Causes
There is high increase of number of requests for this endpoint within a short span of time.

#### Possible Solutions
You can retry the requests with a random exponential backoff or can follow any of the other techniques to avoid rate limiting."#,
            TwilioProxyError::ErrorCode80619 => r#"## Error - 80619

### Chat Channel Attribute Error

Chat Channel was not found or did not have expected attributes (twilioNumber, serviceNumber, status, proxySession) required for Proxy integration.  If you are using the same Chat service for both Flex and non-Flex use-cases, this is expected for non-flex messages and you can probably ignore this event.
"#,
            TwilioProxyError::ErrorCode80607 => r#"## WARNING - 80607

### Session Closed

Session is closed and cannot be updated ## Warning

### Session Closed

Example Message: Session is closed and cannot be updated

#### Possible Causes
You tried to update a Session that is closed.

#### Possible Solutions
None"#,
            TwilioProxyError::ErrorCode80306 => r#"## Warning

### Not Found Chat Service

Found no chat instance associated with service {ProxyServiceSid}"#,
            TwilioProxyError::ErrorCode80904 => r#"## WARNING - 80904

### Expired Proxy Session

  Proxy session has expired.

#### Possible Causes
* Session has expired due to **"Ttl"** or **"DateExpiry"** has been reached. 
    * If Session has DateExpiry property set, it overrides the Ttl value and session will expire as soon as DateExpiry is reached.
    * If Both the properties are not set, default Ttl is set to 90 days and session expires when Ttl is reached. 

#### Possible Solutions
Create a new session or re-open a closed session within the service. Refer: [docs](https://www.twilio.com/docs/proxy/api/session)"#,
            TwilioProxyError::ErrorCode80504 => r#"## ERROR - 80504

### An internal server error has occurred.

 An internal server error has occurred.

#### Possible Causes
An internal server error has occurred.

#### Possible Solutions
No action required."#,
            TwilioProxyError::ErrorCode80802 => r#"## ERROR - 80802

### Simultaneous requests to create the same Identifier in one or more Sessions

Possible duplicate Participant created in multiple Sessions. Multiple requests to the create-participant endpoint with the same identifier were made simultaneously or very close in time.  This can result in the same proxy-identifier being assigned, causing Participants with the same Identifier/ProxyIdentifier pair to be active in multiple Sessions.  This can result in calls and messages from affected Participants being routed to unintended recipients.

#### Possible Causes
Multiple requests to the create-participant endpoint with the same identifier were made simultaneously or very close in time. 

#### Possible Solutions
1. If you are trying to add the same person to multiple Sessions, make the requests sequentially.

2. Request that the ProxyAllowParticipantConflict flag be removed from your account.  Proxy will reject requests that could result in having multiple Participants in active sessions with the same Identifier/ProxyIdentifier pair.  In this case, Session create or Participant create, Proxy will return a 409 status code.  You can retry the failed request after the previous (conflicting) request has completed.  However, in order to opt-in to this behavior, the ProxyAllowParticipantConflict flag on your account must be removed*.  See option 3 for details on how to opt-in to this behavior on a per-request basis if you wish to do some testing first.

3. Include the form parameter FailOnParticipantConflict=True in your requests to Participant create (and/or Session create, if you include participant parameters) to allow Proxy to reject the request if it would result in this condition.  Proxy will return a 409 (error code 80623) in this case.

*Note that this flag also impacts the behavior of the Session update endpoint, on attempting to re-open closed sessions, when those requests would result in the condition described.  This endpoint would return 400 (error code 80604)."#,
            TwilioProxyError::ErrorCode80614 => r#"## Warning

### No Partner Participant Found

Example Message: Incoming message or call found no target participant in session <SessionSid> for participant <ParticipantSid>"#,
            TwilioProxyError::ErrorCode80208 => r#"## Warning

### No Available Unused Proxy

This Service has no compatible Proxy numbers for this Participant. Failed to find a proxy number for {identifier}. All matching proxies for this Participant are already in use in other Sessions."#,
            TwilioProxyError::ErrorCode80301 => r#"## ERROR - 80301

### Not Found Phone Number SID

 Phone Number SID is not associated with the Account SID.

#### Possible Causes
The Account SID doesn't own the Phone Number SID.

#### Possible Solutions
The Phone Number SID should be owned by the Account SID."#,
            TwilioProxyError::ErrorCode80101 => r#"## ERROR - 80101

### Number Already Added to Another Service

 A phone number can be added in one service at a time.

#### Possible Causes
Phone Number has already been added to another Service.

#### Possible Solutions
Add a different phone number or remove the number from the relevant Proxy Service. "#,
            TwilioProxyError::ErrorCode80617 => r#"## Error - 80617 

### Flex Configuration Error

A Flex component was not configured correctly.

#### Possible Causes 
* You made changes to one of your Flex phone numbers
* You are trying to use a phone number with Flex but have not created a Flex Flow for that number
* You manually deleted a Chat Channel or Proxy Session
* You removed or changed the Chat Channel Sid setting in your Proxy Service
"#,
            TwilioProxyError::ErrorCode80206 => r#"## WARNING - 80206

### No Available Proxy

This Service has no compatible Proxy numbers for this Participant. Failed to find a proxy number for +1415555XXXX. Either you have no numbers meeting your service's GeoMatchLevel for the target number, or the numbers you do have are already in use by the participant in other Sessions. ## Warning

### No Available Proxy

Example Message: This Service has no compatible Proxy numbers for this Participant. Failed to find a proxy number for +1415555XXXX. Either you have no numbers meeting your service's GeoMatchLevel for the target number, or the numbers you do have are already in use by the participant in other Sessions.

#### Possible Causes
This Service has no compatible Proxy numbers for this Participant. Failed to find a proxy number for <phone-number>. Either you have no numbers meeting your service's GeoMatchLevel for the target number, or the numbers you do have are already in use by the participant in other Sessions.

#### Possible Solutions
1. Add one or more numbers to your pool matching this Participant's area code
2. If this Participant is in an active Session, close that Session to make the number available to them."#,
            TwilioProxyError::ErrorCode80913 => r#"## ERROR - 80913

### Out-Of-Session Callback Error

 Callback to Out-of-Session handler returned a non-success status code.

#### Possible Causes
* Your Out-of-Session callback URL is invalid.

#### Possible Solutions
* Verify that your Out-of-Session callback URL is valid and that your server is correctly processing Out-of-Session callbacks from Proxy.
* Please read over the [Out-Of-Session Callback guide] (https://www.twilio.com/docs/proxy/out-session-callback-response-guide) for more detailed information."#,
            TwilioProxyError::ErrorCode80502 => r#"## Error

### Internal Server Error from Downstream

A downstream server failed or refused to process a request."#,
            TwilioProxyError::ErrorCode80624 => r#"## WARNING - 80624

### Approaching Maximium Number Pool Size

Your Proxy number pool has reached 85% of the maximum limit.   Proxy number pools can include a maximum of 5000 Reserved numbers and 500 un-reserved numbers.  We have detected that are getting close to one of those limits.  Once you reach the limit, you will not be able to add more numbers to your pool.

#### Possible Causes
You are nearing one of the limits in your Proxy number pool.

#### Possible Solutions
We strongly suggest that you shard numbers across multiple Proxy services.  If you need more numbers than are specified by these limits, consider creating another Proxy Service for the additional numbers."#,
            TwilioProxyError::ErrorCode80622 => r#"## ERROR - 80622

### Maximum Pool Size Error

 You attempted to add a number to a Proxy Number Pool that has reached its maximum allowed size.  A Proxy Service pool can have up to 500 unreserved numbers and 5000 reserved numbers.

#### Possible Causes
You attempted to add a number to a Proxy Number Pool that has reached its maximum allowed size.  A Proxy Service pool can have up to 500 unreserved numbers and 5000 reserved numbers.

#### Possible Solutions
Remove unused numbers, or create a new Proxy Service with additional numbers."#,
            TwilioProxyError::ErrorCode80501 => r#"## Error

### Storage Operation Failed

An internal server error has occurred during a storage operation"#,
            TwilioProxyError::ErrorCode80618 => r#"## Error - 80618

### Chat Integration Error

A Chat component was not configured correctly for integration with Proxy Service

#### Possible Causes 
* You manually deleted a Chat Channel or Proxy Session
* You removed or changed the Chat Channel Sid setting in your Proxy Service
"#,
            TwilioProxyError::ErrorCode80304 => r#"## Warning

### Not Found Unmanaged Identifier

Unmanaged Proxy Identifier not found.  Unmanaged identifiers include non-phone number identifiers (e.g., a facebook messenger page id). Possibly you tried to add a participant with a proxy identifier for a channel that is not managed by Twilio or by your Proxy Service."#,
            TwilioProxyError::ErrorCode80901 => r#"## WARNING - 80901

### Message Matched Stop Word

 Message matched a stop word.

#### Possible Causes
Message matched a STOP or START word, not proxying. See our [help](https://support.twilio.com/hc/en-us/articles/223134027-Twilio-support-for-opt-out-keywords-SMS-STOP-filtering-) pages for more information. 

#### Possible Solutions
Send a message with START, to resubscribe and continue the interactions. "#,
            TwilioProxyError::ErrorCode80505 => r#"## Error

### Flex Configuration Error

A Flex component was not configured correctly.  Please reach out to Flex support."#,
            TwilioProxyError::ErrorCode80207 => r#"## ERROR - 80207

###  No unreserved numbers in proxy pool.

 Failed to find a proxy number.

#### Possible Causes
The only matching candidate number(s) in your pool are marked as Reserved.

#### Possible Solutions
Verify there is at least one unreserved number in your proxy pool."#,
            TwilioProxyError::ErrorCode80203 => r#"## ERROR - 80203

### No Available Proxy For Country

 This Service has no compatible Proxy numbers for this Participant. 

This Service has no available Proxy numbers for the participant's number country code <country>.

#### Possible Causes
There are no numbers in the Service that can be assigned to this participant's identifier.

#### Possible Solutions
Add a Twilio number from your account to the Service that matches the participant's country code. Alternatively, pass the proxyIdentifier's optional property when creating the participant. Please note that international tolls may apply."#,
            TwilioProxyError::ErrorCode80201 => r#"## WARNING - 80201

### No Available Voice Proxy

This Service has no compatible Proxy numbers for this Participant. This Service has no available Proxy numbers having voice capabilities. This can happen if you attempted to create a participant in a session for which you did not specify a Mode in a country that does not support combined voice and sms capabilities. ## Warning

### No Available Voice Proxy

Example Message: This Service has no compatible Proxy numbers for this Participant. This Service has no available Proxy numbers having voice capabilities. This can happen if you attempted to create a participant in a session for which you did not specify a Mode in a country that does not support combined voice and sms capabilities.

#### Possible Causes
You tried to add a Participant to a Session with a mode that includes voice, but your proxy pool does not include any voice-capable numbers.

#### Possible Solutions
Add one or more numbers to your pool that have voice capability."#,
            TwilioProxyError::ErrorCode80613 => r#"## ERROR - 80613

### Downstream Request Rejected

A downstream service rejected a request from Proxy on your behalf. A downstream service rejected a request from Proxy while processing a request for your service.

#### Possible Causes
Unknown.  You may be able to find more information if the downstream emitted any debugger notifications for the request.

#### Possible Solutions
Unknown.  You may be able to find more information if the downstream emitted any debugger notifications for the request."#,
            TwilioProxyError::ErrorCode80910 => r#"## Warning

### Message To Voice Only Session Rejected

Example Message: Inbound message to voice-only session rejected."#,
            TwilioProxyError::ErrorCode80305 => r#"## Error

### Not Found Unmanaged Identifier Sid

Example Message: Unmanaged Proxy Identifier not found.  Unmanaged identifiers include non-phone number identifiers (e.g., a facebook messenger page id). If you want to include channels such as Facebook Messenger, WhatsApp, etc. as Proxy participants, you must configure the channel integration callback url to point to your proxy service: https://webhooks.twilio.com/v1/Accounts/{AccountSid}/Proxy/{ServiceSid}/Webhooks/Message"#,
            TwilioProxyError::ErrorCode80909 => r#"## WARNING - 80909

### Inbound Contact Rejected

 Interaction not completed due to InterceptCallbackUrl.

#### Possible Causes
A request to InterceptCallbackUrl received 403 and the interaction was aborted/blocked.

#### Possible Solutions
If this was unintended, the logic in the InterceptCallbackUrl could be revised. See behavior in the webhooks [documentation](https://www.twilio.com/docs/proxy/api/webhooks)."#,
            TwilioProxyError::ErrorCode80103 => r#"## ERROR - 80103

### Participant Already In Session

 The identifier is already a participant in the Session

#### Possible Causes
Participant has already been added to the Session

#### Possible Solutions
Create a new Session to add this participant, or add a new participant in the Session."#,
            TwilioProxyError::ErrorCode80616 => r#"## Error

### Unsupported Identifier Type For Session Mode

Cannot add <Identifier> to session <SessionSid>. The identifier is not compatible with the session mode."#,
            TwilioProxyError::ErrorCode80620 => r#"## Error - 80620

### Chat Configured Proxy Identifier Not Found

Received a chat message for a proxy identifier that does not exist in your Proxy pool

#### Possible Causes
* You removed a Flex number from your Proxy pool "#,
            TwilioProxyError::ErrorCode80801 => r#"## ERROR - 80801

### Invalid attempt to Re-Open a Session

Participant is active in another Session A request was made to re-open a Session that would result in one or both of the Participants being active in multiple Sessions.  This could result in calls or messages from the affected Participant being routed to an unintended recipient.

#### Possible Causes
Multiple Sessions in your service have a Participant with the same identifier/proxy-identifier pair, and one of those was active at the time a request was made to re-open one of the closed sessions.

#### Possible Solutions
1. Avoid re-opening sessions unless you can be certain that the Participants have not been assigned the same ProxyIdentifier in other sessions.

2. Ask that the ProxyAllowParticipantConflict flag be removed from your account.  By default, Proxy now rejects requests such as this one that would result in having two participants in active sessions with the same proxy identifier.  However, this new behavior requires removing the ProxyAllowParticipantConflict flag from your account.  When you are prepared to handle 400 on Session update requests (as well as 409 from Session create and Participant create endpoints), you can request to have this flag removed.

3. Include the experimental form parameter FailOnParticipantConflict=true in your Session create/update and Participant create requests to allow Proxy to reject the individual request if it would result in this condition.  This will allow you to test your code before choosing to remove the ProxyAllowParticipantConflict flag described above."#,
            TwilioProxyError::ErrorCode80506 => r#"## ERROR - 80506

### Service Creation is restricted for new customers

 Blocking Proxy Service Creation API for new accounts/sub-accounts/projects as Proxy is in EOS.

#### Possible Causes
Proxy is in EOS and hence, we are blocking the service creation request for new accounts/sub-accounts/projects.

#### Possible Solutions
If required, ask for an exempt for your account"#,
            TwilioProxyError::ErrorCode80404 => r#"## ERROR - 80404

### Participant Identifier Invalid

Participant identifier provided does not appear to be a valid, reachable phone number. 

#### Possible Causes
The participant's number passed in the participant creation is incorrect.

#### Possible Solutions
Pass a valid participant's number in [E.164 format](https://www.twilio.com/docs/glossary/what-e164)."#,
            TwilioProxyError::ErrorCode80608 => r#"## ERROR - 80608

### Session Status Invalid

Session status change not supported Session Status Invalid



#### Possible Causes
Session status change not supported. 

#### Possible Solutions
To re-open a session the valid status update is from `closed` to `in-progress`. To close a session, update from `open` to `closed` or from `in-progress` to `closed`. 
"#,
            TwilioProxyError::ErrorCode80615 => r#"## Error

### Account Sid on Legal Hold

Account <AccountSid> is on legal hold.  Cannot delete record <Sid>."#,
            TwilioProxyError::ErrorCode80625 => r#"## ERROR - 80625

### Unauthorized Request. Signature is missing or is invalid

 A valid signature header needs to be provided for the request to be authenticated.

#### Possible Causes
Header - 'X-Twilio-Signature' is missing or is invalid

#### Possible Solutions
Provide a valid signature value in header 'X-Twilio-Signature'"#
        }
    }
}
