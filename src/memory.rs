use std::str::Utf8Error;
use winapi::shared::minwindef::HMODULE;
use winapi::um::psapi::{GetModuleInformation, MODULEINFO};
use crate::{get_module_handle, read_null_terminated_string};
use crate::cast;
use std::mem::{size_of, zeroed};
use winapi::um::processthreadsapi::GetCurrentProcess;

pub struct Memory<'a> {
    pub module_name: &'a str,
    pub module_handle: HMODULE,
    pub module_size: u32,
    pub module_base_address: usize,
}

impl<'a> Memory<'a> {
    pub fn from_module_name(module_name: &'a str) -> Self {
            let module_handle = get_module_handle(module_name);
        unsafe {
            let mut module_info: MODULEINFO = zeroed::<MODULEINFO>();
            GetModuleInformation(GetCurrentProcess(), module_handle, &mut module_info, size_of::<MODULEINFO>() as u32);
            Memory {
                module_name,
                module_handle,
                module_base_address: module_info.lpBaseOfDll as usize,
                module_size: module_info.SizeOfImage,
            }
        }
    }

    pub fn read<T>(&self, address: i32) -> *const T {
        cast!(self.module_handle as usize + address as usize, T)
    }

    pub fn read_mut<T>(&self, address: i32) -> *mut T {
        cast!(mut self.module_handle as usize + address as usize, T)
    }

    pub fn read_string(&self, address: i32) -> Result<String, Utf8Error> {
        unsafe{
            Ok(read_null_terminated_string(self.module_handle as usize + address as usize)?)
        }

    }

    pub unsafe fn signature_scan(&self, pattern: &[u8], _offset: i32, _extra: i32) {
        let right_most_wildcard_index = match get_right_most_wildcard(pattern){
            Some(i) => i,
            None => pattern.len()
        };
        let bmt = build_bad_match_table(pattern, right_most_wildcard_index);

        let base = self.module_base_address as *mut u8;
        let end = self.module_base_address + self.module_size as usize;
        let mut current = (base as *const u8).offset(pattern.len() as isize - 1 as isize);

        let mut flag = false;
        println!("start");
        while (current as usize) < end {
            for (i, p) in pattern.iter().rev().enumerate() {
                // if pattern == current or pattern == ?, then
                if *p == b'\x3F' || *p == *current {
                    if p == &pattern[0] {
                        // This is fired when the pattern is found.
                        println!("pattern found");
                        flag = true;
                        break;
                    }
                    current = current.offset(-1);
                } else {
                    let movement_num = if let Some(i) = bmt.get(&*current) {
                        i.clone()
                    } else { right_most_wildcard_index };
                    current = current.offset(movement_num as isize + i as isize);
                    break;
                }
            }
            if flag {
                break;
            }
        }
    }
}

use std::collections::HashMap;

fn build_bad_match_table(pattern: &[u8], right_most_wildcard_index: usize) -> HashMap<&u8, usize> {
    let mut bad_match_table = HashMap::new();
    let pattern_length = pattern.len();
    for (i, p) in pattern.iter().enumerate() {
        let table_value = (pattern_length as isize - i as isize - 1) as usize;
        // if right_most_wildcard_index is pattern.len(), it's gonna be classified to else block anytime.
        let table_value = if table_value > right_most_wildcard_index { right_most_wildcard_index + 1 } else { table_value };
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

