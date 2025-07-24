from cleansweep.types.json import Json

from dataclasses import dataclass

regex = str

@dataclass
class UserSettings:
    # Black list files get ignored 
    ignore_files_with_extension: list[str]
    ignore_file_names_containing: list[regex]
    ignore_files_smaller_than: int 
    ignore_files_whos_directory_contains: list[regex]
    # White listed files get stored in a file if they aren't deleted
    prioritise_files_with_extension: list[str]
    prioritise_file_names_containing: list[regex]
    prioritise_files_larger_than: int
    prioritise_files_whos_directory_contains: list[regex]
