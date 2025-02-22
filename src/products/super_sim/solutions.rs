// AUTO-GENERATED. DO NOT MODIFY. YOUR CHANGES WILL BE OVER-WRITTEN
use crate::products::TwilioSuperSimError;
use standard_error::traits::StandardErrorSolutionsTrait;

impl StandardErrorSolutionsTrait for TwilioSuperSimError {
    fn solutions(&self) -> Option<&'static str> {
        match self {
            TwilioSuperSimError::ErrorCode83602 => Some(r#"  * Adjust your StartTime or EndTime parameters so that they are on a UTC hour cutoff (e.g. "2021-07-15T04:00:00Z")."#),
            TwilioSuperSimError::ErrorCode83603 => Some(r#" * Adjust your StartTime or EndTime parameters so that the time range is less than or equal to 31 days."#),
            TwilioSuperSimError::ErrorCode83003 => Some(r#"You do not need to register the Super SIM as it is already associated to your Account. The error message contains the Super SIM SID that you can use to look it up."#),
            TwilioSuperSimError::ErrorCode83001 => Some(r#"Ensure both the ICCID and Registration Code are provided in the request while registering a Super SIM."#),
            TwilioSuperSimError::ErrorCode83002 => Some(r#"Ensure the ICCID and Registration Code you used in the request belong to a registrable Super SIM. If you are unable to register a new Super SIM, please contact Twilio Support."#),
            TwilioSuperSimError::ErrorCode83007 => Some(r#"* Provide a Fleet in the activation request or first assign your Super SIM to a Fleet and then try activating it again."#),
            TwilioSuperSimError::ErrorCode83605 => Some(r#"Adjust your StartTime parameter to be at most 18 months in the past."#),
            TwilioSuperSimError::ErrorCode83008 => Some(r#"* If you want to change your Super SIM’s Fleet while it is in status ready or active, you need to provide an alternative Fleet in the request.
* Update the Super SIM’s status to inactive. This can be done either as a separate update request or in the same request as removing the Fleet."#),
            TwilioSuperSimError::ErrorCode83010 => Some(r#"* Check the public documentation for valid operations."#),
            TwilioSuperSimError::ErrorCode83703 => Some(r#" * Implement backoffs in connection attempts on your device to not overwhelm the network infrastructure."#),
            TwilioSuperSimError::ErrorCode83500 => Some(r#"* Twilio tries to restock eSIM Profiles as soon as possible. If you would like to be notified when eSIM Profiles are available again, please open a ticket with customer support."#),
            TwilioSuperSimError::ErrorCode83705 => Some(r#" * Use the Twilio Console or REST API to activate your SIM by setting its `status` to `active`."#),
            TwilioSuperSimError::ErrorCode83005 => Some(r#"* Ensure that the defined Super SIM SID or Unique Name is present in your Account."#),
            TwilioSuperSimError::ErrorCode83700 => Some(r#"- Retry connecting to the Network from your device.
- If this issue persists for your SIM, please contact Twilio Support."#),
            TwilioSuperSimError::ErrorCode83011 => Some(r#"* Specify an alternative Unique Name in your request.
* Change the Unique Name of the Super SIM already using the desired Unique Name to something else and then retry your request."#),
            TwilioSuperSimError::ErrorCode83402 => Some(r#"Ensure that your callback server can handle the callback and responds with a 2xx HTTP status code. See the [Debugger](https://www.twilio.com/console/debugger) in Console for further information on the callback response."#),
            TwilioSuperSimError::ErrorCode83000 => Some(r#"* Retry to register your Super SIM.
* If this is a continuous experience, please contact Twilio Support."#),
            TwilioSuperSimError::ErrorCode83702 => Some(r#" * Check the Network Access Profile associated with the SIM and enable the Network if you would like to use it."#),
            TwilioSuperSimError::ErrorCode83401 => Some(r#"Ensure your device is attached to a cellular network and try to send the IP Command again."#),
            TwilioSuperSimError::ErrorCode83400 => Some(r#"* Try to send the IP Command again.
* If this is a continuous experience, please contact Twilio Support."#),
            TwilioSuperSimError::ErrorCode83004 => Some(r#"* Retry updating your Super SIM.
* If you continue to experience this issue, please contact Twilio Support."#),
            TwilioSuperSimError::ErrorCode83604 => Some(r#" * Adjust your StartTime or EndTime parameters so that the time range is less than or equal to the threshold allowed for the desired Granularity."#),
            TwilioSuperSimError::ErrorCode83006 => Some(r#"* Ensure that the defined Fleet SID or Unique Name is present in your Account."#),
            TwilioSuperSimError::ErrorCode83600 => Some(r#"  * Refer to the API documentation and adjust your parameter value to a supported value."#),
            TwilioSuperSimError::ErrorCode83601 => Some(r#"  * Adjust your StartTime or EndTime parameters so that they are at midnight UTC (e.g. "2021-07-15T00:00:00Z")."#),
            TwilioSuperSimError::ErrorCode83009 => Some(r#"* Wait for the current Super SIM operation to be complete and try again."#),
            TwilioSuperSimError::ErrorCode83701 => Some(r#"* Retry connecting to the Network from your device.
* If this issue persists for your SIM, please contact Twilio Support."#),
            TwilioSuperSimError::ErrorCode83704 => Some(r#" * Use the Twilio Console or REST API to activate your SIM by setting its `status` to `ready` or `active`."#)
        }
    }
}
