// AUTO-GENERATED. DO NOT MODIFY. YOUR CHANGES WILL BE OVER-WRITTEN
use crate::products::TwilioVoiceIntelligenceError;
use standard_error::traits::StandardErrorMessageTrait;
use std::borrow::Cow;

impl StandardErrorMessageTrait for TwilioVoiceIntelligenceError {
    fn message(&self) -> Cow<'static, str> {
        match self {
            TwilioVoiceIntelligenceError::ErrorCode95112 => r#"Invalid sample rate found on media"#.into(),
            TwilioVoiceIntelligenceError::ErrorCode95105 => r#"Failed to download audio file"#.into(),
            TwilioVoiceIntelligenceError::ErrorCode95201 => r#"Status callback response timed out"#.into(),
            TwilioVoiceIntelligenceError::ErrorCode95104 => r#"Invalid media type"#.into(),
            TwilioVoiceIntelligenceError::ErrorCode95110 => r#"Failed to download media: file was not found"#.into(),
            TwilioVoiceIntelligenceError::ErrorCode95115 => r#"Media has too many channels"#.into(),
            TwilioVoiceIntelligenceError::ErrorCode95116 => r#"PCI Recordings are not supported"#.into(),
            TwilioVoiceIntelligenceError::ErrorCode95100 => r#"Failed to transcribe"#.into(),
            TwilioVoiceIntelligenceError::ErrorCode95119 => r#"Voice Intelligence does not support encrypted recordings"#.into(),
            TwilioVoiceIntelligenceError::ErrorCode95108 => r#"Failed to delete resource"#.into(),
            TwilioVoiceIntelligenceError::ErrorCode95114 => r#"Media duration is too long."#.into(),
            TwilioVoiceIntelligenceError::ErrorCode95200 => r#"Status callback response error"#.into(),
            TwilioVoiceIntelligenceError::ErrorCode95118 => r#"Twilio Recordings with external storage are not supported"#.into(),
            TwilioVoiceIntelligenceError::ErrorCode95111 => r#"Failed to download media file: unauthorized"#.into(),
            TwilioVoiceIntelligenceError::ErrorCode95302 => r#"Voice Intelligence Error"#.into(),
            TwilioVoiceIntelligenceError::ErrorCode95300 => r#"Voice Intelligence Service was not found"#.into(),
            TwilioVoiceIntelligenceError::ErrorCode95006 => r#"Media url is empty"#.into(),
            TwilioVoiceIntelligenceError::ErrorCode95005 => r#"Invalid media url"#.into(),
            TwilioVoiceIntelligenceError::ErrorCode95102 => r#"Transcript media processing error"#.into(),
            TwilioVoiceIntelligenceError::ErrorCode95106 => r#"Speech-to-Text engine error"#.into(),
            TwilioVoiceIntelligenceError::ErrorCode95109 => r#"Media content is empty"#.into(),
            TwilioVoiceIntelligenceError::ErrorCode95250 => r#"AI/ML Features Addendum has not been accepted"#.into(),
            TwilioVoiceIntelligenceError::ErrorCode95301 => r#"Language code on the Transcription request does not match the Voice Intelligence Service language"#.into(),
            TwilioVoiceIntelligenceError::ErrorCode95113 => r#"Corrupted media provided"#.into(),
            TwilioVoiceIntelligenceError::ErrorCode95103 => r#"Media file reached maximum size"#.into()
        }
    }
}
