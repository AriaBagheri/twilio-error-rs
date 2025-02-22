// AUTO-GENERATED. DO NOT MODIFY. YOUR CHANGES WILL BE OVER-WRITTEN
use crate::products::TwilioInterconnectError;
use standard_error::traits::StandardErrorMessageTrait;
use std::borrow::Cow;

impl StandardErrorMessageTrait for TwilioInterconnectError {
    fn message(&self) -> Cow<'static, str> {
        match self {
            TwilioInterconnectError::ErrorCode62034 => r#"Interconnect: No useful parameters provided."#.into(),
            TwilioInterconnectError::ErrorCode62015 => r#"Connection in transition"#.into(),
            TwilioInterconnectError::ErrorCode32303 => r#"Interconnect: Multiple SIP Dials with Interconnect Connection (TNX) SID"#.into(),
            TwilioInterconnectError::ErrorCode32301 => r#"Interconnect: Invalid Connection (TNX) SID"#.into(),
            TwilioInterconnectError::ErrorCode62028 => r#"Interconnect: Connection expired."#.into(),
            TwilioInterconnectError::ErrorCode62200 => r#"Provisioning failure - Network-API is unavailable!"#.into(),
            TwilioInterconnectError::ErrorCode62001 => r#"Invalid SID"#.into(),
            TwilioInterconnectError::ErrorCode62005 => r#" Bandwidth reserve not found."#.into(),
            TwilioInterconnectError::ErrorCode62017 => r#"No IP route specified"#.into(),
            TwilioInterconnectError::ErrorCode62006 => r#" MPLS carrier is not associated with the exchange."#.into(),
            TwilioInterconnectError::ErrorCode62003 => r#"MPLS carrier not found."#.into(),
            TwilioInterconnectError::ErrorCode62025 => r#"Interconnect: Invalid connection type"#.into(),
            TwilioInterconnectError::ErrorCode62012 => r#"Connection not found."#.into(),
            TwilioInterconnectError::ErrorCode62053 => r#"Interconnect: The subaccount is not authorized to access this connection.  "#.into(),
            TwilioInterconnectError::ErrorCode62020 => r#"Connection pending deletion."#.into(),
            TwilioInterconnectError::ErrorCode62021 => r#"IP Gateway Invalid"#.into(),
            TwilioInterconnectError::ErrorCode62009 => r#"Account SID was not found."#.into(),
            TwilioInterconnectError::ErrorCode62024 => r#"Missing connection type"#.into(),
            TwilioInterconnectError::ErrorCode62027 => r#"Interconnect: Extra MPLS parameter"#.into(),
            TwilioInterconnectError::ErrorCode62008 => r#"Bandwidth reserve is not associated with the exchange."#.into(),
            TwilioInterconnectError::ErrorCode62100 => r#"IP address(es) already linked to another connection"#.into(),
            TwilioInterconnectError::ErrorCode62007 => r#"Direct connect is not associated with the exchange."#.into(),
            TwilioInterconnectError::ErrorCode62019 => r#"IP route exceeds permitted IP range"#.into(),
            TwilioInterconnectError::ErrorCode32304 => r#"Interconnect: Connection (TNX) SID is not Active"#.into(),
            TwilioInterconnectError::ErrorCode62014 => r#"Connection not ready"#.into(),
            TwilioInterconnectError::ErrorCode62010 => r#"No authentication was provided."#.into(),
            TwilioInterconnectError::ErrorCode62002 => r#"Exchange not found"#.into(),
            TwilioInterconnectError::ErrorCode62004 => r#"Direct connect not found."#.into(),
            TwilioInterconnectError::ErrorCode62026 => r#"Interconnect: Missing MPLS carrier SID"#.into(),
            TwilioInterconnectError::ErrorCode62023 => r#"Missing exchange"#.into(),
            TwilioInterconnectError::ErrorCode62018 => r#"Invalid IP route"#.into(),
            TwilioInterconnectError::ErrorCode62000 => r#"Failed to write to the database."#.into(),
            TwilioInterconnectError::ErrorCode62013 => r#"Unable to identify account owner of connection."#.into(),
            TwilioInterconnectError::ErrorCode62035 => r#"Interconnect: No bandwidth was specified in the request"#.into(),
            TwilioInterconnectError::ErrorCode62022 => r#"Missing account SID"#.into(),
            TwilioInterconnectError::ErrorCode62016 => r#"Connection not active"#.into(),
            TwilioInterconnectError::ErrorCode62052 => r#"Interconnect: This account is not a subaccount of the Interconnect Connection owner's account."#.into(),
            TwilioInterconnectError::ErrorCode62011 => r#"Unauthorized"#.into(),
            TwilioInterconnectError::ErrorCode32302 => r#"Interconnect: Connection (TNX) SID not found"#.into(),
            TwilioInterconnectError::ErrorCode62220 => r#"Provisioning failure - Requested bandwidth not available on the network device."#.into()
        }
    }
}
