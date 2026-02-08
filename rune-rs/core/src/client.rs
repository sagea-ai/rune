//! Session- and turn-scoped helpers for talking to model provider APIs.
//!
//! This version has been stripped of rune-api dependencies and currently provides
//! stubs to satisfy the build. Real local model integration will be added here.

use std::sync::Arc;
use std::sync::Mutex;
use std::sync::OnceLock;
use std::sync::atomic::AtomicBool;

use crate::api_bridge::CoreAuthProvider;
use crate::client_common::Prompt;
use crate::client_common::ResponseEvent;
use crate::client_common::ResponseStream;
use crate::error::Result;
use crate::model_provider_info::ModelProviderInfo;

use rune_protocol::ThreadId;
use rune_protocol::config_types::ReasoningSummary as ReasoningSummaryConfig;
use rune_protocol::config_types::Verbosity as VerbosityConfig;
use rune_protocol::models::ResponseItem;
use rune_protocol::openai_models::ModelInfo;
use rune_protocol::openai_models::ReasoningEffort as ReasoningEffortConfig;
use rune_protocol::protocol::SessionSource;

use futures::StreamExt; // For stream mapping if needed
use rune_otel::OtelManager;
use tokio::sync::oneshot;
use tokio::task::JoinHandle;

use crate::AuthManager;
use crate::auth::RuneAuth;
use crate::memory_trace::ApiMemoryTrace;

// Import OllamaError as ApiError to satisfy OtelManager and general usage
pub use rune_protocol::ollama_types::OllamaError as ApiError;

// --- Stubs for types previously from rune-api ---

pub struct ApiMemoryTraceSummaryOutput {
    pub trace_summary: String,
    pub memory_summary: String,
}

pub struct ApiWebSocketConnection;

// Stub implementation for RequestTelemetry
pub trait RequestTelemetry: Send + Sync {}
pub struct ApiTelemetry;
impl ApiTelemetry {
    pub fn new(_: OtelManager) -> Self { Self }
}
impl RequestTelemetry for ApiTelemetry {}

// Stub for provider
pub struct ApiProvider;

// Stub for ApiResponsesOptions
pub struct ApiResponsesOptions;

// Stub for Compression
#[derive(Debug, Clone, Copy, Default)]
pub struct Compression;

// --- End Stubs ---

struct PreconnectedWebSocket {
    // connection: ApiWebSocketConnection, // Stubbed out
    turn_state: Option<String>,
}

type PreconnectTask = JoinHandle<Option<PreconnectedWebSocket>>;

/// Session-scoped state shared by all [`ModelClient`] clones.
struct ModelClientState {
    auth_manager: Option<Arc<AuthManager>>,
    conversation_id: ThreadId,
    provider: ModelProviderInfo,
    session_source: SessionSource,
    model_verbosity: Option<VerbosityConfig>,
    enable_responses_websockets: bool,
    enable_responses_websockets_v2: bool,
    enable_request_compression: bool,
    include_timing_metrics: bool,
    beta_features_header: Option<String>,
    disable_websockets: AtomicBool,

    preconnect: Mutex<Option<PreconnectTask>>,
}

impl std::fmt::Debug for ModelClientState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ModelClientState")
            .field("conversation_id", &self.conversation_id)
            .finish()
    }
}

struct CurrentClientSetup {
    auth: Option<RuneAuth>,
    api_provider: ApiProvider,
    api_auth: CoreAuthProvider,
}

#[derive(Debug, Clone)]
pub struct ModelClient {
    state: Arc<ModelClientState>,
}

pub struct ModelClientSession {
    client: ModelClient,
    connection: Option<ApiWebSocketConnection>,
    turn_state: Arc<OnceLock<String>>,
}

impl ModelClient {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        auth_manager: Option<Arc<AuthManager>>,
        conversation_id: ThreadId,
        provider: ModelProviderInfo,
        session_source: SessionSource,
        model_verbosity: Option<VerbosityConfig>,
        enable_responses_websockets: bool,
        enable_responses_websockets_v2: bool,
        enable_request_compression: bool,
        include_timing_metrics: bool,
        beta_features_header: Option<String>,
    ) -> Self {
        Self {
            state: Arc::new(ModelClientState {
                auth_manager,
                conversation_id,
                provider,
                session_source,
                model_verbosity,
                enable_responses_websockets,
                enable_responses_websockets_v2,
                enable_request_compression,
                include_timing_metrics,
                beta_features_header,
                disable_websockets: AtomicBool::new(false),
                preconnect: Mutex::new(None),
            }),
        }
    }

    pub fn new_session(&self) -> ModelClientSession {
        ModelClientSession {
            client: self.clone(),
            connection: None,
            turn_state: Arc::new(OnceLock::new()),
        }
    }

    pub fn pre_establish_connection(
        &self,
        _otel_manager: OtelManager,
        _turn_metadata_header: futures::future::BoxFuture<'static, Option<String>>,
    ) {
        // No-op for now
    }

    async fn current_client_setup(&self) -> Result<CurrentClientSetup> {
        let auth = match self.state.auth_manager.as_ref() {
            Some(manager) => manager.auth().await,
            None => None,
        };
        // Stubbed provider setup
        let api_provider = ApiProvider;
        // Stubbed auth setup
        use crate::api_bridge::auth_provider_from_auth;
        let api_auth = auth_provider_from_auth(auth.clone(), &self.state.provider)?;

        Ok(CurrentClientSetup {
            auth,
            api_provider,
            api_auth,
        })
    }

    pub async fn compact_conversation_history(
        &self,
        prompt: &Prompt,
        _model_info: &ModelInfo,
        _otel_manager: &OtelManager,
    ) -> Result<Vec<ResponseItem>> {
        if prompt.input.is_empty() {
            return Ok(Vec::new());
        }
        // Stub: return empty
        Ok(Vec::new())
    }

    pub async fn summarize_memory_traces(
        &self,
        traces: Vec<ApiMemoryTrace>,
        _model_info: &ModelInfo,
        _effort: Option<ReasoningEffortConfig>,
        _otel_manager: &OtelManager,
    ) -> Result<Vec<ApiMemoryTraceSummaryOutput>> {
        if traces.is_empty() {
            return Ok(Vec::new());
        }
        // Stub: return empty
        Ok(Vec::new())
    }
    
    // Helper to satisfy OtelManager calls in other files if they use it
    fn responses_websocket_enabled(&self) -> bool {
        false
    }

    fn disable_websockets(&self) -> bool {
        true
    }
}

impl ModelClientSession {
    #[allow(clippy::too_many_arguments)]
    pub async fn stream(
        &mut self,
        _prompt: &Prompt,
        _model_info: &ModelInfo,
        _otel_manager: &OtelManager,
        _effort: Option<ReasoningEffortConfig>,
        _summary: ReasoningSummaryConfig,
        _turn_metadata_header: Option<&str>,
    ) -> Result<ResponseStream> {
        // Stub: Return an error indicating not implemented, or an empty stream
        // Returning error is better to signal it's not ready
        Err(crate::error::RuneErr::InvalidRequest("Ollama integration not yet implemented in ModelClient".to_string()))
    }
}
