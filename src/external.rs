use std::{
    fmt::Debug,
    mem::size_of,
    ptr::null_mut,
};

use winapi::{
        shared::{
            minwindef::{ FALSE, HMODULE, LPCVOID, LPVOID, TRUE },
            basetsd::SIZE_T,
        },
        um::{
            errhandlingapi::GetLastError,
            handleapi::{ CloseHandle, INVALID_HANDLE_VALUE },
            processthreadsapi::OpenProcess,
            winnt::{ HANDLE, PROCESS_ALL_ACCESS },
            memoryapi::{ ReadProcessMemory, WriteProcessMemory },
            tlhelp32::{
                CreateToolhelp32Snapshot, Module32First, Module32Next, Process32First, Process32Next,
                MODULEENTRY32, PROCESSENTRY32, TH32CS_SNAPMODULE, TH32CS_SNAPMODULE32, TH32CS_SNAPPROCESS,
            }
        }
};

use crate::read_null_terminated_string;
use thiserror::Error;

#[derive(Error, Debug)]
#[cfg(feature = "external")]
pub enum ToyArmsExternalError {
    #[error("Taking snapshot FAILED.")]
    SnapshotFailed,
    #[error("No more files")]
    NoMoreFiles,
    #[error("Process not found")]
    ProcessNotFound,
    #[error("Module not found")]
    ModuleNotFound,
    #[error("ReadProcessMemory failed")]
    ReadProcessMemoryFailed,
    #[error("WriteProcessMemory failed")]
    WriteProcessMemoryFailed,
}

#[derive(Debug)]
#[cfg(feature = "external")]
pub struct ModuleEx {
    pub module_size: u32,
    pub module_base_address: usize,
    pub module_handle: HMODULE,
    pub module_name: String,
    pub module_path: String,
}

#[cfg(feature = "external")]
impl ModuleEx {
    pub fn from_module_entry(module_entry: &MODULEENTRY32, module_name: String) -> Self {
        unsafe {
            ModuleEx {
                module_size: module_entry.modBaseSize,
                module_base_address: module_entry.modBaseAddr as usize,
                module_handle: module_entry.hModule,
                module_name,
                module_path: read_null_terminated_string(module_entry.szExePath.as_ptr() as usize)
                    .unwrap(),
            }
        }
    }
}

#[cfg(feature = "external")]
#[derive(Debug)]
pub struct MemoryEx<'a> {
    #[warn(dead_code)]
    pub process_name: &'a str,
    pub process_id: u32,
    pub process_handle: HANDLE,
}

#[cfg(feature = "external")]
impl<'a> MemoryEx<'a> {
    pub fn from_process_name(process_name: &'a str) -> Self {
        let process_id = get_process_id(process_name).unwrap();
        let process_handle = get_process_handle(process_id);
        MemoryEx {
            process_name,
            process_id,
            process_handle,
        }
    }

    /// read fetches the value that given address is holding.
    /// * `base_address` - the address that is supposed to have the value you want
    #[cfg(feature = "external")]
    pub fn read<T>(&self, base_address: usize) -> Result<T, ToyArmsExternalError> {
        unsafe {
            let mut buffer: T = std::mem::zeroed::<T>();
            let ok = ReadProcessMemory(
                self.process_handle,
                base_address as LPCVOID,
                &mut buffer as *mut _ as LPVOID,
                size_of::<LPVOID>() as SIZE_T,
                null_mut::<SIZE_T>(),
            );
            if ok == FALSE {
                return Err(ToyArmsExternalError::ReadProcessMemoryFailed);
            }
            Ok(buffer)
        }
    }

    /// write overwrites the value that given base_address is holding.
    /// * `base_address` - the address that is supposed have the value you want to tamper with.
    /// * `value` - new value you wanna overwrite
    #[cfg(feature = "external")]
    pub fn write<T>(&self, base_address: usize, value: &mut T) -> Result<(), ToyArmsExternalError> {
        unsafe {
            let ok = WriteProcessMemory(
                self.process_handle,
                base_address as LPVOID,
                value as *mut T as LPCVOID,
                size_of::<LPCVOID>() as SIZE_T,
                null_mut::<SIZE_T>(),
            );
            if ok == FALSE {
                println!("{}", GetLastError());
                return Err(ToyArmsExternalError::WriteProcessMemoryFailed);
            }
        }
        Ok(())
    }

