// AUTO-GENERATED. DO NOT MODIFY. YOUR CHANGES WILL BE OVER-WRITTEN
use crate::products::TwilioPhoneNumbersError;
use standard_error::traits::StandardErrorSolutionsTrait;

impl StandardErrorSolutionsTrait for TwilioPhoneNumbersError {
    fn solutions(&self) -> Option<&'static str> {
        match self {
            TwilioPhoneNumbersError::ErrorCode22225 => Some(r#"Please review the FromBundleSid specified to ensure it is correctly referencing the account's BundleSid and not another account."#),
            TwilioPhoneNumbersError::ErrorCode22152 => Some(r#"Contact the phone number's current carrier to reactivate the phone number, so that you can port it to Twilio."#),
            TwilioPhoneNumbersError::ErrorCode22155 => Some(r#"Update PIN in the port in request to one that matches the losing carrier's information. 

PINs are unique to a single phone numbers, so you should not need a new request to change the PIN for just this phone number. Instead update the port in request with the correct PIN."#),
            TwilioPhoneNumbersError::ErrorCode22205 => Some(r#"Please refer to the Supporting Documents LIST or End-User LIST resources to find the correct Supporting Document SID or End-User SID to assign as an item to a Bundle."#),
            TwilioPhoneNumbersError::ErrorCode22212 => Some(r#"Ensure that the End-User Type value is spelled correctly or is present in the End-User Type LIST resource."#),
            TwilioPhoneNumbersError::ErrorCode22112 => None,
            TwilioPhoneNumbersError::ErrorCode18056 => Some(r#"
Please ensure you provide the work email of the Authorized Representative to proceed."#),
            TwilioPhoneNumbersError::ErrorCode22213 => Some(r#"Ensure that the Number Type value is spelled correctly or is an acceptable Number Type in the Regulations LIST resource"#),
            TwilioPhoneNumbersError::ErrorCode22210 => Some(r#"Refer to the Supporting Document Types LIST resource to find the correct name type for your request."#),
            TwilioPhoneNumbersError::ErrorCode18060 => Some(r#"Please re-submit after uploading the excerpt from the commercial register."#),
            TwilioPhoneNumbersError::ErrorCode18605 => Some(r#"Please feel free to edit the Business Profile & update the missing information. You can also reach out to our support team at trusthub-verify@twilio.com for any additional clarifications."#),
            TwilioPhoneNumbersError::ErrorCode18609 => Some(r#"Please resubmit your registration using either an EIN or a DUN number."#),
            TwilioPhoneNumbersError::ErrorCode22171 => Some(r#"Check what fields are required for each country and update those fields with valid values."#),
            TwilioPhoneNumbersError::ErrorCode22122 => None,
            TwilioPhoneNumbersError::ErrorCode21628 => None,
            TwilioPhoneNumbersError::ErrorCode22118 => None,
            TwilioPhoneNumbersError::ErrorCode22217 => Some(r#"Please refer to the Regulations API resource to ensure that all required fields are present in the Supporting Document."#),
            TwilioPhoneNumbersError::ErrorCode18008 => Some(r#"Please ensure the names match exactly and that all the individual's last names are provided. Please amend firstly the name in the Business & legal representative information / Individual information section of the bundle, and then in the Bundle fields under the uploaded supporting document."#),
            TwilioPhoneNumbersError::ErrorCode22133 => Some(r#"If the number is an US phone number, please open a porting request through the Console. See this [support page](https://support.twilio.com/hc/en-us/articles/223179348-Porting-a-Phone-Number-to-Twilio) for more information on US porting.

Otherwise create a support ticket to port in a phone number from another country. See this [support page](https://support.twilio.com/hc/en-us/articles/115000781088-International-Porting-Process) for more information on international porting."#),
            TwilioPhoneNumbersError::ErrorCode22130 => Some(r#"Not Action Required"#),
            TwilioPhoneNumbersError::ErrorCode22114 => None,
            TwilioPhoneNumbersError::ErrorCode22111 => None,
            TwilioPhoneNumbersError::ErrorCode22151 => Some(r#"Contact the phone number's current carrier and resolve any outstanding issues that is preventing them from approving the port."#),
            TwilioPhoneNumbersError::ErrorCode21650 => None,
            TwilioPhoneNumbersError::ErrorCode18036 => Some(r#"Please fill out the required fields in the form."#),
            TwilioPhoneNumbersError::ErrorCode18608 => Some(r#"Since the customer profile or trust product is already copied, please move forward without any additional action"#),
            TwilioPhoneNumbersError::ErrorCode18053 => Some(r#"Please reach out to us via email numbers-regulatory-review@twilio.com and share the details of how the company and website are connected."#),
            TwilioPhoneNumbersError::ErrorCode22153 => Some(r#"Update the port in request to include an authorized representative that is on record with the losing carrier for this phone number.

Note if this is the only phone number in a port in request that got rejected you may need to start a new port in request with this phone number so that you can correct the authorized representative name as it may be different from the other phone numbers in this request. "#),
            TwilioPhoneNumbersError::ErrorCode18051 => Some(r#"Please check the Authorized Representative's email for the details on the issue, please resubmit the bundle after rectifying the issue mentioned in the email."#),
            TwilioPhoneNumbersError::ErrorCode18604 => Some(r#"Please feel free to edit the Business Profile & update the missing information. You can also reach out to our support team at trusthub-verify@twilio.com for any additional clarifications."#),
            TwilioPhoneNumbersError::ErrorCode22401 => Some(r#"Please refer to documentation to understand which fields are supported."#),
            TwilioPhoneNumbersError::ErrorCode22200 => Some(r#"Ensure that the End-User Type value or Number Type value is spelled correctly or is an acceptable value from the End-User Type LIST resource or Number Type LIST resource."#),
            TwilioPhoneNumbersError::ErrorCode18606 => Some(r#"Please feel free to edit the Business Profile & update the missing information. You can also reach out to our support team at trusthub-verify@twilio.com for any additional clarifications."#),
            TwilioPhoneNumbersError::ErrorCode22102 => Some(r#"Please specify a valid phone number in [E164 format](http://en.wikipedia.org/wiki/E.164) (i.e. "+1 format")."#),
            TwilioPhoneNumbersError::ErrorCode18607 => Some(r#"Please move forward without copying the customer profile or trust product"#),
            TwilioPhoneNumbersError::ErrorCode18005 => Some(r#"Please fill out the required form fields and resubmit the bundle."#),
            TwilioPhoneNumbersError::ErrorCode18017 => Some(r#"Please upload a Supporting Document that shows an address."#),
            TwilioPhoneNumbersError::ErrorCode21648 => Some(r#"Re-assign the current number(s) to another twilio-approved Regulatory Bundle."#),
            TwilioPhoneNumbersError::ErrorCode18052 => Some(r#"Please resubmit for an individual with acceptable age. Please make sure to take necessary steps to remove underaged person's personal information from the Bundle."#),
            TwilioPhoneNumbersError::ErrorCode22402 => Some(r#"Please refer to documentation to understand which operations are permitted within each region."#),
            TwilioPhoneNumbersError::ErrorCode21630 => None,
            TwilioPhoneNumbersError::ErrorCode21449 => None,
            TwilioPhoneNumbersError::ErrorCode18055 => Some(r#"Please check and resubmit the bundle with a valid email address and continue."#),
            TwilioPhoneNumbersError::ErrorCode22215 => Some(r#"Please refer to the Regulations API to ensure that all required fields are present in the End-User."#),
            TwilioPhoneNumbersError::ErrorCode18054 => Some(r#"Please review and provide a valid phone number in E164 format."#),
            TwilioPhoneNumbersError::ErrorCode18603 => Some(r#"Please feel free to edit the Business Profile & update the missing information. You can also reach out to our support team at trusthub-verify@twilio.com for any additional clarifications."#),
            TwilioPhoneNumbersError::ErrorCode18058 => Some(r#"Please amend the Tax ID Number field in the Business & Legal Representative Information section to the Authorized Representative´s personal tax ID."#),
            TwilioPhoneNumbersError::ErrorCode18012 => Some(r#"Please resubmit with a valid address or create a new bundle selecting the appropriate phone number type."#),
            TwilioPhoneNumbersError::ErrorCode22403 => Some(r#"Please refer to documentation to understand which operations are permitted within each region."#),
            TwilioPhoneNumbersError::ErrorCode22400 => Some(r#"Please contact Twilio Support either via the [Console](https://console.twilio.com) or the [Help Center](https://support.twilio.com) to request assistance with deleting the Active Route Configuration for an inactive phone number reference."#),
            TwilioPhoneNumbersError::ErrorCode18061 => Some(r#"Please upload a business registration or excerpt from the commercial registry as a Supporting Document and ensure it matches the information provided in the Bundle."#),
            TwilioPhoneNumbersError::ErrorCode18024 => Some(r#"Please resubmit with a valid/uncorrupted file."#),
            TwilioPhoneNumbersError::ErrorCode18602 => Some(r#"Please feel free to edit the Business Profile & update the missing information. You can also reach out to our support team at trusthub-verify@twilio.com for any additional clarifications.

Please be sure to use the legal company name and EIN number found in your Tax records."#),
            TwilioPhoneNumbersError::ErrorCode22224 => Some(r#"Ensure both Bundles are in the correct twilio-approved status and that the destination Bundle is a Copy of the source Bundle."#),
            TwilioPhoneNumbersError::ErrorCode22207 => Some(r#"Check to see if the JSON is valid and resend the request."#),
            TwilioPhoneNumbersError::ErrorCode22230 => Some(r#"Create a new Address on your Account or Sub-Account (if you don’t already have one) and reach out to [Twilio Support](https://support.twilio.com/hc/en-us) to link it to the Regulatory Bundle that you wish to copy."#),
            TwilioPhoneNumbersError::ErrorCode22159 => Some(r#"Check with your current carrier to determine if you are using any services that would prevent the phone number from being ported to Twilio. If you can remove those services you may still be able to attempt a port. 

Otherwise, please reach out to Support at porting@twilio.com and they can help to troubleshoot the rejection further and find alternatives for you."#),
            TwilioPhoneNumbersError::ErrorCode22204 => Some(r#"Remove the status update parameter or remove the property update(s) in the request."#),
            TwilioPhoneNumbersError::ErrorCode22113 => None,
            TwilioPhoneNumbersError::ErrorCode22203 => Some(r#"Refer to the Bundle REST API status section for which statuses you are able to choose from."#),
            TwilioPhoneNumbersError::ErrorCode18099 => Some(r#"Resubmit the bundle with the IsTest parameter to False or omitted if it should be considered for approval"#),
            TwilioPhoneNumbersError::ErrorCode18022 => Some(r#"If you are an individual, please share the document issued in the last three months. If you are a business, please share the document issued within the last year."#),
            TwilioPhoneNumbersError::ErrorCode22214 => Some(r#"Please refer to the Regulations API to ensure that the End User satisfying the requirement is in the regulatory bundle."#),
            TwilioPhoneNumbersError::ErrorCode22209 => Some(r#"Please refer to the Supporting Documents REST Public API documentation for the update instance resource to pick from an allowed status enum list."#),
            TwilioPhoneNumbersError::ErrorCode22404 => Some(r#"Contact Twilio support."#),
            TwilioPhoneNumbersError::ErrorCode22119 => None,
            TwilioPhoneNumbersError::ErrorCode18038 => Some(r#"Please amend the information and resubmit the bundle."#),
            TwilioPhoneNumbersError::ErrorCode22105 => None,
            TwilioPhoneNumbersError::ErrorCode22117 => None,
            TwilioPhoneNumbersError::ErrorCode22108 => None,
            TwilioPhoneNumbersError::ErrorCode10002 => Some(r#"Upgrade to a full account and try the feature again."#),
            TwilioPhoneNumbersError::ErrorCode18004 => Some(r#"Re-upload the Supporting Document without any redactions or covered portions. For any ID document, make sure to provide a color copy of the ID document without any reductions. Make sure that all the edges of the ID document and MRZ data for passports are visible."#),
            TwilioPhoneNumbersError::ErrorCode18014 => Some(r#"Please resubmit the bundle with the correct address."#),
            TwilioPhoneNumbersError::ErrorCode18001 => Some(r#"Please re-submit after uploading a Supporting Document according to our Guidelines. 
https://www.twilio.com/en-us/guidelines/regulatory"#),
            TwilioPhoneNumbersError::ErrorCode22222 => Some(r#"Remove Emergency Status from the request"#),
            TwilioPhoneNumbersError::ErrorCode18011 => Some(r#"Please resubmit with the accurate business name and the corresponding business registration number."#),
            TwilioPhoneNumbersError::ErrorCode22218 => Some(r#"Please ensure that the field in the Supporting Document matches the field in the End-User."#),
            TwilioPhoneNumbersError::ErrorCode22208 => Some(r#"Send the attributes in the update request followed with a status update in another request."#),
            TwilioPhoneNumbersError::ErrorCode22120 => None,
            TwilioPhoneNumbersError::ErrorCode22206 => Some(r#"Please refer to the Supporting Documents LIST or End-User LIST resources to find the correct Supporting Document SID or End-User SID to assign as an item to a Bundle."#),
            TwilioPhoneNumbersError::ErrorCode22135 => Some(r#"Run the Portability Check again"#),
            TwilioPhoneNumbersError::ErrorCode22500 => Some(r#"Update the phone number API version to 2010-04-01 or later version"#),
            TwilioPhoneNumbersError::ErrorCode18007 => Some(r#"Please upload a copy of both sides and make sure to upload them in one document."#),
            TwilioPhoneNumbersError::ErrorCode22121 => None,
            TwilioPhoneNumbersError::ErrorCode22211 => Some(r#"Resend the request including the FriendlyName"#),
            TwilioPhoneNumbersError::ErrorCode22202 => Some(r#"* Please ensure you are passing a regulation sid or phone number country and type
* If you need to know which regulation sid or phone number country and type to use, please refer to the Regulations REST API."#),
            TwilioPhoneNumbersError::ErrorCode22115 => None,
            TwilioPhoneNumbersError::ErrorCode18010 => Some(r#"Please resubmit with the accurate business registration number to ensure it matches the local company register or commercial register."#),
            TwilioPhoneNumbersError::ErrorCode22170 => Some(r#"Update the Bundle SID to one that exists or is approved."#),
            TwilioPhoneNumbersError::ErrorCode22158 => Some(r#"Resubmit the port in request with a different target port date. Please review the new date to ensure it does not fall on a public holiday. 

Contact support at porting@twilio.com if you need help selecting a target port date."#),
            TwilioPhoneNumbersError::ErrorCode22157 => Some(r#"Some NTT plans only allowed to request specific subscription rights. 

If your current NTT plan is the "Light Plan", then only the Termination subscription right will be accepted by NTT.

Otherwise you should request the subscription right "Suspension"."#),
            TwilioPhoneNumbersError::ErrorCode18020 => Some(r#"Please upload a Supporting Document that shows the Authorized Representative as part of the business."#),
            TwilioPhoneNumbersError::ErrorCode22136 => Some(r#"If needed, see this [support page](https://support.twilio.com/hc/en-us/articles/223135327-Moving-Twilio-Phone-Numbers-to-another-Twilio-Account) for more information on how to move the line to a different subaccount."#),
            TwilioPhoneNumbersError::ErrorCode22104 => None,
            TwilioPhoneNumbersError::ErrorCode22106 => None,
            TwilioPhoneNumbersError::ErrorCode18002 => Some(r#"Please re-submit after uploading a clear copy of the supporting document"#),
            TwilioPhoneNumbersError::ErrorCode18018 => Some(r#"Please share a full physical address."#),
            TwilioPhoneNumbersError::ErrorCode22221 => Some(r#"Please register an emergency address with the from number."#),
            TwilioPhoneNumbersError::ErrorCode21631 => Some(r#"- Add an address that satisfies the locality requirement.
- Contact Twilio Support."#),
            TwilioPhoneNumbersError::ErrorCode21615 => Some(r#"- Add an address that satisfies the locality requirement
- Contact Twilio Support"#),
            TwilioPhoneNumbersError::ErrorCode18003 => Some(r#"Please resubmit with a valid supporting document."#),
            TwilioPhoneNumbersError::ErrorCode18062 => Some(r#"Please amend the business details to include an active business registration."#),
            TwilioPhoneNumbersError::ErrorCode22132 => Some(r#"See this [support page](https://support.twilio.com/hc/en-us/articles/223135327-Moving-Twilio-Phone-Numbers-to-another-Twilio-Account) for more information on how to move the line to your account."#),
            TwilioPhoneNumbersError::ErrorCode22101 => None,
            TwilioPhoneNumbersError::ErrorCode22201 => Some(r#"Use the Regulations LIST resource with filters to find the correct Number Group."#),
            TwilioPhoneNumbersError::ErrorCode22100 => None,
            TwilioPhoneNumbersError::ErrorCode22150 => Some(r#"Please contact Twilio's Porting Operations team at porting@twilio.com for support."#),
            TwilioPhoneNumbersError::ErrorCode22173 => Some(r#"Update the Address SID to one that exists or check the account you are using."#),
            TwilioPhoneNumbersError::ErrorCode22103 => None,
            TwilioPhoneNumbersError::ErrorCode18009 => Some(r#"Please resubmit with the appropriate identity document. Please ensure the document uploaded and the information you provided match."#),
            TwilioPhoneNumbersError::ErrorCode22131 => Some(r#"If needed, see this [support page](https://support.twilio.com/hc/en-us/articles/223135327-Moving-Twilio-Phone-Numbers-to-another-Twilio-Account) for more information on how to move the line to a different subaccount."#),
            TwilioPhoneNumbersError::ErrorCode22109 => None,
            TwilioPhoneNumbersError::ErrorCode22156 => Some(r#"Update Account Number in the port in request to one that matches the losing carrier's information. 

Note if this is the only phone number in a port in request that got rejected you may need to start a new port in request with this phone number so that you can correct the account number as it may be different from the other phone numbers in this request."#),
            TwilioPhoneNumbersError::ErrorCode21645 => Some(r#"Unassign the Supporting Document from the Regulatory Bundle(s)"#),
            TwilioPhoneNumbersError::ErrorCode21651 => None,
            TwilioPhoneNumbersError::ErrorCode18015 => Some(r#"Please resubmit the bundle with the correct address."#),
            TwilioPhoneNumbersError::ErrorCode18023 => Some(r#"Please upload a Business Document showing a currently active business registration."#),
            TwilioPhoneNumbersError::ErrorCode22199 => Some(r#"Double Check that the phone number has a configuration for each region you are working with.
"#),
            TwilioPhoneNumbersError::ErrorCode21649 => Some(r#"- Build a new Bundle that satisfies the regulations of the phone number.
- Contact Twilio Support."#),
            TwilioPhoneNumbersError::ErrorCode22116 => None,
            TwilioPhoneNumbersError::ErrorCode22350 => Some(r#"You cannot release a Phone Number that is being ported out. Please, reach out to porting@twilio.com for assistance."#),
            TwilioPhoneNumbersError::ErrorCode22110 => None,
            TwilioPhoneNumbersError::ErrorCode18013 => Some(r#"Please resubmit with a valid domestic address or create a new bundle selecting the appropriate phone number type."#),
            TwilioPhoneNumbersError::ErrorCode18037 => Some(r#"Please fill out the required form fields and resubmit the bundle."#),
            TwilioPhoneNumbersError::ErrorCode22172 => Some(r#"Please review the list of phone numbers and submit one port in request for all numbers from each country. A port in request can only contain phone numbers from one country. 

Also double check that all phone numbers are in E.164 format so that Twilio can correct identify the country code. 
https://www.twilio.com/docs/glossary/what-e164"#),
            TwilioPhoneNumbersError::ErrorCode21646 => Some(r#"Wait for the Supporting Document to transition to a deletable status (DRAFT, TWILIO_REJECTED, TWILIO_APPROVED). If your Supporting Document is idle for more than 3 business days, please reach out to Twilio Support through the [Console](https://console.twilio.com) or [Help Center](https://support.twilio.com)."#),
            TwilioPhoneNumbersError::ErrorCode18021 => Some(r#"Please ensure the names match exactly and that all the individual's last names are provided. Please amend firstly the name in the Business & legal representative information / Individual information section of the bundle, and then in the Bundle fields under the uploaded supporting document."#),
            TwilioPhoneNumbersError::ErrorCode22216 => Some(r#"Please refer to the Regulations API resource to ensure that a Supporting Document satisfying the requirement is in the regulatory bundle."#),
            TwilioPhoneNumbersError::ErrorCode22123 => None,
            TwilioPhoneNumbersError::ErrorCode22154 => Some(r#"Check the address submitted in the port in request for typos or change it to an address that matches the losing carrier's information for this phone number. 

Note if this is the only phone number in a port in request that got rejected you may need to start a new port in request with this phone number so that you can correct the address as it may be different from the other phone numbers in this request. "#),
            TwilioPhoneNumbersError::ErrorCode22228 => Some(r#"Please reach out to Twilio Support through the [Console](https://console.twilio.com) or [Help Center](https://support.twilio.com) by referencing this API error code with the BundleSid requesting assistance with fixing the Bundle."#),
            TwilioPhoneNumbersError::ErrorCode21629 => None,
            TwilioPhoneNumbersError::ErrorCode18016 => Some(r#"Please amend the data entered in the Address fields to match exactly the address in the documents you have provided."#),
            TwilioPhoneNumbersError::ErrorCode18050 => Some(r#"Please review the Supporting Documents for any rejection reason(s), and resubmit with correct Supporting Documents."#),
            TwilioPhoneNumbersError::ErrorCode22107 => None,
            TwilioPhoneNumbersError::ErrorCode18601 => Some(r#"Please edit the Business Profile & update the missing information. You can also reach out to our support team at trusthub-verify@twilio.com for any additional clarifications."#),
            TwilioPhoneNumbersError::ErrorCode22300 => Some(r#"Please reach out to verifymyaccount@twilio.com to have this restriction lifted from your account. Be sure to include your Twilio Account SID, the name of your business or project, and a brief explanation of your use case."#),
            TwilioPhoneNumbersError::ErrorCode22229 => Some(r#"Metadata of the Supporting Document can be updated but the Address is immutable. Either a new Supporting Document that validates the same information can be supplied or a new Regulatory Bundle can be created."#),
            TwilioPhoneNumbersError::ErrorCode18610 => Some(r#"Create a new Address on your Account or Sub-Account (if you don’t already have one) and reach out to [Twilio Support](https://support.twilio.com/hc/en-us) to link it to the Customer Profile or Trust Product that you wish to copy."#),
            TwilioPhoneNumbersError::ErrorCode18059 => Some(r#"Ensure the dates are current and the information on the Photo ID matches the Authorized Individual's information and resubmit with a valid government-issued Photo ID."#),
            TwilioPhoneNumbersError::ErrorCode180017 => Some(r#"Please resubmit with a valid domestic address or create a new bundle selecting the appropriate phone number type."#),
            TwilioPhoneNumbersError::ErrorCode18006 => Some(r#"You may have attached the wrong document or shared inaccurate information."#),
            TwilioPhoneNumbersError::ErrorCode21647 => Some(r#"Wait for the Regulatory Bundle to transition to a deletable status (DRAFT, TWILIO_REJECTED, TWILIO_APPROVED).

If the Regulatory Bundle is idle for 3 or more business days, please reach out to Twilio Support through the [Console](https://console.twilio.com) or [Help Center](https://support.twilio.com)."#),
            TwilioPhoneNumbersError::ErrorCode22219 => Some(r#"Please refer to the Regulations API to ensure that an AddressSid is assigned to a Supporting Document."#),
            TwilioPhoneNumbersError::ErrorCode21644 => Some(r#"Reassign a new End-User to all Bundle(s) and then perform the DELETE operation."#),
            TwilioPhoneNumbersError::ErrorCode18019 => Some(r#"Please provide a government-issued ID or passport for the Authorized Representative to fulfill this requirement."#),
            TwilioPhoneNumbersError::ErrorCode18039 => Some(r#"x"#),
            TwilioPhoneNumbersError::ErrorCode18057 => Some(r#"Please share a different authorized representative or reach out to us via email numbers-regulatory-review@twilio.com and share the details of how the company and website are connected."#),
            TwilioPhoneNumbersError::ErrorCode22226 => Some(r#"Please ensure From FromBundleSid is Copy of the provisioning Bundle."#)
        }
    }
}
