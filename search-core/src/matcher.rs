// src/matcher.rs
use memchr::{memchr, memmem, memrchr};
use regex::RegexBuilder;

/// find all occurrences of pattern (byte offset, length)
pub fn find_matches(target: &[u8], pattern: &[u8], case_insensitive: bool) -> Vec<(usize, usize)> {
    if pattern.is_empty() {
        return Vec::new();
    }

    if !case_insensitive {
        let finder = memmem::Finder::new(pattern);
        return finder
            .find_iter(target)
            .map(|start| (start, pattern.len()))
            .collect();
    }

    if let (Ok(target_str), Ok(pattern_str)) =
        (std::str::from_utf8(target), std::str::from_utf8(pattern))
    {
        if let Ok(re) = RegexBuilder::new(&regex::escape(pattern_str))
            .case_insensitive(true)
            .build()
        {
            return re
                .find_iter(target_str)
                .map(|m| (m.start(), m.end() - m.start()))
                .collect();
        }
    }

    let pattern_lower: Vec<u8> = pattern.iter().map(|b| b.to_ascii_lowercase()).collect();
    let pat_len = pattern_lower.len();
    if target.len() < pat_len {
        return Vec::new();
    }

    let mut matches = Vec::new();
    for i in 0..=target.len() - pat_len {
        let mut matched = true;
        for j in 0..pat_len {
            if target[i + j].to_ascii_lowercase() != pattern_lower[j] {
                matched = false;
                break;
            }
        }
        if matched {
            matches.push((i, pat_len));
        }
    }
    matches
}

/// extract line context (line number, column, line range)
pub fn extract_line_context(data: &[u8], pos: usize) -> (usize, usize, (usize, usize)) {
    // find line start
    let line_start = memrchr(b'\n', &data[..pos]).map(|i| i + 1).unwrap_or(0);

    // find line end
    let line_end = memchr(b'\n', &data[pos..])
        .map(|i| pos + i)
        .unwrap_or(data.len());

    // calculate line number
    let line_number = bytecount::count(&data[..line_start], b'\n') + 1;

    // calculate column
    let prefix_bytes = &data[line_start..pos];
    let column = match std::str::from_utf8(prefix_bytes) {
        Ok(s) => s.chars().count() + 1,
        Err(_) => prefix_bytes.len() + 1,
    };

    // line range is from start to end of line
    let line_range = (line_start, line_end);

    (line_number, column, line_range)
}
