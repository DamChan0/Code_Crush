//matcher is just find byte offset of the pattern in the pool

pub fn find_matches(target: &str, hit_pattern: &str, case_insensitive: bool) -> Option<Vec<usize>> {
    if hit_pattern.is_empty() {
        return None;
    }
    let mut offset = Vec::new();
    let mut start = 0;
    if case_insensitive {
        let target_lower = target.to_lowercase();
        let pattern_lower = hit_pattern.to_lowercase();
        while let Some(pos) = target_lower[start..].find(&pattern_lower) {
            let absolute_pos = start + pos;
            offset.push(absolute_pos);
            start = absolute_pos + pattern_lower.len();
        }
    } else {
        while let Some(pos) = target[start..].find(hit_pattern) {
            let absolute_pos = start + pos;
            offset.push(absolute_pos);
            start = absolute_pos + hit_pattern.len();
        }
    }
    if offset.is_empty() {
        return None;
    }
    Some(offset)
}
