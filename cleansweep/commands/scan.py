
from argparse import Namespace, _SubParsersAction

from cleansweep.interfaces.command import CommandInterface 
from cleansweep.utils.scan_command import scan

class ScanCommand(CommandInterface):
    @staticmethod
    def command(args: Namespace) -> None:
        scan(args)

    @classmethod
    def register_subparser(cls, subparsers: _SubParsersAction) -> None:
        list_parser = subparsers.add_parser('scan', help="TODO")
        list_parser.add_argument(
            '--path',
            type=str,
            required=False,
            help = "Choose whether to modify, display or reset your settings"
        )
        list_parser.set_defaults(func=cls.command)
