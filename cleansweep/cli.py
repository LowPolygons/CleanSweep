import argparse

from cleansweep.systems.file_system.filesystem import FileSystemManager
from pathlib import Path

def main():
    # One command will list all previously flagged files, maybe with a splash of colour 
    # One command will list DEFAULT user specifications ( like black/white listed files )
    # One command will delete files with inline specification (such as .out files for eg), with optional additional flags like --directory
    parser = argparse.ArgumentParser(description="CleanSweep CLI tool")
    parser.add_argument('command', choices=['list', 'scan'], help='Subcommand to run')
    args = parser.parse_args()

    if args.command == 'list':
        print("Listing flagged files...")
    elif args.command == 'scan':
        files = FileSystemManager.get_file_names_recursive(Path("")) 
        for file in files:
            print("File: ", file)
if __name__ == "__main__":
    main()


