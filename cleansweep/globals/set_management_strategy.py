from enum import Enum

class SetManagementStrategy(Enum):
    FirstAndLast = 0
    First = 1
    Last = 2
    EveryN = 3
    NEvenlySpaced = 4
    Null = 5

NUMBER_OF_SET_MANAGEMENT_OPTIONS = 6  

def display_management_strategies():
    print("This displays which files will be saved")
    print("0 - FirstAndLast")
    print("1 - First")
    print("2 - Last")
    print("3 - Every N Files")
    print("4 - N Evenly Spaced Files")
