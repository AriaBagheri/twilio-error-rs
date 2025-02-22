// AUTO-GENERATED. DO NOT MODIFY. YOUR CHANGES WILL BE OVER-WRITTEN
use crate::products::TwilioProxyError;
use standard_error::traits::StandardErrorCausesTrait;

impl StandardErrorCausesTrait for TwilioProxyError {
    fn causes(&self) -> Option<&'static str> {
        match self {
            TwilioProxyError::ErrorCode80612 => Some(r#"You tried to add a participant with an identifier that already exists, or existed for a participant that was deleted."#),
            TwilioProxyError::ErrorCode80611 => Some(r#"It is in use by one or more Sessions."#),
            TwilioProxyError::ErrorCode80911 => None,
            TwilioProxyError::ErrorCode80308 => Some(r#"Wrong unique name."#),
            TwilioProxyError::ErrorCode80623 => Some(r#"A request to create a Participant for an Identifier was made while another ongoing request was in progress for the same Identifier in a different Session.  This can result in the same proxy identifier being assigned in multiple sessions for the same Identifier, causing incorrect behavior."#),
            TwilioProxyError::ErrorCode80621 => Some(r#"There is high increase of number of requests for this endpoint within a short span of time."#),
            TwilioProxyError::ErrorCode80619 => None,
            TwilioProxyError::ErrorCode80607 => Some(r#"You tried to update a Session that is closed."#),
            TwilioProxyError::ErrorCode80306 => None,
            TwilioProxyError::ErrorCode80904 => Some(r#"* Session has expired due to **"Ttl"** or **"DateExpiry"** has been reached. 
    * If Session has DateExpiry property set, it overrides the Ttl value and session will expire as soon as DateExpiry is reached.
    * If Both the properties are not set, default Ttl is set to 90 days and session expires when Ttl is reached. "#),
            TwilioProxyError::ErrorCode80504 => Some(r#"An internal server error has occurred."#),
            TwilioProxyError::ErrorCode80802 => Some(r#"Multiple requests to the create-participant endpoint with the same identifier were made simultaneously or very close in time. "#),
            TwilioProxyError::ErrorCode80614 => None,
            TwilioProxyError::ErrorCode80208 => None,
            TwilioProxyError::ErrorCode80301 => Some(r#"The Account SID doesn't own the Phone Number SID."#),
            TwilioProxyError::ErrorCode80101 => Some(r#"Phone Number has already been added to another Service."#),
            TwilioProxyError::ErrorCode80617 => None,
            TwilioProxyError::ErrorCode80206 => Some(r#"This Service has no compatible Proxy numbers for this Participant. Failed to find a proxy number for <phone-number>. Either you have no numbers meeting your service's GeoMatchLevel for the target number, or the numbers you do have are already in use by the participant in other Sessions."#),
            TwilioProxyError::ErrorCode80913 => Some(r#"* Your Out-of-Session callback URL is invalid."#),
            TwilioProxyError::ErrorCode80502 => None,
            TwilioProxyError::ErrorCode80624 => Some(r#"You are nearing one of the limits in your Proxy number pool."#),
            TwilioProxyError::ErrorCode80622 => Some(r#"You attempted to add a number to a Proxy Number Pool that has reached its maximum allowed size.  A Proxy Service pool can have up to 500 unreserved numbers and 5000 reserved numbers."#),
            TwilioProxyError::ErrorCode80501 => None,
            TwilioProxyError::ErrorCode80618 => None,
            TwilioProxyError::ErrorCode80304 => None,
            TwilioProxyError::ErrorCode80901 => Some(r#"Message matched a STOP or START word, not proxying. See our [help](https://support.twilio.com/hc/en-us/articles/223134027-Twilio-support-for-opt-out-keywords-SMS-STOP-filtering-) pages for more information. "#),
            TwilioProxyError::ErrorCode80505 => None,
            TwilioProxyError::ErrorCode80207 => Some(r#"The only matching candidate number(s) in your pool are marked as Reserved."#),
            TwilioProxyError::ErrorCode80203 => Some(r#"There are no numbers in the Service that can be assigned to this participant's identifier."#),
            TwilioProxyError::ErrorCode80201 => Some(r#"You tried to add a Participant to a Session with a mode that includes voice, but your proxy pool does not include any voice-capable numbers."#),
            TwilioProxyError::ErrorCode80613 => Some(r#"Unknown.  You may be able to find more information if the downstream emitted any debugger notifications for the request."#),
            TwilioProxyError::ErrorCode80910 => None,
            TwilioProxyError::ErrorCode80305 => None,
            TwilioProxyError::ErrorCode80909 => Some(r#"A request to InterceptCallbackUrl received 403 and the interaction was aborted/blocked."#),
            TwilioProxyError::ErrorCode80103 => Some(r#"Participant has already been added to the Session"#),
            TwilioProxyError::ErrorCode80616 => None,
            TwilioProxyError::ErrorCode80620 => None,
            TwilioProxyError::ErrorCode80801 => Some(r#"Multiple Sessions in your service have a Participant with the same identifier/proxy-identifier pair, and one of those was active at the time a request was made to re-open one of the closed sessions."#),
            TwilioProxyError::ErrorCode80506 => Some(r#"Proxy is in EOS and hence, we are blocking the service creation request for new accounts/sub-accounts/projects."#),
            TwilioProxyError::ErrorCode80404 => Some(r#"The participant's number passed in the participant creation is incorrect."#),
            TwilioProxyError::ErrorCode80608 => Some(r#"Session status change not supported. "#),
            TwilioProxyError::ErrorCode80615 => None,
            TwilioProxyError::ErrorCode80625 => Some(r#"Header - 'X-Twilio-Signature' is missing or is invalid"#)
        }
    }
}
