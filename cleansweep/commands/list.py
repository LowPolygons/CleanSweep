import json
import os
from typing import cast
from cleansweep.codecs.file_array_codec import FileArrayCodec
from cleansweep.globals.log_levels import LogLevel
from cleansweep.globals.storage_paths import StoragePaths
from cleansweep.interfaces.command import CommandInterface
from argparse import Namespace, _SubParsersAction

from cleansweep.systems.logger_system import Logger
from cleansweep.types.json import Json
from cleansweep.utils.get_main_path import get_main_path

class ListCommand(CommandInterface):
    @staticmethod
    def command(args: Namespace) -> None:
        Logger().add_line("Running List Command", LogLevel.INFO)

        file_to_load: str 
        if args.choice == 'blacklisted':
            print("Listing blacklisted files - files which will be ignored")
            file_to_load = StoragePaths.black_listed_file_name
        elif args.choice == 'whitelisted':
            print("Listing whitelisted files - files which will be deleted upon purge")
            file_to_load = StoragePaths.white_listed_file_name
        else:
            print("Invalid argument passed")
            return
        # Does the file exist?
        if not os.path.exists(get_main_path() / file_to_load):
            print("The desired file doesn't exist - have you run the setup command?")
            return
        # Open the file to see the listed files
        try:
            unsanitised_json: Json
            with open(get_main_path() / file_to_load, "r") as file:
                unsanitised_json = cast(Json, json.load(file))
            
            maybe_file_array = FileArrayCodec.create_from_json(unsanitised_json)

            if not maybe_file_array:
                print("Issue trying to create the items from the file array")
                return
            
            for curr_file in maybe_file_array:
                print(f"- {curr_file.get_path()}")

        except Exception as err:
            print(f"There was an error trying to list files: {err}")

    @classmethod
    def register_subparser(cls, subparsers: _SubParsersAction) -> None:
        list_parser = subparsers.add_parser('list', help="Command used to list varying lists of files that have been scanned")
        list_parser.add_argument(
            '--choice',
            type=str,
            choices=['blacklisted', 'whitelisted'],
            required=True,
            help = "Choice of which category of files should be listed"
        )
        list_parser.set_defaults(func=cls.command)
