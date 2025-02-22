// AUTO-GENERATED. DO NOT MODIFY. YOUR CHANGES WILL BE OVER-WRITTEN
use crate::products::TwilioPlatformError;
use standard_error::traits::StandardErrorDescriptionTrait;

impl StandardErrorDescriptionTrait for TwilioPlatformError {
    fn description(&self) -> Option<&'static str> {
        match self {
            TwilioPlatformError::ErrorCode20101 => None,
            TwilioPlatformError::ErrorCode20409 => Some(r#"The request could not be completed due to a conflict with the current state of the target resource."#),
            TwilioPlatformError::ErrorCode20162 => Some(r#"The requested resource is already being updated simultaneously by another API request. The last request will be rejected to avoid inconsistent state."#),
            TwilioPlatformError::ErrorCode20157 => None,
            TwilioPlatformError::ErrorCode21102 => None,
            TwilioPlatformError::ErrorCode20159 => None,
            TwilioPlatformError::ErrorCode20105 => None,
            TwilioPlatformError::ErrorCode20404 => Some(r#"## Not Found

The resource was not found. Here are some examples of cases that may trigger
a 404 error.

- Requesting a resource for a sid that does not exist, for example

        GET verify.twilio.com/v2/Services/VA123/VE123
        Or 
        POST verify.twilio.com/v2/Services/VA123/VerificationCheck

             1. The resource has never existed (incorrect SID).
             2. The resource existed but has been deleted.
             3. The resource is a Verification SID that has been "soft deleted" after the 
                 verification process is completed or expired. When a verification is 
                 approved or expires, Twilio performs a "soft delete" on the verification 
                 record. This means that while the record is no longer active and cannot 
                 be retrieved through the API, it is still stored in Twilio's system for 
                 compliance and auditing purposes. As a result, any subsequent 
                 requests to the Verification SID's REST API resource will return a 20404 
                 error, indicating that the resource is not found. 

        GET /2010-04-01/Accounts/AC123/Calls/CA123

    where CA123 is not a call sid that exists for your account

- Trying to retrieve a resource that doesn't exist, for example

        GET /2010-04-01/Accounts/AC123/TwilioCalls/CA123

    where the resource name is Calls, not TwilioCalls. Note that the API is
    case sensitive, so requesting "calls" instead of "Calls" will also return a
    404.

- Missing a sid in the request path. For example, let's say I accidentally
don't set a value for a call sid, using the PHP helper library:

        $callSid = null;
        $client->account->calls->get($callSid);

    This may turn into

        GET /2010-04-01/Accounts/AC123/Calls/.json

    because of the nonexistent sid, which may 404 your request or give you back
    a result you were not expecting. Or, you may initialize the client with an
    empty string for an account sid, which means the URL will get truncated in
    the middle (note the consecutive slashes):

        GET /2010-04-01/Accounts//Calls/CA123.json

- Using a base URL that is not `https://api.twilio.com`. For example, making
  requests to `https://twilio.com` or `https://www.twilio.com` will not work."#),
            TwilioPlatformError::ErrorCode20154 => None,
            TwilioPlatformError::ErrorCode20107 => None,
            TwilioPlatformError::ErrorCode20155 => None,
            TwilioPlatformError::ErrorCode20153 => None,
            TwilioPlatformError::ErrorCode20156 => None,
            TwilioPlatformError::ErrorCode20104 => None,
            TwilioPlatformError::ErrorCode20102 => None,
            TwilioPlatformError::ErrorCode97001 => Some(r#"Twilio will start dropping webhooks until the authorization server provides a valid token."#),
            TwilioPlatformError::ErrorCode20151 => None,
            TwilioPlatformError::ErrorCode20103 => None,
            TwilioPlatformError::ErrorCode21481 => None,
            TwilioPlatformError::ErrorCode20403 => Some(r#"# Error - 20403

## Forbidden

The account lacks permission to access the Twilio API. Typically this means
the account has been suspended or closed. For assistance, please reach out to Twilio Support through the [Console](https://console.twilio.com) or [Help Center](https://support.twilio.com)."#),
            TwilioPlatformError::ErrorCode20160 => None,
            TwilioPlatformError::ErrorCode20152 => None,
            TwilioPlatformError::ErrorCode20106 => None
        }
    }
}
