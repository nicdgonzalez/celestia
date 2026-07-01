use crate::paper::{Build, Version};

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
pub struct Server {
    pub version: Version,
    pub build: Build,
}
