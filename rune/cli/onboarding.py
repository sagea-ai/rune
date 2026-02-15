from __future__ import annotations

import asyncio
import os
import shutil
import subprocess
import sys
from pathlib import Path

import ollama
from rich.console import Console
from rich.panel import Panel
from rich.prompt import Prompt

from rune.core.config import CONFIG_FILE, VibeConfig

console = Console()

def install_ollama() -> bool:
    """Attempt to install Ollama automatically."""
    console.print("[yellow]Ollama not found. Attempting to install...[/yellow]")

    if sys.platform != "linux":
        console.print("[red]Automatic installation is only supported on Linux.[/red]")
        console.print("Please install Ollama manually from https://ollama.com/")
        return False

    try:
        # Use simple curl | sh installation
        install_cmd = "curl -fsSL https://ollama.com/install.sh | sh"

        # We need to run this interactively to allow sudo prompt if needed
        # But subprocess.run might not utilize the TTY properly for sudo password if not careful.
        # However, for a CLI tool, inheriting stdin/stdout usually works.

        subprocess.run(install_cmd, shell=True, check=True, executable="/bin/sh")
        console.print("[green]Ollama installed successfully![/green]")
        return True
    except subprocess.CalledProcessError:
        console.print("[red]Installation failed.[/red]")
        console.print("Please install Ollama manually from https://ollama.com/")
        return False
    except Exception as e:
        console.print(f"[red]An error occurred: {e}[/red]")
        return False

def run_onboarding() -> None:
    """Run the interactive onboarding process."""
    console.print(r"""
[#93c5fd] ░ ░░  ░░░░░░   ░   ░ ░     ░   ░   ░   ░░          ░  ░░     ░  ░ ░  ░  ░  ░          ░         ░[/#93c5fd]
[#93c5fd]    ░  ░░▒█▓░░ ░░     ░░░░░░░░░░░░░░░░░   ░░░░░░░   ░ ░░░░░░░░░░░░░░     ░░░░░░░ ░░░░░░░░░░░░░░░░░░░[/#93c5fd]
[#60a5fa]     ░░▒▓▒██▒█▒░░  ░   ▒██████████████▒░░ ░████▓░ ░  ░░▓████░░▓████░░░   ░▒████▒ ▒████████████████▓░[/#60a5fa]
[#60a5fa]   ░░▒▒██▒██▒█▓▒▒░     ▒████▓░    ░░█████░░████▓░  ░░░░▓████░░▓███████░  ░▒████▒ ▒████▓░░        ░[/#60a5fa]
[#3b82f6] ░░░▓█▓▓█▒▒▒▒█▓▓█▒░░  ░▒████▓░     ░█████ ░████▓░░    ░▓████░░▓███████▓▓░░▒████▒ ▒████▓░  ░    ░░ ░[/#3b82f6]
[#3b82f6] ░█▓▓█▒░▓▓░░▓▒░▓█▓▓▓░ ░▒████▓░   ▒▓▓█████ ░████▓░    ░░▓████░░▓████▓▒▓██▓▓▓████▒ ▒█████▓▓▓▓▓▓▓▓▓░░[/#3b82f6]
[#2563eb] ░▒▓▓██▒░▒█▓░░▒██▓▓█▒  ▒████▓░░░░▓██████▓ ░████▓░    ░░▓████░░▓████░ ░█████████▒ ▒██████████████░░[/#2563eb]
[#2563eb]░▒█▓░▓██▓▒█▓▒███▒▒▓█▒  ▒████████████  ░   ░████▓░ ░ ░ ░▓████░░▓████░  ░░███████▒ ▒████▓░░░[/#2563eb]
[#1d4ed8] ▒██░░░▓██████▓░░▒██▒ ░▒████▓░░███████▒   ░████▓░    ░░▓████░░▓████░ ░ ░░░▒████▒ ▒████▓░    ░[/#1d4ed8]
[#1d4ed8] ░▓██▓░░░███▓░░▒███▒░  ▒████▓░░▒▒▓█████▓▒ ░▒▒▓██▓▓▓▓▓▓▓██▓▒▒░░▓████░     ░▒████▒ ▒█████▓▓▓▓▓▓▓▓▓▓▓▒░[/#1d4ed8]
[#1e40af]  ░░▓██▓▒▒██▒▒███▓░░  ░░▓▓▓▓▒░░  ▒▓▓▓▓▓▓▒   ░░▓▓▓▓▓▓▓▓▓▓▓▒░░░ ▒▓▓▓▓░  ░  ░░▓▓▓▓▒ ░▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▒░[/#1e40af]
[#1e40af] ░  ░░░░░░░░░░░░░░       ░                       ░  ░ ░ ░        ░      ░  ░       ░  ░  ░ ░[/#1e40af]
""")
    console.print("Before we begin, let's get acquainted.\n")

    name = Prompt.ask("What should I call you?")
    console.print(f"\nGreat to meet you, [bold green]{name}[/bold green]!\n")

    console.print("Setting things up for you...")

    # Check Ollama
    ollama_ready = False
    with console.status("Checking Ollama installation..."):
        try:
            ollama.list()
            ollama_ready = True
            console.print("✓ Ollama is running")
        except Exception:
            pass

    if not ollama_ready:
        console.print("[bold red]× Could not connect to Ollama[/bold red]")
        if Prompt.ask("Ollama is not running or not installed. Do you want me to install ensure it is running/install it?", choices=["y", "n"], default="y") == "y":
             if install_ollama():
                 ollama_ready = True
                 # Try starting the server in background if not running?
                 # The install script usually starts the service on Linux (systemd).
                 console.print("Waiting for Ollama to start...")
                 import time
                 time.sleep(5)
                 try:
                     ollama.list()
                     console.print("✓ Ollama is now running")
                 except Exception:
                     console.print("[yellow]Ollama installed but might need a manual start or restart of this terminal.[/yellow]")
                     console.print("Run `ollama serve` in a separate terminal if it's not running.")

    # Verify Sage Reasoning models
    wanted_model = "sage-reasoning:8b" # Default

    if ollama_ready:
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
    else:
        console.print("[yellow]Skipping model verification as Ollama is not ready.[/yellow]")

    # Configuring preferences
    with console.status("Configuring your preferences..."):
        # Create config directory if needed
        CONFIG_FILE.path.parent.mkdir(parents=True, exist_ok=True)

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
