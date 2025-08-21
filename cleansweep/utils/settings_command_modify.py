from dataclasses import astuple, fields
from datetime import date, datetime, timedelta
from enum import Enum
import json
from typing import Optional, Union
from cleansweep.codecs.user_settings_codec import UserSettingsCodec
from cleansweep.containers.user_settings import UserSettings
from cleansweep.globals.log_levels import LogLevel
from cleansweep.globals.storage_paths import StoragePaths
from cleansweep.globals.user_setting_variant import SettingsVariant
from cleansweep.systems.logger_system import Logger
from cleansweep.types.json import Json
from cleansweep.types.user_settings_union import UserSettingsUnion
from cleansweep.utils.get_main_path import get_main_path
from cleansweep.utils.get_user_settings import get_user_settings
from cleansweep.utils.settings_command_display import SettingsCommandDisplay

PROMPT_SESSION_END = "cleansweep finish"
PROMPT_DISPLAY_SETTINGS = "cleansweep display"
NUMBER_OF_USER_SETTINGS = 12

LIST_MODIFY_OPTIONS = [
    "add",
    "delete",
    "reset"
]

class ModifyOption(Enum):
    Add = 0
    Delete = 1
    Reset = 2

class InteractiveEnvironment():
    @classmethod
    def get_number(cls, user_settings: UserSettings) -> Optional[int]:
        number: Optional[int] = None
        while not number:
            try:
                choice = input("Choice => ")

                if choice == PROMPT_SESSION_END:
                    return None
                if choice == PROMPT_DISPLAY_SETTINGS:
                    SettingsCommandDisplay(SettingsVariant.Regular, user_settings)
                    continue

                number = int(choice)
            except (ValueError, TypeError):
                number = None
                continue
        return number

    @classmethod 
    def number_in_range_non_inclusive_upper(cls, lower: int, upper: int, user_settings: UserSettings) -> Optional[int]:
        numerical_choice: int = lower - 1
        while numerical_choice < lower:
            try:
                choice = input("Choice => ")

                if choice == PROMPT_SESSION_END:
                    return None
                if choice == PROMPT_DISPLAY_SETTINGS:
                    SettingsCommandDisplay(SettingsVariant.Regular, user_settings)
                    continue

                numerical_choice = int(choice)

                valid: bool = lower <= numerical_choice < upper

                if not valid:
                    print("Not in correct range")
                    numerical_choice = lower - 1
            except (ValueError, TypeError):
                continue
        return numerical_choice

    @classmethod
    def pick_settings_option_to_modify(cls, user_settings: UserSettings) -> Optional[int]:
        print(f"Choose a setting to modify (1-{NUMBER_OF_USER_SETTINGS})")
        
        choice = cls.number_in_range_non_inclusive_upper(1, NUMBER_OF_USER_SETTINGS+1, user_settings)

        return choice
    
    @classmethod
    def pick_modify_option(cls, user_settings: UserSettings) -> Optional[ModifyOption]:
        print("Choose from (case and space sensitive):   add   delete   reset")
        while True:
            choice = input(" => ")

            if choice == PROMPT_SESSION_END:
                return None
            if choice == PROMPT_DISPLAY_SETTINGS:
                SettingsCommandDisplay(SettingsVariant.Regular, user_settings)
                continue
            if choice not in LIST_MODIFY_OPTIONS:
                continue

            if choice == "add":
                return ModifyOption.Add
            elif choice == "delete":
                return ModifyOption.Delete
            elif choice == "reset":
                return ModifyOption.Reset
            # Should never reach here
            return None

    @classmethod
    def add_to_list(cls, list_obj: list, user_settings: UserSettings) -> Optional[list]:
        addition = input(" - new value => ")
        if addition == PROMPT_SESSION_END:
            return None
        if addition == PROMPT_DISPLAY_SETTINGS:
            SettingsCommandDisplay(SettingsVariant.Regular, user_settings)
            return None

        if not addition in list_obj:
            list_obj.append(addition)
        return list_obj

    @classmethod
    def remove_from_list(cls, list_obj: list, user_settings: UserSettings) -> Optional[list]:
        if len(list_obj) == 0:
            print("List already empty!")
            return list_obj

        print(f"- Please choose an item to remove (1-{len(list_obj)})")
        remove_index = cls.number_in_range_non_inclusive_upper(1, len(list_obj)+1, user_settings)

        if not remove_index:
            return None
        list_obj.pop(remove_index-1)

        return list_obj

    @classmethod
    def reset_list(cls) -> list:
        return []

    @classmethod
    def update_number_value(cls, old_num: int, user_settings: UserSettings) -> Optional[int]:
        print(f"- Current value: {old_num}")
        new_value = cls.get_number(user_settings)

        return new_value

    @classmethod
    def update_date_value(cls, old_date: date, user_settings: UserSettings) -> Optional[date]:
        print(f"Todays date: {datetime.today()}")
        print(f"Number of days different: {(datetime.today().date() - old_date).days}")

        new_num_days = cls.get_number(user_settings)

        if not new_num_days:
            return None
        return (datetime.today() - timedelta(days=new_num_days)).date()

    @classmethod
    def update_parameter(cls, value: UserSettingsUnion, user_settings: UserSettings) -> Optional[UserSettingsUnion]:
        if isinstance(value, int):
            return cls.update_number_value(value, user_settings)
        if isinstance(value, date):
            return cls.update_date_value(value, user_settings)

        # It is a list so confirm which modify status
        maybe_modify_option = cls.pick_modify_option(user_settings)    
        if not maybe_modify_option:
            return None

        if maybe_modify_option == ModifyOption.Add:
            return cls.add_to_list(value, user_settings)
        elif maybe_modify_option == ModifyOption.Delete:
            return cls.remove_from_list(value, user_settings)
        elif maybe_modify_option == ModifyOption.Reset:
            return cls.reset_list()
        # Should never be here
        return None
    
    @classmethod
    def session_end(cls, user_settings: UserSettings) -> None:
        # Writes user settings
        settings_json: Json = UserSettingsCodec.encode_to_json(user_settings)
        try:
            with open(get_main_path() / StoragePaths.user_settings_file_name, "w") as file:
                json.dump(settings_json, file)
        except OSError as err:
            Logger().add_line(f"There was an error trying to write the default user settings to file, err {err}", LogLevel.ERROR)

    @classmethod
    def interactive_environment(cls, session_cutoff: int) -> None:
        maybe_user_settings = get_user_settings(SettingsVariant.Regular)

        if not maybe_user_settings:
            print("Couldn't load the user settings - has the setup command run?")
            return

        print("\nEntering interactive environment:\n")

        SettingsCommandDisplay(SettingsVariant.Regular, maybe_user_settings)

        print("[Info]\n - At any point, enter 'cleansweep finish' to end the session")
        print(" - At any point, enter 'cleansweep display' to display the current user settings")
        print(" - When prompted to choose an option, pick the number based on the order display, starting from zero")
        print(" - The changes made will automatically save upon session close\n")

        while True:
            print("=== === === === === ===")
            # Pick user option and validate it wasnt a session end
            maybe_choice = cls.pick_settings_option_to_modify(maybe_user_settings)
            if not maybe_choice:
                return cls.session_end(maybe_user_settings)

            # 1 indexed
            maybe_choice -= 1
            
            parameter_name = fields(maybe_user_settings)[maybe_choice].name
            parameter_value = astuple(maybe_user_settings)[maybe_choice]

            print(f"Target Parameter: {parameter_name} = {parameter_value}")

            maybe_parameter = cls.update_parameter(parameter_value, maybe_user_settings)
            if not maybe_parameter:
                return cls.session_end(maybe_user_settings)

            setattr(maybe_user_settings, parameter_name, maybe_parameter)
            
            user_input: str = input("Press enter to continue..")

            if user_input == PROMPT_SESSION_END:
                return cls.session_end(maybe_user_settings)
            if user_input == PROMPT_DISPLAY_SETTINGS:
                SettingsCommandDisplay(SettingsVariant.Regular, maybe_user_settings)
