// AUTO-GENERATED. DO NOT MODIFY. YOUR CHANGES WILL BE OVER-WRITTEN
use crate::products::TwilioProxyError;
use standard_error::traits::StandardErrorDescriptionTrait;

impl StandardErrorDescriptionTrait for TwilioProxyError {
    fn description(&self) -> Option<&'static str> {
        match self {
            TwilioProxyError::ErrorCode80612 => None,
            TwilioProxyError::ErrorCode80611 => Some(r#"The Proxy Number cannot be removed from Service. "#),
            TwilioProxyError::ErrorCode80911 => None,
            TwilioProxyError::ErrorCode80308 => Some(r#"Session with the given unique name not found in database."#),
            TwilioProxyError::ErrorCode80623 => Some(r#"Possible duplicate Participant created in multiple Sessions."#),
            TwilioProxyError::ErrorCode80621 => Some(r#"You have reached the limit on the number of requests for this endpoint"#),
            TwilioProxyError::ErrorCode80619 => None,
            TwilioProxyError::ErrorCode80607 => Some(r#"## Warning

### Session Closed

Example Message: Session is closed and cannot be updated"#),
            TwilioProxyError::ErrorCode80306 => None,
            TwilioProxyError::ErrorCode80904 => Some(r#"Proxy session has expired."#),
            TwilioProxyError::ErrorCode80504 => Some(r#"An internal server error has occurred."#),
            TwilioProxyError::ErrorCode80802 => Some(r#"Multiple requests to the create-participant endpoint with the same identifier were made simultaneously or very close in time.  This can result in the same proxy-identifier being assigned, causing Participants with the same Identifier/ProxyIdentifier pair to be active in multiple Sessions.  This can result in calls and messages from affected Participants being routed to unintended recipients."#),
            TwilioProxyError::ErrorCode80614 => None,
            TwilioProxyError::ErrorCode80208 => None,
            TwilioProxyError::ErrorCode80301 => Some(r#"Phone Number SID is not associated with the Account SID."#),
            TwilioProxyError::ErrorCode80101 => Some(r#"A phone number can be added in one service at a time."#),
            TwilioProxyError::ErrorCode80617 => None,
            TwilioProxyError::ErrorCode80206 => Some(r#"## Warning

### No Available Proxy

Example Message: This Service has no compatible Proxy numbers for this Participant. Failed to find a proxy number for +1415555XXXX. Either you have no numbers meeting your service's GeoMatchLevel for the target number, or the numbers you do have are already in use by the participant in other Sessions."#),
            TwilioProxyError::ErrorCode80913 => Some(r#"Callback to Out-of-Session handler returned a non-success status code."#),
            TwilioProxyError::ErrorCode80502 => None,
            TwilioProxyError::ErrorCode80624 => Some(r#"Proxy number pools can include a maximum of 5000 Reserved numbers and 500 un-reserved numbers.  We have detected that are getting close to one of those limits.  Once you reach the limit, you will not be able to add more numbers to your pool."#),
            TwilioProxyError::ErrorCode80622 => Some(r#"You attempted to add a number to a Proxy Number Pool that has reached its maximum allowed size.  A Proxy Service pool can have up to 500 unreserved numbers and 5000 reserved numbers."#),
            TwilioProxyError::ErrorCode80501 => None,
            TwilioProxyError::ErrorCode80618 => None,
            TwilioProxyError::ErrorCode80304 => None,
            TwilioProxyError::ErrorCode80901 => Some(r#"Message matched a stop word."#),
            TwilioProxyError::ErrorCode80505 => None,
            TwilioProxyError::ErrorCode80207 => Some(r#"Failed to find a proxy number."#),
            TwilioProxyError::ErrorCode80203 => Some(r#"This Service has no compatible Proxy numbers for this Participant. 

This Service has no available Proxy numbers for the participant's number country code <country>."#),
            TwilioProxyError::ErrorCode80201 => Some(r#"## Warning

### No Available Voice Proxy

Example Message: This Service has no compatible Proxy numbers for this Participant. This Service has no available Proxy numbers having voice capabilities. This can happen if you attempted to create a participant in a session for which you did not specify a Mode in a country that does not support combined voice and sms capabilities."#),
            TwilioProxyError::ErrorCode80613 => Some(r#"A downstream service rejected a request from Proxy while processing a request for your service."#),
            TwilioProxyError::ErrorCode80910 => None,
            TwilioProxyError::ErrorCode80305 => None,
            TwilioProxyError::ErrorCode80909 => Some(r#"Interaction not completed due to InterceptCallbackUrl."#),
            TwilioProxyError::ErrorCode80103 => Some(r#"The identifier is already a participant in the Session"#),
            TwilioProxyError::ErrorCode80616 => None,
            TwilioProxyError::ErrorCode80620 => None,
            TwilioProxyError::ErrorCode80801 => Some(r#"A request was made to re-open a Session that would result in one or both of the Participants being active in multiple Sessions.  This could result in calls or messages from the affected Participant being routed to an unintended recipient."#),
            TwilioProxyError::ErrorCode80506 => Some(r#"Blocking Proxy Service Creation API for new accounts/sub-accounts/projects as Proxy is in EOS."#),
            TwilioProxyError::ErrorCode80404 => None,
            TwilioProxyError::ErrorCode80608 => Some(r#"Session Status Invalid

"#),
            TwilioProxyError::ErrorCode80615 => None,
            TwilioProxyError::ErrorCode80625 => Some(r#"A valid signature header needs to be provided for the request to be authenticated."#)
        }
    }
}
