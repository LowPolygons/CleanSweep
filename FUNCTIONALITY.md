# Functionality

There are some systems in CleanSweep that are not necessarily intuitive to understand. Read this guide to gain a better understanding of what happens when you run each command.

## Setup
```sh
cleansweep setup
```

CleanSweep stores a global structure called `StoragePaths` which holds all the file paths needed by the program. 

```py
@dataclass
class StoragePaths:
    path_to_home: Path
    main_dir_name: str = ".cleansweep"

    to_delete_file_name: str = "to_delete_files.json"
    to_keep_file_name: str = "to_keep_files.json"
    minimum_flagged_file_name: str = "minimum_flagged_files.json"

    user_settings_file_name: str = "user_settings.json"
    log_file_name: str = "log.txt"
    storage_paths_file_name: str = "storage_paths.json"    
    user_settings_defaults_file_name: str = "user_settings_defaults.json"
    to_delete_local_temp_file_name: str = "STAGED_FOR_DELETION"

    found_sets_file_name: str = "sets_that_were_found.json"
```

When running setup, a folder in your home directory called `.cleansweep` is made, and all the necessary files to get cleansweep going are initialised. 

Those files are:

- To-Delete List => initialises to `[]`
- To-Keep List   => initialised to `[]`
- User-Settings  => initialised to `User-Settings-Defaults`
- Log-File => Keeps a record of all commands run. Mainly a dev tool
- User-Settings-Defaults => initialised to hard-coded settings:

```md
1 - Date-cutoff-time for flagging files: [100 days before current date] 
Files to Keep:
2 - ..with extension: ['z', 'exe', 'd']
3 - ..name contains: ['EXAMPLE_NAME_CONTAINS:', 'cleansweep']
4 - ..directory contains: ['EXAMPLE_DIRECTORY_CONTAINS:', 'cleansweep']
5 - ..name starts with: ['.']
6 - ..smaller than: 1000 bytes
7 - ..larger than: 10000000000 bytes
Files to Delete:
8 - ..with extension: ['out']
9 - ..name contains: ['OUTPUT', 'HISTORY', 'slurm-']
10 - ..name starts with: ['deleteme']
11 - ..directory contains: []
12 - ..larger than: 1001 bytes

13 - Consider Access Date when Filtering: False
Files which may be in a set:
14 - .. with extension: ['h5']
15 - .. name contains: ['test_']
```

The defaults can be seen at any point using 
```sh
cleansweep settings display defaults
```

## Settings

The settings define the various categories needed for filtering files in the various scans.

### Display

The display command will attempt to load the chosen setting list from the `.cleansweep` directory and print it to the screen.

If it cannot find the file, it will warn that the setup command may not have run.

### Reset

The Reset command will open both the defaults and current user settings files from the `.cleansweep` directory. 

The command will fail if one or both of these files are not found. Once again, it will prompt you to run the setup command.

It will then copy the defaults stored in the json file into your current settings file. 

This means that if you want to customise your default settings, you need to manually edit the defaults json file.

#### Note: if you demolish and re-setup your cleansweep, your modified defaults will *not* save

### Modify

The modify command will fail to run if it fails to open your settings.

The interactive environment is self explanatory and provides information on how to operate it.

Running 
```sh
cleansweep display
``` 
will print your user settings at any point in the process

Running
```sh
cleansweep finish
```
will gracefully end your session at any point.

If you fail to exit the session through the finish command, none of your changes in the current environment session will save.

Running the settings command will *not* adjust your existing Keep/Delete list to match the new settings. It is up to the user to do this.

## Scan

From the current or provided filepath, it will run a recursive scan for every single file. This can take some time.

These file paths are then converted into `FileItem` objects, storing some `FileStatistics`

```py
class FileItem:
    def __init__(self, item_path: Path) -> None:
        self.__path: Path = item_path
        self.__stats: FileStatistics = FileStatistics()

@dataclass 
class FileStatistics:
    name: str
    size: int
    extension: str
    last_accessed: date # Optional, disabled by default
    last_modified: date
```

