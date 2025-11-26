import json
import os
from typing import Optional, cast
from enum import Enum
from cleansweep.codecs.user_settings_codec import UserSettingsCodec
from cleansweep.containers.user_settings import UserSettings
from cleansweep.globals.storage_paths import StoragePaths
from cleansweep.globals.user_setting_variant import SettingsVariant
from cleansweep.types.json import Json
from cleansweep.utils.get_main_path import get_main_path
from cleansweep.utils.get_user_settings import get_user_settings

def SettingsCommandDisplay(option: SettingsVariant, maybe_settings: Optional[UserSettings] = None) -> None:
    if not maybe_settings:
        maybe_settings = get_user_settings(option)
    
        if not maybe_settings:
            print(f"There was an issue trying to load the user settings.. have you run the setup command?")
            return
    # Print the user settings
    print(f"\n1 - Date-cutoff-time for flagging files: {maybe_settings.flag_date_cutoff}")
    print(f"Files to Keep:")
    print(f"2 - ..with extension: {maybe_settings.ignore_files_with_extension}")
    print(f"3 - ..name contains: {maybe_settings.ignore_file_names_containing}")
    print(f"4 - ..directory contains: {maybe_settings.ignore_files_whos_directory_contains}")
    print(f"5 - ..name starts with: {maybe_settings.ignore_file_names_starting_with}")
    print(f"6 - ..smaller than: {maybe_settings.ignore_files_smaller_than} bytes")
    print(f"7 - ..larger than: {maybe_settings.ignore_files_larger_than} bytes")
    print(f"Files to Delete:")
    print(f"8 - ..with extension: {maybe_settings.prioritise_files_with_extension}")
    print(f"9 - ..name contains: {maybe_settings.prioritise_file_names_containing}")
    print(f"10 - ..name starts with: {maybe_settings.prioritise_file_names_starting_with}")
    print(f"11 - ..directory contains: {maybe_settings.prioritise_files_whos_directory_contains}")
    print(f"12 - ..larger than: {maybe_settings.prioritise_files_larger_than} bytes\n")
    print(f"13 - Consider Access Date when Filtering: {maybe_settings.consider_access_date_when_filtering}")
