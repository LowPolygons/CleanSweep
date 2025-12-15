from pathlib import Path
from cleansweep.globals.storage_paths import StoragePaths
from cleansweep.interfaces.command import CommandInterface 
from cleansweep.systems.filter_system import FilterSystem 
from cleansweep.globals.flag_codes import FlagCodes
from cleansweep.containers.file_item import FileItem
from cleansweep.utils.get_user_settings import get_user_settings
from cleansweep.globals.user_setting_variant import SettingsVariant
from cleansweep.utils.get_main_path import get_main_path
from cleansweep.codecs.file_array_codec import FileArrayCodec
from argparse import Namespace, _SubParsersAction
from typing import cast
from cleansweep.types.json import Json

import json
import os

class ActivateOverrideCommand(CommandInterface):
    @staticmethod
    def command(args: Namespace) -> None:
        print("The ToKeep files will now be iterated over -\nAny which meet your override filter stats will be added to the Delete list")

        maybe_user_settings = get_user_settings(SettingsVariant.Regular)

        if not maybe_user_settings:
            print("There was an error trying to load user settings.. have you run the setup command?")
            return

        user_home_dir: Path = Path.home()
        storage_path: StoragePaths = StoragePaths(user_home_dir)

        # Create the main cleansweep dir
        main_folder: Path = user_home_dir / storage_path.main_dir_name
        
        # Create the main folder if it doesn't exist 
        if not main_folder.exists():
            print("CleanSweep has not been initialised")
            return
    
        unsanitised_json_keep: Json
        unsanitised_json_delete: Json

        try:
            with open(get_main_path() / storage_path.to_delete_file_name, "r") as file:
                unsanitised_json_delete = cast(Json, json.load(file))
            with open(get_main_path() / storage_path.to_keep_file_name, "r") as file:
                unsanitised_json_keep = cast(Json, json.load(file))
        except OSError as err:
            Logger().add_line(f"There was an error trying to perform overriding: {err}", LogLevel.ERROR)
            return

        maybe_to_keep = FileArrayCodec.create_from_json(unsanitised_json_keep)
        maybe_to_delete = FileArrayCodec.create_from_json(unsanitised_json_delete)
        overriden_files: list[FileItem] = []

        if maybe_to_keep is None or maybe_to_delete is None:
            print("Issue trying to create the items from the file array - have you run the scan?")
            return
        
        if not maybe_to_keep: # Apparently this is the pythonic way of checking if a list is empty
            print("Your ToKeep list is empty, there is nothing to override")
            return
        
        for file_index in range (len(maybe_to_keep)-1, -1, -1):
            file = maybe_to_keep[file_index]
            matches_override = FilterSystem.file_meets_flag_requirements(file, maybe_user_settings, FlagCodes.Override)

            if matches_override:
                popped_file = maybe_to_keep.pop(file_index)
                overriden_files.append(popped_file)
        
        if len(overriden_files) == 0:
            print("No files were flagged for overriding.")
            return

        print("The following files have been added to the delete list:")
        for file in overriden_files:
            maybe_to_delete.append(file)

            print(f"- {file.get_path()}")
        print(f"({len(overriden_files)} files moved)")

        # Save them
        jsoned_to_keep = FileArrayCodec.encode_to_json(maybe_to_keep)
        jsoned_to_delete = FileArrayCodec.encode_to_json(maybe_to_delete)

        try:
            with open(get_main_path() / StoragePaths.to_keep_file_name, "w") as file:
                json.dump(jsoned_to_keep, file)
            with open(get_main_path() / StoragePaths.to_delete_file_name, "w") as file:
                json.dump(jsoned_to_delete, file)
        except OSError as err:
            Logger().add_line(f"There was an error trying to perform overriding: {err}", LogLevel.ERROR)
            return
            

    @classmethod
    def register_subparser(cls, subparsers: _SubParsersAction) -> None:
        acov_parser = subparsers.add_parser('activate-override', help="Command to apply the override filtering on your previously scanned files")
        acov_parser.set_defaults(func=cls.command)
