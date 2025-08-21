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
                    white_list_substrings: list[str], 
                    black_list_substrings: list[str], 
                    white_list_starts_with: list[str], 
                    black_list_starts_with: list[str]) -> FilterCodes:
        # Black list first
        for starts_with in black_list_starts_with:
            if self.__stats.name.find(starts_with) == STARTS_WITH_SUBSTR:
                return FilterCodes.BlackListed       
        for name_contains in black_list_substrings:
            if self.__stats.name.find(name_contains) is not SUBSTR_NOT_FOUND:
                return FilterCodes.BlackListed

        # Then Whitelist
        for starts_with in white_list_starts_with:
            if self.__stats.name.find(starts_with) == STARTS_WITH_SUBSTR:
                return FilterCodes.WhiteListed
        for name_contains in white_list_substrings:
            if self.__stats.name.find(name_contains) is not SUBSTR_NOT_FOUND:
                return FilterCodes.WhiteListed

        return FilterCodes.NotSpecial

    # Filters the path of the item for a set of substrings to see if it should be black/whitelisted 
    def filter_path(self, 
                    white_list: list[str], 
                    black_list: list[str]) -> FilterCodes:
        # Prioritise the black list 
        for path_substr in black_list:
            if str(self.__path).find(path_substr) is not SUBSTR_NOT_FOUND:
                return FilterCodes.BlackListed
        # Then filter the white list 
        for path_substr in white_list:
            if str(self.__path).find(path_substr) is not SUBSTR_NOT_FOUND:
                return FilterCodes.WhiteListed
        
        return FilterCodes.NotSpecial

    # Filters the size of the item, and seeing if it should be black/whitelisted
    def filter_size(self, 
                    white_list: int,
                    black_list_lower_than: int, 
                    black_list_higher_than: int) -> FilterCodes:
        # Prioritise black list 
        if self.__stats.size < black_list_lower_than or \
           self.__stats.size > black_list_higher_than:
            return FilterCodes.BlackListed
        
        if self.__stats.size > white_list:
            return FilterCodes.WhiteListed

        return FilterCodes.NotSpecial

    # Filters the extension to see if it matches any in the provided black/whitelist
    def filter_extension(self, 
                         white_list: list[str], 
                         black_list: list[str]) -> FilterCodes:
        for extension in black_list:
            if self.__stats.extension == extension:
                return FilterCodes.BlackListed
        
        for extension in white_list:
            if self.__stats.extension == extension:
                return FilterCodes.WhiteListed

        return FilterCodes.NotSpecial
    
    def was_last_modified_before(self, date_cutoff: date):
        return self.__stats.last_modified < date_cutoff and \
               self.__stats.last_accessed < date_cutoff
