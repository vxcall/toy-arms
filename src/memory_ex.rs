use std::mem::size_of;
use winapi::shared::minwindef::{FALSE, TRUE};
use winapi::um::handleapi::{CloseHandle, INVALID_HANDLE_VALUE};
use winapi::um::processthreadsapi::OpenProcess;
use winapi::um::tlhelp32::{CreateToolhelp32Snapshot, Module32First, Module32Next, MODULEENTRY32, Process32First, Process32Next, PROCESSENTRY32, TH32CS_SNAPMODULE, TH32CS_SNAPMODULE32, TH32CS_SNAPPROCESS};
use winapi::um::winnt::{HANDLE, PROCESS_ALL_ACCESS};
use crate::read_null_terminated_string;
use thiserror::Error;
use winapi::um::errhandlingapi::GetLastError;

#[derive(Error, Debug)]
pub enum MemoryExError {
    #[error("Taking snapshot FAILED.")]
    SnapshotFailed,
    #[error("No more files")]
    NoMoreFiles,
    #[error("Process not found")]
    ProcessNotFound,
    #[error("Module not found")]
    ModuleNotFound
}

pub struct MemoryEx<'a> {
    process_name: &'a str,
    pub process_id: u32,
    pub process_handle: HANDLE,
}

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

    pub fn get_module_info(&self, module_name: &str) -> Result<MODULEENTRY32, MemoryExError>  {
        unsafe {
            let snap_handle = CreateToolhelp32Snapshot(TH32CS_SNAPMODULE | TH32CS_SNAPMODULE32, self.process_id);
            if snap_handle == INVALID_HANDLE_VALUE {
                return Err(MemoryExError::SnapshotFailed);
            }
            let mut module_entry: MODULEENTRY32 = MODULEENTRY32::default();
            module_entry.dwSize = size_of::<MODULEENTRY32>() as u32;
            if Module32First(snap_handle, &mut module_entry) == TRUE {
                if read_null_terminated_string(module_entry.szModule.as_ptr() as usize).unwrap() == module_name {
                    return Ok(module_entry);
                }
                loop {
                    if Module32Next(snap_handle, &mut module_entry) == FALSE {
                        if GetLastError() == 18 {
                            return Err(MemoryExError::NoMoreFiles);
                        }
                    }
                    if read_null_terminated_string(module_entry.szModule.as_ptr() as usize).unwrap() == module_name {
                        return Ok(module_entry);
                    }
                }
            }
            Err(MemoryExError::ModuleNotFound)
        }
    }
}

fn get_process_id(process_name: &str) -> Result<u32, MemoryExError> {
    unsafe {
        let snap_handle = CreateToolhelp32Snapshot(TH32CS_SNAPPROCESS, 0);
        if snap_handle == INVALID_HANDLE_VALUE {
            return Err(MemoryExError::SnapshotFailed);
        }
        let mut proc_entry: PROCESSENTRY32 = PROCESSENTRY32::default();
        proc_entry.dwSize = size_of::<PROCESSENTRY32>() as u32;
        if Process32First(snap_handle, &mut proc_entry) == 1 {
            if read_null_terminated_string(proc_entry.szExeFile.as_ptr() as usize).unwrap() == process_name {
                return Ok(proc_entry.th32ProcessID as u32);
            }
            loop {
                if Process32Next(snap_handle, &mut proc_entry) == FALSE {
                    if GetLastError() == 18 {
                        return Err(MemoryExError::NoMoreFiles);
                    }
                }
                if read_null_terminated_string(proc_entry.szExeFile.as_ptr() as usize).unwrap() == process_name {
                    return Ok(proc_entry.th32ProcessID as u32);
                }
            }
        }
        CloseHandle(snap_handle);
    }
    Err(MemoryExError::ProcessNotFound)
}

fn get_process_handle(process_id: u32) -> HANDLE {
    unsafe {
        OpenProcess(PROCESS_ALL_ACCESS, FALSE, process_id as u32)
    }
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
    assert_ne!(module_info.hModule as usize, 0x0);

}
