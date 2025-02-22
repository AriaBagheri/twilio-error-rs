// AUTO-GENERATED. DO NOT MODIFY. YOUR CHANGES WILL BE OVER-WRITTEN
use crate::products::TwilioLookupError;
use standard_error::traits::StandardErrorMessageTrait;
use std::borrow::Cow;

impl StandardErrorMessageTrait for TwilioLookupError {
    fn message(&self) -> Cow<'static, str> {
        match self {
            TwilioLookupError::ErrorCode60616 => r#"Lookup rate limit exceeded"#.into(),
            TwilioLookupError::ErrorCode60699 => r#"Lookup Usage Disabled"#.into(),
            TwilioLookupError::ErrorCode60606 => r#"Lookup Package is Not Enabled"#.into(),
            TwilioLookupError::ErrorCode60620 => r#"Lookup SIM Swap Information is Incomplete From Carrier"#.into(),
            TwilioLookupError::ErrorCode60614 => r#"Lookup Request Must Be Completed in Different Twilio Region"#.into(),
            TwilioLookupError::ErrorCode60624 => r#"Maximum rate limit for Trial Accounts"#.into(),
            TwilioLookupError::ErrorCode60611 => r#"Lookup Package Quota Reached"#.into(),
            TwilioLookupError::ErrorCode60619 => r#"Lookup Request Cannot be Completed in Twilio Region"#.into(),
            TwilioLookupError::ErrorCode60612 => r#"Requested phone number not mobile"#.into(),
            TwilioLookupError::ErrorCode60621 => r#"Lookup Carrier Information Not Available"#.into(),
            TwilioLookupError::ErrorCode60609 => r#"Live activity not enabled"#.into(),
            TwilioLookupError::ErrorCode60607 => r#"Lookup Unsupported Country"#.into(),
            TwilioLookupError::ErrorCode60618 => r#"Lookup Malformed Request Parameter"#.into(),
            TwilioLookupError::ErrorCode60601 => r#"Authorization required for Canada lookups"#.into(),
            TwilioLookupError::ErrorCode60610 => r#"Phone number outside of coverage"#.into(),
            TwilioLookupError::ErrorCode60608 => r#"Lookup Error"#.into(),
            TwilioLookupError::ErrorCode60623 => r#"Verification Not Found"#.into(),
            TwilioLookupError::ErrorCode60617 => r#"Lookup Not Enough Request Parameters"#.into(),
            TwilioLookupError::ErrorCode60613 => r#"Lookup Provider Degradation"#.into(),
            TwilioLookupError::ErrorCode60622 => r#"Invalid Magic Number"#.into(),
            TwilioLookupError::ErrorCode60600 => r#"Unprovisioned or out of coverage"#.into()
        }
    }
}
