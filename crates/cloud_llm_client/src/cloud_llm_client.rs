use std::str::FromStr;
use std::sync::Arc;

use anyhow::Context as _;
use serde::{Deserialize, Serialize};
use strum::{Display, EnumIter, EnumString};
use uuid::Uuid;

/// The name of the header used to indicate which version of Zed the client is running.
pub const ZED_VERSION_HEADER_NAME: &str = "x-zed-version";

/// The name of the header used to indicate which edit prediction experiment should be used.
pub const PREFERRED_EXPERIMENT_HEADER_NAME: &str = "x-zed-preferred-experiment";

/// The name of the header used to indicate when a request failed due to an
/// expired LLM token.
///
/// The client may use this as a signal to refresh the token.
pub const EXPIRED_LLM_TOKEN_HEADER_NAME: &str = "x-zed-expired-token";

/// The name of the header used to indicate when a request failed due to an outdated LLM token.
///
/// A token is considered "outdated" when we can't parse the claims (e.g., after adding a new required claim).
///
/// This is distinct from [`EXPIRED_LLM_TOKEN_HEADER_NAME`] which indicates the token's time-based validity has passed.
/// An outdated token means the token's structure is incompatible with the current server expectations.
pub const OUTDATED_LLM_TOKEN_HEADER_NAME: &str = "x-zed-outdated-token";

/// The name of the header used to indicate the minimum required Zed version.
///
/// This can be used to force a Zed upgrade in order to continue communicating
/// with the LLM service.
pub const MINIMUM_REQUIRED_VERSION_HEADER_NAME: &str = "x-zed-minimum-required-version";

/// The name of the header used by the client to indicate to the server that it supports receiving status messages.
pub const CLIENT_SUPPORTS_STATUS_MESSAGES_HEADER_NAME: &str =
    "x-zed-client-supports-status-messages";

/// The name of the header used by the client to indicate to the server that it supports receiving a "stream_ended" request completion status.
pub const CLIENT_SUPPORTS_STATUS_STREAM_ENDED_HEADER_NAME: &str =
    "x-zed-client-supports-stream-ended-request-completion-status";

/// The name of the header used by the server to indicate to the client that it supports sending status messages.
pub const SERVER_SUPPORTS_STATUS_MESSAGES_HEADER_NAME: &str =
    "x-zed-server-supports-status-messages";

#[derive(Debug, PartialEq, Clone, Copy, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum UsageLimit {
    Limited(i32),
    Unlimited,
}

impl FromStr for UsageLimit {
    type Err = anyhow::Error;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        match value {
            "unlimited" => Ok(Self::Unlimited),
            limit => limit
                .parse::<i32>()
                .map(Self::Limited)
                .context("failed to parse limit"),
        }
    }
}

#[derive(
    Debug, PartialEq, Eq, Hash, Clone, Copy, Serialize, Deserialize, EnumString, EnumIter, Display,
)]
#[serde(rename_all = "snake_case")]
#[strum(serialize_all = "snake_case")]
pub enum LanguageModelProvider {
    OpenAi,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CompletionBody {
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub thread_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub prompt_id: Option<String>,
    pub provider: LanguageModelProvider,
    pub model: String,
    pub provider_request: serde_json::Value,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CompletionRequestStatus {
    Queued {
        position: usize,
    },
    Started,
    Failed {
        code: String,
        message: String,
        request_id: Uuid,
        /// Retry duration in seconds.
        retry_after: Option<f64>,
    },
    /// The cloud sends a StreamEnded message when the stream from the LLM provider finishes.
    StreamEnded,
    #[serde(other)]
    Unknown,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CompletionEvent<T> {
    Status(CompletionRequestStatus),
    Event(T),
}

impl<T> CompletionEvent<T> {
    pub fn into_status(self) -> Option<CompletionRequestStatus> {
        match self {
            Self::Status(status) => Some(status),
            Self::Event(_) => None,
        }
    }

    pub fn into_event(self) -> Option<T> {
        match self {
            Self::Event(event) => Some(event),
            Self::Status(_) => None,
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct WebSearchBody {
    pub query: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct WebSearchResponse {
    pub results: Vec<WebSearchResult>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct WebSearchResult {
    pub title: String,
    pub url: String,
    pub text: String,
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Serialize, Deserialize)]
pub struct LanguageModelId(pub Arc<str>);

impl std::fmt::Display for LanguageModelId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct LanguageModel {
    pub provider: LanguageModelProvider,
    pub id: LanguageModelId,
    pub display_name: String,
    #[serde(default)]
    pub is_latest: bool,
    pub max_token_count: usize,
    pub max_token_count_in_max_mode: Option<usize>,
    pub max_output_tokens: usize,
    pub supports_tools: bool,
    pub supports_images: bool,
    pub supports_thinking: bool,
    #[serde(default)]
    pub supports_fast_mode: bool,
    pub supported_effort_levels: Vec<SupportedEffortLevel>,
    #[serde(default)]
    pub supports_streaming_tools: bool,
    /// Only used by OpenAI.
    #[serde(default)]
    pub supports_parallel_tool_calls: bool,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SupportedEffortLevel {
    pub name: Arc<str>,
    pub value: Arc<str>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub is_default: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ListModelsResponse {
    pub models: Vec<LanguageModel>,
    pub default_model: Option<LanguageModelId>,
    pub default_fast_model: Option<LanguageModelId>,
    pub recommended_models: Vec<LanguageModelId>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_usage_limit_from_str() {
        let limit = UsageLimit::from_str("unlimited").unwrap();
        assert!(matches!(limit, UsageLimit::Unlimited));

        let limit = UsageLimit::from_str(&0.to_string()).unwrap();
        assert!(matches!(limit, UsageLimit::Limited(0)));

        let limit = UsageLimit::from_str(&50.to_string()).unwrap();
        assert!(matches!(limit, UsageLimit::Limited(50)));

        for value in ["not_a_number", "50xyz"] {
            let limit = UsageLimit::from_str(value);
            assert!(limit.is_err());
        }
    }
}
