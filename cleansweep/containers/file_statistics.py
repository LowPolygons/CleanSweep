from dataclasses import dataclass
from datetime import date
from pathlib import Path


# The point of the class is to provide a set of options to allow for filtering
@dataclass 
class FileStatistics:
    name: str
    size: int
    extension: str
    date_created: date
    last_modified: date
   
    def format_self(self, file: Path):
        pass
