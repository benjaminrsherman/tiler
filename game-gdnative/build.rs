use anyhow::Result;
use std::path::Path;
use std::time::SystemTime;

use walkdir::WalkDir;

include!("src/puzzles/mod.rs");

fn modified_time(direntry: &walkdir::DirEntry) -> SystemTime {
    direntry
        .metadata()
        .ok()
        .and_then(|metadata| metadata.modified().ok())
        .unwrap_or(SystemTime::UNIX_EPOCH)
}

fn puzzle_and_short_name(path: &Path) -> Result<(PuzzleDefinition, String)> {
    let full_shortname = path
        .as_os_str()
        .to_owned()
        .into_string()
        .unwrap()
        .strip_prefix("src/puzzles/")
        .unwrap()
        .to_string();

    let fcontents = std::fs::read_to_string(path)?;

    if let Some(shortname) = full_shortname.strip_suffix(".yaml") {
        Ok((serde_yaml::from_str(&fcontents)?, shortname.to_string()))
    } else if let Some(shortname) = full_shortname.strip_suffix(".txt") {
        Ok((
            PuzzleDefinition::from_ascii_art(shortname.to_string(), fcontents),
            shortname.to_string(),
        ))
    } else {
        Err(anyhow::anyhow!("not a file or unknown extension"))
    }
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
        .filter_map(|file| puzzle_and_short_name(file.path()).ok())
        .collect::<Vec<(PuzzleDefinition, String)>>();

    if std::env::var("CI").unwrap_or("false".to_string()) == "true" {
        puzzles.sort_by_cached_key(|(_, shortname)| shortname.clone());
    }

    let mut puzzle_map = phf_codegen::Map::new();
    for (idx, (_, shortname)) in puzzles.iter().enumerate() {
        puzzle_map.entry(shortname, &format!("{idx}"));
    }

    std::fs::write(
        &dest_path,
        format!(
            "pub const PUZZLES: [&'static str; {}] = [\n{}\n];\npub static PUZZLE_NAME_MAP: phf::Map<&'static str, usize> = {};",
            puzzles.len(),
            puzzles
                .iter()
                .map(|(puzzle, _)| format!("\"{}\"", serde_yaml::to_string(puzzle).unwrap()))
                .collect::<Vec<_>>()
                .join(",\n"),
            puzzle_map.build()
        ),
    )
    .unwrap();
}
