// AUTO-GENERATED. DO NOT MODIFY. YOUR CHANGES WILL BE OVER-WRITTEN
use standard_error::traits::StandardErrorFromCodeTrait;
use crate::TwilioProductError;
use super::*;

impl StandardErrorFromCodeTrait for TwilioProductError {
    fn from_code(code: usize) -> Option<Self>
    where
        Self: Sized
    {
        None
            .or_else(|| TwilioPhoneNumbersError::from_code(code).map(Into::into))
            .or_else(|| TwilioProxyError::from_code(code).map(Into::into))
            .or_else(|| TwilioMiscellaneousError::from_code(code).map(Into::into))
            .or_else(|| TwilioProgrammableVoiceError::from_code(code).map(Into::into))
            .or_else(|| TwilioVoiceIntelligenceError::from_code(code).map(Into::into))
            .or_else(|| TwilioInterconnectError::from_code(code).map(Into::into))
            .or_else(|| TwilioFunctionsError::from_code(code).map(Into::into))
            .or_else(|| TwilioLookupError::from_code(code).map(Into::into))
            .or_else(|| TwilioProgrammableSmsError::from_code(code).map(Into::into))
            .or_else(|| TwilioVerifyError::from_code(code).map(Into::into))
            .or_else(|| TwilioProgrammableVideoError::from_code(code).map(Into::into))
            .or_else(|| TwilioProgrammableChatError::from_code(code).map(Into::into))
            .or_else(|| TwilioStudioError::from_code(code).map(Into::into))
            .or_else(|| TwilioNotificationsError::from_code(code).map(Into::into))
            .or_else(|| TwilioOrganizationsApiError::from_code(code).map(Into::into))
            .or_else(|| TwilioTaskrouterError::from_code(code).map(Into::into))
            .or_else(|| TwilioProgrammableWirelessError::from_code(code).map(Into::into))
            .or_else(|| TwilioEventsError::from_code(code).map(Into::into))
            .or_else(|| TwilioFlexError::from_code(code).map(Into::into))
            .or_else(|| TwilioElasticSipTrunkingError::from_code(code).map(Into::into))
            .or_else(|| TwilioProgrammableMessagingError::from_code(code).map(Into::into))
            .or_else(|| TwilioChannelsError::from_code(code).map(Into::into))
            .or_else(|| TwilioSyncError::from_code(code).map(Into::into))
            .or_else(|| TwilioContactsError::from_code(code).map(Into::into))
            .or_else(|| TwilioSuperSimError::from_code(code).map(Into::into))
            .or_else(|| TwilioFrontlineError::from_code(code).map(Into::into))
            .or_else(|| TwilioUnderstandError::from_code(code).map(Into::into))
            .or_else(|| TwilioBrandedCommsError::from_code(code).map(Into::into))
            .or_else(|| TwilioConversationsError::from_code(code).map(Into::into))
            .or_else(|| TwilioProgrammableFaxError::from_code(code).map(Into::into))
            .or_else(|| TwilioPlatformError::from_code(code).map(Into::into))
            .or_else(|| TwilioAiAssistantsError::from_code(code).map(Into::into))
            .or_else(|| TwilioNotifyError::from_code(code).map(Into::into))
    }
}
