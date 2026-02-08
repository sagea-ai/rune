use crate::agent::AgentStatus;
use crate::rune::Rune;
use crate::rune::SteerInputError;
use crate::error::Result as RuneResult;
use crate::protocol::Event;
use crate::protocol::Op;
use crate::protocol::Submission;
use rune_protocol::config_types::Personality;
use rune_protocol::openai_models::ReasoningEffort;
use rune_protocol::protocol::AskForApproval;
use rune_protocol::protocol::SandboxPolicy;
use rune_protocol::protocol::SessionSource;
use rune_protocol::user_input::UserInput;
use std::path::PathBuf;
use tokio::sync::watch;

use crate::state_db::StateDbHandle;

#[derive(Clone, Debug)]
pub struct ThreadConfigSnapshot {
    pub model: String,
    pub model_provider_id: String,
    pub approval_policy: AskForApproval,
    pub sandbox_policy: SandboxPolicy,
    pub cwd: PathBuf,
    pub reasoning_effort: Option<ReasoningEffort>,
    pub personality: Option<Personality>,
    pub session_source: SessionSource,
}

pub struct RuneThread {
    rune: Rune,
    rollout_path: Option<PathBuf>,
}

/// Conduit for the bidirectional stream of messages that compose a thread
/// (formerly called a conversation) in Rune.
impl RuneThread {
    pub(crate) fn new(rune: Rune, rollout_path: Option<PathBuf>) -> Self {
        Self {
            rune,
            rollout_path,
        }
    }

    pub async fn submit(&self, op: Op) -> RuneResult<String> {
        self.rune.submit(op).await
    }

    pub async fn steer_input(
        &self,
        input: Vec<UserInput>,
        expected_turn_id: Option<&str>,
    ) -> Result<String, SteerInputError> {
        self.rune.steer_input(input, expected_turn_id).await
    }

    /// Use sparingly: this is intended to be removed soon.
    pub async fn submit_with_id(&self, sub: Submission) -> RuneResult<()> {
        self.rune.submit_with_id(sub).await
    }

    pub async fn next_event(&self) -> RuneResult<Event> {
        self.rune.next_event().await
    }

    pub async fn agent_status(&self) -> AgentStatus {
        self.rune.agent_status().await
    }

    pub(crate) fn subscribe_status(&self) -> watch::Receiver<AgentStatus> {
        self.rune.agent_status.clone()
    }

    pub fn rollout_path(&self) -> Option<PathBuf> {
        self.rollout_path.clone()
    }

    pub fn state_db(&self) -> Option<StateDbHandle> {
        self.rune.state_db()
    }

    pub async fn config_snapshot(&self) -> ThreadConfigSnapshot {
        self.rune.thread_config_snapshot().await
    }
}
