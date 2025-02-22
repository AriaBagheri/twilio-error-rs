// AUTO-GENERATED. DO NOT MODIFY. YOUR CHANGES WILL BE OVER-WRITTEN
use crate::products::TwilioProgrammableVideoError;
use standard_error::traits::StandardErrorCausesTrait;

impl StandardErrorCausesTrait for TwilioProgrammableVideoError {
    fn causes(&self) -> Option<&'static str> {
        match self {
            TwilioProgrammableVideoError::ErrorCode53628 => None,
            TwilioProgrammableVideoError::ErrorCode53405 => Some(r#"* The Client was unable to establish a media connection.
* A media connection which was active failed liveliness checks.
* The flow of media on the connection has ceased.
* The Participant declined permission to use the local network in a Peer to Peer Room."#),
            TwilioProgrammableVideoError::ErrorCode53000 => Some(r#"The signaling communication path could not be established or has been lost."#),
            TwilioProgrammableVideoError::ErrorCode53203 => Some(r#"* A client added more than the maximum number of Tracks allowed per Participant.
* A client added more than the maximum number of Tracks allowed per Room.
"#),
            TwilioProgrammableVideoError::ErrorCode53112 => None,
            TwilioProgrammableVideoError::ErrorCode53102 => None,
            TwilioProgrammableVideoError::ErrorCode53104 => None,
            TwilioProgrammableVideoError::ErrorCode53408 => Some(r#"* The client has attempted an ICE restart in a SDP answer."#),
            TwilioProgrammableVideoError::ErrorCode53106 => None,
            TwilioProgrammableVideoError::ErrorCode53616 => None,
            TwilioProgrammableVideoError::ErrorCode53301 => None,
            TwilioProgrammableVideoError::ErrorCode53300 => None,
            TwilioProgrammableVideoError::ErrorCode53601 => None,
            TwilioProgrammableVideoError::ErrorCode53620 => None,
            TwilioProgrammableVideoError::ErrorCode53627 => None,
            TwilioProgrammableVideoError::ErrorCode53215 => Some(r#"- An empty set of rules `[]` is not allowed
- A rule containing `all` must have a value of `true` and must not contain any other expression.
- A rule must contain a `type` field.
- If `kind` is used, it must be one of `audio`, `video` or `data`.
- A maximum of 20 rules are allowed per subscriber."#),
            TwilioProgrammableVideoError::ErrorCode53204 => None,
            TwilioProgrammableVideoError::ErrorCode53621 => None,
            TwilioProgrammableVideoError::ErrorCode53617 => Some(r#"There was an internal failure when processing your request. This action will not be retried."#),
            TwilioProgrammableVideoError::ErrorCode53208 => Some(r#"At least one of the bandwidth profile configuration parameters is invalid. This could mean out of range, missing entirely, etc."#),
            TwilioProgrammableVideoError::ErrorCode53205 => None,
            TwilioProgrammableVideoError::ErrorCode53109 => None,
            TwilioProgrammableVideoError::ErrorCode53661 => None,
            TwilioProgrammableVideoError::ErrorCode53120 => Some(r#"* An empty set of rules [] is not allowed.
* A rule containing all must have a value of true and must not contain any other expression.
* A rule must contain a type field.
* If kind is used, it must be audio or video.
* A maximum of 20 recording rules are allowed per room.
"#),
            TwilioProgrammableVideoError::ErrorCode53006 => Some(r#"The Video server is temporarily overloaded and cannot handle new requests."#),
            TwilioProgrammableVideoError::ErrorCode53002 => None,
            TwilioProgrammableVideoError::ErrorCode53207 => Some(r#" * REST API call to Create Room instance with MaxPublishedTracks out of range."#),
            TwilioProgrammableVideoError::ErrorCode53113 => Some(r#"You have already creasted a Room with the name you just used."#),
            TwilioProgrammableVideoError::ErrorCode53632 => None,
            TwilioProgrammableVideoError::ErrorCode53103 => Some(r#"* A user attempted to connect to a Room which does not yet exist, and Ad-hoc Rooms are not enabled.
* You may have chosen to enable Room features that are unsupported or not available to your account.
* A server-side error has occurred."#),
            TwilioProgrammableVideoError::ErrorCode53668 => Some(r#" * If you have specified a public key to cryptographically protect media tracks, this is most likely due to the fact that the credentials resource has been removed from Twilio."#),
            TwilioProgrammableVideoError::ErrorCode53001 => None,
            TwilioProgrammableVideoError::ErrorCode53206 => Some(r#"Connecting another Participant to a Room would exceed the account's concurrent Participants quota"#),
            TwilioProgrammableVideoError::ErrorCode53123 => Some(r#"MaxParticipantDuration is set to a value outside of the acceptable range"#),
            TwilioProgrammableVideoError::ErrorCode53626 => None,
            TwilioProgrammableVideoError::ErrorCode53664 => Some(r#" * If you have specified your own bucket, this is most likely due to the fact that the URL does not conform to an S3 URL."#),
            TwilioProgrammableVideoError::ErrorCode53202 => None,
            TwilioProgrammableVideoError::ErrorCode53304 => None,
            TwilioProgrammableVideoError::ErrorCode53622 => None,
            TwilioProgrammableVideoError::ErrorCode53669 => Some(r#"* If you have specified your own bucket and access credentials, this is most likely due to the fact that the configuration is out of date."#),
            TwilioProgrammableVideoError::ErrorCode53217 => Some(r#"The provided configuration for maxAudioTracks or maxVideoTracks is out of range"#),
            TwilioProgrammableVideoError::ErrorCode53501 => None,
            TwilioProgrammableVideoError::ErrorCode53603 => None,
            TwilioProgrammableVideoError::ErrorCode53110 => None,
            TwilioProgrammableVideoError::ErrorCode53500 => None,
            TwilioProgrammableVideoError::ErrorCode53663 => Some(r#"There was an internal failure when processing your request. This action will not be retried."#),
            TwilioProgrammableVideoError::ErrorCode53662 => None,
            TwilioProgrammableVideoError::ErrorCode53605 => None,
            TwilioProgrammableVideoError::ErrorCode53200 => None,
            TwilioProgrammableVideoError::ErrorCode53630 => None,
            TwilioProgrammableVideoError::ErrorCode53107 => None,
            TwilioProgrammableVideoError::ErrorCode53633 => Some(r#"Due to fixed hardware constraints, Twilio is not able to compose all the media tracks specified in the composition create request. Check the maximum size on <a href="https://www.twilio.com/docs/video/api/compositions-resource#known-issues">Known limitations</a>"#),
            TwilioProgrammableVideoError::ErrorCode53122 => Some(r#"REST API call requesting to record a Track in a peer-to-peer Room."#),
            TwilioProgrammableVideoError::ErrorCode53625 => None,
            TwilioProgrammableVideoError::ErrorCode53614 => None,
            TwilioProgrammableVideoError::ErrorCode53611 => None,
            TwilioProgrammableVideoError::ErrorCode53402 => None,
            TwilioProgrammableVideoError::ErrorCode53127 => Some(r#"Audio only flag is set for this account, which started using video after October 21, 2024"#),
            TwilioProgrammableVideoError::ErrorCode53216 => Some(r#"- The Participant\'s session length exceeds the MaxParticipantDuration value.
- MaxParticipantDuration can be set to a value (in seconds) between 10 minutes (600 seconds) and 24 hours (86400 seconds)."#),
            TwilioProgrammableVideoError::ErrorCode53101 => None,
            TwilioProgrammableVideoError::ErrorCode53108 => None,
            TwilioProgrammableVideoError::ErrorCode53613 => None,
            TwilioProgrammableVideoError::ErrorCode53400 => None,
            TwilioProgrammableVideoError::ErrorCode53125 => Some(r#"A participant tried to publish a video track in an Audio Only Group Room.
A participant tried to connect with a video track in an Audio Only Group Room."#),
            TwilioProgrammableVideoError::ErrorCode53111 => None,
            TwilioProgrammableVideoError::ErrorCode53607 => None,
            TwilioProgrammableVideoError::ErrorCode53003 => None,
            TwilioProgrammableVideoError::ErrorCode53406 => Some(r#"* The subscriber does not support data channels.
* The subscriber can not open a data channel at this moment.
* The subscriber rejected the data channel.
* ICE connection between the server and the client became disconnected."#),
            TwilioProgrammableVideoError::ErrorCode53667 => Some(r#" * If you have specified your own bucket and access credentials, this is most likely due to the fact that the credentials resource has been removed from Twilio."#),
            TwilioProgrammableVideoError::ErrorCode53004 => None,
            TwilioProgrammableVideoError::ErrorCode53623 => None,
            TwilioProgrammableVideoError::ErrorCode53302 => None,
            TwilioProgrammableVideoError::ErrorCode53118 => Some(r#"* The operation you requested cannot be performed on a completed Room."#),
            TwilioProgrammableVideoError::ErrorCode53660 => None,
            TwilioProgrammableVideoError::ErrorCode53666 => Some(r#" * If you have specified a public key to cryptographically protect media tracks, this is most likely due to the fact that the key value is not correct."#),
            TwilioProgrammableVideoError::ErrorCode53201 => None,
            TwilioProgrammableVideoError::ErrorCode53124 => Some(r#"The AudioOnly flag is set for a room that is not a Group Room."#),
            TwilioProgrammableVideoError::ErrorCode53407 => Some(r#"* One or both of the DTLS peers have an invalid certificate.
* One or both of the DTLS peers have an outdated version of DTLS.
* One or both of the DTLS peers lost internet connectivity while performing a DTLS handshake."#),
            TwilioProgrammableVideoError::ErrorCode53631 => None,
            TwilioProgrammableVideoError::ErrorCode53403 => Some(r#"Raised whenever the Video Media Server receives a remote media description but is unable to apply it."#),
            TwilioProgrammableVideoError::ErrorCode53105 => None,
            TwilioProgrammableVideoError::ErrorCode53303 => None,
            TwilioProgrammableVideoError::ErrorCode53610 => None,
            TwilioProgrammableVideoError::ErrorCode53126 => Some(r#"Room type other than Group Rooms has been specified for this account, which started using video after October 21, 2024"#),
            TwilioProgrammableVideoError::ErrorCode53602 => None,
            TwilioProgrammableVideoError::ErrorCode53119 => Some(r#" * Connecting the first Participant in an Ad-hoc Room would exceed the account's concurrent Rooms quota.
 * Creating a Room via the REST API would exceed the account's concurrent Rooms quota."#),
            TwilioProgrammableVideoError::ErrorCode53604 => None,
            TwilioProgrammableVideoError::ErrorCode53100 => None,
            TwilioProgrammableVideoError::ErrorCode53612 => None,
            TwilioProgrammableVideoError::ErrorCode53600 => None,
            TwilioProgrammableVideoError::ErrorCode53209 => Some(r#"REST API call to Update Participant instance with Status that is invalid."#),
            TwilioProgrammableVideoError::ErrorCode53624 => None,
            TwilioProgrammableVideoError::ErrorCode53121 => Some(r#" * Connecting the first Participant in an Ad-hoc Room might exceed the account's concurrent Rooms quota.
 * Creating a Room via the REST API might exceed the account's concurrent Rooms quota."#),
            TwilioProgrammableVideoError::ErrorCode53606 => None,
            TwilioProgrammableVideoError::ErrorCode53665 => Some(r#" * If you have specified your own bucket and access credentials, this is most likely due to the fact that the AWS Access Key ID and Secret Access Key are not correct."#),
            TwilioProgrammableVideoError::ErrorCode53615 => None
        }
    }
}
