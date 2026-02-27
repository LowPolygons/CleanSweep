# Functionality

There are some systems in CleanSweep that are not necessarily intuitive to understand. Read this guide to gain a better understanding of what happens when you run each command.

## Commands

To gain a better understanding of each command, read more below

- ### [Setup](setup.md)

- ### [Settings](settings.md)

- ### [Scan](scan.md)

- ### [Set-Scan](set_scan.md)

- ### [Manage-Sets](manage_sets.md)

- ### [Override](override.md)

- ### [Print-Hidden-Stats](print_hidden_stats.md)

### List

The list command will attempt to load the specified file - or the `ToDelete` list by default - and print each path on a separate line.

If the sets list is chosen, it will print the first item in the list of each set at the lowest identation level, then the full set one level of indentation greater.

If it cannot open the file, it will prompt you that the file may be empty.

### Purge

`stage` must run before `continue`

Stage will read in a ***copy*** of your ToKeep and ToDelete list into temporary files

It is the duty of the user to ensure that files are in the right places in these lists

`continue` will then read the temporary ToDelete list and these will be the files that are deleted after user confirmation

This means that you may find files in your Delete list, and simply removing the line from the list will prevent its deletion

Similarly, moving lines from the keep list into the delete like will cause their deletion

### Reset

Resetting the target list simply writes `{}` to the target file

### Demolish

After the user confirms their choice, the `~/.cleansweep` directory is deleted from the system. No backups are made.

