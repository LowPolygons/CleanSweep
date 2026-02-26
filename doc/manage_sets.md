
# Manage-Sets

Running this command will open an interactive environment where a list of all sets will print to the screen.

Each set is represented by the first file in the set

You can manually go through them and specify how they should be managed, or set a global setting.

## Usage
After you have selected either Default or any single set, you will be prompted to choose how the new Management style will be applied, from the list of:
```rs
pub enum ChoiceInGettingStyle {
    Append,
    Reset,
    Copy,
    Set,
}
```
#### Append -> Add to the list of management styles currently stored

#### Reset -> Remove all management styles currently stored

#### Copy -> Pick another set and overwrite the currently stored styles

#### Set -> Overrwrite with just one new management style

Alternatively, you can choose to preview how the currently stored styles will filter the chosen set (not available when choosing a default)

Note: When choosing a default, the chosen management style is applied directly to each set immediately, not at the end

## The Different Management Methods

### First 
Only the first file is added to the keep list.

### Last
Only the last file is added to the keep list.

### FirstAndLast
Only the first and last file will be added to the keep list. The rest will be added to the delete list.

### FirstN
Only the first 'N' files are added to the keep list

### LastN
Only the last 'N' files are added to the keep list

### FirstNAndLastM
Only the first 'N' and last 'M' files are added to the keep list

### EveryN
Every 'N' files, starting from the **last** file in the sequence will be kept.

For example, the set:

```sh
test0.txt
test1.txt
test2.txt
test3.txt
test4.txt
test5.txt
test6.txt
test7.txt
```

With an `N` of 2 will add the following to the keep list:

```sh
test7.txt
test5.txt
test3.txt
test1.txt
```

### NEvenlySpaced
`N` files, evenly distributed across the set, including the first and last file will be added to the keep list.

If `N` is 1, it will only add the **last** file to the keep list.

As N gets larger, particularly if N exceeds half the sets length, it may become more of an approximation

## Important notes

When stacking multiple management styles, they will be applied in the order you pass htem

Example:
```rs
..test1.txt
..test2.txt
..test3.txt
..test4.txt
..test5.txt
..test6.txt
..test7.txt
..test8.txt
..test9.txt
..test10.txt
..test11.txt
..test12.txt
..test13.txt
..test14.txt
..test15.txt
```

Chosen styles:
```FirstandLast -> FirstN(3), NEvenlySpaced(5)```

Result:

```rs 
// First and Last 
..test1.txt
..test15.txt
// First Three
..test2.txt
..test3.txt
..test4.txt
// N Evenly Spaced (5) 
..test5.txt
..test7.txt
..test9.txt
..test11.txt
..test14.txt
```

The management styles will be applied upon exit and will override your current ToKeep/ToDelete list.

As a result, if you make a mistake in the session, you can just rerun manage-sets straight away and restart
