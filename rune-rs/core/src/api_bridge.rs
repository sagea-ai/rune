use chrono::DateTime;
use chrono::Utc;
use chrono::DateTime;
use chrono::Utc;
// use rune_api::AuthProvider as ApiAuthProvider;
// use rune_api::TransportError;
use rune_protocol::ollama_types::OllamaError as ApiError;
// use rune_api::rate_limits::parse_promo_message;
// use rune_api::rate_limits::parse_rate_limit;
use http::HeaderMap;
use serde::Deserialize;

use crate::auth::RuneAuth;
use crate::error::RuneErr;
use crate::error::ModelCapError;
use crate::error::RetryLimitReachedError;
use crate::error::UnexpectedResponseError;
use crate::error::UsageLimitReachedError;
use crate::model_provider_info::ModelProviderInfo;
use crate::token_data::PlanType;

pub(crate) fn map_api_error(err: ApiError) -> RuneErr {
    match err {
        ApiError::ConnectionFailed { message, .. } => RuneErr::Stream(message, None),
        ApiError::ServerError { message, .. } => RuneErr::InvalidRequest(message),
        ApiError::BadRequest(msg) => RuneErr::InvalidRequest(msg),
        ApiError::ModelNotFound(msg) => RuneErr::InvalidRequest(format!("Model not found: {}", msg)),
        ApiError::Timeout => RuneErr::Timeout,
        ApiError::StreamDisconnected => RuneErr::Stream("Stream disconnected".to_string(), None),
        ApiError::ParseError(msg) => RuneErr::InvalidRequest(format!("Parse error: {}", msg)),
        ApiError::Other(msg) => RuneErr::InternalServerError, // simplified
    }
}

const MODEL_CAP_MODEL_HEADER: &str = "x-rune-model-cap-model";
const MODEL_CAP_RESET_AFTER_HEADER: &str = "x-rune-model-cap-reset-after-seconds";
const REQUEST_ID_HEADER: &str = "x-request-id";
const OAI_REQUEST_ID_HEADER: &str = "x-oai-request-id";
const CF_RAY_HEADER: &str = "cf-ray";

fn extract_request_tracking_id(headers: Option<&HeaderMap>) -> Option<String> {
    extract_request_id(headers).or_else(|| extract_header(headers, CF_RAY_HEADER))
}

fn extract_request_id(headers: Option<&HeaderMap>) -> Option<String> {
    extract_header(headers, REQUEST_ID_HEADER)
        .or_else(|| extract_header(headers, OAI_REQUEST_ID_HEADER))
}

fn extract_header(headers: Option<&HeaderMap>, name: &str) -> Option<String> {
    headers.and_then(|map| {
        map.get(name)
            .and_then(|value| value.to_str().ok())
            .map(str::to_string)
    })
}

pub(crate) fn auth_provider_from_auth(
    auth: Option<RuneAuth>,
    provider: &ModelProviderInfo,
) -> crate::error::Result<CoreAuthProvider> {
    if let Some(api_key) = provider.api_key()? {
        return Ok(CoreAuthProvider {
            token: Some(api_key),
            account_id: None,
        });
    }

    if let Some(token) = provider.experimental_bearer_token.clone() {
        return Ok(CoreAuthProvider {
            token: Some(token),
            account_id: None,
        });
    }

    if let Some(auth) = auth {
        let token = auth.get_token()?;
        Ok(CoreAuthProvider {
            token: Some(token),
            account_id: auth.get_account_id(),
        })
    } else {
        Ok(CoreAuthProvider {
            token: None,
            account_id: None,
        })
    }
}

#[derive(Debug, Deserialize)]
struct UsageErrorResponse {
    error: UsageErrorBody,
}

#[derive(Debug, Deserialize)]
struct UsageErrorBody {
    #[serde(rename = "type")]
    error_type: Option<String>,
    plan_type: Option<PlanType>,
    resets_at: Option<i64>,
}

#[derive(Clone, Default)]
pub(crate) struct CoreAuthProvider {
    token: Option<String>,
    account_id: Option<String>,
}

/*
impl ApiAuthProvider for CoreAuthProvider {
    fn bearer_token(&self) -> Option<String> {
        self.token.clone()
    }

    fn account_id(&self) -> Option<String> {
        self.account_id.clone()
    }
}
*/
