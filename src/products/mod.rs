// AUTO-GENERATED. DO NOT MODIFY. YOUR CHANGES WILL BE OVER-WRITTEN
mod phone_numbers;
use phone_numbers::*;

mod proxy;
use proxy::*;

mod miscellaneous;
use miscellaneous::*;

mod programmable_voice;
use programmable_voice::*;

mod voice_intelligence;
use voice_intelligence::*;

mod interconnect;
use interconnect::*;

mod functions;
use functions::*;

mod lookup;
use lookup::*;

mod programmable_sms;
use programmable_sms::*;

mod verify;
use verify::*;

mod programmable_video;
use programmable_video::*;

mod programmable_chat;
use programmable_chat::*;

mod studio;
use studio::*;

mod notifications;
use notifications::*;

mod organizations_api;
use organizations_api::*;

mod taskrouter;
use taskrouter::*;

mod programmable_wireless;
use programmable_wireless::*;

mod events;
use events::*;

mod flex;
use flex::*;

mod elastic_sip_trunking;
use elastic_sip_trunking::*;

mod programmable_messaging;
use programmable_messaging::*;

mod channels;
use channels::*;

mod sync;
use sync::*;

mod contacts;
use contacts::*;

mod super_sim;
use super_sim::*;

mod frontline;
use frontline::*;

mod understand;
use understand::*;

mod branded_comms;
use branded_comms::*;

mod conversations;
use conversations::*;

mod programmable_fax;
use programmable_fax::*;

mod platform;
use platform::*;

mod ai_assistants;
use ai_assistants::*;

mod notify;
use notify::*;
mod from_code;
pub use from_code::*;

use serde::{Deserialize, Deserializer, Serialize, Serializer};
use serde::de::Error;
use standard_error::traits::StandardErrorFromCodeTrait;

#[derive(Debug, Clone)]
pub enum TwilioProductError {
    PhoneNumbers(TwilioPhoneNumbersError),
    Proxy(TwilioProxyError),
    Miscellaneous(TwilioMiscellaneousError),
    ProgrammableVoice(TwilioProgrammableVoiceError),
    VoiceIntelligence(TwilioVoiceIntelligenceError),
    Interconnect(TwilioInterconnectError),
    Functions(TwilioFunctionsError),
    Lookup(TwilioLookupError),
    ProgrammableSms(TwilioProgrammableSmsError),
    Verify(TwilioVerifyError),
    ProgrammableVideo(TwilioProgrammableVideoError),
    ProgrammableChat(TwilioProgrammableChatError),
    Studio(TwilioStudioError),
    Notifications(TwilioNotificationsError),
    OrganizationsApi(TwilioOrganizationsApiError),
    Taskrouter(TwilioTaskrouterError),
    ProgrammableWireless(TwilioProgrammableWirelessError),
    Events(TwilioEventsError),
    Flex(TwilioFlexError),
    ElasticSipTrunking(TwilioElasticSipTrunkingError),
    ProgrammableMessaging(TwilioProgrammableMessagingError),
    Channels(TwilioChannelsError),
    Sync(TwilioSyncError),
    Contacts(TwilioContactsError),
    SuperSim(TwilioSuperSimError),
    Frontline(TwilioFrontlineError),
    Understand(TwilioUnderstandError),
    BrandedComms(TwilioBrandedCommsError),
    Conversations(TwilioConversationsError),
    ProgrammableFax(TwilioProgrammableFaxError),
    Platform(TwilioPlatformError),
    AiAssistants(TwilioAiAssistantsError),
    Notify(TwilioNotifyError)
}

use standard_error::traits::*;
pub trait SomeTwilioProductError: StandardErrorFromCodeTrait + StandardErrorCodeTrait + StandardErrorDescriptionTrait + StandardErrorCausesTrait + StandardErrorDocsTrait + StandardErrorMessageTrait + StandardErrorSolutionsTrait {}

