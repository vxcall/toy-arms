use std::mem::size_of_val;
use std::{fmt, fmt::Debug};

use winapi::{
    shared::minwindef::HMODULE,
    um::{tlhelp32::MODULEENTRY32, winnt::HANDLE},
};

use crate::error::TAExternalError;
use crate::read;
use smartstring::alias::String;
use utils::utils::read_null_terminated_string;

#[derive(Debug)]
pub struct Module {
    pub process_handle: HANDLE,
    pub size: u32,
    pub base_address: usize,
    pub handle: HMODULE,
    pub name: String,
    pub path: String,
    pub data: Vec<u8>,
}

impl fmt::Display for Module {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "module_name: {}\nmodule_size: {}\nmodule_base_address: {}\nmodule_handle: {:?}\nmodule_path: {}\nprocess_handle: {:?}", self.name, self.size, self.base_address, self.handle, self.path, self.process_handle)
    }
}

impl Default for Module {
    fn default() -> Self {
        Module {
            process_handle: 0x0 as HANDLE,
            size: 0,
            base_address: 0,
            handle: 0x0 as HMODULE,
            name: String::new(),
            path: String::default(),
            data: vec![0u8; 80000000],
        }
    }
}

impl Module {
    pub(crate) fn from_module_entry(
        process_handle: HANDLE,
        module_entry: &MODULEENTRY32,
        module_name: &str,
    ) -> Result<Self, TAExternalError> {
        let mut module = Module {
            process_handle,
            size: module_entry.modBaseSize,
            base_address: module_entry.modBaseAddr as usize,
            handle: module_entry.hModule,
            name: String::from(module_name),
            // This is allowed because szExePath.as_ptr() is the address within module_entry variable, not the address in the target process.
            path: unsafe { read_null_terminated_string(module_entry.szExePath.as_ptr() as usize) }
                .unwrap()
                .parse()
                .unwrap(),
            data: vec![0u8; module_entry.modBaseSize as usize],
        };
        let ok = read::<Vec<u8>>(
            &module.process_handle,
            module.base_address,
            module.data.len(),
            module.data.as_mut_ptr() as *mut Vec<u8>,
        );
        match ok {
            Err(e) => Err(e),
            _ => Ok(module),
        }
    }

    #[allow(dead_code, unused_must_use)]
    pub(crate) fn ensure_data_populated(&mut self) {
        if size_of_val(&self.data) == 80000000 {
            self.data.resize(self.size as usize, 0u8);
        }

        if self.data.iter().all(|&val| val == 0) {
            read::<Vec<u8>>(
                &self.process_handle,
                self.base_address,
                self.data.len(),
                self.data.as_mut_ptr() as *mut Vec<u8>,
            );
        }
    }
}
