# Functionality

There are some systems in CleanSweep that are not necessarily intuitive to understand. Read this guide to gain a better understanding of what happens when you run each command.

## Commands

To gain a better understanding of each command, read more below

- ### [Setup](setup.md)

- ### [Settings](settings.md)

- ### [Scan](scan.md)

- ### [Set-scan](set_scan.md)

- ### [Manage-Sets](manage_sets.md)

- ### [Override](override.md)

- ### [Print-Hidden-Stats](print_hidden_stats.md)

### List

The list command will attempt to load the specified file - or the `ToDelete` list by default - and print each path on a separate line.

If the sets list is chosen, it will print the first item in the list of each set at the lowest identation level, then the full set one level of indentation greater.

If it cannot open the file, it will prompt you that the file may be empty.

### Purge

### Reset

### Demolish

After the user confirms their choice, the `~/.cleansweep` directory is deleted from the system. No backups are made.

