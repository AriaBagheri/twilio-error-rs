// AUTO-GENERATED. DO NOT MODIFY. YOUR CHANGES WILL BE OVER-WRITTEN
use crate::products::TwilioSuperSimError;
use standard_error::traits::StandardErrorMessageTrait;
use std::borrow::Cow;

impl StandardErrorMessageTrait for TwilioSuperSimError {
    fn message(&self) -> Cow<'static, str> {
        match self {
            TwilioSuperSimError::ErrorCode83602 => r#"Request StartTime and/or EndTime must be aligned to UTC hour boundaries."#.into(),
            TwilioSuperSimError::ErrorCode83603 => r#"The maximum allowed query period is 31 days for group by sim queries"#.into(),
            TwilioSuperSimError::ErrorCode83003 => r#"The Super SIM already belongs to the requesting Account."#.into(),
            TwilioSuperSimError::ErrorCode83001 => r#"Parameter missing while registering a Super SIM"#.into(),
            TwilioSuperSimError::ErrorCode83002 => r#"Super SIM cannot be registered"#.into(),
            TwilioSuperSimError::ErrorCode83007 => r#"Unable to activate your Super SIM as it does not belong to a Fleet"#.into(),
            TwilioSuperSimError::ErrorCode83605 => r#"StartTime parameter is too far in the past. It must be within the last 18 months."#.into(),
            TwilioSuperSimError::ErrorCode83008 => r#"Unable to remove your Super SIM from its Fleet"#.into(),
            TwilioSuperSimError::ErrorCode83010 => r#"Unable to update your Super SIM to the desired status"#.into(),
            TwilioSuperSimError::ErrorCode83703 => r#"Attachment Rejected Due To Rate Limiting"#.into(),
            TwilioSuperSimError::ErrorCode83500 => r#"No eSIM Profiles are available"#.into(),
            TwilioSuperSimError::ErrorCode83705 => r#"Attachment Rejected Due To SIM In Inactive State"#.into(),
            TwilioSuperSimError::ErrorCode83005 => r#"Super SIM not found"#.into(),
            TwilioSuperSimError::ErrorCode83700 => r#"Attachment Failed Due To Internal Error"#.into(),
            TwilioSuperSimError::ErrorCode83011 => r#"A Super SIM with the specified Unique Name exists already"#.into(),
            TwilioSuperSimError::ErrorCode83402 => r#"Received error response to IP Command callback request"#.into(),
            TwilioSuperSimError::ErrorCode83000 => r#"Super SIM registration failed due to Internal Error"#.into(),
            TwilioSuperSimError::ErrorCode83702 => r#"Attachment Rejected Due To Network Not Allowed"#.into(),
            TwilioSuperSimError::ErrorCode83401 => r#"The device was not attached to a cellular network"#.into(),
            TwilioSuperSimError::ErrorCode83400 => r#"IP Commands error"#.into(),
            TwilioSuperSimError::ErrorCode83004 => r#"Super SIM update operation failed due to Internal Error"#.into(),
            TwilioSuperSimError::ErrorCode83604 => r#"The requested query period exceeds the maximum allowed period for the requested Granularity"#.into(),
            TwilioSuperSimError::ErrorCode83006 => r#"Super SIM’s Target Fleet not found"#.into(),
            TwilioSuperSimError::ErrorCode83600 => r#" An invalid parameter value was passed to the API"#.into(),
            TwilioSuperSimError::ErrorCode83601 => r#"Request StartTime and/or EndTime must be aligned to UTC day boundaries"#.into(),
            TwilioSuperSimError::ErrorCode83009 => r#"Unable to update your Super SIM’s Fleet while it is in status scheduled"#.into(),
            TwilioSuperSimError::ErrorCode83701 => r#"Data Session Establishment Failed Due To Internal Error"#.into(),
            TwilioSuperSimError::ErrorCode83704 => r#"Attachment Rejected Due To SIM In New State"#.into()
        }
    }
}
