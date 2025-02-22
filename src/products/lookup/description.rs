// AUTO-GENERATED. DO NOT MODIFY. YOUR CHANGES WILL BE OVER-WRITTEN
use crate::products::TwilioLookupError;
use standard_error::traits::StandardErrorDescriptionTrait;

impl StandardErrorDescriptionTrait for TwilioLookupError {
    fn description(&self) -> Option<&'static str> {
        match self {
            TwilioLookupError::ErrorCode60616 => Some(r#"The rate of lookup requests exceeds allowable limits for the requested package."#),
            TwilioLookupError::ErrorCode60699 => Some(r#"Usage of Lookup has been disabled on your account"#),
            TwilioLookupError::ErrorCode60606 => Some(r#"Your account is not enabled to call the requested package."#),
            TwilioLookupError::ErrorCode60620 => Some(r#"Lookup Carrier Information Not Available"#),
            TwilioLookupError::ErrorCode60614 => Some(r#"Lookup package requested is not supported in your Twilio Region.

[Learn more about Twilio Regions here](https://www.twilio.com/docs/global-infrastructure/understanding-twilio-regions)."#),
            TwilioLookupError::ErrorCode60624 => Some(r#"The account has reached the maximum rate limit for Trial Accounts."#),
            TwilioLookupError::ErrorCode60611 => Some(r#"Lookup quota for requested package has been reached"#),
            TwilioLookupError::ErrorCode60619 => Some(r#"The Lookup request cannot be completed in the selected Twilio region."#),
            TwilioLookupError::ErrorCode60612 => Some(r#"Requested phone number is not a mobile number"#),
            TwilioLookupError::ErrorCode60621 => Some(r#"Lookup SIM Swap Information is Incomplete From Carrier"#),
            TwilioLookupError::ErrorCode60609 => Some(r#"live_activity is not enabled on your account"#),
            TwilioLookupError::ErrorCode60607 => Some(r#"No provider found to satisfy the Lookup request"#),
            TwilioLookupError::ErrorCode60618 => Some(r#"An invalid value was passed for query parameters"#),
            TwilioLookupError::ErrorCode60601 => Some(r#"Authorization must be granted to your company by the CLNPC to return data on Canada phone numbers."#),
            TwilioLookupError::ErrorCode60610 => Some(r#"The requested phone number is outside of our range of coverage"#),
            TwilioLookupError::ErrorCode60608 => Some(r#"Lookup provider replied with an error"#),
            TwilioLookupError::ErrorCode60623 => None,
            TwilioLookupError::ErrorCode60617 => Some(r#"Missing at least one valid additional argument to execute the Lookup you requested"#),
            TwilioLookupError::ErrorCode60613 => Some(r#"Temporary service degradation with provider encountered; fallback data returned if available."#),
            TwilioLookupError::ErrorCode60622 => Some(r#"The request was received but failed due to the phone number not being a valid Magic Number."#),
            TwilioLookupError::ErrorCode60600 => Some(r#"No information returned on the requested phone number"#)
        }
    }
}
