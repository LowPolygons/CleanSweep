from cleansweep.globals.log_levels import LogLevel
from cleansweep.globals.singleton_metaclass import Singleton
from cleansweep.globals.storage_paths import StoragePaths

from datetime import date

class Logger(metaclass=Singleton):
    def __init__(self) -> None:
        self.__log_name = StoragePaths.log_file_name
        self.__log_lines: list[str] = []
    @staticmethod
    def log_level_to_string(level: LogLevel) -> str:
        match_level: dict = {
            LogLevel.INFO: "[INFO]",
            LogLevel.WARN: "[WARN]",
            LogLevel.DEBUG: "[DEBUG]",
            LogLevel.ERROR: "[ERROR]"
        }
        return match_level.get(level, "[OTHER]")

    def add_line(self, line: str, log_level: LogLevel):
        self.__log_lines.append("{} {}".format(Logger.log_level_to_string(log_level), line))

    def write_log(self) -> bool:
        log_file_name = StoragePaths.get_formatted_path_with_file(self.__log_name)

        try:
            with open(log_file_name, "a") as log_file:
                log_file.write("\nCLEANSWEEP - RUNDATE: {}\n".format(date.today()))
                for line in self.__log_lines:
                    log_file.write(line)
        except OSError as _:
            return False
        return True