The program will load the user settings and then begin the filtering process

### Filtering

In CleanSweep, there are two main categories: `ToKeep` and `ToDelete`. There is a third `NonSpecial` category.

The filtering process follows a hierarchical process, in the order of:

- `ToKeep`     Most Importance
- `ToDelete`
- `NonSpecial` Least Importance

#### For a file to be labelled as any single category, it must:

- Match *at least* one sub-category in the target category

- Match *exactly zero* sub-categories in a higher-importance category

For example, imagine a set of user settings that contained this information

```md
ToKeep {
  ..with extension ["test"]
  ..name contains ["cleany"]
  ..dir contains ["sweepy"]
}
ToDelete {
  ..with extension ["txt"]
  ..name contains ["debug"]
  ..dir contains ["delete"]
}
```

Consider the following files and their final flag status

#### `/home/test/cleany-debug.txt`  | `ToKeep`
Whilst the file has a `ToDelete` file extension and name-contains sub-category, it meets a `ToKeep` sub-category, therefore making it `ToKeep`

#### `/home/deletemelater/file.out` | `ToDelete`
The file may not contain `debug` or be a text file, but it contains the keyword `delete` in its path, whilst meeting no `ToKeep` sub-categories.

#### `/home/program/metadata.data`  | `NonSpecial`
NonSpecial outlines no sub-categories to match; however, it meets no sub-categories from a higher importance category, thus making it NonSpecial

```py
name_status = file.filter_name(...
size_status = file.filter_size(...
path_status = file.filter_path(...
extension_status = file.filter_extension(...

if name_status == FilterCodes.ToKeep or \
    size_status == FilterCodes.ToKeep or \
    path_status == FilterCodes.ToKeep or \
    extension_status == FilterCodes.ToKeep:
    return FilterCodes.ToKeep 

if name_status == FilterCodes.ToDelete or \
    size_status == FilterCodes.ToDelete or \
    path_status == FilterCodes.ToDelete or \
    extension_status == FilterCodes.ToDelete:
    return FilterCodes.ToDelete 

return FilterCodes.NotSpecial
```

### Saving

Following the filtering process, the program will override any pre-existing ToKeep/ToDelete list and store the new ones.


## Set-Scan

Unlike the regular scan, the Set-Scan does not perform any filtering into a `ToKeep` or `ToDelete` list.

This scan utilises a new category, which can be detected based on these lines from the settings:

```py
set_may_have_extension: list[str]
set_file_name_may_contain: list[str]
```

It performs a similar recursive scan, getting all paths, and only grabs files that meet *at least one* of the above sub-categories.

Then of the remaining files, it splits the paths into its 'stem', 'name', and 'extension' and creates a dictionary, with the stem as the key and the names and extension as a list of values.

For example, the list of paths:

```sh
test1/file1.txt
test1/file2.txt
test1/file3.txt
test1/file_test4.txt
test1/file_test5.txt
test1/file_test6.txt
test1/edgecase.txt
test2/file38.txt
test2/file39.txt
test2/file41.txt
test2/edgecase.txt
```

Will become: 

```py
{
  "test1" : [
    ("file1", "txt"), 
    ("file2", "txt"), 
    ("file3", "txt"),
    ("file_test4", "txt"), 
    ("file_test5", "txt"), 
    ("file_test6", "txt"), 
    ("edgecase", "txt")
  ],
  "test2" : [
    ("file38", "txt"), 
    ("file39", "txt"), 
    ("file41", "txt"), 
    ("edgecase", "txt")
  ]
}
```

Then, iterating over the directories (keys), it splits the file name into the string portion, and the number portion, stored as a string.

This is done through the regex: `\d+(\.\d+)?$`

Any files which do not meet this format are ruled out as not being part of a set.

As this is done on a per-directory basis, the new structure is just a list of the current files.

