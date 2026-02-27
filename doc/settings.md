# Settings

## **[Back](functionality.md)**

The settings define the various categories needed for filtering files in the various scans.

## Display

The display command will attempt to load the user setting list from the `.cleansweep` directory and print it to the screen.

If it cannot find the file, it should warn that the setup command may not have run.

## Reset

The Reset command will open both the defaults and current user settings files from the `.cleansweep` directory. 

The command should fail if one or both of these files are not found. Once again, it will prompt you to run the setup command.

It will then copy the defaults stored in the json file into your current settings file. 

This means that if you want to customise your default settings, you need to manually edit the defaults json file.

#### Note: if you demolish and re-setup your cleansweep, your modified defaults will *not* save

## Modify

The modify command will fail to run if it fails to open your settings.

The interactive environment will open the terminal found in your $EDITOR variable, with 3 backups in case one isn't set

Running the settings modify command will *not* adjust your existing Keep/Delete list to match the new settings. It is up to the user to do this.
