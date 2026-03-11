use clap::Args;

use super::super::{App, Subcommand};

#[derive(Args)]
pub struct Import {}

impl Subcommand for Import {
    fn run(&self, ctx: &App) {}
}
