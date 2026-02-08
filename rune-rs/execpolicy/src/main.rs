use anyhow::Result;
use clap::Parser;
use rune_execpolicy::execpolicycheck::ExecPolicyCheckCommand;

/// CLI for evaluating exec policies
#[derive(Parser)]
#[command(name = "rune-execpolicy")]
enum Cli {
    /// Evaluate a command against a policy.
    Check(ExecPolicyCheckCommand),
}

fn main() -> Result<()> {
    let cli = Cli::parse();
    match cli {
        Cli::Check(cmd) => cmd.run(),
    }
}
