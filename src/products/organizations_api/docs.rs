// AUTO-GENERATED. DO NOT MODIFY. YOUR CHANGES WILL BE OVER-WRITTEN
use crate::products::TwilioOrganizationsApiError;
use standard_error::traits::StandardErrorDocsTrait;

impl StandardErrorDocsTrait for TwilioOrganizationsApiError {
    fn docs(&self) -> &'static str {
        match self {
            TwilioOrganizationsApiError::ErrorCode25008 => r#"## ERROR - 25008

### The requested SCIM user was not found

 The requested SCIM user was not found.

#### Possible Causes
Only managed users can be retrieved using this operation.

#### Possible Solutions
Double-check the user's ID for accuracy. Ensure that the user is a managed user, and if the issue persists, consider that the user's data may have been deleted."#,
            TwilioOrganizationsApiError::ErrorCode25013 => r#"## ERROR - 25013

### Updating the username is unsupported

 Updating the username is unsupported.

#### Possible Causes
The system does not allow updates to the user's username.

#### Possible Solutions
Username updates are not permitted. If changes to the username are necessary, reach out to support for guidance on handling specific scenarios."#,
            TwilioOrganizationsApiError::ErrorCode25004 => r#"## ERROR - 25004

### The provided user's external ID is invalid

 The provided user's external ID is invalid.

#### Possible Causes
The external ID is either less than 2 characters or exceeds 255 characters.

#### Possible Solutions
Ensure the user's external ID is between 2 and 255 characters in length."#,
            TwilioOrganizationsApiError::ErrorCode25015 => r#"## ERROR - 25015

### The SCIM PATCH request is invalid

 The `SCIM PATCH` request is invalid.

#### Possible Causes
The syntax of the `PATCH` request is incorrect.

#### Possible Solutions
Verify that the `PATCH` request adheres to the specified format guidelines. Ensure that the request structure aligns with the SCIM standards for PATCH requests to resolve this issue."#,
            TwilioOrganizationsApiError::ErrorCode25203 => r#"## ERROR - 25203

### Listing role assignments requires query parameters

 Listing role assignments requires query parameters.

#### Possible Causes
The role assignment listing call did not include any query parameters.

#### Possible Solutions
When listing role assignments, the call must include at least one of the following query parameters: "identity," "scope," or both."#,
            TwilioOrganizationsApiError::ErrorCode25106 => r#"## ERROR - 25106

### Failed to complete request due to a bad request

 Failed to complete request due to a bad request

#### Possible Causes
- The resource to be modified has moved into a state that is no longer valid.
- Input on the request did not pass validation.

#### Possible Solutions
- Retry the request after confirming the request is valid.
- Verify if the resource to be modified exists and is in a valid state."#,
            TwilioOrganizationsApiError::ErrorCode25100 => r#"## ERROR - 25100

### The organization was not found

 The organization was not found.

#### Possible Causes
No organization was found for the provided Organization Sid.

#### Possible Solutions
Verify the Organization Sid."#,
            TwilioOrganizationsApiError::ErrorCode25201 => r#"## ERROR - 25201

### The identity for role assignments must be a managed user

 The identity for role assignments must be a managed user.

#### Possible Causes
The role assignments identity was set for an independent user.

#### Possible Solutions
Verify the role assignments identity is a managed user."#,
            TwilioOrganizationsApiError::ErrorCode25001 => r#"## ERROR - 25001

### The specified filter for listing organization users is invalid

 The specified filter for listing organization users is invalid.

#### Possible Causes
The filter value is either unsupported or missing.

#### Possible Solutions
Filtering is only supported using the user's externalId or username. Only "eq" operator is supported. Ensure that the provided filter complies with these supported parameters."#,
            TwilioOrganizationsApiError::ErrorCode25018 => r#"## ERROR - 25018

### The request is not authorized

 The request is not authorized.

#### Possible Causes
Incorrect credentials.

#### Possible Solutions
Verify credentials."#,
            TwilioOrganizationsApiError::ErrorCode25021 => r#"## ERROR - 25021

### Rate limit exceeded

 Rate limit exceeded.

#### Possible Causes
Your application is requesting Organization API resources at a rate that is higher than allowed.

#### Possible Solutions
Verify that your application is not misbehaving and the load is expected. If everything is as expected then contact Customer Support to review your use case and limits."#,
            TwilioOrganizationsApiError::ErrorCode25022 => r#"## ERROR - 25022

### Duplicate username or externalId

 Duplicate username or externalId

#### Possible Causes
A user with the specified username or externalId already exists in the system.

#### Possible Solutions
Verify that the intended user has a unique username and externalId. If necessary, adjust the username or externalId to ensure uniqueness and avoid duplication in the system."#,
            TwilioOrganizationsApiError::ErrorCode25002 => r#"## ERROR - 25002

### The provided user's first name is invalid

 The provided user's first name is invalid.

#### Possible Causes
The first name is either less than 2 characters or exceeds 255 characters.

#### Possible Solutions
Ensure the user's first name is between 2 and 255 characters in length."#,
            TwilioOrganizationsApiError::ErrorCode25016 => r#"## ERROR - 25016

### Updating the organization owner is not allowed

 Updating the organization owner is not allowed.

#### Possible Causes
Update request is trying to modify organization owner.

#### Possible Solutions
If changes to the organization owner are necessary, contact support for guidance on handling specific scenarios."#,
            TwilioOrganizationsApiError::ErrorCode25007 => r#"## ERROR - 25007

### The organization has reached its limit for managed users

 The organization has reached its limit for managed users.

#### Possible Causes
The maximum allowable number of users for the organization has been reached.

#### Possible Solutions
To increase the limit of managed users for your organization, please get in touch with Twilio support."#,
            TwilioOrganizationsApiError::ErrorCode25006 => r#"## ERROR - 25006

### The SCIM schema syntax is invalid

 The SCIM schema syntax is invalid.

#### Possible Causes
The provided SCIM schema is not correctly formatted or does not adhere to the specified syntax.

#### Possible Solutions
Double-check the schema for accuracy and ensure it complies with the RFC 7643 guidelines for SCIM. Refer to the RFC 7643 documentation for detailed information on valid syntax."#,
            TwilioOrganizationsApiError::ErrorCode25101 => r#"## ERROR - 25101

### The organization's account was not found

 The organization's account was not found.

#### Possible Causes
- Requested account doesn't exist
- Requested account is not a managed account

#### Possible Solutions
Verify the Organization Sid and the Account Sid. Ensure that the Organization Sid corresponds to your organization and the Account is a managed account."#,
            TwilioOrganizationsApiError::ErrorCode25023 => r#"## ERROR - 25023

### Invalid page token

 Invalid page token

#### Possible Causes
- The provided page token is invalid 
- The provided page token is expired

#### Possible Solutions
Make sure that you are using the page token from a recent list response."#,
            TwilioOrganizationsApiError::ErrorCode25009 => r#"## ERROR - 25009

### The user's is in an unupdatable status.

 The user's is in an unupdatable status.

#### Possible Causes
The user cannot be updated because they are currently in an unupdatable status.

#### Possible Solutions
 Verify the current status of the user. If the user is in an unupdatable status, updates may be restricted."#,
            TwilioOrganizationsApiError::ErrorCode25005 => r#"## ERROR - 25005

### The provided user's username is invalid

 The provided user's username is invalid.

#### Possible Causes
The username is either less than 2 characters or exceeds 255 characters.

#### Possible Solutions
Ensure the user's username is between 2 and 255 characters in length."#,
            TwilioOrganizationsApiError::ErrorCode25017 => r#"## ERROR - 25017

### The email domain is unverified

 The email domain is unverified.

#### Possible Causes
User creation is attempted with an email domain that is not verified by the organization.

#### Possible Solutions
- Verify the domain associated with the user in twilio.com.
- User creation is only allowed with domains that have been verified."#,
            TwilioOrganizationsApiError::ErrorCode25107 => r#"## ERROR - 25107

### The Request does not contain any authorization information

 The Request does not contain any authorization information (most cases: credentials) or the credentials and the resource have no relation.

#### Possible Causes
- When omitting the Auth header from HTTP request
- Passing an Auth header to access a resource that belongs to some other organization.


#### Possible Solutions
Verify correctness of Auth header."#,
            TwilioOrganizationsApiError::ErrorCode25011 => r#"## ERROR - 25011

### The value for the primary email address is invalid

 The value for the primary email address is invalid.

#### Possible Causes
The provided value for the email address is not valid.

#### Possible Solutions
Double-check the email address value to ensure it adheres to the required format and standards."#,
            TwilioOrganizationsApiError::ErrorCode25109 => r#"## ERROR - 25109

### Request is rate limited

 Request is rate limited.

#### Possible Causes
Your application is requesting resources at a rate that is higher than expected.

#### Possible Solutions
Verify that your application is not misbehaving and the load is expected. If everything is as expected then contact Customer Support to review your use case and limits."#,
            TwilioOrganizationsApiError::ErrorCode25003 => r#"## ERROR - 25003

### The provided user's last name is invalid

 The provided user's last name is invalid.

#### Possible Causes
The last name is either less than 2 characters or exceeds 255 characters.

#### Possible Solutions
Ensure the user's last name is between 2 and 255 characters in length."#,
            TwilioOrganizationsApiError::ErrorCode25200 => r#"## ERROR - 25200

### The scope for role assignments must be a managed account

 The scope for role assignments must be a managed account.

#### Possible Causes
The role assignments scope was set to "organization" or an independent account.

#### Possible Solutions
Verify the role assignments scope is a managed account."#,
            TwilioOrganizationsApiError::ErrorCode25103 => r#"## ERROR - 25103

### The organization's account owner is not a managed user

 The organization's account owner is not a managed user.

#### Possible Causes
- An independent user has been set as the organization's account owner
- A non-managed user has been set as the organization's account owner
- User doesn't exist

#### Possible Solutions
Only managed users can be designated as an organization's account owner."#,
            TwilioOrganizationsApiError::ErrorCode25202 => r#"## ERROR - 25202

### The role assignment has an invalid role

 The role assignment has an invalid role.

#### Possible Causes
The role assignment creation specifies an invalid role.

#### Possible Solutions
Validate that the role provided is valid. Acceptable roles are:

- Admin: IX00000000000000000000000000000102
- Developer: IX00000000000000000000000000000103
- Billing: IX00000000000000000000000000000104
- Support: IX00000000000000000000000000000105"#,
            TwilioOrganizationsApiError::ErrorCode25019 => r#"## ERROR - 25019

### Failed to complete request due to a business rule violations

 Failed to complete request due to a business rule violations.

#### Possible Causes
The resource to be modified has moved into a state that is no longer valid.
Input on the request did not pass validation.

#### Possible Solutions
Verify if the resource to be modified exists and is in a valid state."#,
            TwilioOrganizationsApiError::ErrorCode25012 => r#"## ERROR - 25012

### Updating the email address is unsupported

 Updating the email address is unsupported.

#### Possible Causes
The system does not allow updates to the user's email address.

#### Possible Solutions
Email address updates are not permitted. If changes to the email address are necessary, contact support for assistance with specific cases."#,
            TwilioOrganizationsApiError::ErrorCode25014 => r#"## ERROR - 25014

### The primary email address does not match the username

 The primary email address does not match the username.

#### Possible Causes
The user's primary email address does not match with the specified username.

#### Possible Solutions
Ensure that the primary email address matches the user's username."#,
            TwilioOrganizationsApiError::ErrorCode25104 => r#"## ERROR - 25104

### The organization's account owner was not set during creation

 The organization's account owner was not set during creation.

#### Possible Causes
The organization's account creation request does not specify an owner, and the organization does not currently have an owner assigned.

#### Possible Solutions
- Set the account's owner in the request
- To set an organization's owner, please contact Twilio support"#,
            TwilioOrganizationsApiError::ErrorCode25102 => r#"## ERROR - 25102

### The organization's account limit has been reached

 The organization's account limit has been reached.

#### Possible Causes
Your organization has reached the maximum limit for managed accounts.

#### Possible Solutions
To increase the account limit for your organization, please contact Twilio support."#,
            TwilioOrganizationsApiError::ErrorCode25020 => r#"## ERROR - 25020

### Version conflict in SCIM header

 Version conflict in SCIM header.

#### Possible Causes
The SCIM version specified in the header does not match the version of the entity.

#### Possible Solutions
To resolve this conflict, either fetch the latest user entity to obtain the correct version or avoid setting the version in the header."#,
            TwilioOrganizationsApiError::ErrorCode25010 => r#"## ERROR - 25010

### The primary email address is missing for the user

 The primary email address is missing for the user.

#### Possible Causes
- The request doesn't contain email addresses 
- The request contains more than one email address and doesn't specify "primary" attribute

#### Possible Solutions
To resolve this issue, mark one of the provided emails as the primary email address. Additionally, ensure that at least one of the email addresses matches the user's username. This alignment is necessary to fulfill the primary email address requirement."#,
            TwilioOrganizationsApiError::ErrorCode25105 => r#"## ERROR - 25105

### The request is not authorized

 The request is not authorized.

#### Possible Causes
Incorrect credentials.

#### Possible Solutions
Verify credentials."#
        }
    }
}
