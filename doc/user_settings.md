# User Settings

## **[Back](functionality.md)**

```json
{
  "to_keep_list": {
    // Extension must not contain the .
    // If you want to flag files with no extension, add a ""
    "with_extension": [
      "z",
      "exe",
      "d"
    ],
    // This category is used for both the full name match filter and the name contains filter
    "name_contains": [
      "cleansweep"
    ],
    // As long as any of these substrings are found in the full path, it gets flagged
    "directory_contains": [
      "cleansweep"
    ],
    // Doesn't have to be a char, can be a strng
    "name_starts_with": [
      "."
    ],
    // In Bytes
    "larger_than": 10000000000,
    // For dates, you input the number of days since 'now'
    // In code, 'now' is not the date since this file was modified, it is the exact instance the command is run
    "modified_after": 0,
    "accessed_after": 0
  },

  // Same as above
  "to_delete_list": {
    "with_extension": [
      "out"
    ],
    "name_contains": [
      "OUTPUT",
      "HISTORY",
      "slurm-"
    ],
    "directory_contains": [
      "deleteme"
    ],
    "name_starts_with": [],
    "larger_than": 1001,
    "modified_after": 0,
    "accessed_after": 0
  },

  "set_scan_options": {
    // Again, do not include the . 
    "with_extension": [
      "txt"
    ],
    // The Set Scan will only use the NameContains filter
    "name_contains": [
      "test"
    ]
  }
}
```
