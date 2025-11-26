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
  
    # If the name starts with, or contains a set of strings, it should be black/whitelisted
    def filter_name(self, 
                    to_delete_substrings: list[str], 
                    to_keep_substrings: list[str], 
                    to_delete_starts_with: list[str], 
                    to_keep_starts_with: list[str]) -> FilterCodes:
        # Black list first
        for starts_with in to_keep_starts_with:
            if self.__stats.name.find(starts_with) == STARTS_WITH_SUBSTR:
                return FilterCodes.ToKeep       
        for name_contains in to_keep_substrings:
            if self.__stats.name.find(name_contains) is not SUBSTR_NOT_FOUND:
                return FilterCodes.ToKeep

        # Then Whitelist
        for starts_with in to_delete_starts_with:
            if self.__stats.name.find(starts_with) == STARTS_WITH_SUBSTR:
                return FilterCodes.ToDelete
        for name_contains in to_delete_substrings:
            if self.__stats.name.find(name_contains) is not SUBSTR_NOT_FOUND:
                return FilterCodes.ToDelete

        return FilterCodes.NotSpecial

    # Filters the path of the item for a set of substrings to see if it should be black/whitelisted 
    def filter_path(self, 
                    to_delete: list[str], 
                    to_keep: list[str]) -> FilterCodes:
        # Prioritise the black list 
        for path_substr in to_keep:
            if str(self.__path).find(path_substr) is not SUBSTR_NOT_FOUND:
                return FilterCodes.ToKeep
        # Then filter the white list 
        for path_substr in to_delete:
            if str(self.__path).find(path_substr) is not SUBSTR_NOT_FOUND:
                return FilterCodes.ToDelete
        
        return FilterCodes.NotSpecial

    # Filters the size of the item, and seeing if it should be black/whitelisted
    def filter_size(self, 
                    to_delete: int,
                    to_keep_lower_than: int, 
                    to_keep_higher_than: int) -> FilterCodes:
        # Prioritise black list 
        if self.__stats.size < to_keep_lower_than or \
           self.__stats.size > to_keep_higher_than:
            return FilterCodes.ToKeep
        
        if self.__stats.size > to_delete:
            return FilterCodes.ToDelete

        return FilterCodes.NotSpecial

    # Filters the extension to see if it matches any in the provided black/whitelist
    def filter_extension(self, 
                         to_delete: list[str], 
                         to_keep: list[str]) -> FilterCodes:
        for extension in to_keep:
            if self.__stats.extension == f".{extension}":
                return FilterCodes.ToKeep
        
        for extension in to_delete:
            if self.__stats.extension == f".{extension}":
                return FilterCodes.ToDelete

        return FilterCodes.NotSpecial
    
    def was_last_modified_before(self, date_cutoff: date, consider_last_accessed: bool):
        if consider_last_accessed:
            return self.__stats.last_modified <= date_cutoff and \
                   self.__stats.last_accessed <=date_cutoff
        else:
            return self.__stats.last_modified <= date_cutoff
