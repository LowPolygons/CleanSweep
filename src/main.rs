// Rust Clap Crate - https://dev.to/moseeh_52/getting-started-with-clap-a-beginners-guide-to-rust-cli-apps-1n3f
mod cli;
mod commands;
mod containers;
mod filter_codes;
mod systems;
mod utils;

use clap::Parser;
use cli::{Cli, Commands};

use crate::commands::{
    activate_override, demolish, list, manage_sets, purge, reset, scan, set_scan, settings, setup,
};

fn main() -> Result<(), String> {
    let cli = Cli::parse();

    match &cli.command {
        Commands::List { choice } => list::list(&choice),
        Commands::ActivateOverride => activate_override::activate_override(),
        Commands::Demolish => demolish::demolish(),
        Commands::ManageSets => manage_sets::manage_sets(),
        Commands::Purge { choice } => purge::purge(&choice),
        Commands::Reset { choice } => reset::reset(&choice),
        Commands::Scan {
            path,
            use_custom_filters,
            append_mode,
        } => scan::scan(&path, use_custom_filters, append_mode),
        Commands::SetScan { path } => set_scan::set_scan(&path),
        Commands::Settings { choice } => settings::settings(&choice),
        Commands::Setup => setup::setup(),
    }?;

    Ok(())
}
