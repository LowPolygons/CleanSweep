# CleanSweep
A CLI to help remove old and large files on a system


## Installation

Install Cargo on your machine

```sh
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

Clone the repository

```sh
git clone https://github.com/LowPolygons/CleanSweep
```

Install via cargo

```sh
cd CleanSweep

cargo build -r
```

### Recommended

Add the binary to your path or create an alias
```sh
PATH = $PATH:[your install path]/CleanSweep/target/release/cleansweep
```

## Commands

[More information on how they work](FUNCTIONALITY.md)

### Setup
```sh
cleansweep setup
``` 

Run to setup up the files needed to use CleanSweep

Creates the storage diretory `.cleansweep` in your home directory


### Settings
```sh
cleansweep settings [reset, modify, display] 
```

Run this command to:

- MODIFY: This will open an Interactive Environmenmt where you can customise your settings. Alternatively, this is stored in a JSON file in `.cleansweep`

- RESET: This will copy the default user settings into your active ones

- DISPLAY: This is followed by one of two parameters - `["current", "defaults"]`, and displays to the screen the corresponding settings


### Scan
```sh
cleansweep scan 
    [--path [path]]
    [--use-custom-filters]
    [--no-filter]
    [--ignore-dirs [values]]
```

Run this command to scan for all files relative to your current directory, filter them based on your settings, and then save them to the corresponding file


#### `--path`
More control on where the scan begins, relative to your current directory

#### `--use-custom-filters`
Instead of using the full set of filters, you can list only the filters you want to use in `$HOME/.cleansweep/filter_components.txt`

See [here](FUNCTIONALITY.md) for more information on how your inputs here work

#### `--no-filter`
Performing the scan will automatically place every found file in the to-keep list

Note: recommended when using in tandom with `override` command

#### `--ignore-dirs`
Takes in any number of strings, and any directories which contain any of these substrings will be ignored in the scan. 

Useful example: ` --ignore-dirs .git venv bin target`

### Set-Scan
```sh
cleansweep set-scan 
    [--path [path]]
    [--ignore-dirs [values]]
```

Run this command to scan from the current or provided directory for any sets (files with the same name, differing only by a number) and store for later filtering.

#### `--path`
More control on where the scan begins, relative to your current directory

#### `--ignore-dirs`
Takes in any number of strings, and any directories which contain any of these substrings will be ignored in the scan. 

### List
```sh
cleansweep list --choice [to_keep, to_delete, sets]
    [--summarise]
```

Run this command to print the files stored in the corresponding list

#### `--choice`
Chooses the list to print, defaulting to `to-delete`

#### `--summarise`
Prints a summary of how many files ammounting to how much data in each list.

Note: using the `--sumarise` command in tandom with `--choice` will not change the `--summarise` behaviour


### Manage-Sets
```sh
cleansweep manage-sets
    [--short-mode]
```

Run this command to enter an interactive environment where you can choose how CleanSweep will filter each sets.

Note: when finalising this command, it will override your existing ToKeep/ToDelete list

#### `--short-mode`
When previewing the effects of your management styles per set, this argument will truncate the first 80% of the listed paths to reduce clutter

### Reset
```sh
cleansweep reset --choice [to_keep, to_delete, sets]
```

Run this command to reset the chosen list, or the to-delete list if non specified

### Purge
```sh
cleansweep purge [stage, continue]
```

Run this command to:

- STAGE: Copy your lists to temporary files stored in your current directory to allow for last minute changes and sanity checks

- CONTINUE: Load the temporary list created from the stage command, make final confirmations, delete the files

### Override 
```sh
cleansweep override 
    --list [to-keep, to-delete]
    --filter [filter name]
    [values]

Example:

cleansweep override --list to-keep --filter extension out txt png
```

Run this command to move any files in your chosen list to the opposing list based on your passed parameters.

#### `--list`
Choose which list files will be potentially moved **from**

#### `--filter [filter name] [values]`
Input the name of a filter and the values it will use. For information on how your input is parsed,
look [here](FUNCTIONALITY.md)

### Print-Hidden-Stats
```sh
cleansweep print-hidden-stats
    [--path [path]]
    [--recursive]
    [--ignore-dirs [values]]
```
Run to open an environment where the files scanned in your immediate (or recursive with `--recursive`) directory are listed, allowing you to easily see the hidden information relevant to the program, such as its size, last-modify date or last-access date 

#### `--path`
Optional arg to specify where the scan should start relative to your current directory

#### `--recursive`
Used to specify if you want the scan to be recursive. If not provided, it defaults to your immediate directory

#### `--ignore-dirs`
Takes in any number of strings, and any directories which contain any of these substrings will be ignored in the scan if `--recursive` is used 

### Demolish
```sh
cleansweep demolish
```

Run to uninstall stored cleansweep data from your system
