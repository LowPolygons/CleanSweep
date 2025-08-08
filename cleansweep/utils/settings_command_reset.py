import os
import json
from pathlib import Path
from typing import cast
from cleansweep.globals.storage_paths import StoragePaths
from cleansweep.types.json import Json
from cleansweep.utils.get_main_path import get_main_path

def reset_user_settings():
    main_path: Path = get_main_path()
    defaults = StoragePaths.user_settings_defaults_file_name
    currents = StoragePaths.user_settings_file_name

    if not os.path.exists(main_path / defaults) or not os.path.exists(main_path / currents):
        print(f"One or both of the files were not found.. have you run the setup command?")
        return 
    
    try:
        # Defaults 
        defaults_json: Json
        with open(main_path / defaults, "r") as defaults:
            defaults_json = cast(Json, json.load(defaults))
        
        # Now override the currents
        with open(main_path / currents, "w") as currents:
            json.dump(defaults_json, currents)
       
        print("User settings have been set to the default options")
    except OSError as err:
        print(f"Failed to override settings with defaults: {err}")
