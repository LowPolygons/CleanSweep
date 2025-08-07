from dataclasses import dataclass
from datetime import date 

@dataclass
class UserSettings:
    # THE FLAGS NEEDED FOR A FILE 
    flag_date_cutoff: date 

    # Black list files get ignored 
    ignore_files_with_extension: list[str]
    ignore_file_names_containing: list[str]
    ignore_files_whos_directory_contains: list[str]
    ignore_file_names_starting_with: list[str]
    ignore_files_smaller_than: int
    ignore_files_larger_than: int

    # White listed files get stored in a file if they aren't deleted
    prioritise_files_with_extension: list[str]
    prioritise_file_names_containing: list[str]
    prioritise_files_whos_directory_contains: list[str]
    prioritise_file_names_starting_with: list[str]
    prioritise_files_larger_than: int
