// AUTO-GENERATED. DO NOT MODIFY. YOUR CHANGES WILL BE OVER-WRITTEN
use crate::products::TwilioProgrammableVoiceError;
use standard_error::traits::StandardErrorDocsTrait;

impl StandardErrorDocsTrait for TwilioProgrammableVoiceError {
    fn docs(&self) -> &'static str {
        match self {
            TwilioProgrammableVoiceError::ErrorCode11750 => r#"## ERROR - 11750

### TwiML response body too large

In your response to Twilio’s request, the response body is larger than 64 kB. ## Error - 11750

## TwiML response body too large

In your response to Twilio's request, the response body is larger than 64 kB.

#### Possible Causes
*   The TwiML that you are serving is larger than 64 kB
*   You are serving non-TwiML content in your response; e.g. error message output

#### Possible Solutions
*  Verify that you are serving TwiML in your response to Twilio's request. 
*  Verify that you are including this header in your TwiML response: `<?xml version="1.0" encoding="UTF-8"?>`  
*  Limit your TwiML response to 64 kB or less.
*  Does your TwiML include the [Play verb](https://www.twilio.com/docs/voice/twiml/play)? Double-check the encoding and MIME type to ensure they are supported.
*  Check that your TwiML is formatted properly. To quickly verify your TwiML, you can copy and paste it into a new [TwiML bin in the Twilio Console](https://www.twilio.com/console/runtime/twiml-bins/create).
*   Check to see if your application is throwing errors. This may cause your application to send a large debug output back to Twilio instead of the expected TwiML.
*  If you are trying to send a `200` response in a status callback, use an empty TwiML response: `<Response/>`"#,
            TwilioProgrammableVoiceError::ErrorCode16026 => r#"## ERROR - 16026

### Participant label is in use by another participant

 The participant label specified in an API call to create a voice conference participant is already in use by another active participant in the same conference.

#### Possible Causes
1. The specified label was already used in another API call to add a participant to the conference.
2. The specified label was already used in twiml (Dial Conference) to add an inbound participant to the conference

#### Possible Solutions
Use a label value which uniquely identifies this participant within the conference."#,
            TwilioProgrammableVoiceError::ErrorCode13802 => r#"## ERROR - 13802

### Dial: No referUrl attribute specified

Twilio received a REFER on an established <Dial> session, but there was no referUrl attribute specified. In order to send a SIP REFER message to an established `<Dial>` call, the `referUrl` attribute must be specified in the TwiML. In this case, a REFER was received on a `<Dial>` call without a `referUrl` attribute present.

#### Possible Causes
* Your `<Dial>` verb did not include a `referUrl` attribute or the attribute was misspelled

#### Possible Solutions
* Add a valid `referUrl` attribute to your `<Dial>` verb
* For more information on sending REFER to Twilio, see [Refer to Twilio](https://www.twilio.com/docs/voice/api/refer-to-twilio)"#,
            TwilioProgrammableVoiceError::ErrorCode16002 => r#"## Error - 16002

### Failed to validate conference attributes

The provided conference attributes are invalid."#,
            TwilioProgrammableVoiceError::ErrorCode16003 => r#"## Error - 16003

### Could not recognize conference sid or friendly name

Twilio could not recognize the provided conference sid or friendly name.  Please see https://www.twilio.com/docs/voice/twiml/conference for more documentation."#,
            TwilioProgrammableVoiceError::ErrorCode64106 => r#"## ERROR - 64106

### ConversationRelay: Invalid argument

During media setup an error was encountered due to invalid argument During media setup an error was encountered due to invalid argument.

#### Possible Causes
* One of the parameters in to ConversationRelay is not supported by the chosen provider
See message for more details

#### Possible Solutions
* Make sure the language and voice combinations are supported from the chosen provider."#,
            TwilioProgrammableVoiceError::ErrorCode64017 => r#"## ERROR - 64017

### Pay: BankAccountType Parameter not supported with PaymentMethod = "credit-card"

 A 64017 Error is an indication there is an unexpected value for the BankAccountNumber parameter.

#### Possible Causes
* PaymentMethod selected is "credit-card" and a value is provided for BankAccountNumber Parameter

#### Possible Solutions
* Have PaymentMethod as "ach-debit" when capturing ACH payments which require a BankAccountNumber
* Do not provide BankAccountNumber when collecting a credit card payment"#,
            TwilioProgrammableVoiceError::ErrorCode32702 => r#"## WARNING - 32702

### Voice User-Defined Message: Invalid Content

 Invalid content in the POST request to the UserDefinedMessage Resource. Only data in JSON format is supported.

#### Possible Causes
* Content data in the POST request to the UserDefinedMessage Resource is not as expected, or there is no content in the request.

#### Possible Solutions
* Make sure to include content data in JSON format in the POST request to the UserDefinedMessage Resource."#,
            TwilioProgrammableVoiceError::ErrorCode32652 => r#"## WARNING - 32652

### Real-time Transcriptions: Unsupported <Config> attribute(s) in TwiML

 A 32652 Error is an indication that &lt;Transcription&gt; TwiML contains invalid or unsupported &lt;Config&gt; attributes. 

#### Possible Causes
* Some of the &lt;Config&gt; attribute(s) referenced in the &lt;Transcription&gt; TwiML are not supported by the Transcription provider

#### Possible Solutions
* Review the Debugger message in console and update or remove the offending attributes"#,
            TwilioProgrammableVoiceError::ErrorCode64009 => r#"## ERROR - 64009

### Pay: Twilio is no longer authorized to initiate transactions on your behalf.

 ## Error - 64009

### Pay: Twilio is no longer authorized to initiate transactions on your behalf.

A 64009 Error is an indication that the credentials provided in connector configuration no longer allow Twilio
to perform requests on behalf of your account.


#### Possible Causes
* The Twilio application had its privileges revoked from your account.

#### Possible Solutions
* Create a new connector configuration and run through the scenario where you authorize Twilio to make requests on your behalf again."#,
            TwilioProgrammableVoiceError::ErrorCode64016 => r#"## ERROR - 64016

### Pay: Invalid Action URL

 A 64016 Error is an indication there is something wrong with the value provided for the action parameter.

#### Possible Causes
* Action URL does not use https://

#### Possible Solutions
* Start your action URL with "https://""#,
            TwilioProgrammableVoiceError::ErrorCode37000 => r#"## ERROR - 37000

### WhatsApp Voice: Call permission not granted by consumer

 The business-initiated WhatsApp call failed because the consumer had not granted permission to initiate the call.

#### Possible Causes
The business-initiated WhatsApp call was placed to the consumer without the necessary call permission being accepted.

#### Possible Solutions
Ensure that call permission has been requested and accepted before initiating a call with the consumer."#,
            TwilioProgrammableVoiceError::ErrorCode13342 => r#"## ERROR - 13342

### Gather: Invalid config for Google STT V2 provider

 ## Warning - 13334

### Gather: Invalid configuration for Google STT V2 provider

#### Possible Causes
Invalid configuration for Google STT V2 provider

#### Possible Solutions
Enter valid configuration by referring Google STT V2 provider"#,
            TwilioProgrammableVoiceError::ErrorCode21302 => r#"## WARNING - 21302

### Approaching application creation limit 

 Approaching application creation limit

#### Possible Causes
You are approaching the limit of 1000 TwiML	 applications.

#### Possible Solutions
Delete or update existing applications instead of creating new applications."#,
            TwilioProgrammableVoiceError::ErrorCode31429 => r#"## Error - 31429

### Too Many Requests

Too many requests were sent in a given amount of time."#,
            TwilioProgrammableVoiceError::ErrorCode13801 => r#"## ERROR - 13801

### Refer not allowed on non-SIP call legs

The Refer verb can only be invoked on a SIP call leg. The Refer verb is only allowed on SIP call legs, e.g. you can refer an incoming SIP call that originate from SIP infrastructure or an outbound call to a SIP destination. Calls to or from other destinations, such as a phone number or Twilio Client cannot be referred.

#### Possible Causes
* Your application attempted to invoke the Refer verb on a PSTN or Twilio Client call leg.

#### Possible Solutions
* For more information on understanding what does and does not constitute a SIP call leg, please see [Understanding SIP Call Legs](https://www.twilio.com/docs/voice/twiml/refer#understanding-call-legs).
* While invoking Refer on a SIP leg is unique in that it removes Twilio from the call path, the effect of transferring a call to another party can be accomplished on all leg types by simply using the Dial verb to call the other party."#,
            TwilioProgrammableVoiceError::ErrorCode31904 => r#"## ERROR - 31904

### Stream - WebSocket - Host Unreachable

 The WebSocket connection cannot be created because the server is unreachable.

#### Possible Causes
* Wrong IP address or DNS name
* IP address belongs to the private domain and it is non-routable
* Host is down or disconnected from router

#### Possible Solutions
* Validate the IP address or DNS names are valid and belongs to the public domain
* Validate the host is running and properly connected to the Internet"#,
            TwilioProgrammableVoiceError::ErrorCode32653 => r#"## ERROR - 32653

### Real-time Transcriptions: Internal Error

 A 32653 Error is an indication that there is an internal server error and some internal services failed

#### Possible Causes
* Some error occurred during communication with Twilio internal services

#### Possible Solutions
* If this is a continuous experience, please contact Twilio Support"#,
            TwilioProgrammableVoiceError::ErrorCode64002 => r#"## ERROR - 64002

### Pay: Service unavailable.

 ## Error - 64002

### Pay: Service unavailable.

A 64002 Error is an indication that there is an internal server error and some internal services failed.

#### Possible Causes
* Some internal services in Twilio were not available.
* Payment provider not available.

#### Possible Solutions
* Try to retry the request.
*   If the error persists, please <a href="/help/contact">contact us</a> and we can help you resolve the issue. 
*   Note the time of the error and what you were trying to do when it occurred."#,
            TwilioProgrammableVoiceError::ErrorCode31922 => r#"## ERROR - 31922

### Stream - WebSocket - URL Schema Not Supported

The URL schema is not supported by Programmable Streams 

#### Possible Causes
* URL schema is not supported

#### Possible Solutions
* Verify the schema in TwiML URL is wss"#,
            TwilioProgrammableVoiceError::ErrorCode16020 => r#"## Error - 16020

### Conference is full

A participant has attempted to join a conference that has reached its maximum participant capacity.

Please refer to https://www.twilio.com/docs/voice/twiml/conference for documentation on conference participant capacity."#,
            TwilioProgrammableVoiceError::ErrorCode31901 => r#"## ERROR - 31901

### Stream - WebSocket - Connection Timeout

The WebSocket connection request sent by the stream didn't receive any response from the server 

#### Possible Causes
* WebSocket server is not connected to any network
* Bad IP address provided in the TwiML URL
* Intermediate elements (like ngrok) are not routing traffic
* There is a firewall blocking the traffic

#### Possible Solutions
* Verify server connectivity
* Verify the TwiML URL is correct
* Verify intermediate elements are routing traffic and firewall are not blocking ports"#,
            TwilioProgrammableVoiceError::ErrorCode31940 => r#"## ERROR - 31940

### Stream - Invalid connectorName attribute in TwiML.

connectorName attribute provided does not reference a connector configuration. Invalid connectorName attribute in TwiML.


#### Possible Causes
A 31940 Error is an indication that the value provided with connectorName attribute does not match to an installed configuration.


#### Possible Solutions
* Make sure that the connectorName matches a Unique Name in an installed connector."#,
            TwilioProgrammableVoiceError::ErrorCode17000 => r#"## ERROR - 17000

### Forbidden to access data

 This error code is returned when an account is trying to access a resource created by another account. Note: parent accounts can access subaccount resources, but subaccounts cannot access parent account resources, or other subaccounts under the same parent account. 

#### Possible Causes
This error code is also returned when an account is attempting to access a resource where Voice Insights Advanced Features has not been enabled on that account.

#### Possible Solutions
Once enabled future calls will be retrievable via API."#,
            TwilioProgrammableVoiceError::ErrorCode13328 => r#"## Warning - 13328 

### Gather: Invalid maxSpeechTime value"#,
            TwilioProgrammableVoiceError::ErrorCode13332 => r#"## Warning - 13332

### Gather: Invalid bargeIn value"#,
            TwilioProgrammableVoiceError::ErrorCode64011 => r#"## Error - 64011

### Pay: Connector does not support the requested currency.

A 64011 Error is an indication that the value provided in the currency attribute is not supported by this &lt;Pay&gt; Connector.

#### Possible Causes
* currency is an invalid currency code.
* currency is not supported by the &lt;Pay&gt; Connector.
"#,
            TwilioProgrammableVoiceError::ErrorCode64005 => r#"## ERROR - 64005

### Pay: Connector does not support tokenization.

 ## Error - 64005

### Pay: Connector does not support tokenization.

A 64005 Error is an indication that the request to tokenize credit card failed because the connector does not support tokenization.

#### Possible Causes
* The paymentConnector attribute points to Connector that does not support tokenization.
* The chargeAmount attribute in the Pay Verb TwiML was 0 (or omitted) and you intended to charge.

#### Possible Solutions
* Make sure that the paymentConnector matches a &lt;Pay&gt; Connector that supports tokenization.
* If a charge was intended, please change the chargeAmount attribute and provided a positive value. Please see <A HREF="/docs/voice/twiml/pay#chargeamount">this API documentation</A> for more details."#,
            TwilioProgrammableVoiceError::ErrorCode13335 => r#"## WARNING - 13335

### Gather: speechTimeout auto cannot be used with model default

 Gather: speechTimeout auto cannot be used with model default

#### Possible Causes
speechTimeout value invalid

#### Possible Solutions
speechTimeout value change"#,
            TwilioProgrammableVoiceError::ErrorCode31952 => r#"## ERROR - 31952

### Stream Extension not found: 

 Stream Extension not found

#### Possible Causes
Stream Extension not found

#### Possible Solutions
Try using stream extension in another realm or checking that you are using a valid stream extension."#,
            TwilioProgrammableVoiceError::ErrorCode16099 => r#"## Error - 16099

### Unexpected conference status

The requested conference is in an unexpected state for the desired operation."#,
            TwilioProgrammableVoiceError::ErrorCode32701 => r#"## WARNING - 32701

### Voice User-Defined Message: Invalid Content-Type

 Invalid Content-Type in the POST request to the UserDefinedMessage Resource. Only `application/json` is supported.

#### Possible Causes
* Value of Content-Type in the POST request to the UserDefinedMessage Resource is not as expected, or there is no Content-Type header in the request.

#### Possible Solutions
* Make sure that Content-Type Header is included in the POST request to the UserDefinedMessage Resource and the value of the Content-Type Header is `application/json`."#,
            TwilioProgrammableVoiceError::ErrorCode13330 => r#"## Warning - 13330

### Gather: Invalid hints value"#,
            TwilioProgrammableVoiceError::ErrorCode31409 => r#"## Error - 31409

### Conflict

The request could not be processed because of a conflict in the current state of the resource. Another request may be in progress."#,
            TwilioProgrammableVoiceError::ErrorCode16102 => r#"## Warning - 16102

### Voice Recording: Unavailable because recording is silent

Recording was detected as silent and not stored.  As a result, it is unavailable for access."#,
            TwilioProgrammableVoiceError::ErrorCode31009 => r#"## Error - 31009

### Transport error

No transport available to send or receive messages."#,
            TwilioProgrammableVoiceError::ErrorCode16101 => r#"## Warning - 16101

### Voice Recording : Unavailable because duration is too short

Recording is unavailable because the duration was shorter than the minimum required duration."#,
            TwilioProgrammableVoiceError::ErrorCode15004 => r#"## ERROR - 15004

### Action Callback URL must be an absolute URL when using TwiML to update in-progress calls

Use absolute URLs when updating in-progress calls with TwiML The TwiML provided for updating the in-progress call must use absolute URL for action callback.

#### Possible Causes
A relative URL is provided for action callbacks when updating in-progress call with TwiML.

#### Possible Solutions
Provide an absolute URL for the action attribute when updating in-progress call with TwiML."#,
            TwilioProgrammableVoiceError::ErrorCode13804 => r#"## ERROR - 13804

### AddOns are not supported in this realm

 AddOns are not supported in this realm

#### Possible Causes
AddOns are not supported in this realm

#### Possible Solutions
use AddOns in another realm"#,
            TwilioProgrammableVoiceError::ErrorCode16106 => r#"## Error - 16106

### Voice Recording: Unavailable due to internal encryption error

Recording was not stored due to a transient internal encryption error  (only applicable if call recording encryption feature enabled)"#,
            TwilioProgrammableVoiceError::ErrorCode31103 => r#"## Error - 31103

### Length of parameters cannot exceed MAX_PARAM_LENGTH.

Length of parameters cannot exceed MAX_PARAM_LENGTH."#,
            TwilioProgrammableVoiceError::ErrorCode32019 => r#"## WARNING - 32019

### Twiml and Voice URL are both set. Using Voice URL.

 Twiml and Voice URL are both set. Using Voice URL.

#### Possible Causes
Both Voice URL and embedded twiml are provided in the request

#### Possible Solutions
Use only Voice URL or embedded twiml, not both, in the request"#,
            TwilioProgrammableVoiceError::ErrorCode21263 => r#"## WARNING - 21263

### Invalid Answering Machine Detection Parameters

Answering Machine Detection parameters specific to POST to /Calls API have been passed in POST to /Participants, <Dial><Number>, or <Dial><Sip> Invalid Answering Machine Detection Parameters

#### Possible Causes
AsyncAmd, AsyncAmdStatusCallback, and/or AsyncAmdStatusCallbackMethod parameters from POST to /Calls API have been passed in a POST to /Participants API, or added to <Dial><Number> or <Dial><Sip> TwiML.

#### Possible Solutions
Do not include AsyncAmd, AsyncAmdStatusCallback, or AsyncAmdStatusCallbackMethod parameters in POST requests made to /Participants API and in TwiML responses that include <Dial><Number> or <Dial><Sip> with Answering Machine Detection enabled."#,
            TwilioProgrammableVoiceError::ErrorCode31502 => r#"## Error - 31502

### Bad Gateway 

The server is acting as a gateway or proxy, and received an invalid response from a downstream server while attempting to fulfill the request."#,
            TwilioProgrammableVoiceError::ErrorCode31206 => r#"## Error - 31206

### Rate exceeded authorized limit.

Rate exceeded authorized limit."#,
            TwilioProgrammableVoiceError::ErrorCode16011 => r#"## Error - 16011

### Conference Event: Error Response to Callback URL

Twilio received an error response when attempting to send a conference event to the provided callback URL."#,
            TwilioProgrammableVoiceError::ErrorCode64023 => r#"## ERROR - 64023

### Pay: Invalid Test Bank Account Number

 A 64023 error is an indication that your Pay Connector is in TEST mode, but you did not provide a valid test bank account number when testing an ACH payment.

#### Possible Causes
* Your pay connector is in TEST mode and you provided an invalid test bank account number when testing an ACH payment. For compliance reasons, we can only allow certain test bank account numbers in TEST mode.

#### Possible Solutions
* Check the documentation for the pay connector in the Twilio Console or in our Twilio docs for valid bank account numbers that can be used when in TEST mode."#,
            TwilioProgrammableVoiceError::ErrorCode31408 => r#"## Error - 31408

### Request Timeout

A request timeout occurred."#,
            TwilioProgrammableVoiceError::ErrorCode64018 => r#"## ERROR - 64018

### Pay: Value needed for either Capture or Status parameters

 A 64018 Error is an indication that it is not clear whether to capture a piece of payment information or to complete/cancel the transaction.

#### Possible Causes
* Parameter value not provided for both Capture and Status 
* Parameter value provided for both Capture and Status

#### Possible Solutions
* Ensure either Capture or Status parameters have a value but not both"#,
            TwilioProgrammableVoiceError::ErrorCode53404 => r#"## Error - 53404

### No supported codec

Raised whenever the intersection of codecs supported by the Client and the Server (or, in peer-to-peer, the Client and another Participant) is empty."#,
            TwilioProgrammableVoiceError::ErrorCode16023 => r#"## ERROR - 16023

### Dial->Conference: Invalid participant label, must not exceed 128 characters, must not be a CallSid, must not contain '/'

 The participant label specified in an API call to create a voice conference participant was invalid. It may not exceed 128 characters, it may not be a CallSid, and it may not contain a ‘/’ (even in URL encoded form %2F).

#### Possible Causes
A POST to conference participants call to the Twilio API specified a participant label which is greater than 128 characters or which resembles a CallSid (“CA” followed by 32 lowercase hexadecimal characters), or contains the ‘/’ character or URL encoded ‘/’ character (%2F).

#### Possible Solutions
Shorten the length of the ParticipantLabel parameter, avoid using a call SID and avoid including ‘/’ characters."#,
            TwilioProgrammableVoiceError::ErrorCode31209 => r#"## ERROR - 31209

### Reconnect attempt error.

 Reconnect attempt is not authorized.

#### Possible Causes
The new identity for the reconnection and the old identity for the original call does not match.

#### Possible Solutions
Please use same identity as the original call for the reconnection attempt."#,
            TwilioProgrammableVoiceError::ErrorCode13805 => r#"## WARNING - 13805

### Trial account call duration exceeded 10 minute limit

 Trial account call duration exceeded 10 minute limit

#### Possible Causes
Account is a trial account

#### Possible Solutions
Upgrade trial account"#,
            TwilioProgrammableVoiceError::ErrorCode16022 => r#"## Error - 16022

### Conference does not exist or is completed

The requested conference sid or friendly name refers to a non-existent or completed conference."#,
            TwilioProgrammableVoiceError::ErrorCode64010 => r#"## Error - 64010

### Pay: Payment Gateway rejected token creation.

A 64010 Error is an indication that the request to create token was rejected by the Payment Gateway. 

#### Possible Solutions
* Please see the data returned in the with webhook to action url for more details.  
* Reach out to your Payment Gateway support for more assistance.
"#,
            TwilioProgrammableVoiceError::ErrorCode16025 => r#"## ERROR - 16025

### Dial->Conference: Participant label is in use by another participant

 The participant label specified as an attribute on the conference noun in twiml is already in use by another active participant in the same conference.

#### Possible Causes
1. The same static twiml specifying a participant label was served to two difference calls
2. Dynamically generated twiml specified the same participant label for two different calls
3. The participant label specified in the twiml Is the same as one specified for an outbound participant that was already added to the conference using the Twilio API 

#### Possible Solutions
Use a label value which uniquely identifies this participant within the conference"#,
            TwilioProgrammableVoiceError::ErrorCode13331 => r#"## Warning - 13331

### Gather: Invalid language value"#,
            TwilioProgrammableVoiceError::ErrorCode31402 => r#"## ERROR - 31402

### UserMedia Acquisition Failed

 The browser and end-user allowed permissions, however getting the media failed. Usually this is due to bad constraints, but can sometimes fail due to browser, OS or hardware issues.

#### Possible Causes
* NotFoundError - The deviceID specified was not found.
* The getUserMedia constraints were overconstrained and no devices matched.

#### Possible Solutions
* Ensure the deviceID being specified exists.
* Try acquiring media with fewer constraints."#,
            TwilioProgrammableVoiceError::ErrorCode64019 => r#"## ERROR - 64019

### Pay: Required payment information incomplete

 A 64019 Error is an indication that the pieces of payment information indicated as required were not captured.

#### Possible Causes
* Postal code and Security code were marked as required but not captured during the payment flow
* BankAccountNumber and BankRoutingNumber were not captured
* Credit Card Number and Expiration Date were not captured
* Status parameter value was set to "complete" before all payment information was captured

#### Possible Solutions
* Set Status parameter to "complete" only after all payment information is received via the Capture parameter"#,
            TwilioProgrammableVoiceError::ErrorCode17009 => r#"## ERROR - 17009

### Internal Server Error

 The requested resource failed to complete the request. 

#### Possible Causes
The requested resource failed to complete the request. 

#### Possible Solutions
The request should be retried using exponential backoff strategy until it succeeds."#,
            TwilioProgrammableVoiceError::ErrorCode31211 => r#"## ERROR - 31211

### Call is not in the expected state.

 Call is not in the expected state.

#### Possible Causes
The Call should be at least in the Ringing state to Subscribe and send Call Message.

#### Possible Solutions
Ensure the Call is at least in the Ringing state and the Subscription is successful and try again."#,
            TwilioProgrammableVoiceError::ErrorCode64006 => r#"## ERROR - 64006

### Pay: Connector does not support token type.

 ## Error - 64006

### Pay: Connector does not support token type.

 Error is an indication that the request to tokenize credit card failed because the connector does not support tokenization.

A 64006 Error is an indication that the request to tokenize of a particular type is not supported by the &lt;Pay&gt; connector.  For instance, tokenType="one-time" was used but
only tokenType="reusable" is supported.

#### Possible Causes
* The paymentConnector attribute points to a Connector that does not support the requested type of tokenization.

#### Possible Solutions
* Make sure that the paymentConnector matches a &lt;Pay&gt; Connector that supports the tokenization type you want.
* Make sure you specify the tokenType that matches what you want to perform."#,
            TwilioProgrammableVoiceError::ErrorCode32021 => r#"## WARNING - 32021

### SHAKEN/STIR call verification failed

 Twilio cannot verify incoming SHAKEN PASSporT from carrier

#### Possible Causes
See the error message for details

#### Possible Solutions
No action required. Incoming call to your Twilio number can still be established without verstat."#,
            TwilioProgrammableVoiceError::ErrorCode31604 => r#"## Error - 31604

### Does Not Exist Anywhere

The requested callee does not exist anywhere."#,
            TwilioProgrammableVoiceError::ErrorCode32500 => r#"## ERROR - 32500

### Voice Conversation: Generic error.

 Request through Voice Conversation Service failed.

#### Possible Causes
- Some error occurred during communication with Twilio internal services.

#### Possible Solutions
- See the Debugger in Console for further information. If this is a continuous experience, please contact Twilio Support."#,
            TwilioProgrammableVoiceError::ErrorCode32101 => r#"## WARNING - 32101

### SIP: Invalid phone number

The called number is not a valid E.164 number. Make sure that any phone number sent via SIP to Twilio is always E.164-formatted. SIP: Invalid phone number


#### Possible Causes
The called number is not a valid [E.164](https://www.twilio.com/docs/glossary/what-e164) number.

#### Possible Solutions
Make sure that any phone number sent via SIP to Twilio is always E.164-formatted."#,
            TwilioProgrammableVoiceError::ErrorCode64014 => r#"## ERROR - 64014

### Pay: ECP/ACH requires AVSName Parameter in the <Pay> verb.

 A 64014 Error is an indication that the provided paymentMethod attribute "ach-debit" was used, but the AVSName Parameter was not found inside the <Pay> verb.

#### Possible Causes
Electronic Check Processing requires a name.

#### Possible Solutions
Make sure a "AVSName" Parameter element exists inside the <Pay> verb.

Example:
```
<Pay paymentMethod="ach-debit" ...>
  <Parameter name="AVSName" value="john smith" />
</Pay
```
"#,
            TwilioProgrammableVoiceError::ErrorCode17001 => r#"## ERROR - 17001

### Completed summary for this call wasn't found

 This error code is returned by the /Summary API when a completed summary is requested and it doesn't exist.

#### Possible Causes
The call SID for the call is valid, but the call summarization process was not complete at the time of the request. Summarization takes 30 minutes to complete.

#### Possible Solutions
To see results immediately, use the ProcessingState=partial query parameter, which will return a 200, but the response will contain only a partial or incomplete information until the summarization is completed. "#,
            TwilioProgrammableVoiceError::ErrorCode32009 => r#"## Error - 32009

### Dialing SIP Endpoint failure - User not registered
There was a failure attempting to Dial the specified SIP Endpoint.
The User specified is not registered.

#### Possible Causes 

You tried to Dial a SIP Endpoint that is not currently registered with the corresponding SIP Domain. Please check your TwiML or REST API.

#### Possible Solutions

* If you are using TwiML
Please check that there isn't a typo in  &lt;Dial&gt;&lt;Sip&gt;<b>username</b>@yoursipdomain.sip.us1.twilio.com&lt;/Sip&gt;&lt;/Dial&gt; and compare against your SIP Endpoint configuration. 
Make sure the <b>username</b> matches a username in the Credential List used to authenticate the SIP Endpoint with the SIP Domain. 

* If you are using the REST API
Please check the <b>To</b> field and verify there isn't a typo. Make sure the <b>username</b> matches a username in the Credential List used to authenticate the SIP Endpoint with the SIP Domain.

#### Troubleshooting
* Prior to dialing, you can verify that your SIP Endpoint has successfully registered in the Console "Registered SIP Endpoints" tab found on the SIP Domains page."#,
            TwilioProgrammableVoiceError::ErrorCode64004 => r#"## Error - 64004

### Pay: Invalid paymentConnector attribute in TwiML.

A 64004 Error is an indication that the value provided with paymentConnector attribute of Pay verb
does not match to a Pay Connector configuration. See the <A HREF="/docs/voice/twiml/pay#paymentconnector">Pay Verb</A> API Reference.

#### Possible Solutions
* Make sure that the paymentConnector matches a Unique Name in the &lt;Pay&gt; Connector. Go to <A HREF="/console/voice/pay-connectors">this page</A> for a list of your Connectors."#,
            TwilioProgrammableVoiceError::ErrorCode64022 => r#"## ERROR - 64022

### Pay: Invalid Test Card Number

 A 64022 error is an indication that your Pay Connector is in TEST mode, but you did not provide a valid test card number when testing a payment.

#### Possible Causes
* Your pay connector is in TEST mode and you provided an invalid test card number when testing a payment. For compliance reasons, we can only allow certain test card numbers in TEST mode.

#### Possible Solutions
* Check the documentation for the pay connector in the Twilio Console or in our Twilio docs for valid test card numbers that can be used when in TEST mode."#,
            TwilioProgrammableVoiceError::ErrorCode31002 => r#"## Error - 31002

### Connection declined.

Connection declined."#,
            TwilioProgrammableVoiceError::ErrorCode13247 => r#"## ERROR - 13247

### Dial: Invalid From number (caller ID)

 You attempted to initiate an outbound phone call or message, but the 'From' parameter you supplied was not a valid phone number or alphanumeric sender ID.

Twilio accepts phone numbers in [E.164 format](https://www.twilio.com/docs/glossary/what-e164) (i.e. "+1 format"), 10-digit US and Canadian numbers with any combination of non-digit separators, or Alphanumeric Sender IDs (SMS only) with up to 11 alphanumeric characters [a-zA-Z0-9]. Refer to [this page](https://support.twilio.com/hc/en-us/articles/223133867-Using-Alphanumeric-Sender-ID-with-Messaging-Services) for acceptable characters.

The number must not be on a do-not-originate (DNO) list, and Alpha Sender IDs may not be generic.

#### Possible Causes
* You have supplied a phone number that was not in [E.164 format](https://www.twilio.com/docs/glossary/what-e164)
* Your `From` phone number is on a do-not-originate (DNO) list

#### Possible Solutions
* Ensure your number is formatted in [E.164 format](https://www.twilio.com/docs/glossary/what-e164)
* Ensure that your `From` number is assigned and not on a do-not-originate (DNO) list."#,
            TwilioProgrammableVoiceError::ErrorCode13257 => r#"## ERROR - 13257

### Invalid transcribeCallback URL

 Transcribe callback URL does not conform to standard URL format.

#### Possible Causes
Transcribe callback URL is invalid.

#### Possible Solutions
Correct transcribe callback URL."#,
            TwilioProgrammableVoiceError::ErrorCode31202 => r#"## ERROR - 31202

### Signature validation failed.

 The provided access token failed signature validation.

#### Possible Causes
The access token has an invalid Account SID, API Key, or API Key Secret.

#### Possible Solutions
Ensure the Account SID, API Key, and API Key Secret are valid when generating your access token."#,
            TwilioProgrammableVoiceError::ErrorCode31212 => r#"## ERROR - 31212

### Call Message Event Payload size exceeded authorized limit.

 Call Message Event Payload size exceeded authorized limit.

#### Possible Causes
The payload size of Call Message Event exceeds the authorized limit.

#### Possible Solutions
Reduce payload size of Call Message Event to be within the authorized limit and try again."#,
            TwilioProgrammableVoiceError::ErrorCode32503 => r#"## WARNING - 32503

### Voice Conversation: Callback event response error.

 Request to status callback URL received a response error from the customer-side server.

#### Possible Causes
- This error indicates that Twilio was able to reach the customer-provided URL, however, received an error from the customer-side callback endpoint.
- The URL provided was likely valid, but Twilio might be receiving an error from the customer-side service.

#### Possible Solutions
- Make sure you have a web server up and running that serves that URL with the configured method in TwiML, e.g. POST."#,
            TwilioProgrammableVoiceError::ErrorCode13329 => r#"## Warning - 13329

### Gather: Invalid partialResultCallbackMethod value"#,
            TwilioProgrammableVoiceError::ErrorCode64105 => r#"## ERROR - 64105

### ConversationRelay: Websocket ended

Websocket was disconnected during an ongoing call The websocket disconnected from the server side during an ongoing call

#### Possible Causes
* The websocket server at the url is no longer healthy
* Possible software problem in the websocket server (not Twilio)
* Network issues

#### Possible Solutions
* Make sure the websocket server at the url is healthy
* Investigate logs on the websocket server to see why the disconnect happened"#,
            TwilioProgrammableVoiceError::ErrorCode32601 => r#"## ERROR - 32601

### Virtual Agent: Provider Error

 A 32601 Error is an indication that the &lt;VirtualAgent&gt; connector configuration was rejected by the VirtualAgent provider.

#### Possible Causes
* Connector configuration contains invalid properties


#### Possible Solutions
* Check your &lt;VirtualAgent&gt; connector configuration
* Check any &lt;Config&gt; elements referenced in TwiML
* Check the VirtualAgentProviderError included in the webhook to the action url or the Debugger message in Console for further information. For additional assistance, please contact Twilio Support."#,
            TwilioProgrammableVoiceError::ErrorCode64108 => r#"## ERROR - 64108

### ConversationRelay: RTP Timeout

 An RTP timeout is detected, meaning that audio stopped flowing.

#### Possible Causes
* Phone call finished
* Carrier stopped sending audio to Twilio

#### Possible Solutions
* Contact Twilio support"#,
            TwilioProgrammableVoiceError::ErrorCode32606 => r#"## ERROR - 32606

### Virtual Agent: Resume Error

 A 32606 Error is an indication that the &lt;VirtualAgent&gt; TwiML contains an invalid or unauthorized configuration for resuming an existing conversation.

#### Possible Causes
* &lt;Config name="resumeEndUserId"&gt; element referenced in TwiML contains an invalid value
* &lt;Config name="resumeEndUserId"&gt; element referenced in TwiML is not authorized to resume the conversation from the current Call SID

#### Possible Solutions
* Check the value of the &lt;Config name="resumeEndUserId"&gt; element referenced in TwiML
* Ensure the resume attempt is made from the same Call SID which created the conversation
* Check the VirtualAgentProviderError included in the webhook to the action url or the Debugger message in Console for further information. For additional assistance, please contact Twilio Support."#,
            TwilioProgrammableVoiceError::ErrorCode31920 => r#"## ERROR - 31920

### Stream - WebSocket - Handshake Error

The WebSocket protocol handshake was declined by the server The server has returned an HTTP code different from 101 to the connection request sent by stream

See https://tools.ietf.org/html/rfc6455#section-4 for details.

#### Possible Causes
* The server does not support WebSocket
* The WebSocket protocol is not enabled for the requested URL

#### Possible Solutions
* Verify WebSocket protocol is enabled in the server
* Verify the path in TwiML URL is actually the one supporting WebSocket"#,
            TwilioProgrammableVoiceError::ErrorCode17007 => r#"## ERROR - 17007

### Voice Insights Advanced Features not enabled

 The requested resource's call_sid is valid, but it ended before the Voice Insights product was last enabled for the requester's account.

#### Possible Causes
Voice Insights Advanced Features not enabled on the account when the call was made.

#### Possible Solutions
Enable Voice Insights Advanced Features for API access to future calls."#,
            TwilioProgrammableVoiceError::ErrorCode13333 => r#"## WARNING - 13333

### Gather: Invalid profanityFilter value

 ## Warning - 13333

### Gather: Invalid profanityFilter value

#### Possible Causes
Profanity filter is invalid

#### Possible Solutions
Enter valid profanity filter"#,
            TwilioProgrammableVoiceError::ErrorCode31530 => r#"## Error - 31530

### DNS Resolution Error

Could not connect to the server."#,
            TwilioProgrammableVoiceError::ErrorCode13227 => r#"## ERROR - 13227

### Geo Permission configuration is not permitting call

 You attempted to initiate an outbound phone call to a phone number that is not enabled on your account.

#### Possible Causes
User dialed a destination your application is not enabled to support calling to.

#### Possible Solutions
Please check your [Voice Dialing Geographic Permissions](https://www.twilio.com/console/voice/calls/geo-permissions), fix it, and try again."#,
            TwilioProgrammableVoiceError::ErrorCode32007 => r#"The AOR refers to: username@yoursipdomain.com

There is a limit on the number of SIP Endpoints/bindings per AOR

<b>Possible Causes</b>

There is a limit of 10 bindings (SIP Endpoints) per AOR.

<b>Possible Solutions</b>
Reduce the number of SIP Endpoints"#,
            TwilioProgrammableVoiceError::ErrorCode31104 => r#"## Error - 31104

### Invalid bridge token.

Invalid bridge token."#,
            TwilioProgrammableVoiceError::ErrorCode31208 => r#"## Error - 31208

### User denied access to microphone.

User denied access to microphone."#,
            TwilioProgrammableVoiceError::ErrorCode31910 => r#"## ERROR - 31910

### Stream - WebSocket - SSL Protocol Error

The server doesn’t support SSL, can’t provide a secure connection or the SSL handshake has failed 

#### Possible Causes
* Server doesn’t support SSL
* Stream is trying to connect the wrong port
* Protocol or SSL handshake has failed

#### Possible Solutions
* Verify the server has enabled SSL
* Verify the port in TwiML URL is actually the one supporting SSL
* Verify the certificate is correct and accepted by HTTP clients"#,
            TwilioProgrammableVoiceError::ErrorCode31953 => r#"## WARNING - 31953

### Stream - Media - RTP timeout

 An RTP timeout is detected, meaning that inbound stream stopped flowing.

#### Possible Causes
* Phone call finished
* Carrier stopped sending audio to Twilio

#### Possible Solutions
* Contact Twilio support"#,
            TwilioProgrammableVoiceError::ErrorCode16024 => r#"## ERROR - 16024

### Invalid participant label, must not exceed 128 characters, must not be a CallSid, must not contain '/'

 The participant label specified in an API call to create a voice conference participant was invalid. It may not exceed 128 characters, it may not be a CallSid, and it may not contain a ‘/’ (even in URL encoded form %2F).

#### Possible Causes
A POST to conference participants call to the Twilio API specified a participant label which is greater than 128 characters or which resembles a CallSid (“CA” followed by 32 lowercase hexadecimal characters), or contains the ‘/’ character or URL encoded ‘/’ character (%2F).

#### Possible Solutions
Shorten the length of the ParticipantLabel parameter, avoid using a call SID and avoid including ‘/’ characters."#,
            TwilioProgrammableVoiceError::ErrorCode32650 => r#"## ERROR - 32650

### Real-time Transcriptions: Configuration Error

 A 32650 Error is an indication there is something wrong with the &lt;Transcription&gt; configuration.

#### Possible Causes
* Configuration is missing required properties
* Configuration contains invalid properties

#### Possible Solutions
* Check the TranscriptionError message included in the webhook or the Debugger message in Console for further information and update your &lt;Transcription&gt; configuration"#,
            TwilioProgrammableVoiceError::ErrorCode17008 => r#"## ERROR - 17008

### Internal Server Error - Query Timeout

 Query Timeout

#### Possible Causes
The requested resource exceeded the time limit for the data to be fetched from the underlying data store.

#### Possible Solutions
The request should be retried using exponential backoff strategy until it succeeds."#,
            TwilioProgrammableVoiceError::ErrorCode32106 => r#"Authentication error.

<b>Possible Causes</b>

Authorization name and username do not match.

<b>Possible Solutions</b>

Configure your SIP endpoint so the username and Authorization name match"#,
            TwilioProgrammableVoiceError::ErrorCode32014 => r#"## Warning - 32014

### Call is terminated because of no audio received

#### Possible Causes

* One party doesn't send audio for an extended period of time and call is terminated.

#### Possible Solutions

* It could be the case when one party is disconnected without hanging up due to network failure. Try to call again later.
* If this keeps happening, please contact Twilio support to enable voice trace. With voice trace enabled, Twilio can know which side stops sending audio to track down the root cause."#,
            TwilioProgrammableVoiceError::ErrorCode32651 => r#"## ERROR - 32651

### Real-time Transcriptions: Provider Error

 A 32651 Error is an indication that the &lt;Transcription&gt; provider configuration was rejected by the Speech-to-Text provider.


#### Possible Causes
* Provider configuration contains invalid or unsupported properties

#### Possible Solutions
* Check any provider specific configuration referenced in TwiML
* Check the TranscriptionError message included in the webhook or the Debugger message in Console for further information. For additional assistance, please contact Twilio Support."#,
            TwilioProgrammableVoiceError::ErrorCode13326 => r#"## Warning - 13326 

### Gather: Invalid input value"#,
            TwilioProgrammableVoiceError::ErrorCode31426 => r#"## Error - 31426

### Upgrade Required

This error is raised when an HTTP 426 response is received. The reason for this is most likely because of an incompatible TLS version. To mitigate this, you may need to upgrade the OS or download a more recent version of the SDK."#,
            TwilioProgrammableVoiceError::ErrorCode13750 => r#"## ERROR - 13750

### Twiml verb not supported in this version of the API
Your version of the API does not support this feature. Please use version 2010-04-01 or later."#,
            TwilioProgrammableVoiceError::ErrorCode13240 => r#"## Error - 13240

### Dial->Conference: Invalid Whisper SID

The call SID specified by the whisper parameter is invalid. This may because the specified call is no longer connected to the conference. Validate that this call is connecting to the correct conference and the call SID specified is also in the conference."#,
            TwilioProgrammableVoiceError::ErrorCode16110 => r#"## ERROR - 16110

### Internal failure when bulk deleting recordings from your account

 Despite our best efforts, we have failed to delete some or all of the selected recordings. The operation has been cancelled.

#### Possible Causes
There was an internal failure when processing your request. This action will not be retried.

#### Possible Solutions
* Please retry the delete operation with the same parameters.
* If the error persists, please <a href="/help/contact">contact us</a> to figure out what has happened and how to fix it!
* Note the time of the error and which filter was used to select the recordings."#,
            TwilioProgrammableVoiceError::ErrorCode13220 => r#"## WARNING - 13220

### Dial: Invalid ringTone value

 

#### Possible Causes
Dial: Invalid ringTone value

#### Possible Solutions
Provide valid ringTone value"#,
            TwilioProgrammableVoiceError::ErrorCode21301 => r#"## ERROR - 21301

### Cannot create application: application limit exceeded

 Cannot create application: application limit exceeded 

#### Possible Causes
You have attempted to create a new application that exceeds the limit of 1000 applications.

#### Possible Solutions
Delete or update application in order to create new application."#,
            TwilioProgrammableVoiceError::ErrorCode31504 => r#"## Error - 31504

### Gateway Timeout 

The server, while acting as a gateway or proxy, did not receive a timely response from an upstream server."#,
            TwilioProgrammableVoiceError::ErrorCode31930 => r#"## WARNING - 31930

### Stream - Media - Buffer Overflow

Audio is discarded due to a congestion control condition (restricted bandwidth) in the WebSocket upstream 

#### Possible Causes
* The server has an upstream bandwidth restriction

#### Possible Solutions
* Verify network load"#,
            TwilioProgrammableVoiceError::ErrorCode37001 => r#"## ERROR - 37001

### WhatsApp Voice: Outbound calls are not supported in this region

 The business-initiated WhatsApp call failed because it is not supported in the Twilio region.

#### Possible Causes
Business-initiated WhatsApp calls are not supported in all Twilio regions.

#### Possible Solutions
Ensure that a business-initiated WhatsApp call is placed in the supported Twilio Region."#,
            TwilioProgrammableVoiceError::ErrorCode32603 => r#"## WARNING - 32603

### Virtual Agent: Unsupported <Config> attribute(s) in TwiML

 A 32603 Error is an indication that &lt;VirtualAgent&gt; TwiML contains invalid or unsupported &lt;Config&gt; attributes

#### Possible Causes
* Some of the &lt;Config&gt; attribute(s) referenced in the &lt;VirtualAgent&gt; TwiML are not supported by the VirtualAgent provider. 

#### Possible Solutions
* Ensure the correct &lt;VirtualAgent&gt; connector is being referenced
* Review the Debugger message in console and update or remove the offending attributes"#,
            TwilioProgrammableVoiceError::ErrorCode17002 => r#"## ERROR - 17002

### This call ended more than 30 days ago

 The requested resource's call_sid is valid, but the call ended more than 30 days before the request was made.

#### Possible Causes
Voice Insights only stores 30 days worth of call data.

#### Possible Solutions
Do not request Voice Insights resources for calls that ended more than 30 days ago."#,
            TwilioProgrammableVoiceError::ErrorCode32008 => r#"Registration per second limit reached

<b>Cause</b>

You are allowed:
5 REGISTER requests / second / AOR

<b>Solution</b>

Reduce the frequency (Increase interval) of REGISTRATIONS by your SIP Endpoints."#,
            TwilioProgrammableVoiceError::ErrorCode32711 => r#"## WARNING - 32711

### Voice User-Defined Message: Request to subscription callback URL encountered error 

 Twilio received an error response when attempting to send a User-Defined Message event to the UserDefinedMessageSubscription callback URL.

#### Possible Causes
* The UserDefinedMessageSubscription callback URL is incorrect.
* Your endpoint at the UserDefinedMessageSubscription callback URL was not responding or responded with a 400- or 500-level HTTP status code.

#### Possible Solutions
* Make sure that the UserDefinedMessageSubscription callback URL is correct.
* Make sure that your endpoint at the UserDefinedMessageSubscription callback URL can successfully receive and respond to requests with 200-level HTTP status codes."#,
            TwilioProgrammableVoiceError::ErrorCode16010 => r#"## Error - 16010

### Conference Event: Internal Twilio Error

Twilio experienced an unexpected internal error in attempting to deliver a conference event to the provided call back URL."#,
            TwilioProgrammableVoiceError::ErrorCode16108 => r#"## Error - 16108

### Voice Recording: Request failed due to concurrent recordings

The request failed because there is more than one active recording for the provided Call SID"#,
            TwilioProgrammableVoiceError::ErrorCode13521 => r#"## WARNING - 13521

### `<Say>` element character limits exceeded

 The maximum text length inside a <Say> verb is limited and varies depending on the option used. Basic, Polly and Google TTS have different limits and character count.  
See the <a href='/docs/voice/twiml/say#voice'>Say Verb</a> API Reference for more information.

#### Possible Causes
The maximum text length inside the `<Say>` verb has been exceeded.

#### Possible Solutions
Make sure that the character limit is under the limit for the TTS type being used: 

* Basic TTS Voices are limited to 4096 UTF-8 single byte characters
* Polly TTS Voices are limited to 3000 UTF-8 single byte characters; note that SSML tags do not contribute to the character count
* Google TTS Voices are limited to 5000 UTF-8 single byte characters; note that SSML tags, newlines and spaces contribute to the character count
* The URL-encoded string cannot exceed 4500 characters

"#,
            TwilioProgrammableVoiceError::ErrorCode16113 => r#"## WARNING - 16113

### Voice Recording: Cannot download a dual-channel presentation of a mono recording

 A recording created as a mono recording cannot be downloaded as a dual-channel audio file.

#### Possible Causes
* The recording was requested as a mono recording upon creation

#### Possible Solutions
* Retry download without specifying a channel count"#,
            TwilioProgrammableVoiceError::ErrorCode15009 => r#"## ERROR - 15009

### Internal Server Error

 The requested resource failed to complete the request.

#### Possible Causes
The requested resource failed to complete the request.

#### Possible Solutions
The request should be retried using exponential backoff strategy until it succeeds."#,
            TwilioProgrammableVoiceError::ErrorCode10005 => r#"## ERROR - 10005

### Voice calling has been disabled for this account

 Voice calling has been disabled for this account. 

#### Possible Causes
Your account may have been flagged for review.

#### Possible Solutions
Please reach out to Support for assistance. "#,
            TwilioProgrammableVoiceError::ErrorCode64104 => r#"## ERROR - 64104

### ConversationRelay: Max call duration reached

The maximum call duration was reached and the call was terminated Twilio Voice has a hard limit to the length of a call, once that is reached the call is terminated.

#### Possible Causes
* This call reached the maximum duration.

#### Possible Solutions
* None at this moment."#,
            TwilioProgrammableVoiceError::ErrorCode32220 => r#"## ERROR -  32220

### Specifying an edge is not allowed when dialing SIP registered endpoints

 Twilio SIP domains allow endpoints to register through any supported SIP edge location. When dialing a registered endpoint, Twilio will fork the call to _all_ SIP devices registered under the same username/domain. Dialing one of your SIP domain's localized URIs (such as yourdomain.sip.ashburn.twilio.com) is not supported.

#### Possible Causes
* Attempting to place a call to username@yourdomain.sip.*ashburn*.twilio.com, or any other edge variant (e.g. ashburn-ix, umatilla, sydney, etc.)
* Note that existing registration customers dialing yourdomain.sip.us1.twilio.com or yourdomain.sip.us1-ix.twilio.com _are_ allowed (although this behavior is deprecated).

#### Possible Solutions
* Dial the global URI for your SIP domain, e.g. username@yourdomain.sip.twilio.com."#,
            TwilioProgrammableVoiceError::ErrorCode13340 => r#"## ERROR - 13340

### Gather: Degraded Speech 

 We are sorry for the inconvenience. You may be experiencing degraded <Gather> speech i.e. gather with input="speech dtmf" or input="speech".

#### Possible Causes
Internal Causes

#### Possible Solutions
Please check Twilio's status page to see if engineers are currently working on resolving the issue. "#,
            TwilioProgrammableVoiceError::ErrorCode17400 => r#"## ERROR - 17400

### Invalid query parameter

Invalid query parameter Invalid query parameter has been passed in the request.

#### Possible Causes
Invalid format for country code, SIP URI, Client address, or phone number. Wildcard searches.

#### Possible Solutions
Utilize only two-character country code; e.g. US
Do not attempt wildcard searches; e.g. from=+1206*
Use E.164 address format for telephone numbers; e.g. +14258675309
Use SIP URI format; e.g. sip:user@domain.com
Use Client address format; e.g. client:username"#,
            TwilioProgrammableVoiceError::ErrorCode32215 => r#"## Error - 32215

### Dial failure - Must specify a region in the SIP Domain

#### Possible Causes

* If you do a &lt;Dial&gt;&lt;Sip&gt; without specifying a region

#### Possible Solutions

* Dial the regional SIP domain.
For instance:
&lt;Dial&gt;&lt;Sip&gt;username@yoursipdomain.sip.`us1`.twilio.com&lt;/Dial&gt;&lt;/Sip&gt;.

Note the `us1` above that is specifying that you are calling a SIP endpoint registered to the US1 (North Virginia) data center."#,
            TwilioProgrammableVoiceError::ErrorCode13430 => r#"## WARNING - 13430

### Play: Invalid DTMF String

 Play cannot use the DTMF characters in the configuration.  Must be an alphanumeric, asterisk, or pound sign.

#### Possible Causes
Invalid value in DTMF String

#### Possible Solutions
Correct the DTMF string to use alpha-numeric, pound, or hash symbol."#,
            TwilioProgrammableVoiceError::ErrorCode22005 => r#"## ERROR - 22005

### Call Queue Full

There are more than 24 hours of calls in your call queue ## Error - 22005

### Call Queue Full 

There are more than 24 hours of calls in your call queue.

When you create calls via the REST API they are added to a call queue. Calls are removed from that call queue and placed at a default rate of 1 per second. This calls per second rate can be increased. Twilio will reject requests to place calls that will not be placed within 24 hours. At 1 call per second Twilio will reject calls once there are 86400 calls in your queue. More calls will not be accepted until your queue length decreases, calls in your queue will continue to be placed. If your application requires timely placement of calls, your application performance may be degraded.

#### Possible Causes
* Large spikes in call volume
* Runaway or looping application

#### Possible Solutions
* Validate application logic
* Contact your account manager or Twilio support to purchase additional capacity."#,
            TwilioProgrammableVoiceError::ErrorCode31486 => r#"## Error - 31486

### Busy Here

The callee is busy."#,
            TwilioProgrammableVoiceError::ErrorCode14226 => r#"## ERROR - 14226

### TaskRouter Enqueue not supported in this realm

 TaskRouter Enqueue not supported in this realm

#### Possible Causes
TaskRouter Enqueue not supported in this realm

#### Possible Solutions
Use TaskRouter Enqueue in supported realm"#,
            TwilioProgrammableVoiceError::ErrorCode32654 => r#"## ERROR - 32654

### Real-time Transcriptions: PCI Error

 A 32654 Error is an indication that the &lt;Transcription&gt; TwiML contains an invalid or insecure URL in the statusCallback attribute when PCI mode is enabled on your account

#### Possible Causes
* &lt;Transcription&gt; statusCallback attribute contains an invalid URL
* &lt;Transcription&gt; statusCallback attribute does not specify scheme https

#### Possible Solutions
* Ensure that the &lt;Transcription&gt; TwiML statusCallback attribute contains a valid URL format with scheme https"#,
            TwilioProgrammableVoiceError::ErrorCode31008 => r#"## ERROR - 31008

### Call cancelled

 ## Error - 31008

### Call cancelled

Unable to answer because the call has ended

#### Possible Causes
This error indicates that the incoming call was cancelled because it was not answered in time or it was accepted/rejected by another application instance registered with the same identity.

#### Possible Solutions
N/A"#,
            TwilioProgrammableVoiceError::ErrorCode31000 => r#"## Error - 31000

### Generic error

#### Possible Solutions

- No further information available. Check the debugger for more information on the actual cause. "#,
            TwilioProgrammableVoiceError::ErrorCode31923 => r#"## ERROR - 31923

### Stream - WebSocket - Malformed URL

 

#### Possible Causes
* URL schema is malformed

#### Possible Solutions
* Verify the URL in TwiML is correct "#,
            TwilioProgrammableVoiceError::ErrorCode13327 => r#"## Warning - 13327 

### Gather: Invalid speechTimeout value"#,
            TwilioProgrammableVoiceError::ErrorCode16105 => r#"## Error - 16105

### Voice Recording: Unavailable due to no valid public keys

Recording was not stored because there were no valid public keys on the account  (only applicable if call recording encryption feature enabled)"#,
            TwilioProgrammableVoiceError::ErrorCode16028 => r#"## ERROR - 16028

### Participant to be whispered is not present in the conference

 The participant to be whispered is not present in the conference, so the coach to the participant cannot be added to the conference.

#### Possible Causes
This may because the specified participant is not yet connected to the conference or no longer connected to the conference.

#### Possible Solutions
Validate that this call is connecting to the correct conference and the call SID specified is also in the conference."#,
            TwilioProgrammableVoiceError::ErrorCode13223 => r#"## WARNING - 13223

### Dial: Invalid phone number format

Phone Numbers must be in E.164 format. E.164 numbers are formatted [+] [country code] [subscriber number including area code] and can have a maximum of fifteen digits. Phone Numbers must be in E.164 format. E.164 numbers are formatted [+] [country code] [subscriber number including area code] and can have a maximum of 15 digits.

#### Possible Causes
- The phone number you are attempting to dial is not in E.164 format
- The `<Number>` noun inside your `<Dial>` TwiML instructions is not present
- You are attempting to dial something that is not a phone number

#### Possible Solutions
Check the format of the phone number inside your `<Dial><Number>` TwiML. 

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

"#,
            TwilioProgrammableVoiceError::ErrorCode64107 => r#"## ERROR - 64107

### ConversationRelay: Invalid Message Received

ConversationRelay received a message from websocket server that it could not process A message was received on the websocket that did not match any of the supported messages that ConversationRelay can act upon.

#### Possible Causes
* The websocket server is sending ConversationRelay a message that is not supported
* A component in front of the websocket server (API Gateway or Load Balancer) is sending ConversationRelay a message it cannot process

#### Possible Solutions
* Make sure all messages sent are conforming to the ConversationRelay websocket specification
* Make sure error handling is not sending back errors over the websocket to ConversationRelay
* Make sure API Gateways and Load Balancers are configured to not send errors to the ConversationRelay

Looking at the message can give an insight into why this might happen."#,
            TwilioProgrammableVoiceError::ErrorCode31207 => r#"## Error - 31207

### JWT token expiration too long.

JWT token expiration too long."#,
            TwilioProgrammableVoiceError::ErrorCode13218 => r#"## WARNING - 13218

### Dial: Invalid sequential value

sequential must be "true" or "false". 

#### Possible Causes
sequential value is not "true" or "false".

#### Possible Solutions
sequential must be "true" or "false"."#,
            TwilioProgrammableVoiceError::ErrorCode64008 => r#"## Error - 64008

### Pay: Payment Gateway rejected charge creation.

A 64008 Error is an indication that the request attempted to perform a charge of a credit card
but the attempt failed because the connector returned an error.

#### Possible Causes
* The &lt;Pay&gt; Connector returned an error.

#### Possible Solutions
* Please see the data returned in the with webhook to action url for more details.  
* Reach out to your Payment Gateway support for more assistance."#,
            TwilioProgrammableVoiceError::ErrorCode13238 => r#"## ERROR - 13238

### Dial->Conference: Invalid Verb used in waitUrl, holdUrl, or announceUrl TwiML

 The TwiML that executes via the waitUrl, holdUrl, or announceUrl can only contain Say, Play, Pause, and Redirect verbs. Dial, Gather, Hangup and Record are not allowed.

#### Possible Causes
Dial, Gather, Hangup or Record verbs are used in the TwiML.

#### Possible Solutions
Remove Dial, Gather, Hangup and Record verbs from the TwiML."#,
            TwilioProgrammableVoiceError::ErrorCode64007 => r#"## ERROR - 64007

### Pay: Connector does not support creating charge.

 ## Error - 64007

### Pay: Connector does not support creating charge.

 Error is an indication that the request to tokenize credit card failed because the connector does not support tokenization.

A 64007 Error is an indication that the request to create a charge on the credit card failed because the connector does not support making charges.


#### Possible Causes
* The paymentConnector attribute points to a Connector that does not support charging.

#### Possible Solutions
* Make sure that the paymentConnector matches a &lt;Pay&gt; Connector that supports charging.
* If tokenization was intended, please change the chargeAmount attribute. Please see <A HREF="/docs/voice/twiml/pay#chargeamount">this API documentation</A> for more details."#,
            TwilioProgrammableVoiceError::ErrorCode13239 => r#"## Error - 13239

### Dial->Conference: Invalid Trim Value

The trim parameter you entered is invalid. Acceptable values are trim-silence, do-not-trim, true and false."#,
            TwilioProgrammableVoiceError::ErrorCode31600 => r#"## Error - 31600

### Busy Everywhere 

All possible destinations are busy."#,
            TwilioProgrammableVoiceError::ErrorCode14219 => r#"## ERROR - 14219

### TaskRouter Dial Queue not supported in this realm

 TaskRouter Dial Queue not supported in this realm

#### Possible Causes
TaskRouter Dial Queue not supported in this realm

#### Possible Solutions
Use TaskRouter Dial Queue in supported realm"#,
            TwilioProgrammableVoiceError::ErrorCode31480 => r#"## Error - 31480

### Temporarily Unavailable

The callee is currently unavailable."#,
            TwilioProgrammableVoiceError::ErrorCode13810 => r#"## WARNING - 13810

### Reject: Invalid cause

 A call is rejected for any reason other than Busy or Hangup

#### Possible Causes
Could be timeout, cancelled, or no-answer.

#### Possible Solutions
Check application call timeout duration."#,
            TwilioProgrammableVoiceError::ErrorCode31487 => r#"## Error - 31487

### Request Terminated

The request has terminated as a result of a bye or cancel."#,
            TwilioProgrammableVoiceError::ErrorCode32015 => r#"## WARNING - 32015

### Call is terminated due to exceeding maximum call duration

Session-Timeout ## Warning - 32015

Call is terminated due to exceeding maximum call duration


#### Possible Causes
Maximum call duration as been exceeded.

#### Possible Solutions
Please reduce call duration or adjust account configurable timeLimits."#,
            TwilioProgrammableVoiceError::ErrorCode32212 => r#"## Error - 32212

### Registration Authentication problem

#### Possible Causes 

A SIP Endpoint tried to register with this SIP Domain and there isn't a Credential List mapped.
#### Possible Solutions

Please map a Credential List to the Registration Authentication part of the SIP Domain"#,
            TwilioProgrammableVoiceError::ErrorCode31903 => r#"## ERROR - 31903

### Stream - WebSocket - Connection Broken Pipe

The WebSocket connection has been abruptly closed by the server 

#### Possible Causes
* WebSocket server is down
* WebSocket server has experienced a problem causing the connection to close
* Connection has been lost
* End of stream is found. This happens when the socket has been closed by the Server leaving the websocket in an inconsistent state. The error is fired when the Streamer tries to read or write in a closed TCP socket.

#### Possible Solutions
* Verify server process is up and running
* Verify if there is any related connection error in the server logs
* Verify the server is connected and intermediate elements are routing traffic"#,
            TwilioProgrammableVoiceError::ErrorCode32505 => r#"## ERROR - 32505

### Voice Conversation: Invalid data inside existing conversation.

 Invalid data inside existing conversation.

#### Possible Causes
- Certain data was wrong when the Conversation was created or when the participants were added.

#### Possible Solutions
- Delete this Conversation using the Conversation API."#,
            TwilioProgrammableVoiceError::ErrorCode31500 => r#"## Error - 31500

### Internal Server Error

The server could not fulfill the request due to some unexpected condition."#,
            TwilioProgrammableVoiceError::ErrorCode64103 => r#"## ERROR - 64103

### ConversationRelay: Internal Server Error

An Internal Server Error happened when executing ConversationRelay logic. A 64103 Error is an indication there something happened internal in Twilio that affected your call.

#### Possible Causes
* Network issues
* Other unknown issues

#### Possible Solutions
* Retry
* Contact Twilio with identifying information about this call"#,
            TwilioProgrammableVoiceError::ErrorCode13511 => r#"## WARNING - 13511

### Say: Invalid voice value

 The value of the voice attribute must be valid one.   
See the <a href='/docs/voice/twiml/say#voice'>Say Verb</a> API Reference for more information.


#### Possible Causes
* You have entered an invalid attribute for voice parameter


#### Possible Solutions
* Enter a valid voice as parameter"#,
            TwilioProgrammableVoiceError::ErrorCode32105 => r#"Possible loop in Contact header

<b>Possible Causes</b>

You’re sending `127.0.0.1` or `localhost` in the Contact header which will create a loop at Twilio.

<b>Possible Solutions</b>

 Configure your SIP stack so it populates the Contact header properly."#,
            TwilioProgrammableVoiceError::ErrorCode31400 => r#"## Error - 31400

### Bad Request 

The request could not be understood due to malformed syntax."#,
            TwilioProgrammableVoiceError::ErrorCode31404 => r#"## Error - 31404

### Not Found 

The server has not found anything matching the request."#,
            TwilioProgrammableVoiceError::ErrorCode64015 => r#"## ERROR - 64015

### Pay: `<Pay>` verb is missing a needed Parameter

 A 64015 Error is an indication that the configured connector for the payment method is missing one of the Parameter needed in the `<Pay>` verb.

#### Possible Causes
The connector needs a specific Parameter in the `<Pay>` verb

#### Possible Solutions
The required parameters for the connector are listed in the "Description" section of the connector.
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
- If passing a `description` as a parameter in `<Pay>`, you must also pass the following fields via separate `<Parameter>` nouns: `name`, `kind`, `quantity`, `totalAmount`, `unitAmount`. If you received this error you either left one of these fields out, or passed an invalid value for one of them. See [Braintree Documentation](https://developer.paypal.com/braintree/docs/reference/request/transaction/sale#line_items.description) for more information."#,
            TwilioProgrammableVoiceError::ErrorCode31902 => r#"## ERROR - 31902

### Stream - WebSocket - Connection Refused

The WebSocket connection request sent by the stream is refused by the server host 

#### Possible Causes
* The stream is trying to connect the wrong port
* The WebSocket server is down

#### Possible Solutions
* Verify the TwiML URL is correct
* Verify server process is up and running"#,
            TwilioProgrammableVoiceError::ErrorCode32602 => r#"## ERROR - 32602

### Virtual Agent: Invalid Connector

 A 32602 Error is an indication that the referenced connector is not a valid &lt;VirtualAgent&gt; connector.

#### Possible Causes
* The connector referenced by the supplied connectorName does not exist
* The connector referenced by the supplied connectorName is not a VirtualAgent connector
* The supplied TwiML is missing the connectorName attribute

#### Possible Solutions
* Check the connectorName attribute in your TwiML and ensure that it matches the unique name of a VirtualAgent connector"#,
            TwilioProgrammableVoiceError::ErrorCode13321 => r#"## WARNING - 13321

### Gather -> Say: Invalid voice value

 The value of the voice attribute must be valid one.   
See the <a href='/docs/voice/twiml/say#voice'>Say Verb</a> API Reference for more information.


#### Possible Causes
* You have entered an invalid attribute for voice parameter

#### Possible Solutions
* Enter a valid voice as parameter"#,
            TwilioProgrammableVoiceError::ErrorCode31102 => r#"## Error - 31102

### Authorization token missing in request.

Authorization token missing in request."#,
            TwilioProgrammableVoiceError::ErrorCode31941 => r#"## ERROR - 31941

### Stream - Invalid Track configuration

 Unsupported value of track attribute in TwiML

#### Possible Causes
* Value different of "inbound_track" is used with verb Connect
* Value different of "inbound_track", "outbound_track" or "both_tracks" is used with verb Start

#### Possible Solutions
* Make sure track value is among supported ones"#,
            TwilioProgrammableVoiceError::ErrorCode13338 => r#"## ERROR - 13338

### Gather: Invalid actionOnEmptyResult value

 Defined action on Gather verb is invalid or empty

#### Possible Causes
Defined action on Gather verb is invalid or empty

#### Possible Solutions
Correct action on Gather verb"#,
            TwilioProgrammableVoiceError::ErrorCode64001 => r#"## ERROR - 64001

### Pay: Configuration Error

Failed to retrieve connector configuration. ## Error - 64001

### Pay: Configuration Error

A 64001 Error is an indication there is something wrong with the &lt;Pay&gt; connector configuration.

#### Possible Causes
* Credentials were removed from configuration.
* Credentials were modified and are not matching the Connector Type.
* Configuration is missing Credentials.
* Configuration contains invalid attributes.

#### Possible Solutions
* Recreate your &lt;Pay&gt; connector configuration."#,
            TwilioProgrammableVoiceError::ErrorCode32401 => r#"## WARNING - 32401

### BYOC Trunk routing failure - failover to Twilio default routing.

 Call delivery attempt(s) over the specified BYOC Trunk did not succeed. Failover to Twilio default routing took place.

#### Possible Causes
Twilio encountered errors communicating with your SIP communications infrastructure.

#### Possible Solutions
Look for errors with error code [32011](/docs/api/errors/32011), Error communicating with your SIP communications infrastructure, and follow instructions."#,
            TwilioProgrammableVoiceError::ErrorCode31205 => r#"## Error - 31205

### JWT token expired.

JWT token expired."#,
            TwilioProgrammableVoiceError::ErrorCode31003 => r#"## Error - 31003

### Connection timeout

The server could not produce a response within a suitable amount of time"#,
            TwilioProgrammableVoiceError::ErrorCode21234 => r#"## Error - 21234

### Invalid Machine Detection configuration value

You attempted to initiate an outbound call, but you provided a Machine Detection configuration value that is not allowed"#,
            TwilioProgrammableVoiceError::ErrorCode31105 => r#"## ERROR - 31105

### Invalid client name

 Client name should not contain control, space, delims, or unwise characters and should not be longer than 256 characters.

#### Possible Causes
Client name contains invalid characters or is longer than 256 characters.

#### Possible Solutions
Make sure that client name does not contain any of the invalid characters and length is at most 256 characters."#,
            TwilioProgrammableVoiceError::ErrorCode32504 => r#"## ERROR - 32504

### Voice Conversation: Incomplete Conversation.

 Existing Conversation did not have a Frontline worker participant. 

#### Possible Causes
- Issue with Frontline routing configuration that caused no worker assigned.
- Conversation might not have been created by the Frontline service. This error indicates that the code is directly using Conversation API to the create the conversation, and Frontline cannot determine the format of the call.
- Something went wrong when the Conversation was created by the Frontline worker or admin. 

#### Possible Solutions
- Check that your routing is set up properly in your Frontline Console page and delete this Conversation using the Conversation API."#,
            TwilioProgrammableVoiceError::ErrorCode21262 => r#"## WARNING - 21262

### No AMD status callback URL provided

No Answering Machine Detection status callback URL was provided for this call. Answering Machine Detection was enabled on this call, but no AMD status callback URL was provided, so Twilio has no where to send the results when they are returned. AMD results will still be available in the /Calls resource once the call is complete.

#### Possible Causes
No AMD status callback URL was provided

#### Possible Solutions
Provide an AMD status callback URL when enabling Answering Machine Detection"#,
            TwilioProgrammableVoiceError::ErrorCode32700 => r#"## ERROR - 32700

### Voice User-Defined Message: Internal Twilio Error

 Twilio experienced an unexpected internal error in attempting to deliver an User-Defined Message event to the Voice SDK client.

#### Possible Causes
* An error occurred during communication with Twilio internal services.

#### Possible Solutions
* See the Debugger in the Twilio Console for more information. If you repeatedly receive this error, please contact Twilio Support."#,
            TwilioProgrammableVoiceError::ErrorCode32002 => r#"## Error - 32002

### Dial failure - Twilio SIP Domain not found

#### Possible Causes

* There is a typo in the name of the SIP Domain.
* When a SIP Domain is created you must press SAVE on Console. 

#### Possible Solutions

* Verify the existence and name of the SIP Domain that you created."#,
            TwilioProgrammableVoiceError::ErrorCode64110 => r#"## ERROR - 64110

### ConversationRelay: Account has been opted out

The account has been opted out of ConversationRelay. The call was rejected because your account has been opted out of ConversationRelay.

#### Possible Causes
Account opted out of ConversationRelay.

#### Possible Solutions
Reach out to Twilio to enable ConversationRelay for your account."#,
            TwilioProgrammableVoiceError::ErrorCode21300 => r#"## ERROR - 21300

### Invalid BYOC trunk SID

 You have specified a BYOC trunk SID that is malformed or non-existent.

#### Possible Causes
* The SID in question is not formatted correctly. A valid BYOC trunk SID starts with prefix "BY" and is 34 characters in length.
* The BYOC trunk identified by the SID does not exist. If it existed in the past, it has since been removed.
* The BYOC trunk exists but ownership is with a different account.


#### Possible Solutions
Make sure you submit the SID of a BYOC trunk that you can locate within your account."#,
            TwilioProgrammableVoiceError::ErrorCode31203 => r#"## Error - 31203

### No valid account.

No valid account."#,
            TwilioProgrammableVoiceError::ErrorCode31950 => r#"## WARNING - 31950

### Stream - Protocol - Malformed Message

 The Streamer has received a message that cannot be decoded
(This error is sent once per stream)

#### Possible Causes
* WebSocket binary format is used

#### Possible Solutions
* Do not use binary format"#,
            TwilioProgrammableVoiceError::ErrorCode21215 => r#"## ERROR - 21215

### Geo Permission configuration is not permitting call

 You attempted to initiate an outbound phone call to a phone number that is not enabled on your account.

#### Possible Causes
User dialed a destination your application is not enabled to support calling to.

#### Possible Solutions
Please check your [Voice Dialing Geographic Permissions](https://www.twilio.com/console/voice/calls/geo-permissions), fix it, and try again."#,
            TwilioProgrammableVoiceError::ErrorCode13621 => r#"## Warning - 13621

### Invalid 'recordingStatusCallbackEvent'

One or more of the recordingStatusCallbackEvents provided were invalid."#,
            TwilioProgrammableVoiceError::ErrorCode32506 => r#"## ERROR - 32506

### Voice Conversation: Failed to get worker assigned to a newly created Conversation.

 Your Frontline instance failed to assign a worker after Conversation auto-creation.

#### Possible Causes
- Issue with Frontline routing configuration that caused no worker assigned.
- Conversation might not have been created by the Frontline service. This error indicates that the code is directly using Conversation API to the create the conversation, and Frontline cannot determine the format of the call.
- Something went wrong when the Conversation was created by the Frontline worker or admin. 

#### Possible Solutions
- Check the Frontline routing configurations.
- If a custom Routing was used, make sure the webhook is going to the right URL."#,
            TwilioProgrammableVoiceError::ErrorCode31603 => r#"## Error - 31603

### Decline

The callee does not wish to participate in the call."#,
            TwilioProgrammableVoiceError::ErrorCode21216 => r#"## ERROR - 21216

### API: Call blocked by Twilio

 The destination number is blocked by Twilio.

#### Possible Causes
The outbound call to the destination number was blocked by Twilio. Potential causes may be:
- The destination has a high-risk of fraud
- Due to regulatory reasons, the destination cannot be reached
- You are placing a call to a +1 destination and your account is missing a Primary Customer Profile

#### Possible Solutions
Potential solutions may be:
- If your destination is being incorrectly identified as high-risk of fraud and you have a legitimate need to call this number, please reach out to Twilio Support through the [Console](https://console.twilio.com) or [Help Center](https://support.twilio.com/hc/en-us).
- If you are placing a call to a +1 destination and your account is missing a Primary Customer Profile, create one in [TrustHub](https://console.twilio.com/us1/account/trust-hub/customer-profiles). "#,
            TwilioProgrammableVoiceError::ErrorCode31302 => r#"## ERROR - 31302

### Unsupported Cancel Message Error

 This version of the SDK no longer supports processing cancel push notification messages. You must register via Voice.register(...) on Android or [TwilioVoice registerWithAccessToken:deviceToken:completion:] on iOS with this version of the SDK to stop receiving cancel push notification messages. Cancellations are now handled internally and reported to you on behalf of the SDK.

#### Possible Causes
The identity associated with the Twilio Voice SDK is still registered to receive cancel push notification messages.

#### Possible Solutions
The application must register via Voice.register(...) on Android or [TwilioVoice registerWithAccessToken:deviceToken:completion:] on iOS to stop receiving cancel push notification messages."#,
            TwilioProgrammableVoiceError::ErrorCode64021 => r#"## ERROR - 64021

### Pay: Invalid Operation

 A 64021 Error is an indication invalid operation was provided.

#### Possible Causes
* Start operation on already started Pay session

#### Possible Solutions
* Make sure to use a valid operation"#,
            TwilioProgrammableVoiceError::ErrorCode13803 => r#"## ERROR - 13803

### SMS verb not supported in this realm

 SMS verb not supported in this realm

#### Possible Causes
SMS verb not supported in this realm

#### Possible Solutions
Use SMS verb in another realm"#,
            TwilioProgrammableVoiceError::ErrorCode32710 => r#"## WARNING - 32710

### Voice User-Defined Message: Subscription Callback Internal Error

 Twilio experienced an unexpected internal error in attempting to deliver an User-Defined Message event to the callback URL provided in the Voice User-Defined Message Subscription request.

#### Possible Causes
* An error occurred during communication with Twilio internal services.

#### Possible Solutions
* See the Debugger in the Twilio Console for further information. If you repeatedly receive this error, please contact Twilio Support."#,
            TwilioProgrammableVoiceError::ErrorCode64013 => r#"## ERROR - 64013

### Pay: Connector does not support supplied paymentMethod attribute.

 A 64013 Error is an indication that the provided paymentMethod attribute was not supported by the Connector.

#### Possible Causes
The targeted Payment Connector did not support the paymentMethod. For instance, a credit card might have been supplied but the Connector did not support Credit Cards.

#### Possible Solutions
Make sure the Connector supports the defined paymentMethod attribute."#,
            TwilioProgrammableVoiceError::ErrorCode13619 => r#"Warning - 13619 Record: Transcription feature not available for this type of recording. Possible Causes The transcription attribute was specified for a type of recording that does not currently support transcription. Possible Solutions: Check in the TwiML or API docs whether the recording type you specified allows transcription."#,
            TwilioProgrammableVoiceError::ErrorCode31005 => r#"## Error - 31005

### Connection error

A connection error occurred during the call"#,
            TwilioProgrammableVoiceError::ErrorCode32604 => r#"## ERROR - 32604

### Virtual Agent: Internal Error

 A 32604 Error is an indication that there is an internal server error and some internal services failed.

#### Possible Causes
* Some error occurred during communication with Twilio internal services.

#### Possible Solutions
* If this is a continuous experience, please contact Twilio Support."#,
            TwilioProgrammableVoiceError::ErrorCode32013 => r#"## ERROR - 32013

### SIP: Parent account SIP Interface CPS limit exceeded

 #### Possible Causes 
*  Calls Per Second placed on a SIP domain exceeded the Parent Account limits. 

#### Possible Solutions
*  You've exceeded the number of calls per second you are authorized to make on your Parent Account for SIP Interface. 

#### Possible Causes
Calls Per Second placed on a SIP domain exceeded the Parent Account limits. 

#### Possible Solutions
Increase the CPS on the Parent Account."#,
            TwilioProgrammableVoiceError::ErrorCode31951 => r#"## WARNING - 31951

### Stream - Protocol - Invalid Message

 The Streamer has received a message non compliant with the protocol
(This error is sent once per stream)

#### Possible Causes
* Message does not have JSON format
* Unknown message type
* Missing or extra field in message
* Wrong Stream SID used in message

#### Possible Solutions
* Validate message is compliant with Stream protocols
* Validate message is a valid JSON"#,
            TwilioProgrammableVoiceError::ErrorCode32605 => r#"## ERROR - 32605

### Virtual Agent: PCI Error

 A 32605 Error is an indication that the &lt;VirtualAgent&gt; TwiML contains an invalid or insecure URL in the statusCallback attribute when PCI mode is enabled on your account

#### Possible Causes
* &lt;VirtualAgent&gt; statusCallback attribute contains an invalid URL
* &lt;VirtualAgent&gt; statusCallback attribute does not specify scheme https


#### Possible Solutions
* Ensure that the &lt;VirtualAgent&gt; TwiML statusCallback attribute contains a valid URL format with scheme https"#,
            TwilioProgrammableVoiceError::ErrorCode16104 => r#"## Error - 16104

### Voice Recording: Unavailable due to encryption failure

Recording was not stored due to a public key encryption failure.  (only applicable if call recording encryption feature enabled)"#,
            TwilioProgrammableVoiceError::ErrorCode31204 => r#"## Error - 31204

### Invalid JWT token.

Invalid JWT token."#,
            TwilioProgrammableVoiceError::ErrorCode32022 => r#"## ERROR - 32022

### ACK not received from your SIP endpoint

 No ACK was received from your SIP endpoint 32 seconds after sending a 200 OK response, which resulted in the termination of the Inbound call. As per RFC 3261, the calling device’s SIP endpoint must send an ACK to acknowledge the receipt of a 200 OK response, to establish a successful call. 

#### Possible Causes
* Your NAT configuration may be incorrect.
* Your Firewall may be blocking traffic.
* SIP signaling not adhering to RFC 3261. For example ACK sent to the wrong IP address or using incorrect Route headers or Request URI.
* A network disruption or packet loss within or between your network and the Twilio network may have caused the ACK messages to get lost.

#### Possible Solutions
* Review configurations of your  SIP endpoint. Consult your documentation for details.
* Review logs on your SIP endpoint if possible. Consult your documentation for details on how to enable and access the logs.
* SIP devices generally re-transmit messages multiple times, in the event a single message is lost. Ensure your device is configured with the maximum retransmissions.
* Review proper SIP signaling by downloading PCAP via call log in Twilio console. And if possible, capture PCAP at your SIP endpoint to narrow down the cause of the signaling issue."#,
            TwilioProgrammableVoiceError::ErrorCode64012 => r#"## ERROR - 64012

### Pay: Payment Gateway rejected the card.

 ## Error - 64012

### Pay: Payment Gateway rejected the card.

A 64012 Error is an indication that the provided card was rejected by the Payment Gateway.

#### Possible Causes
* The card used was rejected by the Payment Gateway.

#### Possible Solutions
* Check the PaymentError in the response for more detailed information from the Payment Gateway.
* Prompt the user about the card information again."#,
            TwilioProgrammableVoiceError::ErrorCode31301 => r#"## Error - 31301

### Registration error

An error occurred during registration"#,
            TwilioProgrammableVoiceError::ErrorCode32210 => r#"## Error - 32210

###  SIP: Registration not supported

#### Possible Causes 
*    Registration is not enabled for the SIP domain

#### Possible Solutions
*   Enable registration for the SIP domain."#,
            TwilioProgrammableVoiceError::ErrorCode31960 => r#"## ERROR - 31960

### Stream - Quota exceeded

  Exceeded quota for Dialogflow API

#### Possible Causes
*The Dialogflow API quotas has been exceeded

#### Possible Solutions
*Increase the Dialogflow API quotas for the project used with Twilio Virtual Agent"#,
            TwilioProgrammableVoiceError::ErrorCode53401 => r#"## Error - 53401

### Server is unable to create or apply a local media description

Raised whenever the Server is unable to create or apply a local media description."#,
            TwilioProgrammableVoiceError::ErrorCode64024 => r#"## ERROR - 64024

### Pay: Connector Instance Not Approved

 This connector instance has not been approved by Twilio. Please contact Twilio support to gain approval for LIVE transactions.

#### Possible Causes
This is caused by your connector instance not having been approved by Twilio for LIVE transactions. You must go through a PCI review in order to gain approval for LIVE transactions.

#### Possible Solutions
Contact Twilio Support in order to gain approval for LIVE transactions for the specific connector instance."#,
            TwilioProgrammableVoiceError::ErrorCode31106 => r#"## Error - 31106

### Invalid data

The provided data was not valid."#,
            TwilioProgrammableVoiceError::ErrorCode14300 => r#"## ERROR - 14300

### Start: Invalid nested noun value

 Start or Stop has an invalid noun defined.

#### Possible Causes
Start or Stop has an invalid noun defined.

#### Possible Solutions
Correct the noun used in Twiml"#,
            TwilioProgrammableVoiceError::ErrorCode64020 => r#"## ERROR - 64020

### Pay: Invalid Parameter Value

 A 64020 Error is an indication that one or more of the values provided for an attribute or parameter are invalid.

#### Possible Causes
* Misspelled parameter values
* Incorrect format for parameter values
* Unexpcted value for one or more parameters

#### Possible Solutions
* Ensure that values match what is expected by refering to the API and TwiML docs"#,
            TwilioProgrammableVoiceError::ErrorCode64101 => r#"## ERROR - 64101

### ConversationRelay: Invalid Parameter

Invalid parameter provided to the TwiML. A 64101 Error is an indication there is something wrong with the &lt;ConversationRelay&gt; parameters.

#### Possible Causes
* One of the submitted attributes to the TwiML is invalid.

#### Possible Solutions
* Validate that parameters provided are corrected."#,
            TwilioProgrammableVoiceError::ErrorCode32703 => r#"## WARNING - 32703

### Voice User-Defined Message: Content body exceeded max length

 The content body in the Voice User-Defined Message request exceeds the maximum allowed length of 10000 characters.

#### Possible Causes
* The content body in the Voice POST request to the UserDefinedMessage.Resource exceeds the maximum allowed length of 10000 characters.

#### Possible Solutions
* Make sure that the content body in the request does not exceed the maximum.length."#,
            TwilioProgrammableVoiceError::ErrorCode64102 => r#"## ERROR - 64102

### ConversationRelay: Unable to Connect to Websocket URL

Failed to connect to websocket URL. A 64102 Error is an indication there is something wrong with the &lt;ConversationRelay&gt; url parameter when Twilio attempted to connect to it.

#### Possible Causes
* The url parameter was incorrectly entered
* There is not a websocket server available at the url
* The websocket server at the url is not responding correctly


#### Possible Solutions
* Make sure the url parameter was entered correctly
* Make sure the websocket server is running at said url
* Make sure the websocket server is healthy and able to respond properly"#,
            TwilioProgrammableVoiceError::ErrorCode32214 => r#"## Warning 32214

###  Invalid &lt;Dial&gt;&lt;Sip&gt;

#### Possible Causes
Your TwiML can only &lt;Dial&gt;&lt;Sip&gt;username@yoursipdomain.sip.us1.twilio.com&lt;/Dial&gt;&lt;/Sip&gt;
  to Twilio SIP Domains either in the same subaccount or within the master account.

<b>Possible Solutions</b>

Register the SIP endpoint to a SIP Domain on the same subaccount."#,
            TwilioProgrammableVoiceError::ErrorCode32221 => r#"## ERROR - 32221

### Dialing SIP Endpoint failure - No devices registered in specified edge

 Twilio was unable to dial the specified SIP Endpoint. While the user did have at least one device registered, no devices were registered through the edge location specified in the URI.

#### Possible Causes
* You are trying to migrate from us1-only or us1-ix-only SIP registration and have configured some devices to register through other edges, but you are still using edge-specific URIs (e.g. username@yourdomain.sip.us1.twilio.com) when placing calls in your application.
* If you only intend to use the us1 or us1-ix edges, you may have SIP devices unexpectedly registering through another edge. Check the domain name and outbound proxy settings on your client devices.

#### Possible Solutions
* If you want to take advantage of new global SIP registration features, including the ability to dial any registered endpoint regardless of the edge through which it registered, update your TwiML or REST API app to use your SIP domain’s global URI, e.g username@yourdomain.sip.twilio.com.
* If SIP devices are registering through an edge you did not intend, either re-configure these devices to register through the desired edge, or turn off the SIP Registration functionality in your device settings if the registration behavior is not needed.
* To troubleshoot, you can verify if a SIP Endpoint has registered by using the Console "Registered SIP Endpoints" tab found on the SIP Domains page."#,
            TwilioProgrammableVoiceError::ErrorCode16109 => r#"## Error - 16109

### Voice Recording: Cannot fetch .mp3 encrypted recording

Recording cannot be retrieved in .mp3 format when call recording encryption feature is enabled on the account.  Please re-request with .wav format."#,
            TwilioProgrammableVoiceError::ErrorCode31900 => r#"## ERROR - 31900

### Stream - Unknown Error

This stream has failed due to an unknown cause 

#### Possible Causes
Unknown

#### Possible Solutions
Unknown"#,
            TwilioProgrammableVoiceError::ErrorCode32400 => r#"## ERROR - 32400

### BYOC Trunk routing failure - failover routing disabled.

 Call delivery attempt(s) over the specified BYOC Trunk did not succeed. Failover routing was disabled.

#### Possible Causes
Twilio encountered errors communicating with your SIP communications infrastructure.

#### Possible Solutions
Look for errors with error code [32011](/docs/api/errors/32011), Error communicating with your SIP communications infrastructure, and follow instructions."#,
            TwilioProgrammableVoiceError::ErrorCode16107 => r#"## Error - 16107

### Voice Recording: Encrypted with alternate public key

Recording was encrypted with alternate public key because the one configured on the account is invalid or deleted. (only applicable if call recording encryption feature enabled)"#,
            TwilioProgrammableVoiceError::ErrorCode31403 => r#"## Error - 31403

### Forbidden

The server understood the request, but is refusing to fulfill it."#,
            TwilioProgrammableVoiceError::ErrorCode32223 => r#"## ERROR - 32223

### There is no username in the SIP URI when calling a SIP registered endpoint

 Calling Twilio SIP domains such as "yourdomain.sip.twilio.com" is not allowed. You can only call a registered SIP endpoint such as "username@yourdomain.sip.twilio.com".

#### Possible Causes
* Attempting to place a call from Twilio back to your own Twilio SIP domain. Twilio does not allow SIP calls between Twilio domains either in the same account or different accounts.
* Attempting to call a registered SIP AOR without specifying the username you wanted to call

#### Possible Solutions
* Add the correct username in the SIP URI you call, e.g., username@yourdomain.sip.twilio.com"#,
            TwilioProgrammableVoiceError::ErrorCode13513 => r#"## WARNING - 13513

### Say: Invalid rate value

 Say rate value is invalid.  The function will use a default rate value of 150.

#### Possible Causes
Say rate value is invalid.

#### Possible Solutions
Change Say rate value."#,
            TwilioProgrammableVoiceError::ErrorCode32600 => r#"## ERROR - 32600

### Virtual Agent: Configuration Error

 A 32600 Error is an indication there is something wrong with the &lt;VirtualAgent&gt; connector configuration.

#### Possible Causes
* Connector configuration is missing required properties
* Connector configuration contains invalid properties

#### Possible Solutions
* Update your &lt;VirtualAgent&gt; connector configuration"#,
            TwilioProgrammableVoiceError::ErrorCode31931 => r#"## WARNING - 31931

### Stream - Media - Media Discarded

 Audio discarded due to buffer overflow in the Websocket downstream .
NOTE: This error is sent once per stream

#### Possible Causes
* More than 10 minutes of audio is buffered by the Streamer. No more data is accepted.

#### Possible Solutions
* Wait until previous media has been processed before to send more media."#,
            TwilioProgrammableVoiceError::ErrorCode31924 => r#"## ERROR - 31924

### Stream - Websocket - Protocol Error

 Malformed message or message not conformant with WebSocket protocol

#### Possible Causes
* The WebSocket control frame was fragmented

#### Possible Solutions
* Verify the messages sent by the server"#,
            TwilioProgrammableVoiceError::ErrorCode16021 => r#"## Error - 16021

### Failed to join conference due to account concurrency limit exceeded

A conference participant could not be created, as the account is currently at or above its concurrency limit."#,
            TwilioProgrammableVoiceError::ErrorCode31100 => r#"## Error - 31100

### Malformed request

The request could not be understood due to malformed syntax."#,
            TwilioProgrammableVoiceError::ErrorCode31201 => r#"## Error - 31201

### Twilio Client: Error occurred while accessing microphone

#### Possible Solutions

- Check for conflicting applications that might be accessing or attempting to access the microphone
- Try unplugging and reconnecting the microphone, if applicable, to reacquire the microphone"#,
            TwilioProgrammableVoiceError::ErrorCode16000 => r#"## Error - 16000

### Whisper Not Available on Twilio Conference

Whisper is only available on Agent Conference, to create an Agent Conference configure your account to use Agent Conference in the [conference settings page](https://www.twilio.com/console/voice/settings/conferences)"#,
            TwilioProgrammableVoiceError::ErrorCode13219 => r#"## WARNING - 13219

### Dial: Invalid answerOnBridge value

answerOnBridge must be "true" or "false". 

#### Possible Causes
answerOnBridge value is not "true" or "false".

#### Possible Solutions
answerOnBridge must be "true" or "false"."#,
            TwilioProgrammableVoiceError::ErrorCode14207 => r#"## WARNING - 14207

###  Enqueue: The targeted queue reached max queue size

 The default max queue size is 1000 and can be set to a maximum of 5000. The targeted queue has reached its maxSize limit. Once the queue reaches the limit, new requests to queue calls using <Enqueue> will be rejected.

#### Possible Causes
A full queue can indicate higher <Enqueue> volume, longer queue times, or an incorrectly configured queue’s maxSize.

#### Possible Solutions
A queue’s maxSize can be adjusted to a maximum of 5000. Use the [Queue API](https://www.twilio.com/docs/voice/api/queue-resource) to identify the current maxSize and adjust accordingly.

If a waitUrl is provided for <Enqueue>, the parameters MaxQueueSize and CurrentQueueSize are included in the Webhook request to the waitUrl. Your application can use this information to determine queue capacity and adjust max queue size via the [API](https://www.twilio.com/docs/voice/api/queue-resource).

If the queue is already configured for the maximum allowed queue size (5000), one possible workaround is to create multiple queues and distribute the <Enqueue> requests between them."#,
            TwilioProgrammableVoiceError::ErrorCode31101 => r#"## Error - 31101

### Missing parameter array in request.

Missing parameter array in request."#,
            TwilioProgrammableVoiceError::ErrorCode13512 => r#"## WARNING - 13512

### Gather element has an invalid "language" attribute value

 A Gather element has an invalid value for its "language" attribute. See [the accepted language values list](https://www.twilio.com/docs/voice/twiml/gather#languagetags) and update your Gather TwiML.

#### Possible Causes
Your Gather TwiML has an invalid language attribute value. (Example: `en` is an invalid value, but `en-US` is a valid value.) 

#### Possible Solutions
Find your preferred language in [this list of accepted values](https://www.twilio.com/docs/voice/twiml/gather#languagetags) and update your Gather TwiML accordingly."#,
            TwilioProgrammableVoiceError::ErrorCode31007 => r#"## Error - 31007

### Twilio Client: This JS client version has been deprecated

#### Possible Solutions

- Migrate to newer version of Twilio JS client by following the direction provided here: https://www.twilio.com/docs/voice/client/javascript/sdk-11-12-migration-guide"#,
            TwilioProgrammableVoiceError::ErrorCode31503 => r#"## ERROR - 31503

### Service Unavailable

 ## Error - 31503

### Service Unavailable

The server is currently unable to handle the request due to a temporary overloading or maintenance of the server. This error can also be caused by the Application SID provided in the access token pointing to an inaccessible URL.

#### Possible Causes
* The server is under maintenance.
* The Application SID provided in the access token points to an inaccessible URL.

#### Possible Solutions
Please check the Application SID URL. For further assistance, please reach out to Twilio Support through the [Console](https://console.twilio.com) or [Help Center](https://support.twilio.com)."#,
            TwilioProgrammableVoiceError::ErrorCode13620 => r#"## WARNING - 13620

### Record: Transcription not available for this recording

 Record: Transcription feature not available for this recording.

#### Possible Causes
Transcription feature not supported for recordings when your account has one or more of the following features enabled: Recording Encryption, PCI recordings.

#### Possible Solutions
Contact support to disable the corresponding feature(s) on your account."#,
            TwilioProgrammableVoiceError::ErrorCode31001 => r#"## Error - 31001

### Application not found.

Application not found."#,
            TwilioProgrammableVoiceError::ErrorCode13337 => r#"## ERROR - 13337

### Gather: callback must be over HTTPS when using gather with PCI compliance

 Gather: callback must be over HTTPS when using gather with PCI compliance

#### Possible Causes
Gather URL must use HTTPS. 

#### Possible Solutions
Gather URL must use HTTPS and valid URL format."#,
            TwilioProgrammableVoiceError::ErrorCode13258 => r#"## ERROR - 13258

### Dial->Sim not supported in this realm

 Dial->Sim not supported in this realm

#### Possible Causes
Dial->Sim not supported in this realm

#### Possible Solutions
Use Dial->Sim in another realm"#,
            TwilioProgrammableVoiceError::ErrorCode32501 => r#"## ERROR - 32501

### Voice Conversation: TwiML attributes validation error.

 Invalid attributes found in <Conversation> noun.

#### Possible Causes
- Invalid attributes within the <Conversation> noun.
- Can be caused by any invalid syntax within the <Conversation> noun.

#### Possible Solutions
- Check attribute names or the values following the <Conversation> noun.
- Refer to the <Conversation> twiml docs page “/docs/voice/twiml/connect/conversation” for more information."#,
            TwilioProgrammableVoiceError::ErrorCode13256 => r#"## WARNING - 13256

### Invalid recordingStatusCallback URL

 A recording was initiated using an invalid URL for the recordingStatusCallback parameter.
The recording will be processed, but status notifications will not be sent.

#### Possible Causes
The recording status callback URL you provided is not valid.

#### Possible Solutions
Set recordingStatusCallback to a valid URL or leave blank to disable callbacks."#,
            TwilioProgrammableVoiceError::ErrorCode13214 => r#"## WARNING - 13214

### Dial: Invalid callerId value

callerId must be the calling number, called number, or a validated outgoing number. See the <a href='/docs/api/twiml/dial#callerId'>Dial Verb</a> API Reference for more information. ## Warning - 13214 

### Dial: Invalid callerId value

callerId must be the calling number, called number, or a validated outgoing number.  Additionally,
if the calling leg is a client leg, the caller ID is required and must be a validated outgoing number. 
See the <a href='/docs/api/twiml/dial#callerId'>Dial Verb</a> API Reference for more information.

#### Possible Causes
Carriers may send invalid Caller IDs on Incoming calls to Twilio. If a specific Caller ID is not explicitly defined on the <Dial> verb, the invalid Caller ID will be passed to the destination. This may cause some destination providers to reject the call, and Twilio will mark it as failed.

#### Possible Solutions
To work around this, your application should recognize invalid caller IDs, and substitute them with a valid Caller ID to construct the subsequent <Dial>. "#,
            TwilioProgrammableVoiceError::ErrorCode16111 => r#"## ERROR - 16111

### Voice Recording: Upload file to external AWS S3 bucket failed (Invalid Configuration)

 Twilio failed to upload the recording file to the external AWS S3 bucket specified in Voice Settings due to an invalid configuration. The recording file has been deleted, and if the resource exists, the resource has been marked as “failed”.

#### Possible Causes
* Invalid S3 bucket URL
* Invalid AWS S3 bucket credentials
* AWS S3 bucket credentials not found in Credential Storage

#### Possible Solutions
* Ensure AWS S3 bucket credentials are valid and available in Credential Storage
* Set a valid S3 bucket URL
* Disable external storage
* Ensure External Storage settings are provided for each home region you want to use"#,
            TwilioProgrammableVoiceError::ErrorCode32655 => r#"## WARNING - 32655

### Real-time Transcriptions: Intelligence Service Unreachable

 A 32655 Error indicates a connection to the configured Intelligence Service could not be established due to an internal error. While you will continue to receive real-time transcription events, no Language Operators will be executed, and the transcript will not be saved

#### Possible Causes
* Some internal error occurred while trying to establish connection with the configured Intelligence service 

#### Possible Solutions
* If this is a continuous experience, please contact Twilio Support"#,
            TwilioProgrammableVoiceError::ErrorCode16001 => r#"## Error - 16001

### Conference is not bridged

The conference you have tried to access is not currently active."#,
            TwilioProgrammableVoiceError::ErrorCode13622 => r#"## WARNING - 13622

### Record: invalid recordingTrack value

The recordingTrack can be "inbound", "outbound", or "both" 

#### Possible Causes
Invalid combinations of recordingTrack with requested channels

#### Possible Solutions
Check valid combinations of recordingTrack with requested channels"#,
            TwilioProgrammableVoiceError::ErrorCode13334 => r#"## WARNING - 13334

### Gather: Invalid model value

 ## Warning - 13334

### Gather: Invalid model value

#### Possible Causes
Invalid model

#### Possible Solutions
Enter a valid model"#,
            TwilioProgrammableVoiceError::ErrorCode31401 => r#"## ERROR - 31401

### UserMedia Permission Denied

 The browser or end-user denied permissions to user media. Therefore we were unable to acquire input audio.

#### Possible Causes
* The user denied the getUserMedia request.
* The browser denied the getUserMedia request.
* The application has not been configured with the proper permissions.

#### Possible Solutions
* The user should accept the request next time prompted. If the browser saved the deny, the user should change that permission in their browser.
* The user should to verify that the browser has permission to access the microphone at this address.
* The user should ensure that the proper permissions have been granted in the mobile device OS."#,
            TwilioProgrammableVoiceError::ErrorCode13225 => r#"## ERROR - 13225

### Dial: Call blocked by Twilio

 The destination number is blocked by Twilio.

#### Possible Causes
The outbound call to the destination number was blocked by Twilio. Potential causes may be:
- The destination has a high-risk of fraud
- Due to regulatory reasons, the destination cannot be reached
- You are placing a call to a +1 destination and your account is missing a Primary Customer Profile

#### Possible Solutions
Potential solutions may be:
- If your destination is being incorrectly identified as high-risk of fraud and you have a legitimate need to call this number, please reach out to Twilio Support through the [Console](https://console.twilio.com) or [Help Center](https://support.twilio.com/hc/en-us).
- If you are placing a call to a +1 destination and your account is missing a Primary Customer Profile, create one in [TrustHub](https://console.twilio.com/us1/account/trust-hub/customer-profiles). 
"#,
            TwilioProgrammableVoiceError::ErrorCode22001 => r#"## ERROR - 22001

### Call timed out

 The call timed out before system could complete the call

#### Possible Causes
The call timed out before system could complete the call

#### Possible Solutions
CPS limits may be too low"#,
            TwilioProgrammableVoiceError::ErrorCode31942 => r#"## ERROR - 31942

### Stream - Invalid connector configuration

 Connector invalid configuration

#### Possible Causes
* When using Virtual Agent make sure the Project ID, Conversation Profile ID and language configurations are correct.

#### Possible Solutions
* Review configuration"#,
            TwilioProgrammableVoiceError::ErrorCode31481 => r#"## Error - 31481

### Call/Transaction Does Not Exist 

The call no longer exists."#,
            TwilioProgrammableVoiceError::ErrorCode32502 => r#"## WARNING - 32502

### Voice Conversation: Callback event internal error.

 Internal error occurred when executing a request to one of the status callback URLs specified in the Conversation noun. This error will occur when Voice Conversation service fails to reach out to Callback service.

#### Possible Causes
- Twilio failed to execute callback request when Voice Conversation service attempted to reach out to status Callback service. 

#### Possible Solutions
- If the error persists, please <a href="/help/contact">contact us</a> to figure out what has happened and how to fix it!"#,
            TwilioProgrammableVoiceError::ErrorCode31006 => r#"## Error - 31006

### Audio device error

Unable to start the audio device"#,
            TwilioProgrammableVoiceError::ErrorCode13699 => r#"## WARNING - 13699

### Record: Invalid trim value

 If a recording has an invalid trim value in configuration.  Defaults to no trim.

#### Possible Causes
Invalid recording trim value.

#### Possible Solutions
Correct the recording trim value."#,
            TwilioProgrammableVoiceError::ErrorCode31921 => r#"## ERROR - 31921

### Stream - WebSocket - Close Error

The remote server has closed the WebSocket connection 

#### Possible Causes
* The remote server ended the connection with a termination code listed in https://tools.ietf.org/html/rfc6455#section-7.4.1
* The Media Stream has received a websocket close event defined by https://developer.mozilla.org/en-US/docs/Web/API/CloseEvent

#### Possible Solutions
* Verify error logs in the remote websocket server"#,
            TwilioProgrammableVoiceError::ErrorCode32018 => r#"## ERROR - 32018

### Twiml size exceeded maximum allowed value

 Twiml size exceeded maximum allowed value of 4000 characters

#### Possible Causes
Twiml size exceeded maximum allowed value of 4000 characters

#### Possible Solutions
Reduce the size of the twiml embedded in the request"#,
            TwilioProgrammableVoiceError::ErrorCode64003 => r#"## ERROR - 64003

### Pay: Invalid charge amount.

 ## Error - 64003

### Pay: Invalid charge amount.

A 64003 Error is an indication that the charge amount is invalid. 

#### Possible Causes
* The chargeAmount attribute has a number of decimals that does not conform to the currency type. For example, more than 2 decimals were provided for US Dollars .
* chargeAmount attribute is negative.

#### Possible Solutions
* Make sure the charge amount has correct number of decimal values.
* Make sure the charge amount is a positive value."#,
            TwilioProgrammableVoiceError::ErrorCode64109 => r#"## ERROR - 64109

### ConversationRelay: Concurrency limit reached

The concurrency limit for ConversationRelay was reached for this account The call was rejected because you reached the maximum limit of concurrent calls available for your account.

#### Possible Causes
The allowed concurrencylimit on your account was reached.

#### Possible Solutions
If needed, reach out to Twilio to see if they can increase your concurrency limit."#,
            TwilioProgrammableVoiceError::ErrorCode16027 => r#"## ERROR - 16027

### Participant to be whispered is on hold

 The participant to be whispered is on hold, so the coach to the participant cannot be added to the conference.

#### Possible Causes
The participant to be whispered is on hold.

#### Possible Solutions
Validate that this call is connecting to the correct conference and the call SID specified is also in the conference."#,
            TwilioProgrammableVoiceError::ErrorCode31484 => r#"## ERROR - 31484

### Address Incomplete

 The provided phone number is malformed.

#### Possible Causes
* The outbound call was made with a phone number that has an invalid format.

#### Possible Solutions
* Ensure the phone number dialed is formatted correctly."#,
            TwilioProgrammableVoiceError::ErrorCode31210 => r#"## ERROR - 31210

### Call Message Event Type is invalid.

 The Message Type used for sending Call Message Event is invalid.
Please see Twilio's <a href="/docs/voice/sdks/call-message-events/"> Voice SDK Call Message Events Documentation</a> for more information on valid Message Type.


#### Possible Causes
The Message Type used for sending Call Message Event is invalid.

#### Possible Solutions
* Verify that Message Type being sent with Call Message Event is a value supported by Twilio Programmable Voice
* Use one of the <a href="/docs/voice/sdks/call-message-events/">supported Message Types</a> to call a Twilio function."#,
            TwilioProgrammableVoiceError::ErrorCode16112 => r#"## ERROR - 16112

### Voice Recording: Upload file to external AWS S3 bucket failed (Access Denied)

 Twilio failed to upload the recording file to the external AWS S3 bucket specified in Voice Settings due to access denied. The recording file has been deleted, and the resource has been marked as “failed”.

#### Possible Causes
* AWS credentials does not have enough permissions to upload to the bucket
* The external S3 bucket has SSE-S3 enabled, only SSE-KMS is supported

#### Possible Solutions
* Ensure AWS S3 bucket credentials are valid and have enough permission to access
* If SSE-S3 is enabled, please consider switching to SSE-KMS
* Disable external storage"#,
            TwilioProgrammableVoiceError::ErrorCode13216 => r#"## WARNING - 13216

### Invalid timeLimit value

timeLimit must be a positive integer, in seconds. See the <a href='/docs/api/twiml/dial#timeLimit'>Dial Verb</a> API Reference for more information. ## Warning - 13216 

### Invalid timeLimit value

timeLimit must be a positive integer, in seconds. See the <a href='/docs/api/twiml/dial#timeLimit'>Dial Verb</a> API Reference for more information.







#### Possible Causes
timeLimit must be a positive integer, in seconds.

#### Possible Solutions
See the <a href='/docs/api/twiml/dial#timeLimit'>Dial Verb</a> API Reference for more information."#,
            TwilioProgrammableVoiceError::ErrorCode14218 => r#"## ERROR - 14218

### Dial->Queue: Could not update worker to provided activity

 Unable to update external user queue

#### Possible Causes
Unable to update the external user queue

#### Possible Solutions
Check network or connectivity on external queue"#
        }
    }
}
