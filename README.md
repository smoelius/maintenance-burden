# maintenance-burden

**Calculate the number of lines deleted for each file in a git repository**

The purpose of this tool is to help you judge where you might be spending time on your software in ways that aren't providing value.

Put simply, a line that is deleted can no longer provide value. Moreover, deleting a line takes effort, e.g., in deciding whether to delete the line. This is on top of the effort required to add the line in the fist place. Hence, if one is looking to reduce their software's maintenance costs, they might scrutinize files with large numbers of deleted lines.

## Example

Running `maintenance-burden` on its own repository produces the following output:

<!-- maintenance-burden-start -->

```
       0  .github/dependabot.yml
       0  .gitignore
       0  rustfmt.toml
       1  .github/workflows/dependabot.yml
       1  tests/ci.rs
       2  CHANGELOG.md
       3  .github/workflows/ci.yml
       8  Cargo.toml
       8  src/options.rs
      18  tests/dogfood.rs
     104  src/main.rs
     107  README.md
     119  Cargo.lock
```

<!-- maintenance-burden-end -->

## Usage

```
Usage: maintenance-burden [OPTIONS] [PATHS]...

Arguments:
  [PATHS]  Show the number of lines deleted for only the files at PATHS (the quantity
           is still calculated for each file in the repository); see also --exclude

Options:
      --exclude  Show the number of lines deleted for all files except those at PATHS,
                 instead of only those at PATHS
      --verbose  Show the difference between the number of lines added and the current
                 number of lines if not equal to the number of lines deleted
  -h, --help     Print help
  -V, --version  Print version

For some files, the following two quantities may differ:

  - the number of lines deleted
  - the number of lines added minus the current number of lines

This can happen because of an incomplete git history, or because git reports that a
file was renamed when it was not. Passing --verbose shows the latter quantity in
parentheses next to the former when they differ.
```
