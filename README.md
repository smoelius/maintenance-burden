# maintenance-burden

Calculate the _maintenance burden_ of each file in a git repository

This tool defines the _maintenance burden_ of a file to be:

- the total number of lines deleted from the file across all commits in the repository

For most files, this is the same as the total number of lines added minus the file's current number of lines. However, the two quantities can differ because of an incomplete git history, or git reporting that a file was renamed when it was not.

**Rationale.** Think of maintaining a vehicle. The parts that are still in the vehicle may provide value, but once a part is replaced, it is simply a cost. Preliminary experiments suggest this metaphor applies well to software.

## Example

Running `maintenance-burden` on its own repository produces the following output:

<!-- maintenance-burden-start -->

```
       0  .github/dependabot.yml
       0  .github/workflows/ci.yml
       0  .gitignore
       0  CHANGELOG.md
       0  src/options.rs
       0  tests/ci.rs
       1  Cargo.toml
       6  tests/dogfood.rs
      14  Cargo.lock
      29  README.md
      92  src/main.rs
```

<!-- maintenance-burden-end -->
