// AUTO-GENERATED. DO NOT MODIFY. YOUR CHANGES WILL BE OVER-WRITTEN
use crate::products::TwilioProgrammableVoiceError;
use standard_error::traits::StandardErrorMessageTrait;
use std::borrow::Cow;

impl StandardErrorMessageTrait for TwilioProgrammableVoiceError {
    fn message(&self) -> Cow<'static, str> {
        match self {
            TwilioProgrammableVoiceError::ErrorCode11750 => r#"TwiML response body too large"#.into(),
            TwilioProgrammableVoiceError::ErrorCode16026 => r#"Participant label is in use by another participant"#.into(),
            TwilioProgrammableVoiceError::ErrorCode13802 => r#"Dial: No referUrl attribute specified"#.into(),
            TwilioProgrammableVoiceError::ErrorCode16002 => r#"Failed to validate conference attributes"#.into(),
            TwilioProgrammableVoiceError::ErrorCode16003 => r#"Could not recognize conference sid or friendly name"#.into(),
            TwilioProgrammableVoiceError::ErrorCode64106 => r#"ConversationRelay: Invalid argument"#.into(),
            TwilioProgrammableVoiceError::ErrorCode64017 => r#"Pay: BankAccountType Parameter not supported with PaymentMethod = "credit-card""#.into(),
            TwilioProgrammableVoiceError::ErrorCode32702 => r#"Voice User-Defined Message: Invalid Content"#.into(),
            TwilioProgrammableVoiceError::ErrorCode32652 => r#"Real-time Transcriptions: Unsupported <Config> attribute(s) in TwiML"#.into(),
            TwilioProgrammableVoiceError::ErrorCode64009 => r#"Pay: Twilio is no longer authorized to initiate transactions on your behalf."#.into(),
            TwilioProgrammableVoiceError::ErrorCode64016 => r#"Pay: Invalid Action URL"#.into(),
            TwilioProgrammableVoiceError::ErrorCode37000 => r#"WhatsApp Voice: Call permission not granted by consumer"#.into(),
            TwilioProgrammableVoiceError::ErrorCode13342 => r#"Gather: Invalid config for Google STT V2 provider"#.into(),
            TwilioProgrammableVoiceError::ErrorCode21302 => r#"Approaching application creation limit "#.into(),
            TwilioProgrammableVoiceError::ErrorCode31429 => r#"Too Many Requests"#.into(),
            TwilioProgrammableVoiceError::ErrorCode13801 => r#"Refer not allowed on non-SIP call legs"#.into(),
            TwilioProgrammableVoiceError::ErrorCode31904 => r#"Stream - WebSocket - Host Unreachable"#.into(),
            TwilioProgrammableVoiceError::ErrorCode32653 => r#"Real-time Transcriptions: Internal Error"#.into(),
            TwilioProgrammableVoiceError::ErrorCode64002 => r#"Pay: Service unavailable."#.into(),
            TwilioProgrammableVoiceError::ErrorCode31922 => r#"Stream - WebSocket - URL Schema Not Supported"#.into(),
            TwilioProgrammableVoiceError::ErrorCode16020 => r#"Conference is full"#.into(),
            TwilioProgrammableVoiceError::ErrorCode31901 => r#"Stream - WebSocket - Connection Timeout"#.into(),
            TwilioProgrammableVoiceError::ErrorCode31940 => r#"Stream - Invalid connectorName attribute in TwiML."#.into(),
            TwilioProgrammableVoiceError::ErrorCode17000 => r#"Forbidden to access data"#.into(),
            TwilioProgrammableVoiceError::ErrorCode13328 => r#"Gather: Invalid maxSpeechTime value"#.into(),
            TwilioProgrammableVoiceError::ErrorCode13332 => r#"Gather: Invalid bargeIn value"#.into(),
            TwilioProgrammableVoiceError::ErrorCode64011 => r#"Pay: Connector does not support the requested currency."#.into(),
            TwilioProgrammableVoiceError::ErrorCode64005 => r#"Pay: Connector does not support tokenization."#.into(),
            TwilioProgrammableVoiceError::ErrorCode13335 => r#"Gather: speechTimeout auto cannot be used with model default"#.into(),
            TwilioProgrammableVoiceError::ErrorCode31952 => r#"Stream Extension not found: "#.into(),
            TwilioProgrammableVoiceError::ErrorCode16099 => r#"Unexpected conference status"#.into(),
            TwilioProgrammableVoiceError::ErrorCode32701 => r#"Voice User-Defined Message: Invalid Content-Type"#.into(),
            TwilioProgrammableVoiceError::ErrorCode13330 => r#"Gather: Invalid hints value"#.into(),
            TwilioProgrammableVoiceError::ErrorCode31409 => r#"Conflict"#.into(),
            TwilioProgrammableVoiceError::ErrorCode16102 => r#"Voice Recording: Unavailable because recording is silent"#.into(),
            TwilioProgrammableVoiceError::ErrorCode31009 => r#"Transport error"#.into(),
            TwilioProgrammableVoiceError::ErrorCode16101 => r#"Voice Recording : Unavailable because duration is too short"#.into(),
            TwilioProgrammableVoiceError::ErrorCode15004 => r#"Action Callback URL must be an absolute URL when using TwiML to update in-progress calls"#.into(),
            TwilioProgrammableVoiceError::ErrorCode13804 => r#"AddOns are not supported in this realm"#.into(),
            TwilioProgrammableVoiceError::ErrorCode16106 => r#"Voice Recording: Unavailable due to internal encryption error"#.into(),
            TwilioProgrammableVoiceError::ErrorCode31103 => r#"Length of parameters cannot exceed MAX_PARAM_LENGTH."#.into(),
            TwilioProgrammableVoiceError::ErrorCode32019 => r#"Twiml and Voice URL are both set. Using Voice URL."#.into(),
            TwilioProgrammableVoiceError::ErrorCode21263 => r#"Invalid Answering Machine Detection Parameters"#.into(),
            TwilioProgrammableVoiceError::ErrorCode31502 => r#"Bad Gateway"#.into(),
            TwilioProgrammableVoiceError::ErrorCode31206 => r#"Rate exceeded authorized limit."#.into(),
            TwilioProgrammableVoiceError::ErrorCode16011 => r#"Conference Event: Error Response to Callback URL"#.into(),
            TwilioProgrammableVoiceError::ErrorCode64023 => r#"Pay: Invalid Test Bank Account Number"#.into(),
            TwilioProgrammableVoiceError::ErrorCode31408 => r#"Request Timeout"#.into(),
            TwilioProgrammableVoiceError::ErrorCode64018 => r#"Pay: Value needed for either Capture or Status parameters"#.into(),
            TwilioProgrammableVoiceError::ErrorCode53404 => r#"No supported codec"#.into(),
            TwilioProgrammableVoiceError::ErrorCode16023 => r#"Dial->Conference: Invalid participant label, must not exceed 128 characters, must not be a CallSid, must not contain '/'"#.into(),
            TwilioProgrammableVoiceError::ErrorCode31209 => r#"Reconnect attempt error."#.into(),
            TwilioProgrammableVoiceError::ErrorCode13805 => r#"Trial account call duration exceeded 10 minute limit"#.into(),
            TwilioProgrammableVoiceError::ErrorCode16022 => r#"Conference does not exist or is completed"#.into(),
            TwilioProgrammableVoiceError::ErrorCode64010 => r#"Pay: Payment Gateway rejected token creation."#.into(),
            TwilioProgrammableVoiceError::ErrorCode16025 => r#"Dial->Conference: Participant label is in use by another participant"#.into(),
            TwilioProgrammableVoiceError::ErrorCode13331 => r#"Gather: Invalid language value"#.into(),
            TwilioProgrammableVoiceError::ErrorCode31402 => r#"UserMedia Acquisition Failed"#.into(),
            TwilioProgrammableVoiceError::ErrorCode64019 => r#"Pay: Required payment information incomplete"#.into(),
            TwilioProgrammableVoiceError::ErrorCode17009 => r#"Internal Server Error"#.into(),
            TwilioProgrammableVoiceError::ErrorCode31211 => r#"Call is not in the expected state."#.into(),
            TwilioProgrammableVoiceError::ErrorCode64006 => r#"Pay: Connector does not support token type."#.into(),
            TwilioProgrammableVoiceError::ErrorCode32021 => r#"SHAKEN/STIR call verification failed"#.into(),
            TwilioProgrammableVoiceError::ErrorCode31604 => r#"Does Not Exist Anywhere"#.into(),
            TwilioProgrammableVoiceError::ErrorCode32500 => r#"Voice Conversation: Generic error."#.into(),
            TwilioProgrammableVoiceError::ErrorCode32101 => r#"SIP: Invalid phone number"#.into(),
            TwilioProgrammableVoiceError::ErrorCode64014 => r#"Pay: ECP/ACH requires AVSName Parameter in the <Pay> verb."#.into(),
            TwilioProgrammableVoiceError::ErrorCode17001 => r#"Completed summary for this call wasn't found"#.into(),
            TwilioProgrammableVoiceError::ErrorCode32009 => r#"The user you tried to dial is not registered with the corresponding SIP Domain"#.into(),
            TwilioProgrammableVoiceError::ErrorCode64004 => r#"Pay: Invalid paymentConnector attribute in TwiML."#.into(),
            TwilioProgrammableVoiceError::ErrorCode64022 => r#"Pay: Invalid Test Card Number"#.into(),
            TwilioProgrammableVoiceError::ErrorCode31002 => r#"Connection declined."#.into(),
            TwilioProgrammableVoiceError::ErrorCode13247 => r#"Dial: Invalid From number (caller ID)"#.into(),
            TwilioProgrammableVoiceError::ErrorCode13257 => r#"Invalid transcribeCallback URL"#.into(),
            TwilioProgrammableVoiceError::ErrorCode31202 => r#"Signature validation failed."#.into(),
            TwilioProgrammableVoiceError::ErrorCode31212 => r#"Call Message Event Payload size exceeded authorized limit."#.into(),
            TwilioProgrammableVoiceError::ErrorCode32503 => r#"Voice Conversation: Callback event response error."#.into(),
            TwilioProgrammableVoiceError::ErrorCode13329 => r#"Gather: Invalid partialResultCallbackMethod value"#.into(),
            TwilioProgrammableVoiceError::ErrorCode64105 => r#"ConversationRelay: Websocket ended"#.into(),
            TwilioProgrammableVoiceError::ErrorCode32601 => r#"Virtual Agent: Provider Error"#.into(),
            TwilioProgrammableVoiceError::ErrorCode64108 => r#"ConversationRelay: RTP Timeout"#.into(),
            TwilioProgrammableVoiceError::ErrorCode32606 => r#"Virtual Agent: Resume Error"#.into(),
            TwilioProgrammableVoiceError::ErrorCode31920 => r#"Stream - WebSocket - Handshake Error"#.into(),
            TwilioProgrammableVoiceError::ErrorCode17007 => r#"Voice Insights Advanced Features not enabled"#.into(),
            TwilioProgrammableVoiceError::ErrorCode13333 => r#"Gather: Invalid profanityFilter value"#.into(),
            TwilioProgrammableVoiceError::ErrorCode31530 => r#"DNS Resolution Error"#.into(),
            TwilioProgrammableVoiceError::ErrorCode13227 => r#"Geo Permission configuration is not permitting call"#.into(),
            TwilioProgrammableVoiceError::ErrorCode32007 => r#"SIP: Too many endpoints/bindings for the Address-of-record (AOR)"#.into(),
            TwilioProgrammableVoiceError::ErrorCode31104 => r#"Invalid bridge token."#.into(),
            TwilioProgrammableVoiceError::ErrorCode31208 => r#"User denied access to microphone."#.into(),
            TwilioProgrammableVoiceError::ErrorCode31910 => r#"Stream - WebSocket - SSL Protocol Error"#.into(),
            TwilioProgrammableVoiceError::ErrorCode31953 => r#"Stream - Media - RTP timeout"#.into(),
            TwilioProgrammableVoiceError::ErrorCode16024 => r#"Invalid participant label, must not exceed 128 characters, must not be a CallSid, must not contain '/'"#.into(),
            TwilioProgrammableVoiceError::ErrorCode32650 => r#"Real-time Transcriptions: Configuration Error"#.into(),
            TwilioProgrammableVoiceError::ErrorCode17008 => r#"Internal Server Error - Query Timeout"#.into(),
            TwilioProgrammableVoiceError::ErrorCode32106 => r#"SIP: Authentication Error"#.into(),
            TwilioProgrammableVoiceError::ErrorCode32014 => r#"Call is terminated because of no audio received"#.into(),
            TwilioProgrammableVoiceError::ErrorCode32651 => r#"Real-time Transcriptions: Provider Error"#.into(),
            TwilioProgrammableVoiceError::ErrorCode13326 => r#"Gather: Invalid input value"#.into(),
            TwilioProgrammableVoiceError::ErrorCode31426 => r#"Upgrade Required"#.into(),
            TwilioProgrammableVoiceError::ErrorCode13750 => r#"Twiml verb not supported by this API version."#.into(),
            TwilioProgrammableVoiceError::ErrorCode13240 => r#"Dial->Conference: Invalid Whisper SID"#.into(),
            TwilioProgrammableVoiceError::ErrorCode16110 => r#"Internal failure when bulk deleting recordings from your account"#.into(),
            TwilioProgrammableVoiceError::ErrorCode13220 => r#"Dial: Invalid ringTone value"#.into(),
            TwilioProgrammableVoiceError::ErrorCode21301 => r#"Cannot create application: application limit exceeded"#.into(),
            TwilioProgrammableVoiceError::ErrorCode31504 => r#"Gateway Timeout"#.into(),
            TwilioProgrammableVoiceError::ErrorCode31930 => r#"Stream - Media - Buffer Overflow"#.into(),
            TwilioProgrammableVoiceError::ErrorCode37001 => r#"WhatsApp Voice: Outbound calls are not supported in this region"#.into(),
            TwilioProgrammableVoiceError::ErrorCode32603 => r#"Virtual Agent: Unsupported <Config> attribute(s) in TwiML"#.into(),
            TwilioProgrammableVoiceError::ErrorCode17002 => r#"This call ended more than 30 days ago"#.into(),
            TwilioProgrammableVoiceError::ErrorCode32008 => r#"SIP: Registration per second limit reached"#.into(),
            TwilioProgrammableVoiceError::ErrorCode32711 => r#"Voice User-Defined Message: Request to subscription callback URL encountered error "#.into(),
            TwilioProgrammableVoiceError::ErrorCode16010 => r#"Conference Event: Internal Twilio Error"#.into(),
            TwilioProgrammableVoiceError::ErrorCode16108 => r#"Voice Recording: Request failed due to concurrent recordings"#.into(),
            TwilioProgrammableVoiceError::ErrorCode13521 => r#"`<Say>` element character limits exceeded"#.into(),
            TwilioProgrammableVoiceError::ErrorCode16113 => r#"Voice Recording: Cannot download a dual-channel presentation of a mono recording"#.into(),
            TwilioProgrammableVoiceError::ErrorCode15009 => r#"Internal Server Error"#.into(),
            TwilioProgrammableVoiceError::ErrorCode10005 => r#"Voice calling has been disabled for this account"#.into(),
            TwilioProgrammableVoiceError::ErrorCode64104 => r#"ConversationRelay: Max call duration reached"#.into(),
            TwilioProgrammableVoiceError::ErrorCode32220 => r#"Specifying an edge is not allowed when dialing SIP registered endpoints"#.into(),
            TwilioProgrammableVoiceError::ErrorCode13340 => r#"Gather: Degraded Speech "#.into(),
            TwilioProgrammableVoiceError::ErrorCode17400 => r#"Invalid query parameter"#.into(),
            TwilioProgrammableVoiceError::ErrorCode32215 => r#"Dial failure calling a SIP Domain without specifying a region"#.into(),
            TwilioProgrammableVoiceError::ErrorCode13430 => r#"Play: Invalid DTMF String"#.into(),
            TwilioProgrammableVoiceError::ErrorCode22005 => r#"Call Queue Full"#.into(),
            TwilioProgrammableVoiceError::ErrorCode31486 => r#"Busy Here"#.into(),
            TwilioProgrammableVoiceError::ErrorCode14226 => r#"TaskRouter Enqueue not supported in this realm"#.into(),
            TwilioProgrammableVoiceError::ErrorCode32654 => r#"Real-time Transcriptions: PCI Error"#.into(),
            TwilioProgrammableVoiceError::ErrorCode31008 => r#"Call cancelled"#.into(),
            TwilioProgrammableVoiceError::ErrorCode31000 => r#"Generic error"#.into(),
            TwilioProgrammableVoiceError::ErrorCode31923 => r#"Stream - WebSocket - Malformed URL"#.into(),
            TwilioProgrammableVoiceError::ErrorCode13327 => r#"Gather: Invalid speechTimeout value"#.into(),
            TwilioProgrammableVoiceError::ErrorCode16105 => r#"Voice Recording: Unavailable due to no valid public keys"#.into(),
            TwilioProgrammableVoiceError::ErrorCode16028 => r#"Participant to be whispered is not present in the conference"#.into(),
            TwilioProgrammableVoiceError::ErrorCode13223 => r#"Dial: Invalid phone number format"#.into(),
            TwilioProgrammableVoiceError::ErrorCode64107 => r#"ConversationRelay: Invalid Message Received"#.into(),
            TwilioProgrammableVoiceError::ErrorCode31207 => r#"JWT token expiration too long."#.into(),
            TwilioProgrammableVoiceError::ErrorCode13218 => r#"Dial: Invalid sequential value"#.into(),
            TwilioProgrammableVoiceError::ErrorCode64008 => r#"Pay: Payment Gateway rejected charge creation."#.into(),
            TwilioProgrammableVoiceError::ErrorCode13238 => r#"Dial->Conference: Invalid Verb used in waitUrl, holdUrl, or announceUrl TwiML"#.into(),
            TwilioProgrammableVoiceError::ErrorCode64007 => r#"Pay: Connector does not support creating charge."#.into(),
            TwilioProgrammableVoiceError::ErrorCode13239 => r#"Dial->Conference: Invalid Trim Value"#.into(),
            TwilioProgrammableVoiceError::ErrorCode31600 => r#"Busy Everywhere"#.into(),
            TwilioProgrammableVoiceError::ErrorCode14219 => r#"TaskRouter Dial Queue not supported in this realm"#.into(),
            TwilioProgrammableVoiceError::ErrorCode31480 => r#"Temporarily Unavailable"#.into(),
            TwilioProgrammableVoiceError::ErrorCode13810 => r#"Reject: Invalid cause"#.into(),
            TwilioProgrammableVoiceError::ErrorCode31487 => r#"Request Terminated"#.into(),
            TwilioProgrammableVoiceError::ErrorCode32015 => r#"Call is terminated due to exceeding maximum call duration"#.into(),
            TwilioProgrammableVoiceError::ErrorCode32212 => r#"SIP: Registration Authentication problem"#.into(),
            TwilioProgrammableVoiceError::ErrorCode31903 => r#"Stream - WebSocket - Connection Broken Pipe"#.into(),
            TwilioProgrammableVoiceError::ErrorCode32505 => r#"Voice Conversation: Invalid data inside existing conversation."#.into(),
            TwilioProgrammableVoiceError::ErrorCode31500 => r#"Internal Server Error"#.into(),
            TwilioProgrammableVoiceError::ErrorCode64103 => r#"ConversationRelay: Internal Server Error"#.into(),
            TwilioProgrammableVoiceError::ErrorCode13511 => r#"Say: Invalid voice value"#.into(),
            TwilioProgrammableVoiceError::ErrorCode32105 => r#"SIP: Invalid contact header"#.into(),
            TwilioProgrammableVoiceError::ErrorCode31400 => r#"Bad Request"#.into(),
            TwilioProgrammableVoiceError::ErrorCode31404 => r#"Not Found"#.into(),
            TwilioProgrammableVoiceError::ErrorCode64015 => r#"Pay: `<Pay>` verb is missing a needed Parameter"#.into(),
            TwilioProgrammableVoiceError::ErrorCode31902 => r#"Stream - WebSocket - Connection Refused"#.into(),
            TwilioProgrammableVoiceError::ErrorCode32602 => r#"Virtual Agent: Invalid Connector"#.into(),
            TwilioProgrammableVoiceError::ErrorCode13321 => r#"Gather -> Say: Invalid voice value"#.into(),
            TwilioProgrammableVoiceError::ErrorCode31102 => r#"Authorization token missing in request."#.into(),
            TwilioProgrammableVoiceError::ErrorCode31941 => r#"Stream - Invalid Track configuration"#.into(),
            TwilioProgrammableVoiceError::ErrorCode13338 => r#"Gather: Invalid actionOnEmptyResult value"#.into(),
            TwilioProgrammableVoiceError::ErrorCode64001 => r#"Pay: Configuration Error"#.into(),
            TwilioProgrammableVoiceError::ErrorCode32401 => r#"BYOC Trunk routing failure - failover to Twilio default routing."#.into(),
            TwilioProgrammableVoiceError::ErrorCode31205 => r#"JWT token expired."#.into(),
            TwilioProgrammableVoiceError::ErrorCode31003 => r#"Connection timeout"#.into(),
            TwilioProgrammableVoiceError::ErrorCode21234 => r#"Invalid Machine Detection configuration value"#.into(),
            TwilioProgrammableVoiceError::ErrorCode31105 => r#"Invalid client name"#.into(),
            TwilioProgrammableVoiceError::ErrorCode32504 => r#"Voice Conversation: Incomplete Conversation."#.into(),
            TwilioProgrammableVoiceError::ErrorCode21262 => r#"No AMD status callback URL provided"#.into(),
            TwilioProgrammableVoiceError::ErrorCode32700 => r#"Voice User-Defined Message: Internal Twilio Error"#.into(),
            TwilioProgrammableVoiceError::ErrorCode32002 => r#"SIP: Dial failure - Twilio SIP Domain not found"#.into(),
            TwilioProgrammableVoiceError::ErrorCode64110 => r#"ConversationRelay: Account has been opted out"#.into(),
            TwilioProgrammableVoiceError::ErrorCode21300 => r#"Invalid BYOC trunk SID"#.into(),
            TwilioProgrammableVoiceError::ErrorCode31203 => r#"No valid account."#.into(),
            TwilioProgrammableVoiceError::ErrorCode31950 => r#"Stream - Protocol - Malformed Message"#.into(),
            TwilioProgrammableVoiceError::ErrorCode21215 => r#"Geo Permission configuration is not permitting call"#.into(),
            TwilioProgrammableVoiceError::ErrorCode13621 => r#"Invalid 'recordingStatusCallbackEvent'"#.into(),
            TwilioProgrammableVoiceError::ErrorCode32506 => r#"Voice Conversation: Failed to get worker assigned to a newly created Conversation."#.into(),
            TwilioProgrammableVoiceError::ErrorCode31603 => r#"Decline"#.into(),
            TwilioProgrammableVoiceError::ErrorCode21216 => r#"API: Call blocked by Twilio"#.into(),
            TwilioProgrammableVoiceError::ErrorCode31302 => r#"Unsupported Cancel Message Error"#.into(),
            TwilioProgrammableVoiceError::ErrorCode64021 => r#"Pay: Invalid Operation"#.into(),
            TwilioProgrammableVoiceError::ErrorCode13803 => r#"SMS verb not supported in this realm"#.into(),
            TwilioProgrammableVoiceError::ErrorCode32710 => r#"Voice User-Defined Message: Subscription Callback Internal Error"#.into(),
            TwilioProgrammableVoiceError::ErrorCode64013 => r#"Pay: Connector does not support supplied paymentMethod attribute."#.into(),
            TwilioProgrammableVoiceError::ErrorCode13619 => r#"Record: Transcription feature not available for this type of recording."#.into(),
            TwilioProgrammableVoiceError::ErrorCode31005 => r#"Connection error"#.into(),
            TwilioProgrammableVoiceError::ErrorCode32604 => r#"Virtual Agent: Internal Error"#.into(),
            TwilioProgrammableVoiceError::ErrorCode32013 => r#"SIP: Parent account SIP Interface CPS limit exceeded"#.into(),
            TwilioProgrammableVoiceError::ErrorCode31951 => r#"Stream - Protocol - Invalid Message"#.into(),
            TwilioProgrammableVoiceError::ErrorCode32605 => r#"Virtual Agent: PCI Error"#.into(),
            TwilioProgrammableVoiceError::ErrorCode16104 => r#"Voice Recording: Unavailable due to encryption failure"#.into(),
            TwilioProgrammableVoiceError::ErrorCode31204 => r#"Invalid JWT token."#.into(),
            TwilioProgrammableVoiceError::ErrorCode32022 => r#"ACK not received from your SIP endpoint"#.into(),
            TwilioProgrammableVoiceError::ErrorCode64012 => r#"Pay: Payment Gateway rejected the card."#.into(),
            TwilioProgrammableVoiceError::ErrorCode31301 => r#"Registration error"#.into(),
            TwilioProgrammableVoiceError::ErrorCode32210 => r#"SIP: Register not supported"#.into(),
            TwilioProgrammableVoiceError::ErrorCode31960 => r#"Stream - Quota exceeded"#.into(),
            TwilioProgrammableVoiceError::ErrorCode53401 => r#"Server is unable to create or apply a local media description"#.into(),
            TwilioProgrammableVoiceError::ErrorCode64024 => r#"Pay: Connector Instance Not Approved"#.into(),
            TwilioProgrammableVoiceError::ErrorCode31106 => r#"Invalid data"#.into(),
            TwilioProgrammableVoiceError::ErrorCode14300 => r#"Start: Invalid nested noun value"#.into(),
            TwilioProgrammableVoiceError::ErrorCode64020 => r#"Pay: Invalid Parameter Value"#.into(),
            TwilioProgrammableVoiceError::ErrorCode64101 => r#"ConversationRelay: Invalid Parameter"#.into(),
            TwilioProgrammableVoiceError::ErrorCode32703 => r#"Voice User-Defined Message: Content body exceeded max length"#.into(),
            TwilioProgrammableVoiceError::ErrorCode64102 => r#"ConversationRelay: Unable to Connect to Websocket URL"#.into(),
            TwilioProgrammableVoiceError::ErrorCode32214 => r#"SIP: Invalid <Dial><Sip> "#.into(),
            TwilioProgrammableVoiceError::ErrorCode32221 => r#"Dialing SIP Endpoint failure - No devices registered in specified edge"#.into(),
            TwilioProgrammableVoiceError::ErrorCode16109 => r#"Voice Recording: Cannot fetch .mp3 encrypted recording"#.into(),
            TwilioProgrammableVoiceError::ErrorCode31900 => r#"Stream - Unknown Error"#.into(),
            TwilioProgrammableVoiceError::ErrorCode32400 => r#"BYOC Trunk routing failure - failover routing disabled."#.into(),
            TwilioProgrammableVoiceError::ErrorCode16107 => r#"Voice Recording: Encrypted with alternate public key"#.into(),
            TwilioProgrammableVoiceError::ErrorCode31403 => r#"Forbidden"#.into(),
            TwilioProgrammableVoiceError::ErrorCode32223 => r#"There is no username in the SIP URI when calling a SIP registered endpoint"#.into(),
            TwilioProgrammableVoiceError::ErrorCode13513 => r#"Say: Invalid rate value"#.into(),
            TwilioProgrammableVoiceError::ErrorCode32600 => r#"Virtual Agent: Configuration Error"#.into(),
            TwilioProgrammableVoiceError::ErrorCode31931 => r#"Stream - Media - Media Discarded"#.into(),
            TwilioProgrammableVoiceError::ErrorCode31924 => r#"Stream - Websocket - Protocol Error"#.into(),
            TwilioProgrammableVoiceError::ErrorCode16021 => r#"Failed to join conference due to account concurrency limit exceeded"#.into(),
            TwilioProgrammableVoiceError::ErrorCode31100 => r#"Malformed request"#.into(),
            TwilioProgrammableVoiceError::ErrorCode31201 => r#"Twilio Client: Error occurred while accessing microphone"#.into(),
            TwilioProgrammableVoiceError::ErrorCode16000 => r#"Whisper Not Available on Twilio Conference"#.into(),
            TwilioProgrammableVoiceError::ErrorCode13219 => r#"Dial: Invalid answerOnBridge value"#.into(),
            TwilioProgrammableVoiceError::ErrorCode14207 => r#" Enqueue: The targeted queue reached max queue size"#.into(),
            TwilioProgrammableVoiceError::ErrorCode31101 => r#"Missing parameter array in request."#.into(),
            TwilioProgrammableVoiceError::ErrorCode13512 => r#"Gather element has an invalid "language" attribute value"#.into(),
            TwilioProgrammableVoiceError::ErrorCode31007 => r#"Twilio Client: Client version not supported"#.into(),
            TwilioProgrammableVoiceError::ErrorCode31503 => r#"Service Unavailable"#.into(),
            TwilioProgrammableVoiceError::ErrorCode13620 => r#"Record: Transcription not available for this recording"#.into(),
            TwilioProgrammableVoiceError::ErrorCode31001 => r#"Application not found."#.into(),
            TwilioProgrammableVoiceError::ErrorCode13337 => r#"Gather: callback must be over HTTPS when using gather with PCI compliance"#.into(),
            TwilioProgrammableVoiceError::ErrorCode13258 => r#"Dial->Sim not supported in this realm"#.into(),
            TwilioProgrammableVoiceError::ErrorCode32501 => r#"Voice Conversation: TwiML attributes validation error."#.into(),
            TwilioProgrammableVoiceError::ErrorCode13256 => r#"Invalid recordingStatusCallback URL"#.into(),
            TwilioProgrammableVoiceError::ErrorCode13214 => r#"Dial: Invalid callerId value"#.into(),
            TwilioProgrammableVoiceError::ErrorCode16111 => r#"Voice Recording: Upload file to external AWS S3 bucket failed (Invalid Configuration)"#.into(),
            TwilioProgrammableVoiceError::ErrorCode32655 => r#"Real-time Transcriptions: Intelligence Service Unreachable"#.into(),
            TwilioProgrammableVoiceError::ErrorCode16001 => r#"Conference is not bridged"#.into(),
            TwilioProgrammableVoiceError::ErrorCode13622 => r#"Record: invalid recordingTrack value"#.into(),
            TwilioProgrammableVoiceError::ErrorCode13334 => r#"Gather: Invalid model value"#.into(),
            TwilioProgrammableVoiceError::ErrorCode31401 => r#"UserMedia Permission Denied"#.into(),
            TwilioProgrammableVoiceError::ErrorCode13225 => r#"Dial: Call blocked by Twilio"#.into(),
            TwilioProgrammableVoiceError::ErrorCode22001 => r#"Call timed out"#.into(),
            TwilioProgrammableVoiceError::ErrorCode31942 => r#"Stream - Invalid connector configuration"#.into(),
            TwilioProgrammableVoiceError::ErrorCode31481 => r#"Call/Transaction Does Not Exist"#.into(),
            TwilioProgrammableVoiceError::ErrorCode32502 => r#"Voice Conversation: Callback event internal error."#.into(),
            TwilioProgrammableVoiceError::ErrorCode31006 => r#"Audio device error"#.into(),
            TwilioProgrammableVoiceError::ErrorCode13699 => r#"Record: Invalid trim value"#.into(),
            TwilioProgrammableVoiceError::ErrorCode31921 => r#"Stream - WebSocket - Close Error"#.into(),
            TwilioProgrammableVoiceError::ErrorCode32018 => r#"Twiml size exceeded maximum allowed value"#.into(),
            TwilioProgrammableVoiceError::ErrorCode64003 => r#"Pay: Invalid charge amount."#.into(),
            TwilioProgrammableVoiceError::ErrorCode64109 => r#"ConversationRelay: Concurrency limit reached"#.into(),
            TwilioProgrammableVoiceError::ErrorCode16027 => r#"Participant to be whispered is on hold"#.into(),
            TwilioProgrammableVoiceError::ErrorCode31484 => r#"Address Incomplete"#.into(),
            TwilioProgrammableVoiceError::ErrorCode31210 => r#"Call Message Event Type is invalid."#.into(),
            TwilioProgrammableVoiceError::ErrorCode16112 => r#"Voice Recording: Upload file to external AWS S3 bucket failed (Access Denied)"#.into(),
            TwilioProgrammableVoiceError::ErrorCode13216 => r#"Invalid timeLimit value"#.into(),
            TwilioProgrammableVoiceError::ErrorCode14218 => r#"Dial->Queue: Could not update worker to provided activity"#.into()
        }
    }
}
