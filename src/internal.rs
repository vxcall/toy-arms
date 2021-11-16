use std::str::Utf8Error;
use winapi::shared::minwindef::{DWORD, HMODULE};
use winapi::um::psapi::{EnumProcessModules, GetModuleBaseNameA, GetModuleInformation, MODULEINFO};
use crate::{FARPROC, get_module_handle, GetProcAddress, read_null_terminated_string};
use crate::cast;
use std::mem::{size_of, zeroed};
use winapi::um::processthreadsapi::GetCurrentProcess;
use crate::pattern_scan_core::boyer_moore_horspool;
use thiserror::Error;
use winapi::um::libloaderapi::GetModuleFileNameA;
use winapi::um::winnt::{CHAR, LPSTR};

#[derive(Error, Debug)]
#[cfg(feature = "internal")]
pub enum ToyArmsInternalError {
    #[error("get_all_module_handles failed")]
    GetAllModuleHandlesFailed,
    #[error("pattern_scan_all_modules failed")]
    PatternScanALlModulesFailed,
}

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
    #[cfg(feature = "internal")]
    pub fn read<T>(&self, address: i32) -> *const T {
        cast!(self.module_handle as usize + address as usize, T)
    }

    /// read_mut not only fetches the value, make it mutable.
    #[cfg(feature = "internal")]
    pub fn read_mut<T>(&self, address: i32) -> *mut T {
        cast!(mut self.module_handle as usize + address as usize, T)
    }

    /// read_string reads the string untill the null terminator that is in the given module
    /// * `address` - relative address of the head of the string.
    #[cfg(feature = "internal")]
    pub fn read_string(&self, address: i32) -> Result<String, Utf8Error> {
        unsafe{
            Ok(read_null_terminated_string(self.module_handle as usize + address as usize)?)
        }
    }


    /// find_pattern scans over entire module and returns the address if there is matched byte pattern in module.
    /// * `pattern` - pattern string you're looking for. format: "8D 34 85 ? ? ? ? 89 15 ? ? ? ? 8B 41 08 8B 48 04 83 F9 FF"
    #[cfg(feature = "internal")]
    pub fn find_pattern(&self, pattern: &str) -> Option<usize> {
        let base = self.module_base_address as *mut u8;
        let end = self.module_base_address + self.module_size as usize;
        unsafe {
            return match boyer_moore_horspool(base, end, pattern) {
                Some(e) => Some(e as usize),
                None => None
            }
        }
    }

    /// pattern scan basically be for calculating offset of some value. It adds the offset to the pattern-matched address, dereferences, and add the `extra`.
    /// * `pattern` - pattern string you're looking for. format: "8D 34 85 ? ? ? ? 89 15 ? ? ? ? 8B 41 08 8B 48 04 83 F9 FF"
    /// * `offset` - offset of the address from pattern's base.
    /// * `extra` - offset of the address from dereferenced address.
    #[cfg(feature = "internal")]
    pub fn pattern_scan(&self, pattern: &str, offset: isize, extra: usize) -> Option<usize> {
        unsafe {
        let address = self.find_pattern(pattern)?;
        let address = (address as *mut u8).offset(offset) as *mut usize;
            // calculate relative address
            Some(*address - self.module_base_address + extra)
        }
    }
}

/// This function is for when you don't know which module has the pattern. It returns the address and module name.
/// * `pattern` - pattern string you're looking for. format: "8D 34 85 ? ? ? ? 89 15 ? ? ? ? 8B 41 08 8B 48 04 83 F9 FF"
/// * `offset` - offset of the address from pattern's base.
/// * `extra` - offset of the address from dereferenced address.
#[cfg(feature = "internal")]
pub fn pattern_scan_all_modules(pattern: &str, offset: isize, extra: usize) -> Result<(usize, String), ToyArmsInternalError> {
    unsafe {
        let all_handles = get_all_module_handles()?;
        let process_handle = GetCurrentProcess();
        for handle in all_handles {
            let mut module_info: MODULEINFO = std::mem::zeroed::<MODULEINFO>();
            GetModuleInformation(process_handle, handle, &mut module_info, size_of::<MODULEINFO>() as u32);
            let base = module_info.lpBaseOfDll as *mut u8;
            let end = module_info.lpBaseOfDll as usize + module_info.SizeOfImage as usize;
            match boyer_moore_horspool(base, end, pattern) {
                Some(e) => {
                    let mut module_name: [CHAR; 100] = [0; 100];
                    GetModuleBaseNameA(GetCurrentProcess(), handle, &mut module_name as LPSTR, std::mem::size_of_val(&module_name) as u32);
                    let module_name = read_null_terminated_string(&mut module_name as *mut i8 as usize).unwrap();
                    return Ok((*(e.offset(offset) as *mut usize) - base as usize + extra, module_name));
                },
                None => continue,
            }
        }
        Err(ToyArmsInternalError::PatternScanALlModulesFailed)
    }
}

/// * `module_name` - name of module that the desired function is in.
/// * `function_name` - name of the function you want
#[cfg(feature = "internal")]
pub unsafe fn get_module_function_address(module_name: &str, function_name: &str) -> Option<FARPROC> {
    let module_handle = match get_module_handle(module_name) {
        Some(e) => e,
        None => return None
    };
    Some(GetProcAddress(module_handle, crate::make_lpcstr(function_name)))
}

#[cfg(feature = "internal")]
fn get_all_module_handles() -> Result<Vec<HMODULE>, ToyArmsInternalError> {
    unsafe {
        // Buffer size is 300 * sizeof(HMODULE)
        let mut module_handles: [HMODULE; 300] = [0 as HMODULE; 300];
        // Make a buffer for required_size[out] by zero initializing the DWORD space.
        let mut required_size = std::mem::zeroed::<DWORD>();
        // The last parameter is implicitly: &mut required_size as *mut DWORD
        if EnumProcessModules(GetCurrentProcess(), module_handles.as_mut_ptr(), std::mem::size_of_val(&module_handles) as u32, &mut required_size) != 0 {
            let number_of_handles = required_size as usize / std::mem::size_of::<HMODULE>();
            // If buffer is smaller than required, call EnumProcessModules with bigger buffer.
            if 300 * std::mem::size_of::<HMODULE>() < number_of_handles {
                println!("Buffer is smaller than required size.");
            }
            Ok(module_handles.iter().filter( |e| {**e != 0 as HMODULE}).map(|e| { e.clone() }).collect::<Vec<HMODULE>>())
        } else {
            return Err(ToyArmsInternalError::GetAllModuleHandlesFailed);
        }
    }
}
