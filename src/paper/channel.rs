use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Channel {
    #[expect(dead_code)]
    Alpha,
    #[expect(dead_code)]
    Beta,
    Stable,
}

impl fmt::Display for Channel {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            Self::Alpha => "ALPHA".fmt(f),
            Self::Beta => "BETA".fmt(f),
            Self::Stable => "STABLE".fmt(f),
        }
    }
}
