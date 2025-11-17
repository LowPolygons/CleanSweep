from datetime import datetime
from cleansweep.globals.log_levels import LogLevel
from cleansweep.interfaces.json_codec import JsonCodecInterface
from cleansweep.containers.user_settings import UserSettings
from cleansweep.systems.logger_system import Logger
from cleansweep.types.json import Json
from cleansweep.systems.json_reader import JsonReader
from typing import cast, Optional

class UserSettingsCodec(JsonCodecInterface[UserSettings]):
    @staticmethod
    def encode_to_json(obj: UserSettings) -> Json:
        data: Json = {
            "flag_date_cutoff" : obj.flag_date_cutoff.strftime("%Y-%m-%d"),
            "consider_access_date" : obj.consider_access_date_when_filtering,
            "blacklist_files" : {
                "extension_is" : cast(list[Json], obj.ignore_files_with_extension),
                "name_contains" : cast(list[Json], obj.ignore_file_names_containing),
                "name_starts_with" : cast(list[Json], obj.ignore_file_names_starting_with),
                "directory_contains" : cast(list[Json], obj.ignore_files_whos_directory_contains),
                "smaller_than" : obj.ignore_files_smaller_than,
                "larger_than" : obj.ignore_files_larger_than
            },
            "whitelist_files" : {
                "extension_is" : cast(list[Json], obj.prioritise_files_with_extension),
                "name_contains" : cast(list[Json], obj.prioritise_file_names_containing),
                "name_starts_with" : cast(list[Json], obj.prioritise_file_names_starting_with),
                "directory_contains" : cast(list[Json], obj.prioritise_files_whos_directory_contains),
                "larger_than" : obj.prioritise_files_larger_than
            }
        }
        return data

    @staticmethod
    def create_from_json(obj: Json) -> Optional[UserSettings]:
        try:
            validated_obj: dict[str, Json] = JsonReader.extract_json_dict(obj)
            blacklisted: dict[str, Json] = JsonReader.extract_json_dict(validated_obj["blacklist_files"])
            whitelisted: dict[str, Json] = JsonReader.extract_json_dict(validated_obj["whitelist_files"])

            unformatted_flag_date_cutoff = JsonReader.extract_str(validated_obj["flag_date_cutoff"])
            ignore_files_with_extension = JsonReader.extract_list_of_type(blacklisted["extension_is"], str)
            ignore_file_names_containing = JsonReader.extract_list_of_type(blacklisted["name_contains"], str)
            ignore_file_names_starting_with = JsonReader.extract_list_of_type(blacklisted["name_starts_with"], str)
            ignore_files_whos_directory_contains = JsonReader.extract_list_of_type(blacklisted["directory_contains"], str)
            ignore_files_smaller_than = JsonReader.extract_int(blacklisted["smaller_than"])
            ignore_files_larger_than = JsonReader.extract_int(blacklisted["larger_than"])

            prioritise_files_with_extension = JsonReader.extract_list_of_type(whitelisted["extension_is"], str)
            prioritise_file_names_containing = JsonReader.extract_list_of_type(whitelisted["name_contains"], str)
            prioritise_file_names_starting_with = JsonReader.extract_list_of_type(whitelisted["name_starts_with"], str)
            prioritise_files_whos_directory_contains = JsonReader.extract_list_of_type(whitelisted["directory_contains"], str)
            prioritise_files_larger_than = JsonReader.extract_int(whitelisted["larger_than"])

            flag_date_cutoff = datetime.strptime(unformatted_flag_date_cutoff, "%Y-%m-%d").date()

            consider_access_date_when_filtering = JsonReader.extract_bool(validated_obj["consider_access_date"])

            for extension in ignore_files_with_extension:
                extension = extension[1:] if extension.startswith(".") else extension
            for extension in prioritise_files_with_extension:
                extension = extension[1:] if extension.startswith(".") else extension

            return UserSettings(
                flag_date_cutoff, 
                ignore_files_with_extension, 
                ignore_file_names_containing, 
                ignore_files_whos_directory_contains,
                ignore_file_names_starting_with,
                ignore_files_smaller_than,
                ignore_files_larger_than,
                prioritise_files_with_extension,
                prioritise_file_names_containing,
                prioritise_file_names_starting_with,
                prioritise_files_whos_directory_contains,
                prioritise_files_larger_than,
                consider_access_date_when_filtering
            )

        except Exception as err:
            Logger().add_line(f"Error trying to index some values in the User Settings Json: {err}", LogLevel.ERROR)
            return None

