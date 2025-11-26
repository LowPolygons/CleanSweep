
from typing import Optional
from cleansweep.codecs.file_item_codec import FileItemCodec
from cleansweep.containers.file_item import FileItem
from cleansweep.globals.log_levels import LogLevel
from cleansweep.interfaces.json_codec import JsonCodecInterface
from cleansweep.systems.json_reader import JsonReader
from cleansweep.systems.logger_system import Logger
from cleansweep.types.json import Json
from pathlib import Path

FileSetArray = list[list[str]]

class FileSetArrayCodec(JsonCodecInterface[FileSetArray]):
    @staticmethod
    def encode_to_json(obj: FileSetArray) -> Json:
        return obj

    @staticmethod
    def create_from_json(obj: Json) -> Optional[FileSetArray]:
        try:
            file_objects: list[list[str]] = []
            for a_set in obj:
                set_arr: list[str] = []
                for str_path in a_set:
                    maybe_exists = Path(str_path)

                    if not maybe_exists.exists():
                        print(f"Failed to create file at path {str_path}")
                        continue

                    set_arr.append(str_path)
                file_objects.append(set_arr)

            return file_objects
        except Exception as err:
            Logger().add_line(f"Error trying to create the list of files: {err}", LogLevel.ERROR)
            return None
