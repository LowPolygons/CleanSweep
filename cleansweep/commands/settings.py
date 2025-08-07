from cleansweep.interfaces.command import CommandInterface 
from cleansweep.systems.settings_system import InteractiveSettingsSystem
from cleansweep.utils.settings_command_list_default import SettingsCommandListDefault 
from argparse import Namespace, _SubParsersAction


from cleansweep.types.json import Json

class SettingsCommand(CommandInterface):
    @staticmethod
    def command(args: Namespace) -> None:
        if args.mode == 'modify':
            settings_system = InteractiveSettingsSystem()
            settings_system.environment(0)
        elif args.mode == 'display':
            # Load file and print
            pass
        elif args.mode == 'reset':
            # Copy the defaults into the not defaults
            pass
        elif args.mode == 'list-defaults':
            # Load the default settings and attempt to display
            SettingsCommandListDefault() 
        else:
            print("Unknown argument, options are: modify, display, reset, list-defaults") 

    @classmethod
    def register_subparser(cls, subparsers: _SubParsersAction) -> None:
        list_parser = subparsers.add_parser('settings', help="TODO")
        list_parser.add_argument(
            '--mode',
            type=str,
            choices=['modify', 'display', 'reset', 'list-defaults'],
            required=True,
            help = "Choose whether to modify, display or reset your settings"
        )
        list_parser.set_defaults(func=cls.command)
        # TODO: Add arguments if necessary
