// AUTO-GENERATED. DO NOT MODIFY. YOUR CHANGES WILL BE OVER-WRITTEN
use crate::products::TwilioFunctionsError;
use standard_error::traits::StandardErrorSolutionsTrait;

impl StandardErrorSolutionsTrait for TwilioFunctionsError {
    fn solutions(&self) -> Option<&'static str> {
        match self {
            TwilioFunctionsError::ErrorCode82001 => Some(r#"* Check the Function route you are invoking exists.
* Check whether the Function's *Access Control* is set to Private and whether you intended to have it set as so. If it is Private, then your request must be properly signed."#),
            TwilioFunctionsError::ErrorCode82009 => Some(r#"Use the new Functions/Assets UI or Serverless API, and upgrade your @twilio/runtime-handler dependency version to 1.2.0 or higher. Please refer to [this](https://www.twilio.com/docs/serverless/functions-assets/handler) for more information about the Runtime Handler, how to upgrade the version, and the currently available versions.
For Classic Functions, or incompatible Runtime Handler versions, only String-valued headers are permitted."#),
            TwilioFunctionsError::ErrorCode82006 => Some(r#"Try reducing number of Environment Variables or size of the Environment Variables stored."#),
            TwilioFunctionsError::ErrorCode82005 => Some(r#"Check the highlighted message above for further debugging"#),
            TwilioFunctionsError::ErrorCode82002 => Some(r#"* Your Function must contain a callback.
* Make sure you place the Function callback `callback(err, response)` is placed correctly in your Function code.
 * If you are using a JavaScript promise, make sure the callback is called in both success and catch blocks.
* Your Function responded with an error."#),
            TwilioFunctionsError::ErrorCode82004 => Some(r#"Check the highlighted message above for further debugging"#),
            TwilioFunctionsError::ErrorCode82007 => Some(r#"* Determine what [runtime versions are currently available](https://www.twilio.com/docs/serverless/functions-assets/faq#which-runtimes-are-available-for-twilio-functions).
* Migrate to a new valid Node.js version by following our migration guide:  [https://www.twilio.com/docs/serverless/functions-assets/node-upgrade](https://www.twilio.com/docs/serverless/functions-assets/node-upgrade)
* When using the Serverless API pass a dedicated valid runtime version when creating a new Build: [https://www.twilio.com/docs/serverless/api/resource/build](https://www.twilio.com/docs/serverless/api/resource/build)"#),
            TwilioFunctionsError::ErrorCode82008 => Some(r#"1. Pass cookies and headers less than 4kb. Set cookies less than 4kb in the response.
2. Send less than 75 headers. Set less than 75 headers/cookies in the response."#),
            TwilioFunctionsError::ErrorCode82003 => Some(r#"* Retry deployment after sometime."#)
        }
    }
}
