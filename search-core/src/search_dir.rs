use std::path::Path;

use walkdir::WalkDir;

use crate::{search, types::MatchInfo};

pub fn search_dir(root_dir: &Path, pattern: &str) -> std::io::Result<Vec<MatchInfo>> {
    let mut matches = Vec::new();

    for file in WalkDir::new(root_dir)
        .into_iter()
        .filter_map(|entry| entry.ok())
        .filter(|entry| entry.file_type().is_file())
    {
        if let Ok(Some(file_matches)) = search::search_in_file(file.path(), pattern) {
            matches.extend(file_matches);
        }
    }
    Ok(matches)
}
