// AUTO-GENERATED. DO NOT MODIFY. YOUR CHANGES WILL BE OVER-WRITTEN
use crate::products::TwilioProxyError;
use standard_error::traits::StandardErrorFromCodeTrait;

impl StandardErrorFromCodeTrait for TwilioProxyError {
    fn from_code(code: usize) -> Option<Self> {
        match code {
            80612 => Some(TwilioProxyError::ErrorCode80612),
            80611 => Some(TwilioProxyError::ErrorCode80611),
            80911 => Some(TwilioProxyError::ErrorCode80911),
            80308 => Some(TwilioProxyError::ErrorCode80308),
            80623 => Some(TwilioProxyError::ErrorCode80623),
            80621 => Some(TwilioProxyError::ErrorCode80621),
            80619 => Some(TwilioProxyError::ErrorCode80619),
            80607 => Some(TwilioProxyError::ErrorCode80607),
            80306 => Some(TwilioProxyError::ErrorCode80306),
            80904 => Some(TwilioProxyError::ErrorCode80904),
            80504 => Some(TwilioProxyError::ErrorCode80504),
            80802 => Some(TwilioProxyError::ErrorCode80802),
            80614 => Some(TwilioProxyError::ErrorCode80614),
            80208 => Some(TwilioProxyError::ErrorCode80208),
            80301 => Some(TwilioProxyError::ErrorCode80301),
            80101 => Some(TwilioProxyError::ErrorCode80101),
            80617 => Some(TwilioProxyError::ErrorCode80617),
            80206 => Some(TwilioProxyError::ErrorCode80206),
            80913 => Some(TwilioProxyError::ErrorCode80913),
            80502 => Some(TwilioProxyError::ErrorCode80502),
            80624 => Some(TwilioProxyError::ErrorCode80624),
            80622 => Some(TwilioProxyError::ErrorCode80622),
            80501 => Some(TwilioProxyError::ErrorCode80501),
            80618 => Some(TwilioProxyError::ErrorCode80618),
            80304 => Some(TwilioProxyError::ErrorCode80304),
            80901 => Some(TwilioProxyError::ErrorCode80901),
            80505 => Some(TwilioProxyError::ErrorCode80505),
            80207 => Some(TwilioProxyError::ErrorCode80207),
            80203 => Some(TwilioProxyError::ErrorCode80203),
            80201 => Some(TwilioProxyError::ErrorCode80201),
            80613 => Some(TwilioProxyError::ErrorCode80613),
            80910 => Some(TwilioProxyError::ErrorCode80910),
            80305 => Some(TwilioProxyError::ErrorCode80305),
            80909 => Some(TwilioProxyError::ErrorCode80909),
            80103 => Some(TwilioProxyError::ErrorCode80103),
            80616 => Some(TwilioProxyError::ErrorCode80616),
            80620 => Some(TwilioProxyError::ErrorCode80620),
            80801 => Some(TwilioProxyError::ErrorCode80801),
            80506 => Some(TwilioProxyError::ErrorCode80506),
            80404 => Some(TwilioProxyError::ErrorCode80404),
            80608 => Some(TwilioProxyError::ErrorCode80608),
            80615 => Some(TwilioProxyError::ErrorCode80615),
            80625 => Some(TwilioProxyError::ErrorCode80625),
            _ => None
        }
    }
}
