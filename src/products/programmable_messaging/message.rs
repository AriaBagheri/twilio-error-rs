// AUTO-GENERATED. DO NOT MODIFY. YOUR CHANGES WILL BE OVER-WRITTEN
use crate::products::TwilioProgrammableMessagingError;
use standard_error::traits::StandardErrorMessageTrait;
use std::borrow::Cow;

impl StandardErrorMessageTrait for TwilioProgrammableMessagingError {
    fn message(&self) -> Cow<'static, str> {
        match self {
            TwilioProgrammableMessagingError::ErrorCode20504 => r#"Twilio Internal Error"#.into(),
            TwilioProgrammableMessagingError::ErrorCode30884 => r#"Campaign vetting rejection - Spam/Phishing"#.into(),
            TwilioProgrammableMessagingError::ErrorCode35133 => r#"Invalid 'Messages' array provided in the request"#.into(),
            TwilioProgrammableMessagingError::ErrorCode30758 => r#"Unknown Error Code"#.into(),
            TwilioProgrammableMessagingError::ErrorCode21660 => r#"Mismatch between the 'From' number and the account"#.into(),
            TwilioProgrammableMessagingError::ErrorCode30897 => r#"Campaign vetting rejection - Disallowed Content"#.into(),
            TwilioProgrammableMessagingError::ErrorCode63036 => r#"The specified phone number cannot be reached using RCS at this time."#.into(),
            TwilioProgrammableMessagingError::ErrorCode30447 => r#"Toll Free verification rejection - phone number not provisioned to Twilio"#.into(),
            TwilioProgrammableMessagingError::ErrorCode30444 => r#"Toll Free verification rejection - Disallowed: Fraud"#.into(),
            TwilioProgrammableMessagingError::ErrorCode30474 => r#"Toll Free verification rejection - Need end business"#.into(),
            TwilioProgrammableMessagingError::ErrorCode36005 => r#"Broadcast File Type Invalid"#.into(),
            TwilioProgrammableMessagingError::ErrorCode30440 => r#"Toll Free verification rejection - Unknown Error"#.into(),
            TwilioProgrammableMessagingError::ErrorCode30901 => r#"Campaign rejection - The campaign registration request timed out."#.into(),
            TwilioProgrammableMessagingError::ErrorCode30749 => r#"Brand Registration Failure: Email address duplicate threshold reached"#.into(),
            TwilioProgrammableMessagingError::ErrorCode30888 => r#"Campaign vetting rejection - Age Gate Not Present / Not Acceptable"#.into(),
            TwilioProgrammableMessagingError::ErrorCode21736 => r#"Brand 2FA Failed"#.into(),
            TwilioProgrammableMessagingError::ErrorCode11321 => r#"Misconfigured webhook"#.into(),
            TwilioProgrammableMessagingError::ErrorCode30480 => r#"Toll-Free phone number verification rejection - Disallowed - Phishing"#.into(),
            TwilioProgrammableMessagingError::ErrorCode30750 => r#"Brand Registration Failure: Mobile phone number duplicate threshold reached"#.into(),
            TwilioProgrammableMessagingError::ErrorCode30994 => r#"Campaign Registration Failed"#.into(),
            TwilioProgrammableMessagingError::ErrorCode63041 => r#"Template paused"#.into(),
            TwilioProgrammableMessagingError::ErrorCode30890 => r#"Campaign vetting rejection - Subscriber Help"#.into(),
            TwilioProgrammableMessagingError::ErrorCode11322 => r#"Invalid webhook method"#.into(),
            TwilioProgrammableMessagingError::ErrorCode63047 => r#"Link to a sample media file returned an invalid Content-Type"#.into(),
            TwilioProgrammableMessagingError::ErrorCode30473 => r#"Toll Free verification rejection - Cannot validate business website URL"#.into(),
            TwilioProgrammableMessagingError::ErrorCode30647 => r#"Failed to Upsert Contact"#.into(),
            TwilioProgrammableMessagingError::ErrorCode216602 => r#"Content Type is not supported on this channel"#.into(),
            TwilioProgrammableMessagingError::ErrorCode30446 => r#"Toll Free verification rejection - Opt-in not sufficient: express consent required"#.into(),
            TwilioProgrammableMessagingError::ErrorCode21737 => r#"Campaign Registration Blocked for Non-Compliant Brand"#.into(),
            TwilioProgrammableMessagingError::ErrorCode30471 => r#"Toll Free verification rejection - High Risk - Non-secured URL in sample message"#.into(),
            TwilioProgrammableMessagingError::ErrorCode21731 => r#"Cannot perform operation on suspended brand"#.into(),
            TwilioProgrammableMessagingError::ErrorCode30460 => r#"Toll Free verification rejection - Disallowed: Third Party Debt Collection"#.into(),
            TwilioProgrammableMessagingError::ErrorCode36001 => r#"Broadcast SID Not Found"#.into(),
            TwilioProgrammableMessagingError::ErrorCode30754 => r#"Brand Registration Failure: Invalid postal address"#.into(),
            TwilioProgrammableMessagingError::ErrorCode20410 => r#"Gone"#.into(),
            TwilioProgrammableMessagingError::ErrorCode30646 => r#"Failed to Upsert Consent"#.into(),
            TwilioProgrammableMessagingError::ErrorCode36004 => r#"Error Retrieving File"#.into(),
            TwilioProgrammableMessagingError::ErrorCode30610 => r#"Message couldn't be delivered"#.into(),
            TwilioProgrammableMessagingError::ErrorCode21659 => r#"'From' is not a Twilio phone number or Short Code country mismatch"#.into(),
            TwilioProgrammableMessagingError::ErrorCode30734 => r#"Brand Registration Failure: Sole Proprietor brands are not enabled"#.into(),
            TwilioProgrammableMessagingError::ErrorCode30475 => r#"Toll Free verification rejection - Opt-in not sufficient: Cannot combine consent for messaging with requirement for service"#.into(),
            TwilioProgrammableMessagingError::ErrorCode30993 => r#"Campaign Registration Failed"#.into(),
            TwilioProgrammableMessagingError::ErrorCode30903 => r#"Campaign rejection - Incorrect Sole Prop Brand Registration"#.into(),
            TwilioProgrammableMessagingError::ErrorCode30701 => r#"Brand Registration Failure: Invalid input parameters"#.into(),
            TwilioProgrammableMessagingError::ErrorCode21267 => r#"Alphanumeric Sender ID cannot be used as the 'From' number on trial accounts"#.into(),
            TwilioProgrammableMessagingError::ErrorCode21732 => r#"Campaign limit reached on the Brand"#.into(),
            TwilioProgrammableMessagingError::ErrorCode30445 => r#"Toll Free verification rejection - Could not validate business information"#.into(),
            TwilioProgrammableMessagingError::ErrorCode21266 => r#"'To' and 'From' numbers cannot be the same"#.into(),
            TwilioProgrammableMessagingError::ErrorCode30757 => r#"Brand Registration Failure: Missing Business Registration Number "#.into(),
            TwilioProgrammableMessagingError::ErrorCode30456 => r#"Toll Free verification rejection - Disallowed: SHAFT - Alcohol"#.into(),
            TwilioProgrammableMessagingError::ErrorCode63051 => r#"WhatsApp Business Account is Locked"#.into(),
            TwilioProgrammableMessagingError::ErrorCode11100 => r#"Invalid URL format"#.into(),
            TwilioProgrammableMessagingError::ErrorCode63046 => r#"Template approved"#.into(),
            TwilioProgrammableMessagingError::ErrorCode30908 => r#"Campaign vetting rejection - Compliant Privacy Policy Required"#.into(),
            TwilioProgrammableMessagingError::ErrorCode21703 => r#"The Messaging Service does not have a phone number available to send a message"#.into(),
            TwilioProgrammableMessagingError::ErrorCode30463 => r#"Toll Free verification rejection - Disallowed: Stock Alerts/Platforms"#.into(),
            TwilioProgrammableMessagingError::ErrorCode30895 => r#"Campaign vetting rejection - Direct Lending - Campaign and Content Attribute Error"#.into(),
            TwilioProgrammableMessagingError::ErrorCode21635 => r#"'To' number cannot be a landline"#.into(),
            TwilioProgrammableMessagingError::ErrorCode30729 => r#"Brand Registration Failure: Country code not allowed"#.into(),
            TwilioProgrammableMessagingError::ErrorCode30896 => r#"Campaign vetting rejection - Opt-in Error"#.into(),
            TwilioProgrammableMessagingError::ErrorCode30906 => r#"Campaign Rejected by Twilio"#.into(),
            TwilioProgrammableMessagingError::ErrorCode30907 => r#"Campaign vetting rejection - Website URL Validation Issue"#.into(),
            TwilioProgrammableMessagingError::ErrorCode30902 => r#"Campaign rejection - A DCA2 rejected this campaign registration request."#.into(),
            TwilioProgrammableMessagingError::ErrorCode30441 => r#"Toll-Free phone number verification rejection - Disallowed: SHAFT - Sex"#.into(),
            TwilioProgrammableMessagingError::ErrorCode30465 => r#"Toll Free verification rejection - Disallowed: Risk Investment/Get Rich Quick Schemes"#.into(),
            TwilioProgrammableMessagingError::ErrorCode35127 => r#"'Messages' Body and ContentVariables cannot be provided together"#.into(),
            TwilioProgrammableMessagingError::ErrorCode63045 => r#"Link to a sample media file returned an unexpected error status"#.into(),
            TwilioProgrammableMessagingError::ErrorCode30797 => r#"Brand Registration Feedback: Non government entity registered as a government entity. Must be a U.S. government entity."#.into(),
            TwilioProgrammableMessagingError::ErrorCode30885 => r#"Campaign vetting rejection - High Risk"#.into(),
            TwilioProgrammableMessagingError::ErrorCode30894 => r#"Campaign vetting rejection - Invalid Brand Information"#.into(),
            TwilioProgrammableMessagingError::ErrorCode36009 => r#"Stats for Broadcast SID Not Found"#.into(),
            TwilioProgrammableMessagingError::ErrorCode30620 => r#"Message couldn't be delivered"#.into(),
            TwilioProgrammableMessagingError::ErrorCode30702 => r#"Brand Registration Failure: Registration not found"#.into(),
            TwilioProgrammableMessagingError::ErrorCode30991 => r#"Campaign Registration Failed"#.into(),
            TwilioProgrammableMessagingError::ErrorCode36008 => r#"Invalid Page Token"#.into(),
            TwilioProgrammableMessagingError::ErrorCode30883 => r#"Campaign vetting rejection - Content Violation"#.into(),
            TwilioProgrammableMessagingError::ErrorCode30442 => r#"Toll-Free phone number verification rejection - Disallowed: Spam"#.into(),
            TwilioProgrammableMessagingError::ErrorCode30892 => r#"Campaign vetting rejection - Invalid Sample Message - Public URL Shorteners"#.into(),
            TwilioProgrammableMessagingError::ErrorCode30751 => r#"Brand Registration Failure: Unsupported mobile phone number"#.into(),
            TwilioProgrammableMessagingError::ErrorCode30891 => r#"Campaign vetting rejection - Invalid Website URL"#.into(),
            TwilioProgrammableMessagingError::ErrorCode30455 => r#"Toll-Free phone number verification rejection - Disallowed: SHAFT - Hate"#.into(),
            TwilioProgrammableMessagingError::ErrorCode63043 => r#"Link to a sample media file returns 403 Forbidden"#.into(),
            TwilioProgrammableMessagingError::ErrorCode36003 => r#"Broadcast Upload File Size Exceeded"#.into(),
            TwilioProgrammableMessagingError::ErrorCode35134 => r#"`Messages` array contains duplicate `To` numbers."#.into(),
            TwilioProgrammableMessagingError::ErrorCode36007 => r#"Broadcast Upload Status Check Failure"#.into(),
            TwilioProgrammableMessagingError::ErrorCode30796 => r#"Brand Registration Feedback: Non public entity registered as a public for profit entity or the stock information mismatch."#.into(),
            TwilioProgrammableMessagingError::ErrorCode30470 => r#"Toll Free verification rejection - High Risk: Deceptive Marketing"#.into(),
            TwilioProgrammableMessagingError::ErrorCode30476 => r#"Toll Free verification rejection - Opt-in not provided"#.into(),
            TwilioProgrammableMessagingError::ErrorCode30990 => r#"Campaign Suspension"#.into(),
            TwilioProgrammableMessagingError::ErrorCode30436 => r#"Invalid Phone Number SID"#.into(),
            TwilioProgrammableMessagingError::ErrorCode30452 => r#"Toll-Free phone number verification unable to process - email invalid"#.into(),
            TwilioProgrammableMessagingError::ErrorCode30793 => r#"Brand Registration Failure: Validation problems with connected bundles"#.into(),
            TwilioProgrammableMessagingError::ErrorCode30468 => r#"Toll Free verification rejection - Disallowed: Third-party Lead Generation"#.into(),
            TwilioProgrammableMessagingError::ErrorCode63052 => r#"Legacy WhatsApp Template Used"#.into(),
            TwilioProgrammableMessagingError::ErrorCode30748 => r#"Brand Registration Failure: Phone number duplicate threshold reached"#.into(),
            TwilioProgrammableMessagingError::ErrorCode30992 => r#"Campaign Registration Failed"#.into(),
            TwilioProgrammableMessagingError::ErrorCode30451 => r#"Toll-Free phone number verification unable to process - address invalid"#.into(),
            TwilioProgrammableMessagingError::ErrorCode30457 => r#"Toll Free verification rejection - Disallowed: SHAFT - Firearms"#.into(),
            TwilioProgrammableMessagingError::ErrorCode30467 => r#"Toll Free verification rejection - Disallowed: Credit Repair"#.into(),
            TwilioProgrammableMessagingError::ErrorCode30798 => r#"Brand Registration Feedback: No IRS 501c tax-exempt status found."#.into(),
            TwilioProgrammableMessagingError::ErrorCode30461 => r#"Toll Free verification rejection - Disallowed: Gambling"#.into(),
            TwilioProgrammableMessagingError::ErrorCode30893 => r#"Campaign vetting rejection - Inconsistency between Sample Message and Use-case"#.into(),
            TwilioProgrammableMessagingError::ErrorCode30469 => r#"Toll Free verification rejection - Disallowed: Illegal substances/articles"#.into(),
            TwilioProgrammableMessagingError::ErrorCode30753 => r#"Brand Registration Failure: Unsupported email address"#.into(),
            TwilioProgrammableMessagingError::ErrorCode30448 => r#"Toll-Free phone number verification rejection - Age Gate"#.into(),
            TwilioProgrammableMessagingError::ErrorCode30449 => r#"Toll Free verification rejection - URL issues in sample message"#.into(),
            TwilioProgrammableMessagingError::ErrorCode30790 => r#"Brand Registration Failure: Internal system error"#.into(),
            TwilioProgrammableMessagingError::ErrorCode30459 => r#"Toll Free verification rejection - Disallowed: Cannabis/CBD"#.into(),
            TwilioProgrammableMessagingError::ErrorCode30795 => r#"Brand Registration Feedback: Data mismatch related to tax id and its associated properties."#.into(),
            TwilioProgrammableMessagingError::ErrorCode30437 => r#"Toll Free verification rejection - Edit time expired"#.into(),
            TwilioProgrammableMessagingError::ErrorCode30887 => r#"Campaign vetting rejection - Opt-out Error"#.into(),
            TwilioProgrammableMessagingError::ErrorCode30880 => r#"Campaign vetting rejection - Unknown Error"#.into(),
            TwilioProgrammableMessagingError::ErrorCode30899 => r#"Campaign rejection - The campaign registration failed because of carrier rejection(s)."#.into(),
            TwilioProgrammableMessagingError::ErrorCode30905 => r#"Campaign Review Pending by Twilio"#.into(),
            TwilioProgrammableMessagingError::ErrorCode36006 => r#"Broadcast Cannot Be Canceled"#.into(),
            TwilioProgrammableMessagingError::ErrorCode30479 => r#"Toll Free verification rejection - justification needed for more than five toll free phone numbers per businesses"#.into(),
            TwilioProgrammableMessagingError::ErrorCode30886 => r#"Campaign vetting rejection - Invalid Campaign Description"#.into(),
            TwilioProgrammableMessagingError::ErrorCode30882 => r#"Campaign vetting rejection - Terms & Conditions"#.into(),
            TwilioProgrammableMessagingError::ErrorCode21733 => r#"Default Messaging Service Not Found"#.into(),
            TwilioProgrammableMessagingError::ErrorCode30464 => r#"Toll Free verification rejection - Disallowed: Cryptocurrency"#.into(),
            TwilioProgrammableMessagingError::ErrorCode30752 => r#"Brand Registration Failure: Invalid or expired OTP"#.into(),
            TwilioProgrammableMessagingError::ErrorCode35132 => r#"Bulk : Provided Attributes JSON is not valid"#.into(),
            TwilioProgrammableMessagingError::ErrorCode21268 => r#"Sending to Premium rate or Information Service numbers is not allowed"#.into(),
            TwilioProgrammableMessagingError::ErrorCode30472 => r#"Toll Free verification rejection - Could not verify Business"#.into(),
            TwilioProgrammableMessagingError::ErrorCode30135 => r#"Unable to issue certificate"#.into(),
            TwilioProgrammableMessagingError::ErrorCode30898 => r#"Campaign vetting rejection - Excessive EIN  "#.into(),
            TwilioProgrammableMessagingError::ErrorCode21265 => r#"'To' number cannot be a Short Code"#.into(),
            TwilioProgrammableMessagingError::ErrorCode30904 => r#"Campaign Not Shared with Twilio"#.into(),
            TwilioProgrammableMessagingError::ErrorCode30791 => r#"Brand Registration Failure: Temporary system error"#.into(),
            TwilioProgrammableMessagingError::ErrorCode30794 => r#"US A2P Registration: General error"#.into(),
            TwilioProgrammableMessagingError::ErrorCode30889 => r#"Campaign vetting rejection - Embedded Phone Number"#.into(),
            TwilioProgrammableMessagingError::ErrorCode30478 => r#"Toll Free verification rejection - single phone number used for multiple businesses"#.into(),
            TwilioProgrammableMessagingError::ErrorCode63040 => r#"Template Rejected"#.into(),
            TwilioProgrammableMessagingError::ErrorCode30909 => r#"Campaign vetting rejection - CTA Verification Issue"#.into(),
            TwilioProgrammableMessagingError::ErrorCode30443 => r#"Toll-Free verification rejection - Disallowed: Loan Marketing"#.into(),
            TwilioProgrammableMessagingError::ErrorCode30462 => r#"Toll Free verification rejection - Disallowed: Sweepstakes"#.into(),
            TwilioProgrammableMessagingError::ErrorCode63024 => r#"Invalid message recipient"#.into(),
            TwilioProgrammableMessagingError::ErrorCode30136 => r#"Unable to renew certificate"#.into(),
            TwilioProgrammableMessagingError::ErrorCode30703 => r#"Brand Registration Failure: Duplicate record detected"#.into(),
            TwilioProgrammableMessagingError::ErrorCode30799 => r#"Brand Registration Feedback: We were unable to verify the details of the registration data."#.into(),
            TwilioProgrammableMessagingError::ErrorCode11206 => r#"HTTP protocol violation"#.into(),
            TwilioProgrammableMessagingError::ErrorCode63042 => r#"Template disabled"#.into(),
            TwilioProgrammableMessagingError::ErrorCode30712 => r#"Brand Registration Failure: Max registration limit reached"#.into(),
            TwilioProgrammableMessagingError::ErrorCode30466 => r#"Toll Free verification rejection - Disallowed - Debt Reduction"#.into(),
            TwilioProgrammableMessagingError::ErrorCode21661 => r#"'From' number is not SMS-capable"#.into(),
            TwilioProgrammableMessagingError::ErrorCode30756 => r#"Brand Registration Failure: Obfuscation check failure"#.into(),
            TwilioProgrammableMessagingError::ErrorCode30747 => r#"Brand Registration Failure: Address duplicate threshold reached"#.into(),
            TwilioProgrammableMessagingError::ErrorCode30881 => r#"Campaign vetting rejection - Invalid Brand Support Email"#.into(),
            TwilioProgrammableMessagingError::ErrorCode30615 => r#"Message couldn't be delivered"#.into(),
            TwilioProgrammableMessagingError::ErrorCode30458 => r#"Toll Free verification rejection - Disallowed: SHAFT - Tobacco / Vape"#.into(),
            TwilioProgrammableMessagingError::ErrorCode30755 => r#"Brand Registration Failure: Unsupported country code"#.into(),
            TwilioProgrammableMessagingError::ErrorCode36002 => r#"Broadcast Request Limit Reached"#.into(),
            TwilioProgrammableMessagingError::ErrorCode63044 => r#"Link to a sample media file returns 404 Not Found"#.into(),
            TwilioProgrammableMessagingError::ErrorCode30792 => r#"Brand Registration Failure: General error"#.into(),
            TwilioProgrammableMessagingError::ErrorCode30900 => r#"Campaign rejection - The campaign use case is ineligible for registration."#.into(),
            TwilioProgrammableMessagingError::ErrorCode63049 => r#"Meta chose not to deliver this WhatsApp marketing message"#.into(),
            TwilioProgrammableMessagingError::ErrorCode30477 => r#"Toll Free verification rejection - Opt-in - Third party information sharing not allowed"#.into(),
            TwilioProgrammableMessagingError::ErrorCode30630 => r#"Attempt to send to unsubscribed recipient"#.into()
        }
    }
}
