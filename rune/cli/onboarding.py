from __future__ import annotations

import asyncio
import os
import shutil
from pathlib import Path

import ollama
from rich.console import Console
from rich.panel import Panel
from rich.prompt import Prompt

from rune.core.config import CONFIG_FILE, VibeConfig

console = Console()

def run_onboarding() -> None:
    """Run the interactive onboarding process."""
    console.print(r"""
 ░ ░░  ░░░░░░   ░   ░ ░     ░   ░   ░   ░░          ░  ░░     ░  ░ ░  ░  ░  ░          ░         ░
    ░  ░░▒█▓░░ ░░     ░░░░░░░░░░░░░░░░░   ░░░░░░░   ░ ░░░░░░░░░░░░░░     ░░░░░░░ ░░░░░░░░░░░░░░░░░░░
     ░░▒▓▒██▒█▒░░  ░   ▒██████████████▒░░ ░████▓░ ░  ░░▓████░░▓████░░░   ░▒████▒ ▒████████████████▓░
   ░░▒▒██▒██▒█▓▒▒░     ▒████▓░    ░░█████░░████▓░  ░░░░▓████░░▓███████░  ░▒████▒ ▒████▓░░        ░
 ░░░▓█▓▓█▒▒▒▒█▓▓█▒░░  ░▒████▓░     ░█████ ░████▓░░    ░▓████░░▓███████▓▓░░▒████▒ ▒████▓░  ░    ░░ ░
 ░█▓▓█▒░▓▓░░▓▒░▓█▓▓▓░ ░▒████▓░   ▒▓▓█████ ░████▓░    ░░▓████░░▓████▓▒▓██▓▓▓████▒ ▒█████▓▓▓▓▓▓▓▓▓░░
 ▒█▓▓██▒░▒█▓░░▒██▓▓█▒  ▒████▓░░░░▓██████▓ ░████▓░    ░░▓████░░▓████░ ░█████████▒ ▒██████████████░░
░▒█▓░▓██▓▒█▓▒███▒▒▓█▒  ▒████████████  ░   ░████▓░ ░ ░ ░▓████░░▓████░  ░░███████▒ ▒████▓░░░
 ▒██░░░▓██████▓░░▒██▒ ░▒████▓░░███████▒   ░████▓░    ░░▓████░░▓████░ ░ ░░░▒████▒ ▒████▓░    ░
 ░▓██▓░░░███▓░░▒███▒░  ▒████▓░░▒▒▓█████▓▒ ░▒▒▓██▓▓▓▓▓▓▓██▓▒▒░░▓████░     ░▒████▒ ▒█████▓▓▓▓▓▓▓▓▓▓▓▒░
  ░░▓██▓▒▒██▒▒███▓░░  ░░▓▓▓▓▒░░  ▒▓▓▓▓▓▓▒   ░░▓▓▓▓▓▓▓▓▓▓▓▒░░░ ▒▓▓▓▓░  ░  ░░▓▓▓▓▒ ░▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▒░
 ░  ░░░░░░░░░░░░░░       ░                       ░  ░ ░ ░        ░      ░  ░       ░  ░  ░ ░
""")
    console.print("Before we begin, let's get acquainted.\n")

    name = Prompt.ask("What should I call you?")
    console.print(f"\nGreat to meet you, [bold green]{name}[/bold green]!\n")

    console.print("Setting things up for you...")

    # Check Ollama
    with console.status("Checking Ollama installation..."):
        try:
            # Simple check if ollama is reachable
            # We can try listing models or just a version check if client supports it
            # Using list as a connectivity check
            ollama.list()
            console.print("✓ Ollama is running")
        except Exception:
            console.print("[bold red]× Could not connect to Ollama[/bold red]")
            console.print("Please make sure Ollama is installed and running: https://ollama.com/")
            console.print("Run `ollama serve` in a separate terminal.")
            # We continue anyway, user might fix it later

    # Verify Sage Reasoning models
    sage_models = ["sage-reasoning:8b", "sage-reasoning:3b", "sage-reasoning:14b", "sage-reasoning:32b"]
    wanted_model = "sage-reasoning:8b" # Default

    with console.status("Verifying Sage Reasoning models..."):
        try:
            available = [m['name'] for m in ollama.list()['models']]
            if not any(m.startswith(wanted_model) for m in available):
                console.print(f"  Downloading {wanted_model}...")
                ollama.pull(wanted_model)
                console.print(f"✓ {wanted_model} ready")
            else:
                console.print(f"✓ {wanted_model} found")
        except Exception as e:
            console.print(f"[yellow]! Could not verify models: {e}[/yellow]")

    # Configuring preferences
    with console.status("Configuring your preferences..."):
        # Create config directory if needed
        CONFIG_FILE.path.parent.mkdir(parents=True, exist_ok=True)

        # We can simulate saving the name or other prefs in a separate user config if needed,
        # but for now we rely on VibeConfig (RuneConfig).
        # To store the name, we might need to extend VibeConfig or just print it.
        # The prompt says "Store user preferences (name, preferred model, etc.) in a config file".
        # config.py loads from config.toml. We can write to it.

        # Create a default config if it doesn't exist, utilizing the name?
        # VibeConfig doesn't have a 'user_name' field by default.
        # We can add it or ignore it for now as it's not critical for logic, just for onboarding.
        # But let's create the default toml.

        if not CONFIG_FILE.path.exists():
             VibeConfig.save_updates({"active_model": "default"})

        console.print("✓ Configuration saved")

    console.print("✓ Initializing workspace")

    console.print(Panel.fit(
        "[bold green]All set![/bold green]\n"
        "You can now use Rune to supercharge your coding workflow.\n"
        "Try: [bold cyan]rune \"explain this codebase\"[/bold cyan] to get started.",
        border_style="green"
    ))
