
from argparse import Namespace, _SubParsersAction

from cleansweep.globals.log_levels import LogLevel
from cleansweep.interfaces.command import CommandInterface 
from cleansweep.systems.logger_system import Logger
from cleansweep.utils.set_scan_command import set_scan

class SetScanCommand(CommandInterface):
    @staticmethod
    def command(args: Namespace) -> None:
        Logger().add_line("Running set-scan command", LogLevel.INFO)
        set_scan(args)

    @classmethod
    def register_subparser(cls, subparsers: _SubParsersAction) -> None:
        list_parser = subparsers.add_parser('set-scan', help="Command used to perform a recursive scan for sets starting from your current, or given path")
        list_parser.add_argument(
            '--path',
            type=str,
            required=False,
            help = "Optional parameter to specify the path"
        )
        list_parser.set_defaults(func=cls.command)
