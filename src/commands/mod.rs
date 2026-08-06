/// Represents the application's command-line interface.
#[derive(clap::Parser)]
#[clap(about = "Manager for Paper Minecraft servers.")]
pub struct Cli {
    /// Name of plugin to install.
    pub name: String,
}
