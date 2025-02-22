// AUTO-GENERATED. DO NOT MODIFY. YOUR CHANGES WILL BE OVER-WRITTEN
use crate::products::TwilioBrandedCommsError;
use standard_error::traits::StandardErrorFromCodeTrait;

impl StandardErrorFromCodeTrait for TwilioBrandedCommsError {
    fn from_code(code: usize) -> Option<Self> {
        match code {
            60704 => Some(TwilioBrandedCommsError::ErrorCode60704),
            60723 => Some(TwilioBrandedCommsError::ErrorCode60723),
            60712 => Some(TwilioBrandedCommsError::ErrorCode60712),
            60719 => Some(TwilioBrandedCommsError::ErrorCode60719),
            60721 => Some(TwilioBrandedCommsError::ErrorCode60721),
            60727 => Some(TwilioBrandedCommsError::ErrorCode60727),
            60706 => Some(TwilioBrandedCommsError::ErrorCode60706),
            60709 => Some(TwilioBrandedCommsError::ErrorCode60709),
            60716 => Some(TwilioBrandedCommsError::ErrorCode60716),
            60702 => Some(TwilioBrandedCommsError::ErrorCode60702),
            60714 => Some(TwilioBrandedCommsError::ErrorCode60714),
            60713 => Some(TwilioBrandedCommsError::ErrorCode60713),
            60707 => Some(TwilioBrandedCommsError::ErrorCode60707),
            60710 => Some(TwilioBrandedCommsError::ErrorCode60710),
            60722 => Some(TwilioBrandedCommsError::ErrorCode60722),
            60703 => Some(TwilioBrandedCommsError::ErrorCode60703),
            60700 => Some(TwilioBrandedCommsError::ErrorCode60700),
            60725 => Some(TwilioBrandedCommsError::ErrorCode60725),
            60715 => Some(TwilioBrandedCommsError::ErrorCode60715),
            60717 => Some(TwilioBrandedCommsError::ErrorCode60717),
            60708 => Some(TwilioBrandedCommsError::ErrorCode60708),
            60701 => Some(TwilioBrandedCommsError::ErrorCode60701),
            60726 => Some(TwilioBrandedCommsError::ErrorCode60726),
            60724 => Some(TwilioBrandedCommsError::ErrorCode60724),
            60711 => Some(TwilioBrandedCommsError::ErrorCode60711),
            _ => None
        }
    }
}
