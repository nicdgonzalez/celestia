use std::path::PathBuf;
use std::{env, fs};

use anyhow::Context as _;
use colored::Colorize as _;

use crate::commands::Run;
use crate::modrinth::{ListVersionsRequest, ModrinthClient, SearchRequest};

#[derive(clap::Args)]
pub struct Add {
    // Some flags only work when there is one plugin listed; we split our arguments here so
    // clap can validate our invariants before it reaches our business logic.
    name: String,
    additional_names: Vec<String>,

    /// Include non-stable releases.
    #[clap(long)]
    allow_experimental: bool,

    /// Run without committing changes.
    #[clap(long)]
    dry_run: bool,
}

impl Run for Add {
    async fn run(mut self) -> anyhow::Result<()> {
        let modrinth = ModrinthClient::new();

        let mut plugins = env::current_dir().unwrap_or(PathBuf::from("/"));
        plugins.push("plugins");

        if !self.dry_run {
            fs::create_dir_all(&plugins).context("failed to create plugins directory")?;
        }

        // Insert at the front so it's in the same order as how the user provided them.
        self.additional_names.insert(0, self.name);

        for name in self.additional_names {
            let search_request = SearchRequest { name: &name };
            let plugin = modrinth
                .search_projects(search_request)
                .await
                .context("failed to search for projects")?
                .find(|project| name.eq_ignore_ascii_case(&project.name))
                .context("plugin not found")?;

            let list_request = ListVersionsRequest {
                id: &plugin.id,
                // TODO: Read this version dynamically from whichever server we are currently in.
                version: "26.2",
                allow_experimental: self.allow_experimental,
            };
            let version = modrinth
                .list_versions(list_request)
                .await
                .context("failed to list versions")?
                .find(|v| v.file.is_some())
                .context("version not found")?;

            let file = version.file.unwrap();

            println!(
                "{label} {name} ({version})",
                label = "Downloading".bold().green(),
                name = plugin.name,
                version = version.id
            );

            let bytes = modrinth
                .download(&file)
                .await
                .context("failed to download plugin")?;

            let output = plugins.join(&file.file_name);

            if !self.dry_run {
                fs::write(&output, bytes).context("failed to save plugin")?;
            }
        }

        Ok(())
    }
}
