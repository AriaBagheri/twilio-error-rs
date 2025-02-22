// AUTO-GENERATED. DO NOT MODIFY. YOUR CHANGES WILL BE OVER-WRITTEN
use crate::products::TwilioNotifyError;
use standard_error::traits::StandardErrorDocsTrait;

impl StandardErrorDocsTrait for TwilioNotifyError {
    fn docs(&self) -> &'static str {
        match self {
            TwilioNotifyError::ErrorCode52402 => r#"## ERROR - 52402

### General rendering problem with variables in a parsed template

 The error pertains to a general rendering issue with variables in an successfully parsed template.

#### Possible Causes
Various limitations such as reaching the maximum allowed template size of 6400 bytes or encountering a rendering timeout of 3000 milliseconds might contribute to this error.

#### Possible Solutions
To pinpoint the underlying cause, customers should inspect the debugger logs in the console for further details."#,
            TwilioNotifyError::ErrorCode52117 => r#"## ERROR - 52117

### Deprecated GCM/FCM API is used to send notifications

Migrate to FCMv1 Credentials to send notifications. Google has deprecated the legacy FCM API in June 2024: https://help.twilio.com/articles/20768292997147-Updating-Twilio-Push-for-FCM-HTTP-v1-API

#### Possible Causes
Detected the use of a legacy FCM server key credential. This credential has stopped working after Google has deprecated the legacy FCM API in June 2024. Your action is required in order for FCM push notifications to continue to work as the legacy FCM API is deprecated.

#### Possible Solutions
Use a new FCMv1 credential to keep sending push notifications. See instructions linked in the description above."#,
            TwilioNotifyError::ErrorCode52311 => r#"## ERROR - 52311

### Delivery callback invocation failed

Notify is unable to invoke the given callback 

#### Possible Causes
Invalid callback URL or Internal Server Error

#### Possible Solutions
Check the callback URL"#,
            TwilioNotifyError::ErrorCode52310 => r#"## ERROR - 52310

### Notification TTL has expired

 Notification expired during processing

#### Possible Causes
Notification took too much time or there are too many notifications

#### Possible Solutions
Increase TTL for notification or try again later"#,
            TwilioNotifyError::ErrorCode52403 => r#"## ERROR - 52403

### Provided text template in the notification request cannot be parsed

 An error may arise if the provided text template in the notification request cannot be parsed, potentially due to syntax errors in the input.

#### Possible Causes
The error might be attributed to syntax errors within the provided text template.

#### Possible Solutions
To identify the precise issue, customers are advised to examine the debugger logs in the console for detailed error messages."#,
            TwilioNotifyError::ErrorCode52115 => r#"## ERROR - 52115

### Service account does not have permission to send FCM messages

 The service account associated with the provided FCM credentials does not have the required permissions to send push notifications

#### Possible Causes
The role(s) granted to the provided service account do not include the cloudmessaging.messages.create permission

#### Possible Solutions
Update the roles of the service account associated with the given FCM credentials in Google Cloud console to include the cloudmessaging.messages.create permission or provide credentials for a service account that has this permission"#,
            TwilioNotifyError::ErrorCode52404 => r#"## ERROR - 52404

### Undefined variable within the notification payload template

 The error indicates that there are one or more undefined variables in the provided template of the notification payload.

#### Possible Causes
1. Missing variables might have been overlooked during template creation.
2. Variables could have been mistyped or not properly declared.

#### Possible Solutions
1. Check the debugger logs in the console to identify the exact missing variables.
2. Ensure that all necessary variables are included in either or both of the per-recipient object and the default variables of the notification request.
3. Double-check the spelling and syntax of variables in the template."#,
            TwilioNotifyError::ErrorCode52400 => r#"## ERROR - 52400

### Exceeded maximum iterations in template rendering

The maximum number of defined iterations in templated fields of the notification payload has been reached. Your template contains an iterative operation that has reached the maximum allowed limit of 1000 iterations in its definition.

#### Possible Causes
Some syntax-specific iteration operators in the template, such as for-loops, cycles, table-rows, and others, may have reached the maximum limit.

#### Possible Solutions
1. Consider reducing the number of iterations required in your template.
2. Review the template structure to optimize the use of iteration operations.
3. Break down complex iterations into smaller, more manageable chunks if possible."#
        }
    }
}
