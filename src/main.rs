use anyhow::{Context, Result};
use log::debug;
use once_cell::sync::Lazy;
use regex::Regex;
use std::{
    borrow::Cow,
    collections::BTreeMap,
    fs::{read_to_string, symlink_metadata},
    io::{BufRead, BufReader},
    str::FromStr,
};
use subprocess::{Exec, Redirection};

fn main() -> Result<()> {
    env_logger::init();

    let mut lines_map = BTreeMap::<String, Option<usize>>::new();
    let mut added_map = BTreeMap::<String, usize>::new();
    let mut deleted_map = BTreeMap::<String, usize>::new();
    let mut rename_map = BTreeMap::<String, String>::new();

    let mut popen = Exec::cmd("git")
        .args(&["log", "--numstat", "--pretty="])
        .stdout(Redirection::Pipe)
        .popen()?;

    let reader = BufReader::new(popen.stdout.take().unwrap());

    for result in reader.lines() {
        let line = result.with_context(|| "Failed to read git output")?;

        let &[added, deleted, pathname] = line.split('\t').collect::<Vec<_>>().as_slice() else {
            panic!("Unexpected line: {line:?}");
        };

        assert_eq!(added == "-", deleted == "-");

        if added == "-" {
            continue;
        }

        let added = usize::from_str(added).unwrap();
        let deleted = usize::from_str(deleted).unwrap();

        let maybe_from_to = is_rename(pathname);

        let current_to_name = maybe_from_to
            .as_ref()
            .map_or(pathname, |[_, to_name]| to_name);

        let final_to_name = rename_map
            .get(current_to_name)
            .map_or(current_to_name, String::as_str);

        if lines_map.get(final_to_name).is_none() {
            let maybe_lines = file_lines(final_to_name)
                .map_err(|error| debug!("{error}"))
                .ok();
            lines_map.insert(final_to_name.to_owned(), maybe_lines);
        }

        if lines_map.get(final_to_name).unwrap().is_none() {
            continue;
        }

        *added_map.entry(final_to_name.to_owned()).or_default() += added;
        *deleted_map.entry(final_to_name.to_owned()).or_default() += deleted;

        if let Some([from_name, _]) = &maybe_from_to {
            rename_map.insert(from_name.to_string(), final_to_name.to_owned());
        }
    }

    let mut results = deleted_map
        .iter()
        .map(|(path, &deleted)| {
            let lines = lines_map.get(path).unwrap().unwrap();
            let added = *added_map.get(path).unwrap();
            (deleted, added as isize - lines as isize, path)
        })
        .collect::<Vec<_>>();

    results.sort_by_key(|&(deleted, _, _)| deleted);

    maybe_warn(&results);

    display_results(&results);

    Ok(())
}

static COMPLEX_RE: Lazy<Regex> = Lazy::new(|| Regex::new(r"\{(.*) => (.*)\}").unwrap());
static SIMPLE_RE: Lazy<Regex> = Lazy::new(|| Regex::new("^(.*) => (.*)$").unwrap());

fn is_rename(s: &str) -> Option<[Cow<str>; 2]> {
    if let Some(captures) = COMPLEX_RE.captures(s) {
        assert_eq!(3, captures.len());
        let start = captures.get(0).unwrap().start();
        let end = captures.get(0).unwrap().end();
        Some(
            [
                s[..start].to_owned() + &captures[1] + &s[end..],
                s[..start].to_owned() + &captures[2] + &s[end..],
            ]
            .map(|s| s.replace("//", "/"))
            .map(Cow::Owned),
        )
    } else if let Some(captures) = SIMPLE_RE.captures(s) {
        assert_eq!(3, captures.len());
        Some(
            [
                captures.get(1).unwrap().as_str(),
                captures.get(2).unwrap().as_str(),
            ]
            .map(Cow::Borrowed),
        )
    } else {
        None
    }
}

fn file_lines(path: &str) -> Result<usize> {
    let metadata =
        symlink_metadata(path).with_context(|| format!("Failed to get metadata for `{path}`"))?;

    // smoelius: Hack.
    if metadata.is_symlink() {
        return Ok(1);
    }

    let contents = read_to_string(path).with_context(|| format!("Failed to read `{path}`"))?;

    Ok(contents.lines().count())
}

fn maybe_warn(results: &[(usize, isize, &String)]) {
    if results
        .iter()
        .any(|&(deleted, diff, _)| deleted as isize != diff)
    {
        eprintln!(
            "\
Warning: For some files, the total number of lines deleted differs from the total number
of lines added minus the file's current number of lines. This could be due to an
incomplete git history, or to git reporting that a file was renamed when it was not.
"
        );
    }
}

fn display_results(results: &[(usize, isize, &String)]) {
    let width = results.iter().fold(0, |x, &(deleted, diff, _)| {
        std::cmp::max(x, diff_msg(deleted, diff).len())
    });

    for &(deleted, diff, path) in results {
        println!("{deleted:>8}{:>width$}  {path}", diff_msg(deleted, diff));
    }
}

fn diff_msg(deleted: usize, diff: isize) -> String {
    if deleted as isize == diff {
        String::new()
    } else {
        format!(" ({diff})")
    }
}
