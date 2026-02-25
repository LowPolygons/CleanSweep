// Rust Clap Crate - https://dev.to/moseeh_52/getting-started-with-clap-a-beginners-guide-to-rust-cli-apps-1n3f
mod cleansweep_error;
mod cli;
mod commands;
mod containers;
mod filter_codes;
mod systems;
mod utils;

use std::process::ExitCode;

use clap::Parser;
use cli::{Cli, Commands};

use crate::{
    cleansweep_error::CleanSweepErrors,
    commands::{
        demolish, list, manage_sets, override_command, print_hidden_stats, purge, reset, scan,
        set_scan, settings, setup,
    },
};

fn main() -> ExitCode {
    let cli = Cli::parse();

    let success: Result<(), CleanSweepErrors> =
        match &cli.command {
            Commands::ManageSets => manage_sets::command::manage_sets()
                .map_err(|e| CleanSweepErrors::ManageSetsFailure(e)),
            Commands::List { choice, summarise } => {
                list::list(&choice, summarise).map_err(|e| CleanSweepErrors::ListFailure(e))
            }
            Commands::Override {
                list,
                filter,
                values,
            } => override_command::override_command(list, filter, values.clone())
                .map_err(|e| CleanSweepErrors::OverrideFailure(e)),
            Commands::Demolish => {
                demolish::demolish().map_err(|e| CleanSweepErrors::DemolishFailure(e))
            }
            Commands::Purge { choice } => {
                purge::purge(&choice).map_err(|e| CleanSweepErrors::PurgeFailure(e))
            }
            Commands::Reset { choice } => {
                reset::reset(&choice).map_err(|e| CleanSweepErrors::ResetFailure(e))
            }
            Commands::Scan {
                path,
                use_custom_filters,
                no_filter,
                ignore_dirs,
            } => scan::scan(&path, use_custom_filters, no_filter, ignore_dirs)
                .map_err(|e| CleanSweepErrors::ScanFailure(e)),
            Commands::SetScan { path, ignore_dirs } => set_scan::set_scan(&path, ignore_dirs)
                .map_err(|e| CleanSweepErrors::SetScanFailure(e)),
            Commands::Settings { choice } => {
                settings::settings(&choice).map_err(|e| CleanSweepErrors::SettingsFailure(e))
            }
            Commands::Setup => setup::setup().map_err(|e| CleanSweepErrors::SetupFailure(e)),
            Commands::PrintHiddenStats {
                path,
                recursive,
                ignore_dirs,
            } => print_hidden_stats::print_hidden_stats(path, recursive, ignore_dirs)
                .map_err(|e| CleanSweepErrors::PrintHiddenStatsFailure(e)),
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
