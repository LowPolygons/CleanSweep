from enum import Enum 

class FlagCodes(Enum):
    NotFlagged = 0
    Flagged = 1
    FlaggedToKeep = 2
    FlaggedToDelete = 3
    Override = 4
