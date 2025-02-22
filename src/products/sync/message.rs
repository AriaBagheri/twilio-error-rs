// AUTO-GENERATED. DO NOT MODIFY. YOUR CHANGES WILL BE OVER-WRITTEN
use crate::products::TwilioSyncError;
use standard_error::traits::StandardErrorMessageTrait;
use std::borrow::Cow;

impl StandardErrorMessageTrait for TwilioSyncError {
    fn message(&self) -> Cow<'static, str> {
        match self {
            TwilioSyncError::ErrorCode54011 => r#"Invalid TTL"#.into(),
            TwilioSyncError::ErrorCode54006 => r#"Request entity too large"#.into(),
            TwilioSyncError::ErrorCode54201 => r#"Map Item not found"#.into(),
            TwilioSyncError::ErrorCode54155 => r#"List Item revision mismatch"#.into(),
            TwilioSyncError::ErrorCode54451 => r#"Invalid 'Order' query parameter"#.into(),
            TwilioSyncError::ErrorCode54351 => r#"Invalid identity"#.into(),
            TwilioSyncError::ErrorCode54156 => r#"Invalid List Item data"#.into(),
            TwilioSyncError::ErrorCode54450 => r#"Invalid 'Direction' query parameter"#.into(),
            TwilioSyncError::ErrorCode54251 => r#"Invalid Message Stream Message data"#.into(),
            TwilioSyncError::ErrorCode54100 => r#"Document not found"#.into(),
            TwilioSyncError::ErrorCode54003 => r#"Invalid If-Match header"#.into(),
            TwilioSyncError::ErrorCode54509 => r#"Query expression contains too many operators"#.into(),
            TwilioSyncError::ErrorCode54103 => r#"Document revision mismatch"#.into(),
            TwilioSyncError::ErrorCode54101 => r#"Invalid Document data"#.into(),
            TwilioSyncError::ErrorCode54507 => r#"Invalid query"#.into(),
            TwilioSyncError::ErrorCode54053 => r#"Invalid friendly name"#.into(),
            TwilioSyncError::ErrorCode54452 => r#"Invalid 'Bounds' query parameter"#.into(),
            TwilioSyncError::ErrorCode54301 => r#"Unique name already exists"#.into(),
            TwilioSyncError::ErrorCode54009 => r#"Rate limit exceeded"#.into(),
            TwilioSyncError::ErrorCode54051 => r#"Invalid webhook URL"#.into(),
            TwilioSyncError::ErrorCode54354 => r#"Permission not found"#.into(),
            TwilioSyncError::ErrorCode54502 => r#"Invalid index name"#.into(),
            TwilioSyncError::ErrorCode54453 => r#"Invalid 'PageToken' query parameter"#.into(),
            TwilioSyncError::ErrorCode54300 => r#"Unique name not found"#.into(),
            TwilioSyncError::ErrorCode54206 => r#"Invalid Map Item data"#.into(),
            TwilioSyncError::ErrorCode54200 => r#"Map not found"#.into(),
            TwilioSyncError::ErrorCode54302 => r#"Invalid unique name"#.into(),
            TwilioSyncError::ErrorCode54208 => r#"Map Item already exists"#.into(),
            TwilioSyncError::ErrorCode54510 => r#"Query expression contains an array with too many items"#.into(),
            TwilioSyncError::ErrorCode54050 => r#"Service Instance not found"#.into(),
            TwilioSyncError::ErrorCode54151 => r#"List Item not found"#.into(),
            TwilioSyncError::ErrorCode54150 => r#"List not found"#.into(),
            TwilioSyncError::ErrorCode54007 => r#"Access forbidden for identity"#.into(),
            TwilioSyncError::ErrorCode54008 => r#"Invalid JSON"#.into(),
            TwilioSyncError::ErrorCode54209 => r#"Invalid Map Item key"#.into(),
            TwilioSyncError::ErrorCode54458 => r#"Invalid List Item index"#.into(),
            TwilioSyncError::ErrorCode54205 => r#"Map Item revision mismatch"#.into(),
            TwilioSyncError::ErrorCode54056 => r#"Account cannot access requested Service Instance"#.into(),
            TwilioSyncError::ErrorCode54010 => r#"No parameters specified"#.into(),
            TwilioSyncError::ErrorCode54454 => r#"Sync: Invalid 'PageSize' query parameter"#.into(),
            TwilioSyncError::ErrorCode54419 => r#"Number of subscriptions per connection is over the limit"#.into(),
            TwilioSyncError::ErrorCode54250 => r#"Message Stream not found"#.into()
        }
    }
}
