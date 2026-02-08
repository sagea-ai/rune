use clap::Parser;
use rune_responses_api_proxy::Args as ResponsesApiProxyArgs;

#[ctor::ctor]
fn pre_main() {
    rune_process_hardening::pre_main_hardening();
}

pub fn main() -> anyhow::Result<()> {
    let args = ResponsesApiProxyArgs::parse();
    rune_responses_api_proxy::run_main(args)
}
