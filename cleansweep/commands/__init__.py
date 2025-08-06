from typing import Iterable
from .list import ListCommand
from .purge import PurgeCommand
from .scan import ScanCommand
from .settings import SettingsCommand
from .setup import SetupCommand
from cleansweep.interfaces.command import CommandInterface

COMMANDS: Iterable[type[CommandInterface]] = [
    ListCommand,
    ScanCommand,
    PurgeCommand,
    SettingsCommand,
    SetupCommand
]
