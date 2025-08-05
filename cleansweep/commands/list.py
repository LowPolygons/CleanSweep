from cleansweep.commands.command import CommandInterface
from argparse import Namespace, _SubParsersAction

# arg name

class ListCommand(CommandInterface):
    @staticmethod
    def command(args: Namespace) -> None:
        print("Listing directories")

    @classmethod
    def register_subparser(cls, subparsers: _SubParsersAction) -> None:
        list_parser = subparsers.add_parser('list', help="TODO")
        list_parser.set_defaults(func=cls.command)
        # TODO: Add arguments if necessary
