// AUTO-GENERATED. DO NOT MODIFY. YOUR CHANGES WILL BE OVER-WRITTEN
use crate::products::TwilioVoiceIntelligenceError;
use standard_error::traits::StandardErrorFromCodeTrait;

impl StandardErrorFromCodeTrait for TwilioVoiceIntelligenceError {
    fn from_code(code: usize) -> Option<Self> {
        match code {
            95112 => Some(TwilioVoiceIntelligenceError::ErrorCode95112),
            95105 => Some(TwilioVoiceIntelligenceError::ErrorCode95105),
            95201 => Some(TwilioVoiceIntelligenceError::ErrorCode95201),
            95104 => Some(TwilioVoiceIntelligenceError::ErrorCode95104),
            95110 => Some(TwilioVoiceIntelligenceError::ErrorCode95110),
            95115 => Some(TwilioVoiceIntelligenceError::ErrorCode95115),
            95116 => Some(TwilioVoiceIntelligenceError::ErrorCode95116),
            95100 => Some(TwilioVoiceIntelligenceError::ErrorCode95100),
            95119 => Some(TwilioVoiceIntelligenceError::ErrorCode95119),
            95108 => Some(TwilioVoiceIntelligenceError::ErrorCode95108),
            95114 => Some(TwilioVoiceIntelligenceError::ErrorCode95114),
            95200 => Some(TwilioVoiceIntelligenceError::ErrorCode95200),
            95118 => Some(TwilioVoiceIntelligenceError::ErrorCode95118),
            95111 => Some(TwilioVoiceIntelligenceError::ErrorCode95111),
            95302 => Some(TwilioVoiceIntelligenceError::ErrorCode95302),
            95300 => Some(TwilioVoiceIntelligenceError::ErrorCode95300),
            95006 => Some(TwilioVoiceIntelligenceError::ErrorCode95006),
            95005 => Some(TwilioVoiceIntelligenceError::ErrorCode95005),
            95102 => Some(TwilioVoiceIntelligenceError::ErrorCode95102),
            95106 => Some(TwilioVoiceIntelligenceError::ErrorCode95106),
            95109 => Some(TwilioVoiceIntelligenceError::ErrorCode95109),
            95250 => Some(TwilioVoiceIntelligenceError::ErrorCode95250),
            95301 => Some(TwilioVoiceIntelligenceError::ErrorCode95301),
            95113 => Some(TwilioVoiceIntelligenceError::ErrorCode95113),
            95103 => Some(TwilioVoiceIntelligenceError::ErrorCode95103),
            _ => None
        }
    }
}
