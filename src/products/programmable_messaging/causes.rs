// AUTO-GENERATED. DO NOT MODIFY. YOUR CHANGES WILL BE OVER-WRITTEN
use crate::products::TwilioProgrammableMessagingError;
use standard_error::traits::StandardErrorCausesTrait;

impl StandardErrorCausesTrait for TwilioProgrammableMessagingError {
    fn causes(&self) -> Option<&'static str> {
        match self {
            TwilioProgrammableMessagingError::ErrorCode20504 => Some(r#"* If this message was being processed during an incident, it is possible that this message was affected. See [Twilio's status page](https://status.twilio.com/) for ongoing and historical incidents.
* There was an unrecoverable anomaly that occurred while processing this particular message."#),
            TwilioProgrammableMessagingError::ErrorCode30884 => Some(r#"The campaign cannot be approved because it found to be a known Spam or Phishing campaign."#),
            TwilioProgrammableMessagingError::ErrorCode35133 => Some(r#"The Messages array is either empty or contains too many items"#),
            TwilioProgrammableMessagingError::ErrorCode30758 => Some(r#"Missing Registration Authority"#),
            TwilioProgrammableMessagingError::ErrorCode21660 => Some(r#"The number you are using does not belong to the account whose credentials are present in the API request."#),
            TwilioProgrammableMessagingError::ErrorCode30897 => Some(r#"The campaign cannot be approved because it has disallowed content types, such as Loan Marketing, 3rd party debt collection, gambling, sweepstakes, stock alerts, cryptocurrency, risk investments, debt reduction, credit repair, 3rd party lead generation, federally illegal substances."#),
            TwilioProgrammableMessagingError::ErrorCode63036 => Some(r#"- Device is switched off
- Device does not have an internet connection
- Device is not RCS Business Messaging capable"#),
            TwilioProgrammableMessagingError::ErrorCode30447 => Some(r#"* The toll-free verification was submitted before the toll-free phone number completed porting to Twilio. 
* The toll-free phone number is not provisioned to Twilio."#),
            TwilioProgrammableMessagingError::ErrorCode30444 => Some(r#"The toll-free phone number cannot be verified because the submission indicated that the business is associated with or the toll-free verification submission indicated Fraud. "#),
            TwilioProgrammableMessagingError::ErrorCode30474 => Some(r#"The toll-free phone number cannot be verified because the ISV information was submitted instead of the end business. For ISVs or systems & software powering SMS: If your software enables other businesses to send SMS/MMS to their customers, it’s expected that the information provided represents the end-business (your customer) that is engaging with the opted-in consumer, since they have the relationship with the end consumer (the receiver of the messages). 

When you submit a Verification, you need to collect and disclose the information of the business. As a best practice, this should become part of your onboarding process and customer education. Note that there is no trial experience supported on toll-free phone numbers within the industry, so if you have a trial product that includes SMS/MMS, you must collect for a full verification to send messages. 

Submissions that are missing information for the end-business, or are populated with ISV/aggregator information may be rejected. Exceptions may apply when the use case clearly showcases that the ISV manages opt-in mechanisms and is the sole message content creator.

Note: If you are an ISV, prior to submitting a Toll-Free Verification request with Twilio, you will need to create a TrustHub primary Customer Profile for your business within your parent account and get it approved. A primary customer profile must have the business identity set to ISV, and then submitted and approved before registering end businesses. For step-by-step guidance, see [Create a Primary Customer Profile](https://www.twilio.com/docs/trust-hub/trusthub-rest-api/console-create-a-primary-customer-profile)."#),
            TwilioProgrammableMessagingError::ErrorCode36005 => Some(r#"* You attempted to upload a file that isn't of type .json.gz"#),
            TwilioProgrammableMessagingError::ErrorCode30440 => Some(r#"Unknown"#),
            TwilioProgrammableMessagingError::ErrorCode30901 => Some(r#"Reasons for this failure can be:

* Review was not completed
* Third parties involved in campaign registration did not respond within defined SLA"#),
            TwilioProgrammableMessagingError::ErrorCode30749 => Some(r#"An email address can not be used multiple times for Sole Proprietor Brand registrations. It is possible that the Starter customer profile bundle used for Sole Proprietor brand registration contains email address which has been used to register one or more Sole Proprietor Brands already. "#),
            TwilioProgrammableMessagingError::ErrorCode30888 => Some(r#"The campaign cannot be approved because you indicated that the campaign includes age-gated content, but an Age Gate is not present or is not acceptable on your website and/or opt-in policy."#),
            TwilioProgrammableMessagingError::ErrorCode21736 => Some(r#"* The Brand Contact Email provided is invalid or failed validation.
* Brand Contact Email is missing.
* The 2FA email sent was not actioned on or completed. "#),
            TwilioProgrammableMessagingError::ErrorCode11321 => Some(r#"Misconfigured URL for phone number "#),
            TwilioProgrammableMessagingError::ErrorCode30480 => Some(r#"The toll-free phone number cannot be verified because the business or message content was found to be associated with a known Phishing campaign. "#),
            TwilioProgrammableMessagingError::ErrorCode30750 => Some(r#"Each Sole Proprietor Brand registration requires a unique active wireless mobile phone number. It is possible that the Sole Proprietor A2P profile bundle used for brand registration contains a mobile number which has been used to register a Sole Proprietor Brand already. "#),
            TwilioProgrammableMessagingError::ErrorCode30994 => Some(r#"It's possible the brand associated with this campaign is awaiting 2FA email verification."#),
            TwilioProgrammableMessagingError::ErrorCode63041 => Some(r#"* Negative customer feedback (i.e. multiple customers reporting messages or blocking number)"#),
            TwilioProgrammableMessagingError::ErrorCode30890 => Some(r#"The campaign cannot be approved because it was indicated that you have a HELP message reply but it does not contain brand name, phone number, or email address."#),
            TwilioProgrammableMessagingError::ErrorCode11322 => Some(r#"Incorrect HTTP method"#),
            TwilioProgrammableMessagingError::ErrorCode63047 => Some(r#"- The web server hosting the media file is not sending Content-Type headers
- The web server hosting the media file is sending an invalid Content-Type header"#),
            TwilioProgrammableMessagingError::ErrorCode30473 => Some(r#"The toll-free phone number cannot be verified because the business website submitted is not accessible or available. The business website URL must pertain to the website the consumer is engaging with. In any of the following cases, the business website won’t be reviewable:
* The business website does not load or is not operational
* The business website is in a private state that requires a login/password"#),
            TwilioProgrammableMessagingError::ErrorCode30647 => Some(r#"**INVALID_CORRELATION_ID:** The correlation_id field is not a valid 32-character UUID. Each correlation_id should be a unique identifier, exactly 32 characters long, to map the response to the original request.

**INVALID_COUNTRY_ISO_CODE:** The country_iso_code is not a valid ISO 3166-1 alpha-2 country code. This field should be a 2-character uppercase string (e.g., "US" for the United States).

**INVALID_CONTACT_ID:** The contact_id field is not in the correct [E.164 format.](http://localhost:4200/docs/glossary/what-e164)

**INVALID_ZIP_CODE:** The zip_code field is not a valid postal code. The zip code should conform to the format used in the corresponding country as indicated by the country_iso_code.

**INTERNAL_SERVER_ERROR:** This error indicates an internal server issue that caused the upsert process to fail. Please retry the request."#),
            TwilioProgrammableMessagingError::ErrorCode216602 => Some(r#"The content type you are using is not available for the channel you are using"#),
            TwilioProgrammableMessagingError::ErrorCode30446 => Some(r#"The Opt-in workflow provided in the toll-free verification is not sufficient. Express Consent is required. Express Consent is more than collecting a phone number - it is a voluntary, informed end user choice for a specific purpose - it requires upfront disclosure and intentional action. Explicit opt-in leaves no ambiguity for why the customer gave the phone number, and for how they expect the company will communicate with them.  Every end business must have consent from each recipient they send a message to. Consumers may give permission over text, on a form, on a website, or verbally. Consumers may also give written permission. Listing “SMS” in your Terms of Service or Privacy Policy is not Express Consent. 

Note: If there is promotional messaging on this toll-free phone number, it requires express written consent.

Opt-in refers to the process of getting a consumer’s permission to send them text messages. According to TCPA law, businesses must have "express written consent" from the consumer before texting them. NonConsumer (A2P) Message Senders should: 

* Obtain a Consumer’s consent to receive messages generally; 
* Obtain a Consumer’s express written consent to specifically receive marketing messages; 
* Ensure that Consumers have the ability to revoke consent"#),
            TwilioProgrammableMessagingError::ErrorCode21737 => Some(r#"* Brand is missing a Brand Contact Email and has not completed the Brand 2FA process which will prevent the creation of new campaigns. "#),
            TwilioProgrammableMessagingError::ErrorCode30471 => Some(r#"Toll-Free phone number cannot be verified because the campaign provided an unsecure URL in the sample message. URLs sent in message content are required to support the https: protocol and the sample message needs to reflect that."#),
            TwilioProgrammableMessagingError::ErrorCode21731 => Some(r#"Your Brand has been suspended because of one or more violations against the US A2P 10DLC policies."#),
            TwilioProgrammableMessagingError::ErrorCode30460 => Some(r#"The toll-free phone number cannot be verified because the submission indicated that the business is associated with or the toll-free verification submission indicated a forbidden messaging category (Debt collection or forgiveness: Third party debt collection). Third party debt collection is not permitted, consent must be obtained directly from end-users. 

"Third-party debt collection" means originating from any party other than the one who is owed the debt. For example, a hospital could send messages regarding bills for its own patients, assuming they provided opt-in to receive that messaging.

Debt consolidation, debt reduction and credit repair programs are prohibited regardless if there is first-party consent. 

Note: While third party debt collection is not permitted, a debt collection business that has direct consent from a consumer to send related content may do so. "#),
            TwilioProgrammableMessagingError::ErrorCode36001 => Some(r#"* You provided an empty Broadcast SID
* You provided a Broadcast SID that does not exist"#),
            TwilioProgrammableMessagingError::ErrorCode30754 => Some(r#"You have used an invalid address for brand registration."#),
            TwilioProgrammableMessagingError::ErrorCode20410 => Some(r#"* You are  attempting to send messages via a discontinued API
* You are sending messages via the deprecated “SMS/Messages” endpoint
"#),
            TwilioProgrammableMessagingError::ErrorCode30646 => Some(r#"**INVALID_CORRELATION_ID:** The correlation_id field is not a valid 32-character UUID. Each correlation_id should be a unique identifier, exactly 32 characters long, to map the response to the original request.

**INVALID_SENDER_ID:** The sender_id field is invalid. It must be either a valid Messaging Service SID or a from phone number. Ensure the sender_id is correctly formatted.

**INVALID_CONTACT_ID:** The contact_id field is not in the correct [E.164 format.](http://localhost:4200/docs/glossary/what-e164)

**INVALID_STATUS:** The status field is invalid. It must be one of the following values: opt-in or opt-out.

**INVALID_SOURCE:** The source field is invalid. It must be one of the following values: website, offline, opt-in-message, opt-out-message, or others.

**INTERNAL_SERVER_ERROR:** The request failed due to an internal server error during the upsert process. Retry the request."#),
            TwilioProgrammableMessagingError::ErrorCode36004 => Some(r#"* You did not specify a key with the name "file" in the Broadcast Upload request body"#),
            TwilioProgrammableMessagingError::ErrorCode30610 => Some(r#"Marketing, promotional & other non essential messages are restricted under the local regulatory requirement (e.g.TCPA in US) to certain hours of the day. Specifically, these communications cannot be made before 8 a.m. or after 9 p.m. local time at the recipient's location. This restriction is intended to protect consumers from intrusive communications during times they are likely to be resting or engaged in personal activities."#),
            TwilioProgrammableMessagingError::ErrorCode21659 => Some(r#"* The number you are using is in the process of porting/hosting.
* You have provided an incorrect `From` number. 
* The number may be formatted incorrectly. Twilio accepts numbers in [E.164 format](https://www.twilio.com/docs/glossary/what-e164)
* If the number is a Short Code, it must be associated with the same country as the destination address.

"#),
            TwilioProgrammableMessagingError::ErrorCode30734 => Some(r#"Sole proprietor brand registration is not enabled for your account."#),
            TwilioProgrammableMessagingError::ErrorCode30475 => Some(r#"Your opt-in mechanism requires that the consumer opt-in to messaging as a requirement of using your service, which is not allowed.

Consent cannot be a requirement for your business’s service. A business may not combine collecting a consumer’s phone number with opting-in to receive marketing messaging. An explicit opt-in leaves no ambiguity for why the customer gave the phone number, and for how they expect the company will communicate with them. 

Opt-in refers to the process of getting a consumer’s permission to send them text messages. According to TCPA law, businesses must have "express written consent" from the consumer before texting them. NonConsumer (A2P) Message Senders should: 
* Obtain a Consumer’s consent to receive messages generally; 
* Obtain a Consumer’s express written consent to specifically receive marketing messages; and 
* Ensure that Consumers have the ability to revoke consent.

Note: If there is promotional messaging on this toll-free phone number, it requires express written consent."#),
            TwilioProgrammableMessagingError::ErrorCode30993 => Some(r#"It is possible that all the necessary elections for the Campaign's CNP migration process to complete, were not done in time. "#),
            TwilioProgrammableMessagingError::ErrorCode30903 => Some(r#"Reasons for this failure can be:

* Failure to meet the specified criteria for Sole Proprietor registration, entities with EINs should be registered as a Standard Brand.

* Incorrect or incomplete registration information provided during the brand registration process.

* Inconsistencies or discrepancies in the provided information."#),
            TwilioProgrammableMessagingError::ErrorCode30701 => Some(r#"The fields provided in the error message failed validations. Most string parameters are subject to min and/or max length validations. Some parameters are subject to additional country-specific validation rules, including business_registration_number, state, and postal_code. Please refer to the following table for validation details.

#### Validation Rules

|Property|Description|Validation|
|--- |--- |--- |
|friendly_name|Brand/Marketing/DBA name of the business|Max length 255|
|business_name|The legal name of the business|Max length 255|
|business_registration_number|Tax ID of the business|Max length 21|
|first_name|First name of business contact|Max length 100|
|last_name|Last name of business contact|Max length 100|
|phone_number|The support contact telephone in e.164 format. E.g. +12023339999|Max length 20|
|mobile_phone_number|Valid wireless telephone in e.164 format. E.g. +12023339999.|Max length 20|
|street|Street name and house number. E.g. 1000 Sunset Hill Road|Max length 100|
|city|City name|Max length 100|
|region|State, region or province. For the United States, please use 2 character codes. E.g. 'CA' for California.|Max length 20|
|postal_code|Zipcode or postal code. E.g. 21012|Max length 10|
|iso_country|2 letter ISO-2 country code. E.g. US, CA Length 2||
|email|The email address of support contact|Max length 100|
|stock_ticker|The stock symbol of the brand|Max length 10|
|stock_exchange|The stock exchange of the brand|Enumerated values: "NONE", "NASDAQ", "NYSE", "AMEX", "AMX", "ASX", "B3", "BME", "BSE", "FRA", "ICEX", "JPX", "JSE", "KRX", "LON", "NSE", "OMX", "SEHK", "SGX", "SSE", "STO", "SWX", "SZSE", "TSX", "TWSE", "VSE", "OTHER"|
|website_url|The website/online presence of the business|Max length 100|
|business_industry|The segment in which the business operates in|Enumerated values: "AUTOMOTIVE", "AGRICULTURE", "BANKING", "CONSUMER", "EDUCATION", "ENGINEERING", "ENERGY", "OIL_AND_GAS", "FAST_MOVING_CONSUMER_GOODS", "FINANCIAL", "FINTECH", "FOOD_AND_BEVERAGE", "GOVERNMENT", "HEALTHCARE", "HOSPITALITY", "INSURANCE", "LEGAL", "MANUFACTURING", "MEDIA", "ONLINE", "RAW_MATERIALS", "REAL_ESTATE", "RELIGION", "RETAIL", "JEWELRY", "TECHNOLOGY", "TELECOMMUNICATIONS", "TRANSPORTATION", "TRAVEL", "ELECTRONICS", "NOT_FOR_PROFIT"|
"#),
            TwilioProgrammableMessagingError::ErrorCode21267 => Some(r#"You are trying to use an Alphanumeric Sender ID on a Trial Account. Trial accounts [cannot send from Alphanumeric Sender IDs](https://www.twilio.com/docs/messaging/guides/how-to-use-your-free-trial-account#sending-sms-and-mms-messages)

"#),
            TwilioProgrammableMessagingError::ErrorCode21732 => Some(r#"Your request to create a Campaign has failed because there’s already an existing Campaign associated with this Sole Proprietor Brand."#),
            TwilioProgrammableMessagingError::ErrorCode30445 => Some(r#"The Toll-Free phone number cannot be verified because of invalid information. The Business Information such as business name or address that was submitted could not be verified or the website URL could be not validated because it is not accessible or available. Or the ISV contact information was provided instead of its customer’s contact information. "#),
            TwilioProgrammableMessagingError::ErrorCode21266 => Some(r#"You attempted to send an SMS from a Twilio number to itself (i.e. putting the same Twilio number in the `To` and `From` parameters)."#),
            TwilioProgrammableMessagingError::ErrorCode30757 => Some(r#"Missing Business Registration Number"#),
            TwilioProgrammableMessagingError::ErrorCode30456 => Some(r#"The toll-free phone number cannot be verified because the submission indicated that the business is associated with or the toll-free verification submission indicated a forbidden messaging category (SHAFT: Alcohol).

Alcohol traffic is allowed on Toll Free, Short Code, and Long Code in the US, as long as proper age gating procedures are in place. Age gating means that website users must input their date of birth. It cannot be a yes or no question.

All age-gated content into Canada must be blocked across Toll Free, Short Code, and Long Code. The only way to send age-gated traffic into Canada (even with proper age-gating) is to receive a special carrier exemption. Allowed age gated content in Canada include: pocket knives, lighters, and non-alcoholic beverages."#),
            TwilioProgrammableMessagingError::ErrorCode63051 => Some(r#"Violating the WhatsApp Terms of Service or Commerce Policy"#),
            TwilioProgrammableMessagingError::ErrorCode11100 => Some(r#"*   Bad URL for phone number configuration
*   Bad URL passed to outgoing call REST request
*   Bad URL in Play or Redirect Verb Body
*   Bad URL provided for a verb's action attribute
*   No URL provided in Record verb action attribute when modifying a live call
*   Unsupported characters in auth portion of URL"#),
            TwilioProgrammableMessagingError::ErrorCode63046 => Some(r#"WhatsApp has reported the template is now approved."#),
            TwilioProgrammableMessagingError::ErrorCode30908 => Some(r#"* *Missing Privacy Policy*: A compliant privacy policy was not located on the website provided or Message Flow during the review process.
* *Policy Inconsistencies*: Discrepancies in the provided privacy policy such as multiple privacy policies.
* *Mobile Information Sharing*: Privacy policy indicates end-user mobile information is shared with third parties/affiliates.
"#),
            TwilioProgrammableMessagingError::ErrorCode21703 => Some(r#"• You attempted to send a message to a United States or Canada mobile number, but you do not have any US/Canada numbers or short codes in your Messaging Service. US/Canada mobile numbers are not reachable from Twilio numbers from outside the US/Canada, due to limitations imposed by carriers.**

• You only have a short code number in your Messaging Service, and the recipient is not reachable from your short code (for example, the To number is from a different country than your short code, or is on a [carrier that does not support short code messaging](https://support.twilio.com/hc/en-us/articles/223182088-What-carriers-are-supported-on-Twilio-short-codes-)).

• You attempted to send an MMS message, but you do not have any US/Canada long code numbers or an [MMS-enabled short code](https://support.twilio.com/hc/en-us/articles/223134667-Sending-receiving-MMS-messages-with-short-codes) in your Messaging Service.**

• You only have an [Alphanumeric Sender ID](https://support.twilio.com/hc/en-us/articles/223181348-Getting-started-with-Alphanumeric-Sender-ID) in your Messaging Service, and the To number is in a [country where Alphanumeric Sender ID is unsupported](https://support.twilio.com/hc/en-us/articles/223133767-International-support-for-Alphanumeric-Sender-ID).

• You attempted to send a message to a United States mobile number, but you do not have any 10DLC numbers that are registered with a verified A2P Campaign in the Messaging Service. Only numbers registered with a verified A2P Campaign can be allowed to send to US based numbers. 

** If you have recently added the number to a Messaging Service and while the number has a “Pending Registration” status, you may be experiencing a known issue which disables the ability to benefit from a temporary exception Twilio has made available to sending messages from “Pending Registration” numbers.  You can reattempt to send from the impacted number by including the number in the "From" parameter of your API request instead of using the Messaging Service SID. However, this exception will sunset in the coming weeks.  For more information about the 21703 error in relation to numbers “Pending Registration”, and processing times to complete the Number Registration going forward, please review updated [A2P 10DLC Number Registration Best Practices](https://support.twilio.com/hc/en-us/articles/19622397178139-A2P-10DLC-Number-Registration-Best-Practices). To check the status of your numbers at any given time, you can use the Number Registration CSV report as described in [How do I check that I have completed US A2P 10DLC registration](https://support.twilio.com/hc/en-us/articles/4418081745179-How-do-I-check-that-I-have-completed-US-A2P-10DLC-registration-)."#),
            TwilioProgrammableMessagingError::ErrorCode30463 => Some(r#"The toll-free phone number cannot be verified because the submission indicated that the business is associated with or the toll-free verification submission indicated a forbidden messaging category (High-risk financial services: Stock alerts/stock platforms). 

No businesses that solely operate in stocks, investing, or cryptocurrency are allowed to send SMS traffic. If there is a mixed use case where that is a partial aspect of the business it may be approved based on the other use case content.

Note, however, that stock platforms are allowed a two-factor authentication (2FA) messaging use case. "#),
            TwilioProgrammableMessagingError::ErrorCode30895 => Some(r#"The campaign may involve direct lending or loan arrangements, and this was not accurately reflected in the content attribute during registration."#),
            TwilioProgrammableMessagingError::ErrorCode21635 => Some(r#"The `To` number you provided is a landline number."#),
            TwilioProgrammableMessagingError::ErrorCode30729 => Some(r#"Provided country code has been blacklisted and is not allowed for US A2P Brand registrations. Supported list of country codes are : PR,PS,PT,PW,PY,QA,AD,AE,AF,AG,AI,AL,AM,AO,AQ,AR,AS,AT,RE,AU,AW,AX,AZ,RO,BA,BB,RS,BD,RU,BE,BF,BG,RW,BH,BI,BJ,BL,BM,BN,BO,SA,SB,BQ,SC,BR,BS,SD,SE,BT,SG,BV,SH,BW,SI,SJ,BY,BZ,SK,SL,SM,SN,SO,CA,SR,CC,SS,ST,CD,CF,SV,CG,SX,CH,CI,SZ,CK,CL,CM,CN,CO,CR,TC,TD,TF,TG,CV,CW,TH,CX,TJ,CY,TK,CZ,TL,TM,TN,TO,TR,TT,DE,TV,TW,DJ,TZ,DK,DM,DO,UA,UG,DZ,UM,EC,US,EE,EG,EH,UY,UZ,VA,ER,VC,ES,VE,ET,VG,VI,VN,VU,FI,FJ,FK,FM,FO,FR,WF,GA,GB,WS,GD,GE,GF,GG,GH,GI,GL,GM,GN,GP,GQ,GR,GS,GT,XE,GU,GW,GY,XK,HK,HM,HN,HR,YE,HT,HU,YT,ID,IE,IL,IM,IN,IO,ZA,IQ,IS,IT,ZM,JE,ZW,JM,JO,JP,KE,KG,KH,KI,KM,KN,KR,KW,KY,KZ,LA,LB,LC,LI,LK,LR,LS,LT,LU,LV,LY,MA,MC,MD,ME,MF,MG,MH,MK,ML,MM,MN,MO,MP,MQ,MR,MS,MT,MU,MV,MW,MX,MY,MZ,NA,NC,NE,NF,NG,NI,NL,NO,NP,NR,NU,NZ,OM,PA,PE,PF,PG,PH,PK,PL,PM,PN"#),
            TwilioProgrammableMessagingError::ErrorCode30896 => Some(r#"* Opt-in message workflow does not meet the requirements for the specific campaign type.
* Consent is required but not adequately provided or maintained.
* Opt-in information is shared with third-party entities."#),
            TwilioProgrammableMessagingError::ErrorCode30906 => Some(r#"* Twilio identified compliance violations during the review process. The specific reason for rejection is provided in the CAMPAIGN_SHARE_DELETE webhook from TCR."#),
            TwilioProgrammableMessagingError::ErrorCode30907 => Some(r#"* *Mismatched Website Information*: The website URL submitted does not correspond to the Brand information or the campaign's intended use as registered.
* *Brand-Campaign Discrepancy*: There is a discrepancy between the Brand details and the campaign details on provided website.
"#),
            TwilioProgrammableMessagingError::ErrorCode30902 => Some(r#"Reasons for this failure can be:

* Third parties involved in campaign registration rejected the campaign registration request."#),
            TwilioProgrammableMessagingError::ErrorCode30441 => Some(r#"The toll-free phone number cannot be verified because the submission indicated that the business is associated with or the toll-free verification submission indicated a forbidden messaging category - (SHAFT: Sex). 

Twilio does not allow Hate speech, harassment, exploitative, abusive, or any communications that originate from a hate group."#),
            TwilioProgrammableMessagingError::ErrorCode30465 => Some(r#"The toll-free phone number cannot be verified because the submission indicated that the business is associated with or the toll-free verification submission indicated a forbidden messaging category (Get Rich Quick Schemes: Risk Investment). 


Use cases in this category pertain to minimal effort for maximum and/or guaranteed financial gains. These categories in the telecoms industry produce high consumer complaints and are not permissible on carrier routes.
"#),
            TwilioProgrammableMessagingError::ErrorCode35127 => Some(r#"You provided both a 'Messages' Body and 'ContentVariables' parameters in your request"#),
            TwilioProgrammableMessagingError::ErrorCode63045 => Some(r#"- The server that hosts the sample media file is not able to return the data"#),
            TwilioProgrammableMessagingError::ErrorCode30797 => Some(r#"* The submitted company is not a US entity. Only US entities qualify for the Government Entity Type. 
* The company’s submission as a Government Entity type could not be confirmed from independent sources. 
"#),
            TwilioProgrammableMessagingError::ErrorCode30885 => Some(r#"The campaign cannot be approved because it is considered Fraud or Deceptive Marketing."#),
            TwilioProgrammableMessagingError::ErrorCode30894 => Some(r#"The campaign cannot be approved because registration needs to be associated with the brand behind the campaign. If you are an ISV registering a direct offering, campaign description needs to indicate a direct offering."#),
            TwilioProgrammableMessagingError::ErrorCode36009 => Some(r#"* You provided an empty Broadcast SID
* You provided a Broadcast SID that does not exist
* Stats for the Broadcast SID are not available yet"#),
            TwilioProgrammableMessagingError::ErrorCode30620 => Some(r#"This error occurs when a message is sent using a Sender ID that does not align with the registered use case for that particular Sender ID in your account. Ensure that the message content matches the use case registered with the Sender ID to comply with regulations and avoid failures."#),
            TwilioProgrammableMessagingError::ErrorCode30702 => Some(r#"Following case can cause this error: 

* Brand is no longer active or has been deleted."#),
            TwilioProgrammableMessagingError::ErrorCode30991 => Some(r#"Reason(s) for this failure can be:

* Brand is in unverified state"#),
            TwilioProgrammableMessagingError::ErrorCode36008 => Some(r#"* The provided page token has more or less than 34 characters
* The provided page token has non-alphanumeric characters.
* The prefix of the provided page token is something other than "BC""#),
            TwilioProgrammableMessagingError::ErrorCode30883 => Some(r#"The campaign cannot be approved because the submission indicated that the business was part of a prohibited SHAFT category or the content was potentially sending SHAFT material. SHAFT (Sex, Hate, Alcohol, Firearms, Tobacco/Vape) and Marijuana/CBD are prohibited categories."#),
            TwilioProgrammableMessagingError::ErrorCode30442 => Some(r#"The toll-free phone number cannot be verified because the business or message content was found to be associated with a known spam or fraudulent campaign. "#),
            TwilioProgrammableMessagingError::ErrorCode30892 => Some(r#"The campaign cannot be approved because public URL shorteners are not accepted. The website URL included in the sample messages was either from a public URL shortener or is unsecured."#),
            TwilioProgrammableMessagingError::ErrorCode30751 => Some(r#"You have provided an unsupported mobile phone number for brand registration."#),
            TwilioProgrammableMessagingError::ErrorCode30891 => Some(r#"* An invalid URL was provided during the registration process.
* The website associated with the campaign is not functioning or inaccessible.
* Opt-in flow is not found on website provided.
* Lack of proper indication in the campaign description if the registration pertains to a pre-launch website."#),
            TwilioProgrammableMessagingError::ErrorCode30455 => Some(r#"The toll-free phone number cannot be verified because the submission indicated that the business is associated with or the toll-free verification submission indicated a forbidden messaging category - (SHAFT: Hate). 

Twilio does not allow Hate speech, harassment, exploitative, abusive, or any communications that originate from a hate group."#),
            TwilioProgrammableMessagingError::ErrorCode63043 => Some(r#"- Server requires authorization"#),
            TwilioProgrammableMessagingError::ErrorCode36003 => Some(r#"* You attempted to upload a file that is greater than 25 MB"#),
            TwilioProgrammableMessagingError::ErrorCode35134 => Some(r#"The provided `Messages` array has one or more duplicate `To` numbers"#),
            TwilioProgrammableMessagingError::ErrorCode36007 => Some(r#"* The Broadcast SID that you provided is for a Broadcast that is in a state other than PENDING_UPLOAD. Those other states include: UPLOADED, QUEUED, EXECUTING, EXECUTION_FAILURE, EXECUTION_COMPLETED, CANCELATION_REQUESTED, CANCELED"#),
            TwilioProgrammableMessagingError::ErrorCode30796 => Some(r#"* The submitted stock symbol and stock exchange could not be verified for the submitted legal company name. 
* The submitted stock exchange is not recognized.  
"#),
            TwilioProgrammableMessagingError::ErrorCode30470 => Some(r#"The toll-free phone number cannot be verified because the submission indicated that the business is associated with or the toll-free verification submission indicated a forbidden messaging category  (High-risk: Deceptive Marketing). 

Marketing messages must be truthful, not misleading, and, when appropriate, backed by scientific evidence in order to meet the standard held by the Federal Trade Commission’s (FTC) Truth In Advertising rules. The FTC prohibits unfair or deceptive advertising in any medium, including text messages.
"#),
            TwilioProgrammableMessagingError::ErrorCode30476 => Some(r#"Proof of opt-in for consumer to provide consent was not provided. Every end business must have consent from each recipient they send a message to. It’s insufficient to select an opt-in method without description or providing evidence.

You may have submitted a URL for the opt-in workflow, but it did not demonstrate how the consumer opts-in to messaging. The URL can contain a link to the web form where the consumer opts-in, a hosted image file with a screenshot of how the consumer opts-in, or a link to a document that tells the story of the opt-in (such as a verbal opt-in script). 

Any URL submitted must be reachable, resolvable and of access to the public. 

Opt-in refers to the process of getting a consumer’s permission to send them text messages. A2P message Senders should: 
* Obtain a Consumer’s consent to receive messages generally; 
* Obtain a Consumer’s express written consent to specifically receive marketing messages; and 
* Ensure that Consumers have the ability to revoke consent.

Note: Any business with a terms of service or privacy policy that mentions sharing or selling consumer data/opt-in information is considered noncompliant. 
"#),
            TwilioProgrammableMessagingError::ErrorCode30990 => Some(r#"*Active Suspension*: The campaign has been suspended due to a violation of messaging policies or other related issues.

*Suspension Reason Category : Suspension Reason Description*
* Campaign-to-traffic mismatch: In-market traffic does not match with the registered campaign.
* Spam: Including but not limited to any kind of unwanted or unsolicited messaging.
* Controlled substance: Including but not limited to messaging pertaining to controlled substances.
* Phishing Messages: Including but not limited to messaging designed to gain access.
* Excessive Complaints: Including but not limited to unacceptable volumes of consumer complaints.
* Illicit Content: Including but not limited to messages relating to illegal activity.
* Fraudulent Messages: Including but not limited to counterfeit/fraudulent goods or activities.
* Affiliate Marketing: Including but not limited to sharing of opt-ins to affiliate companies.
* Promotional Gambling: Including but not limited to the act of gambling or promoting gambling.
* Lack of Age Gate: No age gate mechanism for messaging campaigns that require it.
* Illegal Sweepstakes: Including but not limited to sweepstakes that do not follow required laws.
* Other: As applicable."#),
            TwilioProgrammableMessagingError::ErrorCode30436 => Some(r#"* The phone number SID does not exist or does not belong to your account.
* The phone number SID is not a 34 character string prefixed with “PN”
"#),
            TwilioProgrammableMessagingError::ErrorCode30452 => Some(r#"The toll-free phone number cannot be verified because the email address is invalid. It could be due to:

* Incorrectly formatted email address
* Unknown email domain
* Email is a suspected disposable address
* Email is invalid
"#),
            TwilioProgrammableMessagingError::ErrorCode30793 => Some(r#"Following cases can cause this error :

* We were not able to find A2P bundle with the bundle SID provided as A2PProfileBundleSid

* We were not able to find Customer Profile bundle with the bundle SID provided as CustomerProfileBundleSid

* Invalid/unsupported stock exchange information was provided as part of A2P bundle (applicable only for public entities registering Low Volume Standard or Standard Brands)"#),
            TwilioProgrammableMessagingError::ErrorCode30468 => Some(r#"The toll-free phone number cannot be verified because the submission indicated that the business is associated with or the toll-free verification submission indicated a forbidden messaging category (Third-party lead generation services and marketing). 

Any third-party use cases are strictly forbidden. Consent must be obtained by the end business directly from consumers. Consent cannot be given to one business and then transferred to another. Any business with a terms of service or privacy policy that mentions sharing or selling consumer data/opt-in information is considered noncompliant.
"#),
            TwilioProgrammableMessagingError::ErrorCode63052 => Some(r#"Template sent using body field."#),
            TwilioProgrammableMessagingError::ErrorCode30748 => Some(r#"A phone number can not be used multiple times for Sole Proprietor Brand registrations. It is possible that the Starter customer profile bundle used for Sole Proprietor brand registration contains phone number that has been used to register multiple Sole Proprietor brands already. "#),
            TwilioProgrammableMessagingError::ErrorCode30992 => Some(r#"The user cancelled the CNP migration process before it could complete."#),
            TwilioProgrammableMessagingError::ErrorCode30451 => Some(r#"The toll-free phone number cannot be verified because the address is invalid. It could be due to:

* The provided address is not a valid US or Canada address
* Invalid State or Region
* Invalid Country
* Invalid House Number (or Unit/Apartment/Suite Number)
* Invalid City
* Invalid Postal Code
* Invalid Street
* Invalid Address - multiple issues
"#),
            TwilioProgrammableMessagingError::ErrorCode30457 => Some(r#"The toll-free phone number cannot be verified because the submission indicated that the business is associated with or the toll-free verification submission indicated a forbidden messaging category (SHAFT: Firearms and accessories). 

Firearms and accessories, Tobacco, Vape, and E-cigarettes are not allowed on Toll Free, Short Code, or Long Code regardless of age gating."#),
            TwilioProgrammableMessagingError::ErrorCode30467 => Some(r#"The toll-free phone number cannot be verified because the submission indicated that the business is associated with or the toll-free verification submission indicated a forbidden messaging category (Debt collection or forgiveness: Credit/debt repair). 

Debt consolidation, debt reduction and credit repair programs are prohibited regardless if there is first-party consent.
"#),
            TwilioProgrammableMessagingError::ErrorCode30798 => Some(r#"* The submitted company is not a US entity. Only US entities qualify for the Non-Profit Entity Type. 
* The submitted company is not verified as a US IRS recognised non-profit organisation. 
* The submitted company is verified as a US IRS recognised non-profit organisation but the charitable subsection code was not reported.
"#),
            TwilioProgrammableMessagingError::ErrorCode30461 => Some(r#"The toll-free phone number cannot be verified because the submission indicated that the business is associated with or the toll-free verification submission indicated a forbidden messaging category (Gambling). 

Gambling traffic is prohibited in the US and Canada on all number types (Toll Free, Short Code, Long Code). The gambling category includes, but is not limited to:
 
* Casino apps
* Websites that offer gambling
* Sweepstakes
* 50/50 Raffles
* Contests
* Betting/Sports picks"#),
            TwilioProgrammableMessagingError::ErrorCode30893 => Some(r#"* Sample messages are missing, unclear, or their content does not match the campaign's use-case.
* Invalid content within the sample messages."#),
            TwilioProgrammableMessagingError::ErrorCode30469 => Some(r#"The toll-free phone number cannot be verified because the submission indicated that the business is associated with or the toll-free verification submission indicated a forbidden messaging category (Illegal substances/articles).

Cannabis, CBD, Kratom, or drug paraphernalia product businesses are prohibited from utilizing SMS/MMS messaging on Twilio in the US and Canada, regardless of content. These restrictions apply regardless of the federal or state legality. All use cases for these are disallowed from sending SMS whether it contains cannabis content or not, even for 2FA purposes it is not permissible for such entities. Illegal substances/articles include, but are not limited to: 

* Cannabis
* CBD
* Kratom
* Paraphernalia products
* Vape/E-cigs
* Fireworks"#),
            TwilioProgrammableMessagingError::ErrorCode30753 => Some(r#"You have used an unsupported email address for brand registration"#),
            TwilioProgrammableMessagingError::ErrorCode30448 => Some(r#"The Toll-Free phone number cannot be verified because an Age Gate is not present or is not acceptable on your website and/or opt-in policy."#),
            TwilioProgrammableMessagingError::ErrorCode30449 => Some(r#"The toll-free phone number cannot be verified because the website URL in the sample message that you submitted was either from a public URL shortener or is non-secure.

Links to websites embedded within a message cannot conceal or obscure the Message Sender’s identity and are not intended to cause harm or deceive Consumers. A website URL used in messaging must be a dedicated custom domain that belongs to your business, not a free shared public link shortener.The domain must align with the message sender identified in the text message itself. Web addresses contained in messages as well as any websites to which they redirect should unambiguously identify the website owner (i.e., a person or legally registered business entity) and include contact information, such as a postal mailing address."#),
            TwilioProgrammableMessagingError::ErrorCode30790 => Some(r#"There was an internal system error when processing brand registration request."#),
            TwilioProgrammableMessagingError::ErrorCode30459 => Some(r#"The toll-free phone number cannot be verified because the submission indicated that the business is associated with or the toll-free verification submission indicated a forbidden messaging category (Illegal Substances/articles: Cannabis/CBD).

Messages related to cannabis are not allowed in the United States as federal laws prohibit its sale, even though some states have legalized it. Similarly, messages related to CBD are not permissible in the United States, as certain states prohibit its sale. Twilio defines a cannabis message as any message which relates to the marketing or sale of a cannabis product, regardless of whether or not those messages explicitly contain cannabis terms, images, or links to cannabis websites."#),
            TwilioProgrammableMessagingError::ErrorCode30795 => Some(r#"* The submitted US EIN is invalid. 
* The submitted Canadian government ID cannot be verified.
* The submitted foreign government ID cannot be verified.
* The submitted legal company name does not match with US EIN. 
* The submitted legal company name does not match with the Canadian government ID. 
* The submitted legal company name does not match with foreign government ID. 
* The submitted US EIN appears to be a US SSN.
"#),
            TwilioProgrammableMessagingError::ErrorCode30437 => Some(r#"There were no edits to the verification within 7 days of its rejection."#),
            TwilioProgrammableMessagingError::ErrorCode30887 => Some(r#"The campaign cannot be approved because it was indicated that you are collecting and processing consumer Opt-Outs, but the workflow is unclear, missing opt-out keywords or opt-message."#),
            TwilioProgrammableMessagingError::ErrorCode30880 => Some(r#"The campaign cannot be approved because of an unknown error and may stem from an issue raised by other vetting parties in the ecosystem."#),
            TwilioProgrammableMessagingError::ErrorCode30899 => Some(r#"One or more carriers rejected this campaign upon submission."#),
            TwilioProgrammableMessagingError::ErrorCode30905 => Some(r#"* Messaging Service was attempted to be associated before receiving the CAMPAIGN_SHARE_ACCEPT webhook from TCR."#),
            TwilioProgrammableMessagingError::ErrorCode36006 => Some(r#"* The Broadcast already completed execution, either successfully or with a failure
* The Broadcast was already canceled"#),
            TwilioProgrammableMessagingError::ErrorCode30479 => Some(r#"Verification supports up to 5 toll-free phone numbers for a single entity in a verification submission with no additional information required for traffic intended for US subscribers. For businesses requesting 6 or more toll-free phone numbers approved for verification, please include a detailed explanation in the “Additional Information” field as to the reason for additional toll-free phone numbers. 

For ISVs or aggregators who provide messaging service to businesses, it’s expected that the information provided represents the entity (your customer) that is engaging with the opted-in consumer. Therefore, a toll-free verification should not be associated with the ISV.  Exceptions may apply when the use case clearly showcases that the ISV manages opt-in mechanisms and is the sole message content creator.

Sending to Canadian handsets qualify for 1 toll-free number per business, as outlined in the Canadian Code of Conduct guide."#),
            TwilioProgrammableMessagingError::ErrorCode30886 => Some(r#"The campaign does not thoroughly describe explain the purpose or description does not match the use case."#),
            TwilioProgrammableMessagingError::ErrorCode30882 => Some(r#"The campaign cannot be approved because affiliated marketing is not a supported use case. Terms and conditions do not support this use case."#),
            TwilioProgrammableMessagingError::ErrorCode21733 => Some(r#"A default messaging service was not created for the account. "#),
            TwilioProgrammableMessagingError::ErrorCode30464 => Some(r#"The toll-free phone number cannot be verified because the submission indicated that the business is associated with or the toll-free verification submission indicated a forbidden messaging category (High-risk financial services: Cryptocurrency). 

No businesses that solely operate in stocks, investing, or cryptocurrency are allowed to send SMS traffic. If there is a mixed use case where that is a partial aspect of the business it may be approved based on the other use case content."#),
            TwilioProgrammableMessagingError::ErrorCode30752 => Some(r#"You have used either an invalid or an expired OTP for brand verification."#),
            TwilioProgrammableMessagingError::ErrorCode35132 => Some(r#"Provided attributes is not a valid JSON"#),
            TwilioProgrammableMessagingError::ErrorCode21268 => Some(r#"You attempted to send an SMS to premium rate or information service numbers.
"#),
            TwilioProgrammableMessagingError::ErrorCode30472 => Some(r#"The toll-free phone number cannot be verified. The business could not be verified, this may not be a valid business. "#),
            TwilioProgrammableMessagingError::ErrorCode30135 => Some(r#"Domain DNS setup is not completed.
There is an issue with the certificate authority"#),
            TwilioProgrammableMessagingError::ErrorCode30898 => Some(r#"The campaign cannot be approved because it the same EIN is used for multiple brands. "#),
            TwilioProgrammableMessagingError::ErrorCode21265 => Some(r#"You attempted to send an SMS to Short Code.
"#),
            TwilioProgrammableMessagingError::ErrorCode30904 => Some(r#"* Campaign has not been shared with Twilio's DCA.
* Provided campaignID is incorrect."#),
            TwilioProgrammableMessagingError::ErrorCode30791 => Some(r#"There was a temporary system error when processing brand registration request.

"#),
            TwilioProgrammableMessagingError::ErrorCode30794 => Some(r#"Your US A2P Brand or Campaign registration request was not processed successfully. While our systems were not able to identify the exact cause of failure, the error description provided as part of registration response should indicate the possible issue. "#),
            TwilioProgrammableMessagingError::ErrorCode30889 => Some(r#"The campaign cannot be approved because embedded phone number use is selected, but is not reflective in the sample message."#),
            TwilioProgrammableMessagingError::ErrorCode30478 => Some(r#"A toll-free verification has been received for a toll-free phone number that is assigned to multiple businesses. The toll-free phone number must be 1:1 with a single business, it cannot be used for multiple businesses. "#),
            TwilioProgrammableMessagingError::ErrorCode63040 => Some(r#"* Template purpose is not clear, concise, or well-written. For example, "Hi, {{1 2 Thank you.}}" is vague and not clear how it would be used or what the placeholder values represent.
* Template placeholders are not sequential in order. For example, "{{1 and 2 and 4 }}" are included but the placeholder {{3}} does not exist in the template.
* Template placeholders cannot be right next to each other like "{}{{1 2{}}}".
* Template content is identical to the content of another existing template.
* Template body cannot start or end with a placeholder.
* Template body cannot have the \n newline character.
* Template footer cannot have emojis or the \n newline character.
* Templates with Text headers cannot have emojis, asterisks, formatting markup, or the \n newline character.
* Templates with Media headers (video, image, document) must include a sample with their submission.
* Template is abusive or threatening in nature, such as containing content threatening to take legal action against a user.

For the most updated guidelines, please view WhatsApp’s documentation: https://developers.facebook.com/docs/whatsapp/message-templates/guidelines.

This error may also occur if you have an approved template that has similar body to a rejected template."#),
            TwilioProgrammableMessagingError::ErrorCode30909 => Some(r#"* *Unverifiable CTA*: The Call to Action associated with the campaign could not be verified during the review process.  If opt-in occurs outside of the website,then provided information was incomplete or not fully detailed.
"#),
            TwilioProgrammableMessagingError::ErrorCode30443 => Some(r#"The toll-free phone number cannot be verified because the submission indicated that the business is associated with or the toll-free verification submission indicated a forbidden messaging category (High-risk financial services: New loan soliciting). 

New loan soliciting, Payday loans, Short term high-interest loans, Third-party loans and Student loans are disallowed content. 

"Third-party" means originating from any party other than the one which will service the loan. Examples of third-party loans could include: auto, mortgage, personal, etc. First party loan content is acceptable if it is not promotional messaging.

Note: Any business with a terms of service or privacy policy that mentions sharing or selling consumer data/opt-in information is considered noncompliant.
"#),
            TwilioProgrammableMessagingError::ErrorCode30462 => Some(r#"The toll-free phone number cannot be verified because the submission indicated that the business is associated with or the toll-free verification submission indicated a forbidden messaging category (Gambling - Sweepstakes). 

Gambling traffic is prohibited in the US and Canada on all number types (Toll Free, Short Code, Long Code). The gambling category includes, but is not limited to:
 
* Casino apps
* Websites that offer gambling
* Sweepstakes
* 50/50 Raffles
* Contests
* Betting/Sports picks"#),
            TwilioProgrammableMessagingError::ErrorCode63024 => Some(r#"For WhatsApp, reasons can include:

* Recipient phone number is not a WhatsApp enabled phone number.
* Recipient has not accepted WhatsApp's Terms of Service and Privacy Policy.
* Recipient is using an old, unsupported version of the WhatsApp client for their phone."#),
            TwilioProgrammableMessagingError::ErrorCode30136 => Some(r#"Domain DNS setup is not completed.
There is an issue with the certificate authority"#),
            TwilioProgrammableMessagingError::ErrorCode30703 => Some(r#"* The mobile phone number provided in the brand registration request has already been used to register a US A2P Brand with TCR. 

* An existing brand is using the provided business identification type and number.

* An existing brand is using the provided business legal name.

* Authorised representative's phone number provided for brand registration has already been registered. Please retry with a different phone number.

* Email address provided for brand registration has already been registered with another brand. Please try again with a different email address.

* Website provided for brand registration has already been registered. Please try again with a different website url.

* Business name provided for brand registration has already been registered. Business names are expected to be unique across all brands.

* Stock symbol provided for brand registration has already been registered"#),
            TwilioProgrammableMessagingError::ErrorCode30799 => Some(r#"#### If your company is publicly listed and has an EIN, then it is possible that:

* The submitted US EIN is invalid. 

* The submitted Canadian government ID cannot be verified.

* The submitted foreign government ID cannot be verified.

* The submitted legal company name does not match with US EIN. 

* The submitted legal company name does not match with the Canadian government ID. 

* The submitted legal company name does not match with foreign government ID. 

* The submitted US EIN appears to be a US SSN. 

#### If you provided a stock exchange and stock ticker symbol, then it is possible that:

* The submitted stock symbol and stock exchange could not be verified for the submitted legal company name. 

* The submitted stock exchange is not recognized.  

#### If your company is a registered government entity, then it is possible that

*  The submitted company is not a US entity. Only US entities qualify for the Government Entity Type. 

*  The company’s submission as a Government Entity type could not be confirmed from independent sources. 

#### If your company is a non-profit entity, then it is possible that

* The submitted company is not a US entity. Only US entities qualify for the Non-Profit Entity Type. 

* The submitted company is not verified as a US IRS recognized non-profit organization. 

* The submitted company is verified as a US IRS recognized non-profit organization but the charitable subsection code was not reported.
"#),
            TwilioProgrammableMessagingError::ErrorCode11206 => Some(r#"* Server misconfiguration
* Network disruption
* Making HTTP connections to an HTTPS port
* There is an invalid symbol in the TwiML Cookie"#),
            TwilioProgrammableMessagingError::ErrorCode63042 => Some(r#"Negative customer feedback (i.e. multiple customers reporting messages or blocking number)"#),
            TwilioProgrammableMessagingError::ErrorCode30712 => Some(r#"Your account has reached maximum number allowed brand registrations."#),
            TwilioProgrammableMessagingError::ErrorCode30466 => Some(r#"The toll-free phone number cannot be verified because the submission indicated that the business is associated with or the toll-free verification submission indicated a forbidden messaging category  (Debt collection or forgiveness: Debt reduction). 

Debt consolidation, debt reduction and credit repair programs are prohibited regardless if there is first-party consent. 
"#),
            TwilioProgrammableMessagingError::ErrorCode21661 => Some(r#"The number you are using is not capable of sending messages."#),
            TwilioProgrammableMessagingError::ErrorCode30756 => Some(r#" Obfuscation check failed for brand registration inputs."#),
            TwilioProgrammableMessagingError::ErrorCode30747 => Some(r#"Reusing addresses for Sole Proprietor Brands is not allowed. Each Sole Proprietor Brand should have a unique address linked to it. It is possible that the Starter customer profile bundle associated with the brand you tried to register has a duplicate address linked to it. "#),
            TwilioProgrammableMessagingError::ErrorCode30881 => Some(r#"The campaign cannot be approved because support email needs is not associated with the brand. The brand support email was either invalid or associated with a public domain email."#),
            TwilioProgrammableMessagingError::ErrorCode30615 => Some(r#"Marketing, promotional & other non essential messages are restricted under the local regulatory requirement (e.g.TCPA in US) to certain hours of the day. Specifically, these communications cannot be made before 8 a.m. or after 9 p.m. local time at the recipient's location. This restriction is intended to protect consumers from intrusive communications during times they are likely to be resting or engaged in personal activities."#),
            TwilioProgrammableMessagingError::ErrorCode30458 => Some(r#"The toll-free phone number cannot be verified because the submission indicated that the business is associated with or the toll-free verification submission indicated a forbidden messaging category (SHAFT: Tobacco/Vape). 

Firearms, Tobacco, Vape, and E-cigarettes are not allowed on Toll Free, Short Code, or Long Code regardless of age gating."#),
            TwilioProgrammableMessagingError::ErrorCode30755 => Some(r#"You have used an country code which is not yet support for US A2P brand registrations. Supported list of country codes are : PR,PS,PT,PW,PY,QA,AD,AE,AF,AG,AI,AL,AM,AO,AQ,AR,AS,AT,RE,AU,AW,AX,AZ,RO,BA,BB,RS,BD,RU,BE,BF,BG,RW,BH,BI,BJ,BL,BM,BN,BO,SA,SB,BQ,SC,BR,BS,SD,SE,BT,SG,BV,SH,BW,SI,SJ,BY,BZ,SK,SL,SM,SN,SO,CA,SR,CC,SS,ST,CD,CF,SV,CG,SX,CH,CI,SZ,CK,CL,CM,CN,CO,CR,TC,TD,TF,TG,CV,CW,TH,CX,TJ,CY,TK,CZ,TL,TM,TN,TO,TR,TT,DE,TV,TW,DJ,TZ,DK,DM,DO,UA,UG,DZ,UM,EC,US,EE,EG,EH,UY,UZ,VA,ER,VC,ES,VE,ET,VG,VI,VN,VU,FI,FJ,FK,FM,FO,FR,WF,GA,GB,WS,GD,GE,GF,GG,GH,GI,GL,GM,GN,GP,GQ,GR,GS,GT,XE,GU,GW,GY,XK,HK,HM,HN,HR,YE,HT,HU,YT,ID,IE,IL,IM,IN,IO,ZA,IQ,IS,IT,ZM,JE,ZW,JM,JO,JP,KE,KG,KH,KI,KM,KN,KR,KW,KY,KZ,LA,LB,LC,LI,LK,LR,LS,LT,LU,LV,LY,MA,MC,MD,ME,MF,MG,MH,MK,ML,MM,MN,MO,MP,MQ,MR,MS,MT,MU,MV,MW,MX,MY,MZ,NA,NC,NE,NF,NG,NI,NL,NO,NP,NR,NU,NZ,OM,PA,PE,PF,PG,PH,PK,PL,PM,PN"#),
            TwilioProgrammableMessagingError::ErrorCode36002 => Some(r#"* You are at the limit of Broadcast requests for your account and cannot create any additional Broadcasts"#),
            TwilioProgrammableMessagingError::ErrorCode63044 => Some(r#"- The file does not exist 
- The URL provided is incorrect"#),
            TwilioProgrammableMessagingError::ErrorCode30792 => Some(r#"There was a general system error when processing brand registration request."#),
            TwilioProgrammableMessagingError::ErrorCode30900 => Some(r#"Reasons for this failure can be: 

* Provided use case is not supported yet.
* Campaign use case not supported by one of the carriers.
* The brand is not qualified to run campaign of provide use case for one of the carriers.
* The brand with use case campaign is not within sub-usecase bounds."#),
            TwilioProgrammableMessagingError::ErrorCode63049 => Some(r#"Meta blocked the delivery of this marketing template messages to the recipient."#),
            TwilioProgrammableMessagingError::ErrorCode30477 => Some(r#"The Terms of Service or Privacy Policy on the business’s website indicates that Opt-in is shared with third parties, affiliates, partners, etc. Any business with a terms of service or privacy policy that mentions sharing or selling consumer opt-in data is considered noncompliant. Consent cannot be bought, sold, shared, transferred, or exchanged. "#),
            TwilioProgrammableMessagingError::ErrorCode30630 => Some(r#"The end user handset has responded with "STOP" or [another opt-out keyword.](https://help.twilio.com/articles/223134027-Twilio-support-for-opt-out-keywords-SMS-STOP-filtering)"#)
        }
    }
}
