// AUTO-GENERATED. DO NOT MODIFY. YOUR CHANGES WILL BE OVER-WRITTEN
use crate::products::TwilioBrandedCommsError;
use standard_error::traits::StandardErrorCausesTrait;

impl StandardErrorCausesTrait for TwilioBrandedCommsError {
    fn causes(&self) -> Option<&'static str> {
        match self {
            TwilioBrandedCommsError::ErrorCode60704 => Some(r#"- The origination number does not belong to the account
- The origination number is incorrect
- The origination number is not correctly branded, or linked to a business/brand
- The origination number is linked to a deactivated business/brand
- The origination number is linked to a business/brand that's not vetted, or the vetting was rejected"#),
            TwilioBrandedCommsError::ErrorCode60723 => Some(r#"You sent a POST request to update a brand that is in status "Pending Verification" or "Verified"."#),
            TwilioBrandedCommsError::ErrorCode60712 => Some(r#"Communication with the Regulatory Compliance API failed"#),
            TwilioBrandedCommsError::ErrorCode60719 => Some(r#"The phone number is not associated to a brand, or it is, but the brand or the business are no verified."#),
            TwilioBrandedCommsError::ErrorCode60721 => Some(r#"At least one of the phone numbers sent in the payload when creating/updating a Branded Channel are already associated with a different Branded Channel."#),
            TwilioBrandedCommsError::ErrorCode60727 => Some(r#"- The channel does not exist anymore
- The phone number sent is invalid/incorrect"#),
            TwilioBrandedCommsError::ErrorCode60706 => Some(r#"- The `PushToken` is iOS-incorrect or empty"#),
            TwilioBrandedCommsError::ErrorCode60709 => Some(r#"- Each Account can have only one Business Profile, and you're attempting to create another for the same Account"#),
            TwilioBrandedCommsError::ErrorCode60716 => Some(r#"Brand logo file format was not interpreted as PNG"#),
            TwilioBrandedCommsError::ErrorCode60702 => Some(r#"- The business does not exist anymore
- The `BusinessSid` parameter is invalid/incorrect"#),
            TwilioBrandedCommsError::ErrorCode60714 => Some(r#"The brand, or its associated business, may have been removed"#),
            TwilioBrandedCommsError::ErrorCode60713 => Some(r#"Communication with the Regulatory Identification API failed"#),
            TwilioBrandedCommsError::ErrorCode60707 => Some(r#"- The branded channel does not exist anymore
- The `BrandedChannelSid`, `BrandSid` or `BusinessSid` parameter is invalid/incorrect"#),
            TwilioBrandedCommsError::ErrorCode60710 => Some(r#"- The number queried is not branded by Twilio or any other CPS
- The service to discover CPS is having issues querying phone numbers"#),
            TwilioBrandedCommsError::ErrorCode60722 => Some(r#"You sent a dismiss request for a business that is in status "Draft" or "Rejected".
"#),
            TwilioBrandedCommsError::ErrorCode60703 => Some(r#"- The origination or destination number is missing the `+` sign, or has separators like spaces or dashes"#),
            TwilioBrandedCommsError::ErrorCode60700 => Some(r#"- Details included in the response body"#),
            TwilioBrandedCommsError::ErrorCode60725 => Some(r#"A brand must be verified in order to have any branded channels"#),
            TwilioBrandedCommsError::ErrorCode60715 => Some(r#"The file may be corrupted, or the file format is not expected (PNG is expected, max size 128kb)"#),
            TwilioBrandedCommsError::ErrorCode60717 => Some(r#"It may be caused by a temporal communication issue with the storage bucket."#),
            TwilioBrandedCommsError::ErrorCode60708 => Some(r#"- Our provider (Route 53) did not find the DNS record you're trying to update or delete"#),
            TwilioBrandedCommsError::ErrorCode60701 => Some(r#"- Details included in the response body"#),
            TwilioBrandedCommsError::ErrorCode60726 => Some(r#"A business must be verified in order to have any brands"#),
            TwilioBrandedCommsError::ErrorCode60724 => Some(r#"You sent a dismiss request for a brand that is in status "Draft" or "Rejected".
"#),
            TwilioBrandedCommsError::ErrorCode60711 => Some(r#"You sent a *POST* request to update a business that is in status "Pending Verification" or "Verified""#)
        }
    }
}
