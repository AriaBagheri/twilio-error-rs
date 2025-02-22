// AUTO-GENERATED. DO NOT MODIFY. YOUR CHANGES WILL BE OVER-WRITTEN
use crate::products::TwilioBrandedCommsError;
use standard_error::traits::StandardErrorDescriptionTrait;

impl StandardErrorDescriptionTrait for TwilioBrandedCommsError {
    fn description(&self) -> Option<&'static str> {
        match self {
            TwilioBrandedCommsError::ErrorCode60704 => Some(r#"The origination number (`From`) used was not found as a valid branded phone number"#),
            TwilioBrandedCommsError::ErrorCode60723 => Some(r#"Brands can only be updated when they have certain status values."#),
            TwilioBrandedCommsError::ErrorCode60712 => Some(r#"KYC, part of the Branded Comms product, depends on Twilio's regulatory APIs"#),
            TwilioBrandedCommsError::ErrorCode60719 => Some(r#"You are requesting a branded call for phone number that is not branded (it is not associated to a verified brand and business)"#),
            TwilioBrandedCommsError::ErrorCode60721 => Some(r#"When you create/update a Branded Channel resource, you send a list of your phone numbers to be associated with it. You cannot have a phone number associated with more than one Branded Channel."#),
            TwilioBrandedCommsError::ErrorCode60727 => Some(r#"Returned when looking for a channel with no match in the storage"#),
            TwilioBrandedCommsError::ErrorCode60706 => Some(r#"Registering a new device, the Push Token is empty or invalid"#),
            TwilioBrandedCommsError::ErrorCode60709 => Some(r#"This Account already has a Business Profile"#),
            TwilioBrandedCommsError::ErrorCode60716 => Some(r#"The file sent for the brand logo image is not the expected, PNG"#),
            TwilioBrandedCommsError::ErrorCode60702 => Some(r#"The Business was not found"#),
            TwilioBrandedCommsError::ErrorCode60714 => Some(r#"You are asking for a brand that does not exist for the business"#),
            TwilioBrandedCommsError::ErrorCode60713 => Some(r#"KYC, part of the Branded Comms product, depends on Twilio's regulatory APIs"#),
            TwilioBrandedCommsError::ErrorCode60707 => Some(r#"Branded Channel not found"#),
            TwilioBrandedCommsError::ErrorCode60710 => Some(r#"This phone number was not found on the Call Placement Service (CPS) Directory"#),
            TwilioBrandedCommsError::ErrorCode60722 => Some(r#"Businesses (AKA Business Profiles) can only be dismissed when they have certain status values. Dismissing a business will set its status to "Draft"."#),
            TwilioBrandedCommsError::ErrorCode60703 => Some(r#"The origination or destination numbers are not valid numbers in E.164 format"#),
            TwilioBrandedCommsError::ErrorCode60700 => Some(r#"Server error, generic 500 error"#),
            TwilioBrandedCommsError::ErrorCode60725 => Some(r#"You are creating a branded channel for a brand in a status does not allow it to have branded channels"#),
            TwilioBrandedCommsError::ErrorCode60715 => Some(r#"The file sent for the brand logo image could not be read"#),
            TwilioBrandedCommsError::ErrorCode60717 => Some(r#"The file sent for the brand logo image could not be stored in the storage bucket"#),
            TwilioBrandedCommsError::ErrorCode60708 => Some(r#"DNS Record for that phone number not found"#),
            TwilioBrandedCommsError::ErrorCode60701 => Some(r#"Bad Request, generic 400 error"#),
            TwilioBrandedCommsError::ErrorCode60726 => Some(r#"You are creating a brand for a business in a status does not allow it to have brands"#),
            TwilioBrandedCommsError::ErrorCode60724 => Some(r#"Brands can only be dismissed when they have certain status values. Dismissing a brand will set its status to "Draft"."#),
            TwilioBrandedCommsError::ErrorCode60711 => Some(r#"Businesses (AKA Business Profiles) can only be updated when they have certain status values. "#)
        }
    }
}
