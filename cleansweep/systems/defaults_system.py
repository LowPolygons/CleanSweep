from pathlib import Path
from cleansweep.globals.storage_paths import StoragePaths
from cleansweep.containers.user_settings import UserSettings
from cleansweep.codecs.user_settings_codec import UserSettingsCodec
from datetime import datetime, timedelta
import json
from cleansweep.types.json import Json

TEN_GB: int = 10000000000

# Primary use: to be called immediately after setup
class DefaultsWriter:
    @staticmethod
    def write_user_settings(path: Path) -> None:
        container_of_defaults: UserSettings = UserSettings(
            #Blacklist
            datetime.now() - timedelta(days=100), # TODO: refactor so that this value is the difference, not a relative date
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
            1001
        )
        
        defaults_json: Json = UserSettingsCodec.encode_to_json(container_of_defaults)
        try:
            with open(path, "w") as file:
                json.dump(defaults_json, file)
        except OSError as err:
            print(f"There was an error trying to write the default user settings to file, err {err}")

    
