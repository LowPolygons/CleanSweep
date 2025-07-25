from dataclasses import dataclass
from datetime import date, datetime
from pathlib import Path
import os

# The point of the class is to provide a set of options to allow for filtering
@dataclass 
class FileStatistics:
    name: str
    size: int
    extension: str
    last_accessed: date
    last_modified: date
  
    def format_self(self, file: Path) -> bool:
        # Must be a file 
        if not os.path.exists(file) or \
            not os.path.isfile(file):
            return False    
         
        self.name = os.path.basename(file)
        self.size = os.path.getsize(file)
        self.extension = file.suffix # Empty string if none
        self.last_accessed = datetime.fromtimestamp(os.path.getatime(file))
        self.last_modified = datetime.fromtimestamp(os.path.getmtime(file))

        return True 
