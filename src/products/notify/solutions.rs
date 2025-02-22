// AUTO-GENERATED. DO NOT MODIFY. YOUR CHANGES WILL BE OVER-WRITTEN
use crate::products::TwilioNotifyError;
use standard_error::traits::StandardErrorSolutionsTrait;

impl StandardErrorSolutionsTrait for TwilioNotifyError {
    fn solutions(&self) -> Option<&'static str> {
        match self {
            TwilioNotifyError::ErrorCode52402 => Some(r#"To pinpoint the underlying cause, customers should inspect the debugger logs in the console for further details."#),
            TwilioNotifyError::ErrorCode52117 => Some(r#"Use a new FCMv1 credential to keep sending push notifications. See instructions linked in the description above."#),
            TwilioNotifyError::ErrorCode52311 => Some(r#"Check the callback URL"#),
            TwilioNotifyError::ErrorCode52310 => Some(r#"Increase TTL for notification or try again later"#),
            TwilioNotifyError::ErrorCode52403 => Some(r#"To identify the precise issue, customers are advised to examine the debugger logs in the console for detailed error messages."#),
            TwilioNotifyError::ErrorCode52115 => Some(r#"Update the roles of the service account associated with the given FCM credentials in Google Cloud console to include the cloudmessaging.messages.create permission or provide credentials for a service account that has this permission"#),
            TwilioNotifyError::ErrorCode52404 => Some(r#"1. Check the debugger logs in the console to identify the exact missing variables.
2. Ensure that all necessary variables are included in either or both of the per-recipient object and the default variables of the notification request.
3. Double-check the spelling and syntax of variables in the template."#),
            TwilioNotifyError::ErrorCode52400 => Some(r#"1. Consider reducing the number of iterations required in your template.
2. Review the template structure to optimize the use of iteration operations.
3. Break down complex iterations into smaller, more manageable chunks if possible."#)
        }
    }
}
