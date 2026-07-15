pub mod model;

use std::path::PathBuf;

use crate::Plugin;
use crate::manifest::model::TomlManifest;
use crate::server::Server;

pub struct Manifest {
    server: Server,
    plugins: Vec<Plugin>,
}

impl Manifest {
    /// Target Paper server details.
    #[must_use]
    pub fn server(&self) -> &Server {
        &self.server
    }

    /// Plugins to install on the server.
    #[must_use]
    pub fn plugins(&self) -> &[Plugin] {
        &self.plugins
    }
}

impl From<TomlManifest> for Manifest {
    fn from(value: TomlManifest) -> Self {
        Self {
            server: value.server.into(),
            plugins: value
                .plugins
                .into_iter()
                .map(|(name, plugin)| {
                    let file = plugin
                        .file
                        .unwrap_or_else(|| PathBuf::from(format!("{}.jar", name.as_str())));

                    Plugin::new(name, plugin.source.into(), file)
                })
                .collect(),
        }
    }
}
