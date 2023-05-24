// SPDX-License-Identifier: GPL-3.0

use crate::Errors;
use std::collections::HashSet;
use std::fs::{DirEntry, File};
use std::io::{self, BufRead};
use std::path::{Path, PathBuf};

fn parse_ignore_file(
    names: &mut HashSet<PathBuf>,
    path: &DirEntry,
    comment: &str,
) -> Result<(), Errors> {
    // read the file contents
    if let Ok(lines) = read_lines(path.path()) {
        for line in lines {
            let l = line.unwrap();
            if !l.starts_with(comment) {
                let path = PathBuf::from(l);
                // ensure the path in there file exists, if it doesn't than just ignore
                if path.exists() {
                    names.insert(if path.is_relative() {
                        path.canonicalize()
                            .map_err(|_| Errors::CantCanonicalize { cause: path })?
                    } else {
                        path
                    });
                }
            }
        }
    }

    Ok(())
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

pub fn parse_for_ignores(
    paths: &mut HashSet<PathBuf>,
    entries: &Vec<DirEntry>,
) -> Result<(), Errors> {
    for entry in entries {
        let name = entry.file_name();
        if name == ".gitignore" {
            // if there is a .gitignore there is probably a .git
            let git_path = PathBuf::from(".git");
            // this check is done for scenario where .gitignore exists
            // but not the .git directory
            if git_path.exists() {
                let full_path = git_path
                    .canonicalize()
                    .map_err(|_| Errors::CantCanonicalize { cause: git_path })?;
                paths.insert(full_path);
            }
            let comment = "#";
            parse_ignore_file(paths, entry, comment)?;
        }
        if name == ".ignore" {
            let comment = "//";
            parse_ignore_file(paths, entry, comment)?;
        }
    }

    Ok(())
}

pub fn check_match(hs: &HashSet<PathBuf>, check: &PathBuf) -> bool {
    if hs.contains(check) {
        return true;
    }
    return false;
}
