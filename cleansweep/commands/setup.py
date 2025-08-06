from cleansweep.interfaces.command import CommandInterface 
from argparse import Namespace, _SubParsersAction

class SetupCommand(CommandInterface):
    @staticmethod
    def command(args: Namespace) -> None:
        print("Setting up file system for CleanSweep...")

    @classmethod
    def register_subparser(cls, subparsers: _SubParsersAction) -> None:
        list_parser = subparsers.add_parser('setup', help="TODO")
        list_parser.set_defaults(func=cls.command)
