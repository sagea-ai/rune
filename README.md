# Rune 🔮

**Rune is a powerful CLI coding agent by SAGEA, powered by Ollama and Sage Reasoning models.**

Iterate on your codebase using natural language. Rune understands your project context, executes commands, edits files, and helps you build software faster.

```
 ░ ░░  ░░░░░░   ░   ░ ░     ░   ░   ░   ░░          ░  ░░     ░  ░ ░  ░  ░  ░          ░         ░
    ░  ░░▒█▓░░ ░░     ░░░░░░░░░░░░░░░░░   ░░░░░░░   ░ ░░░░░░░░░░░░░░     ░░░░░░░ ░░░░░░░░░░░░░░░░░░░
     ░░▒▓▒██▒█▒░░  ░   ▒██████████████▒░░ ░████▓░ ░  ░░▓████░░▓████░░░   ░▒████▒ ▒████████████████▓░
   ░░▒▒██▒██▒█▓▒▒░     ▒████▓░    ░░█████░░████▓░  ░░░░▓████░░▓███████░  ░▒████▒ ▒████▓░░        ░
 ░░░▓█▓▓█▒▒▒▒█▓▓█▒░░  ░▒████▓░     ░█████ ░████▓░░    ░▓████░░▓███████▓▓░░▒████▒ ▒████▓░  ░    ░░ ░
 ░█▓▓█▒░▓▓░░▓▒░▓█▓▓▓░ ░▒████▓░   ▒▓▓█████ ░████▓░    ░░▓████░░▓████▓▒▓██▓▓▓████▒ ▒█████▓▓▓▓▓▓▓▓▓░░
 ░▒▓▓██▒░▒█▓░░▒██▓▓█▒  ▒████▓░░░░▓██████▓ ░████▓░    ░░▓████░░▓████░ ░█████████▒ ▒██████████████░░
░▒█▓░▓██▓▒█▓▒███▒▒▓█▒  ▒████████████  ░   ░████▓░ ░ ░ ░▓████░░▓████░  ░░███████▒ ▒████▓░░░
 ▒██░░░▓██████▓░░▒██▒ ░▒████▓░░███████▒   ░████▓░    ░░▓████░░▓████░ ░ ░░░▒████▒ ▒████▓░    ░
 ░▓██▓░░░███▓░░▒███▒░  ▒████▓░░▒▒▓█████▓▒ ░▒▒▓██▓▓▓▓▓▓▓██▓▒▒░░▓████░     ░▒████▒ ▒█████▓▓▓▓▓▓▓▓▓▓▓▒░
  ░░▓██▓▒▒██▒▒███▓░░  ░░▓▓▓▓▒░░  ▒▓▓▓▓▓▓▒   ░░▓▓▓▓▓▓▓▓▓▓▓▒░░░ ▒▓▓▓▓░  ░  ░░▓▓▓▓▒ ░▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▒░
 ░  ░░░░░░░░░░░░░░       ░                       ░  ░ ░ ░        ░      ░  ░       ░  ░  ░ ░
```

## Features

- **Local & Private**: Runs entirely on your machine using [Ollama](https://ollama.com).
- **Sage Reasoning**: Defaults to `sage-reasoning` models (3b, 8b, 14b) for intelligent coding assistance.
- **Project-Aware**: Automatically scans your project structure and git status.
- **Powerful Tools**:
  - Read/Write files
  - Run shell commands
  - Grep search
  - Manage Todos
- **Personalized Onboarding**: Sets up your environment and preferences automatically.

## Prerequisites

1. **Python 3.12+**
2. **Ollama**: Must be installed and running (`ollama serve`).
   - Download from [ollama.com](https://ollama.com).

## Installation

Clone the repository and install locally:

```bash
git clone https://github.com/sagea-ai/rune.git
cd rune
pip install -e .
```

## Quick Start

1. Ensure Ollama is running:
   ```bash
   ollama serve
   ```

2. Run Rune:
   ```bash
   rune
   ```

3. **First Run**: Rune will guide you through a personalized onboarding process to set up your profile and download necessary models (`sage-reasoning:8b` by default).

4. **Start Coding**:
   ```
   > Create a snake game in python using pygame
   ```

## Usage

### Interactive Mode

Simply run `rune` to enter the interactive chat.

- **`@filename`**: Reference a file (e.g., `> Explain @main.py`)
- **`!command`**: Run a shell command directly (e.g., `> !ls -la`)
- **`Ctrl+J`**: Insert newline
- **`/help`**: View available slash commands

### Configuration

Rune uses a TOML configuration file located at `~/.rune/config.toml`.

```toml
[rune]
active_model = "default"  # Maps to sage-reasoning:8b

[[models]]
name = "sage-reasoning:8b"
provider = "ollama"
alias = "default"
```

## License

Apache 2.0. See [LICENSE](LICENSE) for details.

## Attribution

Rune is a fork of [Mistral Vibe](https://github.com/mistralai/mistral-vibe), originally developed by Mistral AI, and now maintained by SAGEA.
