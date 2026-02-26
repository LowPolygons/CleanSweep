use thiserror::Error;

use crate::commands::{
    demolish::DemolishError, list::ListError, manage_sets::command::ManageSetsError,
    override_command::OverrideError, print_hidden_stats::PrintHiddenStatsError, purge::PurgeError,
    reset::ResetError, scan::ScanError, set_scan::SetScanError, settings::SettingsError,
    setup::SetupError,
};

#[derive(Debug, Error)]
pub enum CleanSweepErrors {
    #[error("List Command Failure: {0}")]
    ListFailure(ListError),

    #[error("Override Command Failure: {0}")]
    OverrideFailure(OverrideError),

    #[error("Demolish Command Failure: {0}")]
    DemolishFailure(DemolishError),

    #[error("Manage Sets Command Failure: {0}")]
    ManageSetsFailure(ManageSetsError),

    #[error("Purge Command Failure: {0}")]
    PurgeFailure(PurgeError),

    #[error("Reset Command Failure: {0}")]
    ResetFailure(ResetError),

    #[error("Scan Command Failure: {0}")]
    ScanFailure(ScanError),

    #[error("Set Scan Command Failure: {0}")]
    SetScanFailure(SetScanError),

    #[error("Settings Command Failure: {0}")]
    SettingsFailure(SettingsError),

    #[error("Setup Command Failure: {0}")]
    SetupFailure(SetupError),

    #[error("Print Hidden Stats Command Failure: {0}")]
    PrintHiddenStatsFailure(PrintHiddenStatsError),
}
