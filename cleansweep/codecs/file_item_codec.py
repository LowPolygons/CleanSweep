from datetime import datetime
from pathlib import Path
from typing import Optional
from cleansweep.globals.log_levels import LogLevel
from cleansweep.interfaces.json_codec import JsonCodecInterface
from cleansweep.containers.file_item import FileItem
from cleansweep.systems.json_reader import JsonReader
from cleansweep.systems.logger_system import Logger
from cleansweep.types.json import Json

class FileItemCodec(JsonCodecInterface[FileItem]):
    @staticmethod
    def encode_to_json(obj: FileItem) -> Json:
        # TODO: move the stats as json to here and swap for a getter 
        return str(obj.get_path())

    @staticmethod
    def create_from_json(obj: Json) -> Optional[FileItem]:
        # NOTE: The files are stored in an array, this function is not called on that array, eather the individual items in that array
        try:
            validated_path_str: str = JsonReader.extract_str(obj)
            path: Path = Path(validated_path_str)

            item = FileItem(path)
            success = item.stat_calculate()
            
            if not success:
                raise Exception("Extracting file metadata failed - possibly doesn't exist")
            
            return item

        except Exception as err:
            Logger().add_line(f"Error trying to parse File Item json to object: {err}", LogLevel.ERROR)
            return None
