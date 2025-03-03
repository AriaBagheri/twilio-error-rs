// AUTO-GENERATED. DO NOT MODIFY. YOUR CHANGES WILL BE OVER-WRITTEN
use crate::products::TwilioFlexError;
use standard_error::traits::StandardErrorCausesTrait;

impl StandardErrorCausesTrait for TwilioFlexError {
    fn causes(&self) -> Option<&'static str> {
        match self {
            TwilioFlexError::ErrorCode45357 => Some(r#"An error occurred retrieving Participant data from conversations api."#),
            TwilioFlexError::ErrorCode45784 => Some(r#"We encountered an unexpected error when processing get all deployment keys request."#),
            TwilioFlexError::ErrorCode45796 => Some(r#"Conflict in the state of target resource possibly because of concurrent requests."#),
            TwilioFlexError::ErrorCode45770 => Some(r#"We encountered an unexpected error while processing createInteractionChannel Request, Downstream Error."#),
            TwilioFlexError::ErrorCode45782 => Some(r#"We encountered an unexpected error when deleting deployment key. "#),
            TwilioFlexError::ErrorCode49008 => Some(r#"The Profile Connector {Profile Connector Instance SID or Unique Name} was not found."#),
            TwilioFlexError::ErrorCode45351 => Some(r#"A create Conversation or add Participant request was made with a Participant having both or neither of identity and address specified."#),
            TwilioFlexError::ErrorCode45376 => Some(r#"We did not add a Participant to a Channel because the Channel has either been closed or deleted."#),
            TwilioFlexError::ErrorCode90002 => Some(r#"* Your Flex Application is sending too many errors to the Twilio Debugger
* Your Flex Plugin is misconfigured causing a lot of errors to be published"#),
            TwilioFlexError::ErrorCode45308 => Some(r#"Meeting is not in the right state for adding the participants"#),
            TwilioFlexError::ErrorCode45731 => Some(r#"Pre engagment data is over the maximum allowed size of 16KB. "#),
            TwilioFlexError::ErrorCode45601 => Some(r#"* Custom code has been written to throw and report a Flex error event"#),
            TwilioFlexError::ErrorCode45356 => Some(r#"An error occurred retrieving Conversation data from conversations api."#),
            TwilioFlexError::ErrorCode45797 => Some(r#"A member of Twilio Support obtained customer approval to generate a support access token in order to provide support to the customer. "#),
            TwilioFlexError::ErrorCode45776 => Some(r#"An unexpected error occurred while fetching configuration associated with the given deployment key."#),
            TwilioFlexError::ErrorCode45309 => Some(r#"Meeting is not setup successfully for the operation"#),
            TwilioFlexError::ErrorCode45373 => Some(r#"We failed to remove a Participant from a Channel due to an unexpected error condition."#),
            TwilioFlexError::ErrorCode45101 => Some(r#"Queried configuration is not yet created for specified account."#),
            TwilioFlexError::ErrorCode45377 => Some(r#"The request was not completed because the AccountSid provided does not match the credentials in our records. "#),
            TwilioFlexError::ErrorCode45711 => Some(r#"Request is attempting to access resource where access is not allowed for given credentials."#),
            TwilioFlexError::ErrorCode45362 => Some(r#"The Participant may have already been removed."#),
            TwilioFlexError::ErrorCode45374 => Some(r#"We failed to delete a Channel due to an unexpected error condition."#),
            TwilioFlexError::ErrorCode45207 => Some(r#"Flex Agents as part of standard Flex Orchestration are removed from Chat Channels once they are done working a specific Task related to a Channel. Possible cause to this error is that this orchestration is not working or Flex is unaware that the Custom Task Channel used has chat capabilities. "#),
            TwilioFlexError::ErrorCode45773 => Some(r#"We encountered an unexpected error when creating token."#),
            TwilioFlexError::ErrorCode45700 => Some(r#"We encountered an unexpected error while processing your create WebChat endpoint."#),
            TwilioFlexError::ErrorCode45103 => Some(r#"Workspace is not defined in the configuration."#),
            TwilioFlexError::ErrorCode45361 => Some(r#"Conversations API returned an error"#),
            TwilioFlexError::ErrorCode45719 => Some(r#"Your account is sending too many concurrent Rest API requests to Twilio servers."#),
            TwilioFlexError::ErrorCode45003 => Some(r#"Attempt to access resource where access is not allowed for given credentials."#),
            TwilioFlexError::ErrorCode45307 => Some(r#"Meeting is not in the right state for adding the participants"#),
            TwilioFlexError::ErrorCode45355 => Some(r#"No Participants were found in a Conversation that triggered a new Channel to be created. The Participant may have been removed after the Conversation was created."#),
            TwilioFlexError::ErrorCode45371 => Some(r#"We failed to update the status of a Channel (e.g., to close the Channel) due to an unexpected error condition."#),
            TwilioFlexError::ErrorCode45762 => Some(r#"Address configuration associated with the provided address SID has an invalid or missing auto create type.  

The address configuration could be incorrect or out of date. "#),
            TwilioFlexError::ErrorCode45203 => Some(r#"Chat User related to a Chat Channel could not be found. "#),
            TwilioFlexError::ErrorCode45744 => Some(r#"A resource provided in create webchat conversation request could not be found by conversations API. "#),
            TwilioFlexError::ErrorCode45109 => Some(r#"Application attempted adding skills beyond Maximum Limit"#),
            TwilioFlexError::ErrorCode45010 => Some(r#"Your application is requesting Flex API resources at a rate that is higher than expected."#),
            TwilioFlexError::ErrorCode45206 => Some(r#"The request could not be completed due to a conflict as the Chat User is already part of the desired Chat Channel. This is possibly related to duplicate requests to Twilio Flex."#),
            TwilioFlexError::ErrorCode45701 => Some(r#"We encountered an unexpected error while processing your request. Our service is currently unavailable. "#),
            TwilioFlexError::ErrorCode45367 => Some(r#"sInvalid or missing parameters supplied in the create participant request."#),
            TwilioFlexError::ErrorCode45381 => Some(r#"Conversations API returned an error"#),
            TwilioFlexError::ErrorCode45794 => Some(r#"Account already has maximum allowed number of deployment keys."#),
            TwilioFlexError::ErrorCode45208 => Some(r#"An Active Chat Channel already exists with this unique name - new Channel not created. "#),
            TwilioFlexError::ErrorCode45370 => Some(r#"We failed to create an inbound Channel due to an unexpected error condition."#),
            TwilioFlexError::ErrorCode45358 => Some(r#"Update Channel request was made without valid 'status' parameter."#),
            TwilioFlexError::ErrorCode45005 => Some(r#"Service or one of it dependent services does not respond, timeout, etc."#),
            TwilioFlexError::ErrorCode45360 => Some(r#"Conversations API returned an error"#),
            TwilioFlexError::ErrorCode45733 => Some(r#"Pre-engagement data is not in valid JSON format."#),
            TwilioFlexError::ErrorCode49009 => Some(r#"Credentials are present, but aren’t parse-able due to an invalid SID placed inside of the configuration."#),
            TwilioFlexError::ErrorCode45002 => Some(r#"Attempt to access resource with invalid credentials or without credentials."#),
            TwilioFlexError::ErrorCode45760 => Some(r#"Address configuration data associated with the provided address SID is missing auto create data. "#),
            TwilioFlexError::ErrorCode90003 => Some(r#"* An invalid content type or no content being returned when Twilio tries to retrieve your plugins using the custom plugins URL."#),
            TwilioFlexError::ErrorCode45302 => Some(r#"Bad connectivity, User error"#),
            TwilioFlexError::ErrorCode45375 => Some(r#"We failed to delete a Channel since the provided AccountSid did not match witth our records."#),
            TwilioFlexError::ErrorCode45301 => Some(r#"Flex encountered a service error when connecting to a user"#),
            TwilioFlexError::ErrorCode45369 => Some(r#"We failed to create an outbound Channel due to an unexpected error condition."#),
            TwilioFlexError::ErrorCode45366 => Some(r#"Invalid or missing parameters supplied in the create Channel request."#),
            TwilioFlexError::ErrorCode45771 => Some(r#"We encountered an unexpected error processing the request."#),
            TwilioFlexError::ErrorCode45763 => Some(r#"Address configuration associated with the provided address SID has auto create type set to STUDIO but no corresponding studio flow SID. "#),
            TwilioFlexError::ErrorCode45209 => Some(r#"Adding a user to the Chat Channel has timed out waiting for a response from Twilio Chat."#),
            TwilioFlexError::ErrorCode45780 => Some(r#"Your account is sending too many concurrent requests."#),
            TwilioFlexError::ErrorCode45359 => Some(r#"Update Channel request was made with invalid 'status'."#),
            TwilioFlexError::ErrorCode45009 => Some(r#"A Flex component was not configured correctly."#),
            TwilioFlexError::ErrorCode45102 => Some(r#"Configuration for the same account was changed by other request mostly at the same time."#),
            TwilioFlexError::ErrorCode45600 => Some(r#"* There may have been an issue initializing Flex UI (no network, wrong config)
* There may be an error happening in Flex UI code
* There may be an error happening in user's custom code/plugin"#),
            TwilioFlexError::ErrorCode45352 => Some(r#"Conversations API returned an error"#),
            TwilioFlexError::ErrorCode90004 => Some(r#"* You have logged a message field that exceeds a total of 5000 allowed bytes"#),
            TwilioFlexError::ErrorCode45778 => Some(r#"No deployment configuration associated with the given deployment key."#),
            TwilioFlexError::ErrorCode45745 => Some(r#"Invalid or missing parameters in the create conversation request."#),
            TwilioFlexError::ErrorCode45006 => Some(r#"Requested resource could not be found."#),
            TwilioFlexError::ErrorCode45764 => Some(r#"Address configuration associated with the provided address SID has auto create type set to WEBHOOK but no webhook URL was provided."#),
            TwilioFlexError::ErrorCode45201 => Some(r#"Requested resource could not be found."#),
            TwilioFlexError::ErrorCode45772 => Some(r#"Mandatory headers might be missing in the request."#),
            TwilioFlexError::ErrorCode45378 => Some(r#"We failed to inactive a Channel due to an unexpected error condition."#),
            TwilioFlexError::ErrorCode45795 => Some(r#"The friendly name must consists of at least 1 character and up to a maximum of 64 characters."#),
            TwilioFlexError::ErrorCode45210 => Some(r#"Required Flow attributes were missing from the request or collided with existing Flows."#),
            TwilioFlexError::ErrorCode45402 => Some(r#"Issues on WFM vendor side"#),
            TwilioFlexError::ErrorCode45785 => Some(r#"We encountered an unexpected error when deleting deployment keys."#),
            TwilioFlexError::ErrorCode45004 => Some(r#"Request body validation fails."#),
            TwilioFlexError::ErrorCode45372 => Some(r#"We failed to add a Participant to a Channel due to an unexpected error condition."#),
            TwilioFlexError::ErrorCode45211 => Some(r#"Will pop up when users try to create a new channel with a reserved identity."#),
            TwilioFlexError::ErrorCode45353 => Some(r#"Multiple concurrent inbound messages to a non-existent conversation triggered multiple create Channel requests for the same Conversation. The duplicate request was dropped."#),
            TwilioFlexError::ErrorCode45305 => Some(r#"Inactive user"#),
            TwilioFlexError::ErrorCode45777 => Some(r#"No address configuration is linked with the given deployment key."#),
            TwilioFlexError::ErrorCode45775 => Some(r#"Token provided in the request is invalid."#),
            TwilioFlexError::ErrorCode93105 => Some(r#"The consent submission happens more than once"#),
            TwilioFlexError::ErrorCode45741 => Some(r#"Request attempted to access resource where access is not allowed for given credentials."#),
            TwilioFlexError::ErrorCode45202 => Some(r#"The request could not be completed due to a conflict with the current state of the resource."#),
            TwilioFlexError::ErrorCode45312 => Some(r#"Voice API returned an error while trying to remove the participant"#),
            TwilioFlexError::ErrorCode45007 => Some(r#"The request could not be completed due to a conflict with the current state of the target resource."#),
            TwilioFlexError::ErrorCode45788 => Some(r#"We encountered an unexpected error when updating deployment key."#),
            TwilioFlexError::ErrorCode45313 => Some(r#"Transfer process took too long to complete or the voice API returned error"#),
            TwilioFlexError::ErrorCode45781 => Some(r#"We encountered an unexpected error when creating deployment key."#),
            TwilioFlexError::ErrorCode456001 => Some(r#"You are approaching 25 Flex Plugins on your account."#),
            TwilioFlexError::ErrorCode456002 => Some(r#"You are approaching 500 Flex Plugin Versions on your account."#),
            TwilioFlexError::ErrorCode45401 => Some(r#"RTA callback is misconfigured or down"#),
            TwilioFlexError::ErrorCode45761 => Some(r#"Address configuration data associated with the provided address SID does not have auto create enabled in address configuration. When disabled, conversations will not be auto-created on incoming messages.  "#),
            TwilioFlexError::ErrorCode45765 => Some(r#"No address configuration has been created for the provided address SID.

The address SID provided is incorrect."#),
            TwilioFlexError::ErrorCode45789 => Some(r#"You cannot link deployment key to multiple address configuration Sids."#),
            TwilioFlexError::ErrorCode45303 => Some(r#"User was already engaged on the media channel"#),
            TwilioFlexError::ErrorCode45304 => Some(r#"Flex encountered a timeout error"#),
            TwilioFlexError::ErrorCode45205 => Some(r#"The Flex Flow referenced in this request or related Messaging request cannot be found."#),
            TwilioFlexError::ErrorCode45363 => Some(r#"Conversations API returned an error"#),
            TwilioFlexError::ErrorCode45354 => Some(r#"An inbound message triggered a Channel to start, but the associated Conversation was closed or deleted before the request was processed."#),
            TwilioFlexError::ErrorCode90000 => None,
            TwilioFlexError::ErrorCode45008 => Some(r#"The server is not able to process the request due to something that is perceived to be a client error (e.g. malformed request syntax or invalid request message)."#),
            TwilioFlexError::ErrorCode45380 => Some(r#"We failed to active a Channel due to an unexpected error condition."#),
            TwilioFlexError::ErrorCode45212 => Some(r#"Some request attributes were either missing, invalid, or collided with existing Flex Flows."#),
            TwilioFlexError::ErrorCode45368 => Some(r#"The Channel was not created or has already been deleted."#),
            TwilioFlexError::ErrorCode45310 => Some(r#"Participant is not in the meeting anymore"#),
            TwilioFlexError::ErrorCode45001 => Some(r#"Flex encountered a general service error."#),
            TwilioFlexError::ErrorCode45050 => Some(r#"Missing or invalid required attributes, such as full_name, email or roles"#),
            TwilioFlexError::ErrorCode45715 => Some(r#"The address configuration associated with the address SID provided could have invalid parameters. This could be because the account credentials provided are not associated with the address SID provided. "#),
            TwilioFlexError::ErrorCode45306 => Some(r#"Operation was canceled"#),
            TwilioFlexError::ErrorCode45350 => Some(r#"We encountered an unexpected error processing a request. "#),
            TwilioFlexError::ErrorCode45774 => Some(r#"We encountered an unexpected error when refreshing token."#),
            TwilioFlexError::ErrorCode45379 => Some(r#"Conversations API returned an error"#),
            TwilioFlexError::ErrorCode45204 => Some(r#"Flex Channel could not be found. "#)
        }
    }
}
