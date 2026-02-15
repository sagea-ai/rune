from __future__ import annotations

from rune.core.config import Backend
from rune.core.llm.backend.generic import GenericBackend
from rune.core.llm.backend.ollama import OllamaBackend

BACKEND_FACTORY = {Backend.OLLAMA: OllamaBackend, Backend.GENERIC: GenericBackend}
