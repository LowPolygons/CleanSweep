from dataclasses import dataclass
from pathlib import Path

@dataclass
class StoragePaths:
    # Where the files will be located
    path_to_home: Path
    main_dir_name: str = ".cleansweep"
    # The file names
    white_listed_file_name: str = "white_listed_files.json" 
    black_listed_file_name: str = "black_listed_files.json"
    user_settings_file_name: str = "user_settings.json"
    log_file_name: str = "log.txt"
    storage_paths_file_name: str = "storage_paths.json"    
    user_settings_defaults_file_name: str = "user_settings_defaults.json"
    to_delete_local_temp_file_name: str = "STAGED_FOR_DELETION"
