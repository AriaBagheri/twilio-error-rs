// AUTO-GENERATED. DO NOT MODIFY. YOUR CHANGES WILL BE OVER-WRITTEN
use crate::products::TwilioVoiceIntelligenceError;
use standard_error::traits::StandardErrorCausesTrait;

impl StandardErrorCausesTrait for TwilioVoiceIntelligenceError {
    fn causes(&self) -> Option<&'static str> {
        match self {
            TwilioVoiceIntelligenceError::ErrorCode95112 => Some(r#"The media file sample rate is less than 8KHz."#),
            TwilioVoiceIntelligenceError::ErrorCode95105 => Some(r#"Timeout trying to download the file. Connection problem."#),
            TwilioVoiceIntelligenceError::ErrorCode95201 => Some(r#"The server receiving status callbacks sent by Twilio takes too long to respond"#),
            TwilioVoiceIntelligenceError::ErrorCode95104 => Some(r#"Media type is not supported."#),
            TwilioVoiceIntelligenceError::ErrorCode95110 => Some(r#"Media file was deleted.
Misspelled URL."#),
            TwilioVoiceIntelligenceError::ErrorCode95115 => Some(r#"Multi channel audio containing more than 2 channels. It could come from a recording of a conference."#),
            TwilioVoiceIntelligenceError::ErrorCode95116 => Some(r#"The account is marked as PCI compliant."#),
            TwilioVoiceIntelligenceError::ErrorCode95100 => Some(r#"This is an internal issue."#),
            TwilioVoiceIntelligenceError::ErrorCode95119 => Some(r#"The account uses Voice Recording Encryption"#),
            TwilioVoiceIntelligenceError::ErrorCode95108 => Some(r#"Resource was already deleted"#),
            TwilioVoiceIntelligenceError::ErrorCode95114 => Some(r#"Media duration is longer than 8h."#),
            TwilioVoiceIntelligenceError::ErrorCode95200 => Some(r#"The callback URL specified is incorrect. Your application has an issue while handling callback requests or suffering and outage"#),
            TwilioVoiceIntelligenceError::ErrorCode95118 => Some(r#"Twilio Recordings use external storage"#),
            TwilioVoiceIntelligenceError::ErrorCode95111 => Some(r#"Protected resource.
Expired pre-signed URL."#),
            TwilioVoiceIntelligenceError::ErrorCode95302 => Some(r#"An issue with the Voice Intelligence Product caused an error."#),
            TwilioVoiceIntelligenceError::ErrorCode95300 => Some(r#"The service specified in the "intelligenceService" property does not exist or has not been created under the account.
"#),
            TwilioVoiceIntelligenceError::ErrorCode95006 => Some(r#"Create transcript requests cannot have an empty media url if no recording is specified."#),
            TwilioVoiceIntelligenceError::ErrorCode95005 => Some(r#"Provided media url is not valid"#),
            TwilioVoiceIntelligenceError::ErrorCode95102 => Some(r#"The recording was deleted.
The recording is not authorized to the given account.

"#),
            TwilioVoiceIntelligenceError::ErrorCode95106 => Some(r#"Transcript language is not the one specified in the service."#),
            TwilioVoiceIntelligenceError::ErrorCode95109 => Some(r#"Media content is empty. Media duration is less than 2 seconds."#),
            TwilioVoiceIntelligenceError::ErrorCode95250 => Some(r#"The Predictive and Generative AI/ML Features Addendum has not been accepted yet."#),
            TwilioVoiceIntelligenceError::ErrorCode95301 => Some(r#"The language defined in the Voice Intelligence Service is different from the one specified in the "languageCode" property."#),
            TwilioVoiceIntelligenceError::ErrorCode95113 => Some(r#"Corruption of data.
Error recording media."#),
            TwilioVoiceIntelligenceError::ErrorCode95103 => Some(r#"The audio file size is larger than 3 GiB."#)
        }
    }
}
