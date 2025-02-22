// AUTO-GENERATED. DO NOT MODIFY. YOUR CHANGES WILL BE OVER-WRITTEN
use crate::products::TwilioProgrammableVoiceError;
use standard_error::traits::StandardErrorCausesTrait;

impl StandardErrorCausesTrait for TwilioProgrammableVoiceError {
    fn causes(&self) -> Option<&'static str> {
        match self {
            TwilioProgrammableVoiceError::ErrorCode11750 => Some(r#"*   The TwiML that you are serving is larger than 64 kB
*   You are serving non-TwiML content in your response; e.g. error message output"#),
            TwilioProgrammableVoiceError::ErrorCode16026 => Some(r#"1. The specified label was already used in another API call to add a participant to the conference.
2. The specified label was already used in twiml (Dial Conference) to add an inbound participant to the conference"#),
            TwilioProgrammableVoiceError::ErrorCode13802 => Some(r#"* Your `<Dial>` verb did not include a `referUrl` attribute or the attribute was misspelled"#),
            TwilioProgrammableVoiceError::ErrorCode16002 => None,
            TwilioProgrammableVoiceError::ErrorCode16003 => None,
            TwilioProgrammableVoiceError::ErrorCode64106 => Some(r#"* One of the parameters in to ConversationRelay is not supported by the chosen provider
See message for more details"#),
            TwilioProgrammableVoiceError::ErrorCode64017 => Some(r#"* PaymentMethod selected is "credit-card" and a value is provided for BankAccountNumber Parameter"#),
            TwilioProgrammableVoiceError::ErrorCode32702 => Some(r#"* Content data in the POST request to the UserDefinedMessage Resource is not as expected, or there is no content in the request."#),
            TwilioProgrammableVoiceError::ErrorCode32652 => Some(r#"* Some of the &lt;Config&gt; attribute(s) referenced in the &lt;Transcription&gt; TwiML are not supported by the Transcription provider"#),
            TwilioProgrammableVoiceError::ErrorCode64009 => Some(r#"* The Twilio application had its privileges revoked from your account."#),
            TwilioProgrammableVoiceError::ErrorCode64016 => Some(r#"* Action URL does not use https://"#),
            TwilioProgrammableVoiceError::ErrorCode37000 => Some(r#"The business-initiated WhatsApp call was placed to the consumer without the necessary call permission being accepted."#),
            TwilioProgrammableVoiceError::ErrorCode13342 => Some(r#"Invalid configuration for Google STT V2 provider"#),
            TwilioProgrammableVoiceError::ErrorCode21302 => Some(r#"You are approaching the limit of 1000 TwiML	 applications."#),
            TwilioProgrammableVoiceError::ErrorCode31429 => None,
            TwilioProgrammableVoiceError::ErrorCode13801 => Some(r#"* Your application attempted to invoke the Refer verb on a PSTN or Twilio Client call leg."#),
            TwilioProgrammableVoiceError::ErrorCode31904 => Some(r#"* Wrong IP address or DNS name
* IP address belongs to the private domain and it is non-routable
* Host is down or disconnected from router"#),
            TwilioProgrammableVoiceError::ErrorCode32653 => Some(r#"* Some error occurred during communication with Twilio internal services"#),
            TwilioProgrammableVoiceError::ErrorCode64002 => Some(r#"* Some internal services in Twilio were not available.
* Payment provider not available."#),
            TwilioProgrammableVoiceError::ErrorCode31922 => Some(r#"* URL schema is not supported"#),
            TwilioProgrammableVoiceError::ErrorCode16020 => None,
            TwilioProgrammableVoiceError::ErrorCode31901 => Some(r#"* WebSocket server is not connected to any network
* Bad IP address provided in the TwiML URL
* Intermediate elements (like ngrok) are not routing traffic
* There is a firewall blocking the traffic"#),
            TwilioProgrammableVoiceError::ErrorCode31940 => Some(r#"A 31940 Error is an indication that the value provided with connectorName attribute does not match to an installed configuration.
"#),
            TwilioProgrammableVoiceError::ErrorCode17000 => Some(r#"This error code is also returned when an account is attempting to access a resource where Voice Insights Advanced Features has not been enabled on that account."#),
            TwilioProgrammableVoiceError::ErrorCode13328 => None,
            TwilioProgrammableVoiceError::ErrorCode13332 => None,
            TwilioProgrammableVoiceError::ErrorCode64011 => None,
            TwilioProgrammableVoiceError::ErrorCode64005 => Some(r#"* The paymentConnector attribute points to Connector that does not support tokenization.
* The chargeAmount attribute in the Pay Verb TwiML was 0 (or omitted) and you intended to charge."#),
            TwilioProgrammableVoiceError::ErrorCode13335 => Some(r#"speechTimeout value invalid"#),
            TwilioProgrammableVoiceError::ErrorCode31952 => Some(r#"Stream Extension not found"#),
            TwilioProgrammableVoiceError::ErrorCode16099 => None,
            TwilioProgrammableVoiceError::ErrorCode32701 => Some(r#"* Value of Content-Type in the POST request to the UserDefinedMessage Resource is not as expected, or there is no Content-Type header in the request."#),
            TwilioProgrammableVoiceError::ErrorCode13330 => None,
            TwilioProgrammableVoiceError::ErrorCode31409 => None,
            TwilioProgrammableVoiceError::ErrorCode16102 => None,
            TwilioProgrammableVoiceError::ErrorCode31009 => None,
            TwilioProgrammableVoiceError::ErrorCode16101 => None,
            TwilioProgrammableVoiceError::ErrorCode15004 => Some(r#"A relative URL is provided for action callbacks when updating in-progress call with TwiML."#),
            TwilioProgrammableVoiceError::ErrorCode13804 => Some(r#"AddOns are not supported in this realm"#),
            TwilioProgrammableVoiceError::ErrorCode16106 => None,
            TwilioProgrammableVoiceError::ErrorCode31103 => None,
            TwilioProgrammableVoiceError::ErrorCode32019 => Some(r#"Both Voice URL and embedded twiml are provided in the request"#),
            TwilioProgrammableVoiceError::ErrorCode21263 => Some(r#"AsyncAmd, AsyncAmdStatusCallback, and/or AsyncAmdStatusCallbackMethod parameters from POST to /Calls API have been passed in a POST to /Participants API, or added to <Dial><Number> or <Dial><Sip> TwiML."#),
            TwilioProgrammableVoiceError::ErrorCode31502 => None,
            TwilioProgrammableVoiceError::ErrorCode31206 => None,
            TwilioProgrammableVoiceError::ErrorCode16011 => None,
            TwilioProgrammableVoiceError::ErrorCode64023 => Some(r#"* Your pay connector is in TEST mode and you provided an invalid test bank account number when testing an ACH payment. For compliance reasons, we can only allow certain test bank account numbers in TEST mode."#),
            TwilioProgrammableVoiceError::ErrorCode31408 => None,
            TwilioProgrammableVoiceError::ErrorCode64018 => Some(r#"* Parameter value not provided for both Capture and Status 
* Parameter value provided for both Capture and Status"#),
            TwilioProgrammableVoiceError::ErrorCode53404 => None,
            TwilioProgrammableVoiceError::ErrorCode16023 => Some(r#"A POST to conference participants call to the Twilio API specified a participant label which is greater than 128 characters or which resembles a CallSid (“CA” followed by 32 lowercase hexadecimal characters), or contains the ‘/’ character or URL encoded ‘/’ character (%2F)."#),
            TwilioProgrammableVoiceError::ErrorCode31209 => Some(r#"The new identity for the reconnection and the old identity for the original call does not match."#),
            TwilioProgrammableVoiceError::ErrorCode13805 => Some(r#"Account is a trial account"#),
            TwilioProgrammableVoiceError::ErrorCode16022 => None,
            TwilioProgrammableVoiceError::ErrorCode64010 => None,
            TwilioProgrammableVoiceError::ErrorCode16025 => Some(r#"1. The same static twiml specifying a participant label was served to two difference calls
2. Dynamically generated twiml specified the same participant label for two different calls
3. The participant label specified in the twiml Is the same as one specified for an outbound participant that was already added to the conference using the Twilio API "#),
            TwilioProgrammableVoiceError::ErrorCode13331 => None,
            TwilioProgrammableVoiceError::ErrorCode31402 => Some(r#"* NotFoundError - The deviceID specified was not found.
* The getUserMedia constraints were overconstrained and no devices matched."#),
            TwilioProgrammableVoiceError::ErrorCode64019 => Some(r#"* Postal code and Security code were marked as required but not captured during the payment flow
* BankAccountNumber and BankRoutingNumber were not captured
* Credit Card Number and Expiration Date were not captured
* Status parameter value was set to "complete" before all payment information was captured"#),
            TwilioProgrammableVoiceError::ErrorCode17009 => Some(r#"The requested resource failed to complete the request. "#),
            TwilioProgrammableVoiceError::ErrorCode31211 => Some(r#"The Call should be at least in the Ringing state to Subscribe and send Call Message."#),
            TwilioProgrammableVoiceError::ErrorCode64006 => Some(r#"* The paymentConnector attribute points to a Connector that does not support the requested type of tokenization."#),
            TwilioProgrammableVoiceError::ErrorCode32021 => Some(r#"See the error message for details"#),
            TwilioProgrammableVoiceError::ErrorCode31604 => None,
            TwilioProgrammableVoiceError::ErrorCode32500 => Some(r#"- Some error occurred during communication with Twilio internal services."#),
            TwilioProgrammableVoiceError::ErrorCode32101 => Some(r#"The called number is not a valid [E.164](https://www.twilio.com/docs/glossary/what-e164) number."#),
            TwilioProgrammableVoiceError::ErrorCode64014 => Some(r#"Electronic Check Processing requires a name."#),
            TwilioProgrammableVoiceError::ErrorCode17001 => Some(r#"The call SID for the call is valid, but the call summarization process was not complete at the time of the request. Summarization takes 30 minutes to complete."#),
            TwilioProgrammableVoiceError::ErrorCode32009 => None,
            TwilioProgrammableVoiceError::ErrorCode64004 => None,
            TwilioProgrammableVoiceError::ErrorCode64022 => Some(r#"* Your pay connector is in TEST mode and you provided an invalid test card number when testing a payment. For compliance reasons, we can only allow certain test card numbers in TEST mode."#),
            TwilioProgrammableVoiceError::ErrorCode31002 => None,
            TwilioProgrammableVoiceError::ErrorCode13247 => Some(r#"* You have supplied a phone number that was not in [E.164 format](https://www.twilio.com/docs/glossary/what-e164)
* Your `From` phone number is on a do-not-originate (DNO) list"#),
            TwilioProgrammableVoiceError::ErrorCode13257 => Some(r#"Transcribe callback URL is invalid."#),
            TwilioProgrammableVoiceError::ErrorCode31202 => Some(r#"The access token has an invalid Account SID, API Key, or API Key Secret."#),
            TwilioProgrammableVoiceError::ErrorCode31212 => Some(r#"The payload size of Call Message Event exceeds the authorized limit."#),
            TwilioProgrammableVoiceError::ErrorCode32503 => Some(r#"- This error indicates that Twilio was able to reach the customer-provided URL, however, received an error from the customer-side callback endpoint.
- The URL provided was likely valid, but Twilio might be receiving an error from the customer-side service."#),
            TwilioProgrammableVoiceError::ErrorCode13329 => None,
            TwilioProgrammableVoiceError::ErrorCode64105 => Some(r#"* The websocket server at the url is no longer healthy
* Possible software problem in the websocket server (not Twilio)
* Network issues"#),
            TwilioProgrammableVoiceError::ErrorCode32601 => Some(r#"* Connector configuration contains invalid properties
"#),
            TwilioProgrammableVoiceError::ErrorCode64108 => Some(r#"* Phone call finished
* Carrier stopped sending audio to Twilio"#),
            TwilioProgrammableVoiceError::ErrorCode32606 => Some(r#"* &lt;Config name="resumeEndUserId"&gt; element referenced in TwiML contains an invalid value
* &lt;Config name="resumeEndUserId"&gt; element referenced in TwiML is not authorized to resume the conversation from the current Call SID"#),
            TwilioProgrammableVoiceError::ErrorCode31920 => Some(r#"* The server does not support WebSocket
* The WebSocket protocol is not enabled for the requested URL"#),
            TwilioProgrammableVoiceError::ErrorCode17007 => Some(r#"Voice Insights Advanced Features not enabled on the account when the call was made."#),
            TwilioProgrammableVoiceError::ErrorCode13333 => Some(r#"Profanity filter is invalid"#),
            TwilioProgrammableVoiceError::ErrorCode31530 => None,
            TwilioProgrammableVoiceError::ErrorCode13227 => Some(r#"User dialed a destination your application is not enabled to support calling to."#),
            TwilioProgrammableVoiceError::ErrorCode32007 => None,
            TwilioProgrammableVoiceError::ErrorCode31104 => None,
            TwilioProgrammableVoiceError::ErrorCode31208 => None,
            TwilioProgrammableVoiceError::ErrorCode31910 => Some(r#"* Server doesn’t support SSL
* Stream is trying to connect the wrong port
* Protocol or SSL handshake has failed"#),
            TwilioProgrammableVoiceError::ErrorCode31953 => Some(r#"* Phone call finished
* Carrier stopped sending audio to Twilio"#),
            TwilioProgrammableVoiceError::ErrorCode16024 => Some(r#"A POST to conference participants call to the Twilio API specified a participant label which is greater than 128 characters or which resembles a CallSid (“CA” followed by 32 lowercase hexadecimal characters), or contains the ‘/’ character or URL encoded ‘/’ character (%2F)."#),
            TwilioProgrammableVoiceError::ErrorCode32650 => Some(r#"* Configuration is missing required properties
* Configuration contains invalid properties"#),
            TwilioProgrammableVoiceError::ErrorCode17008 => Some(r#"The requested resource exceeded the time limit for the data to be fetched from the underlying data store."#),
            TwilioProgrammableVoiceError::ErrorCode32106 => None,
            TwilioProgrammableVoiceError::ErrorCode32014 => None,
            TwilioProgrammableVoiceError::ErrorCode32651 => Some(r#"* Provider configuration contains invalid or unsupported properties"#),
            TwilioProgrammableVoiceError::ErrorCode13326 => None,
            TwilioProgrammableVoiceError::ErrorCode31426 => None,
            TwilioProgrammableVoiceError::ErrorCode13750 => None,
            TwilioProgrammableVoiceError::ErrorCode13240 => None,
            TwilioProgrammableVoiceError::ErrorCode16110 => Some(r#"There was an internal failure when processing your request. This action will not be retried."#),
            TwilioProgrammableVoiceError::ErrorCode13220 => Some(r#"Dial: Invalid ringTone value"#),
            TwilioProgrammableVoiceError::ErrorCode21301 => Some(r#"You have attempted to create a new application that exceeds the limit of 1000 applications."#),
            TwilioProgrammableVoiceError::ErrorCode31504 => None,
            TwilioProgrammableVoiceError::ErrorCode31930 => Some(r#"* The server has an upstream bandwidth restriction"#),
            TwilioProgrammableVoiceError::ErrorCode37001 => Some(r#"Business-initiated WhatsApp calls are not supported in all Twilio regions."#),
            TwilioProgrammableVoiceError::ErrorCode32603 => Some(r#"* Some of the &lt;Config&gt; attribute(s) referenced in the &lt;VirtualAgent&gt; TwiML are not supported by the VirtualAgent provider. "#),
            TwilioProgrammableVoiceError::ErrorCode17002 => Some(r#"Voice Insights only stores 30 days worth of call data."#),
            TwilioProgrammableVoiceError::ErrorCode32008 => None,
            TwilioProgrammableVoiceError::ErrorCode32711 => Some(r#"* The UserDefinedMessageSubscription callback URL is incorrect.
* Your endpoint at the UserDefinedMessageSubscription callback URL was not responding or responded with a 400- or 500-level HTTP status code."#),
            TwilioProgrammableVoiceError::ErrorCode16010 => None,
            TwilioProgrammableVoiceError::ErrorCode16108 => None,
            TwilioProgrammableVoiceError::ErrorCode13521 => Some(r#"The maximum text length inside the `<Say>` verb has been exceeded."#),
            TwilioProgrammableVoiceError::ErrorCode16113 => Some(r#"* The recording was requested as a mono recording upon creation"#),
            TwilioProgrammableVoiceError::ErrorCode15009 => Some(r#"The requested resource failed to complete the request."#),
            TwilioProgrammableVoiceError::ErrorCode10005 => Some(r#"Your account may have been flagged for review."#),
            TwilioProgrammableVoiceError::ErrorCode64104 => Some(r#"* This call reached the maximum duration."#),
            TwilioProgrammableVoiceError::ErrorCode32220 => Some(r#"* Attempting to place a call to username@yourdomain.sip.*ashburn*.twilio.com, or any other edge variant (e.g. ashburn-ix, umatilla, sydney, etc.)
* Note that existing registration customers dialing yourdomain.sip.us1.twilio.com or yourdomain.sip.us1-ix.twilio.com _are_ allowed (although this behavior is deprecated)."#),
            TwilioProgrammableVoiceError::ErrorCode13340 => Some(r#"Internal Causes"#),
            TwilioProgrammableVoiceError::ErrorCode17400 => Some(r#"Invalid format for country code, SIP URI, Client address, or phone number. Wildcard searches."#),
            TwilioProgrammableVoiceError::ErrorCode32215 => None,
            TwilioProgrammableVoiceError::ErrorCode13430 => Some(r#"Invalid value in DTMF String"#),
            TwilioProgrammableVoiceError::ErrorCode22005 => Some(r#"* Large spikes in call volume
* Runaway or looping application"#),
            TwilioProgrammableVoiceError::ErrorCode31486 => None,
            TwilioProgrammableVoiceError::ErrorCode14226 => Some(r#"TaskRouter Enqueue not supported in this realm"#),
            TwilioProgrammableVoiceError::ErrorCode32654 => Some(r#"* &lt;Transcription&gt; statusCallback attribute contains an invalid URL
* &lt;Transcription&gt; statusCallback attribute does not specify scheme https"#),
            TwilioProgrammableVoiceError::ErrorCode31008 => Some(r#"This error indicates that the incoming call was cancelled because it was not answered in time or it was accepted/rejected by another application instance registered with the same identity."#),
            TwilioProgrammableVoiceError::ErrorCode31000 => None,
            TwilioProgrammableVoiceError::ErrorCode31923 => Some(r#"* URL schema is malformed"#),
            TwilioProgrammableVoiceError::ErrorCode13327 => None,
            TwilioProgrammableVoiceError::ErrorCode16105 => None,
            TwilioProgrammableVoiceError::ErrorCode16028 => Some(r#"This may because the specified participant is not yet connected to the conference or no longer connected to the conference."#),
            TwilioProgrammableVoiceError::ErrorCode13223 => Some(r#"- The phone number you are attempting to dial is not in E.164 format
- The `<Number>` noun inside your `<Dial>` TwiML instructions is not present
- You are attempting to dial something that is not a phone number"#),
            TwilioProgrammableVoiceError::ErrorCode64107 => Some(r#"* The websocket server is sending ConversationRelay a message that is not supported
* A component in front of the websocket server (API Gateway or Load Balancer) is sending ConversationRelay a message it cannot process"#),
            TwilioProgrammableVoiceError::ErrorCode31207 => None,
            TwilioProgrammableVoiceError::ErrorCode13218 => Some(r#"sequential value is not "true" or "false"."#),
            TwilioProgrammableVoiceError::ErrorCode64008 => None,
            TwilioProgrammableVoiceError::ErrorCode13238 => Some(r#"Dial, Gather, Hangup or Record verbs are used in the TwiML."#),
            TwilioProgrammableVoiceError::ErrorCode64007 => Some(r#"* The paymentConnector attribute points to a Connector that does not support charging."#),
            TwilioProgrammableVoiceError::ErrorCode13239 => None,
            TwilioProgrammableVoiceError::ErrorCode31600 => None,
            TwilioProgrammableVoiceError::ErrorCode14219 => Some(r#"TaskRouter Dial Queue not supported in this realm"#),
            TwilioProgrammableVoiceError::ErrorCode31480 => None,
            TwilioProgrammableVoiceError::ErrorCode13810 => Some(r#"Could be timeout, cancelled, or no-answer."#),
            TwilioProgrammableVoiceError::ErrorCode31487 => None,
            TwilioProgrammableVoiceError::ErrorCode32015 => Some(r#"Maximum call duration as been exceeded."#),
            TwilioProgrammableVoiceError::ErrorCode32212 => None,
            TwilioProgrammableVoiceError::ErrorCode31903 => Some(r#"* WebSocket server is down
* WebSocket server has experienced a problem causing the connection to close
* Connection has been lost
* End of stream is found. This happens when the socket has been closed by the Server leaving the websocket in an inconsistent state. The error is fired when the Streamer tries to read or write in a closed TCP socket."#),
            TwilioProgrammableVoiceError::ErrorCode32505 => Some(r#"- Certain data was wrong when the Conversation was created or when the participants were added."#),
            TwilioProgrammableVoiceError::ErrorCode31500 => None,
            TwilioProgrammableVoiceError::ErrorCode64103 => Some(r#"* Network issues
* Other unknown issues"#),
            TwilioProgrammableVoiceError::ErrorCode13511 => Some(r#"* You have entered an invalid attribute for voice parameter
"#),
            TwilioProgrammableVoiceError::ErrorCode32105 => None,
            TwilioProgrammableVoiceError::ErrorCode31400 => None,
            TwilioProgrammableVoiceError::ErrorCode31404 => None,
            TwilioProgrammableVoiceError::ErrorCode64015 => Some(r#"The connector needs a specific Parameter in the `<Pay>` verb"#),
            TwilioProgrammableVoiceError::ErrorCode31902 => Some(r#"* The stream is trying to connect the wrong port
* The WebSocket server is down"#),
            TwilioProgrammableVoiceError::ErrorCode32602 => Some(r#"* The connector referenced by the supplied connectorName does not exist
* The connector referenced by the supplied connectorName is not a VirtualAgent connector
* The supplied TwiML is missing the connectorName attribute"#),
            TwilioProgrammableVoiceError::ErrorCode13321 => Some(r#"* You have entered an invalid attribute for voice parameter"#),
            TwilioProgrammableVoiceError::ErrorCode31102 => None,
            TwilioProgrammableVoiceError::ErrorCode31941 => Some(r#"* Value different of "inbound_track" is used with verb Connect
* Value different of "inbound_track", "outbound_track" or "both_tracks" is used with verb Start"#),
            TwilioProgrammableVoiceError::ErrorCode13338 => Some(r#"Defined action on Gather verb is invalid or empty"#),
            TwilioProgrammableVoiceError::ErrorCode64001 => Some(r#"* Credentials were removed from configuration.
* Credentials were modified and are not matching the Connector Type.
* Configuration is missing Credentials.
* Configuration contains invalid attributes."#),
            TwilioProgrammableVoiceError::ErrorCode32401 => Some(r#"Twilio encountered errors communicating with your SIP communications infrastructure."#),
            TwilioProgrammableVoiceError::ErrorCode31205 => None,
            TwilioProgrammableVoiceError::ErrorCode31003 => None,
            TwilioProgrammableVoiceError::ErrorCode21234 => None,
            TwilioProgrammableVoiceError::ErrorCode31105 => Some(r#"Client name contains invalid characters or is longer than 256 characters."#),
            TwilioProgrammableVoiceError::ErrorCode32504 => Some(r#"- Issue with Frontline routing configuration that caused no worker assigned.
- Conversation might not have been created by the Frontline service. This error indicates that the code is directly using Conversation API to the create the conversation, and Frontline cannot determine the format of the call.
- Something went wrong when the Conversation was created by the Frontline worker or admin. "#),
            TwilioProgrammableVoiceError::ErrorCode21262 => Some(r#"No AMD status callback URL was provided"#),
            TwilioProgrammableVoiceError::ErrorCode32700 => Some(r#"* An error occurred during communication with Twilio internal services."#),
            TwilioProgrammableVoiceError::ErrorCode32002 => None,
            TwilioProgrammableVoiceError::ErrorCode64110 => Some(r#"Account opted out of ConversationRelay."#),
            TwilioProgrammableVoiceError::ErrorCode21300 => Some(r#"* The SID in question is not formatted correctly. A valid BYOC trunk SID starts with prefix "BY" and is 34 characters in length.
* The BYOC trunk identified by the SID does not exist. If it existed in the past, it has since been removed.
* The BYOC trunk exists but ownership is with a different account.
"#),
            TwilioProgrammableVoiceError::ErrorCode31203 => None,
            TwilioProgrammableVoiceError::ErrorCode31950 => Some(r#"* WebSocket binary format is used"#),
            TwilioProgrammableVoiceError::ErrorCode21215 => Some(r#"User dialed a destination your application is not enabled to support calling to."#),
            TwilioProgrammableVoiceError::ErrorCode13621 => None,
            TwilioProgrammableVoiceError::ErrorCode32506 => Some(r#"- Issue with Frontline routing configuration that caused no worker assigned.
- Conversation might not have been created by the Frontline service. This error indicates that the code is directly using Conversation API to the create the conversation, and Frontline cannot determine the format of the call.
- Something went wrong when the Conversation was created by the Frontline worker or admin. "#),
            TwilioProgrammableVoiceError::ErrorCode31603 => None,
            TwilioProgrammableVoiceError::ErrorCode21216 => Some(r#"The outbound call to the destination number was blocked by Twilio. Potential causes may be:
- The destination has a high-risk of fraud
- Due to regulatory reasons, the destination cannot be reached
- You are placing a call to a +1 destination and your account is missing a Primary Customer Profile"#),
            TwilioProgrammableVoiceError::ErrorCode31302 => Some(r#"The identity associated with the Twilio Voice SDK is still registered to receive cancel push notification messages."#),
            TwilioProgrammableVoiceError::ErrorCode64021 => Some(r#"* Start operation on already started Pay session"#),
            TwilioProgrammableVoiceError::ErrorCode13803 => Some(r#"SMS verb not supported in this realm"#),
            TwilioProgrammableVoiceError::ErrorCode32710 => Some(r#"* An error occurred during communication with Twilio internal services."#),
            TwilioProgrammableVoiceError::ErrorCode64013 => Some(r#"The targeted Payment Connector did not support the paymentMethod. For instance, a credit card might have been supplied but the Connector did not support Credit Cards."#),
            TwilioProgrammableVoiceError::ErrorCode13619 => None,
            TwilioProgrammableVoiceError::ErrorCode31005 => None,
            TwilioProgrammableVoiceError::ErrorCode32604 => Some(r#"* Some error occurred during communication with Twilio internal services."#),
            TwilioProgrammableVoiceError::ErrorCode32013 => Some(r#"Calls Per Second placed on a SIP domain exceeded the Parent Account limits. "#),
            TwilioProgrammableVoiceError::ErrorCode31951 => Some(r#"* Message does not have JSON format
* Unknown message type
* Missing or extra field in message
* Wrong Stream SID used in message"#),
            TwilioProgrammableVoiceError::ErrorCode32605 => Some(r#"* &lt;VirtualAgent&gt; statusCallback attribute contains an invalid URL
* &lt;VirtualAgent&gt; statusCallback attribute does not specify scheme https
"#),
            TwilioProgrammableVoiceError::ErrorCode16104 => None,
            TwilioProgrammableVoiceError::ErrorCode31204 => None,
            TwilioProgrammableVoiceError::ErrorCode32022 => Some(r#"* Your NAT configuration may be incorrect.
* Your Firewall may be blocking traffic.
* SIP signaling not adhering to RFC 3261. For example ACK sent to the wrong IP address or using incorrect Route headers or Request URI.
* A network disruption or packet loss within or between your network and the Twilio network may have caused the ACK messages to get lost."#),
            TwilioProgrammableVoiceError::ErrorCode64012 => Some(r#"* The card used was rejected by the Payment Gateway."#),
            TwilioProgrammableVoiceError::ErrorCode31301 => None,
            TwilioProgrammableVoiceError::ErrorCode32210 => None,
            TwilioProgrammableVoiceError::ErrorCode31960 => Some(r#"*The Dialogflow API quotas has been exceeded"#),
            TwilioProgrammableVoiceError::ErrorCode53401 => None,
            TwilioProgrammableVoiceError::ErrorCode64024 => Some(r#"This is caused by your connector instance not having been approved by Twilio for LIVE transactions. You must go through a PCI review in order to gain approval for LIVE transactions."#),
            TwilioProgrammableVoiceError::ErrorCode31106 => None,
            TwilioProgrammableVoiceError::ErrorCode14300 => Some(r#"Start or Stop has an invalid noun defined."#),
            TwilioProgrammableVoiceError::ErrorCode64020 => Some(r#"* Misspelled parameter values
* Incorrect format for parameter values
* Unexpcted value for one or more parameters"#),
            TwilioProgrammableVoiceError::ErrorCode64101 => Some(r#"* One of the submitted attributes to the TwiML is invalid."#),
            TwilioProgrammableVoiceError::ErrorCode32703 => Some(r#"* The content body in the Voice POST request to the UserDefinedMessage.Resource exceeds the maximum allowed length of 10000 characters."#),
            TwilioProgrammableVoiceError::ErrorCode64102 => Some(r#"* The url parameter was incorrectly entered
* There is not a websocket server available at the url
* The websocket server at the url is not responding correctly
"#),
            TwilioProgrammableVoiceError::ErrorCode32214 => None,
            TwilioProgrammableVoiceError::ErrorCode32221 => Some(r#"* You are trying to migrate from us1-only or us1-ix-only SIP registration and have configured some devices to register through other edges, but you are still using edge-specific URIs (e.g. username@yourdomain.sip.us1.twilio.com) when placing calls in your application.
* If you only intend to use the us1 or us1-ix edges, you may have SIP devices unexpectedly registering through another edge. Check the domain name and outbound proxy settings on your client devices."#),
            TwilioProgrammableVoiceError::ErrorCode16109 => None,
            TwilioProgrammableVoiceError::ErrorCode31900 => Some(r#"Unknown"#),
            TwilioProgrammableVoiceError::ErrorCode32400 => Some(r#"Twilio encountered errors communicating with your SIP communications infrastructure."#),
            TwilioProgrammableVoiceError::ErrorCode16107 => None,
            TwilioProgrammableVoiceError::ErrorCode31403 => None,
            TwilioProgrammableVoiceError::ErrorCode32223 => Some(r#"* Attempting to place a call from Twilio back to your own Twilio SIP domain. Twilio does not allow SIP calls between Twilio domains either in the same account or different accounts.
* Attempting to call a registered SIP AOR without specifying the username you wanted to call"#),
            TwilioProgrammableVoiceError::ErrorCode13513 => Some(r#"Say rate value is invalid."#),
            TwilioProgrammableVoiceError::ErrorCode32600 => Some(r#"* Connector configuration is missing required properties
* Connector configuration contains invalid properties"#),
            TwilioProgrammableVoiceError::ErrorCode31931 => Some(r#"* More than 10 minutes of audio is buffered by the Streamer. No more data is accepted."#),
            TwilioProgrammableVoiceError::ErrorCode31924 => Some(r#"* The WebSocket control frame was fragmented"#),
            TwilioProgrammableVoiceError::ErrorCode16021 => None,
            TwilioProgrammableVoiceError::ErrorCode31100 => None,
            TwilioProgrammableVoiceError::ErrorCode31201 => None,
            TwilioProgrammableVoiceError::ErrorCode16000 => None,
            TwilioProgrammableVoiceError::ErrorCode13219 => Some(r#"answerOnBridge value is not "true" or "false"."#),
            TwilioProgrammableVoiceError::ErrorCode14207 => Some(r#"A full queue can indicate higher <Enqueue> volume, longer queue times, or an incorrectly configured queue’s maxSize."#),
            TwilioProgrammableVoiceError::ErrorCode31101 => None,
            TwilioProgrammableVoiceError::ErrorCode13512 => Some(r#"Your Gather TwiML has an invalid language attribute value. (Example: `en` is an invalid value, but `en-US` is a valid value.) "#),
            TwilioProgrammableVoiceError::ErrorCode31007 => None,
            TwilioProgrammableVoiceError::ErrorCode31503 => Some(r#"* The server is under maintenance.
* The Application SID provided in the access token points to an inaccessible URL."#),
            TwilioProgrammableVoiceError::ErrorCode13620 => Some(r#"Transcription feature not supported for recordings when your account has one or more of the following features enabled: Recording Encryption, PCI recordings."#),
            TwilioProgrammableVoiceError::ErrorCode31001 => None,
            TwilioProgrammableVoiceError::ErrorCode13337 => Some(r#"Gather URL must use HTTPS. "#),
            TwilioProgrammableVoiceError::ErrorCode13258 => Some(r#"Dial->Sim not supported in this realm"#),
            TwilioProgrammableVoiceError::ErrorCode32501 => Some(r#"- Invalid attributes within the <Conversation> noun.
- Can be caused by any invalid syntax within the <Conversation> noun."#),
            TwilioProgrammableVoiceError::ErrorCode13256 => Some(r#"The recording status callback URL you provided is not valid."#),
            TwilioProgrammableVoiceError::ErrorCode13214 => Some(r#"Carriers may send invalid Caller IDs on Incoming calls to Twilio. If a specific Caller ID is not explicitly defined on the <Dial> verb, the invalid Caller ID will be passed to the destination. This may cause some destination providers to reject the call, and Twilio will mark it as failed."#),
            TwilioProgrammableVoiceError::ErrorCode16111 => Some(r#"* Invalid S3 bucket URL
* Invalid AWS S3 bucket credentials
* AWS S3 bucket credentials not found in Credential Storage"#),
            TwilioProgrammableVoiceError::ErrorCode32655 => Some(r#"* Some internal error occurred while trying to establish connection with the configured Intelligence service "#),
            TwilioProgrammableVoiceError::ErrorCode16001 => None,
            TwilioProgrammableVoiceError::ErrorCode13622 => Some(r#"Invalid combinations of recordingTrack with requested channels"#),
            TwilioProgrammableVoiceError::ErrorCode13334 => Some(r#"Invalid model"#),
            TwilioProgrammableVoiceError::ErrorCode31401 => Some(r#"* The user denied the getUserMedia request.
* The browser denied the getUserMedia request.
* The application has not been configured with the proper permissions."#),
            TwilioProgrammableVoiceError::ErrorCode13225 => Some(r#"The outbound call to the destination number was blocked by Twilio. Potential causes may be:
- The destination has a high-risk of fraud
- Due to regulatory reasons, the destination cannot be reached
- You are placing a call to a +1 destination and your account is missing a Primary Customer Profile"#),
            TwilioProgrammableVoiceError::ErrorCode22001 => Some(r#"The call timed out before system could complete the call"#),
            TwilioProgrammableVoiceError::ErrorCode31942 => Some(r#"* When using Virtual Agent make sure the Project ID, Conversation Profile ID and language configurations are correct."#),
            TwilioProgrammableVoiceError::ErrorCode31481 => None,
            TwilioProgrammableVoiceError::ErrorCode32502 => Some(r#"- Twilio failed to execute callback request when Voice Conversation service attempted to reach out to status Callback service. "#),
            TwilioProgrammableVoiceError::ErrorCode31006 => None,
            TwilioProgrammableVoiceError::ErrorCode13699 => Some(r#"Invalid recording trim value."#),
            TwilioProgrammableVoiceError::ErrorCode31921 => Some(r#"* The remote server ended the connection with a termination code listed in https://tools.ietf.org/html/rfc6455#section-7.4.1
* The Media Stream has received a websocket close event defined by https://developer.mozilla.org/en-US/docs/Web/API/CloseEvent"#),
            TwilioProgrammableVoiceError::ErrorCode32018 => Some(r#"Twiml size exceeded maximum allowed value of 4000 characters"#),
            TwilioProgrammableVoiceError::ErrorCode64003 => Some(r#"* The chargeAmount attribute has a number of decimals that does not conform to the currency type. For example, more than 2 decimals were provided for US Dollars .
* chargeAmount attribute is negative."#),
            TwilioProgrammableVoiceError::ErrorCode64109 => Some(r#"The allowed concurrencylimit on your account was reached."#),
            TwilioProgrammableVoiceError::ErrorCode16027 => Some(r#"The participant to be whispered is on hold."#),
            TwilioProgrammableVoiceError::ErrorCode31484 => Some(r#"* The outbound call was made with a phone number that has an invalid format."#),
            TwilioProgrammableVoiceError::ErrorCode31210 => Some(r#"The Message Type used for sending Call Message Event is invalid."#),
            TwilioProgrammableVoiceError::ErrorCode16112 => Some(r#"* AWS credentials does not have enough permissions to upload to the bucket
* The external S3 bucket has SSE-S3 enabled, only SSE-KMS is supported"#),
            TwilioProgrammableVoiceError::ErrorCode13216 => Some(r#"timeLimit must be a positive integer, in seconds."#),
            TwilioProgrammableVoiceError::ErrorCode14218 => Some(r#"Unable to update the external user queue"#)
        }
    }
}
