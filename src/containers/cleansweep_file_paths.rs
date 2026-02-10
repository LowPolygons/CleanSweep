pub enum FileStyle {
    Json(&'static str),
    Txt(&'static str),
    PathOnly(&'static str),
    NoExtension(&'static str),
}

pub enum CleansweepFilePaths {
    MainDirectoryName(FileStyle), // All stored as seen here in $HOME/
    ToDelete(FileStyle),
    ToKeep(FileStyle),
    UserSettings(FileStyle),
    LogFile(FileStyle),
    UserSettingsDefault(FileStyle),
    ToDeleteLocalTemp(FileStyle),
    ToKeepLocalTemp(FileStyle),
    FoundSets(FileStyle),
    FilterComponentList(FileStyle),
}

#[rustfmt::skip]
impl CleansweepFilePaths {
    pub fn name(self) -> FileStyle {
        match self {
            CleansweepFilePaths::MainDirectoryName(_) => FileStyle::PathOnly(".cleansweep"),
            CleansweepFilePaths::ToDelete(_) => FileStyle::Json("to_delete_files.json"),
            CleansweepFilePaths::ToKeep(_) => FileStyle::Json("to_keep_files.json"),
            CleansweepFilePaths::UserSettings(_) => FileStyle::Json("user_settings.json"),
            CleansweepFilePaths::UserSettingsDefault(_) => FileStyle::Json("user_settings_default.json"),
            CleansweepFilePaths::LogFile(_) => FileStyle::Txt("log.txt"),
            CleansweepFilePaths::ToDeleteLocalTemp(_) => FileStyle::NoExtension("STAGED_FOR_DELETION"),
            CleansweepFilePaths::ToKeepLocalTemp(_) => FileStyle::NoExtension("FILES_KEPT_SANITY_CHECK"),
            CleansweepFilePaths::FoundSets(_) => FileStyle::Json("found_sets.json"),
            CleansweepFilePaths::FilterComponentList(_) => FileStyle::Txt("filter_components.txt"),
        }
    }
}
