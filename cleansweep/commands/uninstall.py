from cleansweep.interfaces.command import CommandInterface 
from argparse import Namespace, _SubParsersAction

class UninstallCommand(CommandInterface):
    @staticmethod
    def command(args: Namespace) -> None:
        print("Removing the stored data from CleanSweep")

    @classmethod
    def register_subparser(cls, subparsers: _SubParsersAction) -> None:
        list_parser = subparsers.add_parser('uninstall', help="TODO")
        list_parser.set_defaults(func=cls.command)
        # TODO: Add arguments if necessary
