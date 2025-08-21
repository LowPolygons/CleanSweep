import argparse

from cleansweep.systems.logger_system import Logger
from cleansweep.commands import COMMANDS

def main():
    parser = argparse.ArgumentParser(description="Example CLI Tool")
    subparsers = parser.add_subparsers(dest="command", required=True)

    for command in COMMANDS:
        command.register_subparser(subparsers)
        
    args = parser.parse_args()
    args.func(args)
    
    log_write_success = Logger().write_log()

    if not log_write_success:
        print("Failed to write log file.")

if __name__ == "__main__":
    main()


