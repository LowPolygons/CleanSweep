pub enum CleansweepFilePaths {
    MainDirectoryName, // All stored as seen here in $HOME/
    ToDelete,
    ToKeep,
    UserSettings,
    LogFile,
    UserSettingsDefault,
    ToDeleteLocalTemp,
    ToKeepLocalTemp,
    FoundSets,
}

impl CleansweepFilePaths {
    pub fn name(self) -> &'static str {
        match self {
            CleansweepFilePaths::MainDirectoryName => ".cleansweep",
            CleansweepFilePaths::ToDelete => "to_delete_files.json",
            CleansweepFilePaths::ToKeep => "to_keep_files.json",
            CleansweepFilePaths::UserSettings => "user_settings.json",
            CleansweepFilePaths::UserSettingsDefault => "user_settings_default.json",
            CleansweepFilePaths::LogFile => "log.txt",
            CleansweepFilePaths::ToDeleteLocalTemp => "STAGED_FOR_DELETION",
            CleansweepFilePaths::ToKeepLocalTemp => "FILES_KEPT_SANITY_CHECK",
            CleansweepFilePaths::FoundSets => "found_sets.json",
        }
    }
}
