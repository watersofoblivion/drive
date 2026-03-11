use clap::Subcommand as ClapSubcommand;

use super::{App, Subcommand};

mod import;

use import::Import;

#[derive(ClapSubcommand)]
pub enum Subcommands {
    Import(Import),
}

impl Subcommand for Subcommands {
    fn run(&self, ctx: &App) {
        match self {
            Self::Import(cmd) => cmd.run(ctx),
        }
    }
}
