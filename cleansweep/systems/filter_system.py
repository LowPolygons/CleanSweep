from cleansweep.containers.file_item import FileItem
from cleansweep.containers.user_settings import UserSettings
from cleansweep.globals.filter_codes import FilterCodes
from cleansweep.globals.flag_codes import FlagCodes
from cleansweep.systems.file_accessor_and_filter import FileAccessorAndFilter

class FilterSystem:
    @classmethod
    def file_meets_flag_requirements(cls, file: FileItem, user_settings: UserSettings, flag_code: FlagCodes):
        temp_file_name_substrings: list[str]
        temp_file_name_starts_with: list[str]
        temp_file_extensions: list[str]
        temp_file_path_substrings: list[str]
        temp_file_min_size: Optional[int] = None
        temp_file_max_size: int

        match flag_code:
            case FlagCodes.FlaggedToKeep:
                temp_file_name_substrings = user_settings.ignore_file_names_containing
                temp_file_name_starts_with = user_settings.ignore_file_names_starting_with
                temp_file_extensions = user_settings.ignore_files_with_extension
                temp_file_path_substrings = user_settings.ignore_files_whos_directory_contains
                temp_file_min_size = user_settings.ignore_files_smaller_than
                temp_file_max_size = user_settings.ignore_files_larger_than
            case FlagCodes.FlaggedToDelete:
                temp_file_name_substrings = user_settings.prioritise_file_names_containing
                temp_file_name_starts_with = user_settings.prioritise_file_names_starting_with
                temp_file_extensions = user_settings.prioritise_files_with_extension
                temp_file_path_substrings = user_settings.prioritise_files_whos_directory_contains
                temp_file_max_size = user_settings.prioritise_files_larger_than
            case FlagCodes.Override:
                temp_file_name_substrings = user_settings.override_file_names_containing
                temp_file_name_starts_with = user_settings.override_file_names_starting_with
                temp_file_extensions = user_settings.override_files_with_extension
                temp_file_path_substrings = user_settings.override_files_whos_directory_contains
                temp_file_max_size = user_settings.override_files_larger_than
            case _:
                return
        
        name_status = FileAccessorAndFilter.is_file_name_in_list(file, temp_file_name_substrings)
        name_begins_status = FileAccessorAndFilter.is_file_name_in_list(file, temp_file_name_starts_with, True)
        size_status = FileAccessorAndFilter.is_file_size_greater_than(file, temp_file_max_size)
        smaller_than_size_status: Optional[bool] = \
            not FileAccessorAndFilter.is_file_size_greater_than(file, temp_file_min_size) \
            if temp_file_min_size is not None else None
        path_status = FileAccessorAndFilter.is_file_path_in_list(file, temp_file_path_substrings)
        extension_status = FileAccessorAndFilter.is_file_extension_in_list(file, temp_file_extensions)

        return (name_status or 
                name_begins_status or
                size_status or
                smaller_than_size_status or
                path_status or
                extension_status)
        

    @classmethod
    def determine_file_filtration_status(cls, file: FileItem, user_settings: UserSettings) -> FilterCodes:
        to_keep_status = cls.file_meets_flag_requirements(file, user_settings, FlagCodes.FlaggedToKeep)
        
        if to_keep_status:
            return FilterCodes.ToKeep
        
        to_delete_status = cls.file_meets_flag_requirements(file, user_settings, FlagCodes.FlaggedToDelete)

        if to_delete_status:
            return FilterCodes.ToDelete
        
        return FilterCodes.NotSpecial
        
    @staticmethod
    def file_is_flagged(file: FileItem, user_settings: UserSettings) -> FlagCodes:
        # The default condition for a file being flagged is that it is older than a certain date
        file_meets_age_requirement = file.was_last_modified_before(user_settings.flag_date_cutoff, user_settings.consider_access_date_when_filtering)

        if not file_meets_age_requirement:
            return FlagCodes.NotFlagged
 
        # From here, check that it hasn't been black listed
        filtration_status = FilterSystem.determine_file_filtration_status(file, user_settings)
        
        if filtration_status == FilterCodes.ToKeep:
            return FlagCodes.FlaggedToKeep

        if filtration_status == FilterCodes.ToDelete:
            return FlagCodes.FlaggedToDelete 
        
        return FlagCodes.Flagged
