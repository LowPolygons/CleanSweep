from cleansweep.interfaces.command import CommandInterface 
from argparse import Namespace, _SubParsersAction

class ScanCommand(CommandInterface):
    @staticmethod
    def command(args: Namespace) -> None:
        print("Scanning directories")

    @classmethod
    def register_subparser(cls, subparsers: _SubParsersAction) -> None:
        list_parser = subparsers.add_parser('scan', help="TODO")
        list_parser.set_defaults(func=cls.command)
        # TODO: Add arguments if necessary
