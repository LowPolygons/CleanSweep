from datetime import date
from pathlib import Path

from cleansweep.containers.file_statistics import FileStatistics
from cleansweep.types.json import Json
from cleansweep.globals.filter_codes import FilterCodes

from . import DATE_FORMAT, SUBSTR_NOT_FOUND, STARTS_WITH_SUBSTR


# Given a path the file item class will gather data about the path to allow handling of whitelist/blacklist
class FileItem:
    def __init__(self, item_path: Path) -> None:
        self.__path: Path = item_path
        self.__stats: FileStatistics = FileStatistics()

    # Bool value determines if the provided path is an actual file
    def stat_calculate(self) -> bool:
        return self.__stats.format_self(self.__path)
    
    def set_statistics(self, precalculated_stats: FileStatistics) -> None:
        self.__stats = precalculated_stats
        
    def get_path(self) -> Path:
        return self.__path
 
    def get_name(self) -> str:
        return self.__stats.name

    def get_size(self) -> int:
        return self.__stats.size

    def get_extension(self) -> str:
        return self.__stats.extension

    def was_last_modified_before(self, date_cutoff: date, consider_last_accessed: bool) -> bool:
        if consider_last_accessed:
            return self.__stats.last_modified <= date_cutoff and \
                   self.__stats.last_accessed <=date_cutoff
        else:
            return self.__stats.last_modified <= date_cutoff

