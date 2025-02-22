// AUTO-GENERATED. DO NOT MODIFY. YOUR CHANGES WILL BE OVER-WRITTEN
use crate::products::TwilioSuperSimError;
use standard_error::traits::StandardErrorDescriptionTrait;

impl StandardErrorDescriptionTrait for TwilioSuperSimError {
    fn description(&self) -> Option<&'static str> {
        match self {
            TwilioSuperSimError::ErrorCode83602 => Some(r#"Certain requests for Usage Records can only be made with StartTime and EndTime parameters that align to a UTC hour cutoff (e.g. "2021-07-15T04:00:00Z")."#),
            TwilioSuperSimError::ErrorCode83603 => Some(r#"Requests for Usage Records grouping by "sim" can query a maximum time period of 31 days."#),
            TwilioSuperSimError::ErrorCode83003 => Some(r#"Raised whenever you try to register a Super SIM that is already registered to your Account."#),
            TwilioSuperSimError::ErrorCode83001 => Some(r#"Raised whenever a required parameter was not passed with the request while registering a Super SIM. The error object’s detail field will indicate which parameter was not provided."#),
            TwilioSuperSimError::ErrorCode83002 => Some(r#"The ICCID and Registration Code do not belong to a registrable Super SIM."#),
            TwilioSuperSimError::ErrorCode83007 => Some(r#"Your Super SIM cannot be activated without being assigned to a Fleet."#),
            TwilioSuperSimError::ErrorCode83605 => Some(r#"You can query usage data up to 18 months in the past. The oldest allowed StartTime is 18 months prior to the start of the current UTC day. For example, if the current time is 2021-12-13T5:00:00Z, then you can query usage as far back as 2020-06-13T:00:00:00Z."#),
            TwilioSuperSimError::ErrorCode83008 => Some(r#"Your Super SIM must be assigned to a Fleet while it is in status ‘ready’ or ‘active’."#),
            TwilioSuperSimError::ErrorCode83010 => Some(r#"Your Super SIM cannot be updated from its current status to the desired status."#),
            TwilioSuperSimError::ErrorCode83703 => Some(r#"The SIM was not able to attach to the specified Network because your device has repeatedly attempted to connect too frequently and is rate limited. Rate limiting is enforced to protect the network infrastracture and prevent one device from impacting the experience of other users on the Network."#),
            TwilioSuperSimError::ErrorCode83500 => Some(r#"Twilio currently does not have eSIM Profiles available for ordering."#),
            TwilioSuperSimError::ErrorCode83705 => Some(r#"The SIM was not able to attach to the specified Network because your SIM is `inactive`. A SIM must be in the `ready` or `active` states to access the Network."#),
            TwilioSuperSimError::ErrorCode83005 => Some(r#"Your Super SIM could not be found."#),
            TwilioSuperSimError::ErrorCode83700 => Some(r#"An unexpected error occurred while attempting to process your SIM's attachment request to the Network. This is usually a transient error and should resolve itself in time."#),
            TwilioSuperSimError::ErrorCode83011 => Some(r#"Your Super SIM cannot be updated with the provided Unique Name because another Super SIM on your Account is already using it."#),
            TwilioSuperSimError::ErrorCode83402 => Some(r#"Twilio received an error response when attempting to send an IP Command event to the provided callback URL."#),
            TwilioSuperSimError::ErrorCode83000 => Some(r#"Your Super SIM registration request has caused an internal error that is not further specified."#),
            TwilioSuperSimError::ErrorCode83702 => Some(r#"The SIM could not to attach to the specified Network, either because its Fleet's Network Access Profile did not allow the Network or because Twilio has blocked the IMSI currently being used from attaching to the Network."#),
            TwilioSuperSimError::ErrorCode83401 => Some(r#"The device was not attached to a cellular network when Twilio attempted to send the IP Command. The IP Command was not sent to the device."#),
            TwilioSuperSimError::ErrorCode83400 => Some(r#"Raised whenever delivering an IP Command fails with an error that is not covered by a more specific error code."#),
            TwilioSuperSimError::ErrorCode83004 => Some(r#"Your Super SIM update request has caused an internal error that is not further specified."#),
            TwilioSuperSimError::ErrorCode83604 => Some(r#"Requests for Usage Records with granularity "all" may only query up to 18 months of data, granularity "day" may query up to 3 months of data, and granularity "hour" may query up to 31 days of data."#),
            TwilioSuperSimError::ErrorCode83006 => Some(r#"Your Super SIM cannot be assigned to the target Fleet because the Fleet could not be found."#),
            TwilioSuperSimError::ErrorCode83600 => Some(r#"A parameter was passed to the Usage Records API which failed validation. This usually means the value format was invalid, or multiple parameters in the request conflict with each other."#),
            TwilioSuperSimError::ErrorCode83601 => Some(r#"Certain requests for Usage Records can only be made with StartTime and EndTime parameters that align to midnight UTC (e.g. "2021-07-15T00:00:00Z")."#),
            TwilioSuperSimError::ErrorCode83009 => Some(r#"Your Fleet update request cannot be performed on your Super SIM as it is in status scheduled."#),
            TwilioSuperSimError::ErrorCode83701 => Some(r#"An unexpected error occurred while establishing a Data Session for your SIM. This is usually a transient error and should resolve itself in time."#),
            TwilioSuperSimError::ErrorCode83704 => Some(r#"The SIM was not able to attach to the specified network because your SIM has not been activated yet and is in the `new` state. This is the state that SIMs arrive in when they are shipped and they must be activated first via Console or REST API before they can be used."#)
        }
    }
}
