use anyhow::{Context, Result};
use log::debug;
use once_cell::sync::Lazy;
use regex::Regex;
use std::{
    borrow::Cow,
    collections::BTreeMap,
    fs::{read_to_string, symlink_metadata},
    io::BufRead,
    str::FromStr,
};
use subprocess::{Exec, Redirection};

fn main() -> Result<()> {
    env_logger::init();

    let mut lines_map = BTreeMap::<String, Option<usize>>::new();
    let mut added_map = BTreeMap::<String, usize>::new();
    let mut rename_map = BTreeMap::<String, String>::new();

    let mut popen = Exec::cmd("git")
        .args(&["log", "--numstat", "--pretty="])
        .stdout(Redirection::Pipe)
        .popen()?;

    let reader = std::io::BufReader::new(popen.stdout.take().unwrap());

    for result in reader.lines() {
        let line = result.with_context(|| "Failed to read git output")?;

        let &[added, _deleted, pathname] = line.split('\t').collect::<Vec<_>>().as_slice() else {
            panic!("Unexpected line: {line:?}");
        };

        if added == "-" {
            continue;
        }

        let added = usize::from_str(added).unwrap();

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

        if let Some([from_name, _]) = &maybe_from_to {
            rename_map.insert(from_name.to_string(), final_to_name.to_owned());
        }
    }

    let mut results = added_map
        .iter()
        .map(|(path, &added)| {
            let lines = lines_map.get(path.as_str()).unwrap().unwrap();
            (added as isize - lines as isize, path)
        })
        .collect::<Vec<_>>();

    results.sort_by_key(|&(score, _)| score);

    maybe_warn(&results);

    for (score, path) in results {
        println!("{score:>8}  {path}");
    }

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

fn maybe_warn(results: &[(isize, &String)]) {
    if results.iter().next().map_or(false, |&(score, _)| score < 0) {
        eprintln!(
            "\
Warning: For some files, the number of lines added was less than the file's current number of
lines. This was likely caused by git reporting that a file was renamed, when it was not."
        );
    }
}
