# CleanSweep
A CLI to help remove old and large files on a system

This CLI can filter and delete files based on any combination of:

- Name
- Directory
- Size
- Last Modified Date
- Last Access Date
- Whether it is in a set (E.G Simulation Snapshots)


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

[More information on how they work](doc/functionality.md)

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

Run this command to modify, reset, or display your settings

### Scan
```sh
cleansweep scan 
    [--path [path]]
    [--use-custom-filters]
    [--no-filter]
    [--ignore-dirs [values]]
```

Run this command to scan for all files relative to your current directory, filter them based on your settings, and then save them to the corresponding file

### Set-Scan
```sh
cleansweep set-scan 
    [--path [path]]
    [--ignore-dirs [values]]
```

Run this command to scan from the current or provided directory for any sets (files with the same name, differing only by a number) and store for later filtering.

### List
```sh
cleansweep list --choice [to_keep, to_delete, sets]
    [--summarise]
```

Run this command to print the files stored in the corresponding list

### Manage-Sets
```sh
cleansweep manage-sets
    [--short-mode]
```

Run this command to enter an interactive environment where you can choose how CleanSweep will filter each sets.

Note: when finalising this command, it will override your existing ToKeep/ToDelete list

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

- `stage`: Copy your lists to temporary files stored in your current directory to allow for last minute changes and sanity checks

- `continue`: Load the temporary list created from the stage command, make final confirmations, delete the files

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

### Print-Hidden-Stats
```sh
cleansweep print-hidden-stats
    [--path [path]]
    [--recursive]
    [--ignore-dirs [values]]
```
Run to open an environment where the files scanned in your immediate (or recursive with `--recursive`) directory are listed, allowing you to easily see the hidden information relevant to the program, such as its size, last-modify date or last-access date 

### Demolish
```sh
cleansweep demolish
```

Run to uninstall stored cleansweep data from your system
