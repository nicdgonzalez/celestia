use std::collections::HashMap;

use super::plugin::{Name, Plugin};
use super::server::Server;

/// Defines the manifest schema.
#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
pub struct Model {
    pub server: Server,

    #[serde(default)]
    pub plugins: HashMap<Name, Plugin>,
}
