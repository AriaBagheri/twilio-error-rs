// AUTO-GENERATED. DO NOT MODIFY. YOUR CHANGES WILL BE OVER-WRITTEN
use crate::products::TwilioPlatformError;
use standard_error::traits::StandardErrorMessageTrait;
use std::borrow::Cow;

impl StandardErrorMessageTrait for TwilioPlatformError {
    fn message(&self) -> Cow<'static, str> {
        match self {
            TwilioPlatformError::ErrorCode20101 => r#"Invalid Access Token"#.into(),
            TwilioPlatformError::ErrorCode20409 => r#"Conflict"#.into(),
            TwilioPlatformError::ErrorCode20162 => r#"A conflicting resource update is in progress"#.into(),
            TwilioPlatformError::ErrorCode20157 => r#"Expiration Time Exceeds Maximum Time Allowed"#.into(),
            TwilioPlatformError::ErrorCode21102 => r#"Reached maximum number of Services"#.into(),
            TwilioPlatformError::ErrorCode20159 => r#"Invalid Signature"#.into(),
            TwilioPlatformError::ErrorCode20105 => r#"Access Token not yet valid"#.into(),
            TwilioPlatformError::ErrorCode20404 => r#"Not Found"#.into(),
            TwilioPlatformError::ErrorCode20154 => r#"Invalid Claim Set"#.into(),
            TwilioPlatformError::ErrorCode20107 => r#"Invalid Access Token signature"#.into(),
            TwilioPlatformError::ErrorCode20155 => r#"Expiration Time In The Future"#.into(),
            TwilioPlatformError::ErrorCode20153 => r#"Invalid Issuer Or Subject"#.into(),
            TwilioPlatformError::ErrorCode20156 => r#"Expired or Invalid Expiration in Token"#.into(),
            TwilioPlatformError::ErrorCode20104 => r#"Access Token expired or expiration date invalid"#.into(),
            TwilioPlatformError::ErrorCode20102 => r#"Invalid Access Token header"#.into(),
            TwilioPlatformError::ErrorCode97001 => r#"Unable to retrieve OAuth access token"#.into(),
            TwilioPlatformError::ErrorCode20151 => r#"Authentication Failed"#.into(),
            TwilioPlatformError::ErrorCode20103 => r#"Invalid Access Token issuer/subject"#.into(),
            TwilioPlatformError::ErrorCode21481 => r#"Invalid PageToken"#.into(),
            TwilioPlatformError::ErrorCode20403 => r#"403 Forbidden"#.into(),
            TwilioPlatformError::ErrorCode20160 => r#"Invalid Token"#.into(),
            TwilioPlatformError::ErrorCode20152 => r#"Invalid Header"#.into(),
            TwilioPlatformError::ErrorCode20106 => r#"Invalid Access Token grants"#.into()
        }
    }
}