impl TwilioProductError {
    pub fn inner(&self) -> Box<dyn SomeTwilioProductError> {
        self.clone().into_inner()
    }
    pub fn into_inner(
        self,
    ) -> Box<dyn SomeTwilioProductError> {
        match self {
            TwilioProductError::PhoneNumbers(v) => Box::new(v) as Box<dyn SomeTwilioProductError>,
            TwilioProductError::Proxy(v) => Box::new(v) as Box<dyn SomeTwilioProductError>,
            TwilioProductError::Miscellaneous(v) => Box::new(v) as Box<dyn SomeTwilioProductError>,
            TwilioProductError::ProgrammableVoice(v) => Box::new(v) as Box<dyn SomeTwilioProductError>,
            TwilioProductError::VoiceIntelligence(v) => Box::new(v) as Box<dyn SomeTwilioProductError>,
            TwilioProductError::Interconnect(v) => Box::new(v) as Box<dyn SomeTwilioProductError>,
            TwilioProductError::Functions(v) => Box::new(v) as Box<dyn SomeTwilioProductError>,
            TwilioProductError::Lookup(v) => Box::new(v) as Box<dyn SomeTwilioProductError>,
            TwilioProductError::ProgrammableSms(v) => Box::new(v) as Box<dyn SomeTwilioProductError>,
            TwilioProductError::Verify(v) => Box::new(v) as Box<dyn SomeTwilioProductError>,
            TwilioProductError::ProgrammableVideo(v) => Box::new(v) as Box<dyn SomeTwilioProductError>,
            TwilioProductError::ProgrammableChat(v) => Box::new(v) as Box<dyn SomeTwilioProductError>,
            TwilioProductError::Studio(v) => Box::new(v) as Box<dyn SomeTwilioProductError>,
            TwilioProductError::Notifications(v) => Box::new(v) as Box<dyn SomeTwilioProductError>,
            TwilioProductError::OrganizationsApi(v) => Box::new(v) as Box<dyn SomeTwilioProductError>,
            TwilioProductError::Taskrouter(v) => Box::new(v) as Box<dyn SomeTwilioProductError>,
            TwilioProductError::ProgrammableWireless(v) => Box::new(v) as Box<dyn SomeTwilioProductError>,
            TwilioProductError::Events(v) => Box::new(v) as Box<dyn SomeTwilioProductError>,
            TwilioProductError::Flex(v) => Box::new(v) as Box<dyn SomeTwilioProductError>,
            TwilioProductError::ElasticSipTrunking(v) => Box::new(v) as Box<dyn SomeTwilioProductError>,
            TwilioProductError::ProgrammableMessaging(v) => Box::new(v) as Box<dyn SomeTwilioProductError>,
            TwilioProductError::Channels(v) => Box::new(v) as Box<dyn SomeTwilioProductError>,
            TwilioProductError::Sync(v) => Box::new(v) as Box<dyn SomeTwilioProductError>,
            TwilioProductError::Contacts(v) => Box::new(v) as Box<dyn SomeTwilioProductError>,
            TwilioProductError::SuperSim(v) => Box::new(v) as Box<dyn SomeTwilioProductError>,
            TwilioProductError::Frontline(v) => Box::new(v) as Box<dyn SomeTwilioProductError>,
            TwilioProductError::Understand(v) => Box::new(v) as Box<dyn SomeTwilioProductError>,
            TwilioProductError::BrandedComms(v) => Box::new(v) as Box<dyn SomeTwilioProductError>,
            TwilioProductError::Conversations(v) => Box::new(v) as Box<dyn SomeTwilioProductError>,
            TwilioProductError::ProgrammableFax(v) => Box::new(v) as Box<dyn SomeTwilioProductError>,
            TwilioProductError::Platform(v) => Box::new(v) as Box<dyn SomeTwilioProductError>,
            TwilioProductError::AiAssistants(v) => Box::new(v) as Box<dyn SomeTwilioProductError>,
            TwilioProductError::Notify(v) => Box::new(v) as Box<dyn SomeTwilioProductError>
        }
    }
}

impl<'de> Deserialize<'de> for TwilioProductError {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>
    {
        Ok(
            usize::deserialize(deserializer)
                .map(TwilioProductError::from_code)?
                .ok_or_else(|| Error::custom("Invalid error code!"))?
        )
    }
}

impl Serialize for TwilioProductError {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        usize::serialize(&self.inner().code(), serializer)
    }
}
