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
    #[command(about = "Used to print a list of all files contained in the chosen list")]
    List {
        #[arg(long, value_enum, default_value_t = ListAndResetArgs::ToDelete)]
        choice: ListAndResetArgs,
    },

    #[command(
        about = "Used to move files from the chosen list to its opposing list based on a filter you pass"
    )]
    Override {
        #[arg(long, value_enum)]
        list_to_filter: KeepAndDelete,

        #[arg(long)]
        filter: String,

        #[arg(required = true, num_args = 1..)]
        values: Vec<String>,
    },

    #[command(about = "Used to remove the cleansweep installation on your device")]
    Demolish,

    #[command(
        about = "Used to decide how CleanSweep should determine what files from each set should be kept or deleted"
    )]
    ManageSets,

    #[command(
        about = "Used to execute the final sanity check before deletion, and the actual deletion"
    )]
    Purge {
        #[arg(value_enum)]
        choice: PurgeArgs,
    },

    #[command(about = "Used to wipe the chosen list")]
    Reset {
        #[arg(long, value_enum, default_value_t = ListAndResetArgs::ToDelete)]
        choice: ListAndResetArgs,
    },

    #[command(
        about = "Used to scan your provided path (or current directory) and sort into keep/delete list based on your settings"
    )]
    Scan {
        #[arg(long, default_value_t = String::from(""))]
        path: String,

        #[arg(long, short)]
        use_custom_filters: bool,

        #[arg(long, short)]
        no_filter: bool,

        #[arg(long, num_args = 0..)]
        ignore_dirs: Vec<String>,
    },

    #[command(about = "Used to scan for sets from your path or current directory")]
    SetScan {
        #[arg(long, default_value_t = String::from(""))]
        path: String,

        #[arg(long, num_args = 0..)]
        ignore_dirs: Vec<String>,
    },

    #[command(
        about = "Used to display, modify or reset your user settings outlined in the .cleansweep directory"
    )]
    Settings {
        #[arg(value_enum, default_value_t = SettingsArgs::Display)]
        choice: SettingsArgs,
    },

    #[command(about = "Used to set up the initial .cleansweep directory")]
    Setup,

    #[command(
        about = "Used to open a menu where you can easily view the hidden data for the files in your immediate (optionally recursive) directory"
    )]
    PrintHiddenStats {
        #[arg(long, default_value_t = String::from(""))]
        path: String,

        #[arg(long, short)]
        recursive: bool,

        #[arg(long, num_args = 0..)]
        ignore_dirs: Vec<String>,
    },
}

#[derive(Parser)]
#[command(name = "cleansweep")]
#[command(about = "CleanSweep: A CLI tool to assist in mass file deletion")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}
