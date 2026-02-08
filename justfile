set working-directory := "rune-rs"
set positional-arguments

# Display help
help:
    just -l

# `rune`
alias c := rune
rune *args:
    cargo run --bin rune -- "$@"

# `rune exec`
exec *args:
    cargo run --bin rune -- exec "$@"

# Run the CLI version of the file-search crate.
file-search *args:
    cargo run --bin rune-file-search -- "$@"

# Build the CLI and run the app-server test client
app-server-test-client *args:
    cargo build -p rune-cli
    cargo run -p rune-app-server-test-client -- --rune-bin ./target/debug/rune "$@"

# format code
fmt:
    cargo fmt -- --config imports_granularity=Item 2>/dev/null

fix *args:
    cargo clippy --fix --all-features --tests --allow-dirty "$@"

clippy:
    cargo clippy --all-features --tests "$@"

install:
    rustup show active-toolchain
    cargo fetch

# Run `cargo nextest` since it's faster than `cargo test`, though including
# --no-fail-fast is important to ensure all tests are run.
#
# Run `cargo install cargo-nextest` if you don't have it installed.
test:
    cargo nextest run --no-fail-fast

# Build and run Rune from source using Bazel.
# Note we have to use the combination of `[no-cd]` and `--run_under="cd $PWD &&"`
# to ensure that Bazel runs the command in the current working directory.
[no-cd]
bazel-rune *args:
    bazel run //rune-rs/cli:rune --run_under="cd $PWD &&" -- "$@"

bazel-test:
    bazel test //... --keep_going

bazel-remote-test:
    bazel test //... --config=remote --platforms=//:rbe --keep_going

build-for-release:
    bazel build //rune-rs/cli:release_binaries --config=remote

# Run the MCP server
mcp-server-run *args:
    cargo run -p rune-mcp-server -- "$@"

# Regenerate the json schema for rune.toml from the current config types.
write-config-schema:
    cargo run -p rune-core --bin rune-write-config-schema

# Regenerate vendored app-server protocol schema artifacts.
write-app-server-schema *args:
    cargo run -p rune-app-server-protocol --bin write_schema_fixtures -- "$@"

# Tail logs from the state SQLite database
log *args:
    if [ "${1:-}" = "--" ]; then shift; fi; cargo run -p rune-state --bin logs_client -- "$@"

# Model Selection - List available sage-reasoning models
models:
    @echo "Available sage-reasoning models:"
    @echo "  3b  - comethrusws/sage-reasoning:3b (default, fastest)"
    @echo "  8b  - comethrusws/sage-reasoning:8b (balanced)"
    @echo "  14b - comethrusws/sage-reasoning:14b (most capable)"
    @echo ""
    @echo "Usage:"
    @echo "  rune --model comethrusws/sage-reasoning:3b"
    @echo "  rune --model comethrusws/sage-reasoning:8b"
    @echo "  rune --model comethrusws/sage-reasoning:14b"
    @echo ""
    @echo "Quick launch commands:"
    @echo "  just rune-3b  - Run with 3b model"
    @echo "  just rune-8b  - Run with 8b model"
    @echo "  just rune-14b - Run with 14b model"

# Run Rune with sage-reasoning:3b (default)
rune-3b *args:
    cargo run --bin rune -- --model comethrusws/sage-reasoning:3b "$@"

# Run Rune with sage-reasoning:8b
rune-8b *args:
    cargo run --bin rune -- --model comethrusws/sage-reasoning:8b "$@"

# Run Rune with sage-reasoning:14b
rune-14b *args:
    cargo run --bin rune -- --model comethrusws/sage-reasoning:14b "$@"
