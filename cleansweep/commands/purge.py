from cleansweep.interfaces.command import CommandInterface 
from argparse import Namespace, _SubParsersAction
from cleansweep.utils.purge_command_continue import purge_continue
from cleansweep.utils.purge_command_stage import purge_stage

class PurgeCommand(CommandInterface):
    @staticmethod
    def command(args: Namespace) -> None:
        if args.stage:
            purge_stage()
        elif args.continue_deletion:
            purge_continue()
        else:
            print("TODO: better description, Please stage or continue the purge operation")

    @classmethod
    def register_subparser(cls, subparsers: _SubParsersAction) -> None:
        list_parser = subparsers.add_parser('purge', help="TODO")
        list_parser.add_argument(
            '--stage',
            dest="stage",
            required=False,
            action="store_true",
            help = "Prep the whitelisted files for final checks before deletion"
        )
        list_parser.add_argument(
            '--continue',
            dest="continue_deletion",
            required=False,
            action="store_true",
            help = "Enter the final stages of confirmation and deletion for staged files"
        )
        list_parser.set_defaults(func=cls.command)
