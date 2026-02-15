from __future__ import annotations

from unittest.mock import patch

import pytest

from tests.stubs.fake_backend import FakeBackend
from tests.stubs.fake_client import FakeClient
from rune.acp.acp_agent_loop import RuneAcpAgentLoop
from rune.core.agent_loop import AgentLoop
from rune.core.types import LLMChunk, LLMMessage, LLMUsage, Role


@pytest.fixture
def backend() -> FakeBackend:
    backend = FakeBackend(
        LLMChunk(
            message=LLMMessage(role=Role.assistant, content="Hi"),
            usage=LLMUsage(prompt_tokens=1, completion_tokens=1),
        )
    )
    return backend


def _create_acp_agent() -> RuneAcpAgentLoop:
    rune_acp_agent = RuneAcpAgentLoop()
    client = FakeClient()

    rune_acp_agent.on_connect(client)
    client.on_connect(rune_acp_agent)

    return rune_acp_agent  # pyright: ignore[reportReturnType]


@pytest.fixture
def acp_agent_loop(backend: FakeBackend) -> RuneAcpAgentLoop:
    class PatchedAgent(AgentLoop):
        def __init__(self, *args, **kwargs) -> None:
            super().__init__(*args, **kwargs, backend=backend)

    patch("rune.acp.acp_agent_loop.AgentLoop", side_effect=PatchedAgent).start()
    return _create_acp_agent()
