import json
import os
from typing import cast
from enum import Enum
from cleansweep.codecs.user_settings_codec import UserSettingsCodec
from cleansweep.globals.storage_paths import StoragePaths
from cleansweep.types.json import Json
from cleansweep.utils.get_main_path import get_main_path

class DisplayOption(Enum):
    Regular = 0
    Defaults = 1

def SettingsCommandDisplay(option: DisplayOption):
    settings_file: str
    if option == option.Regular:
        settings_file = StoragePaths.user_settings_file_name
    elif option == option.Defaults:
        settings_file = StoragePaths.user_settings_defaults_file_name

    if not os.path.exists(get_main_path() / settings_file):
        print(f"User settings file not found.. have you run the setup command?")
        return
    
    try:
        unsanitised_json: Json
        
        with open(get_main_path() / settings_file, "r") as file:
            unsanitised_json = cast(Json, json.load(file))
        
        maybe_settings = UserSettingsCodec.create_from_json(unsanitised_json)

        if not maybe_settings:
            print("Failed to format the json correctly")
            return

        # Print the user settings
        print(f"Date-cutoff-time for flagging files: {maybe_settings.flag_date_cutoff}")
        print(f"Blacklist Files:")
        print(f"- ..with extension: {maybe_settings.ignore_files_with_extension}")
        print(f"- ..name contains: {maybe_settings.ignore_file_names_containing}")
        print(f"- ..directory contains: {maybe_settings.ignore_files_whos_directory_contains}")
        print(f"- ..name starts with: {maybe_settings.ignore_file_names_starting_with}")
        print(f"- ..smaller than: {maybe_settings.ignore_files_smaller_than} bytes")
        print(f"- ..larger than: {maybe_settings.ignore_files_larger_than} bytes")
        print(f"Whitelist Files:")
        print(f"- ..with extension: {maybe_settings.prioritise_files_with_extension}")
        print(f"- ..name contains: {maybe_settings.prioritise_file_names_containing}")
        print(f"- ..directory contains: {maybe_settings.prioritise_files_whos_directory_contains}")
        print(f"- ..name starts with: {maybe_settings.prioritise_file_names_starting_with}")
        print(f"- ..larger than: {maybe_settings.prioritise_files_larger_than} bytes")
    except OSError as err:
        print(f"There was issue trying to open the user settings file: {err}, consider running the setup command")
