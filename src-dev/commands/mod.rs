//! Application subcommands and their implementations.

mod completions;
mod new;

use std::path::PathBuf;

use clap_verbosity_flag::Verbosity;

use crate::context::Context;

/// Represents a subcommand that can be executed.
pub trait Run {
    /// Executes the subcommand.
    async fn run(&self, ctx: &mut Context) -> anyhow::Result<()>;
}

/// Command-line interface for the application.
#[derive(clap::Parser)]
pub struct Parser {
    #[clap(subcommand)]
    pub subcommand: Subcommand,

    /// Change to the specified directory prior to running the command.
    #[clap(long, short = 'C', global = true)]
    pub directory: Option<PathBuf>,

    #[clap(flatten)]
    pub verbosity: Verbosity,
}

/// Commands supported by the application.
#[derive(clap::Subcommand)]
pub enum Subcommand {
    /// Generates an autocomplete script for the specified shell.
    #[clap(hide = true)]
    Completions(completions::Completions),

    /// Creates a new Minecraft server
    New(new::New),
}

impl Run for Subcommand {
    async fn run(&self, ctx: &mut Context) -> anyhow::Result<()> {
        match *self {
            Self::Completions(ref cmd) => cmd.run(ctx).await,
            Self::New(ref cmd) => cmd.run(ctx).await,
        }
    }
}

pub(super) mod prelude {
    pub use super::Run;
    pub use crate::context::Context;
}
