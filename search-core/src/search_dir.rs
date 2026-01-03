use std::path::Path;

use walkdir::WalkDir;

use crate::{
    search,
    types::{MatchInfo, SearchOptions},
};

/// 숨김 파일/디렉토리인지 확인
fn is_hidden(entry: &walkdir::DirEntry) -> bool {
    entry
        .file_name()
        .to_str()
        .map(|s| s.starts_with('.'))
        .unwrap_or(false)
}

pub fn search_dir(
    root_dir: &Path,
    pattern: &str,
    options: &SearchOptions,
) -> std::io::Result<Vec<MatchInfo>> {
    let mut matches = Vec::new();
    let walker = WalkDir::new(root_dir).into_iter();
    for entry in walker.filter_entry(|e| options.include_hidden || !is_hidden(e)) {
        let entry = match entry {
            Ok(e) => e,
            Err(_) => continue,
        };
        if !entry.file_type().is_file() {
            continue;
        }
        if let Ok(Some(file_matches)) = search::search_in_file(entry.path(), pattern, options) {
            matches.extend(file_matches);
        }
    }
    Ok(matches)
}
