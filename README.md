# CleanSweep
A python CLI tool to help remove old and large files on a system


## Installation

Clone the repository

```sh
git clone https://github.com/LowPolygons/CleanSweep
```

Install via pip

```sh
cd CleanSweep

pip install .
```


## Commands

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
cleansweep scan --path [path]
```

Run this command to scan for all files relative to your current directory, filter them based on your settings, and then save them to the corresponding file

Note: The --path command is optional and is relative to your current directory 



### List
```sh
cleansweep list --choice [blacklisted, whitelisted]
```

Run this command to print the files stored in the corresponding list

Note: If either the black/white list is empty, it will print a custom error/warning message implying you haven't run the setup command - this is not an installation issue, it just flagged no relevant files in the previous scan



### Purge
```sh
cleansweep purge [--stage, --continue]
```

Run this command to:

- STAGE: Copy all files in your whitelisted list to a temporary file stored in your current directory to allow for last minute changes

- CONTINUE: Load the temporary list created from the stage command, make final confirmations, delete the files



### Demolish
```sh
cleansweep demolish
```

Run to uninstall stored cleansweep data from your system
