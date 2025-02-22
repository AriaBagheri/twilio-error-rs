// AUTO-GENERATED. DO NOT MODIFY. YOUR CHANGES WILL BE OVER-WRITTEN
use crate::products::TwilioOrganizationsApiError;
use standard_error::traits::StandardErrorMessageTrait;
use std::borrow::Cow;

impl StandardErrorMessageTrait for TwilioOrganizationsApiError {
    fn message(&self) -> Cow<'static, str> {
        match self {
            TwilioOrganizationsApiError::ErrorCode25008 => r#"The requested SCIM user was not found"#.into(),
            TwilioOrganizationsApiError::ErrorCode25013 => r#"Updating the username is unsupported"#.into(),
            TwilioOrganizationsApiError::ErrorCode25004 => r#"The provided user's external ID is invalid"#.into(),
            TwilioOrganizationsApiError::ErrorCode25015 => r#"The SCIM PATCH request is invalid"#.into(),
            TwilioOrganizationsApiError::ErrorCode25203 => r#"Listing role assignments requires query parameters"#.into(),
            TwilioOrganizationsApiError::ErrorCode25106 => r#"Failed to complete request due to a bad request"#.into(),
            TwilioOrganizationsApiError::ErrorCode25100 => r#"The organization was not found"#.into(),
            TwilioOrganizationsApiError::ErrorCode25201 => r#"The identity for role assignments must be a managed user"#.into(),
            TwilioOrganizationsApiError::ErrorCode25001 => r#"The specified filter for listing organization users is invalid"#.into(),
            TwilioOrganizationsApiError::ErrorCode25018 => r#"The request is not authorized"#.into(),
            TwilioOrganizationsApiError::ErrorCode25021 => r#"Rate limit exceeded"#.into(),
            TwilioOrganizationsApiError::ErrorCode25022 => r#"Duplicate username or externalId"#.into(),
            TwilioOrganizationsApiError::ErrorCode25002 => r#"The provided user's first name is invalid"#.into(),
            TwilioOrganizationsApiError::ErrorCode25016 => r#"Updating the organization owner is not allowed"#.into(),
            TwilioOrganizationsApiError::ErrorCode25007 => r#"The organization has reached its limit for managed users"#.into(),
            TwilioOrganizationsApiError::ErrorCode25006 => r#"The SCIM schema syntax is invalid"#.into(),
            TwilioOrganizationsApiError::ErrorCode25101 => r#"The organization's account was not found"#.into(),
            TwilioOrganizationsApiError::ErrorCode25023 => r#"Invalid page token"#.into(),
            TwilioOrganizationsApiError::ErrorCode25009 => r#"The user's is in an unupdatable status."#.into(),
            TwilioOrganizationsApiError::ErrorCode25005 => r#"The provided user's username is invalid"#.into(),
            TwilioOrganizationsApiError::ErrorCode25017 => r#"The email domain is unverified"#.into(),
            TwilioOrganizationsApiError::ErrorCode25107 => r#"The Request does not contain any authorization information"#.into(),
            TwilioOrganizationsApiError::ErrorCode25011 => r#"The value for the primary email address is invalid"#.into(),
            TwilioOrganizationsApiError::ErrorCode25109 => r#"Request is rate limited"#.into(),
            TwilioOrganizationsApiError::ErrorCode25003 => r#"The provided user's last name is invalid"#.into(),
            TwilioOrganizationsApiError::ErrorCode25200 => r#"The scope for role assignments must be a managed account"#.into(),
            TwilioOrganizationsApiError::ErrorCode25103 => r#"The organization's account owner is not a managed user"#.into(),
            TwilioOrganizationsApiError::ErrorCode25202 => r#"The role assignment has an invalid role"#.into(),
            TwilioOrganizationsApiError::ErrorCode25019 => r#"Failed to complete request due to a business rule violations"#.into(),
            TwilioOrganizationsApiError::ErrorCode25012 => r#"Updating the email address is unsupported"#.into(),
            TwilioOrganizationsApiError::ErrorCode25014 => r#"The primary email address does not match the username"#.into(),
            TwilioOrganizationsApiError::ErrorCode25104 => r#"The organization's account owner was not set during creation"#.into(),
            TwilioOrganizationsApiError::ErrorCode25102 => r#"The organization's account limit has been reached"#.into(),
            TwilioOrganizationsApiError::ErrorCode25020 => r#"Version conflict in SCIM header"#.into(),
            TwilioOrganizationsApiError::ErrorCode25010 => r#"The primary email address is missing for the user"#.into(),
            TwilioOrganizationsApiError::ErrorCode25105 => r#"The request is not authorized"#.into()
        }
    }
}
