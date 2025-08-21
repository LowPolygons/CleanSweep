# CleanSweep
A python CLI tool to help remove old and large files on a system


## Installation

Clone the repository

```bash
git clone https://github.com/LowPolygons/CleanSweep
```

Install via pip

```bash
cd CleanSweep

pip install .
```


## Commands

CleanSweep has 6 commands


### Setup
Usage: `cleansweep setup` 

Run to setup up the files needed to use CleanSweep

Creates the storage diretory `.cleansweep` in your home directory


### Settings
Usage: `cleansweep settings --mode [reset, modify, display, display-defaults]` 

Run this command to:

##### MODIFY: This will open an Interactive Environmenmt where you can customise your settings. Alternatively, this is stored in a JSON file in `.cleansweep`

##### RESET: This will copy the default user settings into your active ones

##### DISPLAY: This will print your active user settings to the screen

##### DISPLAY-DEFAULTS: This will print the defaults to the screen. If you wish to change this, do it by editing the JSON directory in `.cleansweep` 


### Scan
Usage: `cleansweep scan --path [path]`

Run this command to scan for all files relative to your current directory, filter them based on your settings, and then save them to the corresponding file

Note: The --path command is optional and is relative to your current directory 


### List
Usage: `cleansweep list --choice [blacklisted, whitelisted]`

Run this command to print the files stored in the corresponding list

Note: If either the black/white list is empty, it will print a custom error/warning message implying you haven't run the setup command - this is not an installation issue, it just flagged no relevant files in the previous scan


### Purge
Usage: `cleansweep purge [--stage, --continue]`

Run this command to:

##### STAGE: Copy all files in your whitelisted list to a temporary file stored in your current directory to allow for last minute changes

##### CONTINUE: Load the temporary list created from the stage command, make final confirmations, delete the files


### Demolish
Usage: `cleansweep demolish`

Run to uninstall stored cleansweep data from your system
