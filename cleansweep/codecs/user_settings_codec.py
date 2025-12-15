from datetime import datetime
from cleansweep.globals.log_levels import LogLevel
from cleansweep.interfaces.json_codec import JsonCodecInterface
from cleansweep.containers.user_settings import UserSettings
from cleansweep.systems.logger_system import Logger
from cleansweep.types.json import Json
from cleansweep.systems.json_reader import JsonReader
from typing import cast, Optional

# class UserSettings:
#     # THE FLAGS NEEDED FOR A FILE 
#     flag_date_cutoff: date 
#
#     # To Keep files get ignored 
#     ignore_files_with_extension: list[str]
#     ignore_file_names_containing: list[str]
#     ignore_files_whos_directory_contains: list[str]
#     ignore_file_names_starting_with: list[str]
#     ignore_files_smaller_than: int
#     ignore_files_larger_than: int
#
#     # To Delete files get stored in a file
#     prioritise_files_with_extension: list[str]
#     prioritise_file_names_containing: list[str]
#     prioritise_file_names_starting_with: list[str]
#     prioritise_files_whos_directory_contains: list[str]
#     prioritise_files_larger_than: int
#
#     # Filter options
#     consider_access_date_when_filtering: bool
#
#     # Set rules
#     set_may_have_extension: list[str]
#     set_file_name_may_contain: list[str]
#
#     # Override Categories
#     override_files_with_extension: list[str]
#     override_file_names_containing: list[str]
#     override_file_names_starting_with: list[str]
#     override_files_whos_directory_contains: list[str]
#     override_files_larger_than: int

class UserSettingsCodec(JsonCodecInterface[UserSettings]):
    @staticmethod
    def encode_to_json(obj: UserSettings) -> Json:
        data: Json = {
            "flag_date_cutoff" : obj.flag_date_cutoff.strftime("%Y-%m-%d"),
            "consider_access_date" : obj.consider_access_date_when_filtering,
            "to_keep_files" : {
                "extension_is" : cast(list[Json], obj.ignore_files_with_extension),
                "name_contains" : cast(list[Json], obj.ignore_file_names_containing),
                "name_starts_with" : cast(list[Json], obj.ignore_file_names_starting_with),
                "directory_contains" : cast(list[Json], obj.ignore_files_whos_directory_contains),
                "smaller_than" : obj.ignore_files_smaller_than,
                "larger_than" : obj.ignore_files_larger_than
            },
            "to_delete_files" : {
                "extension_is" : cast(list[Json], obj.prioritise_files_with_extension),
                "name_contains" : cast(list[Json], obj.prioritise_file_names_containing),
                "name_starts_with" : cast(list[Json], obj.prioritise_file_names_starting_with),
                "directory_contains" : cast(list[Json], obj.prioritise_files_whos_directory_contains),
                "larger_than" : obj.prioritise_files_larger_than
            },
            "override_files" : {
                "extension_is" : cast(list[Json], obj.override_files_with_extension),
                "name_contains" : cast(list[Json], obj.override_file_names_containing),
                "name_starts_with" : cast(list[Json], obj.override_file_names_starting_with),
                "directory_contains" : cast(list[Json], obj.override_files_whos_directory_contains),
                "larger_than" : obj.override_files_larger_than
            },
            "maybe_set_files" : {
                "extension_is" : cast(list[Json], obj.set_may_have_extension),
                "name_contains" : cast(list[Json], obj.set_file_name_may_contain)
            } 
        }
        return data

    @staticmethod
    def create_from_json(obj: Json) -> Optional[UserSettings]:
        try:
            validated_obj: dict[str, Json] = JsonReader.extract_json_dict(obj)
            to_keep: dict[str, Json] = JsonReader.extract_json_dict(validated_obj["to_keep_files"])
            to_delete: dict[str, Json] = JsonReader.extract_json_dict(validated_obj["to_delete_files"])
            set_files: dict[str, Json] = JsonReader.extract_json_dict(validated_obj["maybe_set_files"])
            override_files: dict[str, Json] = JsonReader.extract_json_dict(validated_obj["override_files"])

            unformatted_flag_date_cutoff = JsonReader.extract_str(validated_obj["flag_date_cutoff"])
            ignore_files_with_extension = JsonReader.extract_list_of_type(to_keep["extension_is"], str)
            ignore_file_names_containing = JsonReader.extract_list_of_type(to_keep["name_contains"], str)
            ignore_file_names_starting_with = JsonReader.extract_list_of_type(to_keep["name_starts_with"], str)
            ignore_files_whos_directory_contains = JsonReader.extract_list_of_type(to_keep["directory_contains"], str)
            ignore_files_smaller_than = JsonReader.extract_int(to_keep["smaller_than"])
            ignore_files_larger_than = JsonReader.extract_int(to_keep["larger_than"])

            prioritise_files_with_extension = JsonReader.extract_list_of_type(to_delete["extension_is"], str)
            prioritise_file_names_containing = JsonReader.extract_list_of_type(to_delete["name_contains"], str)
            prioritise_file_names_starting_with = JsonReader.extract_list_of_type(to_delete["name_starts_with"], str)
            prioritise_files_whos_directory_contains = JsonReader.extract_list_of_type(to_delete["directory_contains"], str)
            prioritise_files_larger_than = JsonReader.extract_int(to_delete["larger_than"])

            override_files_with_extension = JsonReader.extract_list_of_type(override_files["extension_is"], str)
            override_file_names_containing = JsonReader.extract_list_of_type(override_files["name_contains"], str)
            override_file_names_starting_with = JsonReader.extract_list_of_type(override_files["name_starts_with"], str)
            override_files_whos_directory_contains = JsonReader.extract_list_of_type(override_files["directory_contains"], str)
            override_files_larger_than = JsonReader.extract_int(override_files["larger_than"])            

            flag_date_cutoff = datetime.strptime(unformatted_flag_date_cutoff, "%Y-%m-%d").date()

            consider_access_date_when_filtering = JsonReader.extract_bool(validated_obj["consider_access_date"])

            set_may_have_extension = JsonReader.extract_list_of_type(set_files["extension_is"], str)
            set_file_name_may_contain = JsonReader.extract_list_of_type(set_files["name_contains"], str)

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
                consider_access_date_when_filtering,
                set_may_have_extension,
                set_file_name_may_contain,
                override_files_with_extension,
                override_file_names_containing,
                override_file_names_starting_with,
                override_files_whos_directory_contains,
                override_files_larger_than
            )
        except Exception as err:
            Logger().add_line(f"Error trying to index some values in the User Settings Json: {err}", LogLevel.ERROR)
            return None

