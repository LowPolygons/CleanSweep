import os
import json
from typing import cast, Optional
from cleansweep.globals.storage_paths import StoragePaths
from cleansweep.types.json import Json
from cleansweep.utils.get_main_path import get_main_path
from cleansweep.codecs.file_set_array_codec import FileSetArrayCodec
from cleansweep.codecs.file_array_codec import FileArrayCodec
from cleansweep.systems.logger_system import Logger, LogLevel
from cleansweep.globals.set_management_strategy import SetManagementStrategy, display_management_strategies, NUMBER_OF_SET_MANAGEMENT_OPTIONS
from cleansweep.containers.set_management_pair import SetAndManagementPair
from cleansweep.containers.file_item import FileItem
from pathlib import Path

PROMPT_SESSION_END = "cleansweep finish"
PROMPT_DISPLAY_SETTINGS = "cleansweep display"

def print_sets(sets):
    # Due to rules of sets a set will never be empty
    for i in range(0, len(sets)):
        chosen_method = sets[i].management.value[0] if sets[i].management != SetManagementStrategy.Null else "None"
        print(f"{i} - {chosen_method} - {sets[i].set[0]}..")


def print_single_set(single_set: SetAndManagementPair):
    chosen_method = single_set.management.value[0] if single_set.management != SetManagementStrategy.Null else "None"
    print(f"\n- Current strategy: {chosen_method}\n")
    if len(single_set.set) <= 20:
        for item in single_set.set:
            print(f"- {item}")
    else:
        for i in range(3):
            print(f"- {single_set.set[i]}")
        print(f"- ... ({len(single_set.set)-6} items hidden)")
        for item in single_set.set[-3:]:
            print(f"- {item}")
    print("\n")

def get_number(positive_only: bool = False) -> Optional[int]:
    number: Optional[int] = None
    while not number:
        try:
            choice = input("Choice => ")

            if choice == PROMPT_SESSION_END:
                return None
            if choice == PROMPT_DISPLAY_SETTINGS:
                display_management_strategies() 
                continue

            number = int(choice)

            if positive_only and number < 0:
                continue
        except (ValueError, TypeError):
            number = None
            continue
    return number

def number_in_range_non_inclusive_upper(lower: int, upper: int) -> Optional[int]:
    numerical_choice: int = lower - 1
    while numerical_choice < lower:
        try:
            choice = input("Choice => ")

            if choice == PROMPT_SESSION_END:
                return None
            if choice == PROMPT_DISPLAY_SETTINGS:
                display_management_strategies()
                continue

            numerical_choice = int(choice)

            valid: bool = lower <= numerical_choice < upper

            if not valid:
                print("Not in correct range")
                numerical_choice = lower - 1
        except (ValueError, TypeError):
            continue
    return numerical_choice

def choose_management_strategy() -> Optional[tuple[SetManagementStrategy, int]]:
    print("Please pick one of the options below for your management strategy:")
    display_management_strategies()

    choice = number_in_range_non_inclusive_upper(0, NUMBER_OF_SET_MANAGEMENT_OPTIONS)

    if choice == None:
        return None
    match choice:
        case 0:
            return (SetManagementStrategy.FirstAndLast, -1)
        case 1:
            return (SetManagementStrategy.First, -1)
        case 2:
            return (SetManagementStrategy.Last, -1)
        case 3:
            print("Choose what the series should be for saving every N files:")
            other_choice = get_number()
            if other_choice == None:
                return None
            return (SetManagementStrategy.EveryN, other_choice)
        case 4:
            print("Choose how many files to save which will be evenly distributed:")
            other_choice = get_number()
            if other_choice == None:
                return None
            return (SetManagementStrategy.NEvenlySpaced, other_choice)

