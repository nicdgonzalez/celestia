use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Project {
    #[expect(dead_code)]
    Folia,
    Paper,
    #[expect(dead_code)]
    Travertine,
    #[expect(dead_code)]
    Velocity,
    #[expect(dead_code)]
    Waterfall,
}

impl Project {
    #[must_use]
    pub fn as_str(&self) -> &str {
        match *self {
            Self::Folia => "folia",
            Self::Paper => "paper",
            Self::Travertine => "travertine",
            Self::Velocity => "velocity",
            Self::Waterfall => "waterfall",
        }
    }
}

impl fmt::Display for Project {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.as_str().fmt(f)
    }
}
