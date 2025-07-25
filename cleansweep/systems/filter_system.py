from cleansweep.containers.file_item import FileItem
from cleansweep.containers.user_settings import UserSettings
from cleansweep.globals.filter_codes import FilterCodes
from cleansweep.globals.flag_codes import FlagCodes

class FilterSystem: 
    @staticmethod
    def determine_file_filtration_status(file: FileItem, user_settings: UserSettings) -> FilterCodes:
        name_status = file.filter_name(
            user_settings.prioritise_file_names_containing,
            user_settings.ignore_file_names_containing, 
            user_settings.prioritise_file_names_starting_with,
            user_settings.ignore_file_names_starting_with
        )
        size_status = file.filter_size(
            user_settings.prioritise_files_larger_than,
            user_settings.ignore_files_smaller_than,
            user_settings.ignore_files_larger_than
        )
        path_status = file.filter_path(
            user_settings.prioritise_files_whos_directory_contains,
            user_settings.ignore_files_whos_directory_contains 
        )
        extension_status = file.filter_extension(
            user_settings.prioritise_files_with_extension,
            user_settings.ignore_files_with_extension
        )
        # Prioritise a black list first
        if name_status == FilterCodes.BlackListed or \
            size_status == FilterCodes.BlackListed or \
            path_status == FilterCodes.BlackListed or \
            extension_status == FilterCodes.BlackListed:
            return FilterCodes.BlackListed 

        if name_status == FilterCodes.WhiteListed or \
            size_status == FilterCodes.WhiteListed or \
            path_status == FilterCodes.WhiteListed or \
            extension_status == FilterCodes.WhiteListed:
            return FilterCodes.WhiteListed 
        
        return FilterCodes.NotSpecial
        
    @staticmethod
    def file_is_flagged(file: FileItem, user_settings: UserSettings) -> FlagCodes:
        # The default condition for a file being flagged is that it is older than a certain date
        file_meets_age_requirement = file.was_last_modified_before(user_settings.flag_date_cutoff)

        if not file_meets_age_requirement:
            return FlagCodes.NotFlagged
 
        # From here, check that it hasn't been black listed
        filtration_status = FilterSystem.determine_file_filtration_status(file, user_settings)

        if filtration_status == FilterCodes.BlackListed:
            return FlagCodes.NotFlagged

        if filtration_status == FilterCodes.WhiteListed:
            return FlagCodes.FlaggedWhite 
        
        return FlagCodes.Flagged
