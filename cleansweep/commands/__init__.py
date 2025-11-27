from typing import Iterable
from .list import ListCommand
from .purge import PurgeCommand
from .scan import ScanCommand
from .settings import SettingsCommand
from .setup import SetupCommand
from .demolish import DemolishCommand
from .set_scan import SetScanCommand
from .manage_sets import ManageSetsCommand
from .reset import ResetCommand
from cleansweep.interfaces.command import CommandInterface

COMMANDS: Iterable[type[CommandInterface]] = [
    ListCommand,
    ScanCommand,
    PurgeCommand,
    SettingsCommand,
    SetupCommand,
    DemolishCommand,
    SetScanCommand,
    ManageSetsCommand,
    ResetCommand
]
