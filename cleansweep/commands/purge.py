from cleansweep.interfaces.command import CommandInterface 
from argparse import Namespace, _SubParsersAction

class PurgeCommand(CommandInterface):
    @staticmethod
    def command(args: Namespace) -> None:
        print("Purging directories")

    @classmethod
    def register_subparser(cls, subparsers: _SubParsersAction) -> None:
        list_parser = subparsers.add_parser('purge', help="TODO")
        list_parser.set_defaults(func=cls.command)
        # TODO: Add arguments if necessary
