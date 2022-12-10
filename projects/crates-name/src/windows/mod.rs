use std::fs::read_dir;
use std::path::{Path, PathBuf};

use diagnostic_quick::QResult;

#[test]
fn find_user() -> QResult {
    let mut index = vec![];
    for entry in read_dir("C:\\Users")? {
        let entry = entry?.path();
        find_local(&entry, &mut index).ok();
    }
    println!("{:?}", index);
    Ok(())
}


pub fn find_local(user: &Path, index: &mut Vec<PathBuf>) -> QResult {
    let path = user.join(".cargo\\registry\\index");
    for entry in read_dir(path)? {
        let entry = entry?.path();
        if entry.is_dir() {
            index.push(entry.canonicalize()?);
        }
    }
    Ok(())
}