// AUTO-GENERATED. DO NOT MODIFY. YOUR CHANGES WILL BE OVER-WRITTEN
use crate::products::TwilioProgrammableVideoError;
use standard_error::traits::StandardErrorDocsTrait;

impl StandardErrorDocsTrait for TwilioProgrammableVideoError {
    fn docs(&self) -> &'static str {
        match self {
            TwilioProgrammableVideoError::ErrorCode53628 => r#"## Error -  53628

### Room recordings deleted

The recordings for the room specified in your composition job have been deleted. Although the metadata for the recordings is present, the media has been removed from the Twilio Cloud and can't be recovered.

#### Causes

 * You have deleted your Room Recordings, either from the console or the API.

#### Solutions

 * There's no solution for this. Once the recorded media files are deleted from the system, they are gone forever."#,
            TwilioProgrammableVideoError::ErrorCode53405 => r#"## ERROR - 53405

### Media connection failed or Media activity ceased

 Raised by the Client or Server whenever a media connection fails or raised by the Client whenever it detects that media has stopped flowing.

#### Possible Causes
* The Client was unable to establish a media connection.
* A media connection which was active failed liveliness checks.
* The flow of media on the connection has ceased.
* The Participant declined permission to use the local network in a Peer to Peer Room.

#### Possible Solutions
* If the problem persists in a Group Room, try connecting to another region.
* Check your Client's network connectivity.
* Ensure that Network Traversal Service is enabled for your Configuration Profile and/or Room.
* If you've provided custom ICE Servers then ensure that the URLs and credentials are valid.
* Select LocalNetworkPrivacyPolicy.blockAll on iOS 14+ unless you have permission to use the local network."#,
            TwilioProgrammableVideoError::ErrorCode53000 => r#"## ERROR - 53000

### Signaling connection error

 Raised whenever a signaling connection error occurs that is not covered by a more specific error code.

#### Possible Causes
The signaling communication path could not be established or has been lost.

#### Possible Solutions
* Try connecting again.
* If the problem persists, try connecting to another region."#,
            TwilioProgrammableVideoError::ErrorCode53203 => r#"## ERROR - 53203

### The maximum number of published tracks allowed in the Room at the same time has been reached

 The maximum number of published tracks allowed in the Room at the same time has been reached


#### Possible Causes
* A client added more than the maximum number of Tracks allowed per Participant.
* A client added more than the maximum number of Tracks allowed per Room.


#### Possible Solutions
 * Have the Client remove one or more Tracks.
 * Have the Client publish Tracks only if the number of Tracks in a Room is less than the maximum number of Tracks allowed per Room."#,
            TwilioProgrammableVideoError::ErrorCode53112 => r#"### 53112: Status is invalid

Raised in the REST API when Status is not valid or the Room is not in-progress.

#### Causes

 * REST API call to Update Room instance with Status that is invalid or cannot be applied to the current Room state.

#### Solutions

 * Set Status to completed and only apply to a Room that is in-progress.
"#,
            TwilioProgrammableVideoError::ErrorCode53102 => r#"### 53102: Room name contains invalid characters

Raised whenever a Room name contains invalid characters.

#### Causes

 * The Room name contains an invalid UTF-8 byte sequence.

#### Solutions

 * Choose a Room name that is a valid UTF-8 string."#,
            TwilioProgrammableVideoError::ErrorCode53104 => r#"### 53104: Unable to connect to Room

Raised whenever a Client is unable to connect to a Room, and the scenario is not covered by a more specific error code.

#### Causes

 * A server-side error has occurred.

#### Solutions

 * If the problem persists, try connecting to another region."#,
            TwilioProgrammableVideoError::ErrorCode53408 => r#"## ERROR - 53408

### ICE connection restart was attempted, but it is not allowed

 ICE restarts are only allowed on SDP offers, not answers

#### Possible Causes
* The client has attempted an ICE restart in a SDP answer.

#### Possible Solutions
* Ensure that the ICE restart happens only in SDP offers."#,
            TwilioProgrammableVideoError::ErrorCode53106 => r#"### 53106: Room not found

Raised whenever attempting operation on a non-existent Room.

#### Causes

 * Attempted to perform operation on non-existent Room.

#### Solutions

 * Make sure Room exists before re-attempting operation."#,
            TwilioProgrammableVideoError::ErrorCode53616 => r#"## Warning - 53616

### Deleted a recording with custom configuration as time for retries was depleted

Twilio gave up on uploading your recording with custom configuration, as time for retries was depleted. The recording has been removed and will not be available.

#### Causes

 * There was an error with your account's recording settings, either due to the access credentials for your own bucket or to the encryption key.

#### Solutions

 * In the recording settings section of your account's console, review the credentials for accessing your S3 bucket or disable the external storage (media will be uploaded to the default bucket).

* In the recording settings section of your account's console, set a valid public key or disable the encryption (media will be uploaded in raw)."#,
            TwilioProgrammableVideoError::ErrorCode53301 => r#"### 53301: Track name is invalid

Raised whenever a Track name is invalid, and the scenario is not covered by a more specific error code.

#### Causes

 * The Track name does not adhere to the Track name requirements.

#### Solutions

 * Choose a Track name adheres to the Track name requirements.
"#,
            TwilioProgrammableVideoError::ErrorCode53300 => r#"### 53300: Track is invalid

Raised whenever a Track is invalid, and the scenario is not covered by a more specific error code."#,
            TwilioProgrammableVideoError::ErrorCode53601 => r#"## Error - 53601

### AWS credentials for recording upload are invalid

The recording will not be processed.

Raised when the AWS credentials provided in the recording's metadata for uploading the media to S3 are not valid.

#### Causes

 * Providing invalid AWS credentials for S3 access.

#### Solutions

 * Set AWS credentials to a correct value."#,
            TwilioProgrammableVideoError::ErrorCode53620 => r#"## Warning - 53620

### Invalid URL for external S3 bucket in composition settings

The S3 bucket URL configured in your account's composition settings is not valid.

The media will not be uploaded until the configuration has been fixed.

#### Causes

 * If you have specified your own bucket, this is most likely due to the fact that the URL does not conform to an S3 URL.

#### Solutions

 * In the composition settings section of your account's console, set a valid S3 URL for uploading media or disable the external storage (media will be uploaded to the default bucket)."#,
            TwilioProgrammableVideoError::ErrorCode53627 => r#"## Error - 53627

### Internal failure when updating the composition resource

Raised when an error prevented us from updating the resource tracking the media composition.

The media will not be accessible directly from Twilio until we remediate the situation but the composition media file has probably been stored safely to S3.

#### Causes

 * An internal error prevented us from updating a resource. Sorry!

#### Solutions

*   Please <a href="/help/contact">contact us</a> to figure out what has happened and how to fix it!
*   Note the time of the error and what you were trying to do when it occurred!"#,
            TwilioProgrammableVideoError::ErrorCode53215 => r#"## ERROR - 53215

### Invalid Subscribe Rule(s)

 Raised by the server when a Subscribe Rule is invalid.

#### Possible Causes
- An empty set of rules `[]` is not allowed
- A rule containing `all` must have a value of `true` and must not contain any other expression.
- A rule must contain a `type` field.
- If `kind` is used, it must be one of `audio`, `video` or `data`.
- A maximum of 20 rules are allowed per subscriber.

#### Possible Solutions
- Ensure the set of rules has at least one rule.
- If you include `all` in a rule, the only other allowed field is `type`.
- Ensure every rule has a `type` field.
- Ensure `kind` is one of the valid enum values.
- Limit the number of rules to 20 or fewer."#,
            TwilioProgrammableVideoError::ErrorCode53204 => r#"### 53204: Participant not found

Raised whenever attempting operation on a non-existent Participant.

#### Causes

 * Attempting operation on an non-existent Participant.

#### Solutions

 * Make sure participant exists before re-attempting operation."#,
            TwilioProgrammableVideoError::ErrorCode53621 => r#"## Warning - 53621

### Invalid AWS credentials to access external S3 bucket in composition settings

The AWS credentials configured in your account's composition settings are not valid.

The media will not be uploaded until the configuration has been fixed.

#### Causes

 * If you have specified your own bucket and access credentials, this is most likely due to the fact that the AWS Access Key ID and Secret Access Key are not correct.

#### Solutions

 * In the composition settings section of your account's console, set some valid credentials for accessing your S3 bucket or disable the external storage (media will be uploaded to the default bucket)."#,
            TwilioProgrammableVideoError::ErrorCode53617 => r#"## ERROR - 53617

### Internal failure when bulk deleting compositions from your account

 Despite our best efforts, we have failed to delete some or all of the selected compositions. The operation has been cancelled.

#### Possible Causes
There was an internal failure when processing your request. This action will not be retried.

#### Possible Solutions
* Please retry the delete operation with the same parameters.
* If the error persists, please <a href="/help/contact">contact us</a> to figure out what has happened and how to fix it!
* Note the time of the error and which filter was used to select the compositions."#,
            TwilioProgrammableVideoError::ErrorCode53208 => r#"## ERROR - 53208

### Participant's bandwidth profile configuration is invalid

 Participant's bandwidth profile configuration is invalid

#### Possible Causes
At least one of the bandwidth profile configuration parameters is invalid. This could mean out of range, missing entirely, etc.

#### Possible Solutions
* Check the bandwidth profile documentation and confirm that your configuration is valid."#,
            TwilioProgrammableVideoError::ErrorCode53205 => r#"### 53205: Participant disconnected because of duplicate identity

Raised by the server to the existing Participant when a new Participant joins a Room with the same identity as the existing Participant.

#### Causes

 * Using the same identity for more than one Room Participant.

#### Solutions

 * Make sure that each Participant joins a Room with a unique identity.

"#,
            TwilioProgrammableVideoError::ErrorCode53109 => r#"### 53109: Timeout is out of range

Raised in the REST API when Timeout is set out of range.

#### Causes

 * REST API call to Create or Update Room instance with Timeout out of range.

#### Solutions

 * Set the Timeout value (in seconds) from 1 second up to a maximum of 1 hour."#,
            TwilioProgrammableVideoError::ErrorCode53661 => r#"## Warning - 53661

### StatusCallbackMethod is invalid

The recording will be processed, but status notifications will not be sent.

Raised when StatusCallbackMethod is set to an invalid value.

#### Causes

 * Recording created with StatusCallbackMethod set to an invalid value.

#### Solutions

 * Set StatusCallbackMethod to either GET or POST or leave blank for the default (POST)."#,
            TwilioProgrammableVideoError::ErrorCode53120 => r#"## ERROR - 53120

###  Invalid Recording Rule(s)

 Raised by the server when a Recording Rule is invalid.

#### Possible Causes
* An empty set of rules [] is not allowed.
* A rule containing all must have a value of true and must not contain any other expression.
* A rule must contain a type field.
* If kind is used, it must be audio or video.
* A maximum of 20 recording rules are allowed per room.


#### Possible Solutions
* Ensure the set of rules has at least one rule.
* If you include all in a rule, the only other allowed field is type.
* Ensure every rule has a type field.
* Ensure kind is one of the valid enum values.
* Limit the number of recording rules to 20 or fewer."#,
            TwilioProgrammableVideoError::ErrorCode53006 => r#"## ERROR - 53006

### Video server is busy

 Raised when the server is too busy to accept new clients.

#### Possible Causes
The Video server is temporarily overloaded and cannot handle new requests.

#### Possible Solutions
* Try connecting later.
* If the problem persists, try connecting to another region."#,
            TwilioProgrammableVideoError::ErrorCode53002 => r#"### 53002: Signaling connection timed out

Raised whenever the signaling connection times out.
"#,
            TwilioProgrammableVideoError::ErrorCode53207 => r#"## ERROR - 53207

### MaxPublishedTracks is out of range

 ### 53207: MaxPublishedTracks is out of range

Raised in the REST API when MaxPublishedTracks is set out of range.

#### Possible Causes
 * REST API call to Create Room instance with MaxPublishedTracks out of range.

#### Possible Solutions
* Set the MaxPublishedTracks value from 0 (unlimited) up to the maximum allowed for the Room Type."#,
            TwilioProgrammableVideoError::ErrorCode53113 => r#"## ERROR - 53113

### Room creation failed

A Room with the same name already exists. This error is raised when the Server is unable to create a Room because the name you have used is already associated with an existing Room.

The [Video API Room resource page](https://www.twilio.com/docs/video/api/rooms-resource#filter-by-unique-name) will show you how to get a list of existing Rooms’ names, allowing you to choose a new one that has not yet been used.

#### Possible Causes
You have already creasted a Room with the name you just used.

#### Possible Solutions
Create a new Room with a unique name."#,
            TwilioProgrammableVideoError::ErrorCode53632 => r#"## Warning -  53632

### Failed to enqueue a room composition

There was a failure enqueuing a composition for processing.

#### Causes

 * There was an internal error when enqueuing the composition.

#### Solutions

* The composition can be launched again using the <a href="/docs/video/api/compositions-resource">REST API</a>.

* If the error persists, please <a href="/help/contact">contact us</a> to figure out what has happened and how to fix it!"#,
            TwilioProgrammableVideoError::ErrorCode53103 => r#"## ERROR - 53103

###  Unable to create Room

 Raised when the Server is unable to create a Room.

#### Possible Causes
* A user attempted to connect to a Room which does not yet exist, and Ad-hoc Rooms are not enabled.
* You may have chosen to enable Room features that are unsupported or not available to your account.
* A server-side error has occurred.

#### Possible Solutions
* Make sure Ad-hoc Rooms are enabled in the Video settings for your account in Twilio Console, or create the Room explicitly using the REST API before attempting to connect to it.
* Choose a Room configuration supported by your account.
* If the problem persists, try connecting to another region."#,
            TwilioProgrammableVideoError::ErrorCode53668 => r#"## ERROR - 53668

### Public key credentials for media tracks encryption could not be loaded

 The public key credentials configured in your account's composition settings could not be loaded.

Media composition has been marked as FAILED and will not be retried.

#### Possible Causes
 * If you have specified a public key to cryptographically protect media tracks, this is most likely due to the fact that the credentials resource has been removed from Twilio.

#### Possible Solutions
 * In the composition settings section of your account's console, create and assign a new valid public key or disable the encryption (media will be uploaded in raw)."#,
            TwilioProgrammableVideoError::ErrorCode53001 => r#"### 53001: Signaling connection disconnected

Raised whenever the signaling connection is unexpectedly disconnected.

#### Causes

 * The device running your application lost its internet connection.

#### Solutions

 * Ensure the device running your application has access to a stable internet connection.
"#,
            TwilioProgrammableVideoError::ErrorCode53206 => r#"## ERROR - 53206

### The Participant concurrency quota was exceeded

 ### 53206: The concurrent Participants quota was exceeded

#### Possible Causes
Connecting another Participant to a Room would exceed the account's concurrent Participants quota

#### Possible Solutions
 * Complete in-progress Rooms to disconnect Participants.
 * Disconnect connected Participants.
 * Contact support/sales to increase the concurrent Participants quota for your account."#,
            TwilioProgrammableVideoError::ErrorCode53123 => r#"## ERROR - 53123

### MaxParticipantDuration is out of range

 MaxParticipantDuration is set to a value outside of the acceptable range of 600 seconds (10 minutes) and 86400 seconds (24 hours)

#### Possible Causes
MaxParticipantDuration is set to a value outside of the acceptable range

#### Possible Solutions
Set MaxParticipantDuration to a value inside the acceptable range of 600 seconds (10 minutes) and 86400 seconds (24 hours)"#,
            TwilioProgrammableVideoError::ErrorCode53626 => r#"## Error - 53626

### Internal failure while processing media composition

Raised when an internal error prevents us from processing the media composition.

#### Causes

 * We screwed up. Sorry!

#### Solutions

*   If the error persists, please <a href="/help/contact">contact us</a> to figure out what has happened and how to fix it!
*   Note the time of the error and what you were trying to do when it occurred!"#,
            TwilioProgrammableVideoError::ErrorCode53664 => r#"## ERROR - 53664

### Invalid URL for external S3 bucket in composition settings

 The S3 bucket URL configured in your account's composition settings is not valid.

Media composition has been marked as FAILED and will not be retried.

#### Possible Causes
 * If you have specified your own bucket, this is most likely due to the fact that the URL does not conform to an S3 URL.

#### Possible Solutions
 * In the composition settings section of your account's console, set a valid S3 URL for uploading media or disable the external storage (media will be uploaded to the default bucket)."#,
            TwilioProgrammableVideoError::ErrorCode53202 => r#"### 53202: Participant identity contains invalid characters

Raised whenever a Participant identity contains invalid characters.

#### Causes

 * The Participant identity contains an invalid UTF-8 byte sequence.

#### Solutions

 * Choose a Participant identity that is a valid UTF-8 string."#,
            TwilioProgrammableVideoError::ErrorCode53304 => r#"### 53304: Track name is duplicated

Raised whenever a Participant is currently publishing a Track with the same name.

#### Causes

 * A Track with the same name already exists.

#### Solutions

 * Choose a Track name that is unique per Participant.
"#,
            TwilioProgrammableVideoError::ErrorCode53622 => r#"## Warning - 53622

### Invalid public key for media tracks encryption in composition settings

The public key configured in your account's composition settings is not valid.

The media will not be uploaded until the configuration has been fixed.

#### Causes

 * If you have specified a public key to cryptographically protect media tracks, this is most likely due to the fact that the key value is not correct.

#### Solutions

 * In the composition settings section of your account's console, set a valid public key or disable the encryption (media will be uploaded in raw)."#,
            TwilioProgrammableVideoError::ErrorCode53669 => r#"## ERROR - 53669

### Access denied to external S3 bucket configured in composition settings

 Failed to upload media to the external S3 bucket configured in your account's composition settings.

Media composition has been marked as FAILED and will not be retried.

#### Possible Causes
* If you have specified your own bucket and access credentials, this is most likely due to the fact that the configuration is out of date.

#### Possible Solutions
 * In the composition settings section of your account's console, review the credentials for accessing your S3 bucket or disable the external storage (media will be uploaded to the default bucket)."#,
            TwilioProgrammableVideoError::ErrorCode53217 => r#"## ERROR - 53217

### maxAudioTracks or maxVideoTracks configuration is out of range

 The configuration value provided for maxVideoTracks or maxAudioTracks in the bandwidthProfile options is out of range

#### Possible Causes
The provided configuration for maxAudioTracks or maxVideoTracks is out of range

#### Possible Solutions
Use a valid value for maxVideoTracks and maxAudioTracks"#,
            TwilioProgrammableVideoError::ErrorCode53501 => r#"### 53501: Unable to acquire TURN credentials

Raised whenever the Server is unable to return TURN credentials to the Client

#### Causes

* A server-side error has occurred.

#### Solutions

 * If the problem persists, try connecting to another region."#,
            TwilioProgrammableVideoError::ErrorCode53603 => r#"## Error - 53603

### Internal failure while processing a recording

Raised when an internal error prevents us from processing the recording.

#### Causes

 * We screwed up. Sorry!

#### Solutions

*   If the error persists, please <a href="/help/contact">contact us</a> to figure out what has happened and how to fix it!
*   Note the time of the error and what you were trying to do when it occurred!"#,
            TwilioProgrammableVideoError::ErrorCode53110 => r#"### 53110: StatusCallbackMethod is invalid

Raised in the REST API when StatusCallbackMethod is set to an invalid value.

#### Causes

 * REST API call to Create or Update Room instance with StatusCallbackMethod set to an invalid value.

#### Solutions

 * Set StatusCallbackMethod to either GET or POST or leave blank for the default."#,
            TwilioProgrammableVideoError::ErrorCode53500 => r#"### 53500: Unable to acquire configuration

Raised whenever the Client is unable to acquire configuration information from the Server.

#### Causes

 * The device running your application lost its Internet connection.
 * A server-side error has occurred.

#### Solutions

 * Ensure the device running your application has access to a stable internet connection. If the problem persists, try connecting to another region."#,
            TwilioProgrammableVideoError::ErrorCode53663 => r#"## ERROR - 53663

### Internal failure when bulk deleting recordings from your account

 Despite our best efforts, we have failed to delete some or all of the selected recordings. The operation has been cancelled.

#### Possible Causes
There was an internal failure when processing your request. This action will not be retried.

#### Possible Solutions
* Please retry the delete operation with the same parameters.
* If the error persists, please <a href="/help/contact">contact us</a> to figure out what has happened and how to fix it!
* Note the time of the error and which filter was used to select the recordings."#,
            TwilioProgrammableVideoError::ErrorCode53662 => r#"## Warning - 53662

### StatusCallback is invalid

The recording will be processed, but status notifications will not be sent.

Raised when StatusCallback is not a valid URL.

#### Causes

 * Recording created with StatusCallback set to an invalid URL.

#### Solutions

 * Set StatusCallback to a valid URL or leave blank (status notifications will not be sent)."#,
            TwilioProgrammableVideoError::ErrorCode53605 => r#"## Warning - 53605

### Internal failure when retrieving your account's recording settings

Raised when an error prevented us from retrieving your account's recording settings.

The recording will not be uploaded until we manage to read your recording settings.

#### Causes

* An internal error stopped us from uploading the recording. We'll attempt again shortly.

#### Solutions

* If the error persists, please <a href="/help/contact">contact us</a> to figure out what has happened and how to fix it!
* Note the time of the error and what you were trying to do when it occurred!"#,
            TwilioProgrammableVideoError::ErrorCode53200 => r#"### 53200: Participant identity is invalid

Raised whenever a Participant identity is invalid, and the scenario is not covered by a more specific error code.

#### Causes

The Participant identity does not adhere to the Participant identity requirements. 

Possible causes:

* The identity is null or an empty string.
* Another Participant with the same identity already exists in the Room.
* The identity has more than 128 characters (NOTE: Double-byte characters are counted as 2 characters).


#### Solutions

 * Choose a Participant identity adheres to the Participant identity requirements."#,
            TwilioProgrammableVideoError::ErrorCode53630 => r#"## Error -  53630

### Empty track list for composition

The combination of sources and exclusions passed to the composition resulted in an empty track list. The composition job did not start.

#### Causes

 * You specified an incorrect list of sources and exclusions, audio and video, which have resulted in an empty track list.

#### Solutions

 * Recreate the composition using an updated list of sources and exclusions."#,
            TwilioProgrammableVideoError::ErrorCode53107 => r#"### 53107: MaxParticipants is out of range

Raised in the REST API when MaxParticipants is set out of range.

#### Causes

 * REST API call to Create or Update Room instance with MaxParticipants out of range.

#### Solutions

 * Set the MaximumParticipants value from 1 up to the maximum allowed for the Room Type."#,
            TwilioProgrammableVideoError::ErrorCode53633 => r#"## ERROR - 53633

### Composition is too large

The composition job did not start. The composition job did not start because the combined size of the media sources passed to the composition is too large to process.

#### Possible Causes
Due to fixed hardware constraints, Twilio is not able to compose all the media tracks specified in the composition create request. Check the maximum size on <a href="https://www.twilio.com/docs/video/api/compositions-resource#known-issues">Known limitations</a>

#### Possible Solutions
Please reduce the scope of the composition by excluding some of the sources to verity that they are below our maximum size.
If not possible, please <a href="/help/contact">contact us</a> and provide a description of the media files you are attempting to compose."#,
            TwilioProgrammableVideoError::ErrorCode53122 => r#"## ERROR - 53122

### The recording operation requested is not supported for the Room type

 The recording operation requested is not supported for the Room type

#### Possible Causes
REST API call requesting to record a Track in a peer-to-peer Room.

#### Possible Solutions
Make sure that the recording operation requested is supported for the Room type."#,
            TwilioProgrammableVideoError::ErrorCode53625 => r#"## Warning - 53625

### Access denied to external S3 bucket configured in composition settings

Failed to upload media to the external S3 bucket configured in your account's composition settings.

The media will not be uploaded until the configuration has been fixed.

#### Causes

 * If you have specified your own bucket and access credentials, this is most likely due to the fact that the configuration is out of date.

#### Solutions

 * In the composition settings section of your account's console, review the credentials for accessing your S3 bucket or disable the external storage (media will be uploaded to the default bucket)."#,
            TwilioProgrammableVideoError::ErrorCode53614 => r#"## Warning - 53614

### Public key credentials for media tracks encryption could not be loaded

The public key credentials configured in your account's recording settings could not be loaded.

The media will not be uploaded until the configuration has been fixed.

#### Causes

 * If you have specified a public key to cryptographically protect raw media tracks, this is most likely due to the fact that the credentials resource has been removed from Twilio.

#### Solutions

 * In the recording settings section of your account's console, create and assign a new valid public key or disable the encryption (media will be uploaded in raw)."#,
            TwilioProgrammableVideoError::ErrorCode53611 => r#"## Warning - 53611

### Invalid AWS credentials to access external S3 bucket in recording settings

The AWS credentials configured in your account's recording settings are not valid.

The media will not be uploaded until the configuration has been fixed.

#### Causes

 * If you have specified your own bucket and access credentials, this is most likely due to the fact that the AWS Access Key ID and Secret Access Key are not correct.

#### Solutions

 * In the recording settings section of your account's console, set some valid credentials for accessing your S3 bucket or disable the external storage (media will be uploaded to the default bucket)."#,
            TwilioProgrammableVideoError::ErrorCode53402 => r#"### 53402: Client is unable to apply a remote media description

Raised whenever the Client receives a remote media description but is unable to apply it.

#### Causes

 * The Client may not be using a supported WebRTC implementation.
 * The Client may be connecting peer-to-peer with another Participant that is not using a supported WebRTC implementation.
 * The Client may not have the necessary resources to apply a new media description.

#### Solutions

 * If you are experiencing this error using the JavaScript SDK, ensure you are running it with a supported WebRTC implementation.
"#,
            TwilioProgrammableVideoError::ErrorCode53127 => r#"## ERROR - 53127

### audioOnly no longer supported

 The Audio only flag is deprecated and only available to accounts that were active prior to October 21, 2024.

#### Possible Causes
Audio only flag is set for this account, which started using video after October 21, 2024

#### Possible Solutions
Do not specify Audio only flag if account started using video after October 21, 2024"#,
            TwilioProgrammableVideoError::ErrorCode53216 => r#"## ERROR - 53216

### Participant session length exceeded

 Raised by the server when a Participant\'s session length exceeds the MaxParticipantDuration value.

#### Possible Causes
- The Participant\'s session length exceeds the MaxParticipantDuration value.
- MaxParticipantDuration can be set to a value (in seconds) between 10 minutes (600 seconds) and 24 hours (86400 seconds).

#### Possible Solutions
- Increase the MaxParticipantDuration configuration for Rooms (up to 24 hours) via REST API upon room creation or via the Video Settings page in Twilio Console to change the default value."#,
            TwilioProgrammableVideoError::ErrorCode53101 => r#"### 53101: Room name is too long

Raised whenever a Room name is too long.

#### Causes

 * The Room name is too long.

#### Solutions

 * Choose a shorter Room name."#,
            TwilioProgrammableVideoError::ErrorCode53108 => r#"### 53108: RoomType is not valid

Raised in the REST API when the user attempts to create a Room with an invalid RoomType

#### Causes

 * REST API call to Create Room instance with an invalid RoomType.

#### Solutions

 * Set the RoomType value to a valid type or leave this field blank for the default."#,
            TwilioProgrammableVideoError::ErrorCode53613 => r#"## Warning - 53613

### AWS credentials to access external S3 bucket could not be loaded

The AWS credentials configured in your account's recording settings could not be loaded.

The media will not be uploaded until the configuration has been fixed.

#### Causes

This is most likely due to the fact that the AWS Credentials resource has been removed from Twilio.

#### Solutions

 * In the recording settings section of your account's console, create and assign a new credentials entry for accessing your S3 bucket or disable the external storage (media will be uploaded to the default bucket)."#,
            TwilioProgrammableVideoError::ErrorCode53400 => r#"### 53400: Client is unable to create or apply a local media description

Raised whenever a Client is unable to create or apply a local media description.

#### Causes

 * The Client may not be using a supported WebRTC implementation.
 * The Client may not have the necessary resources to create or apply a new media description.

#### Solutions

 * If you are experiencing this error using the JavaScript SDK, ensure you are running it with a supported WebRTC implementation."#,
            TwilioProgrammableVideoError::ErrorCode53125 => r#"## ERROR - 53125

### The track kind is not supported by the Room

 Raised whenever a participant tries to publish a track or connects with a track that is not supported by the Group Room.

#### Possible Causes
A participant tried to publish a video track in an Audio Only Group Room.
A participant tried to connect with a video track in an Audio Only Group Room.

#### Possible Solutions
Make sure that only audio and/or data tracks are published in an Audio Only Group Room."#,
            TwilioProgrammableVideoError::ErrorCode53111 => r#"### 53111: StatusCallback is invalid

Raised in the REST API when StatusCallback is not a valid URL or the url is too long.

#### Causes

 * REST API call to Create or Update Room instance with StatusCallback set to an invalid URL.

#### Solutions

 * Set StatusCallback to a valid URL less than 1024 characters or leave blank to disable callbacks.
"#,
            TwilioProgrammableVideoError::ErrorCode53607 => r#"## Warning - 53607

### Internal failure when updating the recording resource

Raised when an error prevented us from updating an internal resource tracking the media recording.

The recording's media will not be accessible directly from Twilio until we remediate the situation but the media file has probably been stored safely to S3.

#### Causes

* An internal error prevented us from updating a resource. We'll attempt again shortly.

#### Solutions

* If the error persists, please <a href="/help/contact">contact us</a> to figure out what has happened and how to fix it!
* Note the time of the error and what you were trying to do when it occurred!"#,
            TwilioProgrammableVideoError::ErrorCode53003 => r#"### 53003: Client received an invalid signaling message

Raised whenever the Client receives a message from the Server that the Client cannot handle."#,
            TwilioProgrammableVideoError::ErrorCode53406 => r#"## ERROR - 53406

### The data channel used by the Data Track had a problem

 There was a problem while negotiating with the remote media connection and it was not able to complete the channel creation. This can mean that the subscriber does not support data channels or they can not create it at this moment. This can also happen if the data channel suffers connectivity problems once it was connected.

#### Possible Causes
* The subscriber does not support data channels.
* The subscriber can not open a data channel at this moment.
* The subscriber rejected the data channel.
* ICE connection between the server and the client became disconnected.

#### Possible Solutions
* Verify ICE connection.
* Verify that the subscriber supports data channels.
* Verify if the subscriber has reached the maximum number of data channels."#,
            TwilioProgrammableVideoError::ErrorCode53667 => r#"## ERROR - 53667

### AWS credentials to access external S3 bucket could not be loaded

 The AWS credentials configured in your account's composition settings could not be loaded.

Media composition has been marked as FAILED and will not be retried.

#### Possible Causes
 * If you have specified your own bucket and access credentials, this is most likely due to the fact that the credentials resource has been removed from Twilio.

#### Possible Solutions
 * In the composition settings section of your account's console, create and assign a new credentials entry for accessing your S3 bucket or disable the external storage (media will be uploaded to the default bucket)."#,
            TwilioProgrammableVideoError::ErrorCode53004 => r#"### 53004: Client sent an invalid signaling message

Raised whenever the Client sends a message to the Server that the Server cannot handle."#,
            TwilioProgrammableVideoError::ErrorCode53623 => r#"## Warning - 53623

### AWS credentials to access external S3 bucket could not be loaded

The AWS credentials configured in your account's composition settings could not be loaded.

The media will not be uploaded until the configuration has been fixed.

#### Causes

 * If you have specified your own bucket and access credentials, this is most likely due to the fact that the credentials resource has been removed from Twilio.

#### Solutions

 * In the composition settings section of your account's console, create and assign a new credentials entry for accessing your S3 bucket or disable the external storage (media will be uploaded to the default bucket)."#,
            TwilioProgrammableVideoError::ErrorCode53302 => r#"### 53302: Track name is too long

Raised whenever a Track name is too long.

#### Causes

 * The Track name is too long.

#### Solutions

 * Choose a shorter Track name.
"#,
            TwilioProgrammableVideoError::ErrorCode53118 => r#"## ERROR - 53118

### Room Completed Error

 Raised whenever a Room is completed via the REST API.

#### Possible Causes
* The operation you requested cannot be performed on a completed Room.

#### Possible Solutions
* Some operations can only be performed on in-progress Rooms. Check the status of your Room before attempting the operation."#,
            TwilioProgrammableVideoError::ErrorCode53660 => r#"## Warning - 53660

### Status callback response timed out

Raised by a Twilio Service when the StatusCallback request takes too long or does not respond to callbacks sent 

### Causes

 * The server receiving status callbacks sent by Twilio takes too long too respond.

### Solutions

 * Make sure service receiving callbacks does not block incoming requests."#,
            TwilioProgrammableVideoError::ErrorCode53666 => r#"## ERROR - 53666

### Invalid public key for media tracks encryption in composition settings

 The public key configured in your account's composition settings is not valid.

Media composition has been marked as FAILED and will not be retried.

#### Possible Causes
 * If you have specified a public key to cryptographically protect media tracks, this is most likely due to the fact that the key value is not correct.

#### Possible Solutions
 * In the composition settings section of your account's console, set a valid public key or disable the encryption (media will be uploaded in raw)."#,
            TwilioProgrammableVideoError::ErrorCode53201 => r#"### 53201: Participant identity is too long

Raised whenever a Participant identity is too long.

#### Causes

 * The Participant identity is too long.

#### Solutions

 * Choose a shorter Participant identity."#,
            TwilioProgrammableVideoError::ErrorCode53124 => r#"## ERROR - 53124

### The AudioOnly flag is not supported for the Room type

 Raised whenever a participant tries to set the AudioOnly flag for a Room type other than Group Rooms.

#### Possible Causes
The AudioOnly flag is set for a room that is not a Group Room.

#### Possible Solutions
Make sure that the flag is supported by the Room type."#,
            TwilioProgrammableVideoError::ErrorCode53407 => r#"## ERROR - 53407

### Media connection failed due to DTLS handshake failure

 There was a problem while negotiating with the remote DTLS peer. Therefore the Participant will not be able to publish or subscribe to Tracks.

#### Possible Causes
* One or both of the DTLS peers have an invalid certificate.
* One or both of the DTLS peers have an outdated version of DTLS.
* One or both of the DTLS peers lost internet connectivity while performing a DTLS handshake.

#### Possible Solutions
* Ensure that your certificate is valid.
* Ensure that you have a stable internet connection.
* Ensure that the browser or the Mobile SDK supports newer versions of DTLS."#,
            TwilioProgrammableVideoError::ErrorCode53631 => r#"## Warning -  53631

### Failed to enqueue a room composition from a configured composition hook

There was a failure creating or enqueuing a composition for a completed Group Room, using the configuration from a composition hook.

#### Causes

 * We failed to create the composition or the configuration was not suitable for a specific Group Room.

#### Solutions

* Verify the composition hook configured in your account's console.

* The composition can also be launched manually using the <a href="/docs/video/api/compositions-resource">REST API</a>.

* If the error persists, please <a href="/help/contact">contact us</a> to figure out what has happened and how to fix it!"#,
            TwilioProgrammableVideoError::ErrorCode53403 => r#"## ERROR - 53403

### Server is unable to apply a remote media description

 

#### Possible Causes
Raised whenever the Video Media Server receives a remote media description but is unable to apply it.

#### Possible Solutions
* Update to the latest Twilio Video SDK
* Use the latest browser version (if applicable)"#,
            TwilioProgrammableVideoError::ErrorCode53105 => r#"### 53105: Room contains too many Participants

Raised whenever a Client is unable to connect to a Room because the Room contains too many Participants.

#### Causes

 * A Client attempted to connect to a Room that already contained the maximum number of Participants.

#### Solutions

 * Have one or more Participants disconnect from the Room in order to make room for the Client to connect."#,
            TwilioProgrammableVideoError::ErrorCode53303 => r#"### 53303: Track name contains invalid characters

Raised whenever a Track name contains invalid characters.

#### Causes

 * The Track name contains an invalid UTF-8 byte sequence.

#### Solutions

 * Choose a Track name that is a valid UTF-8 string.
"#,
            TwilioProgrammableVideoError::ErrorCode53610 => r#"## Warning - 53610

### Invalid URL for external S3 bucket in recording settings

The S3 bucket URL configured in your account's recording settings is not valid.

The media will not be uploaded until the configuration has been fixed.

#### Causes

 * If you have specified your own bucket, this is most likely due to the fact that the URL does not conform to an S3 URL.

#### Solutions

 * In the recording settings section of your account's console, set a valid S3 URL for uploading media or disable the external storage (media will be uploaded to the default bucket)."#,
            TwilioProgrammableVideoError::ErrorCode53126 => r#"## ERROR - 53126

### Legacy room type no longer supported

 Go Rooms, P2P Rooms and Small Group Rooms are deprecated and only available to accounts that were active prior to October 21, 2024.

#### Possible Causes
Room type other than Group Rooms has been specified for this account, which started using video after October 21, 2024

#### Possible Solutions
Room type for accounts which started using video after October 21, 2024 must be Group Room"#,
            TwilioProgrammableVideoError::ErrorCode53602 => r#"## Error - 53602

### AWS encryption key for recording upload is invalid

The recording will not be processed.

Raised when the AWS encryption key provided in the recording's metadata for uploading the media to S3 is not valid.

#### Causes

 * Providing an invalid AWS encryption key for uploading to S3.

#### Solutions

 * Set AWS encryption key to a correct value."#,
            TwilioProgrammableVideoError::ErrorCode53119 => r#"## ERROR - 53119

### The concurrent Rooms quota was exceeded

 #### 53119: The concurrent Rooms quota was exceeded

#### Possible Causes
 * Connecting the first Participant in an Ad-hoc Room would exceed the account's concurrent Rooms quota.
 * Creating a Room via the REST API would exceed the account's concurrent Rooms quota.

#### Possible Solutions
* Complete existing Rooms.
* Contact support/sales to increase the concurrent Rooms quota for your account."#,
            TwilioProgrammableVideoError::ErrorCode53604 => r#"## Warning - 53604

### Failed to upload the recording to S3

Raised when an error prevented us from uploading the media to S3.

#### Causes

* Despite our best efforts to upload your media to S3, we couldn't do it this time. We'll attempt again shortly.

#### Solutions

* Please verify that any S3 configuration you've provided us is correct, such as AWS access credentials or AWS encryption key.
* If the error persists, please <a href="/help/contact">contact us</a> to figure out what has happened and how to fix it!
* Note the time of the error and what you were trying to do when it occurred!"#,
            TwilioProgrammableVideoError::ErrorCode53100 => r#"### 53100: Room name is invalid

Raised whenever a Room name is invalid, and the scenario is not covered by a more specific error code.

#### Causes

 * The Room name does not adhere to the Room name requirements.

#### Solutions

 * Choose a Room name adheres to the Room name requirements."#,
            TwilioProgrammableVideoError::ErrorCode53612 => r#"## Warning - 53612

### Invalid public key for media tracks encryption in recording settings

The public key configured in your account's recording settings is not valid.

The media will not be uploaded until the configuration has been fixed.

#### Causes

 * If you have specified a public key to cryptographically protect raw media tracks, this is most likely due to the fact that the key value is not correct.

#### Solutions

 * In the recording settings section of your account's console, set a valid public key or disable the encryption (media will be uploaded in raw)."#,
            TwilioProgrammableVideoError::ErrorCode53600 => r#"## Error - 53600

### S3 URL for recording upload is invalid

The recording will not be processed.

Raised when the S3 URL specified in the recording's metadata is not valid.

#### Causes

 * If you have specified your own bucket, this is most likely due to the fact that the URL does not conform to an S3 URL.

#### Solutions

 * Set a valid S3 URL for uploading media or leave blank (media will be uploaded to the default bucket)."#,
            TwilioProgrammableVideoError::ErrorCode53209 => r#"## ERROR - 53209

### Participant status is invalid

 Raised in the REST API when Status is not valid.

#### Possible Causes
REST API call to Update Participant instance with Status that is invalid.

#### Possible Solutions
Set Status to disconnected."#,
            TwilioProgrammableVideoError::ErrorCode53624 => r#"## Warning - 53624

### Public key credentials for media tracks encryption could not be loaded

The public key credentials configured in your account's composition settings could not be loaded.

The media will not be uploaded until the configuration has been fixed.

#### Causes

 * If you have specified a public key to cryptographically protect media tracks, this is most likely due to the fact that the credentials resource has been removed from Twilio.

#### Solutions

 * In the composition settings section of your account's console, create and assign a new valid public key or disable the encryption (media will be uploaded in raw)."#,
            TwilioProgrammableVideoError::ErrorCode53121 => r#"## WARNING - 53121

### Approaching room or participant concurrency quota

 #### 53121: Reached 70% of room or participant concurrency quota

#### Possible Causes
 * Connecting the first Participant in an Ad-hoc Room might exceed the account's concurrent Rooms quota.
 * Creating a Room via the REST API might exceed the account's concurrent Rooms quota.

#### Possible Solutions
* Complete existing Rooms.
* Contact support/sales to increase the concurrent Rooms quota for your account."#,
            TwilioProgrammableVideoError::ErrorCode53606 => r#"## Warning - 53606

### Internal failure when creating the recording resource

Raised when an error prevented us from creating an internal resource tracking the media recording.

The recording will not be accessible directly from Twilio until we remediate the situation but the media file has probably been stored safely to S3.

#### Causes

* An internal error prevented us from creating a resource. We'll attempt again shortly.

#### Solutions

* If the error persists, please <a href="/help/contact">contact us</a> to figure out what has happened and how to fix it!
* Note the time of the error and what you were trying to do when it occurred!"#,
            TwilioProgrammableVideoError::ErrorCode53665 => r#"## ERROR - 53665

### Invalid AWS credentials to access external S3 bucket in composition settings

 The AWS credentials configured in your account's composition settings are not valid.

Media composition has been marked as FAILED and will not be retried.

#### Possible Causes
 * If you have specified your own bucket and access credentials, this is most likely due to the fact that the AWS Access Key ID and Secret Access Key are not correct.

#### Possible Solutions
 * In the composition settings section of your account's console, set some valid credentials for accessing your S3 bucket or disable the external storage (media will be uploaded to the default bucket)."#,
            TwilioProgrammableVideoError::ErrorCode53615 => r#"## Warning - 53615

### Access denied to external S3 bucket configured in recording settings

We failed to upload media to the external S3 bucket configured in your account's recording settings.

The media will not be uploaded until the configuration has been fixed.

#### Causes
This is a configuration error that can have different root causes
   * The IAM user referenced in the credentials provided does not have enough permissions to upload to the bucket.
   * The bucket configured in your settings has SSE-S3 enabled. The only default encryption mechanism supported is SSE-KMS. Please review the guides for more informtion

#### Solutions

 * In the recording settings section of your account's console, review the credentials for accessing your S3 bucket or disable the external storage (media will be uploaded to the default bucket).
* If SSE-S3 is enabled, please consider switching to SSE-KMS."#
        }
    }
}
