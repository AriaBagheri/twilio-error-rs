// AUTO-GENERATED. DO NOT MODIFY. YOUR CHANGES WILL BE OVER-WRITTEN
use crate::products::TwilioVoiceIntelligenceError;
use standard_error::traits::StandardErrorDescriptionTrait;

impl StandardErrorDescriptionTrait for TwilioVoiceIntelligenceError {
    fn description(&self) -> Option<&'static str> {
        match self {
            TwilioVoiceIntelligenceError::ErrorCode95112 => Some(r#"The media sample rate is less than the minimum required to transcribe."#),
            TwilioVoiceIntelligenceError::ErrorCode95105 => Some(r#"Twilio failed to transcribe audio due to download failure. The transcription resource has been marked as “failed”"#),
            TwilioVoiceIntelligenceError::ErrorCode95201 => Some(r#"Twilio attempted to send a transcript event to the callback URL specified and your application didn’t respond before time out"#),
            TwilioVoiceIntelligenceError::ErrorCode95104 => Some(r#"Twilio failed to transcribe audio due to an invalid media type. The transcription resource has been marked as “failed”"#),
            TwilioVoiceIntelligenceError::ErrorCode95110 => Some(r#"Failed to download media file was not found"#),
            TwilioVoiceIntelligenceError::ErrorCode95115 => Some(r#"Validation of audio failed because it contains more than two channels."#),
            TwilioVoiceIntelligenceError::ErrorCode95116 => Some(r#"Voice Intelligence do not support PCI Recordings"#),
            TwilioVoiceIntelligenceError::ErrorCode95100 => Some(r#"Twilio failed to transcribe audio due to an internal issue.The resource has been marked as “failed”"#),
            TwilioVoiceIntelligenceError::ErrorCode95119 => Some(r#"Voice Intelligence does not support encrypted recordings"#),
            TwilioVoiceIntelligenceError::ErrorCode95108 => Some(r#"Twilio failed to delete resource"#),
            TwilioVoiceIntelligenceError::ErrorCode95114 => Some(r#"Validation of audio failed as it is longer than the allowed maximum duration (8h). Some STT providers have this limit in place."#),
            TwilioVoiceIntelligenceError::ErrorCode95200 => Some(r#"Twilio attempted to send a transcript event to the callback URL specified and your application returned a 4xx or 5xx or other HTTP response error"#),
            TwilioVoiceIntelligenceError::ErrorCode95118 => Some(r#"Voice Intelligence does not support Twilio Recordings with external storage"#),
            TwilioVoiceIntelligenceError::ErrorCode95111 => Some(r#"The media file is not accessible. The URL returned “Unauthorized” when trying to download."#),
            TwilioVoiceIntelligenceError::ErrorCode95302 => Some(r#"Voice Intelligence experienced an internal error."#),
            TwilioVoiceIntelligenceError::ErrorCode95300 => Some(r#"The Voice Intelligence Service specified was not found."#),
            TwilioVoiceIntelligenceError::ErrorCode95006 => Some(r#"Twilio failed to create a new transcript due to an empty media url"#),
            TwilioVoiceIntelligenceError::ErrorCode95005 => Some(r#"Twilio failed to create a new transcript due to an invalid media url"#),
            TwilioVoiceIntelligenceError::ErrorCode95102 => Some(r#"Twilio failed to transcribe audio due to an issue with the transcript recording. The transcript files have been deleted and the resource has been marked as “failed”
"#),
            TwilioVoiceIntelligenceError::ErrorCode95106 => Some(r#"Twilio failed to transcribe audio due to a third party speech-to-text engine failure. The transcription resource has been marked as “failed”."#),
            TwilioVoiceIntelligenceError::ErrorCode95109 => Some(r#"Failed to download, media content is empty."#),
            TwilioVoiceIntelligenceError::ErrorCode95250 => Some(r#"[The Predictive and Generative AI/ML Features Addendum](https://www.twilio.com/en-us/legal/ai-terms/predictive-generative-ai-features) has to be accepted."#),
            TwilioVoiceIntelligenceError::ErrorCode95301 => Some(r#"The language code specified in the "languageCode" property does not match the Voice Intelligence Service language of the service specified in the "intelligenceService" property."#),
            TwilioVoiceIntelligenceError::ErrorCode95113 => Some(r#"The media file is corrupted and could not be opened."#),
            TwilioVoiceIntelligenceError::ErrorCode95103 => Some(r#"Twilio failed to transcribe audio due to an issue with the audio file size.  The transcription files have been deleted and the resource has been marked as “failed”"#)
        }
    }
}
