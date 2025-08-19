import json
import os
from typing import cast
from enum import Enum
from cleansweep.codecs.user_settings_codec import UserSettingsCodec
from cleansweep.globals.storage_paths import StoragePaths
from cleansweep.globals.user_setting_variant import SettingsVariant
from cleansweep.types.json import Json
from cleansweep.utils.get_main_path import get_main_path
from cleansweep.utils.get_user_settings import get_user_settings

def SettingsCommandDisplay(option: SettingsVariant):
    maybe_settings = get_user_settings(option)
    
    if not maybe_settings:
        print(f"There was an issue trying to load the user settings.. have you run the setup command?")
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
