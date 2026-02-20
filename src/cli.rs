use clap::{Parser, Subcommand};

#[derive(clap::ValueEnum, Clone, Debug)]
pub enum ListAndResetArgs {
    ToDelete,
    ToKeep,
    Sets,
}

#[derive(clap::ValueEnum, Clone, Debug)]
pub enum KeepAndDelete {
    ToKeep,
    ToDelete,
}

#[derive(clap::ValueEnum, Clone, Debug)]
pub enum SettingsArgs {
    Reset,
    Modify,
    Display,
}

#[derive(clap::ValueEnum, Clone, Debug)]
pub enum PurgeArgs {
    Stage,
    Continue,
}

#[derive(Subcommand, Clone)]
pub enum Commands {
    #[command(about = "List command")]
    List {
        #[arg(long, value_enum, default_value_t = ListAndResetArgs::ToDelete)]
        choice: ListAndResetArgs,
    },

    #[command(about = "Override")]
    Override {
        #[arg(long, value_enum)]
        list_to_filter: KeepAndDelete,

        #[arg(long)]
        filter: String,
    },

    #[command(about = "Demolish")]
    Demolish,

    #[command(about = "Manage Sets")]
    ManageSets,

    #[command(about = "Purge")]
    Purge {
        #[arg(value_enum)]
        choice: PurgeArgs,
    },

    #[command(about = "Reset")]
    Reset {
        #[arg(long, value_enum, default_value_t = ListAndResetArgs::ToDelete)]
        choice: ListAndResetArgs,
    },

    #[command(about = "Scan")]
    Scan {
        #[arg(long, default_value_t = String::from(""))]
        path: String,

        #[arg(long, short)]
        use_custom_filters: bool,

        #[arg(long, short)]
        append_mode: bool,

        #[arg(long, short)]
        no_filter: bool,
    },

    #[command(about = "SetScan")]
    SetScan {
        #[arg(long, default_value_t = String::from(""))]
        path: String,
    },

    #[command(about = "Settings")]
    Settings {
        #[arg(value_enum, default_value_t = SettingsArgs::Display)]
        choice: SettingsArgs,
    },

    #[command(about = "Setup")]
    Setup,
}

#[derive(Parser)]
#[command(name = "cleansweep")]
#[command(about = "CleanSweep: A CLI tool to assist in mass file deletion")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}
