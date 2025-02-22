// AUTO-GENERATED. DO NOT MODIFY. YOUR CHANGES WILL BE OVER-WRITTEN
use crate::products::TwilioElasticSipTrunkingError;
use standard_error::traits::StandardErrorMessageTrait;
use std::borrow::Cow;

impl StandardErrorMessageTrait for TwilioElasticSipTrunkingError {
    fn message(&self) -> Cow<'static, str> {
        match self {
            TwilioElasticSipTrunkingError::ErrorCode21244 => r#"Maximum Number of Trunks reached"#.into(),
            TwilioElasticSipTrunkingError::ErrorCode21257 => r#"Invalid SIP Manipulation Policy SID"#.into(),
            TwilioElasticSipTrunkingError::ErrorCode21253 => r#"Max Connection Policies Reached"#.into(),
            TwilioElasticSipTrunkingError::ErrorCode32208 => r#"SIP: Secure media required"#.into(),
            TwilioElasticSipTrunkingError::ErrorCode32219 => r#"SIP: Redirect failed"#.into(),
            TwilioElasticSipTrunkingError::ErrorCode21248 => r#"Trunk Domain already taken"#.into(),
            TwilioElasticSipTrunkingError::ErrorCode32202 => r#"SIP: Bad user credentials"#.into(),
            TwilioElasticSipTrunkingError::ErrorCode32205 => r#"SIP Trunking: Geo Permission configuration is not permitting call"#.into(),
            TwilioElasticSipTrunkingError::ErrorCode21252 => r#"Invalid Region"#.into(),
            TwilioElasticSipTrunkingError::ErrorCode32207 => r#"SIP: Secure media not accepted"#.into(),
            TwilioElasticSipTrunkingError::ErrorCode21249 => r#"Maximum Origination URIs reached  "#.into(),
            TwilioElasticSipTrunkingError::ErrorCode21256 => r#"Invalid ruleset"#.into(),
            TwilioElasticSipTrunkingError::ErrorCode21259 => r#"Maximum number of SIP Manipulation Polies per account reached"#.into(),
            TwilioElasticSipTrunkingError::ErrorCode32218 => r#"SIP: Transfer not allowed"#.into(),
            TwilioElasticSipTrunkingError::ErrorCode32001 => r#"SIP: Trunk CPS limit exceeded"#.into(),
            TwilioElasticSipTrunkingError::ErrorCode32005 => r#"Voice calling has been disabled for this account"#.into(),
            TwilioElasticSipTrunkingError::ErrorCode21247 => r#"Trunk Dependencies"#.into(),
            TwilioElasticSipTrunkingError::ErrorCode32203 => r#"SIP: Call blocked by Twilio"#.into(),
            TwilioElasticSipTrunkingError::ErrorCode32100 => r#"SIP: Trial accounts can only call verified caller IDs"#.into(),
            TwilioElasticSipTrunkingError::ErrorCode32012 => r#"SIP: Parent account pooled Trunking CPS limit exceeded"#.into(),
            TwilioElasticSipTrunkingError::ErrorCode32011 => r#"Error communicating with your SIP communications infrastructure"#.into(),
            TwilioElasticSipTrunkingError::ErrorCode32201 => r#"SIP: Source IP address not in ACL"#.into(),
            TwilioElasticSipTrunkingError::ErrorCode21245 => r#"Trunk Validation Error"#.into(),
            TwilioElasticSipTrunkingError::ErrorCode32010 => r#"SIP: No valid Origination URIs configured"#.into(),
            TwilioElasticSipTrunkingError::ErrorCode32206 => r#"SIP: Invalid From number (caller ID)"#.into(),
            TwilioElasticSipTrunkingError::ErrorCode21260 => r#"Maximum number of actions per rule reached"#.into(),
            TwilioElasticSipTrunkingError::ErrorCode21261 => r#"Maximum number of conditions per rule reached"#.into(),
            TwilioElasticSipTrunkingError::ErrorCode32115 => r#"X-Branded-CallReason header contains an invalid value."#.into(),
            TwilioElasticSipTrunkingError::ErrorCode21258 => r#"Invalid SIP Manipulation Policy"#.into(),
            TwilioElasticSipTrunkingError::ErrorCode21634 => r#"SIP Trunk is in use for emergency calling"#.into(),
            TwilioElasticSipTrunkingError::ErrorCode32222 => r#"TLS version not allowed"#.into(),
            TwilioElasticSipTrunkingError::ErrorCode21254 => r#"Max Connection Policy Entries Reached"#.into(),
            TwilioElasticSipTrunkingError::ErrorCode21251 => r#"Trunking CPS change not allowed"#.into(),
            TwilioElasticSipTrunkingError::ErrorCode32204 => r#"SIP: 'From' phone number not verified"#.into(),
            TwilioElasticSipTrunkingError::ErrorCode32200 => r#"SIP: Insufficient permissions"#.into(),
            TwilioElasticSipTrunkingError::ErrorCode32020 => r#"SHAKEN/STIR call verification failed due to invalid passport from customer"#.into()
        }
    }
}
