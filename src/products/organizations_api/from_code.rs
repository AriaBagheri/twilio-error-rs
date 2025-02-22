// AUTO-GENERATED. DO NOT MODIFY. YOUR CHANGES WILL BE OVER-WRITTEN
use crate::products::TwilioOrganizationsApiError;
use standard_error::traits::StandardErrorFromCodeTrait;

impl StandardErrorFromCodeTrait for TwilioOrganizationsApiError {
    fn from_code(code: usize) -> Option<Self> {
        match code {
            25008 => Some(TwilioOrganizationsApiError::ErrorCode25008),
            25013 => Some(TwilioOrganizationsApiError::ErrorCode25013),
            25004 => Some(TwilioOrganizationsApiError::ErrorCode25004),
            25015 => Some(TwilioOrganizationsApiError::ErrorCode25015),
            25203 => Some(TwilioOrganizationsApiError::ErrorCode25203),
            25106 => Some(TwilioOrganizationsApiError::ErrorCode25106),
            25100 => Some(TwilioOrganizationsApiError::ErrorCode25100),
            25201 => Some(TwilioOrganizationsApiError::ErrorCode25201),
            25001 => Some(TwilioOrganizationsApiError::ErrorCode25001),
            25018 => Some(TwilioOrganizationsApiError::ErrorCode25018),
            25021 => Some(TwilioOrganizationsApiError::ErrorCode25021),
            25022 => Some(TwilioOrganizationsApiError::ErrorCode25022),
            25002 => Some(TwilioOrganizationsApiError::ErrorCode25002),
            25016 => Some(TwilioOrganizationsApiError::ErrorCode25016),
            25007 => Some(TwilioOrganizationsApiError::ErrorCode25007),
            25006 => Some(TwilioOrganizationsApiError::ErrorCode25006),
            25101 => Some(TwilioOrganizationsApiError::ErrorCode25101),
            25023 => Some(TwilioOrganizationsApiError::ErrorCode25023),
            25009 => Some(TwilioOrganizationsApiError::ErrorCode25009),
            25005 => Some(TwilioOrganizationsApiError::ErrorCode25005),
            25017 => Some(TwilioOrganizationsApiError::ErrorCode25017),
            25107 => Some(TwilioOrganizationsApiError::ErrorCode25107),
            25011 => Some(TwilioOrganizationsApiError::ErrorCode25011),
            25109 => Some(TwilioOrganizationsApiError::ErrorCode25109),
            25003 => Some(TwilioOrganizationsApiError::ErrorCode25003),
            25200 => Some(TwilioOrganizationsApiError::ErrorCode25200),
            25103 => Some(TwilioOrganizationsApiError::ErrorCode25103),
            25202 => Some(TwilioOrganizationsApiError::ErrorCode25202),
            25019 => Some(TwilioOrganizationsApiError::ErrorCode25019),
            25012 => Some(TwilioOrganizationsApiError::ErrorCode25012),
            25014 => Some(TwilioOrganizationsApiError::ErrorCode25014),
            25104 => Some(TwilioOrganizationsApiError::ErrorCode25104),
            25102 => Some(TwilioOrganizationsApiError::ErrorCode25102),
            25020 => Some(TwilioOrganizationsApiError::ErrorCode25020),
            25010 => Some(TwilioOrganizationsApiError::ErrorCode25010),
            25105 => Some(TwilioOrganizationsApiError::ErrorCode25105),
            _ => None
        }
    }
}
