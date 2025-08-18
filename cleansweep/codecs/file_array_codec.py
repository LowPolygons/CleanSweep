
from pathlib import Path
from typing import Optional
from cleansweep.codecs.file_item_codec import FileItemCodec
from cleansweep.containers.file_item import FileItem
from cleansweep.interfaces.json_codec import JsonCodecInterface
from cleansweep.systems.json_reader import JsonReader
from cleansweep.types.json import Json

# TODO: Refactor so that the file that stores black and white listed things doesnt store the entire object data, 
# instead just the paths - it makes this bit far less messy and removes a lot of code that isn't strictly needed 
# and can be (re)evaluated at run time

FileArray = list[FileItem]

class FileArrayCodec(JsonCodecInterface[FileArray]):
    @staticmethod
    def encode_to_json(obj: FileArray) -> Json:
        json_items: list[Json] = []

        for file_item in obj:
            json_items.append(FileItemCodec.encode_to_json(file_item))

        return json_items

    @staticmethod
    def create_from_json(obj: Json) -> Optional[FileArray]:
        try:
            validated_obj: list[Json] = JsonReader.extract_list_of_type(obj, str)
            file_objects: list[FileItem] = []
    
            for json_path in validated_obj:
                maybe_item = FileItemCodec.create_from_json(json_path)

                if not maybe_item:
                    print(f"Failed to create file at path {JsonReader.extract_str(json_path)}")
                    continue

                file_objects.append(maybe_item)

            return file_objects
        except Exception as err:
            print(f"Error trying to create the list of files: {err}")
            return None
