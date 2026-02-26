
# Scan

From the current or provided filepath, it will run a recursive scan for every single file.

This is handled by the FileScanner system. For information on how this works, read [here](todo.md)

These file paths are then converted into `FileContainer` objects, storing some `FileStatistics`

```rs
pub struct FileContainer {
    path: PathBuf,
    statistics: FileStatistics,
}

pub struct FileStatistics {
    name: String,
    size: u64,
    extension: String,
    last_accessed: FileDateData,
    last_modified: FileDateData,
    directory: String,
}
```

The program will load the user settings and then begin the filtering process

## Filtering

In CleanSweep, there are two main categories: `ToKeep` and `ToDelete`. 

The filtering process follows a hierarchical process

### For a file to be labelled as any single category, it must:

- Match *at least* one sub-category in the target category

- Match *exactly zero* sub-categories in a higher-importance category

- Files which do not explicitly fit in a category will be automatically categorised as ToKeep

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

### `/home/test/cleany-debug.txt`  | `ToKeep`
Whilst the file has a `ToDelete` file extension and name-contains sub-category, it meets a `ToKeep` sub-category, therefore making it `ToKeep`

### `/home/deletemelater/file.out` | `ToDelete`
The file may not contain `debug` or be a text file, but it contains the keyword `delete` in its path, whilst meeting no `ToKeep` sub-categories.

### `/home/program/metadata.data`  | `ToKeep`
It meets no sub-categories from either ToKeep or ToDelete, thus making it get automatically put in ToKeep

```rs
let mut filter_code_results: Vec<FilterCodes> = Vec::new();

for filter_category in &self.filterers {
  /* returns filter result into filter_code_results */
}

if filter_code_results.contains(&FilterCodes::ToKeep) {
    return Ok(FilterCodes::ToKeep);
}
if filter_code_results.contains(&FilterCodes::ToDelete) {
    return Ok(FilterCodes::ToDelete);
}

Ok(FilterCodes::ToKeep)
```

## Saving

Following the filtering process, the program will override any pre-existing ToKeep/ToDelete list and store the new ones.

