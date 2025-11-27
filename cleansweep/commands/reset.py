from cleansweep.globals.log_levels import LogLevel
from cleansweep.interfaces.command import CommandInterface 
from cleansweep.systems.logger_system import Logger
from cleansweep.globals.storage_paths import StoragePaths
import os
from argparse import Namespace, _SubParsersAction
from cleansweep.utils.get_main_path import get_main_path
import json
from cleansweep.types.json import Json
from typing import Union

class ResetCommand(CommandInterface):
    @staticmethod
    def command(args: Namespace) -> None:
        files_to_load: list[str] = [] 
        if args.choice == 'to_keep' or args.choice == 'k':
            print("Reseting list of files which will be ignored")
            files_to_load.append(StoragePaths.to_keep_file_name)
        elif args.choice == 'to_delete' or args.choice == 'd':
            print("Reseting list of files which will be deleted upon purge")
            files_to_load.append(StoragePaths.to_delete_file_name)
        else:
            files_to_load.append(StoragePaths.to_keep_file_name)
            files_to_load.append(StoragePaths.to_delete_file_name)
        # Does the file exist?
        for file_to_load in files_to_load:
            if not os.path.exists(get_main_path() / file_to_load):
                print(f"The desired file at {get_main_path() / file_to_load} doesn't exist - have you run the setup command?")
            else:
                try:
                    with open(get_main_path() / file_to_load, "w") as file:
                        file.write("[]") 
                except Exception as err:
                    print(f"There was an error trying to reset files: {err}")
    @classmethod
    def register_subparser(cls, subparsers: _SubParsersAction) -> None:
        reset_parser = subparsers.add_parser('reset', help="Command used to reset cleansweep scan files")
        reset_parser.add_argument(
            '--choice',
            type=str,
            choices=['to_keep', 'k', 'to_delete', 'd'],
            required=False,
            help = "Choice of which category of files should be reset"
        )
        reset_parser.set_defaults(func=cls.command)
