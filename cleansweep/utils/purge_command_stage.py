
import json
from typing import cast
from cleansweep.codecs.file_array_codec import FileArrayCodec
from cleansweep.globals.log_levels import LogLevel
from cleansweep.globals.storage_paths import StoragePaths
from cleansweep.systems.logger_system import Logger
from cleansweep.types.json import Json
from cleansweep.utils.get_main_path import get_main_path


def purge_stage():
    # Load whitelist file, print number of files being staged, write to_delete
    try:
        unsanitised_to_delete: Json
        unsanitised_to_keep: Json

        with open(get_main_path() / StoragePaths.to_delete_file_name, "r") as file:
            unsanitised_to_delete = cast(Json, json.load(file))

        with open(get_main_path() / StoragePaths.to_keep_file_name, "r") as file:
            unsanitised_to_keep = cast(Json, json.load(file))

        maybe_to_delete_array = FileArrayCodec.create_from_json(unsanitised_to_delete)
        maybe_to_keep_array = FileArrayCodec.create_from_json(unsanitised_to_keep)

        if not maybe_to_delete_array or not maybe_to_keep_array:
            print("Issue trying to format an array of files. Are the file(s) empty?")
            return

        # List the number of files
        print(f"There are {len(maybe_to_delete_array)} files being staged for deletion")

        # Save each path on a separate line
        with open(StoragePaths.to_delete_local_temp_file_name, "w") as file:
            for path in maybe_to_delete_array:
                file.write(f"{path.get_path()}\n")

        with open(StoragePaths.to_keep_local_temp_file_name, "w") as file:
            for path in maybe_to_keep_array:
                file.write(f"{path.get_path()}\n")

        # Instruction statement
        print(f"The staged files have been written to {StoragePaths.to_delete_local_temp_file_name} in your current directory.")
        print(f"The files which will not be deleted have been listed in {StoragePaths.to_keep_local_temp_file_name}, ensure to perform a sanity check.")
        print(f"Please check this file for any files you don't want deleting. \nOnce ready, run 'cleansweep purge --continue")
    except Exception as err:
        Logger().add_line(f"There was an issue trying to stage the whitelisted files: '{err}', Maybe confirm the file exists and has data and try again", LogLevel.WARN)
