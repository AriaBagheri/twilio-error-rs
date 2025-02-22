// AUTO-GENERATED. DO NOT MODIFY. YOUR CHANGES WILL BE OVER-WRITTEN
use crate::products::TwilioVoiceIntelligenceError;
use standard_error::traits::StandardErrorSolutionsTrait;

impl StandardErrorSolutionsTrait for TwilioVoiceIntelligenceError {
    fn solutions(&self) -> Option<&'static str> {
        match self {
            TwilioVoiceIntelligenceError::ErrorCode95112 => Some(r#"Use media with a higher sample rate."#),
            TwilioVoiceIntelligenceError::ErrorCode95105 => Some(r#"Ensure the file exist and accessible in the provided url"#),
            TwilioVoiceIntelligenceError::ErrorCode95201 => Some(r#"Ensure that your callback server can handle the request and responds within a reasonable time"#),
            TwilioVoiceIntelligenceError::ErrorCode95104 => Some(r#"Convert the file into one of the supported media types: wav, mp3 or FLAC."#),
            TwilioVoiceIntelligenceError::ErrorCode95110 => Some(r#"Verify the media file was not deleted and the URL is correct."#),
            TwilioVoiceIntelligenceError::ErrorCode95115 => Some(r#"Remove unnecessary channels. Downmix extra channels into one."#),
            TwilioVoiceIntelligenceError::ErrorCode95116 => Some(r#"Use the MediaUrl parameter instead of the SourceSid when creating a Transcription"#),
            TwilioVoiceIntelligenceError::ErrorCode95100 => Some(r#"Contact support"#),
            TwilioVoiceIntelligenceError::ErrorCode95119 => Some(r#"Use decrypted recordings when creating a Transcription"#),
            TwilioVoiceIntelligenceError::ErrorCode95108 => Some(r#"Resource was already deleted. Failed to access S3"#),
            TwilioVoiceIntelligenceError::ErrorCode95114 => Some(r#"Split audio into smaller pieces."#),
            TwilioVoiceIntelligenceError::ErrorCode95200 => Some(r#"Ensure that the callback URL specified is correct. Ensure that your callback server can handle the request and responds with a 2xx HTTP status code."#),
            TwilioVoiceIntelligenceError::ErrorCode95118 => Some(r#"Use the MediaUrl parameter instead of the SourceSid when creating a Transcription"#),
            TwilioVoiceIntelligenceError::ErrorCode95111 => Some(r#"Verify that the media url is not an expired pre-signed URL or ensure open access to the media."#),
            TwilioVoiceIntelligenceError::ErrorCode95302 => Some(r#"Check the Twilio status page.
Report the error to Customer Service."#),
            TwilioVoiceIntelligenceError::ErrorCode95300 => Some(r#"Verify that the service exists for the account in the Voice Intelligence product."#),
            TwilioVoiceIntelligenceError::ErrorCode95006 => Some(r#"Provide a valid media url."#),
            TwilioVoiceIntelligenceError::ErrorCode95005 => Some(r#"Change the request to use a valid URL format"#),
            TwilioVoiceIntelligenceError::ErrorCode95102 => Some(r#"Ensure the recording status is completed before starting to transcribe.
Ensure the recording was not deleted.
Ensure the recording is part of the given account."#),
            TwilioVoiceIntelligenceError::ErrorCode95106 => Some(r#"Ensure the transcript language matches the specified language of the service."#),
            TwilioVoiceIntelligenceError::ErrorCode95109 => Some(r#"Verify the media is not empty"#),
            TwilioVoiceIntelligenceError::ErrorCode95250 => Some(r#"Accept the Predictive and Generative AI/ML Features Addendum on the [Voice Settings](https://console.twilio.com/us1/develop/voice/settings/general?frameUrl=%2Fconsole%2Fvoice%2Fsettings%3Fx-target-region%3Dus1) on Twilio Console."#),
            TwilioVoiceIntelligenceError::ErrorCode95301 => Some(r#"Either:
- Change the "languageCode" property on the Transcription request to match the language defined in the specified Voice Intelligence service.
- Create a new Voice Intelligence service that matches the desired language and update the intelligenceService property on the Transcription request"#),
            TwilioVoiceIntelligenceError::ErrorCode95113 => Some(r#"Verify the media is not corrupted or use another media."#),
            TwilioVoiceIntelligenceError::ErrorCode95103 => Some(r#"Use a different format that allows the file to be compressed, like mp3.
Split the file into shorter audio files and process them separately."#)
        }
    }
}
