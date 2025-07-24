from cleansweep.containers.file_item.file_item import FileItem

class FilterSystem: 
    @staticmethod
    def does_file_match_constraints(file: FileItem) -> bool:
        return True #TODO: CONSTRAINT OBJECT NEEDED
