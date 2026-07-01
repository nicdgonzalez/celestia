// #[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
// pub struct Launcher {
//     preset: Preset,
// }

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Preset {
    /// Skip adding any optimization flags.
    None,
    /// Use flags optimized for high performance.
    #[default]
    Aikars,
    /// Flags that work best with proxy software.
    Proxy,
}

impl Preset {
    /// Collection of command-line flags associated with this preset option.
    pub fn flags(&self) -> Vec<&'static str> {
        match self {
            Self::None => vec![],
            Self::Aikars => vec![
                "-XX:+UseG1GC",
                "-XX:+ParallelRefProcEnabled",
                "-XX:MaxGCPauseMillis=200",
                "-XX:+UnlockExperimentalVMOptions",
                "-XX:+DisableExplicitGC",
                "-XX:+AlwaysPreTouch",
                "-XX:G1HeapWastePercent=5",
                "-XX:G1MixedGCCountTarget=4",
                "-XX:InitiatingHeapOccupancyPercent=15",
                "-XX:G1MixedGCLiveThresholdPercent=90",
                "-XX:G1RSetUpdatingPauseTimePercent=5",
                "-XX:SurvivorRatio=32",
                "-XX:+PerfDisableSharedMem",
                "-XX:MaxTenuringThreshold=1",
                "-Dusing.aikars.flags=https://mcflags.emc.gs",
                "-Daikars.new.flags=true",
                "-XX:G1NewSizePercent=30",
                "-XX:G1MaxNewSizePercent=40",
                "-XX:G1HeapRegionSize=8M",
                "-XX:G1ReservePercent=20",
            ],
            Self::Proxy => vec![
                "-XX:+UseG1GC",
                "-XX:G1HeapRegionSize=4M",
                "-XX:+UnlockExperimentalVMOptions",
                "-XX:+ParallelRefProcEnabled",
                "-XX:+AlwaysPreTouch",
                "-XX:MaxInlineLevel=15",
            ],
        }
    }
}
