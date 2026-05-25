use crate::{LanguageModelProviderId, LanguageModelProviderName};

pub const OPEN_AI_PROVIDER_ID: LanguageModelProviderId = LanguageModelProviderId::new("openai");
pub const OPEN_AI_PROVIDER_NAME: LanguageModelProviderName =
    LanguageModelProviderName::new("OpenAI");

pub const ZED_CLOUD_PROVIDER_ID: LanguageModelProviderId = LanguageModelProviderId::new("zed.dev");
pub const ZED_CLOUD_PROVIDER_NAME: LanguageModelProviderName =
    LanguageModelProviderName::new("Zed");
