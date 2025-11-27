from dataclasses import dataclass
from cleansweep.globals.set_management_strategy import SetManagementStrategy
from typing import Optional

@dataclass
class SetAndManagementPair:
    set: list[str]
    management: SetManagementStrategy
    management_N: Optional[int]
    
