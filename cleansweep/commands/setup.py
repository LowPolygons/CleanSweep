from cleansweep.globals.log_levels import LogLevel
from cleansweep.interfaces.command import CommandInterface 
from cleansweep.systems.logger_system import Logger

from argparse import Namespace, _SubParsersAction

from cleansweep.utils.setup_command import setup

class SetupCommand(CommandInterface):
    @staticmethod
    def command(args: Namespace) -> None:
        Logger().add_line("Running the setup command", LogLevel.INFO)
        setup()

    @classmethod
    def register_subparser(cls, subparsers: _SubParsersAction) -> None:
        list_parser = subparsers.add_parser('setup', help="TODO")
        list_parser.set_defaults(func=cls.command)
