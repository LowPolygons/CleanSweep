from cleansweep.globals.log_levels import LogLevel
from cleansweep.interfaces.command import CommandInterface 
from cleansweep.systems.logger_system import Logger

from argparse import Namespace, _SubParsersAction

from cleansweep.utils.manage_sets_command import manage_sets

class ManageSetsCommand(CommandInterface):
    @staticmethod
    def command(args: Namespace) -> None:
        Logger().add_line("Running the manage sets command", LogLevel.INFO)
        manage_sets()

    @classmethod
    def register_subparser(cls, subparsers: _SubParsersAction) -> None:
        list_parser = subparsers.add_parser('manage-sets', help="Command used to manage any sets found by CleanSweep")
        list_parser.set_defaults(func=cls.command)
