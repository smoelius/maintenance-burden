# maintenance-burden

Calculate the number of lines deleted for each file in a git repository

The purpose of this tool is to help you judge where you might be spending time on your software in ways that aren't providing value.

Put simply, a line that is deleted can no longer provide value. Moreover, deleting a line takes effort, e.g., in deciding whether to delete the line. This is on top of the effort required to add the line in the fist place. Hence, if one is looking to reduce their software's maintenance costs, they might scrutinize files with large numbers of deleted lines.

**Note:** For most files, the number of lines deleted is the same as the number of lines added minus the file's current number of lines. However, the two quantities can differ because of an incomplete git history, or because git reports that a file was renamed when it was not.

## Example

Running `maintenance-burden` on its own repository produces the following output:

<!-- maintenance-burden-start -->

```
       0  .github/dependabot.yml
       0  .github/workflows/ci.yml
       0  .gitignore
       0  tests/ci.rs
       2  CHANGELOG.md
       2  Cargo.toml
       6  tests/dogfood.rs
       7  src/options.rs
      15  Cargo.lock
      44  README.md
     104  src/main.rs
```

<!-- maintenance-burden-end -->
