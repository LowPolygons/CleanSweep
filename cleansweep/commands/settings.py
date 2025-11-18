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
        if args.action == "modify":
            Logger().add_line("Running settings command with --modify", LogLevel.INFO)
            InteractiveEnvironment.interactive_environment(-1)
            print("\nMake sure you re-scan any files to get the most up-to-date list.")
        elif args.action == "reset":
            Logger().add_line("Running settings command with --reset", LogLevel.INFO)
            reset_user_settings()
        elif args.action == "display":
            if args.which == 'current':
                Logger().add_line("Running settings command with --display", LogLevel.INFO)
                # Load file and print
                SettingsCommandDisplay(SettingsVariant.Regular)
                pass
            elif args.which == 'defaults':
                Logger().add_line("Running settings command with --displa", LogLevel.INFO)
                # Load the default settings and attempt to display
                SettingsCommandDisplay(SettingsVariant.Defaults)
            else:
                Logger().add_line("Attempted to run settings display command with invalid args", LogLevel.INFO)
                print("Unknown argument for display command, options: current, defaults")
        else:
            Logger().add_line("Attempted to run settings command with no args", LogLevel.INFO)
            print("Unknown argument, options are: modify, --display, reset") 

    @classmethod
    def register_subparser(cls, subparsers: _SubParsersAction) -> None:
        settings_parser = subparsers.add_parser('settings', help="Command used to modify, display, or reset information relating to your settings")
        settings_sub = settings_parser.add_subparsers(dest="action", required=True)
        # Modify
        settings_sub.add_parser(
            'modify',
            help = "Choose to enter an interactive environment to edit your settings"
        )
        # Display
        display_parser = settings_sub.add_parser(
            'display',
            help = "Choose whether to display your current settings or the system defaults"
        )
        display_parser.add_argument(
            "which",
            choices=["current","defaults"],
            help=""
        )
        # Reset 
        settings_sub.add_parser(
            'reset',
            help = "Choose to reset your current settings to your system defaults"
        )
        settings_parser.set_defaults(func=cls.command)
