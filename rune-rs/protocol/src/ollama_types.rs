// Ollama-specific types to replace rune-api dependencies
//
// This module provides minimal type definitions needed by core for Ollama integration,
// replacing the previously used rune-api crate types.

use crate::models::{RateLimitSnapshot, ResponseItem, TokenUsage};
use serde::{Deserialize, Serialize};
use std::fmt;

/// Events emitted during model response streaming from Ollama
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum ResponseEvent {
    /// Response stream started
    Created,
    
    /// New output item added to the response
    OutputItemAdded(ResponseItem),
    
    /// Output item completed
    OutputItemDone(ResponseItem),
    
    /// Text delta for streaming output
    OutputTextDelta(String),
    
    /// Response completed with token usage information
    Completed {
        response_id: String,
        token_usage: Option<TokenUsage>,
    },
    
    /// Reasoning summary delta (for models that support reasoning)
    ReasoningSummaryDelta {
        delta: String,
    },
    
    /// Reasoning content delta
    ReasoningContentDelta {
        delta: String,
    },
    
    /// Reasoning summary part added
    ReasoningSummaryPartAdded {
        part: String,
    },
    
    /// Server-provided reasoning included
    ServerReasoningIncluded(bool),
    
    /// Rate limit information (stubbed for Ollama compatibility)
    RateLimits(RateLimitSnapshot),
    
    /// Models etag (stubbed for Ollama compatibility)
    ModelsEtag(String),
}

/// Error types for Ollama operations
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum OllamaError {
    /// Network/connection error
    ConnectionFailed {
        message: String,
        status_code: Option<u16>,
    },
    
    /// Ollama server returned an error
    ServerError {
        message: String,
        status_code: Option<u16>,
    },
    
    /// Invalid request
    BadRequest(String),
    
    /// Model not found
    ModelNotFound(String),
    
    /// Timeout waiting for response  
    Timeout,
    
    /// Stream was disconnected
    StreamDisconnected,
    
    /// JSON parsing error
    ParseError(String),
    
    /// Other unspecified error
    Other(String),
}

impl fmt::Display for OllamaError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            OllamaError::ConnectionFailed { message, status_code } => {
                write!(f, "Connection failed: {}", message)?;
                if let Some(code) = status_code {
                    write!(f, " (status: {})", code)?;
                }
                Ok(())
            }
            OllamaError::ServerError { message, status_code } => {
                write!(f, "Server error: {}", message)?;
                if let Some(code) = status_code {
                    write!(f, " (status: {})", code)?;
                }
                Ok(())
            }
            OllamaError::BadRequest(msg) => write!(f, "Bad request: {}", msg),
            OllamaError::ModelNotFound(model) => write!(f, "Model not found: {}", model),
            OllamaError::Timeout => write!(f, "Request timed out"),
            OllamaError::StreamDisconnected => write!(f, "Stream disconnected"),
            OllamaError::ParseError(msg) => write!(f, "Parse error: {}", msg),
            OllamaError::Other(msg) => write!(f, "Error: {}", msg),
        }
    }
}

impl std::error::Error for OllamaError {}

/// Convert OllamaError to generic error types used elsewhere
impl From<OllamaError> for String {
    fn from(err: OllamaError) -> Self {
        err.to_string()
    }
}
