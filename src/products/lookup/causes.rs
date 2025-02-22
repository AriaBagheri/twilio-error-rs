// AUTO-GENERATED. DO NOT MODIFY. YOUR CHANGES WILL BE OVER-WRITTEN
use crate::products::TwilioLookupError;
use standard_error::traits::StandardErrorCausesTrait;

impl StandardErrorCausesTrait for TwilioLookupError {
    fn causes(&self) -> Option<&'static str> {
        match self {
            TwilioLookupError::ErrorCode60616 => Some(r#"Your application is making lookups at a rate higher than allowed for the requested package. This may be intentional or the result of an unexpected condition in your code."#),
            TwilioLookupError::ErrorCode60699 => Some(r#"Our systems have detected abnormal behavior and have disabled Lookup on your account."#),
            TwilioLookupError::ErrorCode60606 => Some(r#"The request was received but failed due to the Account not being enabled to access the requested Lookup package."#),
            TwilioLookupError::ErrorCode60620 => Some(r#"The carrier failed to provide SIM Swap information for this phone number."#),
            TwilioLookupError::ErrorCode60614 => Some(r#"The request was received but failed due to the Lookup request not being supported by the selected Twilio Region due to data residency requirements. This can happen when a request for an EU or GB phone number is made outside of the IE1 Region. Learn more at [Twilio Regions](https://www.twilio.com/docs/global-infrastructure/understanding-twilio-regions) and [Using Lookup with Twilio Regions](https://www.twilio.com/docs/lookup/using-lookup-with-twilio-regions)."#),
            TwilioLookupError::ErrorCode60624 => Some(r#"The error message indicates either excessive usage within a short period or exceeding the established request limit"#),
            TwilioLookupError::ErrorCode60611 => Some(r#"The request was received but failed due to the Account reaching its request quota for the given Lookup package. Some packages may have a quota set when they are in the Pilot phase."#),
            TwilioLookupError::ErrorCode60619 => Some(r#"The request was received but failed due to the Lookup request not being supported by the selected Twilio Region. This can happen when specific Lookup packages or functionality is not available in the Region. Learn more at [Twilio Regions](https://www.twilio.com/docs/global-infrastructure/understanding-twilio-regions) and [Using Lookup with Twilio Regions](https://www.twilio.com/docs/lookup/using-lookup-with-twilio-regions)."#),
            TwilioLookupError::ErrorCode60612 => Some(r#"Lookup package requested is only applicable to mobile phone numbers"#),
            TwilioLookupError::ErrorCode60621 => Some(r#"The request was processed successfully and Last SIM Swap information was returned, but carrier information was not available."#),
            TwilioLookupError::ErrorCode60609 => Some(r#"Your account has not been approved and enabled to access the requested package"#),
            TwilioLookupError::ErrorCode60607 => Some(r#"The request was received but failed due to the phone number being associated with a country that is not supported by the requested Lookup package."#),
            TwilioLookupError::ErrorCode60618 => Some(r#"The request was received but failed due to one or more request parameters being malformed. This can happen when a parameter has exceeded its maximum length constraint. "#),
            TwilioLookupError::ErrorCode60601 => Some(r#"Your account is not authorized for lookups on Canada phone numbers"#),
            TwilioLookupError::ErrorCode60610 => Some(r#"You have provided a phone number that appears as either not provisioned with any carrier or belonging to a carrier outside the scope of our data coverage."#),
            TwilioLookupError::ErrorCode60608 => Some(r#"The Lookup request was received but failed during processing due to an error encountered with downstream providers."#),
            TwilioLookupError::ErrorCode60623 => Some(r#"The Lookup request was received but failed during the verification existence check because it was not found a success verification or it had failed or expired."#),
            TwilioLookupError::ErrorCode60617 => Some(r#"The request was received but failed due to the Lookup package you requested requiring at least one optional request parameter to match against the phone number. If there is a single request parameter, its value may not be left blank."#),
            TwilioLookupError::ErrorCode60613 => Some(r#"Error encountered when calling Lookup data providers. Fallback data returned."#),
            TwilioLookupError::ErrorCode60622 => Some(r#"The phone number sent is not a valid Magic Number."#),
            TwilioLookupError::ErrorCode60600 => Some(r#"Phone number provided is not assigned to a carrier based on our data sources; or the phone number is outside of the scope of our data coverage"#)
        }
    }
}
