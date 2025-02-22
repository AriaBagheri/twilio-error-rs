// AUTO-GENERATED. DO NOT MODIFY. YOUR CHANGES WILL BE OVER-WRITTEN
use crate::products::TwilioPlatformError;
use standard_error::traits::StandardErrorDocsTrait;

impl StandardErrorDocsTrait for TwilioPlatformError {
    fn docs(&self) -> &'static str {
        match self {
            TwilioPlatformError::ErrorCode20101 => r#"# Error - 20101

## Twilio was unable to validate your Access Token

The Access Token provided to the Twilio API was invalid.

To check whether the Access Token is structurally correct,
you can use the tools available at [jwt.io](https://jwt.io/).

For the details of Twilio's specific Access Token implementation, see
[the documentation](/docs/api/rest/access-tokens).
"#,
            TwilioPlatformError::ErrorCode20409 => r#"## ERROR - 20409

### Conflict

 The request could not be completed due to a conflict with the current state of the target resource.

#### Possible Causes
The resource is not in a state that allows this modification.

#### Possible Solutions
Wait until the resource is in a modifiable state."#,
            TwilioPlatformError::ErrorCode20162 => r#"## ERROR - 20162

### A conflicting resource update is in progress

 The requested resource is already being updated simultaneously by another API request. The last request will be rejected to avoid inconsistent state.

#### Possible Causes
The same API resource is being updated by multiple simultaneous API requests from  different threads in a conflicting fashion.

#### Possible Solutions
* Implement an operation retrier and repeat the failed API requests after an interval of time using, exponential backoff algorithm.
* Serialize your API requests that modify the same resources, and wait until the original request completes and returns an API response, before sending any successive modifications."#,
            TwilioPlatformError::ErrorCode20157 => r#"## Error - 20157 

### Expiration Time Exceeds Maximum Time Allowed
The expiration time provided when creating the JWT exceeds 24 hours, which is the maximum duration allowed.

#### Possible Solution:
* Provide a shorter expiration time, which does not exceed the maximum.

See [the capability token docs](https://www.twilio.com/docs/voice/client/capability-tokens) to learn more."#,
            TwilioPlatformError::ErrorCode21102 => r#"## Error - 21102

### Reached maximum number of Services

#### Possible Causes
You have attempted to create a Service, but your parent account already has the maximum amount for this product.

#### Possible Solutions
* Contact support to increase your Service limit
* Delete some of your Services"#,
            TwilioPlatformError::ErrorCode20159 => r#"## Error - 20159 

### Invalid Signature
The JWT provided to the Twilio API has an invalid or mismatching signature.

#### Possible Causes 
* Public key in the JWT does not match the actual private key used for signing the request.

#### Possible Solution:
* Make sure the private key used for signing the JWT matches the public key identifier in JWT.
* To check whether the JWT is structurally correct, you can use the tools available at [jwt.io](https://jwt.io/)."#,
            TwilioPlatformError::ErrorCode20105 => r#"# Error - 20105

## Access Token not yet valid

The Access Token provided to the Twilio API is not yet valid.

Twilio Access Tokens are generated with a `nbf` field that determines the
start of the token's lifetime. Normally this is set to the current time
when the token is generated; check whether your computer's clock has drifted.

To check whether the Access Token is structurally correct,
you can use the tools available at [jwt.io](https://jwt.io/).

For the details of Twilio's specific Access Token implementation and the
requirements of the issuer and subject fields, see [the
documentation](/docs/api/rest/access-tokens)."#,
            TwilioPlatformError::ErrorCode20404 => r#"## ERROR - 20404

### Not Found

The resource was not found. ## Not Found

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
  requests to `https://twilio.com` or `https://www.twilio.com` will not work.

#### Possible Causes
-

#### Possible Solutions
-"#,
            TwilioPlatformError::ErrorCode20154 => r#"## Error - 20154 

### Invalid Claim Set
The JWT could be parsed, but the claims in the payload section are invalid.

#### Possible Causes 
* One of the required claims is missing.
* For JWTs used as part of Public Key Client Validation, we require two custom claims:
  * “hrh” - HTTP headers that were included when calculating the request hash
  * “rqh” - The hash of the HTTP request

#### Possible Solution:
* Make sure you are using the latest Twilio Helper Library
* For the details on JWTs used as part of Public Key Client Validation, see [the documentation](/docs/api/credentials/public-key-client-validation-getting-started)
* To check whether the JWT is structurally correct, you can use the tools available at jwt.io.
"#,
            TwilioPlatformError::ErrorCode20107 => r#"# Error - 20107

## Invalid Access Token signature

The signature for the Access Token provided was invalid.

Check that your code is correctly generating signatures using the algorithm and format expected by the Twilio API.

To check whether the Access Token is structurally correct, you can use the tools available at [jwt.io](https://jwt.io/).

Access Tokens are used by multiple Twilio SDKs, including Programmable Voice, Programmable Video, and Programmable Chat. For the details of Twilio's specific Access Token implementation, see [the documentation](/docs/api/rest/access-tokens).

### Possible Causes

* The supplied Account SID, API Key or API Secret is incorrect.

### Possible Solutions

* Ensure you're using the correct Account SID for your [Live API Credentials](/console/account/settings).
* Verify the API Key was generated for the same account as the supplied Account SID. (The API Secret is not the same as the Account Auth Token.)
* Verify the API Secret is correct for the API Key being used and that no extra characters are included.
* Check if the API Secret passes signature validation when used at [jwt.io](https://jwt.io/).
* Generate a new API Key and Secret for the Account SID and try again."#,
            TwilioPlatformError::ErrorCode20155 => r#"## Error - 20155 

### Expiration Time In The Future
The JWT provided to the Twilio API is not yet valid.

#### Possible Causes 
* The client system that generates the JWT might have issues with system time."#,
            TwilioPlatformError::ErrorCode20153 => r#"## Error - 20153 

### Invalid Issuer Or Subject
The issuer or subject of the JWT provided to the Twilio API was invalid.

#### Possible Causes 
* The issuer is missing from JWT
* The issuer entity is suspended in Twilio.
* The subject is missing from JWT
* The subject entity is suspended in Twilio.
* The issuer and subject are not related.

#### Possible Solution:
* In case of Public Key Client Validation, make sure the account and the API key are valid and not suspended.
* For the details on JWTs used as part of Public Key Client Validation, see [the documentation](/docs/api/credentials/public-key-client-validation-getting-started)
* To check whether the JWT is structurally correct, you can use the tools available at jwt.io."#,
            TwilioPlatformError::ErrorCode20156 => r#"## Error - 20156

### Expired or Invalid Expiration in Token
The JWT provided to the Twilio API has expired, or the expiration time specified in the JWT was invalid.


#### Possible Causes 
* The client system that generates the JWT might have issues with system time.
* To check whether the JWT is structurally correct, you can use the tools available at jwt.io."#,
            TwilioPlatformError::ErrorCode20104 => r#"# Warning - 20104

## Access Token has expired or expiration date is invalid

The Access Token provided to the Twilio API has expired, the expiration time
specified in the token was invalid, or the expiration time specified was too
far in the future.

Access Token expiration times can be set up to 24 hours in the future.

To check whether the Access Token is structurally correct, you can use the tools available at [jwt.io](https://jwt.io/).

Access Tokens are used by multiple Twilio SDKs, including Programmable Voice, Programmable Video, and Programmable Chat. For the details of Twilio's specific Access Token implementation, see [the documentation](/docs/api/rest/access-tokens)."#,
            TwilioPlatformError::ErrorCode20102 => r#"# Error - 20102

## Invalid Access Token header

The header of the Access Token provided to the Twilio API was
invalid.

To check whether the Access Token is structurally correct,
you can use the tools available at [jwt.io](https://jwt.io/).

For the details of Twilio's specific Access Token implementation and the header
requirements, see [the documentation](/docs/api/rest/access-tokens).
"#,
            TwilioPlatformError::ErrorCode97001 => r#"## ERROR - 97001

### Unable to retrieve OAuth access token

Unable to exchange OAuth client credentials for an access token in the authorization server. Twilio will start dropping webhooks until the authorization server provides a valid token.

#### Possible Causes
* The authorization service is unavailable.
* The authorization service is unreachable.
* The client credentials you configured have been rotated or removed.
* The authorization server has started issuing a token which is not of type `Bearer`.

#### Possible Solutions
* Verify your authorization server is running and healthy.
* Verify your authorization server is not protected by a firewall which blocks the requests.
* Update the webhook settings with valid client credentials.
* Update the webhook settings to disable OAuth.
* Verify the authorization server issues tokens of type `Bearer`."#,
            TwilioPlatformError::ErrorCode20151 => r#"## Error - 20151

### Authentication Failed
The Authentication with the provided JWT failed.


#### Possible Causes 
* The system could not perform the authorization for the given request.

#### Possible Solution:
* Retry, if this keeps happening, please contact the Twilio Support
* To check whether the JWT is structurally correct, you can use the tools available at jwt.io."#,
            TwilioPlatformError::ErrorCode20103 => r#"# Error - 20103

## Invalid Access Token issuer or subject

The issuer or subject of the Access Token provided to the Twilio API was
invalid.

**Issuer:** The [API Key](/docs/api/rest/keys) used to sign the the token.

**Subject:** The Twilio Account SID from your [Live Credentials](/console/account/settings).

### Possible Causes
* Account is inactive or suspended.
* Account SID is incorrect.
* API Key is incorrect or has been deleted.

### Possible Solutions

* Ensure the account is active and there are enough funds available.
* Ensure the Account SID is correct and is from your Live Credentials.
* Make sure that the API Key used to sign the request is correct and associated with the supplied Account SID.

To check whether the Access Token is structurally correct, you can use the tools available at [jwt.io](https://jwt.io/).

Access Tokens are used by multiple Twilio SDKs, including Programmable Voice, Programmable Video, and Programmable Chat. For the details of Twilio's specific Access Token implementation and the requirements of the issuer and subject fields, see [the documentation](/docs/api/rest/access-tokens)."#,
            TwilioPlatformError::ErrorCode21481 => r#"### Invalid PageToken

The `PageToken` in the requested URL was not valid. If you are paging through
a result set, be sure you are requesting the provided paging urls in the
Twilio API response."#,
            TwilioPlatformError::ErrorCode20403 => r#"## ERROR - 20403

### 403 Forbidden

Forbidden # Error - 20403

## Forbidden

The account lacks permission to access the Twilio API. Typically this means
the account has been suspended or closed. For assistance, please reach out to Twilio Support through the [Console](https://console.twilio.com) or [Help Center](https://support.twilio.com).

#### Possible Causes
The account has been suspended or closed.

#### Possible Solutions
Reach out to Twilio Support."#,
            TwilioPlatformError::ErrorCode20160 => r#"## Error - 20160 

### Invalid Token
The JWT provided to the Twilio API is not a valid JWT.

#### Possible Solution:
* Make sure you are using the latest version of Twilio Helper libraries
* To check whether the JWT is structurally correct, you can use the tools available at [jwt.io](https://jwt.io/)."#,
            TwilioPlatformError::ErrorCode20152 => r#"## Error - 20152

### Invalid Header
The header of the JWT provided to the Twilio API was invalid.

#### Possible Causes 
* The structure of the header was invalid.
* Required header values not provided
* Provided header values invalid.

#### Possible Solution:
* For the details on JWTs used as part of Public Key Client Validation, see [the documentation](/docs/api/credentials/public-key-client-validation-getting-started)
* To check whether the JWT is structurally correct, you can use the tools available at jwt.io."#,
            TwilioPlatformError::ErrorCode20106 => r#"# Error - 20106

## Invalid Access Token grants

The Access Token signature and issuer were valid, but the grants specified
in the token were invalid, unparseable, or did not authorize the action
being requested.

To check whether the Access Token is structurally correct,
you can use the tools available at [jwt.io](https://jwt.io/).

For the details of Twilio's specific Access Token implementation including the
grant format, see
[the documentation](/docs/api/rest/access-tokens)."#
        }
    }
}
