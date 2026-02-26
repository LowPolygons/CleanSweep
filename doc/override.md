# Override

The override command will load the list of files you provide, apply a filter you pass and subsequently move any files which match to the opposing list.

Example: I no longer want to store any image files which may have previously been flagged as ToKeep

```sh
cleansweep override --list to-keep --filter extension png jpg 
```

Any files stored in your to-keep list which have a png or jpx extension get moved to the delete list, **NOT** vice versa.

This is a one way transaction

This command will not rescan for any missed files
