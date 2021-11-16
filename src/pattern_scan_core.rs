use std::collections::HashMap;
use winapi::shared::minwindef::LPVOID;
use winapi::um::memoryapi::VirtualQuery;
use winapi::um::winnt::{MEM_COMMIT, MEMORY_BASIC_INFORMATION, PAGE_NOACCESS};

pub(crate) unsafe fn boyer_moore_horspool(base: *mut u8, end: usize, pattern: &str) -> Option<*mut u8> {
    let pattern_vec = process_pattern_from_str(pattern);
    let pattern = pattern_vec.as_slice();

    let right_most_wildcard_index = match get_right_most_wildcard(pattern){
        Some(i) => i,
        None => pattern.len()
    };

    let bmt = build_bad_match_table(pattern, right_most_wildcard_index);

    let mut current = (base as *mut u8).offset(pattern.len() as isize - 1 as isize);

    // initializing memory_info and next_page_base just in case the first
    let mut memory_info: MEMORY_BASIC_INFORMATION = MEMORY_BASIC_INFORMATION::default();
    let mut next_page_base = 0x0;

    while (current as usize) < end {
        // if current sticks out of next_page_base, update memory_info and next_page_base.
        if (current as usize) >= next_page_base {
            VirtualQuery(current as LPVOID, &mut memory_info, std::mem::size_of::<MEMORY_BASIC_INFORMATION>());
            next_page_base = memory_info.BaseAddress as usize + memory_info.RegionSize as usize;
            if !is_page_readable(&memory_info) {
                current = (memory_info.BaseAddress as usize + memory_info.RegionSize as usize + pattern.len()) as *mut u8;
                continue;
            }
        }

        // stores the number of how many bytes did they match so far.
        let mut pattern_match_num = 0;
        for (i, p) in pattern.iter().rev().enumerate() {
            // if pattern == current or pattern == ?, then
            if *p == b'\x3F' || *p == *current {
                pattern_match_num += 1;
                if pattern_match_num == pattern.len() {
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

fn is_page_readable(memory_info: &MEMORY_BASIC_INFORMATION) -> bool {
    if memory_info.State != MEM_COMMIT || memory_info.Protect == 0x0 || memory_info.Protect == PAGE_NOACCESS {
        return false;
    }
    true
}

fn process_pattern_from_str(pattern: &str) -> Vec<u8> {
    let p_array = pattern.split(" ").collect::<Vec<&str>>();
    let mut pattern_vec: Vec<u8> = Vec::new();
    for p in p_array {
        if p == "?" {
            pattern_vec.push(b'?');
            continue;
        }
        pattern_vec.push(u8::from_str_radix(p, 16).unwrap());
    }
    pattern_vec
}

// build_bad_match_table returns the Hashmap that holds each byte and the corresponding number of how many bytes to skip.
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