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
from cleansweep.codecs.file_set_array_codec import FileSetArrayCodec
from cleansweep.utils.split_string_into_str_and_number import extract_number_from_end_of_string


# TODO: Perhaps implement the file date stuff too
def set_scan(args: Namespace):
   # Load user settings
    maybe_user_settings = get_user_settings(SettingsVariant.Regular)

    if not maybe_user_settings:
        print("There was an error trying to load user settings.. have you run the setup command?")
        return
    # Get the users additional path if included
    starting_dir: str = os.getcwd()
    
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

    # Filter through them to get the potential sets
    potential_set_item: list[FileItem] = []
    for curr_file in files:
        file_flag_status: bool = curr_file.maybe_in_set(maybe_user_settings.set_may_have_extension, \
                                                        maybe_user_settings.set_file_name_may_contain)
        if file_flag_status:
            potential_set_item.append(curr_file)

    # Stores the full sets
    created_sets: list[list[str]] = []
    
    # Stores the file names separated from the path-> path : [(stem, suffix)]
    separated_files: dict[str, list[tuple[str, str]]] = {}

    for file in potential_set_item:
        path = file.get_path()
        dir_path = str(path.parent)
        
        if dir_path in separated_files:
            separated_files[str(path.parent)].append((path.stem, path.suffix.lstrip(".")))
        else:
            separated_files[str(path.parent)] = [(path.stem, path.suffix.lstrip("."))]

    # Begin iterating over the directories
    for directory, files in separated_files.items():
        # string portion, number portion, file extension 
        split_file_names: list[tuple[str, str, str]] = [] 

        # Populate split file names
        for file in files:
            possible_split = extract_number_from_end_of_string(file[0])

            if possible_split:
                split_file_names.append((possible_split[0], possible_split[1], file[1]))
        
        # Calculate all unique string portions (USP) and send the string into each array
        usps: dict[str, list[tuple[str, str, str]]] = {}

        for string_parts in split_file_names:
            if string_parts[0] in usps:
                usps[string_parts[0]].append(string_parts)
            else:
                usps[string_parts[0]] = [string_parts]

        # Sort USPs based on the number portion 
        for usp, tup in usps.items():
            tup.sort(key=lambda x: float(x[1]))
    
            # Now the lists are sorted they need to be collapsed into the list[str]
            put_together_strings: list[str] = []
            
            if len(tup) > 1:
                for file_name in tup:
                    put_together_strings.append(f"{directory}/{file_name[0]}{file_name[1]}.{file_name[2]}")

                # Insert into created_Sets array
                created_sets.append(put_together_strings)

    number_of_sets = len(created_sets)
    print(f"\nNumber of sets found: {number_of_sets}")
    if number_of_sets == 0:
        print(f"\nNo sets were found - there may be no sets, or your configuration is incorrect.")
    try:
        jsoned_created_sets = FileSetArrayCodec.encode_to_json((starting_dir,created_sets))
        with open(get_main_path() / StoragePaths.found_sets_file_name, "w") as file:
            json.dump(jsoned_created_sets, file)
    except OSError as err:
        Logger().add_line(f"There was an error trying to save the sets file: {err}", LogLevel.ERROR)
        return
