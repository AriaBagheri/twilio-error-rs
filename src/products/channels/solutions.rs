// AUTO-GENERATED. DO NOT MODIFY. YOUR CHANGES WILL BE OVER-WRITTEN
use crate::products::TwilioChannelsError;
use standard_error::traits::StandardErrorSolutionsTrait;

impl StandardErrorSolutionsTrait for TwilioChannelsError {
    fn solutions(&self) -> Option<&'static str> {
        match self {
            TwilioChannelsError::ErrorCode63108 => Some(r#"Wait briefly before reattempting the action"#),
            TwilioChannelsError::ErrorCode63106 => Some(r#"Switch to a handset that supports RCS"#),
            TwilioChannelsError::ErrorCode63103 => Some(r#"* Create a new WABA to use with Twilio"#),
            TwilioChannelsError::ErrorCode63107 => Some(r#"Format the phone number that conforms to E.164 format"#),
            TwilioChannelsError::ErrorCode63101 => Some(r#"* Check that the WABA ID provided is correct
* Check that the SolutionID corresponding to Twilio's Partner Solution is correctly provided in the Embedded Signup implementation
* Check that this WABA has not been linked to other Twilio accounts you own. For security purposes, Twilio cannot provide you details if the WABA is linked to another Twilio Account that you do not own."#),
            TwilioChannelsError::ErrorCode63105 => Some(r#"- Open a support ticket to accomplish the desired action"#),
            TwilioChannelsError::ErrorCode63112 => Some(r#"1. Review Meta's Notification:
   - Action: Access your Meta (Facebook) account to view the specific reason for the ban.
   - How:
     - Log in to your [Meta Business](https://business.facebook.com/).
     - Navigate to the WhatsApp section to find notifications or alerts regarding your WABA status.

2. Submit an Appeal to Meta:
   - Action: If you believe the ban was a mistake or have rectified the issues, submit an appeal to Meta.
   - How:
     - Visit the [WhatsApp Business Help Center](https://www.whatsapp.com/business/help).
     - Follow the instructions to submit an appeal, providing necessary documentation and explanations.

3. Review and Comply with WhatsApp Policies:
   - Action: Ensure your business practices align with [WhatsApp Business Policies](https://www.whatsapp.com/business/api/policies/).
   - How:
     - Thoroughly read and understand the policies.
     - Update your messaging strategies to comply with guidelines. 

4. Complete the Verification Process:
   - Action: Ensure you have initiated the business verification process in your business settings.
   - How:
     - Log in to your [Meta Business](https://business.facebook.com/).
     - Navigate to Business Settings > Security Center.
     - Follow the prompts to start or continue the business verification process.

For more details, visit [Meta's Business Support Home](https://business.facebook.com/accountquality). If you require assistance from Twilio, please open a support ticket."#),
            TwilioChannelsError::ErrorCode63109 => Some(r#"If this was not intentional, visit the [Twilio Support Center](https://help.twilio.com/) to contact support."#),
            TwilioChannelsError::ErrorCode63100 => Some(r#"* Review the response's message parameter for additional details
* Verify the request body contains valid JSON according to the documentation"#),
            TwilioChannelsError::ErrorCode63111 => Some(r#"- Confirm the phone number and WABA still exist within Meta and are shared with Twilio
- Delete the Sender and reonboard."#),
            TwilioChannelsError::ErrorCode63102 => Some(r#"* Use the same WABA ID that is linked to the account
* Delete other WhatsApp Senders to link the Account to another WABA"#)
        }
    }
}
