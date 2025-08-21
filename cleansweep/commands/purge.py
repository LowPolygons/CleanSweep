import time
import json
from pathlib import Path
from typing import cast
from cleansweep.codecs.file_array_codec import FileArrayCodec
from cleansweep.globals.storage_paths import StoragePaths
from cleansweep.interfaces.command import CommandInterface 
from argparse import Namespace, _SubParsersAction

from cleansweep.types.json import Json
from cleansweep.utils.get_main_path import get_main_path

class PurgeCommand(CommandInterface):
    @staticmethod
    def command(args: Namespace) -> None:
        if args.stage:
            # Load whitelist file, print number of files being staged, write to_delete
            try:
                unsanitised_whitelist: Json
                with open(get_main_path() / StoragePaths.white_listed_file_name, "r") as file:
                    unsanitised_whitelist = cast(Json, json.load(file))

                maybe_file_array = FileArrayCodec.create_from_json(unsanitised_whitelist)

                if not maybe_file_array:
                    print("Issue trying to format an array of files. Is the file empty?")
                    return

                # List the number of files
                print(f"There are {len(maybe_file_array)} files being staged for deletion")

                # Save each path on a separate line
                with open(StoragePaths.to_delete_local_temp_file_name, "w") as file:
                    for path in maybe_file_array:
                        file.write(f"{path.get_path()}\n")
                
                # Instruction statement
                print(f"The staged files have been written to {StoragePaths.to_delete_local_temp_file_name} in your current directory.")
                print("Please check this file for any files you don't want deleting. \n Once ready, run 'cleansweep purge --continue")
            except Exception as err:
                print(f"There was an issue trying to stage the whitelisted files: '{err}', Maybe confirm the file exists and has data and try again")
        elif args.continue_deletion:
            # Load the to_delete file, print the files and amount of data to be deleted, run final confirmations, 10s delay and then delete
            staged_paths: list[Path] = []
            try:
                with open(StoragePaths.to_delete_local_temp_file_name, "r") as file:
                    for line in file:
                        staged_paths.append(Path(line.strip()))
            except Exception as err:
                print(f"There was an issue trying to prep staged files for deletion: {err}")

            # Print the staged paths and data to be deleted
            size_of_data_being_deleted: int = 0
            for curr_path in staged_paths:
                print(f"- {curr_path}")
                size_of_data_being_deleted += curr_path.stat().st_size
            print(f"\nAmount of data being deleted: {size_of_data_being_deleted} bytes")

            # Confirmation Stage
            print("Please enter 'confirm' for the following statements to confirm deletion (space/case sensitive):")

            confirm1 = input("\n - I confirm that I want these files to be deleted and that this is an irreversable action \n => ")
            confirm2 = input("\n - I confirm that these files were scanned using my most up-to-date user settings \n => ")

            if confirm1 != "confirm" or \
               confirm2 != "confirm":
                print("Confirmation staged failed. If this was an accident, re-run the command.")
                return

            # Deletion stage
            print("Confirmation successful. Purging will begin in 10 seconds, cancel any time before this to cancel")

            time.sleep(10)
            
            for curr_path in staged_paths:
                try:
                    curr_path.unlink()
                except Exception as err:
                    print(f"Failed to delete {curr_path}: {err}, skipping")
                    continue
            
            # Try delete the temp file
            try:
                Path(StoragePaths.to_delete_local_temp_file_name).unlink()
            except Exception as err:
                print(f"Failed to delete temporary {StoragePaths.to_delete_local_temp_file_name} file: {err}")
                return
        else:
            print("TODO: better description, Please stage or continue the purge operation")
        # Load the whitelisted files and print how many files will be deleted, ideally with info on how much data will be removed
        # Open the file to see the listed files
        # try: 
        #     unsanitised_json: Json
        #     with open(get_main_path() / StoragePaths.white_listed_file_name, "r") as file:
        #         unsanitised_json = cast(Json, json.load(file))
        #
        #     maybe_file_array = FileArrayCodec.create_from_json(unsanitised_json)
        #
        #     if not maybe_file_array:
        #         print("Issue trying to create the items from the file array")
        #         return
        #
        #     # List number of files
        #     print(f"There are {len(maybe_file_array)} files to be staged for deletion. Ensure that this number seems correct")
        #
        #     # Write the files to a 'to_delete' file which is just one path per line with a password on top line
        # except Exception as err:
        #     print(f"There was an error trying to list files: {err}")
        #     return
        # Then a confirmation that they want to delete the files 
        # Then a confirmation that they understand its irreversable
        # Then a confirmation that the files in the whitelist are as up to date as they expect and they haven't forgot to re-run scan if they edited their settings
        
        # Print a statement saying 'in 10 seconds the purge will begin'
        # Possibly have it so that at any point if the user presses enter it immediately cancels the deletion

        # Then print success status
        # Then wipe the whitelist file

    @classmethod
    def register_subparser(cls, subparsers: _SubParsersAction) -> None:
        list_parser = subparsers.add_parser('purge', help="TODO")
        list_parser.add_argument(
            '--stage',
            dest="stage",
            required=False,
            action="store_true",
            help = "Prep the whitelisted files for final checks before deletion"
        )
        list_parser.add_argument(
            '--continue',
            dest="continue_deletion",
            required=False,
            action="store_true",
            help = "Enter the final stages of confirmation and deletion for staged files"
        )
        list_parser.set_defaults(func=cls.command)
        # TODO: Add arguments if necessary
