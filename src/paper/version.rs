use std::error::Error;
use std::fmt;
use std::str::FromStr;
use std::sync::LazyLock;

use regex::Regex;

static VERSION_RE: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"^(?P<major>\d+)\.(?P<minor>\d+)(?:\.(?P<patch>\d+))?(?:-(?P<release_kind>pre|rc)-?(?P<release_number>\d+))?$").unwrap()
});

/// Minecraft version supported by Paper.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Version {
    major: u32,
    minor: u32,
    patch: u32,
    release: Option<Release>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Release {
    PreRelease(u32),
    ReleaseCandidate(u32),
}

impl Default for Version {
    fn default() -> Self {
        Self::from_str("26.2").unwrap() // This can be any valid Paper Minecraft version.
    }
}

impl FromStr for Version {
    type Err = InvalidVersionError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let captures = VERSION_RE.captures(s).ok_or(InvalidVersionError)?;

        let major = captures
            .name("major")
            .map(|m| m.as_str().parse().unwrap())
            .ok_or(InvalidVersionError)?;
        let minor = captures
            .name("minor")
            .map(|m| m.as_str().parse().unwrap())
            .ok_or(InvalidVersionError)?;
        let patch = captures
            .name("patch")
            .map_or(0, |m| m.as_str().parse().unwrap());

        let release = match (
            captures.name("release_kind"),
            captures.name("release_number"),
        ) {
            (Some(kind), Some(number)) => {
                let number = number.as_str().parse().unwrap();
                let kind = match kind.as_str() {
                    "pre" => Release::PreRelease(number),
                    "rc" => Release::ReleaseCandidate(number),
                    other => unreachable!("unexpected release type: {other}"),
                };

                Some(kind)
            }
            (None, None) => None,
            _ => return Err(InvalidVersionError),
        };

        Ok(Self {
            major,
            minor,
            patch,
            release,
        })
    }
}

impl fmt::Display for Version {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut buffer = format!("{}.{}", self.major, self.minor);

        if self.patch > 0 {
            buffer.push('.');
            buffer.push_str(self.patch.to_string().as_str());
        }

        if let Some(release) = self.release {
            buffer.push('-');

            let number = match release {
                Release::PreRelease(number) => {
                    buffer.push_str("pre");
                    number
                }
                Release::ReleaseCandidate(number) => {
                    buffer.push_str("rc");

                    // I'm not sure if Minecraft or Paper changed it, but the release candidate
                    // for 26.2 had a dash between the release type and number (e.g., `26.2-rc-2`),
                    // while previous versions kept them together (e.g., `1.21.11-rc3`).
                    //
                    // At the time of writing, there have been no pre-releases; I am unsure
                    // if this logic should apply to the other branch as well.
                    if self.major >= 26 {
                        buffer.push('-');
                    }

                    number
                }
            };

            buffer.push_str(number.to_string().as_str());
        }

        buffer.fmt(f)
    }
}

impl<'de> serde::Deserialize<'de> for Version {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let version = String::deserialize(deserializer)?;
        version.parse().map_err(serde::de::Error::custom)
    }
}

impl serde::Serialize for Version {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        self.to_string().serialize(serializer)
    }
}

#[derive(Debug)]
pub struct InvalidVersionError;

impl Error for InvalidVersionError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        None
    }
}

impl fmt::Display for InvalidVersionError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        "invalid format".fmt(f)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn semantic_version() {
        let raw = "1.21.9".to_owned();
        let version = Version {
            major: 1,
            minor: 21,
            patch: 9,
            release: None,
        };

        assert_eq!(raw.parse::<Version>().unwrap(), version);
        assert_eq!(version.to_string(), raw);
    }

    #[test]
    fn calendar_version() {
        let raw = "26.2".to_owned();
        let version = Version {
            major: 26,
            minor: 2,
            patch: 0,
            release: None,
        };

        assert_eq!(raw.parse::<Version>().unwrap(), version);
        assert_eq!(version.to_string(), raw);
    }

    #[test]
    fn pre() {
        let raw = "1.21.9-pre3".to_owned();
        let version = Version {
            major: 1,
            minor: 21,
            patch: 9,
            release: Some(Release::PreRelease(3)),
        };

        assert_eq!(raw.parse::<Version>().unwrap(), version);
        assert_eq!(version.to_string(), raw);
    }

    #[test]
    fn rc1() {
        let raw = "1.21.9-rc1".to_owned();
        let version = Version {
            major: 1,
            minor: 21,
            patch: 9,
            release: Some(Release::ReleaseCandidate(1)),
        };

        assert_eq!(raw.parse::<Version>().unwrap(), version);
        assert_eq!(version.to_string(), raw);
    }

    #[test]
    fn rc_2() {
        let raw = "26.2-rc-2".to_owned();
        let version = Version {
            major: 26,
            minor: 2,
            patch: 0,
            release: Some(Release::ReleaseCandidate(2)),
        };

        assert_eq!(raw.parse::<Version>().unwrap(), version);
        assert_eq!(version.to_string(), raw);
    }

    #[test]
    fn compare() {
        assert!(Version::from_str("26.2").unwrap() > Version::from_str("26.1.2").unwrap());
    }
}
