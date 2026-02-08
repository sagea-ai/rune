# rune-core

This crate implements the business logic for Codex. It is designed to be used by the various Codex UIs written in Rust.

## Dependencies

Note that `rune-core` makes some assumptions about certain helper utilities being available in the environment. Currently, this support matrix is:

### macOS

Expects `/usr/bin/sandbox-exec` to be present.

When using the workspace-write sandbox policy, the Seatbelt profile allows
writes under the configured writable roots while keeping `.git` (directory or
pointer file), the resolved `gitdir:` target, and `.codex` read-only.

### Linux

Expects the binary containing `rune-core` to run the equivalent of `codex sandbox linux` (legacy alias: `codex debug landlock`) when `arg0` is `rune-linux-sandbox`. See the `rune-arg0` crate for details.

### All Platforms

Expects the binary containing `rune-core` to simulate the virtual `apply_patch` CLI when `arg1` is `--rune-run-as-apply-patch`. See the `rune-arg0` crate for details.