In the above example, the `test1` list will look like this:

```py
[
  ("file", "1", "txt"),
  ("file", "2", "txt"),
  ("file", "3", "txt"),
  ("file_test", "4", "txt"),
  ("file_test", "5", "txt"),
  ("file_test", "6", "txt")
]
```

Next, all unique string portions (USPs) are extracted from the list, and a new dict is created storing a list of each matching USP:

```py
{
  "file" : [
    ("file", "1", "txt"),
    ("file", "2", "txt"),
    ("file", "3", "txt"),
  ],
  "file_test" : [
    ("file_test", "4", "txt"),
    ("file_test", "5", "txt"),
    ("file_test", "6", "txt")
  ]
}
```

This way, lists in the same directory with similar names can be isolated from each other.

The lists are ordered based on the number portion (cast to a float). 

Finally, they are then re-pieced together back to the full file path, and stored as a set in a list

```py
created_sets = [
  [
    "test1/file1.txt",
    "test1/file2.txt",
    "test1/file3.txt"
  ],
  [
    "test1/file_test4.txt",
    "test1/file_test5.txt",
    "test1/file_test6.txt"
  ],
  [
    "test2/file1.txt",
    "test2/file2.txt",
    "test2/file3.txt"
  ]
]
```

This structure is saved in `~/.cleansweep/sets_that_were_found.json`

## List

The list command will attempt to load the specified file - or the `ToDelete` list by default - and print each path on a separate line.

If the sets list is chosen, it will print the first item in the list of each set at the lowest identation level, then the full set one level of indentation greater.

If it cannot open the file, it will prompt you that the file may be empty.

## Manage-Sets

Running this command will open an interactive environment where a list of all sets will print to the screen.

Each list is numbered, and represented by the first file in the set.

You can manually go through them and specify how they should be managed, or set a global setting.

### The Different Management Methods

#### FirstAndLast - Default
Only the first and last file will be added to the keep list. The rest will be added to the delete list.

#### First 
Only the first file is added to the keep list.

#### Last
Only the last file is added to the keep list.

#### EveryN
Every 'N' files, starting from the **last** file in the sequence will be kept.

For example, the set:

```sh
test0.txt
test1.txt
test2.txt
test3.txt
test4.txt
test5.txt
test6.txt
test7.txt
```

With an `N` of 2 will add the following to the keep list:

```sh
test7.txt
test5.txt
test3.txt
test1.txt
```

#### NEvenlySpaced
`N` files, evenly distributed across the set, including the first and last file will be added to the keep list.

If `N` is 1, it will only add the **last** file to the keep list.

### Finalisation

The session is gracefully terminated by running 
```sh
cleansweep finish
```

In this stage, the `ToKeep` and `ToDelete` list are overriden with the processed files from the sets and saved.

## Purge

### --stage

In the stage section, all files loaded in the `ToDelete` file will be added to a new text file called `STAGED_FOR_DELETION`

The user should then look through this file and perform a final sanity check on the files to be deleted. They can add/subtract files as they wish.

### --continue

Final confirmations are made before the termination begins. There is a 10 second delay after the final confirmation before any deletion begins.

#### Note: CleanSweep does not delete the list of files stored in the `ToDelete` json. It deletes the list in `STAGED_FOR_DELETION`

After the purge, the `ToDelete` list is reset.


## Demolish

After the user confirms their choice, the `~/.cleansweep` directory is deleted from the system. No backups are made automatically.

## Activate-Override

The session attempts to load the `ToKeep` and `ToDelete` arrays.

Based on your override parameters, any parameters listed in `ToKeep` will be prompty moved to the `ToDelete` list.

#### Note: If you make a mistake, rerunning the 'scan' command will override anything in the two lists

#### Advice: This should be used to move very select few files. 

#### -  It is recommended that when moving a precise list of files, set the override size limit to something absurd - One million/billion bytes

#### -  This will prevent any files which only meet the size requirement from being included
