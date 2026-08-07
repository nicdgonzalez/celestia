mod add;

pub trait Run {
    /// Executes the subcommand.
    async fn run(self) -> anyhow::Result<()>;
}

/// Represents the application's command-line interface.
#[derive(clap::Parser)]
#[clap(about = "Manager for Paper Minecraft servers.")]
pub struct Cli {
    #[clap(subcommand)]
    pub subcommand: Subcommand,
}

#[derive(clap::Subcommand)]
pub enum Subcommand {
    /// Adds a plugin.
    Add(add::Add),
}

/// Executes the subcommand.
pub async fn run(args: Cli) -> anyhow::Result<()> {
    match args.subcommand {
        Subcommand::Add(cmd) => cmd.run().await,
    }
}
