from pathlib import Path
from cleansweep.globals.storage_paths import StoragePaths
from cleansweep.interfaces.command import CommandInterface 
from argparse import Namespace, _SubParsersAction

import shutil

class DemolishCommand(CommandInterface):
    @staticmethod
    def command(args: Namespace) -> None:
        print("Removing the stored data from CleanSweep")

        user_home_dir: Path = Path.home()
        storage_path: StoragePaths = StoragePaths(user_home_dir)

        # Create the main cleansweep dir
        main_folder: Path = user_home_dir / storage_path.main_dir_name
        
        # Create the main folder if it doesn't exist 
        if not main_folder.exists():
            print("CleanSweep has not been initialised")
            return
    
        # Confirm removal
        confirmation = input("If you are certain you want to remove CleanSweep, enter 'cleansweep demolish' again\n => ")

        # Remove the cleansweep directory
        if confirmation == "cleansweep demolish":
            shutil.rmtree(main_folder)
            print("Uninstalled cleansweep data from your system - to rebuild, run 'cleansweep setup'")
        

    @classmethod
    def register_subparser(cls, subparsers: _SubParsersAction) -> None:
        list_parser = subparsers.add_parser('demolish', help="Command used to remove cleansweep files from your machine")
        list_parser.set_defaults(func=cls.command)
