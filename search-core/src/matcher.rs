// src/matcher.rs
use memchr::{memchr, memmem, memrchr};

/// find all occurrences of pattern (byte offset)
pub fn find_matches(target: &[u8], pattern: &[u8]) -> Vec<usize> {
    if pattern.is_empty() {
        return Vec::new();
    }
    let finder = memmem::Finder::new(pattern);
    finder.find_iter(target).collect()
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
    let column = String::from_utf8_lossy(prefix_bytes).chars().count() + 1;

    // line range is from start to end of line
    let line_range = (line_start, line_end);

    (line_number, column, line_range)
}
