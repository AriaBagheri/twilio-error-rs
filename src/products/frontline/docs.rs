// AUTO-GENERATED. DO NOT MODIFY. YOUR CHANGES WILL BE OVER-WRITTEN
use crate::products::TwilioFrontlineError;
use standard_error::traits::StandardErrorDocsTrait;

impl StandardErrorDocsTrait for TwilioFrontlineError {
    fn docs(&self) -> &'static str {
        match self {
            TwilioFrontlineError::ErrorCode48028 => r#"## ERROR - 48028

### Outgoing conversation: Unauthorized use of the proxy address

 A conversation could not be created because your Twilio account is not authorized to use the provided Twilio proxy address.

#### Possible Causes
* The outgoing conversation callback returned a Twilio proxy number that was created under a different Twilio account.

#### Possible Solutions
* Make sure that the `proxy_address` value that is returned from the Outgoing Conversation callback belongs to the Twilio account under which the conversation is being created.
    * Check out the [docs](https://www.twilio.com/docs/frontline/nodejs-demo-quickstart#configure-the-twilio-frontline-integration-service) on how to configure your Twilio credentials in the integration service."#,
            TwilioFrontlineError::ErrorCode48005 => r#"## ERROR - 48005

### Callback failed due to timeout

 

#### Possible Causes
Your Frontline Integration Service side processing is exceeding 6 seconds.

#### Possible Solutions
Make sure that callback processing time is less than 6 seconds."#,
            TwilioFrontlineError::ErrorCode48031 => r#"## ERROR - 48031

### Outgoing conversation: Conversation with this contact and proxy address already exists

 A conversation could not be created because the requested contact address and Twilio proxy number are already in an active conversation in the same channel.

#### Possible Causes
* Another Frontline user is already engaged in the conversation with the same contact using the same Twilio proxy number.
* A conversation was created with the same contact using the same Twilio proxy number, but a Frontline user was never added to the conversation (i.e. the conversation was never routed to a Frontline user).

#### Possible Solutions
* If the conversation has a Frontline user assigned to it, ask the user engaged in the conversation to either close the conversation or transfer it to the user who initiated the new conversation creation.
* If the conversation does not have any Frontline users assigned to it, either close the conversation or add the user who initiated the new conversation creation to the existing conversation.
    * If inbound conversations are unintentionally not being routed to a Frontline user, you may want to check your inbound routing configuration for issues. Check out the [docs](https://www.twilio.com/docs/frontline/handle-incoming-conversations) on inbound conversation routing."#,
            TwilioFrontlineError::ErrorCode48029 => r#"## ERROR - 48029

### Outgoing conversation: Contact address type does not match proxy address type

 A conversation could not be created because the provided Twilio proxy address does not match the contact’s expected channel type.

#### Possible Causes
* The outgoing conversation callback returned a Twilio proxy number that does not match the channel type of the contact address.

#### Possible Solutions
* Make sure that the `proxy_address` value that is returned from the Outgoing Conversation callback has the same type as the channel over which the conversation is being created (i.e., for a WhatsApp conversation, use a Twilio proxy number with the prefix _whatsapp:_).
    * Check out the [docs](https://www.twilio.com/docs/frontline/outgoing-conversations) on how to create outgoing conversations."#,
            TwilioFrontlineError::ErrorCode48011 => r#"## ERROR - 48011

### Custom Routing Callback failed due to an internal error

An internal error has occurred that prevented Twilio from processing your callback. 

#### Possible Causes
- We failed to execute callback request.

#### Possible Solutions
 - If the error persists, please <a href="/help/contact">contact us</a> to figure out what has happened and how to fix it!"#,
            TwilioFrontlineError::ErrorCode48027 => r#"## ERROR - 48027

### Outgoing conversation: Proxy address equals contact address

 A conversation could not be created because the provided Twilio proxy address was the same as contact address.

#### Possible Causes
* The outgoing conversation callback returned a Twilio proxy number that was the same as the contact address.

#### Possible Solutions
* Make sure that the `proxy_address` value that is returned from the Outgoing Conversation callback is different from the contact address.
    * Check out the [docs](https://www.twilio.com/docs/frontline/nodejs-demo-quickstart#sign-up-for-a-twilio-account-and-get-a-phone-number) on how to configure Twilio proxy numbers.
    * Check out the [docs](https://www.twilio.com/docs/frontline/nodejs-demo-quickstart#configure-the-twilio-frontline-integration-service) on how to use your Twilio proxy number within the integration service.
    * Check out the [docs](https://www.twilio.com/docs/frontline/outgoing-conversations) on how to create outgoing conversations."#,
            TwilioFrontlineError::ErrorCode48025 => r#"## ERROR - 48025

### Outgoing conversation: Invalid contact address

 A conversation could not be created because the provided contact address was invalid.

#### Possible Causes
* The contact phone number used to initiate a new conversation was invalid.

#### Possible Solutions
* Make sure that the contact address returned from the CRM callback is correct and properly formatted in [E.164 format](https://www.twilio.com/docs/glossary/what-e164), including any channel prefixes if relevant.
    * Check out the [docs](https://www.twilio.com/docs/frontline/my-customers) on CRM callbacks for additional information."#,
            TwilioFrontlineError::ErrorCode48032 => r#"## ERROR - 48032

### Outgoing conversation: Missing Messaging service

 A conversation could not be created because the Messaging service required for Frontline is missing.

#### Possible Causes
* You may have deleted the Messaging service that auto-creates conversations in the Frontline Conversations service.

#### Possible Solutions
* Configure a new "Default Messaging Service" from the console, under the [Conversations defaults](https://console.twilio.com/us1/develop/conversations/manage/defaults?frameUrl=%2Fconsole%2Fconversations%2Fconfiguration%2Fdefaults%3Fx-target-region%3Dus1).
    * Check out the [docs](https://www.twilio.com/docs/frontline/nodejs-demo-quickstart#configure-twilio-conversations) on how to configure Conversations to work with Frontline.
* Reach out to [customer support](http://twilio.com/console/support/tickets/create) for any additional help."#,
            TwilioFrontlineError::ErrorCode48030 => r#"## ERROR - 48030

### Outgoing conversation: Proxy address is not WhatsApp-enabled sender

 A WhatsApp conversation could not be created because the provided Twilio proxy number is not a WhatsApp-enabled sender.

#### Possible Causes
* The Outgoing Conversation callback returned a Twilio proxy number that was properly formatted, but not configured as a WhatsApp-enabled sender.

#### Possible Solutions
* Make sure that the `proxy_address` value that is returned from the Outgoing Conversation callback is a WhatsApp-enabled sender.
    * Check out the [docs](https://www.twilio.com/docs/whatsapp/api#using-twilio-phone-numbers-with-whatsapp) for how to configure WhatsApp-enabled senders as a Twilio proxy number.
    * Manage your WhatsApp-enabled senders from the [console](https://twilio.com/console/sms/whatsapp/senders)."#,
            TwilioFrontlineError::ErrorCode48050 => r#"## ERROR - 48050

### Internal service error

 

#### Possible Causes
* Frontline encountered an internal error.

#### Possible Solutions
* Please reach out to [customer support](http://twilio.com/console/support/tickets/create)."#,
            TwilioFrontlineError::ErrorCode48001 => r#"## WARNING - 48001

### Callback URL is not set

 

#### Possible Causes
Callback URL is not set in Twilio Frontline Console.

#### Possible Solutions
Set a Callback URL in Twilio Frontline Console to use this functionality."#,
            TwilioFrontlineError::ErrorCode48024 => r#"## ERROR - 48024

### Contact conversation limit exceeded

 A Chat conversation could not be created because the Conversations user associated to the given contact is a participant in too many conversations.

#### Possible Causes
* The Conversations user associated to the given contact attempted to create a conversation over Chat channel and exceeded the Conversations service’s maximum user conversations limit.

#### Possible Solutions
* If you frequently exceed the user conversations limit for the Frontline Conversations service, you may want to consider raising your user conversations limit.
    * Read more about Conversations API user conversations limits in our [docs](https://www.twilio.com/docs/conversations/conversations-limits#maximum-channelsconversations-per-identity).
    * In order to request a limit increase, please contact [customer support](http://twilio.com/console/support/tickets/create).
* Close or delete conversations the user is a participant in."#,
            TwilioFrontlineError::ErrorCode48010 => r#"## ERROR - 48010

### Custom Routing Callback failed to execute successfully

 

#### Possible Causes
- The callback endpoint returned an error code.
- The callback processing is exceeding 10 seconds.

#### Possible Solutions
- Check your callback execution logic to ensure it is processing the callback calls correctly and passing back 200 OK.
- Make sure that callback processing time is less than 10 seconds."#,
            TwilioFrontlineError::ErrorCode48004 => r#"## ERROR - 48004

### Callback returned an error

 

#### Possible Causes
Your callback returned an error response, the Callback URL provided might be wrong or callback endpoint might be failing to process request.

#### Possible Solutions
Make sure that Callback URL and endpoint is correct and you have a running web server at that URL."#,
            TwilioFrontlineError::ErrorCode48000 => r#"## ERROR - 48000

### Invalid request payload

The callback request payload provided is invalid 

#### Possible Causes
- You might be using an old version of Frontline application.

#### Possible Solutions
- Update your Frontline application."#,
            TwilioFrontlineError::ErrorCode48002 => r#"## ERROR - 48002

### Callback URL is invalid

 

#### Possible Causes
The Callback URL provided is invalid.

#### Possible Solutions
Make sure that you provided a valid Callback URL."#,
            TwilioFrontlineError::ErrorCode48026 => r#"## ERROR - 48026

### Outgoing conversation: Invalid proxy address

 A conversation could not be created because the provided Twilio proxy address for the Frontline user was invalid.

#### Possible Causes
* The outgoing conversation callback returned an invalid Twilio proxy number or no number at all.

#### Possible Solutions
* Make sure that the `proxy_address` value returned from the Outgoing Conversation callback is present and properly formatted in [E.164 format](https://www.twilio.com/docs/glossary/what-e164), including any channel prefixes if relevant.
    * Check out the [docs](https://www.twilio.com/docs/frontline/outgoing-conversations) on how to create outgoing conversations."#,
            TwilioFrontlineError::ErrorCode48003 => r#"## ERROR - 48003

### Callback returned an invalid response

 

#### Possible Causes
* Your Frontline integration service returned a response which is invalid.
* Your service returned a response that was too large. Responses are capped at 64k.

#### Possible Solutions
* Make sure that callback returns a valid JSON which has the required fields that are defined in Frontline Docs.
* Make sure that your response size is less than 64k."#,
            TwilioFrontlineError::ErrorCode48023 => r#"## ERROR - 48023

### Frontline user conversation limit exceeded

 A conversation could not be created because the Conversations user associated to the given Frontline user is a participant in too many conversations.

#### Possible Causes
* The Conversations user associated to the given Frontline user attempted to create a conversation and exceeded the Conversations service’s maximum user conversations limit.

#### Possible Solutions
* If you frequently exceed the user conversations limit for the Frontline Conversations service, you may want to consider raising your user conversations limit.
    * Read more about Conversations API user conversations limits in our [docs](https://www.twilio.com/docs/conversations/conversations-limits#maximum-channelsconversations-per-identity).
    * In order to request a limit increase, please contact [customer support](http://twilio.com/console/support/tickets/create).
* Ask the Frontline user to close or transfer some of their active conversations.
* Delete conversations the user is a participant in."#,
            TwilioFrontlineError::ErrorCode48033 => r#"## ERROR - 48033

### Outgoing conversation: Invalid contact identity

 A conversation could not be created because the provided contact chat identity was invalid.

#### Possible Causes
* The contact chat identity was not formatted properly.
* A reserved identity was used for contact chat identity.

#### Possible Solutions
* Make sure that the contact chat identity returned from the CRM callback is formatted properly and does not violate the following rules.
    * Should not match a User SID pattern `US[0-9a-f]{32}`.
    * Should not be a reserved identity (`system`, `private`, `unique`).
    * Should not be longer than 256 characters.
* We recommend following the standard URI specification and avoid the following reserved characters `! * ' ( ) ; : @ & = + $ , / ? % # [ ]` for identity."#
        }
    }
}
