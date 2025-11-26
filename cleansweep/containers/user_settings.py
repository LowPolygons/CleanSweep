from dataclasses import dataclass
from datetime import date 

@dataclass
class UserSettings:
    # THE FLAGS NEEDED FOR A FILE 
    flag_date_cutoff: date 

    # To Keep files get ignored 
    ignore_files_with_extension: list[str]
    ignore_file_names_containing: list[str]
    ignore_files_whos_directory_contains: list[str]
    ignore_file_names_starting_with: list[str]
    ignore_files_smaller_than: int
    ignore_files_larger_than: int

    # To Delete files get stored in a file
    prioritise_files_with_extension: list[str]
    prioritise_file_names_containing: list[str]
    prioritise_file_names_starting_with: list[str]
    prioritise_files_whos_directory_contains: list[str]
    prioritise_files_larger_than: int

    # Filter options
    consider_access_date_when_filtering: bool
