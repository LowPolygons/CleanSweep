import os
import json

from typing import Optional, cast
from cleansweep.codecs.user_settings_codec import UserSettingsCodec
from cleansweep.containers.user_settings import UserSettings
from cleansweep.globals.storage_paths import StoragePaths
from cleansweep.globals.user_setting_variant import SettingsVariant
from cleansweep.types.json import Json
from cleansweep.utils.get_main_path import get_main_path

def get_user_settings(option: SettingsVariant) -> Optional[UserSettings]:
    settings_file: str =  StoragePaths.user_settings_file_name
    if option == option.Regular:
        settings_file = StoragePaths.user_settings_file_name
    elif option == option.Defaults:
        settings_file = StoragePaths.user_settings_defaults_file_name

    if not os.path.exists(get_main_path() / settings_file):
        print(f"User settings file not found.. have you run the setup command?")
        return None
    
    try:
        unsanitised_json: Json

        with open(get_main_path() / settings_file, "r") as file:
            unsanitised_json = cast(Json, json.load(file))
        maybe_settings = UserSettingsCodec.create_from_json(unsanitised_json)
        
        if not maybe_settings:
            print("Failed to format the json correctly")
            return None

        # success
        return maybe_settings
    except Exception as err:
        print(f"There was an error trying ro retrieve user settings: {err}")
        return None
