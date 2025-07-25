from dataclasses import dataclass
from pathlib import Path

@dataclass
class StoragePaths:
    # Where the files will be located
    path_to_home: Path
    main_dir_name: str = ".cleansweep"
    # The file names
    white_listed_file_name: str = "white_listed_files.json" 
    user_settings_file_name: str = "user_settings.json"
    log_file_name: str = "log.txt"

    @staticmethod
    def get_formatted_path_with_file(file_name: str) -> str:
        return str(StoragePaths.path_to_home) + "/" + StoragePaths.main_dir_name + "/" + file_name
