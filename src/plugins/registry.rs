use std::fmt;

#[derive(Debug, Clone, Copy, Default, serde::Serialize, serde::Deserialize, clap::ValueEnum)]
#[serde(rename_all = "lowercase")]
#[clap(rename_all = "lowercase")]
pub enum Registry {
    #[default]
    Modrinth,
    Hangar,
}

impl Registry {
    pub fn id(&self) -> &str {
        match *self {
            Self::Modrinth => "modrinth",
            Self::Hangar => "hangar",
        }
    }

    #[expect(dead_code)]
    pub fn display_name(&self) -> &str {
        match *self {
            Self::Modrinth => "Modrinth",
            Self::Hangar => "Hangar",
        }
    }
}

impl fmt::Display for Registry {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.id().fmt(f)
    }
}
