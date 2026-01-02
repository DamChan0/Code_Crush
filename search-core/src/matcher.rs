//matcher is just find byte offset of the pattern in the pool

pub fn find_matches(target: &str, hit_pattern: &str) -> Option<Vec<usize>> {
    if hit_pattern.is_empty() {
        return None;
    }

    let mut offset = Vec::new();
    let mut start = 0;

    while let Some(pos) = target[start..].find(hit_pattern) {
        let absolute_pos = start + pos;
        offset.push(absolute_pos);
        start = absolute_pos + hit_pattern.len();
    }

    if offset.is_empty() {
        return None;
    }

    Some(offset)
}
