use anyhow::{bail, Result};
use std::process::exit;

#[derive(Default)]
pub struct Options {
    pub verbose: bool,
    pub paths: Vec<String>,
}

impl Options {
    pub fn parse() -> Result<Self> {
        let mut options = Self::default();
        for arg in std::env::args().skip(1) {
            if arg == "--help" || arg == "-h" {
                help();
            } else if arg == "--verbose" {
                options.verbose = true;
            } else if arg == "--version" || arg == "-V" {
                version();
            } else if arg.starts_with('-') {
                bail!("unexpected argument '{arg}' found");
            } else {
                options.paths.push(arg);
            }
        }
        Ok(options)
    }

    pub fn included_path(&self, path: &String) -> bool {
        self.paths.is_empty() || self.paths.contains(path)
    }
}

fn help() -> ! {
    println!(
        "\
Calculate the number of lines deleted for each file in a git repository

Usage: maintenance-burden [OPTIONS] [PATHS]...

Arguments:
  [PATHS]  Show the number of lines deleted for only the files at PATHS (the quantity
           is still calculated for each file in the repository)

Options:
      --verbose  Show the difference between the number of lines added and the current
                 number of lines if not equal to the number of lines deleted
  -h, --help     Print help
  -V, --version  Print version

For some files, the following two quantities may differ:

  - the number of lines deleted
  - the number of lines added minus the current number of lines

This can happen because of an incomplete git history, or because git reports that a
file was renamed when it was not. Passing --verbose shows the latter quantity in
parentheses next to the former."
    );
    exit(0);
}

fn version() -> ! {
    println!("maintenance-burden {}", env!("CARGO_PKG_VERSION"));
    exit(0);
}
