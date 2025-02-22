// AUTO-GENERATED. DO NOT MODIFY. YOUR CHANGES WILL BE OVER-WRITTEN
use crate::products::TwilioUnderstandError;
use standard_error::traits::StandardErrorDocsTrait;

impl StandardErrorDocsTrait for TwilioUnderstandError {
    fn docs(&self) -> &'static str {
        match self {
            TwilioUnderstandError::ErrorCode90403 => r#"## ERROR - 90403

### [Autopilot] Signature validation failed

 Signature validation is failed. 

#### Possible Causes
The account owns the messaging/voice service does not have any parent/sub-account relationship with the account owns the Autopilot assistant. 

#### Possible Solutions
Moving the assistant (export and import) into the service account or service account's one of the sub-accounts would solve the problem.  
"#,
            TwilioUnderstandError::ErrorCode90104 => r#"## ERROR - 90104

### Invalid Collect Field Type

 The Field Type in Collect JSON is Invalid

#### Possible Causes
Collect Question has an Invalid Field Type

#### Possible Solutions
Please ensure the type in the Collect Question is valid."#,
            TwilioUnderstandError::ErrorCode90103 => r#"## ERROR - 90103

### Error processing answer during collection

 Error processing answer during collection

#### Possible Causes
An internal error rendered us unable to process the answer. Apologies!

#### Possible Solutions
If the error persists, <a href="/help/contact">contact us</a> to figure out what has happened and how to fix it.
Note the time of the error and what you were trying to do when it occurred. Thank you!"#,
            TwilioUnderstandError::ErrorCode90102 => r#"## ERROR - 90102

### Assistant failure to start collection

 Autopilot was unable to start the collection

#### Possible Causes
An internal error rendered us unable to start the collection. Apologies!

#### Possible Solutions
If the error persists, <a href="/help/contact">contact us</a> to figure out what has happened and how to fix it.
Note the time of the error and what you were trying to do when it occurred. Thank you!"#,
            TwilioUnderstandError::ErrorCode90101 => r#"## ERROR - 90101

### Unique Name Already Exists

 Unique name user created already exists

#### Possible Causes
A same unique name has been created before

#### Possible Solutions
Change to a new unique name"#,
            TwilioUnderstandError::ErrorCode90100 => r#"## ERROR - 90100

### Invalid Autopilot Actions JSON

 Autopilot Actions Configured on the Task are Invalid

#### Possible Causes
Actions JSON does not comply with the Actions Schema (https://carnelian-neanderthal-8008.twil.io/assets/ActionsSchema.json)

#### Possible Solutions
Test your JSON response against the Actions Schema (https://carnelian-neanderthal-8008.twil.io/assets/ActionsSchema.json)"#
        }
    }
}
