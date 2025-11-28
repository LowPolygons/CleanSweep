
from typing import Optional
from cleansweep.codecs.file_item_codec import FileItemCodec
from cleansweep.containers.file_item import FileItem
from cleansweep.globals.log_levels import LogLevel
from cleansweep.interfaces.json_codec import JsonCodecInterface
from cleansweep.systems.json_reader import JsonReader
from cleansweep.systems.logger_system import Logger
from cleansweep.types.json import Json
from pathlib import Path

from cleansweep.types.json import Json

FileSetArray = tuple[str, list[FileItem]]

class FileSetArrayCodec(JsonCodecInterface[FileSetArray]):
    @staticmethod
    def encode_to_json(obj: FileSetArray) -> Json:
        json_items: dict[str, Json] = {
            "path" : obj[0],
            "sets" : obj[1]
        }
        return json_items

    @staticmethod
    def create_from_json(obj: Json) -> Optional[FileSetArray]:
        try:
            # validated_obj: list[Json] = JsonReader.extract_list_of_type(obj, str)
            validated_obj: dict[str, Json] = JsonReader.extract_json_dict(obj)
            path_run_from = JsonReader.extract_str(validated_obj["path"])
            sets = validated_obj["sets"]

            file_objects: list[list[str]] = []
            for a_set in sets:
                set_arr: list[str] = []
                for str_path in a_set:
                    maybe_exists = Path(str_path)

                    if not maybe_exists.exists():
                        print(f"Failed to create file at path {str_path}")
                        continue

                    set_arr.append(str_path)
                file_objects.append(set_arr)

            return (path_run_from, file_objects)
        except Exception as err:
            Logger().add_line(f"Error trying to create the list of files: {err}", LogLevel.ERROR)
            return None
