use std::collections::HashMap;

pub(crate) unsafe fn pattern_scan_core(base: *mut u8, end: usize, pattern: &[u8]) -> Option<*mut u8> {
    let right_most_wildcard_index = match get_right_most_wildcard(pattern){
        Some(i) => i,
        None => pattern.len()
    };
    let bmt = build_bad_match_table(pattern, right_most_wildcard_index);

    let mut current = (base as *mut u8).offset(pattern.len() as isize - 1 as isize);

    while (current as usize) < end {
        for (i, p) in pattern.iter().rev().enumerate() {
            // if pattern == current or pattern == ?, then
            if *p == b'\x3F' || *p == *current {
                if *p == pattern[0] {
                    // This is fired when the pattern is found.
                    return Some(current);
                }
                current = current.offset(-1);
                // if pattern != current
            } else {
                let movement_num = if let Some(i) = bmt.get(&*current) {
                    i.clone()
                } else { right_most_wildcard_index };
                current = current.offset(movement_num as isize + i as isize);
                break;
            }
        }
    }
    None
}

fn build_bad_match_table(pattern: &[u8], right_most_wildcard_index: usize) -> HashMap<&u8, usize> {
    let mut bad_match_table = HashMap::new();
    let pattern_length = pattern.len();
    for (i, p) in pattern.iter().enumerate() {
        let table_value = (pattern_length as isize - i as isize - 2) as usize;
        // if right_most_wildcard_index is pattern.len(), it's gonna be classified to else block anytime.
        let table_value = if table_value > right_most_wildcard_index {
            right_most_wildcard_index + 1
        } else if table_value < 1 {
            1
        } else {
            table_value
        };
        bad_match_table.insert(p, table_value);
    }
    bad_match_table
}

/// get_right_most_wildcard seeks the position of right most question mark and returns its index.
fn get_right_most_wildcard(pattern: &[u8]) -> Option<usize> {
    for (i, p) in pattern.iter().enumerate() {
        // \x3F represents '?' in ASCII table.
        if *p == b'\x3F' {
            return Some(i);
        }
    }
    None
}