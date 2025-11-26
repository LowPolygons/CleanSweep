from cleansweep.globals.log_levels import LogLevel
from cleansweep.interfaces.command import CommandInterface 
from argparse import Namespace, _SubParsersAction
from cleansweep.systems.logger_system import Logger
from cleansweep.utils.purge_command_continue import purge_continue
from cleansweep.utils.purge_command_stage import purge_stage

class PurgeCommand(CommandInterface):
    @staticmethod
    def command(args: Namespace) -> None:
        if args.stage:
            Logger().add_line("Running purge command with '--staged' arg", LogLevel.INFO)
            purge_stage()
        elif args.continue_deletion:
            Logger().add_line("Running purge command with '--continue' arg", LogLevel.INFO)
            purge_continue()
        else:
            Logger().add_line("Running purge command with no args, nothing happens", LogLevel.INFO)
            print("Please run this command with either the --stage or --continue arg. If you run it with both, --stage takes priority")

    @classmethod
    def register_subparser(cls, subparsers: _SubParsersAction) -> None:
        list_parser = subparsers.add_parser('purge', help="Command used to stage and delete any whitelisted files")
        list_parser.add_argument(
            '--stage',
            dest="stage",
            required=False,
            action="store_true",
            help = "Prep the files to delete for final checks before deletion"
        )
        list_parser.add_argument(
            '--continue',
            dest="continue_deletion",
            required=False,
            action="store_true",
            help = "Enter the final stages of confirmation and deletion for staged files"
        )
        list_parser.set_defaults(func=cls.command)
