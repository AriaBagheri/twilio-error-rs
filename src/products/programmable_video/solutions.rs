// AUTO-GENERATED. DO NOT MODIFY. YOUR CHANGES WILL BE OVER-WRITTEN
use crate::products::TwilioProgrammableVideoError;
use standard_error::traits::StandardErrorSolutionsTrait;

impl StandardErrorSolutionsTrait for TwilioProgrammableVideoError {
    fn solutions(&self) -> Option<&'static str> {
        match self {
            TwilioProgrammableVideoError::ErrorCode53628 => None,
            TwilioProgrammableVideoError::ErrorCode53405 => Some(r#"* If the problem persists in a Group Room, try connecting to another region.
* Check your Client's network connectivity.
* Ensure that Network Traversal Service is enabled for your Configuration Profile and/or Room.
* If you've provided custom ICE Servers then ensure that the URLs and credentials are valid.
* Select LocalNetworkPrivacyPolicy.blockAll on iOS 14+ unless you have permission to use the local network."#),
            TwilioProgrammableVideoError::ErrorCode53000 => Some(r#"* Try connecting again.
* If the problem persists, try connecting to another region."#),
            TwilioProgrammableVideoError::ErrorCode53203 => Some(r#" * Have the Client remove one or more Tracks.
 * Have the Client publish Tracks only if the number of Tracks in a Room is less than the maximum number of Tracks allowed per Room."#),
            TwilioProgrammableVideoError::ErrorCode53112 => None,
            TwilioProgrammableVideoError::ErrorCode53102 => None,
            TwilioProgrammableVideoError::ErrorCode53104 => None,
            TwilioProgrammableVideoError::ErrorCode53408 => Some(r#"* Ensure that the ICE restart happens only in SDP offers."#),
            TwilioProgrammableVideoError::ErrorCode53106 => None,
            TwilioProgrammableVideoError::ErrorCode53616 => None,
            TwilioProgrammableVideoError::ErrorCode53301 => None,
            TwilioProgrammableVideoError::ErrorCode53300 => None,
            TwilioProgrammableVideoError::ErrorCode53601 => None,
            TwilioProgrammableVideoError::ErrorCode53620 => None,
            TwilioProgrammableVideoError::ErrorCode53627 => None,
            TwilioProgrammableVideoError::ErrorCode53215 => Some(r#"- Ensure the set of rules has at least one rule.
- If you include `all` in a rule, the only other allowed field is `type`.
- Ensure every rule has a `type` field.
- Ensure `kind` is one of the valid enum values.
- Limit the number of rules to 20 or fewer."#),
            TwilioProgrammableVideoError::ErrorCode53204 => None,
            TwilioProgrammableVideoError::ErrorCode53621 => None,
            TwilioProgrammableVideoError::ErrorCode53617 => Some(r#"* Please retry the delete operation with the same parameters.
* If the error persists, please <a href="/help/contact">contact us</a> to figure out what has happened and how to fix it!
* Note the time of the error and which filter was used to select the compositions."#),
            TwilioProgrammableVideoError::ErrorCode53208 => Some(r#"* Check the bandwidth profile documentation and confirm that your configuration is valid."#),
            TwilioProgrammableVideoError::ErrorCode53205 => None,
            TwilioProgrammableVideoError::ErrorCode53109 => None,
            TwilioProgrammableVideoError::ErrorCode53661 => None,
            TwilioProgrammableVideoError::ErrorCode53120 => Some(r#"* Ensure the set of rules has at least one rule.
* If you include all in a rule, the only other allowed field is type.
* Ensure every rule has a type field.
* Ensure kind is one of the valid enum values.
* Limit the number of recording rules to 20 or fewer."#),
            TwilioProgrammableVideoError::ErrorCode53006 => Some(r#"* Try connecting later.
* If the problem persists, try connecting to another region."#),
            TwilioProgrammableVideoError::ErrorCode53002 => None,
            TwilioProgrammableVideoError::ErrorCode53207 => Some(r#"* Set the MaxPublishedTracks value from 0 (unlimited) up to the maximum allowed for the Room Type."#),
            TwilioProgrammableVideoError::ErrorCode53113 => Some(r#"Create a new Room with a unique name."#),
            TwilioProgrammableVideoError::ErrorCode53632 => None,
            TwilioProgrammableVideoError::ErrorCode53103 => Some(r#"* Make sure Ad-hoc Rooms are enabled in the Video settings for your account in Twilio Console, or create the Room explicitly using the REST API before attempting to connect to it.
* Choose a Room configuration supported by your account.
* If the problem persists, try connecting to another region."#),
            TwilioProgrammableVideoError::ErrorCode53668 => Some(r#" * In the composition settings section of your account's console, create and assign a new valid public key or disable the encryption (media will be uploaded in raw)."#),
            TwilioProgrammableVideoError::ErrorCode53001 => None,
            TwilioProgrammableVideoError::ErrorCode53206 => Some(r#" * Complete in-progress Rooms to disconnect Participants.
 * Disconnect connected Participants.
 * Contact support/sales to increase the concurrent Participants quota for your account."#),
            TwilioProgrammableVideoError::ErrorCode53123 => Some(r#"Set MaxParticipantDuration to a value inside the acceptable range of 600 seconds (10 minutes) and 86400 seconds (24 hours)"#),
            TwilioProgrammableVideoError::ErrorCode53626 => None,
            TwilioProgrammableVideoError::ErrorCode53664 => Some(r#" * In the composition settings section of your account's console, set a valid S3 URL for uploading media or disable the external storage (media will be uploaded to the default bucket)."#),
            TwilioProgrammableVideoError::ErrorCode53202 => None,
            TwilioProgrammableVideoError::ErrorCode53304 => None,
            TwilioProgrammableVideoError::ErrorCode53622 => None,
            TwilioProgrammableVideoError::ErrorCode53669 => Some(r#" * In the composition settings section of your account's console, review the credentials for accessing your S3 bucket or disable the external storage (media will be uploaded to the default bucket)."#),
            TwilioProgrammableVideoError::ErrorCode53217 => Some(r#"Use a valid value for maxVideoTracks and maxAudioTracks"#),
            TwilioProgrammableVideoError::ErrorCode53501 => None,
            TwilioProgrammableVideoError::ErrorCode53603 => None,
            TwilioProgrammableVideoError::ErrorCode53110 => None,
            TwilioProgrammableVideoError::ErrorCode53500 => None,
            TwilioProgrammableVideoError::ErrorCode53663 => Some(r#"* Please retry the delete operation with the same parameters.
* If the error persists, please <a href="/help/contact">contact us</a> to figure out what has happened and how to fix it!
* Note the time of the error and which filter was used to select the recordings."#),
            TwilioProgrammableVideoError::ErrorCode53662 => None,
            TwilioProgrammableVideoError::ErrorCode53605 => None,
            TwilioProgrammableVideoError::ErrorCode53200 => None,
            TwilioProgrammableVideoError::ErrorCode53630 => None,
            TwilioProgrammableVideoError::ErrorCode53107 => None,
            TwilioProgrammableVideoError::ErrorCode53633 => Some(r#"Please reduce the scope of the composition by excluding some of the sources to verity that they are below our maximum size.
If not possible, please <a href="/help/contact">contact us</a> and provide a description of the media files you are attempting to compose."#),
            TwilioProgrammableVideoError::ErrorCode53122 => Some(r#"Make sure that the recording operation requested is supported for the Room type."#),
            TwilioProgrammableVideoError::ErrorCode53625 => None,
            TwilioProgrammableVideoError::ErrorCode53614 => None,
            TwilioProgrammableVideoError::ErrorCode53611 => None,
            TwilioProgrammableVideoError::ErrorCode53402 => None,
            TwilioProgrammableVideoError::ErrorCode53127 => Some(r#"Do not specify Audio only flag if account started using video after October 21, 2024"#),
            TwilioProgrammableVideoError::ErrorCode53216 => Some(r#"- Increase the MaxParticipantDuration configuration for Rooms (up to 24 hours) via REST API upon room creation or via the Video Settings page in Twilio Console to change the default value."#),
            TwilioProgrammableVideoError::ErrorCode53101 => None,
            TwilioProgrammableVideoError::ErrorCode53108 => None,
            TwilioProgrammableVideoError::ErrorCode53613 => None,
            TwilioProgrammableVideoError::ErrorCode53400 => None,
            TwilioProgrammableVideoError::ErrorCode53125 => Some(r#"Make sure that only audio and/or data tracks are published in an Audio Only Group Room."#),
            TwilioProgrammableVideoError::ErrorCode53111 => None,
            TwilioProgrammableVideoError::ErrorCode53607 => None,
            TwilioProgrammableVideoError::ErrorCode53003 => None,
            TwilioProgrammableVideoError::ErrorCode53406 => Some(r#"* Verify ICE connection.
* Verify that the subscriber supports data channels.
* Verify if the subscriber has reached the maximum number of data channels."#),
            TwilioProgrammableVideoError::ErrorCode53667 => Some(r#" * In the composition settings section of your account's console, create and assign a new credentials entry for accessing your S3 bucket or disable the external storage (media will be uploaded to the default bucket)."#),
            TwilioProgrammableVideoError::ErrorCode53004 => None,
            TwilioProgrammableVideoError::ErrorCode53623 => None,
            TwilioProgrammableVideoError::ErrorCode53302 => None,
            TwilioProgrammableVideoError::ErrorCode53118 => Some(r#"* Some operations can only be performed on in-progress Rooms. Check the status of your Room before attempting the operation."#),
            TwilioProgrammableVideoError::ErrorCode53660 => None,
            TwilioProgrammableVideoError::ErrorCode53666 => Some(r#" * In the composition settings section of your account's console, set a valid public key or disable the encryption (media will be uploaded in raw)."#),
            TwilioProgrammableVideoError::ErrorCode53201 => None,
            TwilioProgrammableVideoError::ErrorCode53124 => Some(r#"Make sure that the flag is supported by the Room type."#),
            TwilioProgrammableVideoError::ErrorCode53407 => Some(r#"* Ensure that your certificate is valid.
* Ensure that you have a stable internet connection.
* Ensure that the browser or the Mobile SDK supports newer versions of DTLS."#),
            TwilioProgrammableVideoError::ErrorCode53631 => None,
            TwilioProgrammableVideoError::ErrorCode53403 => Some(r#"* Update to the latest Twilio Video SDK
* Use the latest browser version (if applicable)"#),
            TwilioProgrammableVideoError::ErrorCode53105 => None,
            TwilioProgrammableVideoError::ErrorCode53303 => None,
            TwilioProgrammableVideoError::ErrorCode53610 => None,
            TwilioProgrammableVideoError::ErrorCode53126 => Some(r#"Room type for accounts which started using video after October 21, 2024 must be Group Room"#),
            TwilioProgrammableVideoError::ErrorCode53602 => None,
            TwilioProgrammableVideoError::ErrorCode53119 => Some(r#"* Complete existing Rooms.
* Contact support/sales to increase the concurrent Rooms quota for your account."#),
            TwilioProgrammableVideoError::ErrorCode53604 => None,
            TwilioProgrammableVideoError::ErrorCode53100 => None,
            TwilioProgrammableVideoError::ErrorCode53612 => None,
            TwilioProgrammableVideoError::ErrorCode53600 => None,
            TwilioProgrammableVideoError::ErrorCode53209 => Some(r#"Set Status to disconnected."#),
            TwilioProgrammableVideoError::ErrorCode53624 => None,
            TwilioProgrammableVideoError::ErrorCode53121 => Some(r#"* Complete existing Rooms.
* Contact support/sales to increase the concurrent Rooms quota for your account."#),
            TwilioProgrammableVideoError::ErrorCode53606 => None,
            TwilioProgrammableVideoError::ErrorCode53665 => Some(r#" * In the composition settings section of your account's console, set some valid credentials for accessing your S3 bucket or disable the external storage (media will be uploaded to the default bucket)."#),
            TwilioProgrammableVideoError::ErrorCode53615 => None
        }
    }
}
