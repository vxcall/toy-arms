use winapi::shared::minwindef::LPVOID;
use winapi::um::memoryapi::VirtualQuery;
use winapi::um::winnt::{ MEMORY_BASIC_INFORMATION };
use crate::pattern_scan_common::{build_bad_match_table, is_page_readable, process_pattern_from_str};

pub(crate) unsafe fn boyer_moore_horspool(
    pattern: &str,
    start: usize,
    end: usize,
) -> Option<*mut u8> {
    let pattern_vec = process_pattern_from_str(pattern);
    let pattern = pattern_vec.as_slice();

    let right_most_wildcard_index = if let Some(x) = pattern.iter().rev().position(|&x| x == b'\x3F') {
            x
        } else {
            pattern.len()
        };
    let bmt = build_bad_match_table(pattern, right_most_wildcard_index);

    let mut current = (start as *mut u8).offset(pattern.len() as isize - 1);

    let mut memory_info: MEMORY_BASIC_INFORMATION = MEMORY_BASIC_INFORMATION::default();
    let mut next_page_base = 0x0;

    while (current as usize) < end {
        // if current sticks out of next_page_base, update memory_info and next_page_base.
        if (current as usize) >= next_page_base {
            VirtualQuery(
                current as LPVOID,
                &mut memory_info,
                std::mem::size_of::<MEMORY_BASIC_INFORMATION>(),
            );
            next_page_base = memory_info.BaseAddress as usize + memory_info.RegionSize as usize;
            if !is_page_readable(&memory_info) {
                current = (memory_info.BaseAddress as usize
                    + memory_info.RegionSize as usize
                    + pattern.len()) as *mut u8;
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
                } else {
                    right_most_wildcard_index
                };
                current = current.offset(movement_num as isize + i as isize);
                break;
            }
        }
    }
    None
}