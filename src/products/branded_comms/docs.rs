// AUTO-GENERATED. DO NOT MODIFY. YOUR CHANGES WILL BE OVER-WRITTEN
use crate::products::TwilioBrandedCommsError;
use standard_error::traits::StandardErrorDocsTrait;

impl StandardErrorDocsTrait for TwilioBrandedCommsError {
    fn docs(&self) -> &'static str {
        match self {
            TwilioBrandedCommsError::ErrorCode60704 => r#"## ERROR - 60704

### Phone number not branded by Twilio

 The origination number (`From`) used was not found as a valid branded phone number

#### Possible Causes
- The origination number does not belong to the account
- The origination number is incorrect
- The origination number is not correctly branded, or linked to a business/brand
- The origination number is linked to a deactivated business/brand
- The origination number is linked to a business/brand that's not vetted, or the vetting was rejected

#### Possible Solutions
- Make sure the origination number belongs to the account and is in the correct E.164 format
- Make sure the origination number is correctly linked to a business/brand, that's vetting-approved, and that's active"#,
            TwilioBrandedCommsError::ErrorCode60723 => r#"## WARNING - 60723

### Brand status does not allow updates

 Brands can only be updated when they have certain status values.

#### Possible Causes
You sent a POST request to update a brand that is in status "Pending Verification" or "Verified".

#### Possible Solutions
If you really need to update the brand information, you can dismiss its current status (this action will update the status to "Draft") and then you can POST the update."#,
            TwilioBrandedCommsError::ErrorCode60712 => r#"## ERROR - 60712

### Error communicating with Regulatory Compliance API

 KYC, part of the Branded Comms product, depends on Twilio's regulatory APIs

#### Possible Causes
Communication with the Regulatory Compliance API failed

#### Possible Solutions
Wait a bit in case this is a temporal error, or else contact our support team"#,
            TwilioBrandedCommsError::ErrorCode60719 => r#"## WARNING - 60719

### Branded Call not found

 You are requesting a branded call for phone number that is not branded (it is not associated to a verified brand and business)

#### Possible Causes
The phone number is not associated to a brand, or it is, but the brand or the business are no verified.

#### Possible Solutions
The business and brand associated to the phone number being requested in the branded call must go through a vetting process and get a status of verified"#,
            TwilioBrandedCommsError::ErrorCode60721 => r#"## WARNING - 60721

### Phone Number(s) already used in a Branded Channel

 When you create/update a Branded Channel resource, you send a list of your phone numbers to be associated with it. You cannot have a phone number associated with more than one Branded Channel.

#### Possible Causes
At least one of the phone numbers sent in the payload when creating/updating a Branded Channel are already associated with a different Branded Channel.

#### Possible Solutions
Use a different phone number in the Branded Channel payload, or remove the phone number from the existing Branded Channel if you really want to associate it with the Branded Channel being created/updated."#,
            TwilioBrandedCommsError::ErrorCode60727 => r#"## WARNING - 60727

### Channel not found

 Returned when looking for a channel with no match in the storage

#### Possible Causes
- The channel does not exist anymore
- The phone number sent is invalid/incorrect

#### Possible Solutions
- Check the phone number in your request"#,
            TwilioBrandedCommsError::ErrorCode60706 => r#"## ERROR - 60706

### Invalid Push Token

 Registering a new device, the Push Token is empty or invalid

#### Possible Causes
- The `PushToken` is iOS-incorrect or empty

#### Possible Solutions
- Check the `PushToken` of the iOS device to register"#,
            TwilioBrandedCommsError::ErrorCode60709 => r#"## WARNING - 60709

### Business Profile already exists

 This Account already has a Business Profile

#### Possible Causes
- Each Account can have only one Business Profile, and you're attempting to create another for the same Account

#### Possible Solutions
- Make sure you're using the Account intended to create the new Business Profile"#,
            TwilioBrandedCommsError::ErrorCode60716 => r#"## ERROR - 60716

### Selected logo is not a valid PNG file

 The file sent for the brand logo image is not the expected, PNG

#### Possible Causes
Brand logo file format was not interpreted as PNG

#### Possible Solutions
Make sure file format is PNG indeed"#,
            TwilioBrandedCommsError::ErrorCode60702 => r#"## WARNING - 60702

### Business Profile not found

 The Business was not found

#### Possible Causes
- The business does not exist anymore
- The `BusinessSid` parameter is invalid/incorrect

#### Possible Solutions
- Check the `BusinessSid` parameter in your request, from Console"#,
            TwilioBrandedCommsError::ErrorCode60714 => r#"## WARNING - 60714

### Brand not found

 You are asking for a brand that does not exist for the business

#### Possible Causes
The brand, or its associated business, may have been removed

#### Possible Solutions
Double check that the business SID and the brand SID are correct"#,
            TwilioBrandedCommsError::ErrorCode60713 => r#"## ERROR - 60713

### Error communicating with Regulatory Identification API

 KYC, part of the Branded Comms product, depends on Twilio's regulatory APIs

#### Possible Causes
Communication with the Regulatory Identification API failed

#### Possible Solutions
Wait a bit in case this is a temporal error, or else contact our support team"#,
            TwilioBrandedCommsError::ErrorCode60707 => r#"## WARNING - 60707

### Branded Channel not found

 Branded Channel not found

#### Possible Causes
- The branded channel does not exist anymore
- The `BrandedChannelSid`, `BrandSid` or `BusinessSid` parameter is invalid/incorrect

#### Possible Solutions
- Check the Sids in your request"#,
            TwilioBrandedCommsError::ErrorCode60710 => r#"## WARNING - 60710

### Phone number CPS not found

 This phone number was not found on the Call Placement Service (CPS) Directory

#### Possible Causes
- The number queried is not branded by Twilio or any other CPS
- The service to discover CPS is having issues querying phone numbers

#### Possible Solutions
- Check that the phone number is the same that you branded in Twilio or your CPS provider"#,
            TwilioBrandedCommsError::ErrorCode60722 => r#"## WARNING - 60722

### Business status does not allow dismissal

 Businesses (AKA Business Profiles) can only be dismissed when they have certain status values. Dismissing a business will set its status to "Draft".

#### Possible Causes
You sent a dismiss request for a business that is in status "Draft" or "Rejected".


#### Possible Solutions
You're only interested in dismissing a business to update its information before being vetted by Twilio's vetting team, or if you want to remove its verified status.
To dismiss the business its status must be one of "Pending Verification" or "Verified"."#,
            TwilioBrandedCommsError::ErrorCode60703 => r#"## ERROR - 60703

### Invalid phone numbers format

 The origination or destination numbers are not valid numbers in E.164 format

#### Possible Causes
- The origination or destination number is missing the `+` sign, or has separators like spaces or dashes

#### Possible Solutions
- Check the `From` and `To` numbers in your request"#,
            TwilioBrandedCommsError::ErrorCode60700 => r#"## ERROR - 60700

### Something went wrong. Try again later

 Server error, generic 500 error

#### Possible Causes
- Details included in the response body

#### Possible Solutions
- Details included in the response body"#,
            TwilioBrandedCommsError::ErrorCode60725 => r#"## WARNING - 60725

### Brand status does not allow to have branded channels

 You are creating a branded channel for a brand in a status does not allow it to have branded channels

#### Possible Causes
A brand must be verified in order to have any branded channels

#### Possible Solutions
Make sure your brand already has a verified status before attempting to create branded channels."#,
            TwilioBrandedCommsError::ErrorCode60715 => r#"## ERROR - 60715

### Error reading logo file

 The file sent for the brand logo image could not be read

#### Possible Causes
The file may be corrupted, or the file format is not expected (PNG is expected, max size 128kb)

#### Possible Solutions
Make sure file format and size meet the criteria"#,
            TwilioBrandedCommsError::ErrorCode60717 => r#"## ERROR - 60717

### Error uploading logo to the storage

 The file sent for the brand logo image could not be stored in the storage bucket

#### Possible Causes
It may be caused by a temporal communication issue with the storage bucket.

#### Possible Solutions
Try again later"#,
            TwilioBrandedCommsError::ErrorCode60708 => r#"## WARNING - 60708

### Phone record number not found

 DNS Record for that phone number not found

#### Possible Causes
- Our provider (Route 53) did not find the DNS record you're trying to update or delete

#### Possible Solutions
- Make sure the DNS record for that phone number exists in order to update or delete it"#,
            TwilioBrandedCommsError::ErrorCode60701 => r#"## ERROR - 60701

### Invalid request

 Bad Request, generic 400 error

#### Possible Causes
- Details included in the response body

#### Possible Solutions
- Details included in the response body"#,
            TwilioBrandedCommsError::ErrorCode60726 => r#"## WARNING - 60726

### Business status does not allow to have brands

 You are creating a brand for a business in a status does not allow it to have brands

#### Possible Causes
A business must be verified in order to have any brands

#### Possible Solutions
Make sure your business already has a verified status before attempting to create brands"#,
            TwilioBrandedCommsError::ErrorCode60724 => r#"## WARNING - 60724

### Brand status does not allow dismissal

 Brands can only be dismissed when they have certain status values. Dismissing a brand will set its status to "Draft".

#### Possible Causes
You sent a dismiss request for a brand that is in status "Draft" or "Rejected".


#### Possible Solutions
You're only interested in dismissing a brand to update its information before being vetted by Twilio's vetting team, or if you want to remove its verified status.
To dismiss the brand its status must be one of "Pending Verification" or "Verified"."#,
            TwilioBrandedCommsError::ErrorCode60711 => r#"## ERROR - 60711

### Business status does not allow updates

 Businesses (AKA Business Profiles) can only be updated when they have certain status values. 

#### Possible Causes
You sent a *POST* request to update a business that is in status "Pending Verification" or "Verified"

#### Possible Solutions
If you really need to update the business information, you can dismiss its current status (this action will update the status to "Draft") and then you can POST the update "#
        }
    }
}
