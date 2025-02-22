// AUTO-GENERATED. DO NOT MODIFY. YOUR CHANGES WILL BE OVER-WRITTEN
use crate::products::TwilioOrganizationsApiError;
use standard_error::traits::StandardErrorDescriptionTrait;

impl StandardErrorDescriptionTrait for TwilioOrganizationsApiError {
    fn description(&self) -> Option<&'static str> {
        match self {
            TwilioOrganizationsApiError::ErrorCode25008 => Some(r#"The requested SCIM user was not found."#),
            TwilioOrganizationsApiError::ErrorCode25013 => Some(r#"Updating the username is unsupported."#),
            TwilioOrganizationsApiError::ErrorCode25004 => Some(r#"The provided user's external ID is invalid."#),
            TwilioOrganizationsApiError::ErrorCode25015 => Some(r#"The `SCIM PATCH` request is invalid."#),
            TwilioOrganizationsApiError::ErrorCode25203 => Some(r#"Listing role assignments requires query parameters."#),
            TwilioOrganizationsApiError::ErrorCode25106 => Some(r#"Failed to complete request due to a bad request"#),
            TwilioOrganizationsApiError::ErrorCode25100 => Some(r#"The organization was not found."#),
            TwilioOrganizationsApiError::ErrorCode25201 => Some(r#"The identity for role assignments must be a managed user."#),
            TwilioOrganizationsApiError::ErrorCode25001 => Some(r#"The specified filter for listing organization users is invalid."#),
            TwilioOrganizationsApiError::ErrorCode25018 => Some(r#"The request is not authorized."#),
            TwilioOrganizationsApiError::ErrorCode25021 => Some(r#"Rate limit exceeded."#),
            TwilioOrganizationsApiError::ErrorCode25022 => Some(r#"Duplicate username or externalId"#),
            TwilioOrganizationsApiError::ErrorCode25002 => Some(r#"The provided user's first name is invalid."#),
            TwilioOrganizationsApiError::ErrorCode25016 => Some(r#"Updating the organization owner is not allowed."#),
            TwilioOrganizationsApiError::ErrorCode25007 => Some(r#"The organization has reached its limit for managed users."#),
            TwilioOrganizationsApiError::ErrorCode25006 => Some(r#"The SCIM schema syntax is invalid."#),
            TwilioOrganizationsApiError::ErrorCode25101 => Some(r#"The organization's account was not found."#),
            TwilioOrganizationsApiError::ErrorCode25023 => Some(r#"Invalid page token"#),
            TwilioOrganizationsApiError::ErrorCode25009 => Some(r#"The user's is in an unupdatable status."#),
            TwilioOrganizationsApiError::ErrorCode25005 => Some(r#"The provided user's username is invalid."#),
            TwilioOrganizationsApiError::ErrorCode25017 => Some(r#"The email domain is unverified."#),
            TwilioOrganizationsApiError::ErrorCode25107 => Some(r#"The Request does not contain any authorization information (most cases: credentials) or the credentials and the resource have no relation."#),
            TwilioOrganizationsApiError::ErrorCode25011 => Some(r#"The value for the primary email address is invalid."#),
            TwilioOrganizationsApiError::ErrorCode25109 => Some(r#"Request is rate limited."#),
            TwilioOrganizationsApiError::ErrorCode25003 => Some(r#"The provided user's last name is invalid."#),
            TwilioOrganizationsApiError::ErrorCode25200 => Some(r#"The scope for role assignments must be a managed account."#),
            TwilioOrganizationsApiError::ErrorCode25103 => Some(r#"The organization's account owner is not a managed user."#),
            TwilioOrganizationsApiError::ErrorCode25202 => Some(r#"The role assignment has an invalid role."#),
            TwilioOrganizationsApiError::ErrorCode25019 => Some(r#"Failed to complete request due to a business rule violations."#),
            TwilioOrganizationsApiError::ErrorCode25012 => Some(r#"Updating the email address is unsupported."#),
            TwilioOrganizationsApiError::ErrorCode25014 => Some(r#"The primary email address does not match the username."#),
            TwilioOrganizationsApiError::ErrorCode25104 => Some(r#"The organization's account owner was not set during creation."#),
            TwilioOrganizationsApiError::ErrorCode25102 => Some(r#"The organization's account limit has been reached."#),
            TwilioOrganizationsApiError::ErrorCode25020 => Some(r#"Version conflict in SCIM header."#),
            TwilioOrganizationsApiError::ErrorCode25010 => Some(r#"The primary email address is missing for the user."#),
            TwilioOrganizationsApiError::ErrorCode25105 => Some(r#"The request is not authorized."#)
        }
    }
}
