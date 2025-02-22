// AUTO-GENERATED. DO NOT MODIFY. YOUR CHANGES WILL BE OVER-WRITTEN
use crate::products::TwilioProxyError;
use standard_error::traits::StandardErrorSolutionsTrait;

impl StandardErrorSolutionsTrait for TwilioProxyError {
    fn solutions(&self) -> Option<&'static str> {
        match self {
            TwilioProxyError::ErrorCode80612 => Some(r#"You may not add a participant with the same identifier as a previously removed participant."#),
            TwilioProxyError::ErrorCode80611 => Some(r#"The number can be removed from the Service once it's not in use."#),
            TwilioProxyError::ErrorCode80911 => None,
            TwilioProxyError::ErrorCode80308 => Some(r#"Retry operation with the correct unique name."#),
            TwilioProxyError::ErrorCode80623 => Some(r#"If you need to add the same person to multiple Sessions, ensure that the Participant Create requests are made one at a time.  To allow Proxy to enforce this, please pass the ReturnErrorOnDuplicateRequest=true parameter in your Participant Create requests and/or Session Create requests (if participant parameters are included).  This will be the default behavior for all requests in the future, and will enable us to use more efficient logic to find ProxyIdentifiers for your participants, thus reducing Participant Create latency."#),
            TwilioProxyError::ErrorCode80621 => Some(r#"You can retry the requests with a random exponential backoff or can follow any of the other techniques to avoid rate limiting."#),
            TwilioProxyError::ErrorCode80619 => None,
            TwilioProxyError::ErrorCode80607 => Some(r#"None"#),
            TwilioProxyError::ErrorCode80306 => None,
            TwilioProxyError::ErrorCode80904 => Some(r#"Create a new session or re-open a closed session within the service. Refer: [docs](https://www.twilio.com/docs/proxy/api/session)"#),
            TwilioProxyError::ErrorCode80504 => Some(r#"No action required."#),
            TwilioProxyError::ErrorCode80802 => Some(r#"1. If you are trying to add the same person to multiple Sessions, make the requests sequentially.

2. Request that the ProxyAllowParticipantConflict flag be removed from your account.  Proxy will reject requests that could result in having multiple Participants in active sessions with the same Identifier/ProxyIdentifier pair.  In this case, Session create or Participant create, Proxy will return a 409 status code.  You can retry the failed request after the previous (conflicting) request has completed.  However, in order to opt-in to this behavior, the ProxyAllowParticipantConflict flag on your account must be removed*.  See option 3 for details on how to opt-in to this behavior on a per-request basis if you wish to do some testing first.

3. Include the form parameter FailOnParticipantConflict=True in your requests to Participant create (and/or Session create, if you include participant parameters) to allow Proxy to reject the request if it would result in this condition.  Proxy will return a 409 (error code 80623) in this case.

*Note that this flag also impacts the behavior of the Session update endpoint, on attempting to re-open closed sessions, when those requests would result in the condition described.  This endpoint would return 400 (error code 80604)."#),
            TwilioProxyError::ErrorCode80614 => None,
            TwilioProxyError::ErrorCode80208 => None,
            TwilioProxyError::ErrorCode80301 => Some(r#"The Phone Number SID should be owned by the Account SID."#),
            TwilioProxyError::ErrorCode80101 => Some(r#"Add a different phone number or remove the number from the relevant Proxy Service. "#),
            TwilioProxyError::ErrorCode80617 => None,
            TwilioProxyError::ErrorCode80206 => Some(r#"1. Add one or more numbers to your pool matching this Participant's area code
2. If this Participant is in an active Session, close that Session to make the number available to them."#),
            TwilioProxyError::ErrorCode80913 => Some(r#"* Verify that your Out-of-Session callback URL is valid and that your server is correctly processing Out-of-Session callbacks from Proxy.
* Please read over the [Out-Of-Session Callback guide] (https://www.twilio.com/docs/proxy/out-session-callback-response-guide) for more detailed information."#),
            TwilioProxyError::ErrorCode80502 => None,
            TwilioProxyError::ErrorCode80624 => Some(r#"We strongly suggest that you shard numbers across multiple Proxy services.  If you need more numbers than are specified by these limits, consider creating another Proxy Service for the additional numbers."#),
            TwilioProxyError::ErrorCode80622 => Some(r#"Remove unused numbers, or create a new Proxy Service with additional numbers."#),
            TwilioProxyError::ErrorCode80501 => None,
            TwilioProxyError::ErrorCode80618 => None,
            TwilioProxyError::ErrorCode80304 => None,
            TwilioProxyError::ErrorCode80901 => Some(r#"Send a message with START, to resubscribe and continue the interactions. "#),
            TwilioProxyError::ErrorCode80505 => None,
            TwilioProxyError::ErrorCode80207 => Some(r#"Verify there is at least one unreserved number in your proxy pool."#),
            TwilioProxyError::ErrorCode80203 => Some(r#"Add a Twilio number from your account to the Service that matches the participant's country code. Alternatively, pass the proxyIdentifier's optional property when creating the participant. Please note that international tolls may apply."#),
            TwilioProxyError::ErrorCode80201 => Some(r#"Add one or more numbers to your pool that have voice capability."#),
            TwilioProxyError::ErrorCode80613 => Some(r#"Unknown.  You may be able to find more information if the downstream emitted any debugger notifications for the request."#),
            TwilioProxyError::ErrorCode80910 => None,
            TwilioProxyError::ErrorCode80305 => None,
            TwilioProxyError::ErrorCode80909 => Some(r#"If this was unintended, the logic in the InterceptCallbackUrl could be revised. See behavior in the webhooks [documentation](https://www.twilio.com/docs/proxy/api/webhooks)."#),
            TwilioProxyError::ErrorCode80103 => Some(r#"Create a new Session to add this participant, or add a new participant in the Session."#),
            TwilioProxyError::ErrorCode80616 => None,
            TwilioProxyError::ErrorCode80620 => None,
            TwilioProxyError::ErrorCode80801 => Some(r#"1. Avoid re-opening sessions unless you can be certain that the Participants have not been assigned the same ProxyIdentifier in other sessions.

2. Ask that the ProxyAllowParticipantConflict flag be removed from your account.  By default, Proxy now rejects requests such as this one that would result in having two participants in active sessions with the same proxy identifier.  However, this new behavior requires removing the ProxyAllowParticipantConflict flag from your account.  When you are prepared to handle 400 on Session update requests (as well as 409 from Session create and Participant create endpoints), you can request to have this flag removed.

3. Include the experimental form parameter FailOnParticipantConflict=true in your Session create/update and Participant create requests to allow Proxy to reject the individual request if it would result in this condition.  This will allow you to test your code before choosing to remove the ProxyAllowParticipantConflict flag described above."#),
            TwilioProxyError::ErrorCode80506 => Some(r#"If required, ask for an exempt for your account"#),
            TwilioProxyError::ErrorCode80404 => Some(r#"Pass a valid participant's number in [E.164 format](https://www.twilio.com/docs/glossary/what-e164)."#),
            TwilioProxyError::ErrorCode80608 => Some(r#"To re-open a session the valid status update is from `closed` to `in-progress`. To close a session, update from `open` to `closed` or from `in-progress` to `closed`. 
"#),
            TwilioProxyError::ErrorCode80615 => None,
            TwilioProxyError::ErrorCode80625 => Some(r#"Provide a valid signature value in header 'X-Twilio-Signature'"#)
        }
    }
}
