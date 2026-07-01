use std::io;

use clap::CommandFactory as _;

use crate::commands::Parser;
use crate::commands::prelude::*;

/// Arguments for the `completions` subcommand.
#[derive(clap::Args)]
pub struct Completions {
    shell: clap_complete::Shell,
}

impl Run for Completions {
    async fn run(&self, _: &mut Context) -> anyhow::Result<()> {
        let mut command = Parser::command();
        let program = env!("CARGO_BIN_NAME");
        let mut stdout = io::stdout();

        clap_complete::generate(self.shell, &mut command, program, &mut stdout);

        Ok(())
    }
}
