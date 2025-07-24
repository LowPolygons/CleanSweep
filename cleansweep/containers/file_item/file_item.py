from pathlib import Path

from cleansweep.containers.file_statistics.statistics import FileStatistics
from cleansweep.types.json import Json

DATE_FORMAT = "%Y-%m-%d"

# Given a path the file item class will gather data about the path to allow handling of whitelist/blacklist
class FileItem:
    def __init__(self, item_path: Path) -> None:
        self.__path: Path = item_path
        self.__stats: FileStatistics

    def stats_as_json(self) -> Json:
        json_data: Json = {
            "name" : self.__stats.name,
            "path" : str(self.__path),
            "size" : self.__stats.size,
            "extension" : self.__stats.extension,
            "date_created" : self.__stats.date_created.strftime(DATE_FORMAT),
            "last_modified" : self.__stats.last_modified.strftime(DATE_FORMAT)
        }
        return json_data
