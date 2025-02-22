// AUTO-GENERATED. DO NOT MODIFY. YOUR CHANGES WILL BE OVER-WRITTEN
use crate::products::TwilioVoiceIntelligenceError;
use standard_error::traits::StandardErrorDocsTrait;

impl StandardErrorDocsTrait for TwilioVoiceIntelligenceError {
    fn docs(&self) -> &'static str {
        match self {
            TwilioVoiceIntelligenceError::ErrorCode95112 => r#"## ERROR - 95112

### Invalid sample rate found on media

 The media sample rate is less than the minimum required to transcribe.

#### Possible Causes
The media file sample rate is less than 8KHz.

#### Possible Solutions
Use media with a higher sample rate."#,
            TwilioVoiceIntelligenceError::ErrorCode95105 => r#"## ERROR - 95105

### Failed to download audio file

 Twilio failed to transcribe audio due to download failure. The transcription resource has been marked as “failed”

#### Possible Causes
Timeout trying to download the file. Connection problem.

#### Possible Solutions
Ensure the file exist and accessible in the provided url"#,
            TwilioVoiceIntelligenceError::ErrorCode95201 => r#"## WARNING - 95201

### Status callback response timed out

 Twilio attempted to send a transcript event to the callback URL specified and your application didn’t respond before time out

#### Possible Causes
The server receiving status callbacks sent by Twilio takes too long to respond

#### Possible Solutions
Ensure that your callback server can handle the request and responds within a reasonable time"#,
            TwilioVoiceIntelligenceError::ErrorCode95104 => r#"## ERROR - 95104

### Invalid media type

 Twilio failed to transcribe audio due to an invalid media type. The transcription resource has been marked as “failed”

#### Possible Causes
Media type is not supported.

#### Possible Solutions
Convert the file into one of the supported media types: wav, mp3 or FLAC."#,
            TwilioVoiceIntelligenceError::ErrorCode95110 => r#"## ERROR - 95110

### Failed to download media: file was not found

 Failed to download media file was not found

#### Possible Causes
Media file was deleted.
Misspelled URL.

#### Possible Solutions
Verify the media file was not deleted and the URL is correct."#,
            TwilioVoiceIntelligenceError::ErrorCode95115 => r#"## ERROR - 95115

### Media has too many channels

 Validation of audio failed because it contains more than two channels.

#### Possible Causes
Multi channel audio containing more than 2 channels. It could come from a recording of a conference.

#### Possible Solutions
Remove unnecessary channels. Downmix extra channels into one."#,
            TwilioVoiceIntelligenceError::ErrorCode95116 => r#"## WARNING - 95116

### PCI Recordings are not supported

 Voice Intelligence do not support PCI Recordings

#### Possible Causes
The account is marked as PCI compliant.

#### Possible Solutions
Use the MediaUrl parameter instead of the SourceSid when creating a Transcription"#,
            TwilioVoiceIntelligenceError::ErrorCode95100 => r#"## ERROR - 95100

### Failed to transcribe

 Twilio failed to transcribe audio due to an internal issue.The resource has been marked as “failed”

#### Possible Causes
This is an internal issue.

#### Possible Solutions
Contact support"#,
            TwilioVoiceIntelligenceError::ErrorCode95119 => r#"## WARNING - 95119

### Voice Intelligence does not support encrypted recordings

 Voice Intelligence does not support encrypted recordings

#### Possible Causes
The account uses Voice Recording Encryption

#### Possible Solutions
Use decrypted recordings when creating a Transcription"#,
            TwilioVoiceIntelligenceError::ErrorCode95108 => r#"## ERROR - 95108

### Failed to delete resource

 Twilio failed to delete resource

#### Possible Causes
Resource was already deleted

#### Possible Solutions
Resource was already deleted. Failed to access S3"#,
            TwilioVoiceIntelligenceError::ErrorCode95114 => r#"## ERROR - 95114

### Media duration is too long.

 Validation of audio failed as it is longer than the allowed maximum duration (8h). Some STT providers have this limit in place.

#### Possible Causes
Media duration is longer than 8h.

#### Possible Solutions
Split audio into smaller pieces."#,
            TwilioVoiceIntelligenceError::ErrorCode95200 => r#"## WARNING - 95200

### Status callback response error

 Twilio attempted to send a transcript event to the callback URL specified and your application returned a 4xx or 5xx or other HTTP response error

#### Possible Causes
The callback URL specified is incorrect. Your application has an issue while handling callback requests or suffering and outage

#### Possible Solutions
Ensure that the callback URL specified is correct. Ensure that your callback server can handle the request and responds with a 2xx HTTP status code."#,
            TwilioVoiceIntelligenceError::ErrorCode95118 => r#"## WARNING - 95118

### Twilio Recordings with external storage are not supported

 Voice Intelligence does not support Twilio Recordings with external storage

#### Possible Causes
Twilio Recordings use external storage

#### Possible Solutions
Use the MediaUrl parameter instead of the SourceSid when creating a Transcription"#,
            TwilioVoiceIntelligenceError::ErrorCode95111 => r#"## ERROR - 95111

### Failed to download media file: unauthorized

 The media file is not accessible. The URL returned “Unauthorized” when trying to download.

#### Possible Causes
Protected resource.
Expired pre-signed URL.

#### Possible Solutions
Verify that the media url is not an expired pre-signed URL or ensure open access to the media."#,
            TwilioVoiceIntelligenceError::ErrorCode95302 => r#"## ERROR - 95302

### Voice Intelligence Error

 Voice Intelligence experienced an internal error.

#### Possible Causes
An issue with the Voice Intelligence Product caused an error.

#### Possible Solutions
Check the Twilio status page.
Report the error to Customer Service."#,
            TwilioVoiceIntelligenceError::ErrorCode95300 => r#"## ERROR - 95300

### Voice Intelligence Service was not found

 The Voice Intelligence Service specified was not found.

#### Possible Causes
The service specified in the "intelligenceService" property does not exist or has not been created under the account.


#### Possible Solutions
Verify that the service exists for the account in the Voice Intelligence product."#,
            TwilioVoiceIntelligenceError::ErrorCode95006 => r#"## ERROR - 95006

### Media url is empty

 Twilio failed to create a new transcript due to an empty media url

#### Possible Causes
Create transcript requests cannot have an empty media url if no recording is specified.

#### Possible Solutions
Provide a valid media url."#,
            TwilioVoiceIntelligenceError::ErrorCode95005 => r#"## ERROR - 95005

### Invalid media url

 Twilio failed to create a new transcript due to an invalid media url

#### Possible Causes
Provided media url is not valid

#### Possible Solutions
Change the request to use a valid URL format"#,
            TwilioVoiceIntelligenceError::ErrorCode95102 => r#"## ERROR - 95102

### Transcript media processing error

 Twilio failed to transcribe audio due to an issue with the transcript recording. The transcript files have been deleted and the resource has been marked as “failed”


#### Possible Causes
The recording was deleted.
The recording is not authorized to the given account.



#### Possible Solutions
Ensure the recording status is completed before starting to transcribe.
Ensure the recording was not deleted.
Ensure the recording is part of the given account."#,
            TwilioVoiceIntelligenceError::ErrorCode95106 => r#"## ERROR - 95106

### Speech-to-Text engine error

 Twilio failed to transcribe audio due to a third party speech-to-text engine failure. The transcription resource has been marked as “failed”.

#### Possible Causes
Transcript language is not the one specified in the service.

#### Possible Solutions
Ensure the transcript language matches the specified language of the service."#,
            TwilioVoiceIntelligenceError::ErrorCode95109 => r#"## ERROR - 95109

### Media content is empty

 Failed to download, media content is empty.

#### Possible Causes
Media content is empty. Media duration is less than 2 seconds.

#### Possible Solutions
Verify the media is not empty"#,
            TwilioVoiceIntelligenceError::ErrorCode95250 => r#"## WARNING - 95250

### AI/ML Features Addendum has not been accepted

 [The Predictive and Generative AI/ML Features Addendum](https://www.twilio.com/en-us/legal/ai-terms/predictive-generative-ai-features) has to be accepted.

#### Possible Causes
The Predictive and Generative AI/ML Features Addendum has not been accepted yet.

#### Possible Solutions
Accept the Predictive and Generative AI/ML Features Addendum on the [Voice Settings](https://console.twilio.com/us1/develop/voice/settings/general?frameUrl=%2Fconsole%2Fvoice%2Fsettings%3Fx-target-region%3Dus1) on Twilio Console."#,
            TwilioVoiceIntelligenceError::ErrorCode95301 => r#"## ERROR - 95301

### Language code on the Transcription request does not match the Voice Intelligence Service language

 The language code specified in the "languageCode" property does not match the Voice Intelligence Service language of the service specified in the "intelligenceService" property.

#### Possible Causes
The language defined in the Voice Intelligence Service is different from the one specified in the "languageCode" property.

#### Possible Solutions
Either:
- Change the "languageCode" property on the Transcription request to match the language defined in the specified Voice Intelligence service.
- Create a new Voice Intelligence service that matches the desired language and update the intelligenceService property on the Transcription request"#,
            TwilioVoiceIntelligenceError::ErrorCode95113 => r#"## ERROR - 95113

### Corrupted media provided

 The media file is corrupted and could not be opened.

#### Possible Causes
Corruption of data.
Error recording media.

#### Possible Solutions
Verify the media is not corrupted or use another media."#,
            TwilioVoiceIntelligenceError::ErrorCode95103 => r#"## ERROR - 95103

### Media file reached maximum size

 Twilio failed to transcribe audio due to an issue with the audio file size.  The transcription files have been deleted and the resource has been marked as “failed”

#### Possible Causes
The audio file size is larger than 3 GiB.

#### Possible Solutions
Use a different format that allows the file to be compressed, like mp3.
Split the file into shorter audio files and process them separately."#
        }
    }
}
