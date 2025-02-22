// AUTO-GENERATED. DO NOT MODIFY. YOUR CHANGES WILL BE OVER-WRITTEN
use crate::products::TwilioBrandedCommsError;
use standard_error::traits::StandardErrorMessageTrait;
use std::borrow::Cow;

impl StandardErrorMessageTrait for TwilioBrandedCommsError {
    fn message(&self) -> Cow<'static, str> {
        match self {
            TwilioBrandedCommsError::ErrorCode60704 => r#"Phone number not branded by Twilio"#.into(),
            TwilioBrandedCommsError::ErrorCode60723 => r#"Brand status does not allow updates"#.into(),
            TwilioBrandedCommsError::ErrorCode60712 => r#"Error communicating with Regulatory Compliance API"#.into(),
            TwilioBrandedCommsError::ErrorCode60719 => r#"Branded Call not found"#.into(),
            TwilioBrandedCommsError::ErrorCode60721 => r#"Phone Number(s) already used in a Branded Channel"#.into(),
            TwilioBrandedCommsError::ErrorCode60727 => r#"Channel not found"#.into(),
            TwilioBrandedCommsError::ErrorCode60706 => r#"Invalid Push Token"#.into(),
            TwilioBrandedCommsError::ErrorCode60709 => r#"Business Profile already exists"#.into(),
            TwilioBrandedCommsError::ErrorCode60716 => r#"Selected logo is not a valid PNG file"#.into(),
            TwilioBrandedCommsError::ErrorCode60702 => r#"Business Profile not found"#.into(),
            TwilioBrandedCommsError::ErrorCode60714 => r#"Brand not found"#.into(),
            TwilioBrandedCommsError::ErrorCode60713 => r#"Error communicating with Regulatory Identification API"#.into(),
            TwilioBrandedCommsError::ErrorCode60707 => r#"Branded Channel not found"#.into(),
            TwilioBrandedCommsError::ErrorCode60710 => r#"Phone number CPS not found"#.into(),
            TwilioBrandedCommsError::ErrorCode60722 => r#"Business status does not allow dismissal"#.into(),
            TwilioBrandedCommsError::ErrorCode60703 => r#"Invalid phone numbers format"#.into(),
            TwilioBrandedCommsError::ErrorCode60700 => r#"Something went wrong. Try again later"#.into(),
            TwilioBrandedCommsError::ErrorCode60725 => r#"Brand status does not allow to have branded channels"#.into(),
            TwilioBrandedCommsError::ErrorCode60715 => r#"Error reading logo file"#.into(),
            TwilioBrandedCommsError::ErrorCode60717 => r#"Error uploading logo to the storage"#.into(),
            TwilioBrandedCommsError::ErrorCode60708 => r#"Phone record number not found"#.into(),
            TwilioBrandedCommsError::ErrorCode60701 => r#"Invalid request"#.into(),
            TwilioBrandedCommsError::ErrorCode60726 => r#"Business status does not allow to have brands"#.into(),
            TwilioBrandedCommsError::ErrorCode60724 => r#"Brand status does not allow dismissal"#.into(),
            TwilioBrandedCommsError::ErrorCode60711 => r#"Business status does not allow updates"#.into()
        }
    }
}
