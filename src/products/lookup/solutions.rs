// AUTO-GENERATED. DO NOT MODIFY. YOUR CHANGES WILL BE OVER-WRITTEN
use crate::products::TwilioLookupError;
use standard_error::traits::StandardErrorSolutionsTrait;

impl StandardErrorSolutionsTrait for TwilioLookupError {
    fn solutions(&self) -> Option<&'static str> {
        match self {
            TwilioLookupError::ErrorCode60616 => Some(r#"Our downstream providers do not have uniform rate limits. Confirm the published rate limits for the requested package and debug your application to ensure your lookup request rate remains under allowed limits."#),
            TwilioLookupError::ErrorCode60699 => Some(r#"- [Contact Twilio Support](https://www.twilio.com/help/contact) to re-activate your account."#),
            TwilioLookupError::ErrorCode60606 => Some(r#"- Check that the Account SID provided is correct.
- If the requested package requires it, complete the [Account Security Registration form](https://twlo.my.salesforce-sites.com/countrycarrier/SN_CarrierRegistration_VFP) to enable the package on your Account.
- [Contact Twilio Support](https://www.twilio.com/help/contact) to rule out other issues."#),
            TwilioLookupError::ErrorCode60620 => Some(r#"Do not retry request for this end user."#),
            TwilioLookupError::ErrorCode60614 => Some(r#"- Confirm that the request is selecting the expected Region.
- Confirm that the request is using an API key generated for the expected Region."#),
            TwilioLookupError::ErrorCode60624 => Some(r#"A possible solution to mitigate this issue is to transition to a paid account that offers higher rate limits and unrestricted access to the services."#),
            TwilioLookupError::ErrorCode60611 => Some(r#"- [Contact Twilio Support](https://www.twilio.com/help/contact) to remove the quota limitation."#),
            TwilioLookupError::ErrorCode60619 => Some(r#"- Confirm that the request is selecting the expected Region.
- Confirm that the request is using an API key generated for the expected Region."#),
            TwilioLookupError::ErrorCode60612 => Some(r#"Try again with a mobile phone number"#),
            TwilioLookupError::ErrorCode60621 => Some(r#"- Retry request to rule out transient issues in order to receive carrier information.
- If error persists, [contact Twilio Support](https://www.twilio.com/help/contact) to rule out other issues."#),
            TwilioLookupError::ErrorCode60609 => Some(r#"Please contact Support or Sales to enable the requested package"#),
            TwilioLookupError::ErrorCode60607 => Some(r#"- Check that the phone number provided is correct. 
- Confirm that the phone number's country is supported by the requested Lookup package. Check country coverage in https://www.twilio.com/docs/lookup/v2-api."#),
            TwilioLookupError::ErrorCode60618 => Some(r#"- Review request parameters and retry request."#),
            TwilioLookupError::ErrorCode60601 => Some(r#"To apply for access to query Canada phone number information, see https://help.twilio.com/articles/360004563433-Twilio-Lookup-API-is-Not-Returning-Carrier-Data-for-Canadian-Phone-Numbers"#),
            TwilioLookupError::ErrorCode60610 => Some(r#"Try a supported phone number"#),
            TwilioLookupError::ErrorCode60608 => Some(r#"- Retry request to rule out transient issues. 
- If error persists, [contact Twilio Support](https://www.twilio.com/help/contact) to rule out integration issues."#),
            TwilioLookupError::ErrorCode60623 => Some(r#"- Retry request to rule out transient issues. 
- If error persists, [contact Twilio Support](https://www.twilio.com/help/contact) to rule out integration issues."#),
            TwilioLookupError::ErrorCode60617 => Some(r#"- Review request parameters to ensure at least one optional field, such as FirstName, is included."#),
            TwilioLookupError::ErrorCode60613 => Some(r#"- Retry request to rule out transient issues in order to receive complete information.
- If error persists, [contact Twilio Support](https://www.twilio.com/help/contact) to rule out other issues."#),
            TwilioLookupError::ErrorCode60622 => Some(r#"Check that the phone number provided is correct and one of the pre-configured phone numbers from Magic Numbers."#),
            TwilioLookupError::ErrorCode60600 => Some(r#"Confirm with authoritative evidence that the phone number is valid and assigned to a carrier; then contact Support and let us know"#)
        }
    }
}
