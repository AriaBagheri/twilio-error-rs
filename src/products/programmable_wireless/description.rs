// AUTO-GENERATED. DO NOT MODIFY. YOUR CHANGES WILL BE OVER-WRITTEN
use crate::products::TwilioProgrammableWirelessError;
use standard_error::traits::StandardErrorDescriptionTrait;

impl StandardErrorDescriptionTrait for TwilioProgrammableWirelessError {
    fn description(&self) -> Option<&'static str> {
        match self {
            TwilioProgrammableWirelessError::ErrorCode33010 => Some(r#"Raised whenever an update is requested while another update is already in progress.	"#),
            TwilioProgrammableWirelessError::ErrorCode33101 => Some(r#"Raised when an invalid parameter is provided. Please refer to the [API docs](https://www.twilio.com/docs/iot/wireless/api) for more details."#),
            TwilioProgrammableWirelessError::ErrorCode33004 => Some(r#"Raised whenever a service is temporarily unavailable."#),
            TwilioProgrammableWirelessError::ErrorCode33102 => Some(r#"Raised whenever a required parameter was not passed with the request. The error object’s detail field will indicate which parameter was not provided."#),
            TwilioProgrammableWirelessError::ErrorCode33119 => Some(r#"Raised whenever a SIM connectivity reset is requested but not permitted.
"#),
            TwilioProgrammableWirelessError::ErrorCode33108 => Some(r#"Raised whenever the given Rate Plan is not found.
"#),
            TwilioProgrammableWirelessError::ErrorCode33201 => Some(r#"Raised whenever the requested operation is not permitted.
"#),
            TwilioProgrammableWirelessError::ErrorCode33111 => Some(r#"Raised whenever a Command from your SIM exceeds the maximum length of 160 characters. As a result the Command is truncated but still delivered to the target Callback URL.
"#),
            TwilioProgrammableWirelessError::ErrorCode33107 => Some(r#"Raised whenever the given SIM is not found."#),
            TwilioProgrammableWirelessError::ErrorCode33000 => Some(r#"Your request has caused an error that is not further specified."#),
            TwilioProgrammableWirelessError::ErrorCode33105 => Some(r#"Raised whenever the SIM cannot be transitioned to the requested status.
"#),
            TwilioProgrammableWirelessError::ErrorCode33122 => None,
            TwilioProgrammableWirelessError::ErrorCode33103 => Some(r#"Raised whenever an invalid value for pagination while listing resources is provided."#),
            TwilioProgrammableWirelessError::ErrorCode33120 => Some(r#"Raised whenever a SIM connectivity reset is in progress while another SIM connectivity reset is requested.
"#),
            TwilioProgrammableWirelessError::ErrorCode33104 => Some(r#"Raised whenever the configuration for a given resource is incomplete.
"#),
            TwilioProgrammableWirelessError::ErrorCode33203 => Some(r#"Raised whenever a Command from your SIM is received but not permitted.
"#),
            TwilioProgrammableWirelessError::ErrorCode33121 => Some(r#"Raised whenever a request contains conflicting parameters.
"#),
            TwilioProgrammableWirelessError::ErrorCode33118 => Some(r#"Raised whenever there are too many Commands queued for an individual SIM.
"#)
        }
    }
}
