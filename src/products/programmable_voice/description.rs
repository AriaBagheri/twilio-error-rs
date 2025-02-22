// AUTO-GENERATED. DO NOT MODIFY. YOUR CHANGES WILL BE OVER-WRITTEN
use crate::products::TwilioProgrammableVoiceError;
use standard_error::traits::StandardErrorDescriptionTrait;

impl StandardErrorDescriptionTrait for TwilioProgrammableVoiceError {
    fn description(&self) -> Option<&'static str> {
        match self {
            TwilioProgrammableVoiceError::ErrorCode11750 => Some(r#"## Error - 11750

## TwiML response body too large

In your response to Twilio's request, the response body is larger than 64 kB."#),
            TwilioProgrammableVoiceError::ErrorCode16026 => Some(r#"The participant label specified in an API call to create a voice conference participant is already in use by another active participant in the same conference."#),
            TwilioProgrammableVoiceError::ErrorCode13802 => Some(r#"In order to send a SIP REFER message to an established `<Dial>` call, the `referUrl` attribute must be specified in the TwiML. In this case, a REFER was received on a `<Dial>` call without a `referUrl` attribute present."#),
            TwilioProgrammableVoiceError::ErrorCode16002 => None,
            TwilioProgrammableVoiceError::ErrorCode16003 => None,
            TwilioProgrammableVoiceError::ErrorCode64106 => Some(r#"During media setup an error was encountered due to invalid argument."#),
            TwilioProgrammableVoiceError::ErrorCode64017 => Some(r#"A 64017 Error is an indication there is an unexpected value for the BankAccountNumber parameter."#),
            TwilioProgrammableVoiceError::ErrorCode32702 => Some(r#"Invalid content in the POST request to the UserDefinedMessage Resource. Only data in JSON format is supported."#),
            TwilioProgrammableVoiceError::ErrorCode32652 => Some(r#"A 32652 Error is an indication that &lt;Transcription&gt; TwiML contains invalid or unsupported &lt;Config&gt; attributes. "#),
            TwilioProgrammableVoiceError::ErrorCode64009 => Some(r#"## Error - 64009

### Pay: Twilio is no longer authorized to initiate transactions on your behalf.

A 64009 Error is an indication that the credentials provided in connector configuration no longer allow Twilio
to perform requests on behalf of your account.
"#),
            TwilioProgrammableVoiceError::ErrorCode64016 => Some(r#"A 64016 Error is an indication there is something wrong with the value provided for the action parameter."#),
            TwilioProgrammableVoiceError::ErrorCode37000 => Some(r#"The business-initiated WhatsApp call failed because the consumer had not granted permission to initiate the call."#),
            TwilioProgrammableVoiceError::ErrorCode13342 => Some(r#"## Warning - 13334

### Gather: Invalid configuration for Google STT V2 provider"#),
            TwilioProgrammableVoiceError::ErrorCode21302 => Some(r#"Approaching application creation limit"#),
            TwilioProgrammableVoiceError::ErrorCode31429 => None,
            TwilioProgrammableVoiceError::ErrorCode13801 => Some(r#"The Refer verb is only allowed on SIP call legs, e.g. you can refer an incoming SIP call that originate from SIP infrastructure or an outbound call to a SIP destination. Calls to or from other destinations, such as a phone number or Twilio Client cannot be referred."#),
            TwilioProgrammableVoiceError::ErrorCode31904 => Some(r#"The WebSocket connection cannot be created because the server is unreachable."#),
            TwilioProgrammableVoiceError::ErrorCode32653 => Some(r#"A 32653 Error is an indication that there is an internal server error and some internal services failed"#),
            TwilioProgrammableVoiceError::ErrorCode64002 => Some(r#"## Error - 64002

### Pay: Service unavailable.

A 64002 Error is an indication that there is an internal server error and some internal services failed."#),
            TwilioProgrammableVoiceError::ErrorCode31922 => None,
            TwilioProgrammableVoiceError::ErrorCode16020 => None,
            TwilioProgrammableVoiceError::ErrorCode31901 => None,
            TwilioProgrammableVoiceError::ErrorCode31940 => Some(r#"Invalid connectorName attribute in TwiML.
"#),
            TwilioProgrammableVoiceError::ErrorCode17000 => Some(r#"This error code is returned when an account is trying to access a resource created by another account. Note: parent accounts can access subaccount resources, but subaccounts cannot access parent account resources, or other subaccounts under the same parent account. "#),
            TwilioProgrammableVoiceError::ErrorCode13328 => None,
            TwilioProgrammableVoiceError::ErrorCode13332 => None,
            TwilioProgrammableVoiceError::ErrorCode64011 => None,
            TwilioProgrammableVoiceError::ErrorCode64005 => Some(r#"## Error - 64005

### Pay: Connector does not support tokenization.

A 64005 Error is an indication that the request to tokenize credit card failed because the connector does not support tokenization."#),
            TwilioProgrammableVoiceError::ErrorCode13335 => Some(r#"Gather: speechTimeout auto cannot be used with model default"#),
            TwilioProgrammableVoiceError::ErrorCode31952 => Some(r#"Stream Extension not found"#),
            TwilioProgrammableVoiceError::ErrorCode16099 => None,
            TwilioProgrammableVoiceError::ErrorCode32701 => Some(r#"Invalid Content-Type in the POST request to the UserDefinedMessage Resource. Only `application/json` is supported."#),
            TwilioProgrammableVoiceError::ErrorCode13330 => None,
            TwilioProgrammableVoiceError::ErrorCode31409 => None,
            TwilioProgrammableVoiceError::ErrorCode16102 => None,
            TwilioProgrammableVoiceError::ErrorCode31009 => None,
            TwilioProgrammableVoiceError::ErrorCode16101 => None,
            TwilioProgrammableVoiceError::ErrorCode15004 => Some(r#"The TwiML provided for updating the in-progress call must use absolute URL for action callback."#),
            TwilioProgrammableVoiceError::ErrorCode13804 => Some(r#"AddOns are not supported in this realm"#),
            TwilioProgrammableVoiceError::ErrorCode16106 => None,
            TwilioProgrammableVoiceError::ErrorCode31103 => None,
            TwilioProgrammableVoiceError::ErrorCode32019 => Some(r#"Twiml and Voice URL are both set. Using Voice URL."#),
            TwilioProgrammableVoiceError::ErrorCode21263 => Some(r#"Invalid Answering Machine Detection Parameters"#),
            TwilioProgrammableVoiceError::ErrorCode31502 => None,
            TwilioProgrammableVoiceError::ErrorCode31206 => None,
            TwilioProgrammableVoiceError::ErrorCode16011 => None,
            TwilioProgrammableVoiceError::ErrorCode64023 => Some(r#"A 64023 error is an indication that your Pay Connector is in TEST mode, but you did not provide a valid test bank account number when testing an ACH payment."#),
            TwilioProgrammableVoiceError::ErrorCode31408 => None,
            TwilioProgrammableVoiceError::ErrorCode64018 => Some(r#"A 64018 Error is an indication that it is not clear whether to capture a piece of payment information or to complete/cancel the transaction."#),
            TwilioProgrammableVoiceError::ErrorCode53404 => None,
            TwilioProgrammableVoiceError::ErrorCode16023 => Some(r#"The participant label specified in an API call to create a voice conference participant was invalid. It may not exceed 128 characters, it may not be a CallSid, and it may not contain a ‘/’ (even in URL encoded form %2F)."#),
            TwilioProgrammableVoiceError::ErrorCode31209 => Some(r#"Reconnect attempt is not authorized."#),
            TwilioProgrammableVoiceError::ErrorCode13805 => Some(r#"Trial account call duration exceeded 10 minute limit"#),
            TwilioProgrammableVoiceError::ErrorCode16022 => None,
            TwilioProgrammableVoiceError::ErrorCode64010 => None,
            TwilioProgrammableVoiceError::ErrorCode16025 => Some(r#"The participant label specified as an attribute on the conference noun in twiml is already in use by another active participant in the same conference."#),
            TwilioProgrammableVoiceError::ErrorCode13331 => None,
            TwilioProgrammableVoiceError::ErrorCode31402 => Some(r#"The browser and end-user allowed permissions, however getting the media failed. Usually this is due to bad constraints, but can sometimes fail due to browser, OS or hardware issues."#),
            TwilioProgrammableVoiceError::ErrorCode64019 => Some(r#"A 64019 Error is an indication that the pieces of payment information indicated as required were not captured."#),
            TwilioProgrammableVoiceError::ErrorCode17009 => Some(r#"The requested resource failed to complete the request. "#),
            TwilioProgrammableVoiceError::ErrorCode31211 => Some(r#"Call is not in the expected state."#),
            TwilioProgrammableVoiceError::ErrorCode64006 => Some(r#"## Error - 64006

### Pay: Connector does not support token type.

 Error is an indication that the request to tokenize credit card failed because the connector does not support tokenization.

A 64006 Error is an indication that the request to tokenize of a particular type is not supported by the &lt;Pay&gt; connector.  For instance, tokenType="one-time" was used but
only tokenType="reusable" is supported."#),
            TwilioProgrammableVoiceError::ErrorCode32021 => Some(r#"Twilio cannot verify incoming SHAKEN PASSporT from carrier"#),
            TwilioProgrammableVoiceError::ErrorCode31604 => None,
            TwilioProgrammableVoiceError::ErrorCode32500 => Some(r#"Request through Voice Conversation Service failed."#),
            TwilioProgrammableVoiceError::ErrorCode32101 => Some(r#"SIP: Invalid phone number
"#),
            TwilioProgrammableVoiceError::ErrorCode64014 => Some(r#"A 64014 Error is an indication that the provided paymentMethod attribute "ach-debit" was used, but the AVSName Parameter was not found inside the <Pay> verb."#),
            TwilioProgrammableVoiceError::ErrorCode17001 => Some(r#"This error code is returned by the /Summary API when a completed summary is requested and it doesn't exist."#),
            TwilioProgrammableVoiceError::ErrorCode32009 => None,
            TwilioProgrammableVoiceError::ErrorCode64004 => None,
            TwilioProgrammableVoiceError::ErrorCode64022 => Some(r#"A 64022 error is an indication that your Pay Connector is in TEST mode, but you did not provide a valid test card number when testing a payment."#),
            TwilioProgrammableVoiceError::ErrorCode31002 => None,
            TwilioProgrammableVoiceError::ErrorCode13247 => Some(r#"You attempted to initiate an outbound phone call or message, but the 'From' parameter you supplied was not a valid phone number or alphanumeric sender ID.

Twilio accepts phone numbers in [E.164 format](https://www.twilio.com/docs/glossary/what-e164) (i.e. "+1 format"), 10-digit US and Canadian numbers with any combination of non-digit separators, or Alphanumeric Sender IDs (SMS only) with up to 11 alphanumeric characters [a-zA-Z0-9]. Refer to [this page](https://support.twilio.com/hc/en-us/articles/223133867-Using-Alphanumeric-Sender-ID-with-Messaging-Services) for acceptable characters.

The number must not be on a do-not-originate (DNO) list, and Alpha Sender IDs may not be generic."#),
            TwilioProgrammableVoiceError::ErrorCode13257 => Some(r#"Transcribe callback URL does not conform to standard URL format."#),
            TwilioProgrammableVoiceError::ErrorCode31202 => Some(r#"The provided access token failed signature validation."#),
            TwilioProgrammableVoiceError::ErrorCode31212 => Some(r#"Call Message Event Payload size exceeded authorized limit."#),
            TwilioProgrammableVoiceError::ErrorCode32503 => Some(r#"Request to status callback URL received a response error from the customer-side server."#),
            TwilioProgrammableVoiceError::ErrorCode13329 => None,
            TwilioProgrammableVoiceError::ErrorCode64105 => Some(r#"The websocket disconnected from the server side during an ongoing call"#),
            TwilioProgrammableVoiceError::ErrorCode32601 => Some(r#"A 32601 Error is an indication that the &lt;VirtualAgent&gt; connector configuration was rejected by the VirtualAgent provider."#),
            TwilioProgrammableVoiceError::ErrorCode64108 => Some(r#"An RTP timeout is detected, meaning that audio stopped flowing."#),
            TwilioProgrammableVoiceError::ErrorCode32606 => Some(r#"A 32606 Error is an indication that the &lt;VirtualAgent&gt; TwiML contains an invalid or unauthorized configuration for resuming an existing conversation."#),
            TwilioProgrammableVoiceError::ErrorCode31920 => Some(r#"The server has returned an HTTP code different from 101 to the connection request sent by stream

See https://tools.ietf.org/html/rfc6455#section-4 for details."#),
            TwilioProgrammableVoiceError::ErrorCode17007 => Some(r#"The requested resource's call_sid is valid, but it ended before the Voice Insights product was last enabled for the requester's account."#),
            TwilioProgrammableVoiceError::ErrorCode13333 => Some(r#"## Warning - 13333

### Gather: Invalid profanityFilter value"#),
            TwilioProgrammableVoiceError::ErrorCode31530 => None,
            TwilioProgrammableVoiceError::ErrorCode13227 => Some(r#"You attempted to initiate an outbound phone call to a phone number that is not enabled on your account."#),
            TwilioProgrammableVoiceError::ErrorCode32007 => None,
            TwilioProgrammableVoiceError::ErrorCode31104 => None,
            TwilioProgrammableVoiceError::ErrorCode31208 => None,
            TwilioProgrammableVoiceError::ErrorCode31910 => None,
            TwilioProgrammableVoiceError::ErrorCode31953 => Some(r#"An RTP timeout is detected, meaning that inbound stream stopped flowing."#),
            TwilioProgrammableVoiceError::ErrorCode16024 => Some(r#"The participant label specified in an API call to create a voice conference participant was invalid. It may not exceed 128 characters, it may not be a CallSid, and it may not contain a ‘/’ (even in URL encoded form %2F)."#),
            TwilioProgrammableVoiceError::ErrorCode32650 => Some(r#"A 32650 Error is an indication there is something wrong with the &lt;Transcription&gt; configuration."#),
            TwilioProgrammableVoiceError::ErrorCode17008 => Some(r#"Query Timeout"#),
            TwilioProgrammableVoiceError::ErrorCode32106 => None,
            TwilioProgrammableVoiceError::ErrorCode32014 => None,
            TwilioProgrammableVoiceError::ErrorCode32651 => Some(r#"A 32651 Error is an indication that the &lt;Transcription&gt; provider configuration was rejected by the Speech-to-Text provider.
"#),
            TwilioProgrammableVoiceError::ErrorCode13326 => None,
            TwilioProgrammableVoiceError::ErrorCode31426 => None,
            TwilioProgrammableVoiceError::ErrorCode13750 => None,
            TwilioProgrammableVoiceError::ErrorCode13240 => None,
            TwilioProgrammableVoiceError::ErrorCode16110 => Some(r#"Despite our best efforts, we have failed to delete some or all of the selected recordings. The operation has been cancelled."#),
            TwilioProgrammableVoiceError::ErrorCode13220 => None,
            TwilioProgrammableVoiceError::ErrorCode21301 => Some(r#"Cannot create application: application limit exceeded "#),
            TwilioProgrammableVoiceError::ErrorCode31504 => None,
            TwilioProgrammableVoiceError::ErrorCode31930 => None,
            TwilioProgrammableVoiceError::ErrorCode37001 => Some(r#"The business-initiated WhatsApp call failed because it is not supported in the Twilio region."#),
            TwilioProgrammableVoiceError::ErrorCode32603 => Some(r#"A 32603 Error is an indication that &lt;VirtualAgent&gt; TwiML contains invalid or unsupported &lt;Config&gt; attributes"#),
            TwilioProgrammableVoiceError::ErrorCode17002 => Some(r#"The requested resource's call_sid is valid, but the call ended more than 30 days before the request was made."#),
            TwilioProgrammableVoiceError::ErrorCode32008 => None,
            TwilioProgrammableVoiceError::ErrorCode32711 => Some(r#"Twilio received an error response when attempting to send a User-Defined Message event to the UserDefinedMessageSubscription callback URL."#),
            TwilioProgrammableVoiceError::ErrorCode16010 => None,
            TwilioProgrammableVoiceError::ErrorCode16108 => None,
            TwilioProgrammableVoiceError::ErrorCode13521 => Some(r#"The maximum text length inside a <Say> verb is limited and varies depending on the option used. Basic, Polly and Google TTS have different limits and character count.  
See the <a href='/docs/voice/twiml/say#voice'>Say Verb</a> API Reference for more information."#),
            TwilioProgrammableVoiceError::ErrorCode16113 => Some(r#"A recording created as a mono recording cannot be downloaded as a dual-channel audio file."#),
            TwilioProgrammableVoiceError::ErrorCode15009 => Some(r#"The requested resource failed to complete the request."#),
            TwilioProgrammableVoiceError::ErrorCode10005 => Some(r#"Voice calling has been disabled for this account. "#),
            TwilioProgrammableVoiceError::ErrorCode64104 => Some(r#"Twilio Voice has a hard limit to the length of a call, once that is reached the call is terminated."#),
            TwilioProgrammableVoiceError::ErrorCode32220 => Some(r#"Twilio SIP domains allow endpoints to register through any supported SIP edge location. When dialing a registered endpoint, Twilio will fork the call to _all_ SIP devices registered under the same username/domain. Dialing one of your SIP domain's localized URIs (such as yourdomain.sip.ashburn.twilio.com) is not supported."#),
            TwilioProgrammableVoiceError::ErrorCode13340 => Some(r#"We are sorry for the inconvenience. You may be experiencing degraded <Gather> speech i.e. gather with input="speech dtmf" or input="speech"."#),
            TwilioProgrammableVoiceError::ErrorCode17400 => Some(r#"Invalid query parameter has been passed in the request."#),
            TwilioProgrammableVoiceError::ErrorCode32215 => None,
            TwilioProgrammableVoiceError::ErrorCode13430 => Some(r#"Play cannot use the DTMF characters in the configuration.  Must be an alphanumeric, asterisk, or pound sign."#),
            TwilioProgrammableVoiceError::ErrorCode22005 => Some(r#"## Error - 22005

### Call Queue Full 

There are more than 24 hours of calls in your call queue.

When you create calls via the REST API they are added to a call queue. Calls are removed from that call queue and placed at a default rate of 1 per second. This calls per second rate can be increased. Twilio will reject requests to place calls that will not be placed within 24 hours. At 1 call per second Twilio will reject calls once there are 86400 calls in your queue. More calls will not be accepted until your queue length decreases, calls in your queue will continue to be placed. If your application requires timely placement of calls, your application performance may be degraded."#),
            TwilioProgrammableVoiceError::ErrorCode31486 => None,
            TwilioProgrammableVoiceError::ErrorCode14226 => Some(r#"TaskRouter Enqueue not supported in this realm"#),
            TwilioProgrammableVoiceError::ErrorCode32654 => Some(r#"A 32654 Error is an indication that the &lt;Transcription&gt; TwiML contains an invalid or insecure URL in the statusCallback attribute when PCI mode is enabled on your account"#),
            TwilioProgrammableVoiceError::ErrorCode31008 => Some(r#"## Error - 31008

### Call cancelled

Unable to answer because the call has ended"#),
            TwilioProgrammableVoiceError::ErrorCode31000 => None,
            TwilioProgrammableVoiceError::ErrorCode31923 => None,
            TwilioProgrammableVoiceError::ErrorCode13327 => None,
            TwilioProgrammableVoiceError::ErrorCode16105 => None,
            TwilioProgrammableVoiceError::ErrorCode16028 => Some(r#"The participant to be whispered is not present in the conference, so the coach to the participant cannot be added to the conference."#),
            TwilioProgrammableVoiceError::ErrorCode13223 => Some(r#"Phone Numbers must be in E.164 format. E.164 numbers are formatted [+] [country code] [subscriber number including area code] and can have a maximum of 15 digits."#),
            TwilioProgrammableVoiceError::ErrorCode64107 => Some(r#"A message was received on the websocket that did not match any of the supported messages that ConversationRelay can act upon."#),
            TwilioProgrammableVoiceError::ErrorCode31207 => None,
            TwilioProgrammableVoiceError::ErrorCode13218 => None,
            TwilioProgrammableVoiceError::ErrorCode64008 => None,
            TwilioProgrammableVoiceError::ErrorCode13238 => Some(r#"The TwiML that executes via the waitUrl, holdUrl, or announceUrl can only contain Say, Play, Pause, and Redirect verbs. Dial, Gather, Hangup and Record are not allowed."#),
            TwilioProgrammableVoiceError::ErrorCode64007 => Some(r#"## Error - 64007

### Pay: Connector does not support creating charge.

 Error is an indication that the request to tokenize credit card failed because the connector does not support tokenization.

A 64007 Error is an indication that the request to create a charge on the credit card failed because the connector does not support making charges.
"#),
            TwilioProgrammableVoiceError::ErrorCode13239 => None,
            TwilioProgrammableVoiceError::ErrorCode31600 => None,
            TwilioProgrammableVoiceError::ErrorCode14219 => Some(r#"TaskRouter Dial Queue not supported in this realm"#),
            TwilioProgrammableVoiceError::ErrorCode31480 => None,
            TwilioProgrammableVoiceError::ErrorCode13810 => Some(r#"A call is rejected for any reason other than Busy or Hangup"#),
            TwilioProgrammableVoiceError::ErrorCode31487 => None,
            TwilioProgrammableVoiceError::ErrorCode32015 => Some(r#"## Warning - 32015

Call is terminated due to exceeding maximum call duration
"#),
            TwilioProgrammableVoiceError::ErrorCode32212 => None,
            TwilioProgrammableVoiceError::ErrorCode31903 => None,
            TwilioProgrammableVoiceError::ErrorCode32505 => Some(r#"Invalid data inside existing conversation."#),
            TwilioProgrammableVoiceError::ErrorCode31500 => None,
            TwilioProgrammableVoiceError::ErrorCode64103 => Some(r#"A 64103 Error is an indication there something happened internal in Twilio that affected your call."#),
            TwilioProgrammableVoiceError::ErrorCode13511 => Some(r#"The value of the voice attribute must be valid one.   
See the <a href='/docs/voice/twiml/say#voice'>Say Verb</a> API Reference for more information.
"#),
            TwilioProgrammableVoiceError::ErrorCode32105 => None,
            TwilioProgrammableVoiceError::ErrorCode31400 => None,
            TwilioProgrammableVoiceError::ErrorCode31404 => None,
            TwilioProgrammableVoiceError::ErrorCode64015 => Some(r#"A 64015 Error is an indication that the configured connector for the payment method is missing one of the Parameter needed in the `<Pay>` verb."#),
            TwilioProgrammableVoiceError::ErrorCode31902 => None,
            TwilioProgrammableVoiceError::ErrorCode32602 => Some(r#"A 32602 Error is an indication that the referenced connector is not a valid &lt;VirtualAgent&gt; connector."#),
            TwilioProgrammableVoiceError::ErrorCode13321 => Some(r#"The value of the voice attribute must be valid one.   
See the <a href='/docs/voice/twiml/say#voice'>Say Verb</a> API Reference for more information.
"#),
            TwilioProgrammableVoiceError::ErrorCode31102 => None,
            TwilioProgrammableVoiceError::ErrorCode31941 => Some(r#"Unsupported value of track attribute in TwiML"#),
            TwilioProgrammableVoiceError::ErrorCode13338 => Some(r#"Defined action on Gather verb is invalid or empty"#),
            TwilioProgrammableVoiceError::ErrorCode64001 => Some(r#"## Error - 64001

### Pay: Configuration Error

A 64001 Error is an indication there is something wrong with the &lt;Pay&gt; connector configuration."#),
            TwilioProgrammableVoiceError::ErrorCode32401 => Some(r#"Call delivery attempt(s) over the specified BYOC Trunk did not succeed. Failover to Twilio default routing took place."#),
            TwilioProgrammableVoiceError::ErrorCode31205 => None,
            TwilioProgrammableVoiceError::ErrorCode31003 => None,
            TwilioProgrammableVoiceError::ErrorCode21234 => None,
            TwilioProgrammableVoiceError::ErrorCode31105 => Some(r#"Client name should not contain control, space, delims, or unwise characters and should not be longer than 256 characters."#),
            TwilioProgrammableVoiceError::ErrorCode32504 => Some(r#"Existing Conversation did not have a Frontline worker participant. "#),
            TwilioProgrammableVoiceError::ErrorCode21262 => Some(r#"Answering Machine Detection was enabled on this call, but no AMD status callback URL was provided, so Twilio has no where to send the results when they are returned. AMD results will still be available in the /Calls resource once the call is complete."#),
            TwilioProgrammableVoiceError::ErrorCode32700 => Some(r#"Twilio experienced an unexpected internal error in attempting to deliver an User-Defined Message event to the Voice SDK client."#),
            TwilioProgrammableVoiceError::ErrorCode32002 => None,
            TwilioProgrammableVoiceError::ErrorCode64110 => Some(r#"The call was rejected because your account has been opted out of ConversationRelay."#),
            TwilioProgrammableVoiceError::ErrorCode21300 => Some(r#"You have specified a BYOC trunk SID that is malformed or non-existent."#),
            TwilioProgrammableVoiceError::ErrorCode31203 => None,
            TwilioProgrammableVoiceError::ErrorCode31950 => Some(r#"The Streamer has received a message that cannot be decoded
(This error is sent once per stream)"#),
            TwilioProgrammableVoiceError::ErrorCode21215 => Some(r#"You attempted to initiate an outbound phone call to a phone number that is not enabled on your account."#),
            TwilioProgrammableVoiceError::ErrorCode13621 => None,
            TwilioProgrammableVoiceError::ErrorCode32506 => Some(r#"Your Frontline instance failed to assign a worker after Conversation auto-creation."#),
            TwilioProgrammableVoiceError::ErrorCode31603 => None,
            TwilioProgrammableVoiceError::ErrorCode21216 => Some(r#"The destination number is blocked by Twilio."#),
            TwilioProgrammableVoiceError::ErrorCode31302 => Some(r#"This version of the SDK no longer supports processing cancel push notification messages. You must register via Voice.register(...) on Android or [TwilioVoice registerWithAccessToken:deviceToken:completion:] on iOS with this version of the SDK to stop receiving cancel push notification messages. Cancellations are now handled internally and reported to you on behalf of the SDK."#),
            TwilioProgrammableVoiceError::ErrorCode64021 => Some(r#"A 64021 Error is an indication invalid operation was provided."#),
            TwilioProgrammableVoiceError::ErrorCode13803 => Some(r#"SMS verb not supported in this realm"#),
            TwilioProgrammableVoiceError::ErrorCode32710 => Some(r#"Twilio experienced an unexpected internal error in attempting to deliver an User-Defined Message event to the callback URL provided in the Voice User-Defined Message Subscription request."#),
            TwilioProgrammableVoiceError::ErrorCode64013 => Some(r#"A 64013 Error is an indication that the provided paymentMethod attribute was not supported by the Connector."#),
            TwilioProgrammableVoiceError::ErrorCode13619 => None,
            TwilioProgrammableVoiceError::ErrorCode31005 => None,
            TwilioProgrammableVoiceError::ErrorCode32604 => Some(r#"A 32604 Error is an indication that there is an internal server error and some internal services failed."#),
            TwilioProgrammableVoiceError::ErrorCode32013 => Some(r#"#### Possible Causes 
*  Calls Per Second placed on a SIP domain exceeded the Parent Account limits. 

#### Possible Solutions
*  You've exceeded the number of calls per second you are authorized to make on your Parent Account for SIP Interface. "#),
            TwilioProgrammableVoiceError::ErrorCode31951 => Some(r#"The Streamer has received a message non compliant with the protocol
(This error is sent once per stream)"#),
            TwilioProgrammableVoiceError::ErrorCode32605 => Some(r#"A 32605 Error is an indication that the &lt;VirtualAgent&gt; TwiML contains an invalid or insecure URL in the statusCallback attribute when PCI mode is enabled on your account"#),
            TwilioProgrammableVoiceError::ErrorCode16104 => None,
            TwilioProgrammableVoiceError::ErrorCode31204 => None,
            TwilioProgrammableVoiceError::ErrorCode32022 => Some(r#"No ACK was received from your SIP endpoint 32 seconds after sending a 200 OK response, which resulted in the termination of the Inbound call. As per RFC 3261, the calling device’s SIP endpoint must send an ACK to acknowledge the receipt of a 200 OK response, to establish a successful call. "#),
            TwilioProgrammableVoiceError::ErrorCode64012 => Some(r#"## Error - 64012

### Pay: Payment Gateway rejected the card.

A 64012 Error is an indication that the provided card was rejected by the Payment Gateway."#),
            TwilioProgrammableVoiceError::ErrorCode31301 => None,
            TwilioProgrammableVoiceError::ErrorCode32210 => None,
            TwilioProgrammableVoiceError::ErrorCode31960 => Some(r#" Exceeded quota for Dialogflow API"#),
            TwilioProgrammableVoiceError::ErrorCode53401 => None,
            TwilioProgrammableVoiceError::ErrorCode64024 => Some(r#"This connector instance has not been approved by Twilio. Please contact Twilio support to gain approval for LIVE transactions."#),
            TwilioProgrammableVoiceError::ErrorCode31106 => None,
            TwilioProgrammableVoiceError::ErrorCode14300 => Some(r#"Start or Stop has an invalid noun defined."#),
            TwilioProgrammableVoiceError::ErrorCode64020 => Some(r#"A 64020 Error is an indication that one or more of the values provided for an attribute or parameter are invalid."#),
            TwilioProgrammableVoiceError::ErrorCode64101 => Some(r#"A 64101 Error is an indication there is something wrong with the &lt;ConversationRelay&gt; parameters."#),
            TwilioProgrammableVoiceError::ErrorCode32703 => Some(r#"The content body in the Voice User-Defined Message request exceeds the maximum allowed length of 10000 characters."#),
            TwilioProgrammableVoiceError::ErrorCode64102 => Some(r#"A 64102 Error is an indication there is something wrong with the &lt;ConversationRelay&gt; url parameter when Twilio attempted to connect to it."#),
            TwilioProgrammableVoiceError::ErrorCode32214 => None,
            TwilioProgrammableVoiceError::ErrorCode32221 => Some(r#"Twilio was unable to dial the specified SIP Endpoint. While the user did have at least one device registered, no devices were registered through the edge location specified in the URI."#),
            TwilioProgrammableVoiceError::ErrorCode16109 => None,
            TwilioProgrammableVoiceError::ErrorCode31900 => None,
            TwilioProgrammableVoiceError::ErrorCode32400 => Some(r#"Call delivery attempt(s) over the specified BYOC Trunk did not succeed. Failover routing was disabled."#),
            TwilioProgrammableVoiceError::ErrorCode16107 => None,
            TwilioProgrammableVoiceError::ErrorCode31403 => None,
            TwilioProgrammableVoiceError::ErrorCode32223 => Some(r#"Calling Twilio SIP domains such as "yourdomain.sip.twilio.com" is not allowed. You can only call a registered SIP endpoint such as "username@yourdomain.sip.twilio.com"."#),
            TwilioProgrammableVoiceError::ErrorCode13513 => Some(r#"Say rate value is invalid.  The function will use a default rate value of 150."#),
            TwilioProgrammableVoiceError::ErrorCode32600 => Some(r#"A 32600 Error is an indication there is something wrong with the &lt;VirtualAgent&gt; connector configuration."#),
            TwilioProgrammableVoiceError::ErrorCode31931 => Some(r#"Audio discarded due to buffer overflow in the Websocket downstream .
NOTE: This error is sent once per stream"#),
            TwilioProgrammableVoiceError::ErrorCode31924 => Some(r#"Malformed message or message not conformant with WebSocket protocol"#),
            TwilioProgrammableVoiceError::ErrorCode16021 => None,
            TwilioProgrammableVoiceError::ErrorCode31100 => None,
            TwilioProgrammableVoiceError::ErrorCode31201 => None,
            TwilioProgrammableVoiceError::ErrorCode16000 => None,
            TwilioProgrammableVoiceError::ErrorCode13219 => None,
            TwilioProgrammableVoiceError::ErrorCode14207 => Some(r#"The targeted queue has reached its maxSize limit. Once the queue reaches the limit, new requests to queue calls using <Enqueue> will be rejected."#),
            TwilioProgrammableVoiceError::ErrorCode31101 => None,
            TwilioProgrammableVoiceError::ErrorCode13512 => Some(r#"A Gather element has an invalid value for its "language" attribute. See [the accepted language values list](https://www.twilio.com/docs/voice/twiml/gather#languagetags) and update your Gather TwiML."#),
            TwilioProgrammableVoiceError::ErrorCode31007 => None,
            TwilioProgrammableVoiceError::ErrorCode31503 => Some(r#"## Error - 31503

### Service Unavailable

The server is currently unable to handle the request due to a temporary overloading or maintenance of the server. This error can also be caused by the Application SID provided in the access token pointing to an inaccessible URL."#),
            TwilioProgrammableVoiceError::ErrorCode13620 => Some(r#"Record: Transcription feature not available for this recording."#),
            TwilioProgrammableVoiceError::ErrorCode31001 => None,
            TwilioProgrammableVoiceError::ErrorCode13337 => Some(r#"Gather: callback must be over HTTPS when using gather with PCI compliance"#),
            TwilioProgrammableVoiceError::ErrorCode13258 => Some(r#"Dial->Sim not supported in this realm"#),
            TwilioProgrammableVoiceError::ErrorCode32501 => Some(r#"Invalid attributes found in <Conversation> noun."#),
            TwilioProgrammableVoiceError::ErrorCode13256 => Some(r#"A recording was initiated using an invalid URL for the recordingStatusCallback parameter.
The recording will be processed, but status notifications will not be sent."#),
            TwilioProgrammableVoiceError::ErrorCode13214 => Some(r#"## Warning - 13214 

### Dial: Invalid callerId value

callerId must be the calling number, called number, or a validated outgoing number.  Additionally,
if the calling leg is a client leg, the caller ID is required and must be a validated outgoing number. 
See the <a href='/docs/api/twiml/dial#callerId'>Dial Verb</a> API Reference for more information."#),
            TwilioProgrammableVoiceError::ErrorCode16111 => Some(r#"Twilio failed to upload the recording file to the external AWS S3 bucket specified in Voice Settings due to an invalid configuration. The recording file has been deleted, and if the resource exists, the resource has been marked as “failed”."#),
            TwilioProgrammableVoiceError::ErrorCode32655 => Some(r#"A 32655 Error indicates a connection to the configured Intelligence Service could not be established due to an internal error. While you will continue to receive real-time transcription events, no Language Operators will be executed, and the transcript will not be saved"#),
            TwilioProgrammableVoiceError::ErrorCode16001 => None,
            TwilioProgrammableVoiceError::ErrorCode13622 => Some(r#"## Warning - 13622 

### Record: invalid recordingTrack value

The recordingTrack can be "inbound", "outbound", or "both""#),
            TwilioProgrammableVoiceError::ErrorCode13334 => Some(r#"## Warning - 13334

### Gather: Invalid model value"#),
            TwilioProgrammableVoiceError::ErrorCode31401 => Some(r#"The browser or end-user denied permissions to user media. Therefore we were unable to acquire input audio."#),
            TwilioProgrammableVoiceError::ErrorCode13225 => Some(r#"The destination number is blocked by Twilio."#),
            TwilioProgrammableVoiceError::ErrorCode22001 => Some(r#"The call timed out before system could complete the call"#),
            TwilioProgrammableVoiceError::ErrorCode31942 => Some(r#"Connector invalid configuration"#),
            TwilioProgrammableVoiceError::ErrorCode31481 => None,
            TwilioProgrammableVoiceError::ErrorCode32502 => Some(r#"Internal error occurred when executing a request to one of the status callback URLs specified in the Conversation noun. This error will occur when Voice Conversation service fails to reach out to Callback service."#),
            TwilioProgrammableVoiceError::ErrorCode31006 => None,
            TwilioProgrammableVoiceError::ErrorCode13699 => Some(r#"If a recording has an invalid trim value in configuration.  Defaults to no trim."#),
            TwilioProgrammableVoiceError::ErrorCode31921 => None,
            TwilioProgrammableVoiceError::ErrorCode32018 => Some(r#"Twiml size exceeded maximum allowed value of 4000 characters"#),
            TwilioProgrammableVoiceError::ErrorCode64003 => Some(r#"## Error - 64003

### Pay: Invalid charge amount.

A 64003 Error is an indication that the charge amount is invalid. "#),
            TwilioProgrammableVoiceError::ErrorCode64109 => Some(r#"The call was rejected because you reached the maximum limit of concurrent calls available for your account."#),
            TwilioProgrammableVoiceError::ErrorCode16027 => Some(r#"The participant to be whispered is on hold, so the coach to the participant cannot be added to the conference."#),
            TwilioProgrammableVoiceError::ErrorCode31484 => Some(r#"The provided phone number is malformed."#),
            TwilioProgrammableVoiceError::ErrorCode31210 => Some(r#"The Message Type used for sending Call Message Event is invalid.
Please see Twilio's <a href="/docs/voice/sdks/call-message-events/"> Voice SDK Call Message Events Documentation</a> for more information on valid Message Type.
"#),
            TwilioProgrammableVoiceError::ErrorCode16112 => Some(r#"Twilio failed to upload the recording file to the external AWS S3 bucket specified in Voice Settings due to access denied. The recording file has been deleted, and the resource has been marked as “failed”."#),
            TwilioProgrammableVoiceError::ErrorCode13216 => Some(r#"## Warning - 13216 

### Invalid timeLimit value

timeLimit must be a positive integer, in seconds. See the <a href='/docs/api/twiml/dial#timeLimit'>Dial Verb</a> API Reference for more information.





"#),
            TwilioProgrammableVoiceError::ErrorCode14218 => Some(r#"Unable to update external user queue"#)
        }
    }
}
