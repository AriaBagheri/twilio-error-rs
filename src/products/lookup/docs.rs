// AUTO-GENERATED. DO NOT MODIFY. YOUR CHANGES WILL BE OVER-WRITTEN
use crate::products::TwilioLookupError;
use standard_error::traits::StandardErrorDocsTrait;

impl StandardErrorDocsTrait for TwilioLookupError {
    fn docs(&self) -> &'static str {
        match self {
            TwilioLookupError::ErrorCode60616 => r#"## ERROR - 60616

### Lookup rate limit exceeded

 The rate of lookup requests exceeds allowable limits for the requested package.

#### Possible Causes
Your application is making lookups at a rate higher than allowed for the requested package. This may be intentional or the result of an unexpected condition in your code.

#### Possible Solutions
Our downstream providers do not have uniform rate limits. Confirm the published rate limits for the requested package and debug your application to ensure your lookup request rate remains under allowed limits."#,
            TwilioLookupError::ErrorCode60699 => r#"## ERROR - 60699

### Lookup Usage Disabled

 

#### Possible Causes
Our systems have detected abnormal behavior and have disabled Lookup on your account.

#### Possible Solutions
- [Contact Twilio Support](https://www.twilio.com/help/contact) to re-activate your account."#,
            TwilioLookupError::ErrorCode60606 => r#"## ERROR - 60606

### Lookup Package is Not Enabled

 Your account is not enabled to call the requested package.

#### Possible Causes
The request was received but failed due to the Account not being enabled to access the requested Lookup package.

#### Possible Solutions
- Check that the Account SID provided is correct.
- If the requested package requires it, complete the [Account Security Registration form](https://twlo.my.salesforce-sites.com/countrycarrier/SN_CarrierRegistration_VFP) to enable the package on your Account.
- [Contact Twilio Support](https://www.twilio.com/help/contact) to rule out other issues."#,
            TwilioLookupError::ErrorCode60620 => r#"## ERROR - 60620

### Lookup SIM Swap Information is Incomplete From Carrier

 

#### Possible Causes
The carrier failed to provide SIM Swap information for this phone number.

#### Possible Solutions
Do not retry request for this end user."#,
            TwilioLookupError::ErrorCode60614 => r#"## ERROR - 60614

### Lookup Request Must Be Completed in Different Twilio Region

 

#### Possible Causes
The request was received but failed due to the Lookup request not being supported by the selected Twilio Region due to data residency requirements. This can happen when a request for an EU or GB phone number is made outside of the IE1 Region. Learn more at [Twilio Regions](https://www.twilio.com/docs/global-infrastructure/understanding-twilio-regions) and [Using Lookup with Twilio Regions](https://www.twilio.com/docs/lookup/using-lookup-with-twilio-regions).

#### Possible Solutions
- Confirm that the request is selecting the expected Region.
- Confirm that the request is using an API key generated for the expected Region."#,
            TwilioLookupError::ErrorCode60624 => r#"## ERROR - 60624

### Maximum rate limit for Trial Accounts

 The account has reached the maximum rate limit for Trial Accounts.

#### Possible Causes
The error message indicates either excessive usage within a short period or exceeding the established request limit

#### Possible Solutions
A possible solution to mitigate this issue is to transition to a paid account that offers higher rate limits and unrestricted access to the services."#,
            TwilioLookupError::ErrorCode60611 => r#"## ERROR - 60611

### Lookup Package Quota Reached

 

#### Possible Causes
The request was received but failed due to the Account reaching its request quota for the given Lookup package. Some packages may have a quota set when they are in the Pilot phase.

#### Possible Solutions
- [Contact Twilio Support](https://www.twilio.com/help/contact) to remove the quota limitation."#,
            TwilioLookupError::ErrorCode60619 => r#"## ERROR - 60619

### Lookup Request Cannot be Completed in Twilio Region

 

#### Possible Causes
The request was received but failed due to the Lookup request not being supported by the selected Twilio Region. This can happen when specific Lookup packages or functionality is not available in the Region. Learn more at [Twilio Regions](https://www.twilio.com/docs/global-infrastructure/understanding-twilio-regions) and [Using Lookup with Twilio Regions](https://www.twilio.com/docs/lookup/using-lookup-with-twilio-regions).

#### Possible Solutions
- Confirm that the request is selecting the expected Region.
- Confirm that the request is using an API key generated for the expected Region."#,
            TwilioLookupError::ErrorCode60612 => r#"## ERROR - 60612

### Requested phone number not mobile

 Requested phone number is not a mobile number

#### Possible Causes
Lookup package requested is only applicable to mobile phone numbers

#### Possible Solutions
Try again with a mobile phone number"#,
            TwilioLookupError::ErrorCode60621 => r#"## ERROR - 60621

### Lookup Carrier Information Not Available

 

#### Possible Causes
The request was processed successfully and Last SIM Swap information was returned, but carrier information was not available.

#### Possible Solutions
- Retry request to rule out transient issues in order to receive carrier information.
- If error persists, [contact Twilio Support](https://www.twilio.com/help/contact) to rule out other issues."#,
            TwilioLookupError::ErrorCode60609 => r#"## ERROR - 60609

### Live activity not enabled

 live_activity is not enabled on your account

#### Possible Causes
Your account has not been approved and enabled to access the requested package

#### Possible Solutions
Please contact Support or Sales to enable the requested package"#,
            TwilioLookupError::ErrorCode60607 => r#"## ERROR - 60607

### Lookup Unsupported Country

 No provider found to satisfy the Lookup request

#### Possible Causes
The request was received but failed due to the phone number being associated with a country that is not supported by the requested Lookup package.

#### Possible Solutions
- Check that the phone number provided is correct. 
- Confirm that the phone number's country is supported by the requested Lookup package. Check country coverage in https://www.twilio.com/docs/lookup/v2-api."#,
            TwilioLookupError::ErrorCode60618 => r#"## ERROR - 60618

### Lookup Malformed Request Parameter

 

#### Possible Causes
The request was received but failed due to one or more request parameters being malformed. This can happen when a parameter has exceeded its maximum length constraint. 

#### Possible Solutions
- Review request parameters and retry request."#,
            TwilioLookupError::ErrorCode60601 => r#"## ERROR - 60601

### Authorization required for Canada lookups

 Authorization must be granted to your company by the CLNPC to return data on Canada phone numbers.

#### Possible Causes
Your account is not authorized for lookups on Canada phone numbers

#### Possible Solutions
To apply for access to query Canada phone number information, see https://help.twilio.com/articles/360004563433-Twilio-Lookup-API-is-Not-Returning-Carrier-Data-for-Canadian-Phone-Numbers"#,
            TwilioLookupError::ErrorCode60610 => r#"## ERROR - 60610

### Phone number outside of coverage

 The requested phone number is outside of our range of coverage

#### Possible Causes
You have provided a phone number that appears as either not provisioned with any carrier or belonging to a carrier outside the scope of our data coverage.

#### Possible Solutions
Try a supported phone number"#,
            TwilioLookupError::ErrorCode60608 => r#"## ERROR - 60608

### Lookup Error

 

#### Possible Causes
The Lookup request was received but failed during processing due to an error encountered with downstream providers.

#### Possible Solutions
- Retry request to rule out transient issues. 
- If error persists, [contact Twilio Support](https://www.twilio.com/help/contact) to rule out integration issues."#,
            TwilioLookupError::ErrorCode60623 => r#"## WARNING - 60623

### Verification Not Found

 

#### Possible Causes
The Lookup request was received but failed during the verification existence check because it was not found a success verification or it had failed or expired.

#### Possible Solutions
- Retry request to rule out transient issues. 
- If error persists, [contact Twilio Support](https://www.twilio.com/help/contact) to rule out integration issues."#,
            TwilioLookupError::ErrorCode60617 => r#"## ERROR - 60617

### Lookup Not Enough Request Parameters

 Missing at least one valid additional argument to execute the Lookup you requested

#### Possible Causes
The request was received but failed due to the Lookup package you requested requiring at least one optional request parameter to match against the phone number. If there is a single request parameter, its value may not be left blank.

#### Possible Solutions
- Review request parameters to ensure at least one optional field, such as FirstName, is included."#,
            TwilioLookupError::ErrorCode60613 => r#"## ERROR - 60613

### Lookup Provider Degradation

 

#### Possible Causes
Error encountered when calling Lookup data providers. Fallback data returned.

#### Possible Solutions
- Retry request to rule out transient issues in order to receive complete information.
- If error persists, [contact Twilio Support](https://www.twilio.com/help/contact) to rule out other issues."#,
            TwilioLookupError::ErrorCode60622 => r#"## ERROR - 60622

### Invalid Magic Number

 The request was received but failed due to the phone number not being a valid Magic Number.

#### Possible Causes
The phone number sent is not a valid Magic Number.

#### Possible Solutions
Check that the phone number provided is correct and one of the pre-configured phone numbers from Magic Numbers."#,
            TwilioLookupError::ErrorCode60600 => r#"## ERROR - 60600

### Unprovisioned or out of coverage

 No information returned on the requested phone number

#### Possible Causes
Phone number provided is not assigned to a carrier based on our data sources; or the phone number is outside of the scope of our data coverage

#### Possible Solutions
Confirm with authoritative evidence that the phone number is valid and assigned to a carrier; then contact Support and let us know"#
        }
    }
}
