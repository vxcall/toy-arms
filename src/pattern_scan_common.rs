//! pattern_scan_common has a common components which is referenced by external and internal variants of pattern scan mods.

use std::collections::HashMap;
use winapi::um::winnt::{MEM_COMMIT, MEMORY_BASIC_INFORMATION, PAGE_NOACCESS};

pub(crate) fn is_page_readable(memory_info: &MEMORY_BASIC_INFORMATION) -> bool {
    if memory_info.State != MEM_COMMIT
        || memory_info.Protect == 0x0
        || memory_info.Protect == PAGE_NOACCESS
    {
        return false;
    }
    true
}

pub(crate) fn process_pattern_from_str(pattern: &str) -> Vec<u8> {
    pattern
        .split_whitespace()
        .map(|x| {
            if x.contains('?') {
                b'\x3F'
            } else {
                u8::from_str_radix(x, 16)
                    .expect("Substring not contained within hexadecimal alphanumeric form")
            }
        })
        .collect()
}

// build_bad_match_table returns the Hashmap that holds each byte and the corresponding number of how many bytes to skip.
pub(crate) fn build_bad_match_table(pattern: &[u8], right_most_wildcard_index: usize) -> HashMap<&u8, usize> {
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
