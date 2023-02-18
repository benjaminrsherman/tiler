use std::ffi::OsString;
use std::path::{Path, PathBuf};
use std::time::SystemTime;

use walkdir::WalkDir;

fn modified_time(direntry: &walkdir::DirEntry) -> SystemTime {
    direntry
        .metadata()
        .ok()
        .and_then(|metadata| metadata.modified().ok())
        .unwrap_or(SystemTime::UNIX_EPOCH)
}

fn abspath(path: &Path) -> Option<String> {
    path.canonicalize()
        .ok()
        .map(PathBuf::into_os_string)
        .map(OsString::into_string)
        .map(|x| x.ok())
        .unwrap_or_default()
}

fn main() {
    let out_dir = std::env::var("OUT_DIR").unwrap();
    let dest_path = Path::new(&out_dir).join("puzzle_definitions.rs");

    // If we're in CI, default sorting is good
    // If we're running a local build (i.e. not in CI), sort puzzles by
    // mtime order to speed up iteration cycles.
    let files = if std::env::var("CI").unwrap_or("false".to_string()) == "true" {
        WalkDir::new("src/puzzles")
    } else {
        WalkDir::new("src/puzzles").sort_by(|d1, d2| modified_time(d2).cmp(&modified_time(d1)))
    };

    let mut puzzles = files
        .into_iter()
        .filter_map(|d| d.ok())
        .filter(|direntry| direntry.file_type().is_file())
        .filter_map(|file| abspath(file.path()))
        .filter(|x| x.ends_with(".yaml"))
        .collect::<Vec<String>>();

    if std::env::var("CI").unwrap_or("false".to_string()) == "true" {
        puzzles.sort();
    }

    std::fs::write(
        &dest_path,
        format!(
            "pub const PUZZLES: [&'static str; {}] = [{}];",
            puzzles.len(),
            puzzles
                .iter()
                .map(|path| format!("include_str!(\"{path}\")"))
                .collect::<Vec<_>>()
                .join(", ")
        ),
    )
    .unwrap();
}
