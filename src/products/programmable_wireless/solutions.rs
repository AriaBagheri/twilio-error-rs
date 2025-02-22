// AUTO-GENERATED. DO NOT MODIFY. YOUR CHANGES WILL BE OVER-WRITTEN
use crate::products::TwilioProgrammableWirelessError;
use standard_error::traits::StandardErrorSolutionsTrait;

impl StandardErrorSolutionsTrait for TwilioProgrammableWirelessError {
    fn solutions(&self) -> Option<&'static str> {
        match self {
            TwilioProgrammableWirelessError::ErrorCode33010 => Some(r#" * Verify that there is not an update already in progress for the SIM. If an update is in progress, the SIM's status will be either `scheduled` or `updating` and you will need to wait for it to finish before you can make additional updates to the SIM's status or Rate Plan.
"#),
            TwilioProgrammableWirelessError::ErrorCode33101 => Some(r#"* While removing the Rate Plan from a SIM, ensure the SIM is neither in status `ready` nor `active`.
* While updating the Rate Plan of a SIM, ensure that the SIM is in status `new` or `deactivated` while moving the SIM to a Rate Plan with a different Data Limit Strategy.
* While updating the Rate Plan of a SIM, ensure that a Narrowband SIM has a narrowband-enabled Rate Plan, or a non-narrowband SIM has a corresponding Rate Plan.
* While creating or updating a Rate Plan, ensure `UniqueName` has a different format than a Rate Plan SID, has a maximum length of 64 characters and does not exist yet.
* While creating a Rate Plan, ensure that the limits (e.g. `DataLimit`, `NationalRoamingDataLimit`, `InternationalRoamingDataLimit`) are equal or between `0` and `2000000` (2TB)."#),
            TwilioProgrammableWirelessError::ErrorCode33004 => Some(r#"Try again later."#),
            TwilioProgrammableWirelessError::ErrorCode33102 => Some(r#" * While sending a Command, ensure `Sim` parameter is provided.
 * While transferring a SIM to another Account, ensure a `RatePlan` is provided that present in the target Account.
 * While transferring a SIM to another Account, ensure the SIM to transfer is not in status `ready`."#),
            TwilioProgrammableWirelessError::ErrorCode33119 => Some(r#" * Try again later if there is an ongoing maintenance.
 * Verify the SIM is not a Narrowband SIM.
 * Ensure the SIM is either in status `ready` or `active`.
"#),
            TwilioProgrammableWirelessError::ErrorCode33108 => Some(r#" * While updating a SIM's Rate Plan, ensure the provided `RatePlan` exists on your Account.
 * While transferring a SIM to another Account, ensure the provided `RatePlan` is present in the target Account.
"#),
            TwilioProgrammableWirelessError::ErrorCode33201 => Some(r#" * While sending a Command, ensure the Rate Plan associated to the given SIM has Commands enabled.
 * Contact Twilio Support to help with the SIM transfer.
"#),
            TwilioProgrammableWirelessError::ErrorCode33111 => Some(r#" * Ensure that Commands in text mode do not exceed the maximum length.
"#),
            TwilioProgrammableWirelessError::ErrorCode33107 => Some(r#" * While sending a Command, ensure the SIM exists.
"#),
            TwilioProgrammableWirelessError::ErrorCode33000 => Some(r#"See the [Debugger](https://www.twilio.com/console/debugger) in Console for further information. If this is a continuous experience, please contact Twilio Support."#),
            TwilioProgrammableWirelessError::ErrorCode33105 => Some(r#"* While activating a SIM, ensure that the SIM is compatible with the Rate Plan, e.g. the Rate Plan specifies international roaming for data but the SIM does not support it.
* While changing a SIM's status, ensure that the Status transition is valid. Read more about [Status Values](https://www.twilio.com/docs/iot/wireless/api/sim-resource#status-values).
"#),
            TwilioProgrammableWirelessError::ErrorCode33122 => Some(r#"Consider using a Rate Plan with `payg` Data Metering."#),
            TwilioProgrammableWirelessError::ErrorCode33103 => Some(r#" * Ensure that `Page` is a positive integer.
 * Ensure that `PageSize` is a positive integer and not greater than 1000.
 * Ensure that `PageToken` is valid as provided by a previous request."#),
            TwilioProgrammableWirelessError::ErrorCode33120 => Some(r#" * Try again later and ensure the SIM's `ResetStatus` is not `resetting`.
"#),
            TwilioProgrammableWirelessError::ErrorCode33104 => Some(r#" * While activating a SIM, ensure the SIM is associated with a proper Rate Plan.
 * While sending a Command, ensure the given `sim` is in status `ready` or `active`, and paired with a SIM.
"#),
            TwilioProgrammableWirelessError::ErrorCode33203 => Some(r#" * While sending Commands from your SIM, verify that the SIM is either in status `ready` or `active`.
 * While sending Commands from your SIM, ensure that the associated Rate Plan of the SIM has commands enabled.
"#),
            TwilioProgrammableWirelessError::ErrorCode33121 => Some(r#" * While updating a SIM, ensure not to update `Status` and `ResetStatus` within the same request.
 * While updating a SIM, you cannot simultaneously update its `Status` to `suspended` and remove its Rate Plan.
"#),
            TwilioProgrammableWirelessError::ErrorCode33118 => Some(r#" * While sending a Command, ensure there are less than 100 commands queued for an individual Narrowband SIM.
"#)
        }
    }
}
