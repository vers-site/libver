use std::collections::BTreeSet;
use std::ffi::OsStr;
use std::fs::{File, read_dir};
use std::io::Write;
use std::path::{Path, PathBuf};

use diagnostic_quick::error_3rd::WalkDir;
use diagnostic_quick::QResult;
use itertools::iproduct;
use serde_json::to_string_pretty;

pub fn export_available(path: &Path) -> QResult {
    let all = find_all_file(path)?;
    available_1(&path, &all)?;
    available_2(&path, &all)?;
    available_3(&path, &all)?;
    available_4(&path, &all)?;
    Ok(())
}


fn available_1(path: &Path, all: &BTreeSet<String>) -> QResult {
    let mut file = File::create(path.join(".github").join("available-1.json"))?;
    let mut out = vec![];
    for i in iproduct!('a'..='z') {
        let name = String::from(i);
        if all.contains(&name) {
            continue;
        }
        out.push(name)
    }
    file.write_all(to_string_pretty(&out).unwrap().as_bytes())?;
    Ok(())
}

fn available_2(path: &Path, all: &BTreeSet<String>) -> QResult {
    let mut file = File::create(path.join(".github").join("available-2.json"))?;
    let mut out = vec![];
    for i in iproduct!('a'..='z', 'a'..='z') {
        let name = format!("{}{}", i.0, i.1);
        if all.contains(&name) {
            continue;
        }
        out.push(name)
    }
    file.write_all(to_string_pretty(&out).unwrap().as_bytes())?;
    Ok(())
}

fn available_3(path: &Path, all: &BTreeSet<String>) -> QResult {
    let mut file = File::create(path.join(".github").join("available-3.json"))?;
    let mut out = vec![];
    for i in iproduct!('a'..='z', 'a'..='z', 'a'..='z') {
        let name = format!("{}{}{}", i.0, i.1, i.2);
        if all.contains(&name) {
            continue;
        }
        out.push(name)
    }
    file.write_all(to_string_pretty(&out).unwrap().as_bytes())?;
    Ok(())
}

fn available_4(path: &Path, all: &BTreeSet<String>) -> QResult {
    let mut file = File::create(path.join(".github").join("available-4.json"))?;
    let mut out = vec![];
    for i in iproduct!('a'..='z', 'a'..='z', 'a'..='z', 'a'..='z') {
        let name = format!("{}{}{}{}", i.0, i.1, i.2, i.3);
        if all.contains(&name) {
            continue;
        }
        out.push(name)
    }
    file.write_all(to_string_pretty(&out).unwrap().as_bytes())?;
    Ok(())
}


fn find_all_file(path: &Path) -> QResult<BTreeSet<String>> {
    let mut files = BTreeSet::default();
    for entry in WalkDir::new(path) {
        let entry = entry?;
        if entry.path().is_file() {
            if let Some(s) = entry.path().file_name().and_then(OsStr::to_str) {
                files.insert(s.to_string());
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