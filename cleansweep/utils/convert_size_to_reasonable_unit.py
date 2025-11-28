from enum import Enum

class DataSizes(Enum):
    Bytes = 0
    KiloBytes = 1
    MegaBytes = 2
    GigaBytes = 3
    TerraBytes = 4
    PetaBytes = 5
    ExaByte = 6 # So i don't have to do any sense checking, no one will ever exceed 1000 exabytes

def match_datasize_to_string(inp: DataSizes):
    match inp:
        case DataSizes.Bytes:
            return "B"
        case DataSizes.KiloBytes:
            return "KB"
        case DataSizes.MegaBytes:
            return "MB"
        case DataSizes.GigaBytes:
            return "GB"
        case DataSizes.TerraBytes:
            return "TB"
        case DataSizes.PetaBytes:
            return "PB"
        case DataSizes.ExaByte:
            return "EB"

def convert_size_to_reasonable_unit(file_size_in_bytes: int) -> (float, DataSizes):
    number_units_passed = 0
    floaty_size = float(file_size_in_bytes) / 1000.0
    while floaty_size > 1:
        number_units_passed += 1
        
        floaty_size = floaty_size / 1000.0
    
    # Undo the last one
    floaty_size *= 1000

    if number_units_passed > 6:
        raise ValueError("You are handling seemingly more than 1000 Exabytes. Something has gone wrong")

    return (round(floaty_size, 3), DataSizes(number_units_passed)) 
