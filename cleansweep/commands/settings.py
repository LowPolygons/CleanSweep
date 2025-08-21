from cleansweep.globals.log_levels import LogLevel
from cleansweep.globals.user_setting_variant import SettingsVariant
from cleansweep.interfaces.command import CommandInterface 
from cleansweep.systems.logger_system import Logger
from cleansweep.utils.settings_command_display import SettingsCommandDisplay
from cleansweep.utils.settings_command_reset import reset_user_settings
from cleansweep.utils.settings_command_modify import InteractiveEnvironment 
from argparse import Namespace, _SubParsersAction

class SettingsCommand(CommandInterface):
    @staticmethod
    def command(args: Namespace) -> None:
        if args.mode == 'modify':
            Logger().add_line("Running settings command with --modify", LogLevel.INFO)
            InteractiveEnvironment.interactive_environment(-1)
            print("\nMake sure you re-scan any files to get the most up-to-date list.")
        elif args.mode == 'display':
            Logger().add_line("Running settings command with --display", LogLevel.INFO)
            # Load file and print
            SettingsCommandDisplay(SettingsVariant.Regular)
            pass
        elif args.mode == 'reset':
            Logger().add_line("Running settings command with --reset", LogLevel.INFO)
            reset_user_settings()
        elif args.mode == 'display-defaults':
            Logger().add_line("Running settings command with --displa", LogLevel.INFO)
            # Load the default settings and attempt to display
            SettingsCommandDisplay(SettingsVariant.Defaults) 
        else:
            Logger().add_line("Attempted to run settings command with no args", LogLevel.INFO)
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
