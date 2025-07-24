from pathlib import Path
from cleansweep.stack.stack import Stack

class FileSystemManager:
    @staticmethod
    def get_file_names_immediate(starting_path: Path) -> list[Path]:
        file_names = [item.resolve() for item in starting_path.iterdir() if item.is_file()]

        return file_names 

    @staticmethod
    def get_dir_names_immediate(starting_path: Path) -> list[Path]:
        directory_names = [item.resolve() for item in starting_path.iterdir() if item.is_dir()]

        return directory_names 

    @staticmethod 
    def get_file_names_recursive(starting_path: Path) -> list[Path]:
        found_files: list[Path] = []
        directories_to_search: Stack[Path] = Stack()

        directories_to_search.push(starting_path)
        
        while len(directories_to_search) != 0:
            curr_dir = directories_to_search.pop()

            # The len != 0 condition should prevent this from happening
            if curr_dir is None:
                continue
            
            files_found = FileSystemManager.get_file_names_immediate(curr_dir)
            dirs_found = FileSystemManager.get_dir_names_immediate(curr_dir)
        
            for file in files_found:
                found_files.append(file)

            for dir in dirs_found:
                directories_to_search.push(dir)

        return found_files

