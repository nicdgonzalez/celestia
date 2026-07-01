//! Application subcommands and their implementations.

use std::path::PathBuf;

use clap_verbosity_flag::Verbosity;

use crate::context::Context;

mod add;
mod backup;
mod build;
mod completions;
mod new;
mod remove;
mod restore;
mod start;
mod status;
mod stop;
mod update;

/// A subcommand that can be executed.
pub trait Run {
    /// Executes the subcommand.
    async fn run(&self, ctx: &mut Context) -> anyhow::Result<()>;
}

/// Command-line interface for the application.
#[derive(clap::Parser)]
#[clap(about = "Server manager for Minecraft")]
pub struct Parser {
    #[clap(subcommand)]
    pub subcommand: Subcommand,

    /// Change to the specified directory prior to running the command
    #[clap(long, short = 'C', global = true)]
    pub directory: Option<PathBuf>,

    #[clap(flatten)]
    pub verbosity: Verbosity,
}

/// Commands supported by the application.
#[derive(clap::Subcommand)]
pub enum Subcommand {
    /// Generates an autocomplete script for the specified shell
    #[clap(hide = true)]
    Completions(completions::Completions),
    /// Creates a new Minecraft server
    New(new::New),
    /// Updates the server to a different version of Minecraft
    Update(update::Update),
    /// Applies changes to the server
    Build(build::Build),
    /// Opens the server, allowing players to connect to the world
    Start(start::Start),
    /// Closes the server, disconnecting all players
    Stop(stop::Stop),
    /// Gets basic information about an online server, such as MOTD, player count, etc.
    Status(status::Status),
    /// Adds a plugin to the server
    Add(add::Add),
    /// Removes a plugin from the server
    Remove(remove::Remove),
    /// Creates a new backup of the server
    Backup(backup::Backup),
    /// Overwrites the current server with the latest backup
    Restore(restore::Restore),
}

impl Run for Subcommand {
    async fn run(&self, ctx: &mut Context) -> anyhow::Result<()> {
        match *self {
            Self::Completions(ref cmd) => cmd.run(ctx).await,
            Self::New(ref cmd) => cmd.run(ctx).await,
            Self::Update(ref cmd) => cmd.run(ctx).await,
            Self::Build(ref cmd) => cmd.run(ctx).await,
            Self::Start(ref cmd) => cmd.run(ctx).await,
            Self::Stop(ref cmd) => cmd.run(ctx).await,
            Self::Status(ref cmd) => cmd.run(ctx).await,
            Self::Add(ref cmd) => cmd.run(ctx).await,
            Self::Remove(ref cmd) => cmd.run(ctx).await,
            Self::Backup(ref cmd) => cmd.run(ctx).await,
            Self::Restore(ref cmd) => cmd.run(ctx).await,
        }
    }
}

pub mod prelude {
    pub use super::Run;
    pub use crate::context::Context;
}
