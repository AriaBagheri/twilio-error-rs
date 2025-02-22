// AUTO-GENERATED. DO NOT MODIFY. YOUR CHANGES WILL BE OVER-WRITTEN
use crate::products::TwilioSuperSimError;
use standard_error::traits::StandardErrorDocsTrait;

impl StandardErrorDocsTrait for TwilioSuperSimError {
    fn docs(&self) -> &'static str {
        match self {
            TwilioSuperSimError::ErrorCode83602 => r#"## ERROR - 83602

### Request StartTime and/or EndTime must be aligned to UTC hour boundaries.

 Certain requests for Usage Records can only be made with StartTime and EndTime parameters that align to a UTC hour cutoff (e.g. "2021-07-15T04:00:00Z").

#### Possible Causes
  * This happens if you request Usage Records with StartTime or EndTime parameters that do not fall on an hour cutoff for certain UsageRecord requests.

#### Possible Solutions
  * Adjust your StartTime or EndTime parameters so that they are on a UTC hour cutoff (e.g. "2021-07-15T04:00:00Z")."#,
            TwilioSuperSimError::ErrorCode83603 => r#"## ERROR - 83603

### The maximum allowed query period is 31 days for group by sim queries

 Requests for Usage Records grouping by "sim" can query a maximum time period of 31 days.

#### Possible Causes
 * This happens if you request Usage Records grouped by SIM with StartTime or EndTime parameters that span more than 31 days.

#### Possible Solutions
 * Adjust your StartTime or EndTime parameters so that the time range is less than or equal to 31 days."#,
            TwilioSuperSimError::ErrorCode83003 => r#"## ERROR - 83003

### The Super SIM already belongs to the requesting Account.

 Raised whenever you try to register a Super SIM that is already registered to your Account.

#### Possible Causes
You tried to register a Super SIM that already belongs to your Account.

#### Possible Solutions
You do not need to register the Super SIM as it is already associated to your Account. The error message contains the Super SIM SID that you can use to look it up."#,
            TwilioSuperSimError::ErrorCode83001 => r#"## ERROR - 83001

### Parameter missing while registering a Super SIM

 Raised whenever a required parameter was not passed with the request while registering a Super SIM. The error object’s detail field will indicate which parameter was not provided.

#### Possible Causes
Either you did not provide the Super SIM's ICCID or Registration Code in the request.

#### Possible Solutions
Ensure both the ICCID and Registration Code are provided in the request while registering a Super SIM."#,
            TwilioSuperSimError::ErrorCode83002 => r#"## ERROR - 83002

### Super SIM cannot be registered

 The ICCID and Registration Code do not belong to a registrable Super SIM.

#### Possible Causes
You tried to register a Super SIM that cannot be registered to your Account.

#### Possible Solutions
Ensure the ICCID and Registration Code you used in the request belong to a registrable Super SIM. If you are unable to register a new Super SIM, please contact Twilio Support."#,
            TwilioSuperSimError::ErrorCode83007 => r#"## ERROR - 83007

### Unable to activate your Super SIM as it does not belong to a Fleet

 Your Super SIM cannot be activated without being assigned to a Fleet.

#### Possible Causes
* The Super SIM is not assigned to a Fleet and the activation request did not include a Fleet.

#### Possible Solutions
* Provide a Fleet in the activation request or first assign your Super SIM to a Fleet and then try activating it again."#,
            TwilioSuperSimError::ErrorCode83605 => r#"## ERROR - 83605

### StartTime parameter is too far in the past. It must be within the last 18 months.

 You can query usage data up to 18 months in the past. The oldest allowed StartTime is 18 months prior to the start of the current UTC day. For example, if the current time is 2021-12-13T5:00:00Z, then you can query usage as far back as 2020-06-13T:00:00:00Z.

#### Possible Causes
This happens if you request Usage Records with a StartTime parameter more than 18 months in the past.

#### Possible Solutions
Adjust your StartTime parameter to be at most 18 months in the past."#,
            TwilioSuperSimError::ErrorCode83008 => r#"## ERROR - 83008

### Unable to remove your Super SIM from its Fleet

 Your Super SIM must be assigned to a Fleet while it is in status ‘ready’ or ‘active’.

#### Possible Causes
* The Super SIM is in status ready or active, and must belong to a Fleet.

#### Possible Solutions
* If you want to change your Super SIM’s Fleet while it is in status ready or active, you need to provide an alternative Fleet in the request.
* Update the Super SIM’s status to inactive. This can be done either as a separate update request or in the same request as removing the Fleet."#,
            TwilioSuperSimError::ErrorCode83010 => r#"## ERROR - 83010

### Unable to update your Super SIM to the desired status

 Your Super SIM cannot be updated from its current status to the desired status.

#### Possible Causes
* The Super SIM’s current status cannot be updated to the desired status.

#### Possible Solutions
* Check the public documentation for valid operations."#,
            TwilioSuperSimError::ErrorCode83703 => r#"## ERROR - 83703

### Attachment Rejected Due To Rate Limiting

 The SIM was not able to attach to the specified Network because your device has repeatedly attempted to connect too frequently and is rate limited. Rate limiting is enforced to protect the network infrastracture and prevent one device from impacting the experience of other users on the Network.

#### Possible Causes
 * Your device may be stuck in a loop and is repeatedly and rapidly attempting to reattach to the network.

#### Possible Solutions
 * Implement backoffs in connection attempts on your device to not overwhelm the network infrastructure."#,
            TwilioSuperSimError::ErrorCode83500 => r#"## ERROR - 83500

### No eSIM Profiles are available

 Twilio currently does not have eSIM Profiles available for ordering.

#### Possible Causes
* There are no eSIM Profiles currently available for ordering.

#### Possible Solutions
* Twilio tries to restock eSIM Profiles as soon as possible. If you would like to be notified when eSIM Profiles are available again, please open a ticket with customer support."#,
            TwilioSuperSimError::ErrorCode83705 => r#"## ERROR - 83705

### Attachment Rejected Due To SIM In Inactive State

 The SIM was not able to attach to the specified Network because your SIM is `inactive`. A SIM must be in the `ready` or `active` states to access the Network.

#### Possible Causes
 * Your SIM is in the `inactive` state.

#### Possible Solutions
 * Use the Twilio Console or REST API to activate your SIM by setting its `status` to `active`."#,
            TwilioSuperSimError::ErrorCode83005 => r#"## ERROR - 83005

### Super SIM not found

 Your Super SIM could not be found.

#### Possible Causes
* The Super SIM could not be found. It may have been deleted or transferred to another account.

#### Possible Solutions
* Ensure that the defined Super SIM SID or Unique Name is present in your Account."#,
            TwilioSuperSimError::ErrorCode83700 => r#"## ERROR - 83700

### Attachment Failed Due To Internal Error

 An unexpected error occurred while attempting to process your SIM's attachment request to the Network. This is usually a transient error and should resolve itself in time.

#### Possible Causes
- An unexpected error occurred in processing your SIM's attachment to the Network

#### Possible Solutions
- Retry connecting to the Network from your device.
- If this issue persists for your SIM, please contact Twilio Support."#,
            TwilioSuperSimError::ErrorCode83011 => r#"## ERROR - 83011

### A Super SIM with the specified Unique Name exists already

 Your Super SIM cannot be updated with the provided Unique Name because another Super SIM on your Account is already using it.

#### Possible Causes
* A Unique Name must be unique across all Super SIMs that belong to your Account. Two Super SIMs cannot have the same Unique Name.

#### Possible Solutions
* Specify an alternative Unique Name in your request.
* Change the Unique Name of the Super SIM already using the desired Unique Name to something else and then retry your request."#,
            TwilioSuperSimError::ErrorCode83402 => r#"## ERROR - 83402

### Received error response to IP Command callback request

 Twilio received an error response when attempting to send an IP Command event to the provided callback URL.

#### Possible Causes
Twilio did not receive a 2xx response from the callback URL when attempting to send an IP Command event.

#### Possible Solutions
Ensure that your callback server can handle the callback and responds with a 2xx HTTP status code. See the [Debugger](https://www.twilio.com/console/debugger) in Console for further information on the callback response."#,
            TwilioSuperSimError::ErrorCode83000 => r#"## ERROR - 83000

### Super SIM registration failed due to Internal Error

 Your Super SIM registration request has caused an internal error that is not further specified.

#### Possible Causes
An unspecified error occurred while registering your Super SIM.

#### Possible Solutions
* Retry to register your Super SIM.
* If this is a continuous experience, please contact Twilio Support."#,
            TwilioSuperSimError::ErrorCode83702 => r#"## ERROR - 83702

### Attachment Rejected Due To Network Not Allowed

 The SIM could not to attach to the specified Network, either because its Fleet's Network Access Profile did not allow the Network or because Twilio has blocked the IMSI currently being used from attaching to the Network.

#### Possible Causes
 * The Network Access Profile associated with this SIM's Fleet has blocked access to this Network.
 * Twilio has disabled this Network for all SIMs.

#### Possible Solutions
 * Check the Network Access Profile associated with the SIM and enable the Network if you would like to use it."#,
            TwilioSuperSimError::ErrorCode83401 => r#"## ERROR - 83401

### The device was not attached to a cellular network

 The device was not attached to a cellular network when Twilio attempted to send the IP Command. The IP Command was not sent to the device.

#### Possible Causes
Your device was not attached to a cellular network when Twilio attempted to send the IP Command.

#### Possible Solutions
Ensure your device is attached to a cellular network and try to send the IP Command again."#,
            TwilioSuperSimError::ErrorCode83400 => r#"## ERROR - 83400

### IP Commands error

 Raised whenever delivering an IP Command fails with an error that is not covered by a more specific error code.

#### Possible Causes
An unspecified error occurred while delivering your IP Command.

#### Possible Solutions
* Try to send the IP Command again.
* If this is a continuous experience, please contact Twilio Support."#,
            TwilioSuperSimError::ErrorCode83004 => r#"## ERROR - 83004

### Super SIM update operation failed due to Internal Error

 Your Super SIM update request has caused an internal error that is not further specified.

#### Possible Causes
* An unspecified error occurred while updating your Super SIM.

#### Possible Solutions
* Retry updating your Super SIM.
* If you continue to experience this issue, please contact Twilio Support."#,
            TwilioSuperSimError::ErrorCode83604 => r#"## ERROR - 83604

### The requested query period exceeds the maximum allowed period for the requested Granularity

 Requests for Usage Records with granularity "all" may only query up to 18 months of data, granularity "day" may query up to 3 months of data, and granularity "hour" may query up to 31 days of data.

#### Possible Causes
  * This happens if you request Usage Records with StartTime or EndTime parameters defining a time range exceeding the maximum allowed threshold for a given Granularity.

#### Possible Solutions
 * Adjust your StartTime or EndTime parameters so that the time range is less than or equal to the threshold allowed for the desired Granularity."#,
            TwilioSuperSimError::ErrorCode83006 => r#"## ERROR - 83006

### Super SIM’s Target Fleet not found

 Your Super SIM cannot be assigned to the target Fleet because the Fleet could not be found.

#### Possible Causes
* The defined Fleet’s SID or Unique Name does not exist.

#### Possible Solutions
* Ensure that the defined Fleet SID or Unique Name is present in your Account."#,
            TwilioSuperSimError::ErrorCode83600 => r#"## ERROR - 83600

###  An invalid parameter value was passed to the API

 A parameter was passed to the Usage Records API which failed validation. This usually means the value format was invalid, or multiple parameters in the request conflict with each other.

#### Possible Causes
  * Check the response message for details on which parameter was invalid and the specific reason.

#### Possible Solutions
  * Refer to the API documentation and adjust your parameter value to a supported value."#,
            TwilioSuperSimError::ErrorCode83601 => r#"## ERROR - 83601

### Request StartTime and/or EndTime must be aligned to UTC day boundaries

 Certain requests for Usage Records can only be made with StartTime and EndTime parameters that align to midnight UTC (e.g. "2021-07-15T00:00:00Z").

#### Possible Causes
  * This happens if you request Usage Records with StartTime or EndTime parameters that do not fall on a day cutoff (i.e. midnight UTC) for certain UsageRecord requests.

#### Possible Solutions
  * Adjust your StartTime or EndTime parameters so that they are at midnight UTC (e.g. "2021-07-15T00:00:00Z")."#,
            TwilioSuperSimError::ErrorCode83009 => r#"## ERROR - 83009

### Unable to update your Super SIM’s Fleet while it is in status scheduled

 Your Fleet update request cannot be performed on your Super SIM as it is in status scheduled.

#### Possible Causes
* Another operation on your Super SIM is in progress.

#### Possible Solutions
* Wait for the current Super SIM operation to be complete and try again."#,
            TwilioSuperSimError::ErrorCode83701 => r#"## ERROR - 83701

### Data Session Establishment Failed Due To Internal Error

 An unexpected error occurred while establishing a Data Session for your SIM. This is usually a transient error and should resolve itself in time.

#### Possible Causes
* An unexpected error occurred in setting up a Data Session for your SIM.

#### Possible Solutions
* Retry connecting to the Network from your device.
* If this issue persists for your SIM, please contact Twilio Support."#,
            TwilioSuperSimError::ErrorCode83704 => r#"## ERROR - 83704

### Attachment Rejected Due To SIM In New State

 The SIM was not able to attach to the specified network because your SIM has not been activated yet and is in the `new` state. This is the state that SIMs arrive in when they are shipped and they must be activated first via Console or REST API before they can be used.

#### Possible Causes
 * Your SIM is in the `new` state and is not active.

#### Possible Solutions
 * Use the Twilio Console or REST API to activate your SIM by setting its `status` to `ready` or `active`."#
        }
    }
}
