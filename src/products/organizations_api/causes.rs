// AUTO-GENERATED. DO NOT MODIFY. YOUR CHANGES WILL BE OVER-WRITTEN
use crate::products::TwilioOrganizationsApiError;
use standard_error::traits::StandardErrorCausesTrait;

impl StandardErrorCausesTrait for TwilioOrganizationsApiError {
    fn causes(&self) -> Option<&'static str> {
        match self {
            TwilioOrganizationsApiError::ErrorCode25008 => Some(r#"Only managed users can be retrieved using this operation."#),
            TwilioOrganizationsApiError::ErrorCode25013 => Some(r#"The system does not allow updates to the user's username."#),
            TwilioOrganizationsApiError::ErrorCode25004 => Some(r#"The external ID is either less than 2 characters or exceeds 255 characters."#),
            TwilioOrganizationsApiError::ErrorCode25015 => Some(r#"The syntax of the `PATCH` request is incorrect."#),
            TwilioOrganizationsApiError::ErrorCode25203 => Some(r#"The role assignment listing call did not include any query parameters."#),
            TwilioOrganizationsApiError::ErrorCode25106 => Some(r#"- The resource to be modified has moved into a state that is no longer valid.
- Input on the request did not pass validation."#),
            TwilioOrganizationsApiError::ErrorCode25100 => Some(r#"No organization was found for the provided Organization Sid."#),
            TwilioOrganizationsApiError::ErrorCode25201 => Some(r#"The role assignments identity was set for an independent user."#),
            TwilioOrganizationsApiError::ErrorCode25001 => Some(r#"The filter value is either unsupported or missing."#),
            TwilioOrganizationsApiError::ErrorCode25018 => Some(r#"Incorrect credentials."#),
            TwilioOrganizationsApiError::ErrorCode25021 => Some(r#"Your application is requesting Organization API resources at a rate that is higher than allowed."#),
            TwilioOrganizationsApiError::ErrorCode25022 => Some(r#"A user with the specified username or externalId already exists in the system."#),
            TwilioOrganizationsApiError::ErrorCode25002 => Some(r#"The first name is either less than 2 characters or exceeds 255 characters."#),
            TwilioOrganizationsApiError::ErrorCode25016 => Some(r#"Update request is trying to modify organization owner."#),
            TwilioOrganizationsApiError::ErrorCode25007 => Some(r#"The maximum allowable number of users for the organization has been reached."#),
            TwilioOrganizationsApiError::ErrorCode25006 => Some(r#"The provided SCIM schema is not correctly formatted or does not adhere to the specified syntax."#),
            TwilioOrganizationsApiError::ErrorCode25101 => Some(r#"- Requested account doesn't exist
- Requested account is not a managed account"#),
            TwilioOrganizationsApiError::ErrorCode25023 => Some(r#"- The provided page token is invalid 
- The provided page token is expired"#),
            TwilioOrganizationsApiError::ErrorCode25009 => Some(r#"The user cannot be updated because they are currently in an unupdatable status."#),
            TwilioOrganizationsApiError::ErrorCode25005 => Some(r#"The username is either less than 2 characters or exceeds 255 characters."#),
            TwilioOrganizationsApiError::ErrorCode25017 => Some(r#"User creation is attempted with an email domain that is not verified by the organization."#),
            TwilioOrganizationsApiError::ErrorCode25107 => Some(r#"- When omitting the Auth header from HTTP request
- Passing an Auth header to access a resource that belongs to some other organization.
"#),
            TwilioOrganizationsApiError::ErrorCode25011 => Some(r#"The provided value for the email address is not valid."#),
            TwilioOrganizationsApiError::ErrorCode25109 => Some(r#"Your application is requesting resources at a rate that is higher than expected."#),
            TwilioOrganizationsApiError::ErrorCode25003 => Some(r#"The last name is either less than 2 characters or exceeds 255 characters."#),
            TwilioOrganizationsApiError::ErrorCode25200 => Some(r#"The role assignments scope was set to "organization" or an independent account."#),
            TwilioOrganizationsApiError::ErrorCode25103 => Some(r#"- An independent user has been set as the organization's account owner
- A non-managed user has been set as the organization's account owner
- User doesn't exist"#),
            TwilioOrganizationsApiError::ErrorCode25202 => Some(r#"The role assignment creation specifies an invalid role."#),
            TwilioOrganizationsApiError::ErrorCode25019 => Some(r#"The resource to be modified has moved into a state that is no longer valid.
Input on the request did not pass validation."#),
            TwilioOrganizationsApiError::ErrorCode25012 => Some(r#"The system does not allow updates to the user's email address."#),
            TwilioOrganizationsApiError::ErrorCode25014 => Some(r#"The user's primary email address does not match with the specified username."#),
            TwilioOrganizationsApiError::ErrorCode25104 => Some(r#"The organization's account creation request does not specify an owner, and the organization does not currently have an owner assigned."#),
            TwilioOrganizationsApiError::ErrorCode25102 => Some(r#"Your organization has reached the maximum limit for managed accounts."#),
            TwilioOrganizationsApiError::ErrorCode25020 => Some(r#"The SCIM version specified in the header does not match the version of the entity."#),
            TwilioOrganizationsApiError::ErrorCode25010 => Some(r#"- The request doesn't contain email addresses 
- The request contains more than one email address and doesn't specify "primary" attribute"#),
            TwilioOrganizationsApiError::ErrorCode25105 => Some(r#"Incorrect credentials."#)
        }
    }
}
