use crate::{null_terminated_i8, read_null_terminated_string};
use winapi::{
    shared::minwindef::{DWORD, LPVOID},
    um::libloaderapi::GetModuleHandleA,
    um::winnt::PIMAGE_NT_HEADERS,
    um::winnt::{PIMAGE_DOS_HEADER, PIMAGE_THUNK_DATA},
    um::winnt::{PIMAGE_IMPORT_BY_NAME, PIMAGE_IMPORT_DESCRIPTOR},
};

use std::ptr;

pub struct IatFinder<'a> {
    pub module_name: &'a str,
    pub function_name: &'a str,
    pub target_entry: *mut LPVOID,
}

#[derive(thiserror::Error, Debug)]
pub enum IatLookupError {
    #[error("{0}")]
    ImportEntryNotFound(String),
}

impl<'a> IatFinder<'a> {
    pub fn new(module_name: &'a str, function_name: &'a str) -> Self {
        IatFinder {
            module_name,
            function_name,
            target_entry: ptr::null_mut(),
        }
    }

    pub unsafe fn run(&mut self) {
        self.target_entry = self.find_iat_entry().unwrap();
    }

    unsafe fn find_iat_entry(&self) -> Result<*mut LPVOID, IatLookupError> {
        let dos_base = GetModuleHandleA(null_terminated_i8(self.module_name)) as usize;
        let ptr_dos_header = dos_base as PIMAGE_DOS_HEADER;
        let ptr_nt_headers = (dos_base + (*ptr_dos_header).e_lfanew as usize) as PIMAGE_NT_HEADERS;
        let ptr_optional_header = &(*ptr_nt_headers).OptionalHeader;
        const IMAGE_DIRECTORY_ENTRY_IMPORT: usize = 1;
        let rva_directory_import = (*ptr_optional_header).DataDirectory[IMAGE_DIRECTORY_ENTRY_IMPORT].VirtualAddress;
        let mut ptr_import_descriptor = (dos_base + (rva_directory_import as usize)) as PIMAGE_IMPORT_DESCRIPTOR;
        if rva_directory_import == 0 {
            return Err(IatLookupError::ImportEntryNotFound(String::from("Rva of image directory entry import was not found")));
        }

        while (*ptr_import_descriptor).u.Characteristics() != &0 {
            let dll_name = read_null_terminated_string(dos_base + (*ptr_import_descriptor).Name as usize).unwrap();

            let mut ptr_import_name_table = (dos_base + *(*ptr_import_descriptor).u.OriginalFirstThunk() as usize) as *const DWORD;
            let mut counter = 0;
            while *(*(ptr_import_name_table as PIMAGE_THUNK_DATA)).u1.AddressOfData() != 0 {
                let funciton_info = (dos_base + *ptr_import_name_table as usize) as PIMAGE_IMPORT_BY_NAME;
                let function_name = read_null_terminated_string((*funciton_info).Name.as_ptr() as usize).unwrap_or_default();

                if function_name == self.function_name {
                    println!("found {}", self.function_name);
                    return Ok(((dos_base + (*ptr_import_descriptor).FirstThunk as usize) as *mut LPVOID).offset(counter));
                }


                // Somehow \0 has been inserted between each functions, so need to offset 2 to skip \0.
                ptr_import_name_table = ptr_import_name_table.offset(2);
                counter += 1;
            }
            ptr_import_descriptor = ptr_import_descriptor.offset(1);
        }

        Ok(self.target_entry)
    }
}
