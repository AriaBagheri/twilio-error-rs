// AUTO-GENERATED. DO NOT MODIFY. YOUR CHANGES WILL BE OVER-WRITTEN
use crate::products::TwilioProgrammableVideoError;
use standard_error::traits::StandardErrorDescriptionTrait;

impl StandardErrorDescriptionTrait for TwilioProgrammableVideoError {
    fn description(&self) -> Option<&'static str> {
        match self {
            TwilioProgrammableVideoError::ErrorCode53628 => None,
            TwilioProgrammableVideoError::ErrorCode53405 => Some(r#"Raised by the Client or Server whenever a media connection fails or raised by the Client whenever it detects that media has stopped flowing."#),
            TwilioProgrammableVideoError::ErrorCode53000 => Some(r#"Raised whenever a signaling connection error occurs that is not covered by a more specific error code."#),
            TwilioProgrammableVideoError::ErrorCode53203 => Some(r#"The maximum number of published tracks allowed in the Room at the same time has been reached
"#),
            TwilioProgrammableVideoError::ErrorCode53112 => None,
            TwilioProgrammableVideoError::ErrorCode53102 => None,
            TwilioProgrammableVideoError::ErrorCode53104 => None,
            TwilioProgrammableVideoError::ErrorCode53408 => Some(r#"ICE restarts are only allowed on SDP offers, not answers"#),
            TwilioProgrammableVideoError::ErrorCode53106 => None,
            TwilioProgrammableVideoError::ErrorCode53616 => None,
            TwilioProgrammableVideoError::ErrorCode53301 => None,
            TwilioProgrammableVideoError::ErrorCode53300 => None,
            TwilioProgrammableVideoError::ErrorCode53601 => None,
            TwilioProgrammableVideoError::ErrorCode53620 => None,
            TwilioProgrammableVideoError::ErrorCode53627 => None,
            TwilioProgrammableVideoError::ErrorCode53215 => Some(r#"Raised by the server when a Subscribe Rule is invalid."#),
            TwilioProgrammableVideoError::ErrorCode53204 => None,
            TwilioProgrammableVideoError::ErrorCode53621 => None,
            TwilioProgrammableVideoError::ErrorCode53617 => Some(r#"Despite our best efforts, we have failed to delete some or all of the selected compositions. The operation has been cancelled."#),
            TwilioProgrammableVideoError::ErrorCode53208 => Some(r#"Participant's bandwidth profile configuration is invalid"#),
            TwilioProgrammableVideoError::ErrorCode53205 => None,
            TwilioProgrammableVideoError::ErrorCode53109 => None,
            TwilioProgrammableVideoError::ErrorCode53661 => None,
            TwilioProgrammableVideoError::ErrorCode53120 => Some(r#"Raised by the server when a Recording Rule is invalid."#),
            TwilioProgrammableVideoError::ErrorCode53006 => Some(r#"Raised when the server is too busy to accept new clients."#),
            TwilioProgrammableVideoError::ErrorCode53002 => None,
            TwilioProgrammableVideoError::ErrorCode53207 => Some(r#"### 53207: MaxPublishedTracks is out of range

Raised in the REST API when MaxPublishedTracks is set out of range."#),
            TwilioProgrammableVideoError::ErrorCode53113 => Some(r#"This error is raised when the Server is unable to create a Room because the name you have used is already associated with an existing Room.

The [Video API Room resource page](https://www.twilio.com/docs/video/api/rooms-resource#filter-by-unique-name) will show you how to get a list of existing Rooms’ names, allowing you to choose a new one that has not yet been used."#),
            TwilioProgrammableVideoError::ErrorCode53632 => None,
            TwilioProgrammableVideoError::ErrorCode53103 => Some(r#"Raised when the Server is unable to create a Room."#),
            TwilioProgrammableVideoError::ErrorCode53668 => Some(r#"The public key credentials configured in your account's composition settings could not be loaded.

Media composition has been marked as FAILED and will not be retried."#),
            TwilioProgrammableVideoError::ErrorCode53001 => None,
            TwilioProgrammableVideoError::ErrorCode53206 => Some(r#"### 53206: The concurrent Participants quota was exceeded"#),
            TwilioProgrammableVideoError::ErrorCode53123 => Some(r#"MaxParticipantDuration is set to a value outside of the acceptable range of 600 seconds (10 minutes) and 86400 seconds (24 hours)"#),
            TwilioProgrammableVideoError::ErrorCode53626 => None,
            TwilioProgrammableVideoError::ErrorCode53664 => Some(r#"The S3 bucket URL configured in your account's composition settings is not valid.

Media composition has been marked as FAILED and will not be retried."#),
            TwilioProgrammableVideoError::ErrorCode53202 => None,
            TwilioProgrammableVideoError::ErrorCode53304 => None,
            TwilioProgrammableVideoError::ErrorCode53622 => None,
            TwilioProgrammableVideoError::ErrorCode53669 => Some(r#"Failed to upload media to the external S3 bucket configured in your account's composition settings.

Media composition has been marked as FAILED and will not be retried."#),
            TwilioProgrammableVideoError::ErrorCode53217 => Some(r#"The configuration value provided for maxVideoTracks or maxAudioTracks in the bandwidthProfile options is out of range"#),
            TwilioProgrammableVideoError::ErrorCode53501 => None,
            TwilioProgrammableVideoError::ErrorCode53603 => None,
            TwilioProgrammableVideoError::ErrorCode53110 => None,
            TwilioProgrammableVideoError::ErrorCode53500 => None,
            TwilioProgrammableVideoError::ErrorCode53663 => Some(r#"Despite our best efforts, we have failed to delete some or all of the selected recordings. The operation has been cancelled."#),
            TwilioProgrammableVideoError::ErrorCode53662 => None,
            TwilioProgrammableVideoError::ErrorCode53605 => None,
            TwilioProgrammableVideoError::ErrorCode53200 => None,
            TwilioProgrammableVideoError::ErrorCode53630 => None,
            TwilioProgrammableVideoError::ErrorCode53107 => None,
            TwilioProgrammableVideoError::ErrorCode53633 => Some(r#"The composition job did not start because the combined size of the media sources passed to the composition is too large to process."#),
            TwilioProgrammableVideoError::ErrorCode53122 => Some(r#"The recording operation requested is not supported for the Room type"#),
            TwilioProgrammableVideoError::ErrorCode53625 => None,
            TwilioProgrammableVideoError::ErrorCode53614 => None,
            TwilioProgrammableVideoError::ErrorCode53611 => None,
            TwilioProgrammableVideoError::ErrorCode53402 => None,
            TwilioProgrammableVideoError::ErrorCode53127 => Some(r#"The Audio only flag is deprecated and only available to accounts that were active prior to October 21, 2024."#),
            TwilioProgrammableVideoError::ErrorCode53216 => Some(r#"Raised by the server when a Participant\'s session length exceeds the MaxParticipantDuration value."#),
            TwilioProgrammableVideoError::ErrorCode53101 => None,
            TwilioProgrammableVideoError::ErrorCode53108 => None,
            TwilioProgrammableVideoError::ErrorCode53613 => None,
            TwilioProgrammableVideoError::ErrorCode53400 => None,
            TwilioProgrammableVideoError::ErrorCode53125 => Some(r#"Raised whenever a participant tries to publish a track or connects with a track that is not supported by the Group Room."#),
            TwilioProgrammableVideoError::ErrorCode53111 => None,
            TwilioProgrammableVideoError::ErrorCode53607 => None,
            TwilioProgrammableVideoError::ErrorCode53003 => None,
            TwilioProgrammableVideoError::ErrorCode53406 => Some(r#"There was a problem while negotiating with the remote media connection and it was not able to complete the channel creation. This can mean that the subscriber does not support data channels or they can not create it at this moment. This can also happen if the data channel suffers connectivity problems once it was connected."#),
            TwilioProgrammableVideoError::ErrorCode53667 => Some(r#"The AWS credentials configured in your account's composition settings could not be loaded.

Media composition has been marked as FAILED and will not be retried."#),
            TwilioProgrammableVideoError::ErrorCode53004 => None,
            TwilioProgrammableVideoError::ErrorCode53623 => None,
            TwilioProgrammableVideoError::ErrorCode53302 => None,
            TwilioProgrammableVideoError::ErrorCode53118 => Some(r#"Raised whenever a Room is completed via the REST API."#),
            TwilioProgrammableVideoError::ErrorCode53660 => None,
            TwilioProgrammableVideoError::ErrorCode53666 => Some(r#"The public key configured in your account's composition settings is not valid.

Media composition has been marked as FAILED and will not be retried."#),
            TwilioProgrammableVideoError::ErrorCode53201 => None,
            TwilioProgrammableVideoError::ErrorCode53124 => Some(r#"Raised whenever a participant tries to set the AudioOnly flag for a Room type other than Group Rooms."#),
            TwilioProgrammableVideoError::ErrorCode53407 => Some(r#"There was a problem while negotiating with the remote DTLS peer. Therefore the Participant will not be able to publish or subscribe to Tracks."#),
            TwilioProgrammableVideoError::ErrorCode53631 => None,
            TwilioProgrammableVideoError::ErrorCode53403 => None,
            TwilioProgrammableVideoError::ErrorCode53105 => None,
            TwilioProgrammableVideoError::ErrorCode53303 => None,
            TwilioProgrammableVideoError::ErrorCode53610 => None,
            TwilioProgrammableVideoError::ErrorCode53126 => Some(r#"Go Rooms, P2P Rooms and Small Group Rooms are deprecated and only available to accounts that were active prior to October 21, 2024."#),
            TwilioProgrammableVideoError::ErrorCode53602 => None,
            TwilioProgrammableVideoError::ErrorCode53119 => Some(r#"#### 53119: The concurrent Rooms quota was exceeded"#),
            TwilioProgrammableVideoError::ErrorCode53604 => None,
            TwilioProgrammableVideoError::ErrorCode53100 => None,
            TwilioProgrammableVideoError::ErrorCode53612 => None,
            TwilioProgrammableVideoError::ErrorCode53600 => None,
            TwilioProgrammableVideoError::ErrorCode53209 => Some(r#"Raised in the REST API when Status is not valid."#),
            TwilioProgrammableVideoError::ErrorCode53624 => None,
            TwilioProgrammableVideoError::ErrorCode53121 => Some(r#"#### 53121: Reached 70% of room or participant concurrency quota"#),
            TwilioProgrammableVideoError::ErrorCode53606 => None,
            TwilioProgrammableVideoError::ErrorCode53665 => Some(r#"The AWS credentials configured in your account's composition settings are not valid.

Media composition has been marked as FAILED and will not be retried."#),
            TwilioProgrammableVideoError::ErrorCode53615 => None
        }
    }
}
