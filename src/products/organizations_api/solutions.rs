// AUTO-GENERATED. DO NOT MODIFY. YOUR CHANGES WILL BE OVER-WRITTEN
use crate::products::TwilioOrganizationsApiError;
use standard_error::traits::StandardErrorSolutionsTrait;

impl StandardErrorSolutionsTrait for TwilioOrganizationsApiError {
    fn solutions(&self) -> Option<&'static str> {
        match self {
            TwilioOrganizationsApiError::ErrorCode25008 => Some(r#"Double-check the user's ID for accuracy. Ensure that the user is a managed user, and if the issue persists, consider that the user's data may have been deleted."#),
            TwilioOrganizationsApiError::ErrorCode25013 => Some(r#"Username updates are not permitted. If changes to the username are necessary, reach out to support for guidance on handling specific scenarios."#),
            TwilioOrganizationsApiError::ErrorCode25004 => Some(r#"Ensure the user's external ID is between 2 and 255 characters in length."#),
            TwilioOrganizationsApiError::ErrorCode25015 => Some(r#"Verify that the `PATCH` request adheres to the specified format guidelines. Ensure that the request structure aligns with the SCIM standards for PATCH requests to resolve this issue."#),
            TwilioOrganizationsApiError::ErrorCode25203 => Some(r#"When listing role assignments, the call must include at least one of the following query parameters: "identity," "scope," or both."#),
            TwilioOrganizationsApiError::ErrorCode25106 => Some(r#"- Retry the request after confirming the request is valid.
- Verify if the resource to be modified exists and is in a valid state."#),
            TwilioOrganizationsApiError::ErrorCode25100 => Some(r#"Verify the Organization Sid."#),
            TwilioOrganizationsApiError::ErrorCode25201 => Some(r#"Verify the role assignments identity is a managed user."#),
            TwilioOrganizationsApiError::ErrorCode25001 => Some(r#"Filtering is only supported using the user's externalId or username. Only "eq" operator is supported. Ensure that the provided filter complies with these supported parameters."#),
            TwilioOrganizationsApiError::ErrorCode25018 => Some(r#"Verify credentials."#),
            TwilioOrganizationsApiError::ErrorCode25021 => Some(r#"Verify that your application is not misbehaving and the load is expected. If everything is as expected then contact Customer Support to review your use case and limits."#),
            TwilioOrganizationsApiError::ErrorCode25022 => Some(r#"Verify that the intended user has a unique username and externalId. If necessary, adjust the username or externalId to ensure uniqueness and avoid duplication in the system."#),
            TwilioOrganizationsApiError::ErrorCode25002 => Some(r#"Ensure the user's first name is between 2 and 255 characters in length."#),
            TwilioOrganizationsApiError::ErrorCode25016 => Some(r#"If changes to the organization owner are necessary, contact support for guidance on handling specific scenarios."#),
            TwilioOrganizationsApiError::ErrorCode25007 => Some(r#"To increase the limit of managed users for your organization, please get in touch with Twilio support."#),
            TwilioOrganizationsApiError::ErrorCode25006 => Some(r#"Double-check the schema for accuracy and ensure it complies with the RFC 7643 guidelines for SCIM. Refer to the RFC 7643 documentation for detailed information on valid syntax."#),
            TwilioOrganizationsApiError::ErrorCode25101 => Some(r#"Verify the Organization Sid and the Account Sid. Ensure that the Organization Sid corresponds to your organization and the Account is a managed account."#),
            TwilioOrganizationsApiError::ErrorCode25023 => Some(r#"Make sure that you are using the page token from a recent list response."#),
            TwilioOrganizationsApiError::ErrorCode25009 => Some(r#" Verify the current status of the user. If the user is in an unupdatable status, updates may be restricted."#),
            TwilioOrganizationsApiError::ErrorCode25005 => Some(r#"Ensure the user's username is between 2 and 255 characters in length."#),
            TwilioOrganizationsApiError::ErrorCode25017 => Some(r#"- Verify the domain associated with the user in twilio.com.
- User creation is only allowed with domains that have been verified."#),
            TwilioOrganizationsApiError::ErrorCode25107 => Some(r#"Verify correctness of Auth header."#),
            TwilioOrganizationsApiError::ErrorCode25011 => Some(r#"Double-check the email address value to ensure it adheres to the required format and standards."#),
            TwilioOrganizationsApiError::ErrorCode25109 => Some(r#"Verify that your application is not misbehaving and the load is expected. If everything is as expected then contact Customer Support to review your use case and limits."#),
            TwilioOrganizationsApiError::ErrorCode25003 => Some(r#"Ensure the user's last name is between 2 and 255 characters in length."#),
            TwilioOrganizationsApiError::ErrorCode25200 => Some(r#"Verify the role assignments scope is a managed account."#),
            TwilioOrganizationsApiError::ErrorCode25103 => Some(r#"Only managed users can be designated as an organization's account owner."#),
            TwilioOrganizationsApiError::ErrorCode25202 => Some(r#"Validate that the role provided is valid. Acceptable roles are:

- Admin: IX00000000000000000000000000000102
- Developer: IX00000000000000000000000000000103
- Billing: IX00000000000000000000000000000104
- Support: IX00000000000000000000000000000105"#),
            TwilioOrganizationsApiError::ErrorCode25019 => Some(r#"Verify if the resource to be modified exists and is in a valid state."#),
            TwilioOrganizationsApiError::ErrorCode25012 => Some(r#"Email address updates are not permitted. If changes to the email address are necessary, contact support for assistance with specific cases."#),
            TwilioOrganizationsApiError::ErrorCode25014 => Some(r#"Ensure that the primary email address matches the user's username."#),
            TwilioOrganizationsApiError::ErrorCode25104 => Some(r#"- Set the account's owner in the request
- To set an organization's owner, please contact Twilio support"#),
            TwilioOrganizationsApiError::ErrorCode25102 => Some(r#"To increase the account limit for your organization, please contact Twilio support."#),
            TwilioOrganizationsApiError::ErrorCode25020 => Some(r#"To resolve this conflict, either fetch the latest user entity to obtain the correct version or avoid setting the version in the header."#),
            TwilioOrganizationsApiError::ErrorCode25010 => Some(r#"To resolve this issue, mark one of the provided emails as the primary email address. Additionally, ensure that at least one of the email addresses matches the user's username. This alignment is necessary to fulfill the primary email address requirement."#),
            TwilioOrganizationsApiError::ErrorCode25105 => Some(r#"Verify credentials."#)
        }
    }
}
