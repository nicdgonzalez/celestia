//! This crate implements core concepts independent of any particular user interface.

#![warn(
    clippy::correctness,
    clippy::suspicious,
    clippy::complexity,
    clippy::perf,
    clippy::style,
    clippy::pedantic
)]

mod manifest;
mod package;
mod plugin;
mod registry;
mod server;

pub use manifest::Manifest;
pub use package::Package;
pub use plugin::Plugin;
pub use registry::PluginRegistry;
pub use server::Server;
