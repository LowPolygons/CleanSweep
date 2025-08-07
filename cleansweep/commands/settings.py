from typing import cast
from cleansweep.containers.user_settings import UserSettings
from cleansweep.interfaces.command import CommandInterface 
from cleansweep.systems.settings_system import InteractiveSettingsSystem
from cleansweep.globals.storage_paths import StoragePaths
from cleansweep.codecs.user_settings_codec import UserSettingsCodec
from cleansweep.single_funcs.get_main_path import get_main_path
from cleansweep.single_funcs.settings_command_list_default import SettingsCommandListDefault 
from argparse import Namespace, _SubParsersAction

import os
import json

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
