// AUTO-GENERATED. DO NOT MODIFY. YOUR CHANGES WILL BE OVER-WRITTEN
use crate::products::TwilioInterconnectError;
use standard_error::traits::StandardErrorDescriptionTrait;

impl StandardErrorDescriptionTrait for TwilioInterconnectError {
    fn description(&self) -> Option<&'static str> {
        match self {
            TwilioInterconnectError::ErrorCode62034 => None,
            TwilioInterconnectError::ErrorCode62015 => None,
            TwilioInterconnectError::ErrorCode32303 => None,
            TwilioInterconnectError::ErrorCode32301 => None,
            TwilioInterconnectError::ErrorCode62028 => None,
            TwilioInterconnectError::ErrorCode62200 => Some(r#"Network provisioning calls are traced by network API which at the moment are not available."#),
            TwilioInterconnectError::ErrorCode62001 => None,
            TwilioInterconnectError::ErrorCode62005 => None,
            TwilioInterconnectError::ErrorCode62017 => None,
            TwilioInterconnectError::ErrorCode62006 => None,
            TwilioInterconnectError::ErrorCode62003 => None,
            TwilioInterconnectError::ErrorCode62025 => None,
            TwilioInterconnectError::ErrorCode62012 => None,
            TwilioInterconnectError::ErrorCode62053 => None,
            TwilioInterconnectError::ErrorCode62020 => None,
            TwilioInterconnectError::ErrorCode62021 => None,
            TwilioInterconnectError::ErrorCode62009 => None,
            TwilioInterconnectError::ErrorCode62024 => None,
            TwilioInterconnectError::ErrorCode62027 => None,
            TwilioInterconnectError::ErrorCode62008 => None,
            TwilioInterconnectError::ErrorCode62100 => Some(r#"The IP address is already associated with another Interconnect connection."#),
            TwilioInterconnectError::ErrorCode62007 => None,
            TwilioInterconnectError::ErrorCode62019 => None,
            TwilioInterconnectError::ErrorCode32304 => None,
            TwilioInterconnectError::ErrorCode62014 => None,
            TwilioInterconnectError::ErrorCode62010 => None,
            TwilioInterconnectError::ErrorCode62002 => None,
            TwilioInterconnectError::ErrorCode62004 => None,
            TwilioInterconnectError::ErrorCode62026 => None,
            TwilioInterconnectError::ErrorCode62023 => None,
            TwilioInterconnectError::ErrorCode62018 => None,
            TwilioInterconnectError::ErrorCode62000 => None,
            TwilioInterconnectError::ErrorCode62013 => None,
            TwilioInterconnectError::ErrorCode62035 => None,
            TwilioInterconnectError::ErrorCode62022 => None,
            TwilioInterconnectError::ErrorCode62016 => None,
            TwilioInterconnectError::ErrorCode62052 => None,
            TwilioInterconnectError::ErrorCode62011 => None,
            TwilioInterconnectError::ErrorCode32302 => None,
            TwilioInterconnectError::ErrorCode62220 => Some(r#"Bandwidth is not available on the network device due to lack of capacity"#)
        }
    }
}
