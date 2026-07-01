//! Wrapper for the Paper API.

mod build;
pub use build::Build;

mod channel;
pub use channel::Channel;

mod client;
pub use client::Client;

mod error;
#[expect(unused_imports)]
pub use error::PaperError;

pub mod model;

mod project;
pub use project::Project;

mod version;
pub use version::Version;
