# Setup

## **[Back](functionality.md)**

CleanSweep contains an algebraic Enum, with each value mapping to a relevant file path

```rs
pub enum CleansweepFilePaths {
    MainDirectoryName,
    ToDelete,
    ToKeep,
    UserSettings,
    LogFile,
    UserSettingsDefault,
    ToDeleteLocalTemp,
    ToKeepLocalTemp,
    FoundSets,
    FilterComponentList,
}

CleansweepFilePaths::MainDirectoryName => ".cleansweep",
CleansweepFilePaths::ToDelete => "to_delete_files.json",
CleansweepFilePaths::ToKeep => "to_keep_files.json",
CleansweepFilePaths::UserSettings => "user_settings.json",
CleansweepFilePaths::UserSettingsDefault => "user_settings_default.json",
CleansweepFilePaths::LogFile => "log.txt",
CleansweepFilePaths::ToDeleteLocalTemp => "STAGED_FOR_DELETION",
CleansweepFilePaths::ToKeepLocalTemp => "FILES_KEPT_SANITY_CHECK",
CleansweepFilePaths::FoundSets => "found_sets.json",
CleansweepFilePaths::FilterComponentList => "filter_components.txt",
```

When running setup, a folder in your home directory called `.cleansweep` is made, and all the necessary files to get cleansweep going are initialised. 

```md
- ..log.txt
- ..filter_components.txt
- ..to_delete_files.json
- ..to_keep_files.json
- ..found_sets.json
- ..user_settings.json
- ..user_settings_default.json
```

The User Settings are initialised to the defaults, as seen below

```md
Keep Files...
.. with extension ["z", "exe", "d"]
.. whose name is or contains ["cleansweep"]
.. whose directory contains ["cleansweep"]
.. whose larger than 10000000000
.. whose smaller than 1001
Delete Files...
.. with extension ["out"]
.. whose name is or contains ["OUTPUT", "HISTORY", "slurm-"]
.. whose directory contains ["deleteme"]
.. whose larger than 1001
.. whose smaller than 10000000000
Files which may be in sets..
.. have extension ["h5"]
.. have name containing []
```


