// AUTO-GENERATED. DO NOT MODIFY. YOUR CHANGES WILL BE OVER-WRITTEN
use crate::products::TwilioAiAssistantsError;
use standard_error::traits::StandardErrorSolutionsTrait;

impl StandardErrorSolutionsTrait for TwilioAiAssistantsError {
    fn solutions(&self) -> Option<&'static str> {
        match self {
            TwilioAiAssistantsError::ErrorCode99001 => Some(r#"No further information available."#),
            TwilioAiAssistantsError::ErrorCode99004 => Some(r#"No further action is required by you."#),
            TwilioAiAssistantsError::ErrorCode99002 => Some(r#"Check the status code and error logs in your Tool. 
* If you are using Twilio Functions, try adding more `console.logs`, setting Live Logs on the Function, and executing your Assistants again.
* If you are getting 401/403, verify you are handling the X-Twilio-Signature correctly.
* If you are getting 5xx, check your server logs."#),
            TwilioAiAssistantsError::ErrorCode99006 => Some(r#"* Make sure you have `export type Data = ` as your entry point for the input schema
* Verify the provided schema is a valid TypeScript definition"#),
            TwilioAiAssistantsError::ErrorCode99003 => Some(r#"AI Assistants ensure safe, ethical and compliant interactions. Check the provided prompt for any of the following categories:
* Harmful or dangerous content
* Illegal activities
* Medical advice
* Bias, discrimination and hate speech
* Prompt injection
* Sexual and inappropriate content"#),
            TwilioAiAssistantsError::ErrorCode99005 => Some(r#"* Verify the provided URL is accessible to the public web."#)
        }
    }
}
