import json
from typing import cast
from cleansweep.codecs.user_settings_codec import UserSettingsCodec
from cleansweep.globals.storage_paths import StoragePaths
from cleansweep.utils.get_main_path import get_main_path
from cleansweep.types.json import Json

import os 

def SettingsCommandListDefault() -> None:
    # Load the defaults and display
    if not os.path.exists(get_main_path() / StoragePaths.user_settings_defaults_file_name):
        print(f"Defaults file - File not found.. Have you run the setup command?")
        return
    try:
        unsanitised_json: Json

        with open(get_main_path() / StoragePaths.user_settings_defaults_file_name, "r") as file:
            unsanitised_json = cast(Json, json.load(file))

        defaults_maybe = UserSettingsCodec.create_from_json(unsanitised_json)
        
        if not defaults_maybe:
            print(f"Failed to format the json correctly")
            return 
        
        # Now print the data
        print(f"Date-cutoff-time for flagging files: {defaults_maybe.flag_date_cutoff}")
        print(f"Blacklist Files:")
        print(f"- ..with extension: {defaults_maybe.ignore_files_with_extension}")
        print(f"- ..name contains: {defaults_maybe.ignore_file_names_containing}")
        print(f"- ..directory contains: {defaults_maybe.ignore_files_whos_directory_contains}")
        print(f"- ..name starts with: {defaults_maybe.ignore_file_names_starting_with}")
        print(f"- ..smaller than: {defaults_maybe.ignore_files_smaller_than} bytes")
        print(f"- ..larger than: {defaults_maybe.ignore_files_larger_than} bytes")
        print(f"Whitelist Files:")
        print(f"- ..with extension: {defaults_maybe.prioritise_files_with_extension}")
        print(f"- ..name contains: {defaults_maybe.prioritise_file_names_containing}")
        print(f"- ..directory contains: {defaults_maybe.prioritise_files_whos_directory_contains}")
        print(f"- ..name starts with: {defaults_maybe.prioritise_file_names_starting_with}")
        print(f"- ..larger than: {defaults_maybe.prioritise_files_larger_than} bytes")
    except OSError as err:
        print(f"Failed to open the defaults user settings file: {err}, consider running the setup command")
        return
