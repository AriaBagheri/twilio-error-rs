// AUTO-GENERATED. DO NOT MODIFY. YOUR CHANGES WILL BE OVER-WRITTEN
use crate::products::TwilioConversationsError;
use standard_error::traits::StandardErrorFromCodeTrait;

impl StandardErrorFromCodeTrait for TwilioConversationsError {
    fn from_code(code: usize) -> Option<Self> {
        match code {
            50707 => Some(TwilioConversationsError::ErrorCode50707),
            50426 => Some(TwilioConversationsError::ErrorCode50426),
            50534 => Some(TwilioConversationsError::ErrorCode50534),
            50526 => Some(TwilioConversationsError::ErrorCode50526),
            50386 => Some(TwilioConversationsError::ErrorCode50386),
            50705 => Some(TwilioConversationsError::ErrorCode50705),
            50529 => Some(TwilioConversationsError::ErrorCode50529),
            50708 => Some(TwilioConversationsError::ErrorCode50708),
            50704 => Some(TwilioConversationsError::ErrorCode50704),
            50396 => Some(TwilioConversationsError::ErrorCode50396),
            50452 => Some(TwilioConversationsError::ErrorCode50452),
            50514 => Some(TwilioConversationsError::ErrorCode50514),
            50528 => Some(TwilioConversationsError::ErrorCode50528),
            50542 => Some(TwilioConversationsError::ErrorCode50542),
            50701 => Some(TwilioConversationsError::ErrorCode50701),
            50532 => Some(TwilioConversationsError::ErrorCode50532),
            50706 => Some(TwilioConversationsError::ErrorCode50706),
            50424 => Some(TwilioConversationsError::ErrorCode50424),
            50700 => Some(TwilioConversationsError::ErrorCode50700),
            50703 => Some(TwilioConversationsError::ErrorCode50703),
            50527 => Some(TwilioConversationsError::ErrorCode50527),
            50427 => Some(TwilioConversationsError::ErrorCode50427),
            50541 => Some(TwilioConversationsError::ErrorCode50541),
            50710 => Some(TwilioConversationsError::ErrorCode50710),
            50709 => Some(TwilioConversationsError::ErrorCode50709),
            50531 => Some(TwilioConversationsError::ErrorCode50531),
            50530 => Some(TwilioConversationsError::ErrorCode50530),
            50533 => Some(TwilioConversationsError::ErrorCode50533),
            50540 => Some(TwilioConversationsError::ErrorCode50540),
            50702 => Some(TwilioConversationsError::ErrorCode50702),
            50711 => Some(TwilioConversationsError::ErrorCode50711),
            50453 => Some(TwilioConversationsError::ErrorCode50453),
            50425 => Some(TwilioConversationsError::ErrorCode50425),
            _ => None
        }
    }
}
