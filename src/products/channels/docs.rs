// AUTO-GENERATED. DO NOT MODIFY. YOUR CHANGES WILL BE OVER-WRITTEN
use crate::products::TwilioChannelsError;
use standard_error::traits::StandardErrorDocsTrait;

impl StandardErrorDocsTrait for TwilioChannelsError {
    fn docs(&self) -> &'static str {
        match self {
            TwilioChannelsError::ErrorCode63108 => r#"## ERROR - 63108

### Sender is not ready to add test device

 Sender is not ready to add test device

#### Possible Causes
Sender is not ready to add test device

#### Possible Solutions
Wait briefly before reattempting the action"#,
            TwilioChannelsError::ErrorCode63106 => r#"## ERROR - 63106

### phone_number is not a RCS capable number

 The handset associated with the phone number provided is not RCS capable

#### Possible Causes
The handset associated with the phone number provided is not RCS capable

#### Possible Solutions
Switch to a handset that supports RCS"#,
            TwilioChannelsError::ErrorCode63103 => r#"## ERROR - 63103

### Cannot assign payment method to WABA provided

 The WhatsApp Business Account (WABA) provided is unable to use Twilio's credit line. Due to technical limitations within Meta's systems, once a payment method is added to a WABA it can only be revoked but never fully removed.

#### Possible Causes
* Using a WABA currently assigned to another payment method
* Using a WABA that has been used with another provider

#### Possible Solutions
* Create a new WABA to use with Twilio"#,
            TwilioChannelsError::ErrorCode63107 => r#"## ERROR - 63107

### phone_number must be a valid E.164 formatted phone number

 phone_number provided is not a valid E.164 formatted phone number

#### Possible Causes
phone_number provided is not a valid E.164 formatted phone number

#### Possible Solutions
Format the phone number that conforms to E.164 format"#,
            TwilioChannelsError::ErrorCode63101 => r#"## ERROR - 63101

### WABA ID provided is not valid or unable to be used

 Twilio cannot access the WhatsApp Business Account (WABA) with the ID provided.

#### Possible Causes
* The WABA has been deleted
* The WABA has not been shared with Twilio
* The WABA has already been linked to another Twilio account

#### Possible Solutions
* Check that the WABA ID provided is correct
* Check that the SolutionID corresponding to Twilio's Partner Solution is correctly provided in the Embedded Signup implementation
* Check that this WABA has not been linked to other Twilio accounts you own. For security purposes, Twilio cannot provide you details if the WABA is linked to another Twilio Account that you do not own."#,
            TwilioChannelsError::ErrorCode63105 => r#"## ERROR - 63105

### Channel does not support this action

 Channel does not currently support the requested action

#### Possible Causes
- API Request for unsupported functionality on the given channel

#### Possible Solutions
- Open a support ticket to accomplish the desired action"#,
            TwilioChannelsError::ErrorCode63112 => r#"## ERROR - 63112

### The Meta and/or WhatsApp Business Accounts connected to this Sender were disabled by Meta.

 The Meta and/or WhatsApp Business Accounts connected to this Sender were disabled by Meta or your business verification request is currently pending. To proceed with using WhatsApp Business API features, you must complete the business verification process. Visit your business settings to initiate or resolve the pending business verification request.

#### Possible Causes
1. Policy Violations:
   - Details: Your WABA may have violated Meta's [WhatsApp Business Policies](https://www.whatsapp.com/business/api/policies/), such as sending unsolicited messages, using prohibited content, or failing to comply with data privacy regulations. 

2. Spam or Unsolicited Messaging:
   - Details: Sending a high volume of unsolicited messages or spam can lead to account suspension to protect user experience.

3. Inappropriate Content:
   - Details: Sharing content that violates WhatsApp's guidelines, including offensive, illegal, or harmful material.

4. User Reports:
   - Details: Receiving multiple reports from users about inappropriate behavior or unwanted messages from your account.

5  Incomplete Verification Submission:
   - Details: Required documents or information were not fully provided during the verification process.

#### Possible Solutions
1. Review Meta's Notification:
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

For more details, visit [Meta's Business Support Home](https://business.facebook.com/accountquality). If you require assistance from Twilio, please open a support ticket."#,
            TwilioChannelsError::ErrorCode63109 => r#"## ERROR - 63109

### This Sender has been migrated into a different account.

 This Sender has been migrated into a different account.

#### Possible Causes
The Sender was onboarded to a different account.

#### Possible Solutions
If this was not intentional, visit the [Twilio Support Center](https://help.twilio.com/) to contact support."#,
            TwilioChannelsError::ErrorCode63100 => r#"## ERROR - 63100

### Validation Error

 The request body is either empty, not valid JSON, or failed other validation rules. Please refer to the response's message parameter for additional guidance. 

#### Possible Causes
* Request body was empty
* Request body was not valid JSON
* Request body failed other validation rules

#### Possible Solutions
* Review the response's message parameter for additional details
* Verify the request body contains valid JSON according to the documentation"#,
            TwilioChannelsError::ErrorCode63111 => r#"## ERROR - 63111

### Sender's phone number or WABA returned "not found".

 Meta returned "not found" for the Sender's phone number or WABA.


#### Possible Causes
1. Phone number was deleted from the WABA linked to this account.
2. WABA linked to this account was deleted.
3. WABA was unshared with Twilio

#### Possible Solutions
- Confirm the phone number and WABA still exist within Meta and are shared with Twilio
- Delete the Sender and reonboard."#,
            TwilioChannelsError::ErrorCode63102 => r#"## ERROR - 63102

### Account already linked to another WABA ID

 The Twilio Account contains other WhatsApp Senders and is already linked to an existing WhatsApp Business Account (WABA). Twilio Accounts must be linked to only one WABA ID at a time.

#### Possible Causes
* Attempting to use a different WABA ID than the one linked to the Twilio Account

#### Possible Solutions
* Use the same WABA ID that is linked to the account
* Delete other WhatsApp Senders to link the Account to another WABA"#
        }
    }
}
