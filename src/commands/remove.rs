use anyhow::{Context as _, bail};

use crate::commands::prelude::*;
use crate::manifest::{Manifest, Name};
use crate::notifier::Action;

/// Arguments for the `remove` subcommand.
#[derive(clap::Args)]
pub struct Remove {
    name: Name,
    additional_names: Vec<Name>,
}

impl Run for Remove {
    async fn run(&self, ctx: &mut Context) -> anyhow::Result<()> {
        let manifest_path = ctx.package().manifest();
        let mut manifest = Manifest::open(&manifest_path).context("failed to get manifest")?;

        let mut names = vec![self.name.clone()];
        names.extend(self.additional_names.clone());

        for name in &names {
            match manifest.remove_plugin(name) {
                Some((plugin_name, _)) => ctx.notify(Action::Removed, plugin_name),
                None => bail!("plugin named {name:?} not found"),
            }
        }

        manifest
            .save(&manifest_path)
            .context("failed to save manifest")?;

        Ok(())
    }
}
