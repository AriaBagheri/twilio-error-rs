// AUTO-GENERATED. DO NOT MODIFY. YOUR CHANGES WILL BE OVER-WRITTEN
use crate::products::TwilioFunctionsError;
use standard_error::traits::StandardErrorDocsTrait;

impl StandardErrorDocsTrait for TwilioFunctionsError {
    fn docs(&self) -> &'static str {
        match self {
            TwilioFunctionsError::ErrorCode82001 => r#"## WARNING - 82001

### Function invocation resulted in StatusCode 4xx

 Your Function invocation returned a 4xx StatusCode.

#### Possible Causes
* Function route does not exist.
* Function is set to Private and invocation request is not signed.

#### Possible Solutions
* Check the Function route you are invoking exists.
* Check whether the Function's *Access Control* is set to Private and whether you intended to have it set as so. If it is Private, then your request must be properly signed."#,
            TwilioFunctionsError::ErrorCode82009 => r#"## ERROR - 82009

### Multi-valued headers not supported

 Twilio was unable to process the correct response to your Function invocation because your version of Functions is incompatible with multi-valued HTTP response headers.

#### Possible Causes
A Function invocation has resulted in a Twilio Response with multi-valued headers (i.e. the header value is an array). Multi-valued headers are not supported in Classic Functions, or in @twilio/runtime-handler dependency versions below 1.2.0.

#### Possible Solutions
Use the new Functions/Assets UI or Serverless API, and upgrade your @twilio/runtime-handler dependency version to 1.2.0 or higher. Please refer to [this](https://www.twilio.com/docs/serverless/functions-assets/handler) for more information about the Runtime Handler, how to upgrade the version, and the currently available versions.
For Classic Functions, or incompatible Runtime Handler versions, only String-valued headers are permitted."#,
            TwilioFunctionsError::ErrorCode82006 => r#"## ERROR - 82006

### Environment Context too large

 Context of the function has exceeded maximum limit. Reduce the context size and try again.

#### Possible Causes
Context size consists of account metadata, domain name, path and Environment variables stored as a JSON string. If this string exceeds 3KB, invocations may fail.

#### Possible Solutions
Try reducing number of Environment Variables or size of the Environment Variables stored."#,
            TwilioFunctionsError::ErrorCode82005 => r#"## ERROR - 82005

### Function execution resulted in an error log

 Your Function execution resulted in logging an error message

#### Possible Causes
Function execution code path lead to console.error(...)

#### Possible Solutions
Check the highlighted message above for further debugging"#,
            TwilioFunctionsError::ErrorCode82002 => r#"## ERROR - 82002

### Error on Twilio Function response

 Your Function invocation resulted in StatusCode 5xx.

#### Possible Causes
* Your Function timed out before responding.
* Your Function returned an error response.

#### Possible Solutions
* Your Function must contain a callback.
* Make sure you place the Function callback `callback(err, response)` is placed correctly in your Function code.
 * If you are using a JavaScript promise, make sure the callback is called in both success and catch blocks.
* Your Function responded with an error."#,
            TwilioFunctionsError::ErrorCode82004 => r#"## WARNING - 82004

### Function execution resulted in a warning log

 Your Function execution resulted in logging a warning message

#### Possible Causes
Function execution code path lead to console.warn(...)

#### Possible Solutions
Check the highlighted message above for further debugging"#,
            TwilioFunctionsError::ErrorCode82007 => r#"## ERROR - 82007

### Unsupported Runtime

 The runtime used in the Serverless Build request is not supported on the platform. A runtime defines the environment your Functions will be executed in (e.g. which Node.js version to use for your Functions).

#### Possible Causes
* A Serverless Build request was made without providing the runtime and the runtime used for the last successful build in the service (e.g. a Node.js version that is no longer supported on the platform).
* A Serverless Build request was made with a specified runtime (e.g. node10) that is currently not supported on the platform.

#### Possible Solutions
* Determine what [runtime versions are currently available](https://www.twilio.com/docs/serverless/functions-assets/faq#which-runtimes-are-available-for-twilio-functions).
* Migrate to a new valid Node.js version by following our migration guide:  [https://www.twilio.com/docs/serverless/functions-assets/node-upgrade](https://www.twilio.com/docs/serverless/functions-assets/node-upgrade)
* When using the Serverless API pass a dedicated valid runtime version when creating a new Build: [https://www.twilio.com/docs/serverless/api/resource/build](https://www.twilio.com/docs/serverless/api/resource/build)"#,
            TwilioFunctionsError::ErrorCode82008 => r#"## ERROR - 82008

### Headers or cookies too large

Request or response headers/cookies size is greater than the limits This error is thrown when request or response headers are greater than the enforced limits.

#### Possible Causes
1. The user has passed request/response headers larger than 4kb. 
2. Request/response has more than 75 headers+cookies.

#### Possible Solutions
1. Pass cookies and headers less than 4kb. Set cookies less than 4kb in the response.
2. Send less than 75 headers. Set less than 75 headers/cookies in the response."#,
            TwilioFunctionsError::ErrorCode82003 => r#"## ERROR - 82003

### Deployment Installation Failure

 Your latest Functions and Private Assets failed to be deployed.

#### Possible Causes
* NPM registry is degraded.
* Your Service instance is trying to install a package that requires a gcc compiler. 
* Your Service instance resulted in out of space error due to large packages.

#### Possible Solutions
* Retry deployment after sometime."#
        }
    }
}
