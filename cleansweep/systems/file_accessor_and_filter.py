from cleansweep.containers.file_item import FileItem
from cleansweep.containers import STARTS_WITH_SUBSTR, SUBSTR_NOT_FOUND

class FileAccessorAndFilter:
    @classmethod
    def is_file_name_in_list(cls, file: FileItem, name_substrings: list[str], check_if_starts_with: bool = False):
        file_name = file.get_name()

        if check_if_starts_with:
            for starts_with in name_substrings:
                if file_name.find(starts_with) == STARTS_WITH_SUBSTR:
                    return True
            return False
        else:
            for name_contains in name_substrings:
                if file_name.find(name_contains) is not SUBSTR_NOT_FOUND:
                    return True
            return False

    @classmethod
    def is_file_size_greater_than(cls, file: FileItem, min_size: int):
        file_size = file.get_size()

        return file_size >= min_size


    @classmethod
    def is_file_path_in_list(cls, file: FileItem, path_substrings: list[str]):
        file_path = file.get_path()

        for path in path_substrings:
            if str(file_path).find(path) is not SUBSTR_NOT_FOUND:
                return True
        return False 

    @classmethod
    def is_file_extension_in_list(cls, file: FileItem, extensions: list[str]):
        file_extension = file.get_extension()

        for extension in extensions:
            if file_extension == f".{extension}":
                return True
        return False

    @classmethod
    def maybe_in_set(cls, file: FileItem, extensions: list[str], name_substrs: list[str]) -> bool:
        file_extension_status = cls.is_file_extension_in_list(file, extensions)
        file_name_substr_status = cls.is_file_name_in_list(file, name_substrs)

        if file_extension_status or file_name_substr_status:
            return True

        return False 

