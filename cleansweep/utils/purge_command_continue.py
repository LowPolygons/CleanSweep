
from pathlib import Path
import time
from cleansweep.globals.storage_paths import StoragePaths

def purge_continue():
    # Load the to_delete file, print the files and amount of data to be deleted, run final confirmations, 10s delay and then delete
    staged_paths: list[Path] = []
    try:
        with open(StoragePaths.to_delete_local_temp_file_name, "r") as file:
            for line in file:
                staged_paths.append(Path(line.strip()))
    except Exception as err:
        print(f"There was an issue trying to prep staged files for deletion: {err}")

    # Print the staged paths and data to be deleted
    size_of_data_being_deleted: int = 0
    for curr_path in staged_paths:
        print(f"- {curr_path}")
        size_of_data_being_deleted += curr_path.stat().st_size
    print(f"\nAmount of data being deleted: {size_of_data_being_deleted} bytes")

    # Confirmation Stage
    print("Please enter 'confirm' for the following statements to confirm deletion (space/case sensitive):")

    confirm1 = input("\n - I confirm that I want these files to be deleted and that this is an irreversable action \n => ")
    confirm2 = input("\n - I confirm that these files were scanned using my most up-to-date user settings \n => ")

    if confirm1 != "confirm" or \
        confirm2 != "confirm":
        print("Confirmation staged failed. If this was an accident, re-run the command.")
        return

    # Deletion stage
    print("Confirmation successful. Purging will begin in 10 seconds, cancel any time before this to cancel")

    time.sleep(10)
    
    print("\n - Deleting Files..")
    for curr_path in staged_paths:
        try:
            curr_path.unlink()
        except Exception as err:
            print(f"Failed to delete {curr_path}: {err}, skipping")
            continue
    
    # Try delete the temp file
    try:
        Path(StoragePaths.to_delete_local_temp_file_name).unlink()
    except Exception as err:
        print(f"Failed to delete temporary {StoragePaths.to_delete_local_temp_file_name} file: {err}")
        return
    
    print("\n - Successfully deleted staged files. Enjoy the storage space!")
