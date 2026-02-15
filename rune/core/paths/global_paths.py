from __future__ import annotations

from collections.abc import Callable
import os
from pathlib import Path

from rune import RUNE_ROOT


class GlobalPath:
    def __init__(self, resolver: Callable[[], Path]) -> None:
        self._resolver = resolver

    @property
    def path(self) -> Path:
        return self._resolver()


_DEFAULT_RUNE_HOME = Path.home() / ".rune"


def _get_rune_home() -> Path:
    if rune_home := os.getenv("RUNE_HOME"):
        return Path(rune_home).expanduser().resolve()
    return _DEFAULT_RUNE_HOME


RUNE_HOME = GlobalPath(_get_rune_home)
GLOBAL_CONFIG_FILE = GlobalPath(lambda: RUNE_HOME.path / "config.toml")
GLOBAL_ENV_FILE = GlobalPath(lambda: RUNE_HOME.path / ".env")
GLOBAL_TOOLS_DIR = GlobalPath(lambda: RUNE_HOME.path / "tools")
GLOBAL_SKILLS_DIR = GlobalPath(lambda: RUNE_HOME.path / "skills")
GLOBAL_AGENTS_DIR = GlobalPath(lambda: RUNE_HOME.path / "agents")
GLOBAL_PROMPTS_DIR = GlobalPath(lambda: RUNE_HOME.path / "prompts")
SESSION_LOG_DIR = GlobalPath(lambda: RUNE_HOME.path / "logs" / "session")
TRUSTED_FOLDERS_FILE = GlobalPath(lambda: RUNE_HOME.path / "trusted_folders.toml")
LOG_DIR = GlobalPath(lambda: RUNE_HOME.path / "logs")
LOG_FILE = GlobalPath(lambda: RUNE_HOME.path / "rune.log")

DEFAULT_TOOL_DIR = GlobalPath(lambda: RUNE_ROOT / "core" / "tools" / "builtins")
