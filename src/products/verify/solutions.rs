// AUTO-GENERATED. DO NOT MODIFY. YOUR CHANGES WILL BE OVER-WRITTEN
use crate::products::TwilioVerifyError;
use standard_error::traits::StandardErrorSolutionsTrait;

impl StandardErrorSolutionsTrait for TwilioVerifyError {
    fn solutions(&self) -> Option<&'static str> {
        match self {
            TwilioVerifyError::ErrorCode60367 => Some(r#"Check that the Entity identity in the request body is correct."#),
            TwilioVerifyError::ErrorCode60215 => Some(r#"Delete/update Mailers that you are not using at the moment"#),
            TwilioVerifyError::ErrorCode60378 => Some(r#"Check that the Authenticator response in the request body is correct."#),
            TwilioVerifyError::ErrorCode60374 => Some(r#"Check that the Page token in the request body is correct."#),
            TwilioVerifyError::ErrorCode60373 => Some(r#"Check that the Page size in the request body is correct."#),
            TwilioVerifyError::ErrorCode60213 => Some(r#"* Try updating the Messaging Configuration instead."#),
            TwilioVerifyError::ErrorCode60424 => Some(r#"Try updating the contact with the appropriate address information."#),
            TwilioVerifyError::ErrorCode60388 => Some(r#"Check that the User display name in the request body is provided."#),
            TwilioVerifyError::ErrorCode60365 => Some(r#"Check that the Entity SID in the request parameter/body is correct."#),
            TwilioVerifyError::ErrorCode60237 => Some(r#"Please use a different email with a custom domain."#),
            TwilioVerifyError::ErrorCode60312 => Some(r#"- Create a new Factor and verify it. Then create new Challenges under the new Factor.
- Depending on the Factor's `Config.CodeLength` parameter, await rate limit cooldown period between 1 hour and 24 hours."#),
            TwilioVerifyError::ErrorCode60318 => Some(r#"Verify the Factor first."#),
            TwilioVerifyError::ErrorCode60234 => Some(r#"Set another sender id as default"#),
            TwilioVerifyError::ErrorCode68006 => Some(r#"Check parameters and retry."#),
            TwilioVerifyError::ErrorCode60235 => Some(r#"Validate the object, it contains invalid requirements such as an invalid name. Sender id must have at least 3 characters and up to 11 characters."#),
            TwilioVerifyError::ErrorCode60407 => Some(r#"Check parameters and retry."#),
            TwilioVerifyError::ErrorCode60306 => Some(r#"Review the parameters you are passing from the list above and try again. Accepted parameters are detailed in the [API reference](https://www.twilio.com/docs/verify/api)."#),
            TwilioVerifyError::ErrorCode60236 => Some(r#"Please review the email provided or the email registered of your account administrator users"#),
            TwilioVerifyError::ErrorCode60421 => Some(r#"Check that the contact information in the request body is correct and try again."#),
            TwilioVerifyError::ErrorCode60218 => Some(r#"Log into SendGrid's dashboard and activate the template, or select a different template"#),
            TwilioVerifyError::ErrorCode60310 => Some(r#"- Create a new Factor and verify that one..
- For a TOTP Factor, ask the user to provide the correct TOTP code or create a new Factor if they are unable to generate the correct codes for the Factor they are verifying."#),
            TwilioVerifyError::ErrorCode60381 => Some(r#"Check that the Signature in the request body is correct."#),
            TwilioVerifyError::ErrorCode60377 => Some(r#"Check that the Authenticator attachment in the request body is correct."#),
            TwilioVerifyError::ErrorCode60382 => Some(r#"Check that the Relying party in the request body is correct."#),
            TwilioVerifyError::ErrorCode60440 => Some(r#"Try using a different verification content type"#),
            TwilioVerifyError::ErrorCode60211 => Some(r#"* Delete/Update Bucket that has this same `Interval`"#),
            TwilioVerifyError::ErrorCode60324 => Some(r#"- For a Push Challenge, check the signature sent in AuthPayload and retry.  
- For a TOTP Challenge, check the OTP code sent in AuthPayload and retry."#),
            TwilioVerifyError::ErrorCode60240 => Some(r#"Please contact the support team to validate the requirements needed to be compliant with the country and avoid the override of the message. "#),
            TwilioVerifyError::ErrorCode60205 => Some(r#"* Make sure the number you are sending a verification to is a valid mobile phone number
* Disable validating phone numbers using the Twilio API setting `SkipSmsToLandlines` on your Verification Service to `false`."#),
            TwilioVerifyError::ErrorCode60519 => Some(r#"- Confirm the Verification Check is performed only after a HTTP 200 Response is received from invoking the SNA URL.
- Retry if the Verification is still in its validity window and has not expired.
- Confirm the Verification was created in the correct Twilio Region for the phone number, see [Using Verify Silent Network Auth with Twilio Regions
](https://www.twilio.com/docs/verify/using-verify-silent-network-auth-with-twilio-regions) for more detailed information. If it was not, retry Verification using correct Region."#),
            TwilioVerifyError::ErrorCode60247 => Some(r#"1. Template Substitutions: 
    * Review the length of dynamic variables or placeholders in your template, such as {{customer_name}} or {{order_number}}. These values might be longer than expected once replaced. 
    * Simplify or abbreviate the content of your template. For example, instead of "Dear {{customer_name}}, we are pleased to inform that your code is {{code}}", try "Hi {{customer_name}}, your code is {{code}}"
2. Friendly Name Substitutions:
    * If you're using a friendly name for identification, ensure it's concise. 
3. Code Substitutions:
    * Check for codes that are dynamically added to the message. Long codes may need to be shortened. For example, you could change the code length of your verify service.
4. Custom Substitutions:
    * Look for unnecessary custom substitutions that may be inflating the message length. 

These substitutions are focused on shortening the content without losing clarity, helping users keep their messages within the 500-character limit."#),
            TwilioVerifyError::ErrorCode60229 => Some(r#"Create a translation or use a existent translation for the template"#),
            TwilioVerifyError::ErrorCode60246 => Some(r#"* Use a different UniqueName."#),
            TwilioVerifyError::ErrorCode60375 => Some(r#"Check that the ID in the request body is correct."#),
            TwilioVerifyError::ErrorCode60425 => Some(r#"Check that the Verification ID in the request parameters or body is correct."#),
            TwilioVerifyError::ErrorCode60431 => Some(r#"Check that the Verification in the request parameter/body is correct."#),
            TwilioVerifyError::ErrorCode60500 => Some(r#"- Check that the phone number provided is correct.
- Retry Verification using a channel other than Silent Network Auth for this end user."#),
            TwilioVerifyError::ErrorCode60371 => Some(r#"Check that the config for relying party in the request body is correct."#),
            TwilioVerifyError::ErrorCode60401 => Some(r#"Check parameters and retry"#),
            TwilioVerifyError::ErrorCode60410 => Some(r#"You do not need to take any specific action. The block is temporary and will resolve in 12 hours if we do not detect additional fraud."#),
            TwilioVerifyError::ErrorCode60604 => Some(r#"Update your API key permissions"#),
            TwilioVerifyError::ErrorCode60311 => Some(r#"Check parameters and retry."#),
            TwilioVerifyError::ErrorCode60517 => Some(r#"Retry Verification using a channel other than Silent Network Auth for this end user."#),
            TwilioVerifyError::ErrorCode60245 => Some(r#"Contact SendGrid support to remove that limit or change your SendGrid plan. "#),
            TwilioVerifyError::ErrorCode60369 => Some(r#"Check that the Factor type in the request body is correct."#),
            TwilioVerifyError::ErrorCode60441 => Some(r#"Call Twilio support team"#),
            TwilioVerifyError::ErrorCode60602 => Some(r#"do not use app Hash with channel other than SMS"#),
            TwilioVerifyError::ErrorCode60433 => Some(r#"Try using a different approval content type"#),
            TwilioVerifyError::ErrorCode60212 => Some(r#"* Check if your application is being the target of fraudulent traffic.
* Review your implementation."#),
            TwilioVerifyError::ErrorCode60208 => Some(r#"* Use a different `UniqueName`
* Create a new service and create a rate limit with the desired `UniqueName`"#),
            TwilioVerifyError::ErrorCode60385 => Some(r#"Check that the Public key in the request body is correct."#),
            TwilioVerifyError::ErrorCode60532 => Some(r#"Retry Verification using a channel other than Silent Network Auth for this end user."#),
            TwilioVerifyError::ErrorCode60386 => Some(r#"Check that the Attestation object in the request body is correct."#),
            TwilioVerifyError::ErrorCode60390 => Some(r#"Check that the Factor in the request parameter/body is correct."#),
            TwilioVerifyError::ErrorCode60361 => Some(r#"Check that the Account SID in the request parameter/header is correct."#),
            TwilioVerifyError::ErrorCode60408 => Some(r#"Stop sending the TemplateSid param with non SMS verifications"#),
            TwilioVerifyError::ErrorCode60323 => Some(r#"Create a new Challenge."#),
            TwilioVerifyError::ErrorCode60411 => Some(r#"If you know this phone number is present in the SafeList, do nothing."#),
            TwilioVerifyError::ErrorCode60209 => Some(r#"* Remove unsupported characters from `UniqueName`"#),
            TwilioVerifyError::ErrorCode68002 => Some(r#"Create a new factor."#),
            TwilioVerifyError::ErrorCode60325 => Some(r#"If you want to add more translations, please contact support."#),
            TwilioVerifyError::ErrorCode60221 => Some(r#"Ensure that either a `To` or `VerificationSid` is provided"#),
            TwilioVerifyError::ErrorCode60363 => Some(r#"Check that the Service SID in the request parameter/header is correct."#),
            TwilioVerifyError::ErrorCode60383 => Some(r#"Complete the Factor or re-register the identity."#),
            TwilioVerifyError::ErrorCode60315 => Some(r#"Delete one of the existing Factors of the same `FactorType` to create a new one."#),
            TwilioVerifyError::ErrorCode60404 => Some(r#"Check parameters and retry."#),
            TwilioVerifyError::ErrorCode60437 => Some(r#"Try using a different recipient type"#),
            TwilioVerifyError::ErrorCode60534 => Some(r#"Retry Verification using a channel other than Silent Network Auth for this end user."#),
            TwilioVerifyError::ErrorCode60434 => Some(r#"Try using a different verification content type"#),
            TwilioVerifyError::ErrorCode60223 => Some(r#" * Enable delivery channel in Verify Service settings
 * Change the delivery channel specified in the create verification request"#),
            TwilioVerifyError::ErrorCode60368 => Some(r#"Check that the Challenge details in the request body is correct."#),
            TwilioVerifyError::ErrorCode60510 => Some(r#"- Check that the mobile client is using a mobile data connection to invoke the SNA URL, not WiFi.
- Retry Verification using a channel other than Silent Network Auth for this end user."#),
            TwilioVerifyError::ErrorCode60228 => Some(r#"Provide a TemplateSid that exists"#),
            TwilioVerifyError::ErrorCode60232 => Some(r#"Use a sender id that exists for the given account"#),
            TwilioVerifyError::ErrorCode60326 => Some(r#"Try again 24h later"#),
            TwilioVerifyError::ErrorCode60308 => Some(r#"- Create a new Challenge and verify that one.
- For TOTP, ask the user to provide the correct TOTP code or re-register them with a new Factor if they are unable to generate correct codes for the existing Factor."#),
            TwilioVerifyError::ErrorCode60317 => Some(r#"Check that the binding parameters are not the same as those in an existing Factor."#),
            TwilioVerifyError::ErrorCode60242 => Some(r#"Create and request approval for a whatsApp template in the needed language"#),
            TwilioVerifyError::ErrorCode60202 => Some(r#"You have 5 attempts to check a verification code, after that you will need to wait until the current verification expires (10 minutes) to create a new verification. Find more recommendations for avoiding rate limits while testing Verify in [this blog post](https://www.twilio.com/blog/test-verify-no-rate-limits)."#),
            TwilioVerifyError::ErrorCode60380 => Some(r#"Check that the Client data JSON in the request body is correct."#),
            TwilioVerifyError::ErrorCode60200 => Some(r#"* Use strict [E.164 formatting](https://www.twilio.com/docs/glossary/what-e164), including the `+` sign, for phone numbers in the `To` parameter. Example: `+15017122661`.
* If using the `sna` channel, ensure you have requested and been granted access. See [SNA Overview](https://www.twilio.com/docs/verify/sna) for details."#),
            TwilioVerifyError::ErrorCode60333 => Some(r#"Configure a dedicated Sender ID or phone number to send messages, instead of using a shared one.

To learn more, see <a href="https://support.twilio.com/hc/en-us/articles/12387480513307-Why-was-my-friendly-name-not-included-in-the-Verify-SMS-">Why was my friendly name not included in the Verify SMS?</a>"#),
            TwilioVerifyError::ErrorCode60313 => Some(r#"Request permissions."#),
            TwilioVerifyError::ErrorCode60427 => Some(r#"Check that the Factor ID in the request parameter or body is correct."#),
            TwilioVerifyError::ErrorCode60224 => Some(r#"- Include all the substitutions for the place holder variables with the `TemplateCustomSubstitution` parameter.
- Format the variable object as stringified JSON.

For example, the following template requires a **`uuid`** substitution:

"**Your {{friendly_name}} login link: https://example.com/verify.html?uuid={{uuid}}&code={{code}}**"

The expected request in cURL:
```bash
curl -X POST https://verify.twilio.com/v2/Services/VAXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXX/Verifications \
--data-urlencode "To=+15017122661" \
--data-urlencode "Channel=sms" \
--data-urlencode "TemplateSid=HJXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXX" \
--data-urlencode "TemplateCustomSubstitutions={ \"uuid\": \"MY_UNIQUE_ID\" }" \
-u $TWILIO_ACCOUNT_SID:$TWILIO_AUTH_TOKEN
```

The expected request in Node.js:
```javascript
const verification = await client.verify.v2
  .services("VAXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXX")
  .verifications.create({
    channel: "sms",
    templateSid: "HJXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXX",
    to: "+15017122661",
    templateCustomSubstitutions: '{ "uuid": "MY_UNIQUE_ID" }'
  });
```

The following variables are auto-populated by Twilio and do **not** need to be included in your request:

* `{{friendly_name}}`
* `{{code}}`
* `{{ttl}}`"#),
            TwilioVerifyError::ErrorCode60366 => Some(r#"Check that the Entity in the request body is correct."#),
            TwilioVerifyError::ErrorCode60550 => Some(r#"Check your debugger messages in the console to get the specific validation errors"#),
            TwilioVerifyError::ErrorCode60362 => Some(r#"Check that the Factor SID in the request parameter is correct."#),
            TwilioVerifyError::ErrorCode60220 => Some(r#"Please read this support article https://support.twilio.com/hc/en-us/articles/17024185400859-Use-Case-Vetting-for-Verify-Messages-to-China to get your more details on how to get your use case vetted for China."#),
            TwilioVerifyError::ErrorCode60372 => Some(r#"Check that the Relying party ID in the request body is correct."#),
            TwilioVerifyError::ErrorCode60511 => Some(r#"Retry Verification using a channel other than Silent Network Auth for this end user."#),
            TwilioVerifyError::ErrorCode60201 => Some(r#"Approve your template[s] in the Twilio Console: [console.twilio.com/us1/develop/verify/settings/templates](https://console.twilio.com/us1/develop/verify/settings/templates)

If your custom template is not listed, please reach out to Twilio Support to create a new template. Learn more about verification templates in the documentation: [twilio.com/docs/verify/api/templates](https://twilio.com/docs/verify/api/templates)"#),
            TwilioVerifyError::ErrorCode60436 => Some(r#"Check that the Recipient in the request body is correct."#),
            TwilioVerifyError::ErrorCode60307 => Some(r#"For factor configured with "none" as notification platform, it is not necessary to call the resend push notification endpoint."#),
            TwilioVerifyError::ErrorCode60330 => Some(r#"- Make sure the `webhook` **URL** is reachable
- Remember the `webhook` must allow **HTTP POST** requests
- Make sure the  `webhook` is responding with status code **200** when everything is going well"#),
            TwilioVerifyError::ErrorCode60376 => Some(r#"Check that the RawID in the request body is correct."#),
            TwilioVerifyError::ErrorCode60520 => Some(r#"- Retry Verification using a channel other than Silent Network Auth for this end user.
- [Contact Twilio Support](https://www.twilio.com/help/contact) to rule out integration issues."#),
            TwilioVerifyError::ErrorCode60300 => Some(r#"* Send the required parameter.
* Review the parameter type or value.
* Review the parameter length."#),
            TwilioVerifyError::ErrorCode60406 => Some(r#"Check parameters and retry."#),
            TwilioVerifyError::ErrorCode60392 => Some(r#"Check that the Entity in the request parameter/body is correct."#),
            TwilioVerifyError::ErrorCode60329 => Some(r#"- Do not use `sna` channel for a Verify `Service` with `psd2_enabled` set to `true`."#),
            TwilioVerifyError::ErrorCode60420 => Some(r#"Check that the Contact ID in the request body is correct."#),
            TwilioVerifyError::ErrorCode60370 => Some(r#"Check that the Factor config in the request body is correct."#),
            TwilioVerifyError::ErrorCode60533 => Some(r#"Retry Verification using a channel other than Silent Network Auth for this end user."#),
            TwilioVerifyError::ErrorCode60238 => Some(r#"If you are still getting this error after 72 hours, please reach out to our support team for assistance."#),
            TwilioVerifyError::ErrorCode60403 => Some(r#"Create a new factor."#),
            TwilioVerifyError::ErrorCode60387 => Some(r#"Check that the Attested credential data in the request body is correct."#),
            TwilioVerifyError::ErrorCode60531 => Some(r#"- Check that the mobile client is using a mobile data connection to invoke the SNA URL, not WiFi.
- Retry Verification using a channel other than Silent Network Auth for this end user."#),
            TwilioVerifyError::ErrorCode60239 => Some(r#"If you need access to the BYOT feature please contact Twilio Support."#),
            TwilioVerifyError::ErrorCode60402 => Some(r#"Create a new factor."#),
            TwilioVerifyError::ErrorCode68007 => Some(r#"Check parameters and retry."#),
            TwilioVerifyError::ErrorCode60364 => Some(r#"Check that the Challenge SID in the request parameter/body is correct."#),
            TwilioVerifyError::ErrorCode60244 => Some(r#"If you would like to continue using custom message text migrate to [BYOT templates](https://www.twilio.com/docs/verify/api/templates)."#),
            TwilioVerifyError::ErrorCode60225 => Some(r#"Try to create another translation with a different locale."#),
            TwilioVerifyError::ErrorCode60540 => Some(r#"- Check that the phone number provided is correct. 
- Retry Verification using a channel not based on phone number such as email. 

If fraud is suspected:
- Do not retry Verification using a channel other than Silent Network Auth for this end user.
- [Report potential fraud attempt](https://www.twilio.com/docs/verify/preventing-toll-fraud#what-to-do-if-you-suspect-fraud-on-your-twilio-account)."#),
            TwilioVerifyError::ErrorCode60423 => Some(r#"Try sending a request with more accurate contact information."#),
            TwilioVerifyError::ErrorCode60226 => Some(r#"Send the friendly name if the verification is done to a chinese phone number"#),
            TwilioVerifyError::ErrorCode60222 => Some(r#"Visit https://sendgrid.com/docs/for-developers/sending-email/sender-identity/ to see the Sender Identity requirements"#),
            TwilioVerifyError::ErrorCode60217 => Some(r#"Create a SendGrid email integration for the Service before using the email channel. [Detailed instructions available here.](https://www.twilio.com/docs/verify/email)"#),
            TwilioVerifyError::ErrorCode68005 => Some(r#"Create a new factor."#),
            TwilioVerifyError::ErrorCode60432 => Some(r#"Try using a different relying party type"#),
            TwilioVerifyError::ErrorCode60334 => Some(r#"Configure or use a dedicated template that includes the friendly name."#),
            TwilioVerifyError::ErrorCode60207 => Some(r#"* Delete/update rate limits that you are not using at the moment
* Create a new service and add the desired rate limit to the new service"#),
            TwilioVerifyError::ErrorCode60605 => Some(r#"Investigate your application for potential abuse. If you have a legitimate need to call this number, please check the following: https://support.twilio.com/hc/en-us/articles/5054044369179-Error-60605-Verification-delivery-attempt-blocked-"#),
            TwilioVerifyError::ErrorCode60428 => Some(r#"Try using a different channel type"#),
            TwilioVerifyError::ErrorCode60204 => Some(r#"If you are looking to use [custom verification codes](https://www.twilio.com/docs/verify/api/customization-options#custom-verification-codes), you must first select the **Enable Custom Verification Code** option on your [Verify service](https://console.twilio.com/us1/develop/verify/services) via the Twilio Console. [Learn more](https://www.twilio.com/docs/verify/api/customization-options#custom-verification-codes).

If you are looking to use a [custom friendly name](https://www.twilio.com/docs/verify/api/customization-options#custom-company-name), contact [Twilio Sales](https://www.twilio.com/help/sales) to have this feature enabled."#),
            TwilioVerifyError::ErrorCode60231 => Some(r#"Use a different sender id"#),
            TwilioVerifyError::ErrorCode60227 => Some(r#"Provide a supported channel for the template."#),
            TwilioVerifyError::ErrorCode60603 => Some(r#"Upgrade your Sendgrid subscription "#),
            TwilioVerifyError::ErrorCode68004 => Some(r#"Check parameters and retry."#),
            TwilioVerifyError::ErrorCode60391 => Some(r#"Check that the Challenge in the request parameter/body is correct."#),
            TwilioVerifyError::ErrorCode68003 => Some(r#"Create a new factor."#),
            TwilioVerifyError::ErrorCode60206 => Some(r#"* Make sure that you send the 'Amount' & 'Payee' params
* Set the `Psd2Enabled ` flag on your Verification Service to `false`."#),
            TwilioVerifyError::ErrorCode60219 => Some(r#"Add the required placeholders"#),
            TwilioVerifyError::ErrorCode60233 => Some(r#"Set another sender id as default and try again"#),
            TwilioVerifyError::ErrorCode60728 => Some(r#"* Wait for sometime and try again.
* Check your [support center](https://help.twilio.com/tickets) or email inbox/spam folder for outreach from the Twilio team on how to remove/raise your hourly limit."#),
            TwilioVerifyError::ErrorCode60409 => Some(r#"In case you have changed the custom message, please return to the old one and contact support before changing it again. In case you have not changed the custom message, please contact support to provide the correct template for you.
Note: Consider using templates instead of custom messages, if none of the available  templates work for you, please contact support and we can create it for you."#),
            TwilioVerifyError::ErrorCode60331 => Some(r#"Provide a supported Text-To-Speech using this documentation. https://support.twilio.com/hc/en-us/articles/223132827-What-Languages-can-the-Say-TwiML-Verb-Speak-"#),
            TwilioVerifyError::ErrorCode60405 => Some(r#"Create a new factor."#),
            TwilioVerifyError::ErrorCode60322 => Some(r#"Create a new Challenge."#),
            TwilioVerifyError::ErrorCode60379 => Some(r#"Check that the Authenticator data in the request body is correct."#),
            TwilioVerifyError::ErrorCode60328 => Some(r#"* Lower the rate at which entities are being created
* Keep in mind that this could also happen while using the create factor endpoint
* Verify that your application is not misbehaving and the load is expected. If everything is as expected then contact Customer Support to review your use case and limits"#),
            TwilioVerifyError::ErrorCode60314 => Some(r#"Make sure that the `Public Key` and `Algorithm` are correct."#),
            TwilioVerifyError::ErrorCode60302 => Some(r#"Make sure you are assigning the new `FactorType` to the right Entity."#),
            TwilioVerifyError::ErrorCode60301 => Some(r#"Make sure that there is not any other Entity with the same `Identity` associated to the same Service. "#),
            TwilioVerifyError::ErrorCode60305 => Some(r#"Check Access Token parameters."#),
            TwilioVerifyError::ErrorCode60203 => Some(r#"* Complete a verification lifecycle by calling the [Verification Check endpoint](https://www.twilio.com/docs/verify/api/verification-check).
* Wait for the verification to [expire](https://www.twilio.com/docs/verify/api/rate-limits-and-timeouts) (10 minutes).
* For use with [custom codes](https://www.twilio.com/docs/verify/api/customization-options) you can manually approve or cancel the verification by calling the [Verification Update endpoint](https://www.twilio.com/docs/verify/api/verification#update-a-verification-status).
* **For testing** you can manually cancel the verification by calling the [Verification Update endpoint](https://www.twilio.com/docs/verify/api/verification#update-a-verification-status).

If you identify a delivery issue, please reach out to Twilio Support through the [Console](https://console.twilio.com) or [Help Center](https://support.twilio.com)."#),
            TwilioVerifyError::ErrorCode60422 => Some(r#"Check that the Contact in the request body is correct."#),
            TwilioVerifyError::ErrorCode60384 => Some(r#"Check that the Challenge timeout in the request has positive value"#),
            TwilioVerifyError::ErrorCode60309 => Some(r#"Create a new Challenge and send notifications for it."#),
            TwilioVerifyError::ErrorCode60243 => Some(r#"Ensure that you pass the OTP code via [custom code](https://www.twilio.com/docs/verify/api/customization-options) parameter in the API request, following which it will be automatically map your incoming message to Verify default templates or migrate your custom messages  to use [Verify Custom Templates](https://www.twilio.com/docs/verify/api/templates)"#),
            TwilioVerifyError::ErrorCode60214 => Some(r#"* Disable PSD2 for this Verify Service
* Use SMS channel"#),
            TwilioVerifyError::ErrorCode68001 => Some(r#"Check parameters and retry"#),
            TwilioVerifyError::ErrorCode60241 => Some(r#"Please contact the support team to validate the requirements needed to be compliant with the country and avoid the override of the message."#),
            TwilioVerifyError::ErrorCode60426 => Some(r#"Check that the Verification ID in the request parameter or body is correct."#),
            TwilioVerifyError::ErrorCode60210 => Some(r#"* Delete/update Buckets that you are not using at the moment
* Create a new Rate limit and add the desired Buckets to the new Rate limit"#),
            TwilioVerifyError::ErrorCode60327 => Some(r#"If you want to add the provided channel for the given template, please contact support."#)
        }
    }
}
