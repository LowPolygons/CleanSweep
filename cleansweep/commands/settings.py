from cleansweep.interfaces.command import CommandInterface 
from cleansweep.utils.settings_command_display import SettingsCommandDisplay, DisplayOption
from cleansweep.utils.settings_command_reset import reset_user_settings
from cleansweep.utils.settings_command_modify import interactive_environment
from argparse import Namespace, _SubParsersAction

# TODO: Finish implementing
ARGUMENTS = [
    "modify",
    "display",
    "reset",
    "display-defaults"
]

class SettingsCommand(CommandInterface):
    @staticmethod
    def command(args: Namespace) -> None:
        if args.mode == 'modify':
            interactive_environment(-1)
        elif args.mode == 'display':
            # Load file and print
            SettingsCommandDisplay(DisplayOption.Regular)
            pass
        elif args.mode == 'reset':
            reset_user_settings()
        elif args.mode == 'display-defaults':
            # Load the default settings and attempt to display
            SettingsCommandDisplay(DisplayOption.Defaults) 
        else:
            print("Unknown argument, options are: modify, display, reset, display-defaults") 

    @classmethod
    def register_subparser(cls, subparsers: _SubParsersAction) -> None:
        list_parser = subparsers.add_parser('settings', help="TODO")
        list_parser.add_argument(
            '--mode',
            type=str,
            choices=['modify', 'display', 'reset', 'display-defaults'],
            required=True,
            help = "Choose whether to modify, display or reset your settings"
        )
        list_parser.set_defaults(func=cls.command)
        # TODO: Add arguments if necessary
