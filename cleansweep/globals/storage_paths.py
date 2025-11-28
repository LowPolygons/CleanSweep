from dataclasses import dataclass
from pathlib import Path

@dataclass
class StoragePaths:
    # Where the files will be located
    path_to_home: Path
    main_dir_name: str = ".cleansweep"

    # The file names
    to_delete_file_name: str = "to_delete_files.json"
    to_keep_file_name: str = "to_keep_files.json"

    user_settings_file_name: str = "user_settings.json"
    log_file_name: str = "log.txt"
    storage_paths_file_name: str = "storage_paths.json"    
    user_settings_defaults_file_name: str = "user_settings_defaults.json"
    to_delete_local_temp_file_name: str = "STAGED_FOR_DELETION"

    found_sets_file_name: str = "sets_that_were_found.json"
