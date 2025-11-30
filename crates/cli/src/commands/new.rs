use anyhow::bail;

use crate::commands::Run;
use crate::context::Context;

#[derive(Debug, clap::Args)]
pub struct New;

impl Run for New {
    async fn run(&self, _ctx: &mut Context) -> anyhow::Result<()> {
        bail!("not implemented yet")
    }
}
