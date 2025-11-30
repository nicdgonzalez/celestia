mod new;

use crate::context::Context;

pub trait Run {
    async fn run(&self, ctx: &mut Context) -> anyhow::Result<()>;
}

#[derive(Debug, clap::Subcommand)]
pub enum Subcommand {
    /// Create a new Minecraft server.
    New(new::New),
}

impl Subcommand {
    pub async fn run(&self, ctx: &mut Context) -> anyhow::Result<()> {
        match *self {
            Self::New(ref handler) => handler.run(ctx).await,
        }
    }
}
