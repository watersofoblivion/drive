use clap::Parser;

use super::{Subcommand, Subcommands};

#[derive(Parser)]
#[command(version, about, long_about = None)]
pub struct App {
    #[arg(short, long, value_name = "FILE", default_value = "~/.drive/drive.db")]
    database: Option<String>,

    #[command(subcommand)]
    subcommand: Option<Subcommands>,
}

impl App {
    pub fn run() {
        let app = Self::parse();
        match &app.subcommand {
            Some(subcommand) => subcommand.run(&app),
            None => (),
        }
    }
}
