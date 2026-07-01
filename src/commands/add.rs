use std::path::PathBuf;

use anyhow::Context as _;

use crate::commands::prelude::*;
use crate::manifest::{Manifest, Name, Plugin, Source};
use crate::notifier::Action;
use crate::plugins::{PluginRepositoryFactory, Registry};

/// Arguments for the `add` subcommand.
#[derive(clap::Args)]
pub struct Add {
    name: Name,
    additional_names: Vec<Name>,

    #[clap(long, conflicts_with = "additional_names")]
    version: Option<String>,

    #[clap(long, default_value_t, conflicts_with_all = ["url", "path"])]
    registry: Registry,

    #[clap(long, conflicts_with = "additional_names")]
    url: Option<String>,

    #[clap(long, conflicts_with = "additional_names")]
    path: Option<PathBuf>,
}

impl Run for Add {
    async fn run(&self, ctx: &mut Context) -> anyhow::Result<()> {
        let manifest_path = ctx.package().manifest();
        let mut manifest = Manifest::open(&manifest_path).context("failed to get manifest")?;

        if let Some(url) = &self.url {
            manifest.add_plugin(
                self.name.clone(),
                Plugin {
                    source: Source::Url { url: url.clone() },
                    file: None,
                },
            );
        } else if let Some(path) = &self.path {
            manifest.add_plugin(
                self.name.clone(),
                Plugin {
                    source: Source::Local { path: path.clone() },
                    file: None,
                },
            );
        } else {
            let repository = PluginRepositoryFactory::create(self.registry);

            let mut names = vec![self.name.clone()];
            names.extend(self.additional_names.clone());

            for name in &names {
                // TODO: Default to latest stable build and add an `allow_experimental` flag.
                let results = repository
                    .search(name.as_str())
                    .await
                    .context("failed to search for plugin")?;

                let plugin = results
                    .iter()
                    .find(|result| name.as_str().eq_ignore_ascii_case(&result.name))
                    .with_context(|| format!("plugin not found: {name}"))?;

                ctx.notify(Action::Adding, format!("{} to plugins", plugin.name));

                let artifact = repository
                    .resolve(&plugin.id, self.version.as_deref())
                    .await
                    .context("failed to resolve plugin")?;

                manifest.add_plugin(
                    plugin.name.clone().into(),
                    Plugin {
                        source: Source::Registry {
                            registry: self.registry,
                            id: plugin.id.clone(),
                            version: artifact.version,
                        },
                        file: None,
                    },
                );
            }
        }

        manifest
            .save(&manifest_path)
            .context("failed to save manifest")?;

        ctx.notify(Action::Finished, "Run `build` to apply changes");
        Ok(())
    }
}
