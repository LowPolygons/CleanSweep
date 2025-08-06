from pathlib import Path
from cleansweep.codecs.storage_paths_codec import StoragePathsCodec
from cleansweep.globals.storage_paths import StoragePaths
from cleansweep.interfaces.command import CommandInterface 
from argparse import Namespace, _SubParsersAction
from cleansweep.types.json import Json

import json

class SetupCommand(CommandInterface):
    @staticmethod
    def command(args: Namespace) -> None:
        user_home_dir: Path = Path.home()
        initial_storage_path: StoragePaths = StoragePaths(user_home_dir)

        # Create the main cleansweep dir
        main_folder: Path = user_home_dir / initial_storage_path.main_dir_name
        
        # Create the main folder if it doesn't exist 
        if main_folder.exists():
            print("CleanSweep already exists on this machine")
        else:
            main_folder.mkdir()
            # Create the filepaths needed
            files: dict[str, Path] = {
                "White-Listed" : main_folder / initial_storage_path.white_listed_file_name, 
                "Black-Listed" : main_folder / initial_storage_path.black_listed_file_name,
                "User-Settings" : main_folder / initial_storage_path.user_settings_file_name,
                "Log-File" : main_folder / initial_storage_path.log_file_name,
                "Storage-Paths" : main_folder / initial_storage_path.storage_paths_file_name
            }
            # Initialise the files with an empty json string
            for curr_file in files.values():
                try:
                    with open(curr_file, "w") as file:
                        file.write("{}")
                except OSError as err:
                    print(f"There was an error creating file with path {curr_file} with error {err}")
                    return

            # Now write the storage paths data to its file
            jsoned_storage_paths: Json = StoragePathsCodec.encode_to_json(initial_storage_path)
            try:
                with open(files["Storage-Paths"], "w") as file:
                    json.dump(jsoned_storage_paths, file)
            except OSError as err:
                print(f"There was an error writing the data from the storage path structure with error {err}")
                return
           
            # Print the details of what has just occured
            print("[]===[]===[]===[]===[]\nCleanSweep has been successfully setup:\n")
            for key, value in files.items():
                print(f"- {key} found at {value}")

    @classmethod
    def register_subparser(cls, subparsers: _SubParsersAction) -> None:
        list_parser = subparsers.add_parser('setup', help="TODO")
        list_parser.set_defaults(func=cls.command)
