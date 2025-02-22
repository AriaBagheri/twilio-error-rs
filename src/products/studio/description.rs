// AUTO-GENERATED. DO NOT MODIFY. YOUR CHANGES WILL BE OVER-WRITTEN
use crate::products::TwilioStudioError;
use standard_error::traits::StandardErrorDescriptionTrait;

impl StandardErrorDescriptionTrait for TwilioStudioError {
    fn description(&self) -> Option<&'static str> {
        match self {
            TwilioStudioError::ErrorCode81002 => Some(r#"An unexpected event was received while processing a widget. Studio ignored this event and did not transition to another widget because there was no matching transition available to handle the event. 

*Note:* If the Execution ended correctly, this warning can be ignored."#),
            TwilioStudioError::ErrorCode81026 => Some(r#"The Execution failed because the cumulative number of widgets in your Studio Flow and all its linked Subflows exceeds the maximum allowed widget count of 2,000. All Executions for this Flow will fail until you reduce the number of widgets to 2,000 or fewer."#),
            TwilioStudioError::ErrorCode81007 => None,
            TwilioStudioError::ErrorCode81027 => Some(r#"An error occurred when Studio attempted to parse a type in a widget"#),
            TwilioStudioError::ErrorCode81014 => Some(r#"An internal error occurred while Studio was executing an HTTP Request widget. "#),
            TwilioStudioError::ErrorCode81012 => None,
            TwilioStudioError::ErrorCode81019 => Some(r#"The Twilio phone number connected to your Studio Flow is configured for the deprecated 2008-08-01 API, which is not compatible. Update the phone number API version to 2010-04-01."#),
            TwilioStudioError::ErrorCode81025 => Some(r#"The cumulative number of widgets in your Studio Flow and all its linked Subflows exceeds the maximum allowed widget count of 2,000. You must reduce the number of widgets to 2,000 or fewer to prevent future Executions from failing."#),
            TwilioStudioError::ErrorCode81017 => None,
            TwilioStudioError::ErrorCode81006 => None,
            TwilioStudioError::ErrorCode81005 => None,
            TwilioStudioError::ErrorCode81016 => None,
            TwilioStudioError::ErrorCode81024 => None,
            TwilioStudioError::ErrorCode81004 => None,
            TwilioStudioError::ErrorCode81008 => None,
            TwilioStudioError::ErrorCode81013 => None,
            TwilioStudioError::ErrorCode81018 => Some(r#"An error occurred when Studio tried to evaluate your template syntax."#),
            TwilioStudioError::ErrorCode81021 => Some(r#"The Revision was invalid. Revision must be an integer or enum(LatestPublished, LatestRevision)."#),
            TwilioStudioError::ErrorCode81022 => Some(r#"The Flow Definition is invalid. Check the error response details for an array of validation failures."#),
            TwilioStudioError::ErrorCode81001 => None,
            TwilioStudioError::ErrorCode81000 => Some(r#"This limitation is enforced to prevent infinite loops. Please try to design your flows such that they terminate."#),
            TwilioStudioError::ErrorCode81009 => None,
            TwilioStudioError::ErrorCode81010 => None,
            TwilioStudioError::ErrorCode81015 => None,
            TwilioStudioError::ErrorCode81020 => Some(r#"An error occurred when Studio tried to run a widget that is not supported for the trigger type."#),
            TwilioStudioError::ErrorCode81023 => Some(r#"The Create Execution endpoint was called with To and/or From parameters that could not be processed."#),
            TwilioStudioError::ErrorCode81011 => None
        }
    }
}
