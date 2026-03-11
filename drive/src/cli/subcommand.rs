use super::App;

pub trait Subcommand {
    fn run(&self, ctx: &App);
}
