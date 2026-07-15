use std::io::{self, Stderr};

use crate::notifier::Notifier;

/// Shared state for command execution.
pub struct Context {
    notifier: Notifier<Stderr>,
}

impl Context {
    /// Constructs a new shared state.
    #[must_use]
    pub fn new() -> Self {
        Self {
            notifier: Notifier::new(io::stderr()),
        }
    }

    #[must_use]
    pub const fn notifier(&mut self) -> &mut Notifier<Stderr> {
        &mut self.notifier
    }
}
