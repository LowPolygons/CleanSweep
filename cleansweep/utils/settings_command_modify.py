from cleansweep.containers.user_settings import UserSettings
from cleansweep.globals.storage_paths import StoragePaths
from cleansweep.utils.get_main_path import get_main_path
from cleansweep.utils.settings_command_display import DisplayOption, SettingsCommandDisplay

def interactive_environment(session_cutoff: int) -> None:
    # TODO: Consider an ideological refactor to make this better

    main_path = get_main_path()

    print(f"Your user settings can be found at {main_path / StoragePaths.user_settings_file_name}")
    print(f"If it is empty, run the settings reset command")
    # Attempt to display the user_settings initially
