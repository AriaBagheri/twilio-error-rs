// AUTO-GENERATED. DO NOT MODIFY. YOUR CHANGES WILL BE OVER-WRITTEN
use crate::products::TwilioProgrammableVoiceError;
use standard_error::traits::StandardErrorSolutionsTrait;

impl StandardErrorSolutionsTrait for TwilioProgrammableVoiceError {
    fn solutions(&self) -> Option<&'static str> {
        match self {
            TwilioProgrammableVoiceError::ErrorCode11750 => Some(r#"*  Verify that you are serving TwiML in your response to Twilio's request. 
*  Verify that you are including this header in your TwiML response: `<?xml version="1.0" encoding="UTF-8"?>`  
*  Limit your TwiML response to 64 kB or less.
*  Does your TwiML include the [Play verb](https://www.twilio.com/docs/voice/twiml/play)? Double-check the encoding and MIME type to ensure they are supported.
*  Check that your TwiML is formatted properly. To quickly verify your TwiML, you can copy and paste it into a new [TwiML bin in the Twilio Console](https://www.twilio.com/console/runtime/twiml-bins/create).
*   Check to see if your application is throwing errors. This may cause your application to send a large debug output back to Twilio instead of the expected TwiML.
*  If you are trying to send a `200` response in a status callback, use an empty TwiML response: `<Response/>`"#),
            TwilioProgrammableVoiceError::ErrorCode16026 => Some(r#"Use a label value which uniquely identifies this participant within the conference."#),
            TwilioProgrammableVoiceError::ErrorCode13802 => Some(r#"* Add a valid `referUrl` attribute to your `<Dial>` verb
* For more information on sending REFER to Twilio, see [Refer to Twilio](https://www.twilio.com/docs/voice/api/refer-to-twilio)"#),
            TwilioProgrammableVoiceError::ErrorCode16002 => None,
            TwilioProgrammableVoiceError::ErrorCode16003 => None,
            TwilioProgrammableVoiceError::ErrorCode64106 => Some(r#"* Make sure the language and voice combinations are supported from the chosen provider."#),
            TwilioProgrammableVoiceError::ErrorCode64017 => Some(r#"* Have PaymentMethod as "ach-debit" when capturing ACH payments which require a BankAccountNumber
* Do not provide BankAccountNumber when collecting a credit card payment"#),
            TwilioProgrammableVoiceError::ErrorCode32702 => Some(r#"* Make sure to include content data in JSON format in the POST request to the UserDefinedMessage Resource."#),
            TwilioProgrammableVoiceError::ErrorCode32652 => Some(r#"* Review the Debugger message in console and update or remove the offending attributes"#),
            TwilioProgrammableVoiceError::ErrorCode64009 => Some(r#"* Create a new connector configuration and run through the scenario where you authorize Twilio to make requests on your behalf again."#),
            TwilioProgrammableVoiceError::ErrorCode64016 => Some(r#"* Start your action URL with "https://""#),
            TwilioProgrammableVoiceError::ErrorCode37000 => Some(r#"Ensure that call permission has been requested and accepted before initiating a call with the consumer."#),
            TwilioProgrammableVoiceError::ErrorCode13342 => Some(r#"Enter valid configuration by referring Google STT V2 provider"#),
            TwilioProgrammableVoiceError::ErrorCode21302 => Some(r#"Delete or update existing applications instead of creating new applications."#),
            TwilioProgrammableVoiceError::ErrorCode31429 => None,
            TwilioProgrammableVoiceError::ErrorCode13801 => Some(r#"* For more information on understanding what does and does not constitute a SIP call leg, please see [Understanding SIP Call Legs](https://www.twilio.com/docs/voice/twiml/refer#understanding-call-legs).
* While invoking Refer on a SIP leg is unique in that it removes Twilio from the call path, the effect of transferring a call to another party can be accomplished on all leg types by simply using the Dial verb to call the other party."#),
            TwilioProgrammableVoiceError::ErrorCode31904 => Some(r#"* Validate the IP address or DNS names are valid and belongs to the public domain
* Validate the host is running and properly connected to the Internet"#),
            TwilioProgrammableVoiceError::ErrorCode32653 => Some(r#"* If this is a continuous experience, please contact Twilio Support"#),
            TwilioProgrammableVoiceError::ErrorCode64002 => Some(r#"* Try to retry the request.
*   If the error persists, please <a href="/help/contact">contact us</a> and we can help you resolve the issue. 
*   Note the time of the error and what you were trying to do when it occurred."#),
            TwilioProgrammableVoiceError::ErrorCode31922 => Some(r#"* Verify the schema in TwiML URL is wss"#),
            TwilioProgrammableVoiceError::ErrorCode16020 => None,
            TwilioProgrammableVoiceError::ErrorCode31901 => Some(r#"* Verify server connectivity
* Verify the TwiML URL is correct
* Verify intermediate elements are routing traffic and firewall are not blocking ports"#),
            TwilioProgrammableVoiceError::ErrorCode31940 => Some(r#"* Make sure that the connectorName matches a Unique Name in an installed connector."#),
            TwilioProgrammableVoiceError::ErrorCode17000 => Some(r#"Once enabled future calls will be retrievable via API."#),
            TwilioProgrammableVoiceError::ErrorCode13328 => None,
            TwilioProgrammableVoiceError::ErrorCode13332 => None,
            TwilioProgrammableVoiceError::ErrorCode64011 => None,
            TwilioProgrammableVoiceError::ErrorCode64005 => Some(r#"* Make sure that the paymentConnector matches a &lt;Pay&gt; Connector that supports tokenization.
* If a charge was intended, please change the chargeAmount attribute and provided a positive value. Please see <A HREF="/docs/voice/twiml/pay#chargeamount">this API documentation</A> for more details."#),
            TwilioProgrammableVoiceError::ErrorCode13335 => Some(r#"speechTimeout value change"#),
            TwilioProgrammableVoiceError::ErrorCode31952 => Some(r#"Try using stream extension in another realm or checking that you are using a valid stream extension."#),
            TwilioProgrammableVoiceError::ErrorCode16099 => None,
            TwilioProgrammableVoiceError::ErrorCode32701 => Some(r#"* Make sure that Content-Type Header is included in the POST request to the UserDefinedMessage Resource and the value of the Content-Type Header is `application/json`."#),
            TwilioProgrammableVoiceError::ErrorCode13330 => None,
            TwilioProgrammableVoiceError::ErrorCode31409 => None,
            TwilioProgrammableVoiceError::ErrorCode16102 => None,
            TwilioProgrammableVoiceError::ErrorCode31009 => None,
            TwilioProgrammableVoiceError::ErrorCode16101 => None,
            TwilioProgrammableVoiceError::ErrorCode15004 => Some(r#"Provide an absolute URL for the action attribute when updating in-progress call with TwiML."#),
            TwilioProgrammableVoiceError::ErrorCode13804 => Some(r#"use AddOns in another realm"#),
            TwilioProgrammableVoiceError::ErrorCode16106 => None,
            TwilioProgrammableVoiceError::ErrorCode31103 => None,
            TwilioProgrammableVoiceError::ErrorCode32019 => Some(r#"Use only Voice URL or embedded twiml, not both, in the request"#),
            TwilioProgrammableVoiceError::ErrorCode21263 => Some(r#"Do not include AsyncAmd, AsyncAmdStatusCallback, or AsyncAmdStatusCallbackMethod parameters in POST requests made to /Participants API and in TwiML responses that include <Dial><Number> or <Dial><Sip> with Answering Machine Detection enabled."#),
            TwilioProgrammableVoiceError::ErrorCode31502 => None,
            TwilioProgrammableVoiceError::ErrorCode31206 => None,
            TwilioProgrammableVoiceError::ErrorCode16011 => None,
            TwilioProgrammableVoiceError::ErrorCode64023 => Some(r#"* Check the documentation for the pay connector in the Twilio Console or in our Twilio docs for valid bank account numbers that can be used when in TEST mode."#),
            TwilioProgrammableVoiceError::ErrorCode31408 => None,
            TwilioProgrammableVoiceError::ErrorCode64018 => Some(r#"* Ensure either Capture or Status parameters have a value but not both"#),
            TwilioProgrammableVoiceError::ErrorCode53404 => None,
            TwilioProgrammableVoiceError::ErrorCode16023 => Some(r#"Shorten the length of the ParticipantLabel parameter, avoid using a call SID and avoid including ‘/’ characters."#),
            TwilioProgrammableVoiceError::ErrorCode31209 => Some(r#"Please use same identity as the original call for the reconnection attempt."#),
            TwilioProgrammableVoiceError::ErrorCode13805 => Some(r#"Upgrade trial account"#),
            TwilioProgrammableVoiceError::ErrorCode16022 => None,
            TwilioProgrammableVoiceError::ErrorCode64010 => None,
            TwilioProgrammableVoiceError::ErrorCode16025 => Some(r#"Use a label value which uniquely identifies this participant within the conference"#),
            TwilioProgrammableVoiceError::ErrorCode13331 => None,
            TwilioProgrammableVoiceError::ErrorCode31402 => Some(r#"* Ensure the deviceID being specified exists.
* Try acquiring media with fewer constraints."#),
            TwilioProgrammableVoiceError::ErrorCode64019 => Some(r#"* Set Status parameter to "complete" only after all payment information is received via the Capture parameter"#),
            TwilioProgrammableVoiceError::ErrorCode17009 => Some(r#"The request should be retried using exponential backoff strategy until it succeeds."#),
            TwilioProgrammableVoiceError::ErrorCode31211 => Some(r#"Ensure the Call is at least in the Ringing state and the Subscription is successful and try again."#),
            TwilioProgrammableVoiceError::ErrorCode64006 => Some(r#"* Make sure that the paymentConnector matches a &lt;Pay&gt; Connector that supports the tokenization type you want.
* Make sure you specify the tokenType that matches what you want to perform."#),
            TwilioProgrammableVoiceError::ErrorCode32021 => Some(r#"No action required. Incoming call to your Twilio number can still be established without verstat."#),
            TwilioProgrammableVoiceError::ErrorCode31604 => None,
            TwilioProgrammableVoiceError::ErrorCode32500 => Some(r#"- See the Debugger in Console for further information. If this is a continuous experience, please contact Twilio Support."#),
            TwilioProgrammableVoiceError::ErrorCode32101 => Some(r#"Make sure that any phone number sent via SIP to Twilio is always E.164-formatted."#),
            TwilioProgrammableVoiceError::ErrorCode64014 => Some(r#"Make sure a "AVSName" Parameter element exists inside the <Pay> verb.

Example:
```
<Pay paymentMethod="ach-debit" ...>
  <Parameter name="AVSName" value="john smith" />
</Pay
```
"#),
            TwilioProgrammableVoiceError::ErrorCode17001 => Some(r#"To see results immediately, use the ProcessingState=partial query parameter, which will return a 200, but the response will contain only a partial or incomplete information until the summarization is completed. "#),
            TwilioProgrammableVoiceError::ErrorCode32009 => None,
            TwilioProgrammableVoiceError::ErrorCode64004 => None,
            TwilioProgrammableVoiceError::ErrorCode64022 => Some(r#"* Check the documentation for the pay connector in the Twilio Console or in our Twilio docs for valid test card numbers that can be used when in TEST mode."#),
            TwilioProgrammableVoiceError::ErrorCode31002 => None,
            TwilioProgrammableVoiceError::ErrorCode13247 => Some(r#"* Ensure your number is formatted in [E.164 format](https://www.twilio.com/docs/glossary/what-e164)
* Ensure that your `From` number is assigned and not on a do-not-originate (DNO) list."#),
            TwilioProgrammableVoiceError::ErrorCode13257 => Some(r#"Correct transcribe callback URL."#),
            TwilioProgrammableVoiceError::ErrorCode31202 => Some(r#"Ensure the Account SID, API Key, and API Key Secret are valid when generating your access token."#),
            TwilioProgrammableVoiceError::ErrorCode31212 => Some(r#"Reduce payload size of Call Message Event to be within the authorized limit and try again."#),
            TwilioProgrammableVoiceError::ErrorCode32503 => Some(r#"- Make sure you have a web server up and running that serves that URL with the configured method in TwiML, e.g. POST."#),
            TwilioProgrammableVoiceError::ErrorCode13329 => None,
            TwilioProgrammableVoiceError::ErrorCode64105 => Some(r#"* Make sure the websocket server at the url is healthy
* Investigate logs on the websocket server to see why the disconnect happened"#),
            TwilioProgrammableVoiceError::ErrorCode32601 => Some(r#"* Check your &lt;VirtualAgent&gt; connector configuration
* Check any &lt;Config&gt; elements referenced in TwiML
* Check the VirtualAgentProviderError included in the webhook to the action url or the Debugger message in Console for further information. For additional assistance, please contact Twilio Support."#),
            TwilioProgrammableVoiceError::ErrorCode64108 => Some(r#"* Contact Twilio support"#),
            TwilioProgrammableVoiceError::ErrorCode32606 => Some(r#"* Check the value of the &lt;Config name="resumeEndUserId"&gt; element referenced in TwiML
* Ensure the resume attempt is made from the same Call SID which created the conversation
* Check the VirtualAgentProviderError included in the webhook to the action url or the Debugger message in Console for further information. For additional assistance, please contact Twilio Support."#),
            TwilioProgrammableVoiceError::ErrorCode31920 => Some(r#"* Verify WebSocket protocol is enabled in the server
* Verify the path in TwiML URL is actually the one supporting WebSocket"#),
            TwilioProgrammableVoiceError::ErrorCode17007 => Some(r#"Enable Voice Insights Advanced Features for API access to future calls."#),
            TwilioProgrammableVoiceError::ErrorCode13333 => Some(r#"Enter valid profanity filter"#),
            TwilioProgrammableVoiceError::ErrorCode31530 => None,
            TwilioProgrammableVoiceError::ErrorCode13227 => Some(r#"Please check your [Voice Dialing Geographic Permissions](https://www.twilio.com/console/voice/calls/geo-permissions), fix it, and try again."#),
            TwilioProgrammableVoiceError::ErrorCode32007 => None,
            TwilioProgrammableVoiceError::ErrorCode31104 => None,
            TwilioProgrammableVoiceError::ErrorCode31208 => None,
            TwilioProgrammableVoiceError::ErrorCode31910 => Some(r#"* Verify the server has enabled SSL
* Verify the port in TwiML URL is actually the one supporting SSL
* Verify the certificate is correct and accepted by HTTP clients"#),
            TwilioProgrammableVoiceError::ErrorCode31953 => Some(r#"* Contact Twilio support"#),
            TwilioProgrammableVoiceError::ErrorCode16024 => Some(r#"Shorten the length of the ParticipantLabel parameter, avoid using a call SID and avoid including ‘/’ characters."#),
            TwilioProgrammableVoiceError::ErrorCode32650 => Some(r#"* Check the TranscriptionError message included in the webhook or the Debugger message in Console for further information and update your &lt;Transcription&gt; configuration"#),
            TwilioProgrammableVoiceError::ErrorCode17008 => Some(r#"The request should be retried using exponential backoff strategy until it succeeds."#),
            TwilioProgrammableVoiceError::ErrorCode32106 => None,
            TwilioProgrammableVoiceError::ErrorCode32014 => None,
            TwilioProgrammableVoiceError::ErrorCode32651 => Some(r#"* Check any provider specific configuration referenced in TwiML
* Check the TranscriptionError message included in the webhook or the Debugger message in Console for further information. For additional assistance, please contact Twilio Support."#),
            TwilioProgrammableVoiceError::ErrorCode13326 => None,
            TwilioProgrammableVoiceError::ErrorCode31426 => None,
            TwilioProgrammableVoiceError::ErrorCode13750 => None,
            TwilioProgrammableVoiceError::ErrorCode13240 => None,
            TwilioProgrammableVoiceError::ErrorCode16110 => Some(r#"* Please retry the delete operation with the same parameters.
* If the error persists, please <a href="/help/contact">contact us</a> to figure out what has happened and how to fix it!
* Note the time of the error and which filter was used to select the recordings."#),
            TwilioProgrammableVoiceError::ErrorCode13220 => Some(r#"Provide valid ringTone value"#),
            TwilioProgrammableVoiceError::ErrorCode21301 => Some(r#"Delete or update application in order to create new application."#),
            TwilioProgrammableVoiceError::ErrorCode31504 => None,
            TwilioProgrammableVoiceError::ErrorCode31930 => Some(r#"* Verify network load"#),
            TwilioProgrammableVoiceError::ErrorCode37001 => Some(r#"Ensure that a business-initiated WhatsApp call is placed in the supported Twilio Region."#),
            TwilioProgrammableVoiceError::ErrorCode32603 => Some(r#"* Ensure the correct &lt;VirtualAgent&gt; connector is being referenced
* Review the Debugger message in console and update or remove the offending attributes"#),
            TwilioProgrammableVoiceError::ErrorCode17002 => Some(r#"Do not request Voice Insights resources for calls that ended more than 30 days ago."#),
            TwilioProgrammableVoiceError::ErrorCode32008 => None,
            TwilioProgrammableVoiceError::ErrorCode32711 => Some(r#"* Make sure that the UserDefinedMessageSubscription callback URL is correct.
* Make sure that your endpoint at the UserDefinedMessageSubscription callback URL can successfully receive and respond to requests with 200-level HTTP status codes."#),
            TwilioProgrammableVoiceError::ErrorCode16010 => None,
            TwilioProgrammableVoiceError::ErrorCode16108 => None,
            TwilioProgrammableVoiceError::ErrorCode13521 => Some(r#"Make sure that the character limit is under the limit for the TTS type being used: 

* Basic TTS Voices are limited to 4096 UTF-8 single byte characters
* Polly TTS Voices are limited to 3000 UTF-8 single byte characters; note that SSML tags do not contribute to the character count
* Google TTS Voices are limited to 5000 UTF-8 single byte characters; note that SSML tags, newlines and spaces contribute to the character count
* The URL-encoded string cannot exceed 4500 characters

"#),
            TwilioProgrammableVoiceError::ErrorCode16113 => Some(r#"* Retry download without specifying a channel count"#),
            TwilioProgrammableVoiceError::ErrorCode15009 => Some(r#"The request should be retried using exponential backoff strategy until it succeeds."#),
            TwilioProgrammableVoiceError::ErrorCode10005 => Some(r#"Please reach out to Support for assistance. "#),
            TwilioProgrammableVoiceError::ErrorCode64104 => Some(r#"* None at this moment."#),
            TwilioProgrammableVoiceError::ErrorCode32220 => Some(r#"* Dial the global URI for your SIP domain, e.g. username@yourdomain.sip.twilio.com."#),
            TwilioProgrammableVoiceError::ErrorCode13340 => Some(r#"Please check Twilio's status page to see if engineers are currently working on resolving the issue. "#),
            TwilioProgrammableVoiceError::ErrorCode17400 => Some(r#"Utilize only two-character country code; e.g. US
Do not attempt wildcard searches; e.g. from=+1206*
Use E.164 address format for telephone numbers; e.g. +14258675309
Use SIP URI format; e.g. sip:user@domain.com
Use Client address format; e.g. client:username"#),
            TwilioProgrammableVoiceError::ErrorCode32215 => None,
            TwilioProgrammableVoiceError::ErrorCode13430 => Some(r#"Correct the DTMF string to use alpha-numeric, pound, or hash symbol."#),
            TwilioProgrammableVoiceError::ErrorCode22005 => Some(r#"* Validate application logic
* Contact your account manager or Twilio support to purchase additional capacity."#),
            TwilioProgrammableVoiceError::ErrorCode31486 => None,
            TwilioProgrammableVoiceError::ErrorCode14226 => Some(r#"Use TaskRouter Enqueue in supported realm"#),
            TwilioProgrammableVoiceError::ErrorCode32654 => Some(r#"* Ensure that the &lt;Transcription&gt; TwiML statusCallback attribute contains a valid URL format with scheme https"#),
            TwilioProgrammableVoiceError::ErrorCode31008 => Some(r#"N/A"#),
            TwilioProgrammableVoiceError::ErrorCode31000 => None,
            TwilioProgrammableVoiceError::ErrorCode31923 => Some(r#"* Verify the URL in TwiML is correct "#),
            TwilioProgrammableVoiceError::ErrorCode13327 => None,
            TwilioProgrammableVoiceError::ErrorCode16105 => None,
            TwilioProgrammableVoiceError::ErrorCode16028 => Some(r#"Validate that this call is connecting to the correct conference and the call SID specified is also in the conference."#),
            TwilioProgrammableVoiceError::ErrorCode13223 => Some(r#"Check the format of the phone number inside your `<Dial><Number>` TwiML. 

Verify that the number is in [E.164 format](https://www.twilio.com/docs/glossary/what-e164): 
- Includes the country code 
- Includes the area code
- Does not exceed 15 digits
- Does not include any invalid characters, like hyphens or parentheses

In your TwiML instructions, the phone number must be between the opening and closing `<Number>` tags, which are nested within `<Dial>`'s opening and closing tags:

```
<Dial><Number>+15555555555</Number></Dial>
```

If you are attempting to `<Dial>` an SDK end-user, ensure you're using the `<Client>` noun inside your `<Dial>` tags.

If you are attempting to `<Dial>` a SIP endpoint, ensure you're using the `<Sip>` noun inside your `<Dial>` tags.

For more information, read the [`<Dial>` docs](https://www.twilio.com/docs/voice/twiml/dial).

"#),
            TwilioProgrammableVoiceError::ErrorCode64107 => Some(r#"* Make sure all messages sent are conforming to the ConversationRelay websocket specification
* Make sure error handling is not sending back errors over the websocket to ConversationRelay
* Make sure API Gateways and Load Balancers are configured to not send errors to the ConversationRelay

Looking at the message can give an insight into why this might happen."#),
            TwilioProgrammableVoiceError::ErrorCode31207 => None,
            TwilioProgrammableVoiceError::ErrorCode13218 => Some(r#"sequential must be "true" or "false"."#),
            TwilioProgrammableVoiceError::ErrorCode64008 => None,
            TwilioProgrammableVoiceError::ErrorCode13238 => Some(r#"Remove Dial, Gather, Hangup and Record verbs from the TwiML."#),
            TwilioProgrammableVoiceError::ErrorCode64007 => Some(r#"* Make sure that the paymentConnector matches a &lt;Pay&gt; Connector that supports charging.
* If tokenization was intended, please change the chargeAmount attribute. Please see <A HREF="/docs/voice/twiml/pay#chargeamount">this API documentation</A> for more details."#),
            TwilioProgrammableVoiceError::ErrorCode13239 => None,
            TwilioProgrammableVoiceError::ErrorCode31600 => None,
            TwilioProgrammableVoiceError::ErrorCode14219 => Some(r#"Use TaskRouter Dial Queue in supported realm"#),
            TwilioProgrammableVoiceError::ErrorCode31480 => None,
            TwilioProgrammableVoiceError::ErrorCode13810 => Some(r#"Check application call timeout duration."#),
            TwilioProgrammableVoiceError::ErrorCode31487 => None,
            TwilioProgrammableVoiceError::ErrorCode32015 => Some(r#"Please reduce call duration or adjust account configurable timeLimits."#),
            TwilioProgrammableVoiceError::ErrorCode32212 => None,
            TwilioProgrammableVoiceError::ErrorCode31903 => Some(r#"* Verify server process is up and running
* Verify if there is any related connection error in the server logs
* Verify the server is connected and intermediate elements are routing traffic"#),
            TwilioProgrammableVoiceError::ErrorCode32505 => Some(r#"- Delete this Conversation using the Conversation API."#),
            TwilioProgrammableVoiceError::ErrorCode31500 => None,
            TwilioProgrammableVoiceError::ErrorCode64103 => Some(r#"* Retry
* Contact Twilio with identifying information about this call"#),
            TwilioProgrammableVoiceError::ErrorCode13511 => Some(r#"* Enter a valid voice as parameter"#),
            TwilioProgrammableVoiceError::ErrorCode32105 => None,
            TwilioProgrammableVoiceError::ErrorCode31400 => None,
            TwilioProgrammableVoiceError::ErrorCode31404 => None,
            TwilioProgrammableVoiceError::ErrorCode64015 => Some(r#"The required parameters for the connector are listed in the "Description" section of the connector.
The parameters should be passed inside the `<Pay>` verb.

Example:
```
<Pay>
  <Parameter name="Parameter Name" value="Parameter Value" />
</Pay
```

### Connector Specific Reasons for Error
#### Braintree
- `MerchantAccountId` is required to be passed via a `<Parameter>` noun for all Braintree `<Pay>` executions. This value can be found in the Braintree console by navigating to Settings (gear icon) -> Business.
- If passing a `description` as a parameter in `<Pay>`, you must also pass the following fields via separate `<Parameter>` nouns: `name`, `kind`, `quantity`, `totalAmount`, `unitAmount`. If you received this error you either left one of these fields out, or passed an invalid value for one of them. See [Braintree Documentation](https://developer.paypal.com/braintree/docs/reference/request/transaction/sale#line_items.description) for more information."#),
            TwilioProgrammableVoiceError::ErrorCode31902 => Some(r#"* Verify the TwiML URL is correct
* Verify server process is up and running"#),
            TwilioProgrammableVoiceError::ErrorCode32602 => Some(r#"* Check the connectorName attribute in your TwiML and ensure that it matches the unique name of a VirtualAgent connector"#),
            TwilioProgrammableVoiceError::ErrorCode13321 => Some(r#"* Enter a valid voice as parameter"#),
            TwilioProgrammableVoiceError::ErrorCode31102 => None,
            TwilioProgrammableVoiceError::ErrorCode31941 => Some(r#"* Make sure track value is among supported ones"#),
            TwilioProgrammableVoiceError::ErrorCode13338 => Some(r#"Correct action on Gather verb"#),
            TwilioProgrammableVoiceError::ErrorCode64001 => Some(r#"* Recreate your &lt;Pay&gt; connector configuration."#),
            TwilioProgrammableVoiceError::ErrorCode32401 => Some(r#"Look for errors with error code [32011](/docs/api/errors/32011), Error communicating with your SIP communications infrastructure, and follow instructions."#),
            TwilioProgrammableVoiceError::ErrorCode31205 => None,
            TwilioProgrammableVoiceError::ErrorCode31003 => None,
            TwilioProgrammableVoiceError::ErrorCode21234 => None,
            TwilioProgrammableVoiceError::ErrorCode31105 => Some(r#"Make sure that client name does not contain any of the invalid characters and length is at most 256 characters."#),
            TwilioProgrammableVoiceError::ErrorCode32504 => Some(r#"- Check that your routing is set up properly in your Frontline Console page and delete this Conversation using the Conversation API."#),
            TwilioProgrammableVoiceError::ErrorCode21262 => Some(r#"Provide an AMD status callback URL when enabling Answering Machine Detection"#),
            TwilioProgrammableVoiceError::ErrorCode32700 => Some(r#"* See the Debugger in the Twilio Console for more information. If you repeatedly receive this error, please contact Twilio Support."#),
            TwilioProgrammableVoiceError::ErrorCode32002 => None,
            TwilioProgrammableVoiceError::ErrorCode64110 => Some(r#"Reach out to Twilio to enable ConversationRelay for your account."#),
            TwilioProgrammableVoiceError::ErrorCode21300 => Some(r#"Make sure you submit the SID of a BYOC trunk that you can locate within your account."#),
            TwilioProgrammableVoiceError::ErrorCode31203 => None,
            TwilioProgrammableVoiceError::ErrorCode31950 => Some(r#"* Do not use binary format"#),
            TwilioProgrammableVoiceError::ErrorCode21215 => Some(r#"Please check your [Voice Dialing Geographic Permissions](https://www.twilio.com/console/voice/calls/geo-permissions), fix it, and try again."#),
            TwilioProgrammableVoiceError::ErrorCode13621 => None,
            TwilioProgrammableVoiceError::ErrorCode32506 => Some(r#"- Check the Frontline routing configurations.
- If a custom Routing was used, make sure the webhook is going to the right URL."#),
            TwilioProgrammableVoiceError::ErrorCode31603 => None,
            TwilioProgrammableVoiceError::ErrorCode21216 => Some(r#"Potential solutions may be:
- If your destination is being incorrectly identified as high-risk of fraud and you have a legitimate need to call this number, please reach out to Twilio Support through the [Console](https://console.twilio.com) or [Help Center](https://support.twilio.com/hc/en-us).
- If you are placing a call to a +1 destination and your account is missing a Primary Customer Profile, create one in [TrustHub](https://console.twilio.com/us1/account/trust-hub/customer-profiles). "#),
            TwilioProgrammableVoiceError::ErrorCode31302 => Some(r#"The application must register via Voice.register(...) on Android or [TwilioVoice registerWithAccessToken:deviceToken:completion:] on iOS to stop receiving cancel push notification messages."#),
            TwilioProgrammableVoiceError::ErrorCode64021 => Some(r#"* Make sure to use a valid operation"#),
            TwilioProgrammableVoiceError::ErrorCode13803 => Some(r#"Use SMS verb in another realm"#),
            TwilioProgrammableVoiceError::ErrorCode32710 => Some(r#"* See the Debugger in the Twilio Console for further information. If you repeatedly receive this error, please contact Twilio Support."#),
            TwilioProgrammableVoiceError::ErrorCode64013 => Some(r#"Make sure the Connector supports the defined paymentMethod attribute."#),
            TwilioProgrammableVoiceError::ErrorCode13619 => None,
            TwilioProgrammableVoiceError::ErrorCode31005 => None,
            TwilioProgrammableVoiceError::ErrorCode32604 => Some(r#"* If this is a continuous experience, please contact Twilio Support."#),
            TwilioProgrammableVoiceError::ErrorCode32013 => Some(r#"Increase the CPS on the Parent Account."#),
            TwilioProgrammableVoiceError::ErrorCode31951 => Some(r#"* Validate message is compliant with Stream protocols
* Validate message is a valid JSON"#),
            TwilioProgrammableVoiceError::ErrorCode32605 => Some(r#"* Ensure that the &lt;VirtualAgent&gt; TwiML statusCallback attribute contains a valid URL format with scheme https"#),
            TwilioProgrammableVoiceError::ErrorCode16104 => None,
            TwilioProgrammableVoiceError::ErrorCode31204 => None,
            TwilioProgrammableVoiceError::ErrorCode32022 => Some(r#"* Review configurations of your  SIP endpoint. Consult your documentation for details.
* Review logs on your SIP endpoint if possible. Consult your documentation for details on how to enable and access the logs.
* SIP devices generally re-transmit messages multiple times, in the event a single message is lost. Ensure your device is configured with the maximum retransmissions.
* Review proper SIP signaling by downloading PCAP via call log in Twilio console. And if possible, capture PCAP at your SIP endpoint to narrow down the cause of the signaling issue."#),
            TwilioProgrammableVoiceError::ErrorCode64012 => Some(r#"* Check the PaymentError in the response for more detailed information from the Payment Gateway.
* Prompt the user about the card information again."#),
            TwilioProgrammableVoiceError::ErrorCode31301 => None,
            TwilioProgrammableVoiceError::ErrorCode32210 => None,
            TwilioProgrammableVoiceError::ErrorCode31960 => Some(r#"*Increase the Dialogflow API quotas for the project used with Twilio Virtual Agent"#),
            TwilioProgrammableVoiceError::ErrorCode53401 => None,
            TwilioProgrammableVoiceError::ErrorCode64024 => Some(r#"Contact Twilio Support in order to gain approval for LIVE transactions for the specific connector instance."#),
            TwilioProgrammableVoiceError::ErrorCode31106 => None,
            TwilioProgrammableVoiceError::ErrorCode14300 => Some(r#"Correct the noun used in Twiml"#),
            TwilioProgrammableVoiceError::ErrorCode64020 => Some(r#"* Ensure that values match what is expected by refering to the API and TwiML docs"#),
            TwilioProgrammableVoiceError::ErrorCode64101 => Some(r#"* Validate that parameters provided are corrected."#),
            TwilioProgrammableVoiceError::ErrorCode32703 => Some(r#"* Make sure that the content body in the request does not exceed the maximum.length."#),
            TwilioProgrammableVoiceError::ErrorCode64102 => Some(r#"* Make sure the url parameter was entered correctly
* Make sure the websocket server is running at said url
* Make sure the websocket server is healthy and able to respond properly"#),
            TwilioProgrammableVoiceError::ErrorCode32214 => None,
            TwilioProgrammableVoiceError::ErrorCode32221 => Some(r#"* If you want to take advantage of new global SIP registration features, including the ability to dial any registered endpoint regardless of the edge through which it registered, update your TwiML or REST API app to use your SIP domain’s global URI, e.g username@yourdomain.sip.twilio.com.
* If SIP devices are registering through an edge you did not intend, either re-configure these devices to register through the desired edge, or turn off the SIP Registration functionality in your device settings if the registration behavior is not needed.
* To troubleshoot, you can verify if a SIP Endpoint has registered by using the Console "Registered SIP Endpoints" tab found on the SIP Domains page."#),
            TwilioProgrammableVoiceError::ErrorCode16109 => None,
            TwilioProgrammableVoiceError::ErrorCode31900 => Some(r#"Unknown"#),
            TwilioProgrammableVoiceError::ErrorCode32400 => Some(r#"Look for errors with error code [32011](/docs/api/errors/32011), Error communicating with your SIP communications infrastructure, and follow instructions."#),
            TwilioProgrammableVoiceError::ErrorCode16107 => None,
            TwilioProgrammableVoiceError::ErrorCode31403 => None,
            TwilioProgrammableVoiceError::ErrorCode32223 => Some(r#"* Add the correct username in the SIP URI you call, e.g., username@yourdomain.sip.twilio.com"#),
            TwilioProgrammableVoiceError::ErrorCode13513 => Some(r#"Change Say rate value."#),
            TwilioProgrammableVoiceError::ErrorCode32600 => Some(r#"* Update your &lt;VirtualAgent&gt; connector configuration"#),
            TwilioProgrammableVoiceError::ErrorCode31931 => Some(r#"* Wait until previous media has been processed before to send more media."#),
            TwilioProgrammableVoiceError::ErrorCode31924 => Some(r#"* Verify the messages sent by the server"#),
            TwilioProgrammableVoiceError::ErrorCode16021 => None,
            TwilioProgrammableVoiceError::ErrorCode31100 => None,
            TwilioProgrammableVoiceError::ErrorCode31201 => None,
            TwilioProgrammableVoiceError::ErrorCode16000 => None,
            TwilioProgrammableVoiceError::ErrorCode13219 => Some(r#"answerOnBridge must be "true" or "false"."#),
            TwilioProgrammableVoiceError::ErrorCode14207 => Some(r#"A queue’s maxSize can be adjusted to a maximum of 5000. Use the [Queue API](https://www.twilio.com/docs/voice/api/queue-resource) to identify the current maxSize and adjust accordingly.

If a waitUrl is provided for <Enqueue>, the parameters MaxQueueSize and CurrentQueueSize are included in the Webhook request to the waitUrl. Your application can use this information to determine queue capacity and adjust max queue size via the [API](https://www.twilio.com/docs/voice/api/queue-resource).

If the queue is already configured for the maximum allowed queue size (5000), one possible workaround is to create multiple queues and distribute the <Enqueue> requests between them."#),
            TwilioProgrammableVoiceError::ErrorCode31101 => None,
            TwilioProgrammableVoiceError::ErrorCode13512 => Some(r#"Find your preferred language in [this list of accepted values](https://www.twilio.com/docs/voice/twiml/gather#languagetags) and update your Gather TwiML accordingly."#),
            TwilioProgrammableVoiceError::ErrorCode31007 => None,
            TwilioProgrammableVoiceError::ErrorCode31503 => Some(r#"Please check the Application SID URL. For further assistance, please reach out to Twilio Support through the [Console](https://console.twilio.com) or [Help Center](https://support.twilio.com)."#),
            TwilioProgrammableVoiceError::ErrorCode13620 => Some(r#"Contact support to disable the corresponding feature(s) on your account."#),
            TwilioProgrammableVoiceError::ErrorCode31001 => None,
            TwilioProgrammableVoiceError::ErrorCode13337 => Some(r#"Gather URL must use HTTPS and valid URL format."#),
            TwilioProgrammableVoiceError::ErrorCode13258 => Some(r#"Use Dial->Sim in another realm"#),
            TwilioProgrammableVoiceError::ErrorCode32501 => Some(r#"- Check attribute names or the values following the <Conversation> noun.
- Refer to the <Conversation> twiml docs page “/docs/voice/twiml/connect/conversation” for more information."#),
            TwilioProgrammableVoiceError::ErrorCode13256 => Some(r#"Set recordingStatusCallback to a valid URL or leave blank to disable callbacks."#),
            TwilioProgrammableVoiceError::ErrorCode13214 => Some(r#"To work around this, your application should recognize invalid caller IDs, and substitute them with a valid Caller ID to construct the subsequent <Dial>. "#),
            TwilioProgrammableVoiceError::ErrorCode16111 => Some(r#"* Ensure AWS S3 bucket credentials are valid and available in Credential Storage
* Set a valid S3 bucket URL
* Disable external storage
* Ensure External Storage settings are provided for each home region you want to use"#),
            TwilioProgrammableVoiceError::ErrorCode32655 => Some(r#"* If this is a continuous experience, please contact Twilio Support"#),
            TwilioProgrammableVoiceError::ErrorCode16001 => None,
            TwilioProgrammableVoiceError::ErrorCode13622 => Some(r#"Check valid combinations of recordingTrack with requested channels"#),
            TwilioProgrammableVoiceError::ErrorCode13334 => Some(r#"Enter a valid model"#),
            TwilioProgrammableVoiceError::ErrorCode31401 => Some(r#"* The user should accept the request next time prompted. If the browser saved the deny, the user should change that permission in their browser.
* The user should to verify that the browser has permission to access the microphone at this address.
* The user should ensure that the proper permissions have been granted in the mobile device OS."#),
            TwilioProgrammableVoiceError::ErrorCode13225 => Some(r#"Potential solutions may be:
- If your destination is being incorrectly identified as high-risk of fraud and you have a legitimate need to call this number, please reach out to Twilio Support through the [Console](https://console.twilio.com) or [Help Center](https://support.twilio.com/hc/en-us).
- If you are placing a call to a +1 destination and your account is missing a Primary Customer Profile, create one in [TrustHub](https://console.twilio.com/us1/account/trust-hub/customer-profiles). 
"#),
            TwilioProgrammableVoiceError::ErrorCode22001 => Some(r#"CPS limits may be too low"#),
            TwilioProgrammableVoiceError::ErrorCode31942 => Some(r#"* Review configuration"#),
            TwilioProgrammableVoiceError::ErrorCode31481 => None,
            TwilioProgrammableVoiceError::ErrorCode32502 => Some(r#"- If the error persists, please <a href="/help/contact">contact us</a> to figure out what has happened and how to fix it!"#),
            TwilioProgrammableVoiceError::ErrorCode31006 => None,
            TwilioProgrammableVoiceError::ErrorCode13699 => Some(r#"Correct the recording trim value."#),
            TwilioProgrammableVoiceError::ErrorCode31921 => Some(r#"* Verify error logs in the remote websocket server"#),
            TwilioProgrammableVoiceError::ErrorCode32018 => Some(r#"Reduce the size of the twiml embedded in the request"#),
            TwilioProgrammableVoiceError::ErrorCode64003 => Some(r#"* Make sure the charge amount has correct number of decimal values.
* Make sure the charge amount is a positive value."#),
            TwilioProgrammableVoiceError::ErrorCode64109 => Some(r#"If needed, reach out to Twilio to see if they can increase your concurrency limit."#),
            TwilioProgrammableVoiceError::ErrorCode16027 => Some(r#"Validate that this call is connecting to the correct conference and the call SID specified is also in the conference."#),
            TwilioProgrammableVoiceError::ErrorCode31484 => Some(r#"* Ensure the phone number dialed is formatted correctly."#),
            TwilioProgrammableVoiceError::ErrorCode31210 => Some(r#"* Verify that Message Type being sent with Call Message Event is a value supported by Twilio Programmable Voice
* Use one of the <a href="/docs/voice/sdks/call-message-events/">supported Message Types</a> to call a Twilio function."#),
            TwilioProgrammableVoiceError::ErrorCode16112 => Some(r#"* Ensure AWS S3 bucket credentials are valid and have enough permission to access
* If SSE-S3 is enabled, please consider switching to SSE-KMS
* Disable external storage"#),
            TwilioProgrammableVoiceError::ErrorCode13216 => Some(r#"See the <a href='/docs/api/twiml/dial#timeLimit'>Dial Verb</a> API Reference for more information."#),
            TwilioProgrammableVoiceError::ErrorCode14218 => Some(r#"Check network or connectivity on external queue"#)
        }
    }
}
