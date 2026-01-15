// src/searcher.rs
use anyhow::Result;
use memmap2::Mmap;
use std::fs::File;
use std::path::Path;
use std::sync::Arc; // Arc를 가져옵니다.

use crate::matcher;
use crate::types::{MatchInfo, SearchOptions};

/// searcher a single file and return a vector of `MatchInfo` coordinates
/// this function does not create or allocate any strings
pub fn searcher(
    path: &Path,
    pattern: Arc<String>,
    _options: &SearchOptions,
) -> Result<Vec<MatchInfo>> {
    let file = File::open(path)?;

    // file is empty
    if file.metadata()?.len() == 0 {
        return Ok(Vec::new());
    }

    // map file to memory : whole file is loaded into memory
    let mmap = unsafe { Mmap::map(&file)? };
    // convert pattern to bytes
    let pattern_bytes = pattern.as_bytes();

    // find all occurrences of pattern (byte offset)
    let match_indices = matcher::find_matches(&mmap, pattern_bytes);
    let mut results = Vec::with_capacity(match_indices.len());

    for pos in match_indices {
        // calculate line number, column, line range based on position
        let (line_number, column, line_range) = matcher::extract_line_context(&mmap, pos);

        // make vector of MatchInfo
        results.push(MatchInfo {
            path: path.to_path_buf(),
            line_number,
            column,
            byte_offset: pos,
            line_range,
            matched_text: Arc::clone(&pattern),
        });
    }

    Ok(results)
}
