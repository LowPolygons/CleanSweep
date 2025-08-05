from typing import Iterable
from .list import ListCommand
from .command import CommandInterface

COMMANDS: Iterable[type[CommandInterface]] = [
    ListCommand
]
