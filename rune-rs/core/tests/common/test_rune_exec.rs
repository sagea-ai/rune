#![allow(clippy::expect_used)]
use rune_core::auth::RUNE_API_KEY_ENV_VAR;
use std::path::Path;
use tempfile::TempDir;
use wiremock::MockServer;

pub struct TestRuneExecBuilder {
    home: TempDir,
    cwd: TempDir,
}

impl TestRuneExecBuilder {
    pub fn cmd(&self) -> assert_cmd::Command {
        let mut cmd = assert_cmd::Command::new(
            rune_utils_cargo_bin::cargo_bin("rune-exec")
                .expect("should find binary for rune-exec"),
        );
        cmd.current_dir(self.cwd.path())
            .env("RUNE_HOME", self.home.path())
            .env(RUNE_API_KEY_ENV_VAR, "dummy");
        cmd
    }
    pub fn cmd_with_server(&self, server: &MockServer) -> assert_cmd::Command {
        let mut cmd = self.cmd();
        let base = format!("{}/v1", server.uri());
        cmd.env("OPENAI_BASE_URL", base);
        cmd
    }

    pub fn cwd_path(&self) -> &Path {
        self.cwd.path()
    }
    pub fn home_path(&self) -> &Path {
        self.home.path()
    }
}

pub fn test_rune_exec() -> TestRuneExecBuilder {
    TestRuneExecBuilder {
        home: TempDir::new().expect("create temp home"),
        cwd: TempDir::new().expect("create temp cwd"),
    }
}
