use crate::commands::prelude::*;

/// Arguments for the `restore` subcommand.
#[derive(clap::Args)]
pub struct Restore;

impl Run for Restore {
    async fn run(&self, _ctx: &mut Context) -> anyhow::Result<()> {
        todo!()
    }
}
