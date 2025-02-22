// AUTO-GENERATED. DO NOT MODIFY. YOUR CHANGES WILL BE OVER-WRITTEN
use crate::products::TwilioFlexError;
use standard_error::traits::StandardErrorSolutionsTrait;

impl StandardErrorSolutionsTrait for TwilioFlexError {
    fn solutions(&self) -> Option<&'static str> {
        match self {
            TwilioFlexError::ErrorCode45357 => Some(r#"Retry the request.

Check our <a href="https://status.twilio.com/">status page</a> to see if we are having an outage.

If our <a href="https://status.twilio.com/">status page</a> has no information, <a href="https://support.twilio.com/">contact support</a> with details about the requests that are failing, and we will investigate."#),
            TwilioFlexError::ErrorCode45784 => Some(r#"Retry the request.

Check our status page to see if we are having an outage.

If our status page has no information, contact support with details about the requests that are failing, and we will investigate."#),
            TwilioFlexError::ErrorCode45796 => Some(r#"Retry the request."#),
            TwilioFlexError::ErrorCode45770 => Some(r#""Retry the request.

Check our status page to see if we are having an outage.

If our status page has no information, contact support with details about the requests that are failing, and we will investigate.""#),
            TwilioFlexError::ErrorCode45782 => Some(r#"Retry the request.

Check our status page to see if we are having an outage.

If our status page has no information, contact support with details about the requests that are failing, and we will investigate."#),
            TwilioFlexError::ErrorCode49008 => Some(r#"Ensure that you have the correct Profile Connector Instance SID."#),
            TwilioFlexError::ErrorCode45351 => Some(r#"Specify either identity or address for Participants"#),
            TwilioFlexError::ErrorCode45376 => Some(r#"Ensure that the target Conversation is in the correct state."#),
            TwilioFlexError::ErrorCode90002 => Some(r#"* Ensure that you have put log messages appropriately in your plugin to reduce the number errors being published
* There could be errors outside of your plugins, within Twilio that are causing it to throw errors
* Use `try/catch` around methods and handle exceptions gracefully
* Use `.catch` for all HTTP requests to handle exceptions gracefully"#),
            TwilioFlexError::ErrorCode45308 => Some(r#"Retry operation"#),
            TwilioFlexError::ErrorCode45731 => Some(r#"Ensure pre-engagement data is less than 16KB. 

Read more about creating valid attributes [here.](https://www.twilio.com/docs/conversations/attributes)"#),
            TwilioFlexError::ErrorCode45601 => Some(r#"* Fix custom code so that the error is not thrown
* Remove code which throws and reports a custom Flex error event"#),
            TwilioFlexError::ErrorCode45356 => Some(r#"Retry the request.

Check our <a href="https://status.twilio.com/">status page</a> to see if we are having an outage.

If our <a href="https://status.twilio.com/">status page</a> has no information, <a href="https://support.twilio.com/">contact support</a> with details about the requests that are failing, and we will investigate."#),
            TwilioFlexError::ErrorCode45797 => Some(r#"This support access token will expire after 1hr."#),
            TwilioFlexError::ErrorCode45776 => Some(r#"Retry the request.

Check our status page to see if we are having an outage.

If our status page has no information, contact support with details about the requests that are failing, and we will investigate."#),
            TwilioFlexError::ErrorCode45309 => Some(r#"Retry operation."#),
            TwilioFlexError::ErrorCode45373 => Some(r#"Retry the request.

Check our <a href="https://status.twilio.com/">status page</a> to see if we are having an outage.

If our <a href="https://status.twilio.com/">status page</a> has no information, <a href="https://support.twilio.com/">contact support</a> with details about the requests that are failing, and we will investigate."#),
            TwilioFlexError::ErrorCode45101 => Some(r#"Ensure, that configuration for specific account is created, create one if not."#),
            TwilioFlexError::ErrorCode45377 => Some(r#"Ensure the AccountSid being used is the same as the one used to create the resource. "#),
            TwilioFlexError::ErrorCode45711 => Some(r#"Check that accessed resource is set correctly or apply for permissions."#),
            TwilioFlexError::ErrorCode45362 => Some(r#"No Action."#),
            TwilioFlexError::ErrorCode45374 => Some(r#"Retry the request.

Check our <a href="https://status.twilio.com/">status page</a> to see if we are having an outage.

If our <a href="https://status.twilio.com/">status page</a> has no information, <a href="https://support.twilio.com/">contact support</a> with details about the requests that are failing, and we will investigate."#),
            TwilioFlexError::ErrorCode45207 => Some(r#"Please check that Agents are not part of large number of inactive Channels/Tasks, review any custom programmability you may have in Flex UI (please review Flex Documentation). You can control the Chat Channels and Members through Programmable Chat API. You can also change the Channel Limit in Twilio Programmable Chat Console but keep in mind this cannot be extended indefinitely. "#),
            TwilioFlexError::ErrorCode45773 => Some(r#"Retry the request.

Check our status page to see if we are having an outage.

If our status page has no information, contact support with details about the requests that are failing, and we will investigate."#),
            TwilioFlexError::ErrorCode45700 => Some(r#"Retry the request.

Check our [status page](https://status.twilio.com/) to see if we are having an outage.

If our status page has no information, [contact support](https://www.twilio.com/help/contact) with details about the requests that are failing, and we will investigate."#),
            TwilioFlexError::ErrorCode45103 => Some(r#"Update the configuration: set the workspace."#),
            TwilioFlexError::ErrorCode45361 => Some(r#"Retry the request.

Check our <a href="https://status.twilio.com/">status page</a> to see if we are having an outage.

If our <a href="https://status.twilio.com/">status page</a> has no information, <a href="https://support.twilio.com/">contact support</a> with details about the requests that are failing, and we will investigate."#),
            TwilioFlexError::ErrorCode45719 => Some(r#"An API Request that receives an error 429 response is never processed and is always safe to retry. Please wait for a short period of time and make the request again, or alter your client's settings to issue fewer concurrent requests to the Twilio API."#),
            TwilioFlexError::ErrorCode45003 => Some(r#"Check that accessed resource is set correctly or apply for permissions."#),
            TwilioFlexError::ErrorCode45307 => Some(r#"Retry operation"#),
            TwilioFlexError::ErrorCode45355 => Some(r#"Close this Conversation and start a new one."#),
            TwilioFlexError::ErrorCode45371 => Some(r#"Retry the request.

Check our <a href="https://status.twilio.com/">status page</a> to see if we are having an outage.

If our <a href="https://status.twilio.com/">status page</a> has no information, <a href="https://support.twilio.com/">contact support</a> with details about the requests that are failing, and we will investigate."#),
            TwilioFlexError::ErrorCode45762 => Some(r#"Ensure that the address SID provided is valid and associated with the correct account.

Ensure the auto create type is present and set in the address configuration and is set to one of the valid types: webhook, studio, or default. More information on creating and updating an address configuration resource can be found [here.](https://www.twilio.com/docs/conversations/api/address-configuration-resource#addressconfiguration-properties)"#),
            TwilioFlexError::ErrorCode45203 => Some(r#"Chat User is created automatically by Flex for any inbound end customer requests and automatically when an Agent connects to Flex where that Agent already does not have a Chat User. Chat User is synchronized to Identity Provider provided information (via SSO) on role and name. Please try again and ensure that the relevant Chat Users are not removed in your own implementations. "#),
            TwilioFlexError::ErrorCode45744 => Some(r#"Ensure the account credentials and address SID provided in the request are correct and are all associated with the intended account. 

Ensure there's a default messaging service configured.
Read more about setting up a service [here.](https://www.twilio.com/docs/conversations/fundamentals#configuring-default-conversation-services-and-messaging-services)

Read more about the create conversation request [here.](https://www.twilio.com/docs/conversations/api/conversation-resource#create-a-conversation-resource)

Read more about messaging service resource [here.](https://www.twilio.com/docs/messaging/services/api)"#),
            TwilioFlexError::ErrorCode45109 => Some(r#"Delete some existing Skills or Contact Customer Support to increase your Skills Limit"#),
            TwilioFlexError::ErrorCode45010 => Some(r#"Verify that your application is not misbehaving and the load is expected. If everything is as expected then contact Customer Support to review your use case and limits."#),
            TwilioFlexError::ErrorCode45206 => Some(r#"No further action is necessary however please verify your business logic to avoid duplicate requests."#),
            TwilioFlexError::ErrorCode45701 => Some(r#"Retry the request.

Check our [status page](https://status.twilio.com/) to see if we are having an outage.

If our status page has no information, [contact support](https://www.twilio.com/help/contact) with details about the requests that are failing, and we will investigate."#),
            TwilioFlexError::ErrorCode45367 => Some(r#"Ensure that request parameters are in accordance with the public documentation.
"#),
            TwilioFlexError::ErrorCode45381 => Some(r#"Check our status page to see if we are having an outage.

If our status page has no information, contact support with details about the requests that are failing, and we will investigate."#),
            TwilioFlexError::ErrorCode45794 => Some(r#"Delete any of the existing deployment key to create more."#),
            TwilioFlexError::ErrorCode45208 => Some(r#"Please use the existing active Chat Channel, mark the Channel as inactive by setting Channel Attributes status to 'inactive', remove the Chat Channel or use a different unique name. "#),
            TwilioFlexError::ErrorCode45370 => Some(r#"Retry the request.

Check our <a href="https://status.twilio.com/">status page</a> to see if we are having an outage.

If our <a href="https://status.twilio.com/">status page</a> has no information, <a href="https://support.twilio.com/">contact support</a> with details about the requests that are failing, and we will investigate."#),
            TwilioFlexError::ErrorCode45358 => Some(r#"The only supported operation on a Channel is to update the status to "closed".  A status parameter must be provided."#),
            TwilioFlexError::ErrorCode45005 => Some(r#"Try the request again."#),
            TwilioFlexError::ErrorCode45360 => Some(r#"Retry the request.

Check our <a href="https://status.twilio.com/">status page</a> to see if we are having an outage.

If our <a href="https://status.twilio.com/">status page</a> has no information, <a href="https://support.twilio.com/">contact support</a> with details about the requests that are failing, and we will investigate."#),
            TwilioFlexError::ErrorCode45733 => Some(r#"Ensure pre-engagement data is in valid JSON format.

Read more about creating valid attributes [here.](https://www.twilio.com/docs/conversations/attributes)"#),
            TwilioFlexError::ErrorCode49009 => Some(r#"Please ensure the correct value inside of the SegmentUnifyApiKey."#),
            TwilioFlexError::ErrorCode45002 => Some(r#"Check the credentials and try again."#),
            TwilioFlexError::ErrorCode45760 => Some(r#"Ensure that the address SID provided is valid and associated with the correct account. 

Ensure that the auto create data associated with the address is present and in the correct format. More information on creating and updating an address configuration resource can be found [here.](https://www.twilio.com/docs/conversations/api/address-configuration-resource#addressconfiguration-properties)"#),
            TwilioFlexError::ErrorCode90003 => Some(r#"* Check if the custom plugins URL has been set correctly 
* The custom plugin URL must return a valid JSON object

See the [Deploying Flex Plugins](/docs/flex/developer/plugins/cli#deploying-to-your-own-cdn) docs for more information."#),
            TwilioFlexError::ErrorCode45302 => Some(r#"Please verify user connectivity and retry"#),
            TwilioFlexError::ErrorCode45375 => Some(r#"Ensure the AccountSid being used is the same as the one used to create the resource"#),
            TwilioFlexError::ErrorCode45301 => Some(r#"No further information available."#),
            TwilioFlexError::ErrorCode45369 => Some(r#"Retry the request.

Check our <a href="https://status.twilio.com/">status page</a> to see if we are having an outage.

If our <a href="https://status.twilio.com/">status page</a> has no information, <a href="https://support.twilio.com/">contact support</a> with details about the requests that are failing, and we will investigate."#),
            TwilioFlexError::ErrorCode45366 => Some(r#"Ensure that request parameters are in accordance with the public documentation."#),
            TwilioFlexError::ErrorCode45771 => Some(r#""Retry the request.

Check our status page to see if we are having an outage.

If our status page has no information, contact support with details about the requests that are failing, and we will investigate.""#),
            TwilioFlexError::ErrorCode45763 => Some(r#"Ensure that the address SID provided is valid and associated with the correct account. 

Ensure that the auto create type in address configuration is correct. If your auto create type is set to STUDIO, ensure the studio flow SID is present and correct in address configuration. 

Read more about creating and updating an address configuration resource [here.](https://www.twilio.com/docs/conversations/api/address-configuration-resource#create-an-addressconfiguration-resource)

Read more about creating a studio flow [here.](https://www.twilio.com/docs/usage/webhooks/webhooks-overview)"#),
            TwilioFlexError::ErrorCode45209 => Some(r#"Please refresh your browser and retry your Webchat experience."#),
            TwilioFlexError::ErrorCode45780 => Some(r#"Please wait for a short period of time and make the request again, or alter your client's settings to issue fewer concurrent requests to the Twilio API."#),
            TwilioFlexError::ErrorCode45359 => Some(r#"The only supported status change to a Channel is "closed"."#),
            TwilioFlexError::ErrorCode45009 => Some(r#"Revise and verify the configuration. If everything is as expected then contact Customer Support to review your use case."#),
            TwilioFlexError::ErrorCode45102 => Some(r#"Try to update the configuration again."#),
            TwilioFlexError::ErrorCode45600 => Some(r#"* Fix custom code
* Fix configuration"#),
            TwilioFlexError::ErrorCode45352 => Some(r#"Retry the request.

Check our <a href="https://status.twilio.com/">status page</a> to see if we are having an outage.

If our <a href="https://status.twilio.com/">status page</a> has no information, <a href="https://support.twilio.com/">contact support</a> with details about the requests that are failing, and we will investigate."#),
            TwilioFlexError::ErrorCode90004 => Some(r#"* Try to separate out your logs into individual messages"#),
            TwilioFlexError::ErrorCode45778 => Some(r#"Ensure the deployment key is correct and there exists a valid deployment configuration associated with it."#),
            TwilioFlexError::ErrorCode45745 => Some(r#"Ensure that request parameters are in accordance with the public documentation."#),
            TwilioFlexError::ErrorCode45006 => Some(r#"Check that accessed resource is set correctly."#),
            TwilioFlexError::ErrorCode45764 => Some(r#"Ensure that the address SID provided is valid and associated with the correct account.

Ensure that the auto create type provided is correct. If your auto create type is WEBHOOK, ensure the webhook url is present and valid in the address configuration. 

Read more about creating and updating an address configuration resource [here.](https://www.twilio.com/docs/conversations/api/address-configuration-resource#create-an-addressconfiguration-resource)

Read more about webhooks and their applications [here.](https://www.twilio.com/docs/usage/webhooks/webhooks-overview)"#),
            TwilioFlexError::ErrorCode45201 => Some(r#"Check that accessed resource is set correctly."#),
            TwilioFlexError::ErrorCode45772 => Some(r#"Provide all the mandatory headers."#),
            TwilioFlexError::ErrorCode45378 => Some(r#"Check our status page to see if we are having an outage.

If our status page has no information, contact support with details about the requests that are failing, and we will investigate."#),
            TwilioFlexError::ErrorCode45795 => Some(r#"Provide a friendly name that is between 1 and 64 characters in length."#),
            TwilioFlexError::ErrorCode45210 => Some(r#"Check the returned error details for additional info and adjust the request as necessary."#),
            TwilioFlexError::ErrorCode45402 => Some(r#"Please contact the WFM Vendor for troubleshooting"#),
            TwilioFlexError::ErrorCode45785 => Some(r#"Retry the request.

Check our status page to see if we are having an outage.

If our status page has no information, contact support with details about the requests that are failing, and we will investigate."#),
            TwilioFlexError::ErrorCode45004 => Some(r#"Review the input and fix (if possible) the problem mentioned in the error message."#),
            TwilioFlexError::ErrorCode45372 => Some(r#"Retry the request.

Check our <a href="https://status.twilio.com/">status page</a> to see if we are having an outage.

If our <a href="https://status.twilio.com/">status page</a> has no information, <a href="https://support.twilio.com/">contact support</a> with details about the requests that are failing, and we will investigate."#),
            TwilioFlexError::ErrorCode45211 => Some(r#"Change the identity field in the request and try again"#),
            TwilioFlexError::ErrorCode45353 => Some(r#"No Action."#),
            TwilioFlexError::ErrorCode45305 => Some(r#"Retry operation."#),
            TwilioFlexError::ErrorCode45777 => Some(r#"Ensure the deployment key is correct and there exists a valid address configuration Sid associated with it."#),
            TwilioFlexError::ErrorCode45775 => Some(r#"Provide the correct token."#),
            TwilioFlexError::ErrorCode93105 => Some(r#"Do not submit consent data more than once"#),
            TwilioFlexError::ErrorCode45741 => Some(r#"Check that credentials are correct.

Check that accessed resource is set correctly or apply for permissions."#),
            TwilioFlexError::ErrorCode45202 => Some(r#"Check that resource to be updated is set correctly."#),
            TwilioFlexError::ErrorCode45312 => Some(r#"Retry operation"#),
            TwilioFlexError::ErrorCode45007 => Some(r#"Please verify the current status of the resource to see if desired updates are already present in the system and that the request does not contain information that conflicts with the current state.
More context of the specific resource can be seen under the console message body."#),
            TwilioFlexError::ErrorCode45788 => Some(r#"Retry the request.

Check our status page to see if we are having an outage.

If our status page has no information, contact support with details about the requests that are failing, and we will investigate."#),
            TwilioFlexError::ErrorCode45313 => Some(r#"Retry operation"#),
            TwilioFlexError::ErrorCode45781 => Some(r#"Retry the request.

Check our status page to see if we are having an outage.

If our status page has no information, contact support with details about the requests that are failing, and we will investigate."#),
            TwilioFlexError::ErrorCode456001 => Some(r#"Archive older Plugins if they are no longer used."#),
            TwilioFlexError::ErrorCode456002 => Some(r#"Archive older Plugin Versions if they are no longer used."#),
            TwilioFlexError::ErrorCode45401 => Some(r#"Change account configuration and set correct value for rta_callback_url"#),
            TwilioFlexError::ErrorCode45761 => Some(r#""Ensure the address SID provided is valid and associated with the correct account.

Ensure the address configuration associated with the address is correct. Read more about creating and updating an address configuration resource [here.](https://www.twilio.com/docs/conversations/api/address-configuration-resource#fetch-an-addressconfiguration-resource)""#),
            TwilioFlexError::ErrorCode45765 => Some(r#""Ensure the provided address SID is correct and that the asscoiated address configuration has been created. 

Read more about creating and updating an address configuration resource [here.](https://www.twilio.com/docs/conversations/api/address-configuration-resource#create-an-addressconfiguration-resource)""#),
            TwilioFlexError::ErrorCode45789 => Some(r#"First unlink the deployment key from current address configuration Sid."#),
            TwilioFlexError::ErrorCode45303 => Some(r#"Disconnect media and retry."#),
            TwilioFlexError::ErrorCode45304 => Some(r#"Retry operation."#),
            TwilioFlexError::ErrorCode45205 => Some(r#"Please ensure you have correctly configured a Flex Flow related to your Messaging Channel (e.g. sms) and Integration Type (e.g. studio). You can use the Flex API or Flex Console to do this."#),
            TwilioFlexError::ErrorCode45363 => Some(r#"Retry the request.

Check our <a href="https://status.twilio.com/">status page</a> to see if we are having an outage.

If our <a href="https://status.twilio.com/">status page</a> has no information, <a href="https://support.twilio.com/">contact support</a> with details about the requests that are failing, and we will investigate."#),
            TwilioFlexError::ErrorCode45354 => Some(r#"No Action."#),
            TwilioFlexError::ErrorCode90000 => None,
            TwilioFlexError::ErrorCode45008 => Some(r#"Please verify the request details and ensure Flex setup and preconditions for this request have been met before trying again. "#),
            TwilioFlexError::ErrorCode45380 => Some(r#"Check our status page to see if we are having an outage.

If our status page has no information, contact support with details about the requests that are failing, and we will investigate."#),
            TwilioFlexError::ErrorCode45212 => Some(r#"Check the returned error details for additional info and adjust the request as necessary."#),
            TwilioFlexError::ErrorCode45368 => Some(r#"No Action."#),
            TwilioFlexError::ErrorCode45310 => Some(r#"No action required"#),
            TwilioFlexError::ErrorCode45001 => Some(r#"No further information available."#),
            TwilioFlexError::ErrorCode45050 => Some(r#"Ensure that the user authentication response sent to Flex includes required user attributes."#),
            TwilioFlexError::ErrorCode45715 => Some(r#"Ensure the account credentials and address SID provided are correct and are all associated with the intended account. 

Ensure that the request parameters are in accordance with the public documentation."#),
            TwilioFlexError::ErrorCode45306 => Some(r#"No action required."#),
            TwilioFlexError::ErrorCode45350 => Some(r#"Retry the request.

Check our <a href="https://status.twilio.com/">status page</a> to see if we are having an outage.

If our <a href="https://status.twilio.com/">status page</a> has no information, <a href="https://support.twilio.com/">contact support</a> with details about the requests that are failing, and we will investigate."#),
            TwilioFlexError::ErrorCode45774 => Some(r#"Retry the request.

Check our status page to see if we are having an outage.

If our status page has no information, contact support with details about the requests that are failing, and we will investigate."#),
            TwilioFlexError::ErrorCode45379 => Some(r#"Check our status page to see if we are having an outage.

If our status page has no information, contact support with details about the requests that are failing, and we will investigate."#),
            TwilioFlexError::ErrorCode45204 => Some(r#"Please check your request details. Flex Channel is created automatically on inbound messages and can be created by developer via Flex API for outbound use cases.

Check our [Flex Chat Channel Resource documentation](https://www.twilio.com/docs/flex/developer/messaging/chat-channel), or see how to [add a Custom Chat Channel to Flex](https://www.twilio.com/blog/add-custom-chat-channel-twilio-flex)."#)
        }
    }
}