    #[cfg(feature = "external")]
    pub fn get_module_info(&self, module_name: &str) -> Result<ModuleEx, ToyArmsExternalError> {
        unsafe {
            let snap_handle =
                CreateToolhelp32Snapshot(TH32CS_SNAPMODULE | TH32CS_SNAPMODULE32, self.process_id);
            if snap_handle == INVALID_HANDLE_VALUE {
                return Err(ToyArmsExternalError::SnapshotFailed);
            }
            let mut module_entry: MODULEENTRY32 = MODULEENTRY32::default();
            module_entry.dwSize = size_of::<MODULEENTRY32>() as u32;
            if Module32First(snap_handle, &mut module_entry) == TRUE {
                if read_null_terminated_string(module_entry.szModule.as_ptr() as usize).unwrap()
                    == module_name
                {
                    return Ok(ModuleEx::from_module_entry(
                        &module_entry,
                        module_name.into(),
                    ));
                }
                loop {
                    if Module32Next(snap_handle, &mut module_entry) == FALSE {
                        if GetLastError() == 18 {
                            return Err(ToyArmsExternalError::NoMoreFiles);
                        }
                    }
                    if read_null_terminated_string(module_entry.szModule.as_ptr() as usize).unwrap()
                        == module_name
                    {
                        return Ok(ModuleEx::from_module_entry(
                            &module_entry,
                            module_name.into(),
                        ));
                    }
                }
            }
            Err(ToyArmsExternalError::ModuleNotFound)
        }
    }

    #[cfg(feature = "external")]
    pub fn get_module_base(&self, module_name: &str) -> Result<usize, ToyArmsExternalError> {
        let info: ModuleEx = self.get_module_info(module_name)?;
        Ok(info.module_base_address)
    }
}

#[cfg(feature = "external")]
fn get_process_id(process_name: &str) -> Result<u32, ToyArmsExternalError> {
    unsafe {
        let snap_handle = CreateToolhelp32Snapshot(TH32CS_SNAPPROCESS, 0);
        if snap_handle == INVALID_HANDLE_VALUE {
            return Err(ToyArmsExternalError::SnapshotFailed);
        }
        let mut proc_entry: PROCESSENTRY32 = PROCESSENTRY32::default();
        proc_entry.dwSize = size_of::<PROCESSENTRY32>() as u32;
        if Process32First(snap_handle, &mut proc_entry) == 1 {
            if read_null_terminated_string(proc_entry.szExeFile.as_ptr() as usize).unwrap()
                == process_name
            {
                return Ok(proc_entry.th32ProcessID as u32);
            }
            loop {
                if Process32Next(snap_handle, &mut proc_entry) == FALSE {
                    if GetLastError() == 18 {
                        return Err(ToyArmsExternalError::NoMoreFiles);
                    }
                }
                if read_null_terminated_string(proc_entry.szExeFile.as_ptr() as usize).unwrap()
                    == process_name
                {
                    return Ok(proc_entry.th32ProcessID as u32);
                }
            }
        }
        CloseHandle(snap_handle);
    }
    Err(ToyArmsExternalError::ProcessNotFound)
}

#[cfg(feature = "external")]
fn get_process_handle(process_id: u32) -> HANDLE {
    unsafe { OpenProcess(PROCESS_ALL_ACCESS, FALSE, process_id as u32) }
}

#[test]
#[ignore]
fn test_get_process_id() {
    let process_name = "csgo.exe";
    assert_ne!(0, get_process_id(process_name).unwrap());
}

#[test]
#[ignore]
fn test_get_process_handle() {
    let process_name = "csgo.exe";
    let process_id = get_process_id(process_name).unwrap();
    assert_ne!(0x0, get_process_handle(process_id) as i32);
}

#[test]
#[ignore]
fn test_get_module_info() {
    let memex = MemoryEx::from_process_name("csgo.exe");
    let module_info = memex.get_module_info("client.dll").unwrap();
    assert_ne!(module_info.module_name, "client.dll");
}
