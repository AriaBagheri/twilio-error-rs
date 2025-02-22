// AUTO-GENERATED. DO NOT MODIFY. YOUR CHANGES WILL BE OVER-WRITTEN
use crate::products::TwilioStudioError;
use standard_error::traits::StandardErrorSolutionsTrait;

impl StandardErrorSolutionsTrait for TwilioStudioError {
    fn solutions(&self) -> Option<&'static str> {
        match self {
            TwilioStudioError::ErrorCode81002 => Some(r#"* Incoming Calls only: Enable [Concurrent Calls](https://www.twilio.com/docs/studio/user-guide#handling-concurrent-calls-from-the-same-number-in-studio) in the Flow's Trigger widget.
* REST API-triggered Executions: Use [Studio REST API v2](https://www.twilio.com/docs/studio/rest-api/v2/execution#create-a-new-execution) to create Executions and end any that are still active for the same contact.
* Return results for Function calls and HTTP requests quickly.
* End any [stuck Executions](https://support.twilio.com/hc/en-us/articles/360019383714-Understanding-and-Avoiding-Stuck-Executions-in-Twilio-Studio) via Twilio Console."#),
            TwilioStudioError::ErrorCode81026 => Some(r#"* Reduce the number of widgets in your Subflows"#),
            TwilioStudioError::ErrorCode81007 => None,
            TwilioStudioError::ErrorCode81027 => Some(r#"* Verify the runtime data of your flow execution by checking the Flow Data and inspecting any variables you may have referenced
* Ensure that the configured type matches the variable data."#),
            TwilioStudioError::ErrorCode81014 => Some(r#"* Remove reserved words from the parameter names, e.g. "method"
* Check Flow logs to ensure a valid URL is being used in Liquid variables
* Retry the request
* Check [Twilio Status](https://status.twilio.com/)
* Contact [Twilio Support](https://www.twilio.com/help/contact)"#),
            TwilioStudioError::ErrorCode81012 => None,
            TwilioStudioError::ErrorCode81019 => Some(r#"Update the phone number API version to 2010-04-01"#),
            TwilioStudioError::ErrorCode81025 => Some(r#"* Reduce the number of widgets in your Subflows"#),
            TwilioStudioError::ErrorCode81017 => None,
            TwilioStudioError::ErrorCode81006 => None,
            TwilioStudioError::ErrorCode81005 => None,
            TwilioStudioError::ErrorCode81016 => None,
            TwilioStudioError::ErrorCode81024 => Some(r#"* Ensure Subflow exists
* Remove Subflow widget from Subflow definition"#),
            TwilioStudioError::ErrorCode81004 => None,
            TwilioStudioError::ErrorCode81008 => None,
            TwilioStudioError::ErrorCode81013 => None,
            TwilioStudioError::ErrorCode81018 => Some(r#"* Check the syntax used in your widget (See Liquid docs [here](https://www.twilio.com/docs/studio/user-guide/liquid-template-language))
* Verify the runtime data of your flow execution by checking the Flow Data and inspecting any variables you may have referenced
* Verify that your Liquid curly braces are matched
* Verify that all tags and filters used in your template are correct (The list of available filters is [here](https://www.twilio.com/docs/studio/user-guide/liquid-template-language#standard-filters))
* Use only ASCII characters in variable names"#),
            TwilioStudioError::ErrorCode81021 => Some(r#"* Check the value and ensure it is either an integer or one of the magic identifiers: LatestPublished or LatestRevision"#),
            TwilioStudioError::ErrorCode81022 => Some(r#"* Verify the definition adheres to the JSON schema
* Ensure widget names are all unique
* Remove transitions that reference widgets that don't exist"#),
            TwilioStudioError::ErrorCode81001 => None,
            TwilioStudioError::ErrorCode81000 => Some(r#"* Reduce the number of steps in your flow."#),
            TwilioStudioError::ErrorCode81009 => None,
            TwilioStudioError::ErrorCode81010 => None,
            TwilioStudioError::ErrorCode81015 => None,
            TwilioStudioError::ErrorCode81020 => Some(r#"* Flow should be updated to use only the supported trigger types for this widget
* For Send To Flex widget, only incoming call and incoming chat are supported"#),
            TwilioStudioError::ErrorCode81023 => Some(r#"Double-check the provided To and From parameters for syntax errors."#),
            TwilioStudioError::ErrorCode81011 => None
        }
    }
}
