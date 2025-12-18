mod init;
mod new;

use crate::context::Context;

pub trait Run {
    /// Execute the subcommand handler.
    async fn run(&self, ctx: &mut Context) -> anyhow::Result<()>;
}

#[derive(Debug, clap::Subcommand)]
pub enum Subcommand {
    /// Create and initialize a new Minecraft server to the Celestia database.
    New(new::New),

    /// Register an existing Minecraft server to the Celestia database.
    Init(init::Init),
}

impl Subcommand {
    pub async fn run(&self, ctx: &mut Context) -> anyhow::Result<()> {
        match *self {
            Self::New(ref handler) => handler.run(ctx).await,
            Self::Init(ref handler) => handler.run(ctx).await,
        }
    }
}
