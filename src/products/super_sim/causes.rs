// AUTO-GENERATED. DO NOT MODIFY. YOUR CHANGES WILL BE OVER-WRITTEN
use crate::products::TwilioSuperSimError;
use standard_error::traits::StandardErrorCausesTrait;

impl StandardErrorCausesTrait for TwilioSuperSimError {
    fn causes(&self) -> Option<&'static str> {
        match self {
            TwilioSuperSimError::ErrorCode83602 => Some(r#"  * This happens if you request Usage Records with StartTime or EndTime parameters that do not fall on an hour cutoff for certain UsageRecord requests."#),
            TwilioSuperSimError::ErrorCode83603 => Some(r#" * This happens if you request Usage Records grouped by SIM with StartTime or EndTime parameters that span more than 31 days."#),
            TwilioSuperSimError::ErrorCode83003 => Some(r#"You tried to register a Super SIM that already belongs to your Account."#),
            TwilioSuperSimError::ErrorCode83001 => Some(r#"Either you did not provide the Super SIM's ICCID or Registration Code in the request."#),
            TwilioSuperSimError::ErrorCode83002 => Some(r#"You tried to register a Super SIM that cannot be registered to your Account."#),
            TwilioSuperSimError::ErrorCode83007 => Some(r#"* The Super SIM is not assigned to a Fleet and the activation request did not include a Fleet."#),
            TwilioSuperSimError::ErrorCode83605 => Some(r#"This happens if you request Usage Records with a StartTime parameter more than 18 months in the past."#),
            TwilioSuperSimError::ErrorCode83008 => Some(r#"* The Super SIM is in status ready or active, and must belong to a Fleet."#),
            TwilioSuperSimError::ErrorCode83010 => Some(r#"* The Super SIM’s current status cannot be updated to the desired status."#),
            TwilioSuperSimError::ErrorCode83703 => Some(r#" * Your device may be stuck in a loop and is repeatedly and rapidly attempting to reattach to the network."#),
            TwilioSuperSimError::ErrorCode83500 => Some(r#"* There are no eSIM Profiles currently available for ordering."#),
            TwilioSuperSimError::ErrorCode83705 => Some(r#" * Your SIM is in the `inactive` state."#),
            TwilioSuperSimError::ErrorCode83005 => Some(r#"* The Super SIM could not be found. It may have been deleted or transferred to another account."#),
            TwilioSuperSimError::ErrorCode83700 => Some(r#"- An unexpected error occurred in processing your SIM's attachment to the Network"#),
            TwilioSuperSimError::ErrorCode83011 => Some(r#"* A Unique Name must be unique across all Super SIMs that belong to your Account. Two Super SIMs cannot have the same Unique Name."#),
            TwilioSuperSimError::ErrorCode83402 => Some(r#"Twilio did not receive a 2xx response from the callback URL when attempting to send an IP Command event."#),
            TwilioSuperSimError::ErrorCode83000 => Some(r#"An unspecified error occurred while registering your Super SIM."#),
            TwilioSuperSimError::ErrorCode83702 => Some(r#" * The Network Access Profile associated with this SIM's Fleet has blocked access to this Network.
 * Twilio has disabled this Network for all SIMs."#),
            TwilioSuperSimError::ErrorCode83401 => Some(r#"Your device was not attached to a cellular network when Twilio attempted to send the IP Command."#),
            TwilioSuperSimError::ErrorCode83400 => Some(r#"An unspecified error occurred while delivering your IP Command."#),
            TwilioSuperSimError::ErrorCode83004 => Some(r#"* An unspecified error occurred while updating your Super SIM."#),
            TwilioSuperSimError::ErrorCode83604 => Some(r#"  * This happens if you request Usage Records with StartTime or EndTime parameters defining a time range exceeding the maximum allowed threshold for a given Granularity."#),
            TwilioSuperSimError::ErrorCode83006 => Some(r#"* The defined Fleet’s SID or Unique Name does not exist."#),
            TwilioSuperSimError::ErrorCode83600 => Some(r#"  * Check the response message for details on which parameter was invalid and the specific reason."#),
            TwilioSuperSimError::ErrorCode83601 => Some(r#"  * This happens if you request Usage Records with StartTime or EndTime parameters that do not fall on a day cutoff (i.e. midnight UTC) for certain UsageRecord requests."#),
            TwilioSuperSimError::ErrorCode83009 => Some(r#"* Another operation on your Super SIM is in progress."#),
            TwilioSuperSimError::ErrorCode83701 => Some(r#"* An unexpected error occurred in setting up a Data Session for your SIM."#),
            TwilioSuperSimError::ErrorCode83704 => Some(r#" * Your SIM is in the `new` state and is not active."#)
        }
    }
}
