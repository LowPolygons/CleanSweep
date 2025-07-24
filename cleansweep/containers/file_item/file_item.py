from pathlib import Path
from typing import Dict

from cleansweep.types.json import Json

# Given a path the file item class will gather data about the path to allow handling of whitelist/blacklist
class FileItem:
    def __init__(self, item_path: Path) -> None:
        self.__path = item_path
        self 
    def get_file_size(self) -> int:
        return 0
