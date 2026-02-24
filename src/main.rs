// Rust Clap Crate - https://dev.to/moseeh_52/getting-started-with-clap-a-beginners-guide-to-rust-cli-apps-1n3f
mod cli;
mod commands;
mod containers;
mod filter_codes;
mod systems;
mod utils;

use std::process::ExitCode;

use clap::Parser;
use cli::{Cli, Commands};

use crate::commands::{
    demolish, list, manage_sets, override_command, print_hidden_stats::print_hidden_stats, purge,
    reset, scan, set_scan, settings, setup,
};

fn main() -> ExitCode {
    let cli = Cli::parse();

    let success: Result<(), String> = match &cli.command {
        Commands::List { choice } => list::list(&choice),
        Commands::Override {
            list_to_filter,
            filter,
            values,
        } => override_command::override_command(list_to_filter, filter, values.clone()),
        Commands::Demolish => demolish::demolish(),
        Commands::ManageSets => manage_sets::command::manage_sets(),
        Commands::Purge { choice } => purge::purge(&choice),
        Commands::Reset { choice } => reset::reset(&choice),
        Commands::Scan {
            path,
            use_custom_filters,
            no_filter,
            ignore_dirs,
        } => scan::scan(&path, use_custom_filters, no_filter, ignore_dirs),
        Commands::SetScan { path, ignore_dirs } => set_scan::set_scan(&path, ignore_dirs),
        Commands::Settings { choice } => settings::settings(&choice),
        Commands::Setup => setup::setup(),
        Commands::PrintHiddenStats {
            path,
            recursive,
            ignore_dirs,
        } => print_hidden_stats(path, recursive, ignore_dirs),
    };

    match success {
        Ok(_) => {}
        Err(msg) => {
            println!("{}", msg);

            return ExitCode::FAILURE;
        }
    }

    ExitCode::SUCCESS
}
