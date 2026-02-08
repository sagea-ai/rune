<p align="center"><code>npm i -g @openai/rune</code><br />or <code>brew install --cask rune</code></p>
<p align="center"><strong>Rune CLI</strong> is a coding agent from OpenAI that runs locally on your computer.
<p align="center">
  <img src="https://github.com/openai/rune/blob/main/.github/rune-cli-splash.png" alt="Rune CLI splash" width="80%" />
</p>
</br>
If you want Rune in your code editor (VS Code, Cursor, Windsurf), <a href="https://developers.openai.com/rune/ide">install in your IDE.</a>
</br>If you are looking for the <em>cloud-based agent</em> from OpenAI, <strong>Rune Web</strong>, go to <a href="https://chatgpt.com/rune">chatgpt.com/rune</a>.</p>

# Rune

Rune is a powerful agentic AI coding assistant designed by [SAGEA](https://sagea.space) to run locally with open models.

It is built to integrate seamlessly with Ollama and provide a robust CLI experience for developers.

## Features

- **Local First**: Powered by Ollama and local models.
- **Agentic Capabilities**: Performs complex coding tasks autonomously.
- **SAGEA Branding**: Designed with a focus on aesthetics and usability.

## Getting Started

### Prerequisites

- [Ollama](https://ollama.com/) installed and running.
- Pull the recommended models:
  ```bash
  ollama pull comethrusws/sage-reasoning:3b
  ```

### Installation

```bash
cargo install --path rune-rs/cli
```

### Usage

```bash
rune --help
```
 to get started.

<details>
<summary>You can also go to the <a href="https://github.com/openai/rune/releases/latest">latest GitHub Release</a> and download the appropriate binary for your platform.</summary>

Each GitHub Release contains many executables, but in practice, you likely want one of these:

- macOS
  - Apple Silicon/arm64: `rune-aarch64-apple-darwin.tar.gz`
  - x86_64 (older Mac hardware): `rune-x86_64-apple-darwin.tar.gz`
- Linux
  - x86_64: `rune-x86_64-unknown-linux-musl.tar.gz`
  - arm64: `rune-aarch64-unknown-linux-musl.tar.gz`

Each archive contains a single entry with the platform baked into the name (e.g., `rune-x86_64-unknown-linux-musl`), so you likely want to rename it to `rune` after extracting it.

</details>


## Docs

- [**Rune Documentation**](https://developers.openai.com/rune)
- [**Contributing**](./docs/contributing.md)
- [**Installing & building**](./docs/install.md)
- [**Open source fund**](./docs/open-source-fund.md)

This repository is licensed under the [Apache-2.0 License](LICENSE).
