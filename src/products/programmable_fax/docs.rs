// AUTO-GENERATED. DO NOT MODIFY. YOUR CHANGES WILL BE OVER-WRITTEN
use crate::products::TwilioProgrammableFaxError;
use standard_error::traits::StandardErrorDocsTrait;

impl StandardErrorDocsTrait for TwilioProgrammableFaxError {
    fn docs(&self) -> &'static str {
        match self {
            TwilioProgrammableFaxError::ErrorCode34004 => r#"## Error - 34004
### Fax: Error during fax transmission

Either poor connection quality to the recipient or the recipient terminated the fax early."#,
            TwilioProgrammableFaxError::ErrorCode34003 => r#"## Error - 34003
### Fax: Callee did not answer
"#,
            TwilioProgrammableFaxError::ErrorCode34108 => r#"## Error - 34108
### Fax: Other End Incompatible

The receiving fax machine answered, but then rejected your incoming media.

Try reducing the size of your pdf or sending at a lower quality or resolution. Make sure your pdf is a standard size (US Letter, A4) and portrait-oriented."#,
            TwilioProgrammableFaxError::ErrorCode34106 => r#"## Error - 34106
### Fax: No Fax TwiML action specified

Missing [Receive](https://www.twilio.com/docs/api/fax/receive#receive) verb in received TwiML.

Simplest implementation is `<Receive />`"#,
            TwilioProgrammableFaxError::ErrorCode34002 => r#"## Error - 34002
### Fax: Callee Busy
"#,
            TwilioProgrammableFaxError::ErrorCode34005 => r#"## ERROR - 34005

### Programmable Fax is no longer available

Twilio ended support for Programmable Fax for all accounts on December 17, 2021 The Programmable Fax API by Twilio is no longer supported

#### Possible Causes
You are trying to use the Programmable Fax API which is no longer available

#### Possible Solutions
Please consult the migration guide on: https://support.twilio.com/hc/en-us/articles/223136667-Fax-Support-on-Twilio"#
        }
    }
}
