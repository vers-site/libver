use std::collections::{BTreeMap, BTreeSet};
use std::ffi::OsStr;
use std::fs::read_dir;
use std::path::{Path, PathBuf};

use diagnostic_quick::error_3rd::WalkDir;
use diagnostic_quick::QResult;
use itertools::iproduct;

#[test]
fn find_all() -> QResult {
    let mut out: BTreeMap<usize, Vec<String>> = Default::default();
    let all = find_all_file(&PathBuf::from())?;

    let v1 = vec![];
    for i in iproduct!('a'..='z') {
        if all.contains() { }
    }
    for (i, j) in iproduct!('a'..='z', 'a'..='z') {
        // ..
    }
    for (i, j, k) in iproduct!('a'..='z', 'a'..='z', 'a'..='z') {
        // ..
    }
    for (i, j, k, l) in iproduct!('a'..='z', 'a'..='z', 'a'..='z', 'a'..='z') {
        // ..
    }
    Ok(())
}

fn find_all_file(path: &Path) -> QResult<BTreeSet<String>> {
    let mut files = vec![];
    for entry in WalkDir::new(path) {
        let entry = entry?;
        if entry.path().is_file() {
            if let Some(s) = entry.path().file_name().and_then(OsStr::to_str) {
                files.push(s.to_string());
            }
        }
    }
    Ok(files)
}


pub fn find_indexes() -> QResult<Vec<PathBuf>> {
    let mut index = vec![];
    for entry in read_dir("C:\\Users")? {
        let entry = entry?.path();
        find_registry(&entry, &mut index).ok();
    }
    Ok(index)
}


pub fn find_registry(user: &Path, index: &mut Vec<PathBuf>) -> QResult {
    let path = user.join(".cargo\\registry\\index");
    for entry in read_dir(path)? {
        let entry = entry?.path();
        if entry.is_dir() {
            index.push(entry.join(".cache").canonicalize()?);
        }
    }
    Ok(())
}