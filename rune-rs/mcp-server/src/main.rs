use rune_arg0::arg0_dispatch_or_else;
use rune_common::CliConfigOverrides;
use rune_mcp_server::run_main;

fn main() -> anyhow::Result<()> {
    arg0_dispatch_or_else(|rune_linux_sandbox_exe| async move {
        run_main(rune_linux_sandbox_exe, CliConfigOverrides::default()).await?;
        Ok(())
    })
}
