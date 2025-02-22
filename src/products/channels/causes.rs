// AUTO-GENERATED. DO NOT MODIFY. YOUR CHANGES WILL BE OVER-WRITTEN
use crate::products::TwilioChannelsError;
use standard_error::traits::StandardErrorCausesTrait;

impl StandardErrorCausesTrait for TwilioChannelsError {
    fn causes(&self) -> Option<&'static str> {
        match self {
            TwilioChannelsError::ErrorCode63108 => Some(r#"Sender is not ready to add test device"#),
            TwilioChannelsError::ErrorCode63106 => Some(r#"The handset associated with the phone number provided is not RCS capable"#),
            TwilioChannelsError::ErrorCode63103 => Some(r#"* Using a WABA currently assigned to another payment method
* Using a WABA that has been used with another provider"#),
            TwilioChannelsError::ErrorCode63107 => Some(r#"phone_number provided is not a valid E.164 formatted phone number"#),
            TwilioChannelsError::ErrorCode63101 => Some(r#"* The WABA has been deleted
* The WABA has not been shared with Twilio
* The WABA has already been linked to another Twilio account"#),
            TwilioChannelsError::ErrorCode63105 => Some(r#"- API Request for unsupported functionality on the given channel"#),
            TwilioChannelsError::ErrorCode63112 => Some(r#"1. Policy Violations:
   - Details: Your WABA may have violated Meta's [WhatsApp Business Policies](https://www.whatsapp.com/business/api/policies/), such as sending unsolicited messages, using prohibited content, or failing to comply with data privacy regulations. 

2. Spam or Unsolicited Messaging:
   - Details: Sending a high volume of unsolicited messages or spam can lead to account suspension to protect user experience.

3. Inappropriate Content:
   - Details: Sharing content that violates WhatsApp's guidelines, including offensive, illegal, or harmful material.

4. User Reports:
   - Details: Receiving multiple reports from users about inappropriate behavior or unwanted messages from your account.

5  Incomplete Verification Submission:
   - Details: Required documents or information were not fully provided during the verification process."#),
            TwilioChannelsError::ErrorCode63109 => Some(r#"The Sender was onboarded to a different account."#),
            TwilioChannelsError::ErrorCode63100 => Some(r#"* Request body was empty
* Request body was not valid JSON
* Request body failed other validation rules"#),
            TwilioChannelsError::ErrorCode63111 => Some(r#"1. Phone number was deleted from the WABA linked to this account.
2. WABA linked to this account was deleted.
3. WABA was unshared with Twilio"#),
            TwilioChannelsError::ErrorCode63102 => Some(r#"* Attempting to use a different WABA ID than the one linked to the Twilio Account"#)
        }
    }
}
