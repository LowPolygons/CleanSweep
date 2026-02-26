# Set-Scan

Unlike the regular scan, the Set-Scan does not perform any filtering into a `ToKeep` or `ToDelete` list.

This scan utilises a new category, which can be detected based on these lines from the settings:

```rs
pub struct SetScanOptions {
    with_extension: Vec<String>,
    name_contains: Vec<String>,
}
```
It performs a similar recursive scan, getting all paths, and only grabs files that meet *at least one* of the above sub-categories.

Of the files which meet at least one of the above critera, treated as 'Maybe in a set', it goes through a four step process:


## Step 1

For each file, it will split it into its `dir`, `stem` and `extension`

Each file is then inserted to a hashmap, where the `dir` is the key.

In plain english, this groups together files which are in the same directory

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

```rs
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

## Step 2

Iterating over the directories (keys), it splits the file name into the string portion, and the number portion, stored as a string.

This is done through the regex: `\d+(\.\d+)?$`

Any files which do not meet this format are ruled out as not being part of a set.

As this is done on a per-directory basis, the new structure is just a list of the current files.

The new tuples also store the original full name

In the above example, the `test1` list will look like this:

```rs
[
  ("file", "1", "txt", "file1"),
  ("file", "2", "txt", "file2"),
  ("file", "3", "txt", "file3"),
  ("file_test", "4", "txt", "file_test4"),
  ("file_test", "5", "txt", "file_test5"),
  ("file_test", "6", "txt", "file_test6")
]
```

### Step 3 

Some directories could contain multiple sets. As a result, the program searches for any 'unique string portions',

It then creates a new HashMap, mapping USPs to the files within that USP.

The example set list for this directory becomes:

```rs
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

### Step 4

Finally, per USP, the files are then re-pieced together back to the full file path including the directory, and stored in a SetsReadWriteType

```rs
pub struct SetsReadWriteType {
    pub files: Vec<String>,
}
```

```rs
created_sets: Vec<SetsReadWriteType> = {
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
}
```

This structure is saved in `~/.cleansweep/found_sets.json`
