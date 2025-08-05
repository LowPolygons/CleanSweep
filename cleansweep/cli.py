import argparse

from cleansweep.systems.file_system import FileSystemManager
from pathlib import Path
from cleansweep.systems.logger_system import Logger, LogLevel
from cleansweep.commands import COMMANDS

def main():
    parser = argparse.ArgumentParser(description="Example CLI Tool")
    subparsers = parser.add_subparsers(dest="command", required=True)

    for command in COMMANDS:
        command.register_subparser(subparsers)
        
    args = parser.parse_args()
    args.func(args)
        
if __name__ == "__main__":
    main()


