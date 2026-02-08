# Rune RS

Rune is a powerful agentic AI coding assistant designed by [SAGEA](https://sagea.space) to run locally with open models.

## Code Organization

This folder is the root of a Cargo workspace. It contains the key crates for Rune:

- [`core/`](./core) contains the business logic for Rune.
- [`exec/`](./exec) "headless" CLI for use in automation.
- [`tui/`](./tui) CLI that launches a fullscreen TUI built with [Ratatui](https://ratatui.rs/).
- [`cli/`](./cli) CLI multitool that provides the aforementioned CLIs via subcommands.

## Configuration

Rune uses `~/.rune/rune.toml` for configuration. See `docs/config.md` for details.
