// AUTO-GENERATED. DO NOT MODIFY. YOUR CHANGES WILL BE OVER-WRITTEN
use crate::products::TwilioProgrammableWirelessError;
use standard_error::traits::StandardErrorDocsTrait;

impl StandardErrorDocsTrait for TwilioProgrammableWirelessError {
    fn docs(&self) -> &'static str {
        match self {
            TwilioProgrammableWirelessError::ErrorCode33010 => r#"## ERROR - 33010

### Conflicting update

 Raised whenever an update is requested while another update is already in progress.	

#### Possible Causes
 * You cannot change a SIM's status or its Rate Plan while another update is already in progress.

#### Possible Solutions
 * Verify that there is not an update already in progress for the SIM. If an update is in progress, the SIM's status will be either `scheduled` or `updating` and you will need to wait for it to finish before you can make additional updates to the SIM's status or Rate Plan.
"#,
            TwilioProgrammableWirelessError::ErrorCode33101 => r#"## ERROR - 33101

### Invalid Parameter Value	

 Raised when an invalid parameter is provided. Please refer to the [API docs](https://www.twilio.com/docs/iot/wireless/api) for more details.

#### Possible Causes
* Your request contained invalid parameters. Please refer to the actual error response to get more details.
* While removing the Rate Plan of a SIM, your SIM must not be in status `active` or `ready`.
* While updating the Rate Plan of a SIM, you cannot move from NB-Annual to NB-developer.
* While updating the Rate Plan of a SIM, you can only move the Rate Plan to a different Data Limit Strategy while your SIM is `new` or `deactivated`.
* While updating the Rate Plan of a SIM, Narrowband SIMs cannot be paired with a non-narrowband enabled Rate Plan, or vice versa.
* While creating or updating a Rate Plan, `UniqueName` is invalid.
* While creating a Rate Plan, the limits (e.g. `DataLimit`, `NationalRoamingDataLimit`, `InternationalRoamingDataLimit`) you defined are out of range.

#### Possible Solutions
* While removing the Rate Plan from a SIM, ensure the SIM is neither in status `ready` nor `active`.
* While updating the Rate Plan of a SIM, ensure that the SIM is in status `new` or `deactivated` while moving the SIM to a Rate Plan with a different Data Limit Strategy.
* While updating the Rate Plan of a SIM, ensure that a Narrowband SIM has a narrowband-enabled Rate Plan, or a non-narrowband SIM has a corresponding Rate Plan.
* While creating or updating a Rate Plan, ensure `UniqueName` has a different format than a Rate Plan SID, has a maximum length of 64 characters and does not exist yet.
* While creating a Rate Plan, ensure that the limits (e.g. `DataLimit`, `NationalRoamingDataLimit`, `InternationalRoamingDataLimit`) are equal or between `0` and `2000000` (2TB)."#,
            TwilioProgrammableWirelessError::ErrorCode33004 => r#"## ERROR - 33004

### Service is unavailable	

 Raised whenever a service is temporarily unavailable.

#### Possible Causes
The requested service or any of its downstreams is temporarily unavailable.

#### Possible Solutions
Try again later."#,
            TwilioProgrammableWirelessError::ErrorCode33102 => r#"## ERROR - 33102

### Parameter missing

 Raised whenever a required parameter was not passed with the request. The error object’s detail field will indicate which parameter was not provided.

#### Possible Causes
 * While sending a Command, you did not indicate the `Sim` to send it to.
 * While transferring a SIM to another Account, you need to provide a new Rate Plan present in the target Account or remove the current Rate Plan if the SIM to move is currently assigned to a Rate Plan.
 * While transferring a SIM to another Account, you cannot transfer a SIM in status `ready`.


#### Possible Solutions
 * While sending a Command, ensure `Sim` parameter is provided.
 * While transferring a SIM to another Account, ensure a `RatePlan` is provided that present in the target Account.
 * While transferring a SIM to another Account, ensure the SIM to transfer is not in status `ready`."#,
            TwilioProgrammableWirelessError::ErrorCode33119 => r#"## ERROR - 33119

### SIM connectivity reset not allowed

 Raised whenever a SIM connectivity reset is requested but not permitted.


#### Possible Causes
 * This operation is temporarily unavailable because the downstream carrier is undergoing maintenance and not accepting requests.
 * Your SIM either does not support this operation or is not in a status where it can connect to a cellular network.


#### Possible Solutions
 * Try again later if there is an ongoing maintenance.
 * Verify the SIM is not a Narrowband SIM.
 * Ensure the SIM is either in status `ready` or `active`.
"#,
            TwilioProgrammableWirelessError::ErrorCode33108 => r#"## ERROR - 33108

### Rate Plan not found

 Raised whenever the given Rate Plan is not found.


#### Possible Causes
 * While updating a SIM's Rate Plan, your request was rejected because the Rate Plan was not found.
 * While transferring a SIM to another Account, your request was rejected because the given Rate Plan was not found in the target Account.


#### Possible Solutions
 * While updating a SIM's Rate Plan, ensure the provided `RatePlan` exists on your Account.
 * While transferring a SIM to another Account, ensure the provided `RatePlan` is present in the target Account.
"#,
            TwilioProgrammableWirelessError::ErrorCode33201 => r#"## ERROR - 33201

### Unauthorized

 Raised whenever the requested operation is not permitted.


#### Possible Causes
 * Your Account has been suspended and an asynchronous request has been canceled.
 * While sending Commands to your SIM, your Command was rejected by its Rate Plan.
 * While transferring your SIM to another Account, your request was rejected because the SIM must be `new` and the new Account must either be the requesting Account or a Subaccount of the requesting Account.


#### Possible Solutions
 * While sending a Command, ensure the Rate Plan associated to the given SIM has Commands enabled.
 * Contact Twilio Support to help with the SIM transfer.
"#,
            TwilioProgrammableWirelessError::ErrorCode33111 => r#"## ERROR - 33111

### Command exceeded max length

 Raised whenever a Command from your SIM exceeds the maximum length of 160 characters. As a result the Command is truncated but still delivered to the target Callback URL.


#### Possible Causes
 * The Command from your SIM in text mode exceeded the maximum length of 160 characters.


#### Possible Solutions
 * Ensure that Commands in text mode do not exceed the maximum length.
"#,
            TwilioProgrammableWirelessError::ErrorCode33107 => r#"## ERROR - 33107

### SIM not found

 Raised whenever the given SIM is not found.

#### Possible Causes
 * While sending a Command, your request was rejected because the SIM you provided was not found.


#### Possible Solutions
 * While sending a Command, ensure the SIM exists.
"#,
            TwilioProgrammableWirelessError::ErrorCode33000 => r#"## ERROR - 33000

### Generic Error

 Your request has caused an error that is not further specified.

#### Possible Causes
An unspecified error occurred while carrying out the request.

#### Possible Solutions
See the [Debugger](https://www.twilio.com/console/debugger) in Console for further information. If this is a continuous experience, please contact Twilio Support."#,
            TwilioProgrammableWirelessError::ErrorCode33105 => r#"## ERROR - 33105

### Transition invalid

 Raised whenever the SIM cannot be transitioned to the requested status.


#### Possible Causes
* While activating a SIM, your request was rejected because the SIM you are trying to activate is not compatible with the Rate Plan.
* While changing a SIM's status, your request was rejected because the requested Status change is invalid.


#### Possible Solutions
* While activating a SIM, ensure that the SIM is compatible with the Rate Plan, e.g. the Rate Plan specifies international roaming for data but the SIM does not support it.
* While changing a SIM's status, ensure that the Status transition is valid. Read more about [Status Values](https://www.twilio.com/docs/iot/wireless/api/sim-resource#status-values).
"#,
            TwilioProgrammableWirelessError::ErrorCode33122 => r#"## ERROR - 33122

### Rate Plan Is Not Allowed

Cannot activate a SIM on a Rate Plan with deprecated Data Metering. 

#### Possible Causes
SIM update request was rejected because the SIM cannot be activated on a Rate Plan with deprecated Data Metering.

#### Possible Solutions
Consider using a Rate Plan with `payg` Data Metering."#,
            TwilioProgrammableWirelessError::ErrorCode33103 => r#"## ERROR - 33103

### Paging information invalid

 Raised whenever an invalid value for pagination while listing resources is provided.

#### Possible Causes
* You provided Invalid values for pagination requests.

#### Possible Solutions
 * Ensure that `Page` is a positive integer.
 * Ensure that `PageSize` is a positive integer and not greater than 1000.
 * Ensure that `PageToken` is valid as provided by a previous request."#,
            TwilioProgrammableWirelessError::ErrorCode33120 => r#"## ERROR - 33120

### SIM connectivity reset in progress

 Raised whenever a SIM connectivity reset is in progress while another SIM connectivity reset is requested.


#### Possible Causes
 * The SIM you requested has already a connectivity reset in progress which must complete before resetting again.


#### Possible Solutions
 * Try again later and ensure the SIM's `ResetStatus` is not `resetting`.
"#,
            TwilioProgrammableWirelessError::ErrorCode33104 => r#"## ERROR - 33104

### Configuration incomplete

 Raised whenever the configuration for a given resource is incomplete.


#### Possible Causes
 * While activating a SIM, your request was rejected because the SIM is not associated with a Rate Plan.
 * While sending a Command, your request was rejected because SIM is not properly configured (e.g. the SIM is not in status `active`).

#### Possible Solutions
 * While activating a SIM, ensure the SIM is associated with a proper Rate Plan.
 * While sending a Command, ensure the given `sim` is in status `ready` or `active`, and paired with a SIM.
"#,
            TwilioProgrammableWirelessError::ErrorCode33203 => r#"## ERROR - 33203

### Messaging not allowed

 Raised whenever a Command from your SIM is received but not permitted.


#### Possible Causes
 * The SIM that sent the Command, is not in status `ready` or `active`.
 * The associated Rate Plan of the SIM does not have commands enabled.
 * Breakout Commands from your SIMs are only permitted for Narrowband SIMs.


#### Possible Solutions
 * While sending Commands from your SIM, verify that the SIM is either in status `ready` or `active`.
 * While sending Commands from your SIM, ensure that the associated Rate Plan of the SIM has commands enabled.
"#,
            TwilioProgrammableWirelessError::ErrorCode33121 => r#"## ERROR - 33121

### Invalid Parameter Combination

 Raised whenever a request contains conflicting parameters.


#### Possible Causes
 * While updating your SIM, the request was rejected because you cannot update `Status` and `ResetStatus` simultaneously.
 * Updating a SIM's status to `suspended` and removing its Rate Plan is not permitted.


#### Possible Solutions
 * While updating a SIM, ensure not to update `Status` and `ResetStatus` within the same request.
 * While updating a SIM, you cannot simultaneously update its `Status` to `suspended` and remove its Rate Plan.
"#,
            TwilioProgrammableWirelessError::ErrorCode33118 => r#"## ERROR - 33118

### Number of Commands exceeded

 Raised whenever there are too many Commands queued for an individual SIM.


#### Possible Causes
 * While sending a Command, your Command was rejected because there are already more than 100 commands queued for your individual Narrowband SIM.


#### Possible Solutions
 * While sending a Command, ensure there are less than 100 commands queued for an individual Narrowband SIM.
"#
        }
    }
}
