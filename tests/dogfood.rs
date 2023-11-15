use assert_cmd::Command;
use similar_asserts::SimpleDiff;
use std::{
    env::var,
    fs::{read_to_string, write},
    io::Result,
};

// smoelius: I experimented with computing the README.md's changes directly, rather than iterating.
// But things get tricky when the files' ordering changes.
const MAX_RETRIES: u8 = 4;

const START: &str = "<!-- maintenance-burden-start -->\n\n```\n";
const END: &str = "```\n\n<!-- maintenance-burden-end -->\n";

#[test]
fn dogfood() {
    for _ in 0..MAX_RETRIES {
        let output = Command::cargo_bin("maintenance-burden")
            .unwrap()
            .output()
            .unwrap();
        let stdout = std::str::from_utf8(&output.stdout).unwrap();

        let readme_actual = read_to_string("README.md").unwrap();

        let start = readme_actual.find(START).unwrap();
        let end = readme_actual.find(END).unwrap();

        let readme_expected =
            readme_actual[..start + START.len()].to_owned() + stdout + &readme_actual[end..];

        if readme_expected == readme_actual {
            return;
        }

        if enabled("BLESS") {
            assert!(
                clean().unwrap(),
                "BLESS is enabled but repository is not clean"
            );

            Command::new("git")
                .args(["log", "-1", "--pretty=%s"])
                .assert()
                .try_stdout("Update README.md\n")
                .expect(
                    "BLESS is enabled but last commit message is not `Update README.md`. Try the \
                     following command, then rerun:
    git commit --allow-empty -m Update\\ README.md
",
                );

            write("README.md", readme_expected).unwrap();

            Command::new("git")
                .args(["add", "README.md"])
                .assert()
                .success();

            Command::new("git")
                .args(["commit", "--amend", "--no-edit"])
                .assert()
                .success();
        } else {
            panic!(
                "{}",
                SimpleDiff::from_str(&readme_expected, &readme_actual, "left", "right")
            );
        }
    }

    panic!("Exceeded MAX_RETRIES ({MAX_RETRIES})");
}

fn clean() -> Result<bool> {
    const ARGS: [&[&str]; 2] = [&[], &["--cached"]];

    for args in ARGS {
        let output = Command::new("git")
            .args(["diff", "--exit-code"])
            .args(args)
            .output()?;

        if !output.status.success() {
            return Ok(false);
        }
    }

    Ok(true)
}

fn enabled(key: &str) -> bool {
    var(key).map_or(false, |value| value != "0")
}
