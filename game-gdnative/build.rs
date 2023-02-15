use std::fs;
use std::path::Path;

fn main() {
    let out_dir = std::env::var("OUT_DIR").unwrap();
    let dest_path = Path::new(&out_dir).join("puzzle_definitions.rs");

    let mut puzzles = fs::read_dir("src/puzzles")
        .unwrap()
        .filter_map(|raw_path| {
            let rel_path = raw_path.as_ref().unwrap().path();

            let mut fname = raw_path.unwrap().file_name().into_string().unwrap();

            if fname.ends_with(".yaml") {
                fname.truncate(fname.len() - 5);

                let abspath = fs::canonicalize(rel_path)
                    .unwrap()
                    .into_os_string()
                    .into_string()
                    .unwrap();

                Some((fname, abspath))
            } else {
                None
            }
        })
        .collect::<Vec<(String, String)>>();

    puzzles.sort_by_cached_key(|(puzzle_name, _)| puzzle_name.clone());

    std::fs::write(
        &dest_path,
        format!(
            "pub const PUZZLES: [(&'static str, &'static str); {}] = [{}];",
            puzzles.len(),
            puzzles
                .iter()
                .map(|(fname, path)| format!("(\"{fname}\", include_str!(\"{path}\"))"))
                .collect::<Vec<_>>()
                .join(", ")
        ),
    )
    .unwrap();
}
