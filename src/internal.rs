use std::str::Utf8Error;
use winapi::shared::minwindef::HMODULE;
use winapi::um::psapi::{GetModuleInformation, MODULEINFO};
use crate::{get_module_handle, read_null_terminated_string};
use crate::cast;
use std::mem::{size_of, zeroed};
use winapi::um::processthreadsapi::GetCurrentProcess;
use crate::pattern_scan_core::pattern_scan_core;

#[cfg(feature = "internal")]
pub struct Module<'a> {
    pub module_name: &'a str,
    pub module_handle: HMODULE,
    pub module_size: u32,
    pub module_base_address: usize,
}

#[cfg(feature = "internal")]
impl<'a> Module<'a> {
    pub fn from_module_name(module_name: &'a str) -> Option<Self> {
        let module_handle: HMODULE = match get_module_handle(module_name) {
            Some(e) => e,
            None => return None
        };
        unsafe {
            let mut module_info: MODULEINFO = zeroed::<MODULEINFO>();
            GetModuleInformation(GetCurrentProcess(), module_handle, &mut module_info, size_of::<MODULEINFO>() as u32);
            Some(
                Module {
                    module_name,
                    module_handle,
                    module_base_address: module_info.lpBaseOfDll as usize,
                    module_size: module_info.SizeOfImage,
                }
            )
        }
    }

    /// read fetches the value that given address is holding.
    /// * `base_address` - the address that is supposed to have the value you want
    pub fn read<T>(&self, address: i32) -> *const T {
        cast!(self.module_handle as usize + address as usize, T)
    }

    /// read_mut not only fetches the value, make it mutable.
    pub fn read_mut<T>(&self, address: i32) -> *mut T {
        cast!(mut self.module_handle as usize + address as usize, T)
    }

    /// read_string reads the string untill the null terminator that is in the given module
    /// * `address` - relative address of the head of the string.
    pub fn read_string(&self, address: i32) -> Result<String, Utf8Error> {
        unsafe{
            Ok(read_null_terminated_string(self.module_handle as usize + address as usize)?)
        }

    }

    /// pattern_scan does a pattern scanning over entire module and returns the address.
    /// * `pattern` - pattern string you're looking for. format: "8D 34 85 ? ? ? ? 89 15 ? ? ? ? 8B 41 08 8B 48 04 83 F9 FF"
    /// * `offset` - offset of the address from pattern's base.
    /// * `extra` - offset of the address from dereferenced address.
    pub fn pattern_scan(&self, pattern: &str, offset: isize, extra: usize) -> Option<usize> {
        let p_array = pattern.split(" ").collect::<Vec<&str>>();
        let mut pattern_vec: Vec<u8> = Vec::new();
        for p in p_array {
            if p == "?" {
                pattern_vec.push(b'?');
                continue;
            }
            pattern_vec.push(u8::from_str_radix(p, 16).unwrap());
        }
        let pattern_b = pattern_vec.as_slice();
        let base = self.module_base_address as *mut u8;
        let end = self.module_base_address + self.module_size as usize;
        unsafe {
            let address = pattern_scan_core(base, end, pattern_b)?;
            // calculate relative address
            Some(*(address.offset(offset) as *mut usize) - base as usize + extra)
        }
    }
}