from datetime import datetime, timedelta
from pathlib import Path

from cleansweep.codecs.user_settings_codec import UserSettingsCodec
from cleansweep.containers.user_settings import UserSettings
from cleansweep.globals.log_levels import LogLevel
from cleansweep.systems.logger_system import Logger
from cleansweep.types.json import Json

import json

TEN_GB: int = 10000000000

def write_user_settings(path: Path) -> None:
    container_of_defaults: UserSettings = UserSettings(
        #Blacklist
        datetime.now() - timedelta(days=100),
        ["z", "exe", "d"],
        ["EXAMPLE_NAME_CONTAINS:", "cleansweep"],
        ["EXAMPLE_DIRECTORY_CONTAINS:", "cleansweep"],
        ["."],
        1000,
        TEN_GB,
        #Now whitelist
        ["out"],
        ["OUTPUT", "HISTORY", "slurm-"],
        ["deleteme"],
        [],
        1001,
        # Consider access date
        False
    )
    
    defaults_json: Json = UserSettingsCodec.encode_to_json(container_of_defaults)
    try:
        with open(path, "w") as file:
            json.dump(defaults_json, file)
    except OSError as err:
        Logger().add_line(f"There was an error trying to write the default user settings to file, err {err}", LogLevel.ERROR)
