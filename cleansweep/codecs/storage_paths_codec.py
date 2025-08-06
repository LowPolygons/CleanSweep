from cleansweep.globals.storage_paths import StoragePaths
from cleansweep.interfaces.json_codec import JsonCodecInterface
from cleansweep.types.json import Json

class StoragePathsCodec(JsonCodecInterface[StoragePaths]):
    @staticmethod
    def encode_to_json(obj: StoragePaths) -> Json:
        # Create the main cleansweep dir from the obj specification
        data: Json = {
            "path_to_home" : str(obj.path_to_home),
            "main_dir_name" : str(obj.main_dir_name),
            "white_listed_file_name" : str(obj.white_listed_file_name),
            "black_listed_file_name" : str(obj.black_listed_file_name),
            "user_settings_file_name" : str(obj.user_settings_file_name),
            "log_file_name" : str(obj.log_file_name),
            "storage_paths_file_name" : str(obj.storage_paths_file_name)
        }
        return data

