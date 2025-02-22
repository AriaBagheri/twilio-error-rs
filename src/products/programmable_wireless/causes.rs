// AUTO-GENERATED. DO NOT MODIFY. YOUR CHANGES WILL BE OVER-WRITTEN
use crate::products::TwilioProgrammableWirelessError;
use standard_error::traits::StandardErrorCausesTrait;

impl StandardErrorCausesTrait for TwilioProgrammableWirelessError {
    fn causes(&self) -> Option<&'static str> {
        match self {
            TwilioProgrammableWirelessError::ErrorCode33010 => Some(r#" * You cannot change a SIM's status or its Rate Plan while another update is already in progress."#),
            TwilioProgrammableWirelessError::ErrorCode33101 => Some(r#"* Your request contained invalid parameters. Please refer to the actual error response to get more details.
* While removing the Rate Plan of a SIM, your SIM must not be in status `active` or `ready`.
* While updating the Rate Plan of a SIM, you cannot move from NB-Annual to NB-developer.
* While updating the Rate Plan of a SIM, you can only move the Rate Plan to a different Data Limit Strategy while your SIM is `new` or `deactivated`.
* While updating the Rate Plan of a SIM, Narrowband SIMs cannot be paired with a non-narrowband enabled Rate Plan, or vice versa.
* While creating or updating a Rate Plan, `UniqueName` is invalid.
* While creating a Rate Plan, the limits (e.g. `DataLimit`, `NationalRoamingDataLimit`, `InternationalRoamingDataLimit`) you defined are out of range."#),
            TwilioProgrammableWirelessError::ErrorCode33004 => Some(r#"The requested service or any of its downstreams is temporarily unavailable."#),
            TwilioProgrammableWirelessError::ErrorCode33102 => Some(r#" * While sending a Command, you did not indicate the `Sim` to send it to.
 * While transferring a SIM to another Account, you need to provide a new Rate Plan present in the target Account or remove the current Rate Plan if the SIM to move is currently assigned to a Rate Plan.
 * While transferring a SIM to another Account, you cannot transfer a SIM in status `ready`.
"#),
            TwilioProgrammableWirelessError::ErrorCode33119 => Some(r#" * This operation is temporarily unavailable because the downstream carrier is undergoing maintenance and not accepting requests.
 * Your SIM either does not support this operation or is not in a status where it can connect to a cellular network.
"#),
            TwilioProgrammableWirelessError::ErrorCode33108 => Some(r#" * While updating a SIM's Rate Plan, your request was rejected because the Rate Plan was not found.
 * While transferring a SIM to another Account, your request was rejected because the given Rate Plan was not found in the target Account.
"#),
            TwilioProgrammableWirelessError::ErrorCode33201 => Some(r#" * Your Account has been suspended and an asynchronous request has been canceled.
 * While sending Commands to your SIM, your Command was rejected by its Rate Plan.
 * While transferring your SIM to another Account, your request was rejected because the SIM must be `new` and the new Account must either be the requesting Account or a Subaccount of the requesting Account.
"#),
            TwilioProgrammableWirelessError::ErrorCode33111 => Some(r#" * The Command from your SIM in text mode exceeded the maximum length of 160 characters.
"#),
            TwilioProgrammableWirelessError::ErrorCode33107 => Some(r#" * While sending a Command, your request was rejected because the SIM you provided was not found.
"#),
            TwilioProgrammableWirelessError::ErrorCode33000 => Some(r#"An unspecified error occurred while carrying out the request."#),
            TwilioProgrammableWirelessError::ErrorCode33105 => Some(r#"* While activating a SIM, your request was rejected because the SIM you are trying to activate is not compatible with the Rate Plan.
* While changing a SIM's status, your request was rejected because the requested Status change is invalid.
"#),
            TwilioProgrammableWirelessError::ErrorCode33122 => Some(r#"SIM update request was rejected because the SIM cannot be activated on a Rate Plan with deprecated Data Metering."#),
            TwilioProgrammableWirelessError::ErrorCode33103 => Some(r#"* You provided Invalid values for pagination requests."#),
            TwilioProgrammableWirelessError::ErrorCode33120 => Some(r#" * The SIM you requested has already a connectivity reset in progress which must complete before resetting again.
"#),
            TwilioProgrammableWirelessError::ErrorCode33104 => Some(r#" * While activating a SIM, your request was rejected because the SIM is not associated with a Rate Plan.
 * While sending a Command, your request was rejected because SIM is not properly configured (e.g. the SIM is not in status `active`)."#),
            TwilioProgrammableWirelessError::ErrorCode33203 => Some(r#" * The SIM that sent the Command, is not in status `ready` or `active`.
 * The associated Rate Plan of the SIM does not have commands enabled.
 * Breakout Commands from your SIMs are only permitted for Narrowband SIMs.
"#),
            TwilioProgrammableWirelessError::ErrorCode33121 => Some(r#" * While updating your SIM, the request was rejected because you cannot update `Status` and `ResetStatus` simultaneously.
 * Updating a SIM's status to `suspended` and removing its Rate Plan is not permitted.
"#),
            TwilioProgrammableWirelessError::ErrorCode33118 => Some(r#" * While sending a Command, your Command was rejected because there are already more than 100 commands queued for your individual Narrowband SIM.
"#)
        }
    }
}