def finalise_changes(sets: list[SetAndManagementPair]):
    # First, filter any of the files out of the original ToKeep/ToDelete list. If they fail to open, create new 
    # Does the file exist?
    if not os.path.exists(get_main_path() / StoragePaths.to_keep_file_name):
        print("The file containing files to keep doesn't exist - have you run the setup command?")
        return
    if not os.path.exists(get_main_path() / StoragePaths.to_delete_file_name):
        print("The file containing files to delete doesn't exist - have you run the setup command?")
   
    to_delete_array = []
    to_keep_array = []
    # Load the JSON version of the To Keep List
    try:
        unsanitised_json: Json = []

        with open(get_main_path() / StoragePaths.to_keep_file_name, "r") as file:
            unsanitised_json = cast(Json, json.load(file))
        
        maybe_file_array = FileArrayCodec.create_from_json(unsanitised_json)

        if not maybe_file_array:
            print("To-Keep list not found, creating new one.")
        else:
            to_keep_array = maybe_file_array
    except Exception as err:
        print(f"Failed to open the to-keep files - {err}")
        return
    # Now the JSON version of the To Delete list
    try:
        unsanitised_json: Json = []

        with open(get_main_path() / StoragePaths.to_delete_file_name, "r") as file:
            unsanitised_json = cast(Json, json.load(file))
        
        maybe_file_array = FileArrayCodec.create_from_json(unsanitised_json)

        if not maybe_file_array:
            print("To-Delete list not found, creating new one.")
        else:
            to_delete_array = maybe_file_array
    except Exception as err:
        print(f"Failed to open the to-delete files - {err}")
        return
    
    # Now eliminate any instances found if there
    if to_keep_array != []:
        paths_to_remove = {path for set_obj in sets for path in set_obj.set}

        to_keep_array = [
            obj for obj in to_keep_array
            if str(obj.get_path()) not in paths_to_remove
        ]
    if to_delete_array != []:
        paths_to_remove = {path for set_obj in sets for path in set_obj.set}

        to_delete_array = [
            obj for obj in to_delete_array
            if str(obj.get_path()) not in paths_to_remove
        ]
    # Then based on the rules, add the files to the existing sets
    for set_obj in sets:
        match set_obj.management:
            case SetManagementStrategy.FirstAndLast:
                # Keep first and last
                to_keep_array.append(FileItem(Path(set_obj.set.pop(0))))
                to_keep_array.append(FileItem(Path(set_obj.set.pop(-1))))
            case SetManagementStrategy.First:
                # Keep first
                to_keep_array.append(FileItem(Path(set_obj.set.pop(0))))
            case SetManagementStrategy.Last:
                # Keep last
                to_keep_array.append(FileItem(Path(set_obj.set.pop(-1))))
            case SetManagementStrategy.EveryN:
                # Iterate down so that the shortening of the list doesnt affect anything
                for file_index in range(len(set_obj.set) -1, -1, -1 * set_obj.management_N):
                    to_keep_array.append(FileItem(Path(set_obj.set.pop(file_index))))
            case SetManagementStrategy.NEvenlySpaced:
                if set_obj.management_N == 1:
                    to_keep_array.append(FileItem(Path(set_obj.set.pop(-1))))
                else:
                    to_keep_array.append(FileItem(Path(set_obj.set.pop(0))))
                    to_keep_array.append(FileItem(Path(set_obj.set.pop(-1))))
                if set_obj.management_N > 2:
                    set_obj.management_N -= 2
                    increment: int = len(set_obj.set) // set_obj.management_N

                    # To not cause errors, pop after
                    for increment_num in range(0, set_obj.management_N):
                        to_keep_array.append(FileItem(Path(set_obj.set[increment_num * increment])))
                    for increment_num in range(0, set_obj.management_N):
                        set_obj.set.pop(increment_num * increment)
            case SetManagementStrategy.Null:
                # Keep first and last if not specified
                to_keep_array.append(FileItem(Path(set_obj.set.pop(0))))
                to_keep_array.append(FileItem(Path(set_obj.set.pop(-1))))
        # Delete rest 
        for file in set_obj.set:
            to_delete_array.append(FileItem(Path(file)))

    # Save them
    jsoned_to_keep = FileArrayCodec.encode_to_json(to_keep_array)
    jsoned_to_delete = FileArrayCodec.encode_to_json(to_delete_array)

    try:
        with open(get_main_path() / StoragePaths.to_keep_file_name, "w") as file:
            json.dump(jsoned_to_keep, file)
        with open(get_main_path() / StoragePaths.to_delete_file_name, "w") as file:
            json.dump(jsoned_to_delete, file)
    except OSError as err:
        Logger().add_line(f"There was an error trying to save the ToKeep/ToDelete listed files: {err}", LogLevel.ERROR)
        return

def manage_sets():
    if not os.path.exists(get_main_path() / StoragePaths.found_sets_file_name):
        print("Sets File not found. Consider running the set-scan command, or ensure you have set cleansweep up.")
        return

    sets: list[SetAndManagementPair] = []
    
    try:
        unsanitised: Json

        with open(get_main_path() / StoragePaths.found_sets_file_name, "r") as file:
            unsanitised = cast(Json, json.load(file))

        maybe_sets = FileSetArrayCodec.create_from_json(unsanitised)

        if not maybe_sets:
            return

        for _set in maybe_sets:
            sets.append(SetAndManagementPair(_set, SetManagementStrategy.Null, None))
    
    except Exception as err:
        Logger().add_line(f"There was an error trying to open the sets list: {err}", LogLevel.ERROR)


    print(f"[Info]\n- You are now managing your sets. Please go through the sets and choose how the filtering shall complete\n")
    print(f"- If you do not specify how a set should be handled, it will default to saving only the First and Last\n")
    print(f"- Any files which were previously flagged by the default scan have been removed out from the regular ToKeep/ToDelete lists\n")
    print(f"- As part of this process, any files contained within any of these sets which were flagged in the regular scan will be removed from the regular lists.\n") 
    while True:
        print("=== === === === === ===")
        print(" - At any point, enter 'cleansweep finish' to end the session")
        print(" - At any point, enter 'cleansweep display' to display the management options\n")
        
        print_sets(sets)

        print("\n Please choose which set to manage. Optionally, to set a value for all, enter -1")
        choice = number_in_range_non_inclusive_upper(-1, len(sets))
        if choice == None:
            # Terminate session CORRECTLY by writing changes
            # TODO: consider adding 'cleansweep cancel'
            return finalise_changes(sets)
        if choice != -1:
            print_single_set(sets[choice])
        else:
            print("Setting management style for all sets..")
        management_choice = choose_management_strategy()
        
        if management_choice == None:
            return finalise_changes(sets)
        
        if choice == -1:
            for _set in sets:
                _set.management = management_choice[0]
                _set.management_N = None if management_choice[1] == -1 else management_choice[1]
        
        sets[choice].management = management_choice[0]
        sets[choice].management_N = None if management_choice[1] == -1 else management_choice[1]
