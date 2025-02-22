// AUTO-GENERATED. DO NOT MODIFY. YOUR CHANGES WILL BE OVER-WRITTEN
use crate::products::TwilioFrontlineError;
use standard_error::traits::StandardErrorSolutionsTrait;

impl StandardErrorSolutionsTrait for TwilioFrontlineError {
    fn solutions(&self) -> Option<&'static str> {
        match self {
            TwilioFrontlineError::ErrorCode48028 => Some(r#"* Make sure that the `proxy_address` value that is returned from the Outgoing Conversation callback belongs to the Twilio account under which the conversation is being created.
    * Check out the [docs](https://www.twilio.com/docs/frontline/nodejs-demo-quickstart#configure-the-twilio-frontline-integration-service) on how to configure your Twilio credentials in the integration service."#),
            TwilioFrontlineError::ErrorCode48005 => Some(r#"Make sure that callback processing time is less than 6 seconds."#),
            TwilioFrontlineError::ErrorCode48031 => Some(r#"* If the conversation has a Frontline user assigned to it, ask the user engaged in the conversation to either close the conversation or transfer it to the user who initiated the new conversation creation.
* If the conversation does not have any Frontline users assigned to it, either close the conversation or add the user who initiated the new conversation creation to the existing conversation.
    * If inbound conversations are unintentionally not being routed to a Frontline user, you may want to check your inbound routing configuration for issues. Check out the [docs](https://www.twilio.com/docs/frontline/handle-incoming-conversations) on inbound conversation routing."#),
            TwilioFrontlineError::ErrorCode48029 => Some(r#"* Make sure that the `proxy_address` value that is returned from the Outgoing Conversation callback has the same type as the channel over which the conversation is being created (i.e., for a WhatsApp conversation, use a Twilio proxy number with the prefix _whatsapp:_).
    * Check out the [docs](https://www.twilio.com/docs/frontline/outgoing-conversations) on how to create outgoing conversations."#),
            TwilioFrontlineError::ErrorCode48011 => Some(r#" - If the error persists, please <a href="/help/contact">contact us</a> to figure out what has happened and how to fix it!"#),
            TwilioFrontlineError::ErrorCode48027 => Some(r#"* Make sure that the `proxy_address` value that is returned from the Outgoing Conversation callback is different from the contact address.
    * Check out the [docs](https://www.twilio.com/docs/frontline/nodejs-demo-quickstart#sign-up-for-a-twilio-account-and-get-a-phone-number) on how to configure Twilio proxy numbers.
    * Check out the [docs](https://www.twilio.com/docs/frontline/nodejs-demo-quickstart#configure-the-twilio-frontline-integration-service) on how to use your Twilio proxy number within the integration service.
    * Check out the [docs](https://www.twilio.com/docs/frontline/outgoing-conversations) on how to create outgoing conversations."#),
            TwilioFrontlineError::ErrorCode48025 => Some(r#"* Make sure that the contact address returned from the CRM callback is correct and properly formatted in [E.164 format](https://www.twilio.com/docs/glossary/what-e164), including any channel prefixes if relevant.
    * Check out the [docs](https://www.twilio.com/docs/frontline/my-customers) on CRM callbacks for additional information."#),
            TwilioFrontlineError::ErrorCode48032 => Some(r#"* Configure a new "Default Messaging Service" from the console, under the [Conversations defaults](https://console.twilio.com/us1/develop/conversations/manage/defaults?frameUrl=%2Fconsole%2Fconversations%2Fconfiguration%2Fdefaults%3Fx-target-region%3Dus1).
    * Check out the [docs](https://www.twilio.com/docs/frontline/nodejs-demo-quickstart#configure-twilio-conversations) on how to configure Conversations to work with Frontline.
* Reach out to [customer support](http://twilio.com/console/support/tickets/create) for any additional help."#),
            TwilioFrontlineError::ErrorCode48030 => Some(r#"* Make sure that the `proxy_address` value that is returned from the Outgoing Conversation callback is a WhatsApp-enabled sender.
    * Check out the [docs](https://www.twilio.com/docs/whatsapp/api#using-twilio-phone-numbers-with-whatsapp) for how to configure WhatsApp-enabled senders as a Twilio proxy number.
    * Manage your WhatsApp-enabled senders from the [console](https://twilio.com/console/sms/whatsapp/senders)."#),
            TwilioFrontlineError::ErrorCode48050 => Some(r#"* Please reach out to [customer support](http://twilio.com/console/support/tickets/create)."#),
            TwilioFrontlineError::ErrorCode48001 => Some(r#"Set a Callback URL in Twilio Frontline Console to use this functionality."#),
            TwilioFrontlineError::ErrorCode48024 => Some(r#"* If you frequently exceed the user conversations limit for the Frontline Conversations service, you may want to consider raising your user conversations limit.
    * Read more about Conversations API user conversations limits in our [docs](https://www.twilio.com/docs/conversations/conversations-limits#maximum-channelsconversations-per-identity).
    * In order to request a limit increase, please contact [customer support](http://twilio.com/console/support/tickets/create).
* Close or delete conversations the user is a participant in."#),
            TwilioFrontlineError::ErrorCode48010 => Some(r#"- Check your callback execution logic to ensure it is processing the callback calls correctly and passing back 200 OK.
- Make sure that callback processing time is less than 10 seconds."#),
            TwilioFrontlineError::ErrorCode48004 => Some(r#"Make sure that Callback URL and endpoint is correct and you have a running web server at that URL."#),
            TwilioFrontlineError::ErrorCode48000 => Some(r#"- Update your Frontline application."#),
            TwilioFrontlineError::ErrorCode48002 => Some(r#"Make sure that you provided a valid Callback URL."#),
            TwilioFrontlineError::ErrorCode48026 => Some(r#"* Make sure that the `proxy_address` value returned from the Outgoing Conversation callback is present and properly formatted in [E.164 format](https://www.twilio.com/docs/glossary/what-e164), including any channel prefixes if relevant.
    * Check out the [docs](https://www.twilio.com/docs/frontline/outgoing-conversations) on how to create outgoing conversations."#),
            TwilioFrontlineError::ErrorCode48003 => Some(r#"* Make sure that callback returns a valid JSON which has the required fields that are defined in Frontline Docs.
* Make sure that your response size is less than 64k."#),
            TwilioFrontlineError::ErrorCode48023 => Some(r#"* If you frequently exceed the user conversations limit for the Frontline Conversations service, you may want to consider raising your user conversations limit.
    * Read more about Conversations API user conversations limits in our [docs](https://www.twilio.com/docs/conversations/conversations-limits#maximum-channelsconversations-per-identity).
    * In order to request a limit increase, please contact [customer support](http://twilio.com/console/support/tickets/create).
* Ask the Frontline user to close or transfer some of their active conversations.
* Delete conversations the user is a participant in."#),
            TwilioFrontlineError::ErrorCode48033 => Some(r#"* Make sure that the contact chat identity returned from the CRM callback is formatted properly and does not violate the following rules.
    * Should not match a User SID pattern `US[0-9a-f]{32}`.
    * Should not be a reserved identity (`system`, `private`, `unique`).
    * Should not be longer than 256 characters.
* We recommend following the standard URI specification and avoid the following reserved characters `! * ' ( ) ; : @ & = + $ , / ? % # [ ]` for identity."#)
        }
    }
}
