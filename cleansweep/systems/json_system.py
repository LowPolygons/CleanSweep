from typing import Optional
from cleansweep.types.json import Json
from pathlib import Path
from cleansweep.systems.logger_system import Logger, LogLevel

import json

class JsonIOManager:
    @staticmethod
    def read_json_file(path: Path) -> Optional[Json]:
        try:
            with open(path, "r") as file:
                json_data: Json = json.load(file)
            return json_data

        except OSError as err:
            Logger().add_line("Error attempting to read file {} with error {}".format(path, err), LogLevel.ERROR)
            return None

    @staticmethod
    def write_json_to_file(path: Path, json_obj: Json) -> bool:
        try:
            with open(path, "w") as file:
                json.dump(json_obj, file)
            return True

        except OSError as err:
            Logger().add_line("Error attempting to write object at {} with error {}".format(path, err), LogLevel.ERROR)
            return False

