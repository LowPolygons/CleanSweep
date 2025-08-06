from cleansweep.interfaces.command import CommandInterface 
from cleansweep.systems.settings_system import InteractiveSettingsSystem
from argparse import Namespace, _SubParsersAction

class SettingsCommand(CommandInterface):
    @staticmethod
    def command(args: Namespace) -> None:
        print("Settings config..")
        settings_system = InteractiveSettingsSystem()
        settings_system.environment(0)

    @classmethod
    def register_subparser(cls, subparsers: _SubParsersAction) -> None:
        list_parser = subparsers.add_parser('settings', help="TODO")
        list_parser.set_defaults(func=cls.command)
        # TODO: Add arguments if necessary
