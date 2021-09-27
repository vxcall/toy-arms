use crate::{cast, null_terminated};
use winapi::{
    shared::minwindef::LPVOID, um::libloaderapi::GetModuleHandleA, um::winnt::PIMAGE_DOS_HEADER,
    um::winnt::PIMAGE_IMPORT_DESCRIPTOR, um::winnt::PIMAGE_NT_HEADERS,
    um::winnt::PIMAGE_OPTIONAL_HEADER,
};

use std::{panic, ptr};

pub struct IatFinder<'a> {
    pub module_name: &'a str,
    pub function_name: &'a str,
    target_entry: *mut LPVOID,
}

struct IMAGE_OPTIONAL_HEADER64;
impl<'a> IatFinder<'a> {
    pub fn new(module_name: &'a str, function_name: &'a str) -> Self {
        IatFinder {
            module_name,
            function_name,
            target_entry: ptr::null_mut(),
        }
    }

    pub unsafe fn find_iat_entry(&self) {
        let dos_base = GetModuleHandleA(null_terminated!(self.module_name)) as usize;
        let ptr_dos_header = dos_base as PIMAGE_DOS_HEADER;
        let ptr_nt_headers = (dos_base + (*ptr_dos_header).e_lfanew as usize) as PIMAGE_NT_HEADERS;
        let ptr_optional_header = &(*ptr_nt_headers).OptionalHeader;
        println!("{:p}", ptr_optional_header);
    }
}
