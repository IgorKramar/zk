mod cli;
mod notes;
mod config;
mod commands;
mod templates;

use cli::Cli;
use clap::Parser;

fn main() {
    let cli = Cli::parse();
    commands::handle_command(cli.command);
}