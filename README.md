# CleanSweep
A python CLI tool to help remove old and large files on a system

## TODO:
Swap list of paths to be some sort of tree - therefore if a directory such as '.git' gets flagged but something inside of it doesn't you can filter all of them in one go
alternatively, perform path checks at actual file scan time, and then perform other checks at file item time

Any print statements in errors that are not directly in a 'command' file or util should user the Logger instead and let the command do the final print
