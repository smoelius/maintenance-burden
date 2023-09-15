# maintenance-burden

A simple tool to calculate the _maintenance burden_ of each file in a git repository

We define the _maintenance burden_ of a file to be the difference between the following two quantities:

- the total number of lines added to the file across all commits in the repository
- the file's current number of lines

So, for example, if a file has only ever had lines added (never deleted), it will have a maintenance burden of 0.

The following is the output currently produced by running `maintenance-burden` on its own repository:

<!-- maintenance-burden-start -->

```

```

<!-- maintenance-burden-end -->
