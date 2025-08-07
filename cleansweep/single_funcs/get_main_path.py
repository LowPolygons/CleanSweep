from pathlib import Path
from cleansweep.globals.storage_paths import StoragePaths


def get_main_path() -> Path:
    user_home_dir: Path = Path.home()
    initial_storage_path: StoragePaths = StoragePaths(user_home_dir)

    # Create the main cleansweep dir
    main_folder: Path = user_home_dir / initial_storage_path.main_dir_name

    return main_folder
