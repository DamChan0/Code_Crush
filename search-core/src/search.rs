// search is find the matches by byte offset(from matcher) in the file and return the MatchInfo
// read files to stream and find the matches
// return the MatchInfo

use std::{
    fs::File,
    io::{BufRead, BufReader},
    path::{Path, PathBuf},
};

use crate::{matcher, types::MatchInfo};

pub fn search_in_file(path: &Path, pattern: &str) -> std::io::Result<Option<Vec<MatchInfo>>> {
    // open the file
    let file = File::open(path)?;
    // reader for read file as buffer
    let mut reader = BufReader::new(file);
    //make results container
    let mut results: Vec<MatchInfo> = Vec::new();
    let mut current_line = String::new();
    let mut line_number: usize = 0;
    let mut byte_offset: usize = 0;

    loop {
        // read a line
        let bytes_read = reader.read_line(&mut current_line)?;
        if bytes_read == 0 {
            break;
        }
        line_number += 1; // line number is 1-based

        if let Some(positions) = matcher::find_matches(&current_line, pattern) {
            for pos in positions {
                let column = current_line[..pos].chars().count() + 1; // column is 1-based

                let line_text = current_line
                    .trim_end_matches('\n') // trim the newline character
                    .trim_end_matches('\r') // trim the carriage return character
                    .to_string();

                results.push(MatchInfo {
                    path: path.to_path_buf(),
                    line_number,
                    column,
                    byte_offset: byte_offset + pos,
                    matched_text: pattern.to_string(),
                    line_text,
                });
            }
        }

        byte_offset += bytes_read;

        current_line.clear();
    }
    Ok(Some(results))
}
