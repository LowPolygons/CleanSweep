
from argparse import Namespace
import json
import os
from pathlib import Path
from cleansweep.codecs.file_array_codec import FileArrayCodec
from cleansweep.containers.file_item import FileItem
from cleansweep.globals.flag_codes import FlagCodes
from cleansweep.globals.log_levels import LogLevel
from cleansweep.globals.storage_paths import StoragePaths
from cleansweep.globals.user_setting_variant import SettingsVariant
from cleansweep.systems.filter_system import FilterSystem
from cleansweep.systems.logger_system import Logger
from cleansweep.systems.scanning_system import FileScanningManager
from cleansweep.utils.get_main_path import get_main_path
from cleansweep.utils.get_user_settings import get_user_settings
from cleansweep.utils.convert_size_to_reasonable_unit import convert_size_to_reasonable_unit, DataSizes, match_datasize_to_string

def scan(args: Namespace):
    # Load user settings
    maybe_user_settings = get_user_settings(SettingsVariant.Regular)

    if not maybe_user_settings:
        print("There was an error trying to load user settings.. have you run the setup command?")
        return
    # Get the users additional path if included
    starting_dir: str = ""
    
    if args.path:
        starting_dir = args.path
        
        if not os.path.exists(Path(starting_dir)):
            print("The provided path doesn't exist..")
            return

    # Paths 
    print(f"Scanning path {starting_dir}")
    scanned_paths: list[Path] = FileScanningManager.get_file_names_recursive(Path(starting_dir))  
    files: list[FileItem] = []

    # Convert into file items
    print(f"Converting paths into File Structures")
    for path in scanned_paths:
        curr_file = FileItem(path)
        success = curr_file.stat_calculate()
        if not success:
            print(f"Failed to initialise file at path {path}")
            continue
        files.append(curr_file)

    # Filter through them to get the black/white listed
    to_keep: list[FileItem] = []
    to_delete: list[FileItem] = []
    other_flagged: list[FileItem] = []

    for curr_file in files:
        file_flag_status: FlagCodes = FilterSystem.file_is_flagged(curr_file, maybe_user_settings)

        if file_flag_status == FlagCodes.FlaggedToKeep:
            to_keep.append(curr_file)
        elif file_flag_status == FlagCodes.FlaggedToDelete:
            to_delete.append(curr_file)
        elif file_flag_status == FlagCodes.Flagged:
            other_flagged.append(curr_file)
    
    keep_bytes: int = 0
    delete_bytes: int = 0
    for file in to_keep:
        keep_bytes += file.get_path().stat().st_size  
    for file in to_delete:
        delete_bytes += file.get_path().stat().st_size
    nice_keep = convert_size_to_reasonable_unit(keep_bytes)
    nice_delete = convert_size_to_reasonable_unit(delete_bytes)
    
    print(f"The Keep list has a size of {nice_keep[0]} {match_datasize_to_string(nice_keep[1])} across {len(to_keep)} files")
    print(f"The Delete list has a size of {nice_delete[0]} {match_datasize_to_string(nice_delete[1])} across {len(to_delete)} files")

    # Save them
    jsoned_to_keep = FileArrayCodec.encode_to_json(to_keep)
    jsoned_to_delete = FileArrayCodec.encode_to_json(to_delete)
    jsoned_other_flagged = FileArrayCodec.encode_to_json(other_flagged)

    try:
        with open(get_main_path() / StoragePaths.to_keep_file_name, "w") as file:
            json.dump(jsoned_to_keep, file)
        with open(get_main_path() / StoragePaths.to_delete_file_name, "w") as file:
            json.dump(jsoned_to_delete, file)
        with open(get_main_path() / StoragePaths.minimum_flagged_file_name, "w") as file:
            json.dump(jsoned_other_flagged, file)
    except OSError as err:
        Logger().add_line(f"There was an error trying to save the black/white listed files: {err}", LogLevel.ERROR)
        return

    print("Successfully scanned and saved the flagged files.")
