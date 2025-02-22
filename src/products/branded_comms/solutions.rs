// AUTO-GENERATED. DO NOT MODIFY. YOUR CHANGES WILL BE OVER-WRITTEN
use crate::products::TwilioBrandedCommsError;
use standard_error::traits::StandardErrorSolutionsTrait;

impl StandardErrorSolutionsTrait for TwilioBrandedCommsError {
    fn solutions(&self) -> Option<&'static str> {
        match self {
            TwilioBrandedCommsError::ErrorCode60704 => Some(r#"- Make sure the origination number belongs to the account and is in the correct E.164 format
- Make sure the origination number is correctly linked to a business/brand, that's vetting-approved, and that's active"#),
            TwilioBrandedCommsError::ErrorCode60723 => Some(r#"If you really need to update the brand information, you can dismiss its current status (this action will update the status to "Draft") and then you can POST the update."#),
            TwilioBrandedCommsError::ErrorCode60712 => Some(r#"Wait a bit in case this is a temporal error, or else contact our support team"#),
            TwilioBrandedCommsError::ErrorCode60719 => Some(r#"The business and brand associated to the phone number being requested in the branded call must go through a vetting process and get a status of verified"#),
            TwilioBrandedCommsError::ErrorCode60721 => Some(r#"Use a different phone number in the Branded Channel payload, or remove the phone number from the existing Branded Channel if you really want to associate it with the Branded Channel being created/updated."#),
            TwilioBrandedCommsError::ErrorCode60727 => Some(r#"- Check the phone number in your request"#),
            TwilioBrandedCommsError::ErrorCode60706 => Some(r#"- Check the `PushToken` of the iOS device to register"#),
            TwilioBrandedCommsError::ErrorCode60709 => Some(r#"- Make sure you're using the Account intended to create the new Business Profile"#),
            TwilioBrandedCommsError::ErrorCode60716 => Some(r#"Make sure file format is PNG indeed"#),
            TwilioBrandedCommsError::ErrorCode60702 => Some(r#"- Check the `BusinessSid` parameter in your request, from Console"#),
            TwilioBrandedCommsError::ErrorCode60714 => Some(r#"Double check that the business SID and the brand SID are correct"#),
            TwilioBrandedCommsError::ErrorCode60713 => Some(r#"Wait a bit in case this is a temporal error, or else contact our support team"#),
            TwilioBrandedCommsError::ErrorCode60707 => Some(r#"- Check the Sids in your request"#),
            TwilioBrandedCommsError::ErrorCode60710 => Some(r#"- Check that the phone number is the same that you branded in Twilio or your CPS provider"#),
            TwilioBrandedCommsError::ErrorCode60722 => Some(r#"You're only interested in dismissing a business to update its information before being vetted by Twilio's vetting team, or if you want to remove its verified status.
To dismiss the business its status must be one of "Pending Verification" or "Verified"."#),
            TwilioBrandedCommsError::ErrorCode60703 => Some(r#"- Check the `From` and `To` numbers in your request"#),
            TwilioBrandedCommsError::ErrorCode60700 => Some(r#"- Details included in the response body"#),
            TwilioBrandedCommsError::ErrorCode60725 => Some(r#"Make sure your brand already has a verified status before attempting to create branded channels."#),
            TwilioBrandedCommsError::ErrorCode60715 => Some(r#"Make sure file format and size meet the criteria"#),
            TwilioBrandedCommsError::ErrorCode60717 => Some(r#"Try again later"#),
            TwilioBrandedCommsError::ErrorCode60708 => Some(r#"- Make sure the DNS record for that phone number exists in order to update or delete it"#),
            TwilioBrandedCommsError::ErrorCode60701 => Some(r#"- Details included in the response body"#),
            TwilioBrandedCommsError::ErrorCode60726 => Some(r#"Make sure your business already has a verified status before attempting to create brands"#),
            TwilioBrandedCommsError::ErrorCode60724 => Some(r#"You're only interested in dismissing a brand to update its information before being vetted by Twilio's vetting team, or if you want to remove its verified status.
To dismiss the brand its status must be one of "Pending Verification" or "Verified"."#),
            TwilioBrandedCommsError::ErrorCode60711 => Some(r#"If you really need to update the business information, you can dismiss its current status (this action will update the status to "Draft") and then you can POST the update "#)
        }
    }
}
