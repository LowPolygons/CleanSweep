from typing import Optional
from cleansweep.types.json import Json
from pathlib import Path
import json

class JsonIOManager:
    @staticmethod
    def read_json_file(path: Path) -> Optional[Json]:
        pass

    @staticmethod
    def write_json_to_file(path: Path, json_obj: Json) -> bool:
        return True

