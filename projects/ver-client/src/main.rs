use std::env::current_dir;

use diagnostic_quick::QResult;

use crates_name::utils::export_available;

fn main() -> QResult {
    export_available(&current_dir()?)
}