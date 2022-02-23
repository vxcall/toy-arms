use winapi::shared::minwindef::LPCVOID;
use winapi::um::memoryapi::VirtualQueryEx;
use winapi::um::winnt::{HANDLE, MEMORY_BASIC_INFORMATION};
use crate::external::read;
use crate::pattern_scan_common::{build_bad_match_table, is_page_readable, process_pattern_from_str};

pub(crate) unsafe fn boyer_moore_horspool(
    process_handle: HANDLE,
    pattern: &str,
    start: usize,
    end: usize,
) -> Option<usize> {
    let pattern_vec = process_pattern_from_str(pattern);
    let pattern = pattern_vec.as_slice();

    let right_most_wildcard_index = if let Some(x) = pattern.iter().rev().position(|&x| x == b'\x3F') {
         x
        } else {
        pattern.len()
        };
    let bmt = build_bad_match_table(pattern, right_most_wildcard_index);

    let mut current = start + (pattern.len() as isize - 1) as usize;
    let mut memory_info: MEMORY_BASIC_INFORMATION = MEMORY_BASIC_INFORMATION::default();
    let mut next_page_base = 0x0;

    while current < end {
        if current <= next_page_base {
            VirtualQueryEx(process_handle, current as LPCVOID, &mut memory_info, std::mem::size_of::<MEMORY_BASIC_INFORMATION>());
            next_page_base = memory_info.BaseAddress as usize + memory_info.RegionSize as usize;
            if !is_page_readable(&memory_info) {
                current = memory_info.BaseAddress as usize
                    + memory_info.RegionSize as usize
                    + pattern.len();
                continue;
            }
        }

        let mut pattern_match_num = 0;
        for (i, p) in pattern.iter().rev().enumerate() {
            let current_byte = read::<u8>(process_handle, current).expect("READ FAILED");
            if *p == b'\x3F' || *p == current_byte {
                pattern_match_num += 1;
                if pattern_match_num == pattern.len() {
                    return Some(current);
                }
                current = current - 1;
            } else {
                let movement_num = if let Some(i) = bmt.get(&current_byte) {
                    i.clone()
                } else {
                    right_most_wildcard_index
                };
                current = current + movement_num + i;
                break;
            }
        }
    }
    None
}