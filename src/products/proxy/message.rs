// AUTO-GENERATED. DO NOT MODIFY. YOUR CHANGES WILL BE OVER-WRITTEN
use crate::products::TwilioProxyError;
use standard_error::traits::StandardErrorMessageTrait;
use std::borrow::Cow;

impl StandardErrorMessageTrait for TwilioProxyError {
    fn message(&self) -> Cow<'static, str> {
        match self {
            TwilioProxyError::ErrorCode80612 => r#"Duplicate Entry"#.into(),
            TwilioProxyError::ErrorCode80611 => r#"Proxy Number In Active Sessions"#.into(),
            TwilioProxyError::ErrorCode80911 => r#"Call To Message Only Session Rejected"#.into(),
            TwilioProxyError::ErrorCode80308 => r#"Session with the unique name not found."#.into(),
            TwilioProxyError::ErrorCode80623 => r#"Duplicate Participant Request"#.into(),
            TwilioProxyError::ErrorCode80621 => r#"Requests Rate Limited on Endpoint"#.into(),
            TwilioProxyError::ErrorCode80619 => r#"Chat Channel Attribute Error"#.into(),
            TwilioProxyError::ErrorCode80607 => r#"Session Closed"#.into(),
            TwilioProxyError::ErrorCode80306 => r#"Not Found Chat Service"#.into(),
            TwilioProxyError::ErrorCode80904 => r#"Expired Proxy Session"#.into(),
            TwilioProxyError::ErrorCode80504 => r#"An internal server error has occurred."#.into(),
            TwilioProxyError::ErrorCode80802 => r#"Simultaneous requests to create the same Identifier in one or more Sessions"#.into(),
            TwilioProxyError::ErrorCode80614 => r#" No Partner Participant Found"#.into(),
            TwilioProxyError::ErrorCode80208 => r#"No Available Unused Proxy"#.into(),
            TwilioProxyError::ErrorCode80301 => r#"Not Found Phone Number SID"#.into(),
            TwilioProxyError::ErrorCode80101 => r#"Number Already Added to Another Service"#.into(),
            TwilioProxyError::ErrorCode80617 => r#"Flex Configuration Error"#.into(),
            TwilioProxyError::ErrorCode80206 => r#"No Available Proxy"#.into(),
            TwilioProxyError::ErrorCode80913 => r#"Out-Of-Session Callback Error"#.into(),
            TwilioProxyError::ErrorCode80502 => r#"Internal Server Error from Downstream"#.into(),
            TwilioProxyError::ErrorCode80624 => r#"Approaching Maximium Number Pool Size"#.into(),
            TwilioProxyError::ErrorCode80622 => r#"Maximum Pool Size Error"#.into(),
            TwilioProxyError::ErrorCode80501 => r#"Storage Operation Failed"#.into(),
            TwilioProxyError::ErrorCode80618 => r#"Chat Integration Error"#.into(),
            TwilioProxyError::ErrorCode80304 => r#"Not Found Unmanaged Identifier"#.into(),
            TwilioProxyError::ErrorCode80901 => r#"Message Matched Stop Word"#.into(),
            TwilioProxyError::ErrorCode80505 => r#"Flex Configuration Error"#.into(),
            TwilioProxyError::ErrorCode80207 => r#" No unreserved numbers in proxy pool."#.into(),
            TwilioProxyError::ErrorCode80203 => r#"No Available Proxy For Country"#.into(),
            TwilioProxyError::ErrorCode80201 => r#"No Available Voice Proxy"#.into(),
            TwilioProxyError::ErrorCode80613 => r#"Downstream Request Rejected"#.into(),
            TwilioProxyError::ErrorCode80910 => r#"Message To Voice Only Session Rejected"#.into(),
            TwilioProxyError::ErrorCode80305 => r#"Not Found Unmanaged Identifier Sid"#.into(),
            TwilioProxyError::ErrorCode80909 => r#"Inbound Contact Rejected"#.into(),
            TwilioProxyError::ErrorCode80103 => r#"Participant Already In Session"#.into(),
            TwilioProxyError::ErrorCode80616 => r#"Unsupported Identifier Type For Session Mode"#.into(),
            TwilioProxyError::ErrorCode80620 => r#"Chat Configured Proxy Identifier Not Found"#.into(),
            TwilioProxyError::ErrorCode80801 => r#"Invalid attempt to Re-Open a Session"#.into(),
            TwilioProxyError::ErrorCode80506 => r#"Service Creation is restricted for new customers"#.into(),
            TwilioProxyError::ErrorCode80404 => r#"Participant Identifier Invalid"#.into(),
            TwilioProxyError::ErrorCode80608 => r#"Session Status Invalid"#.into(),
            TwilioProxyError::ErrorCode80615 => r#"Account Sid on Legal Hold"#.into(),
            TwilioProxyError::ErrorCode80625 => r#"Unauthorized Request. Signature is missing or is invalid"#.into()
        }
    }
}
