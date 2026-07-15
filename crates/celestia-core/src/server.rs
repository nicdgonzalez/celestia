use crate::manifest::model::TomlServer;

// TODO: Replace with Paper version.
type Version = String;
// TODO: Replace with Paper build.
type Build = u32;

/// Target Paper server to install.
pub struct Server {
    version: Version,
    build: Build,
}

impl Server {
    /// Construct a new target Paper server.
    #[must_use]
    pub const fn new(version: Version, build: Build) -> Self {
        Self { version, build }
    }

    /// Target Paper server version.
    #[must_use]
    pub fn version(&self) -> &Version {
        &self.version
    }

    /// Target Paper server JAR build.
    #[must_use]
    pub fn build(&self) -> Build {
        self.build
    }
}

impl From<TomlServer> for Server {
    fn from(value: TomlServer) -> Self {
        Self {
            version: value.version.into(),
            build: value.build.into(),
        }
    }
}
