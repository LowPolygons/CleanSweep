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
from cleansweep.containers.file_item import FileItem

class ListCommand(CommandInterface):
    @staticmethod
    def command(args: Namespace) -> None:
        Logger().add_line("Running List Command", LogLevel.INFO)

        args.choice = 'to_delete' if not args.choice else args.choice

        file_to_load: str 
        if args.choice == 'to_keep' or args.choice == 'k':
            print("Listing files which will be ignored")
            file_to_load = StoragePaths.to_keep_file_name
        elif args.choice == 'to_delete' or args.choice == 'd':
            print("Listing files which will be deleted upon purge")
            file_to_load = StoragePaths.to_delete_file_name
        elif args.choice == 'non-special' or args.choice == 'n':
            print("Listing files which meet the minimum requirements to be flagged, but will not be acted upon")
            file_to_load = StoragePaths.minimum_flagged_file_name
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
                print("Issue trying to create the items from the file array, it may be empty")
                return
            
            for curr_file in maybe_file_array:
                print(f"- {curr_file.get_path()}")

        except Exception as err:
            print(f"There was an error trying to list files: {err}")

    @classmethod
    def register_subparser(cls, subparsers: _SubParsersAction) -> None:
        list_parser = subparsers.add_parser('list', help="Command used to list varying lists of files that have been scanned. Optional argument '--choice' which defaults to 'whitelisted'")
        list_parser.add_argument(
            '--choice',
            type=str,
            choices=['to_keep', 'k', 'to_delete', 'd', 'non-special', 'n'],
            required=False,
            help = "Choice of which category of files should be listed"
        )
        list_parser.set_defaults(func=cls.command)
