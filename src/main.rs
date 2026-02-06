mod actions;
mod data;
mod db;
mod filters;
mod templates;

use argh::FromArgs;

/// The AQN2013 cli tool
#[derive(FromArgs)]
struct CliArgs {
    /// which action to run
    #[argh(subcommand)]
    action: actions::CliAction,
}

#[tokio::main(flavor = "current_thread")]
async fn main() -> anyhow::Result<()> {
    let args: CliArgs = argh::from_env();

    match args.action {
        actions::CliAction::GenerateJson(cmd) => cmd.run().await,
        actions::CliAction::RenderHtml(cmd) => cmd.run().await,
    }
}
