use std::sync::Arc;

use collections::HashMap;
use settings::RegisterSetting;

use crate::provider::{
    cloud::ZedDotDevSettings, google::GoogleSettings, lmstudio::LmStudioSettings,
    mistral::MistralSettings, ollama::OllamaSettings, open_ai::OpenAiSettings,
    open_ai_compatible::OpenAiCompatibleSettings, open_router::OpenRouterSettings,
    opencode::OpenCodeSettings,
};

#[derive(Debug, RegisterSetting)]
pub struct AllLanguageModelSettings {
    pub google: GoogleSettings,
    pub lmstudio: LmStudioSettings,
    pub mistral: MistralSettings,
    pub ollama: OllamaSettings,
    pub opencode: OpenCodeSettings,
    pub open_router: OpenRouterSettings,
    pub openai: OpenAiSettings,
    pub openai_compatible: HashMap<Arc<str>, OpenAiCompatibleSettings>,
    pub zed_dot_dev: ZedDotDevSettings,
}

impl settings::Settings for AllLanguageModelSettings {
    const PRESERVED_KEYS: Option<&'static [&'static str]> = Some(&["version"]);

    fn from_settings(content: &settings::SettingsContent) -> Self {
        let language_models = content.language_models.clone().unwrap();
        let google = language_models.google.unwrap();
        let lmstudio = language_models.lmstudio.unwrap();
        let mistral = language_models.mistral.unwrap();
        let ollama = language_models.ollama.unwrap();
        let opencode = language_models.opencode.unwrap();
        let open_router = language_models.open_router.unwrap();
        let openai = language_models.openai.unwrap();
        let openai_compatible = language_models.openai_compatible.unwrap();
        let zed_dot_dev = language_models.zed_dot_dev.unwrap();
        Self {
            google: GoogleSettings {
                api_url: google.api_url.unwrap(),
                available_models: google.available_models.unwrap_or_default(),
            },
            lmstudio: LmStudioSettings {
                api_url: lmstudio.api_url.unwrap(),
                available_models: lmstudio.available_models.unwrap_or_default(),
            },
            mistral: MistralSettings {
                api_url: mistral.api_url.unwrap(),
                available_models: mistral.available_models.unwrap_or_default(),
            },
            ollama: OllamaSettings {
                api_url: ollama.api_url.unwrap(),
                auto_discover: ollama.auto_discover.unwrap_or(true),
                available_models: ollama.available_models.unwrap_or_default(),
                context_window: ollama.context_window,
            },
            opencode: OpenCodeSettings {
                api_url: opencode.api_url.unwrap(),
                available_models: opencode.available_models.unwrap_or_default(),
                show_zen_models: opencode.show_zen_models.unwrap_or(true),
                show_go_models: opencode.show_go_models.unwrap_or(true),
                show_free_models: opencode.show_free_models.unwrap_or(true),
            },
            open_router: OpenRouterSettings {
                api_url: open_router.api_url.unwrap(),
                available_models: open_router.available_models.unwrap_or_default(),
            },
            openai: OpenAiSettings {
                api_url: openai.api_url.unwrap(),
                available_models: openai.available_models.unwrap_or_default(),
            },
            openai_compatible: openai_compatible
                .into_iter()
                .map(|(key, value)| {
                    (
                        key,
                        OpenAiCompatibleSettings {
                            api_url: value.api_url,
                            available_models: value.available_models,
                        },
                    )
                })
                .collect(),
            zed_dot_dev: ZedDotDevSettings {
                available_models: zed_dot_dev.available_models.unwrap_or_default(),
            },
        }
    }
}
