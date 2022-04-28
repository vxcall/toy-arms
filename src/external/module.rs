use std::{fmt, fmt::Debug};
use std::convert::TryInto;

use winapi::{
    shared::{
        minwindef::{ HMODULE },
    },
    um::{
        winnt::{ HANDLE },
        tlhelp32::{
            MODULEENTRY32,
        }
    }
};

use crate::utils_common::read_null_terminated_string;
use smartstring::alias::String;
use crate::external::read;

#[derive(Debug)]
pub struct Module<'a> {
    process_handle: &'a HANDLE,
    pub module_size: u32,
    pub module_base_address: usize,
    pub module_handle: HMODULE,
    pub module_name: &'a str,
    pub module_path: String,
}

impl<'a> fmt::Display for Module<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "module_name: {}\nmodule_size: {}\nmodule_base_address: {}\nmodule_handle: {:?}\nmodule_path: {}\nprocess_handle: {:?}", self.module_name, self.module_size, self.module_base_address, self.module_handle, self.module_path, self.process_handle)
    }
}

impl<'a> Default for Module<'a> {
    fn default() -> Self {
        Module {
            process_handle: &(0x0 as HANDLE),
            module_size: 0,
            module_base_address: 0,
            module_handle: 0x0 as HMODULE,
            module_name: "",
            module_path: String::default(),
        }
    }
}

impl<'a> Module<'a> {
    pub(crate) fn from_module_entry(process_handle: &'a HANDLE, module_entry: &MODULEENTRY32, module_name: &'a str) -> Self {
        Module {
            process_handle,
            module_size: module_entry.modBaseSize,
            module_base_address: module_entry.modBaseAddr as usize,
            module_handle: module_entry.hModule,
            module_name,
            // This is allowed because szExePath.as_ptr() is the address within module_entry variable, not the address in the target process.
            module_path: unsafe { read_null_terminated_string(module_entry.szExePath.as_ptr() as usize) }.unwrap().parse().unwrap(),
        }
    }

    pub fn find_pattern(&self, pattern: &str) -> Option<usize> {
        let base = self.module_base_address;
        let end = self.module_base_address + self.module_size as usize;
        unsafe { crate::external::pattern_scan::boyer_moore_horspool(self.process_handle, pattern, base, end) }
    }

    /// pattern scan basically be for calculating offset of some value. It adds the offset to the pattern-matched address, dereferences, and add the `extra`.
    /// * `pattern` - pattern string you're looking for. format: "8D 34 85 ? ? ? ? 89 15 ? ? ? ? 8B 41 08 8B 48 04 83 F9 FF"
    /// * `offset` - offset of the address from pattern's base.
    /// * `extra` - offset of the address from dereferenced address.
    pub fn pattern_scan<T>(&self, pattern: &str, offset: usize, extra: usize) -> Option<T>
        where T: std::ops::Add<Output = T>,
              T: std::ops::Sub<Output = T>,
              T: std::convert::TryFrom<usize>,
              <T as std::convert::TryFrom<usize>>::Error: Debug,
    {
        let address = self.find_pattern(pattern)?;
        let address = address + offset;
        Some(read::<T>(self.process_handle, address).expect("READ FAILED IN PATTERN SCAN") - self.module_base_address.try_into().unwrap() + extra.try_into().unwrap())
    }

    pub fn find_pattern_specific_range(&self, pattern: &str, start: usize, end: usize) -> Option<usize> {
        unsafe { crate::external::pattern_scan::boyer_moore_horspool(self.process_handle, pattern, start, end) }
    }
}
