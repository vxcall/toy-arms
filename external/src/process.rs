use std::{fmt, fmt::Debug, mem::size_of};

use winapi::{
    shared::minwindef::{FALSE, TRUE},
    um::{
        errhandlingapi::GetLastError,
        handleapi::INVALID_HANDLE_VALUE,
        tlhelp32::{
            CreateToolhelp32Snapshot, Module32First, Module32Next, MODULEENTRY32,
            TH32CS_SNAPMODULE, TH32CS_SNAPMODULE32,
        },
        winnt::HANDLE,
    },
};

use super::error::{SnapshotFailedDetail, TAExternalError};
use winapi::shared::minwindef::BOOL;
use winapi::um::handleapi::CloseHandle;
use winapi::um::processthreadsapi::OpenProcess;
use winapi::um::tlhelp32::{Process32First, Process32Next, PROCESSENTRY32, TH32CS_SNAPPROCESS};
use winapi::um::winnt::PROCESS_ALL_ACCESS;

use crate::module::Module;
use utils::utils::read_null_terminated_string;
use winapi::um::wow64apiset::IsWow64Process;

#[derive(Debug)]
pub struct Process<'a> {
    pub name: &'a str,
    pub id: u32,
    pub handle: HANDLE,
}

impl<'a> fmt::Display for Process<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "process_name: {}\nprocess_id: {}\nprocess_handle: {:?}",
            self.name, self.id, self.handle
        )
    }
}

impl<'a> Default for Process<'a> {
    fn default() -> Self {
        Process {
            name: "",
            id: 0,
            handle: 0x0 as HANDLE,
        }
    }
}

impl<'a> Process<'a> {
    #[inline]
    pub fn from_process_name(process_name: &'a str) -> Result<Self, TAExternalError> {
        let process_id = get_process_id(process_name)?;
        let process_handle = get_process_handle(process_id);
        let mut is_wow64: BOOL = 0;
        Process::is_wow64_process(&process_handle, &mut is_wow64);
        Ok(Process {
            name: process_name,
            id: process_id,
            handle: process_handle,
        })
    }

    fn is_wow64_process(process_handle: &HANDLE, is_wow64: &mut BOOL) -> bool {
        unsafe { IsWow64Process(*process_handle, is_wow64 as *mut BOOL) == 1 }
    }

    pub fn get_module_info(&self, module_name: &'a str) -> Result<Module, TAExternalError> {
        unsafe {
            let snap_handle =
                CreateToolhelp32Snapshot(TH32CS_SNAPMODULE | TH32CS_SNAPMODULE32, self.id);
            if snap_handle == INVALID_HANDLE_VALUE {
                return Err(TAExternalError::SnapshotFailed(
                    SnapshotFailedDetail::InvalidHandle,
                ));
            }
            let mut module_entry: MODULEENTRY32 = MODULEENTRY32::default();
            module_entry.dwSize = size_of::<MODULEENTRY32>() as u32;
            if Module32First(snap_handle, &mut module_entry) == TRUE {
                if read_null_terminated_string(module_entry.szModule.as_ptr() as usize).unwrap()
                    == module_name
                {
                    return Module::from_module_entry(
                        self.handle,
                        &module_entry,
                        module_name,
                    );
                }
                loop {
                    if Module32Next(snap_handle, &mut module_entry) == FALSE {
                        if GetLastError() == 18 {
                            return Err(TAExternalError::SnapshotFailed(
                                SnapshotFailedDetail::NoMoreFiles,
                            ));
                        }
                    }
                    if read_null_terminated_string(module_entry.szModule.as_ptr() as usize).unwrap()
                        == module_name
                    {
                        return Module::from_module_entry(
                            self.handle,
                            &module_entry,
                            module_name,
                        );
                    }
                }
            }
            Err(TAExternalError::ModuleNotFound)
        }
    }

    #[inline]
    pub fn get_module_base(&self, module_name: &str) -> Result<usize, TAExternalError> {
        let info: Module = self.get_module_info(module_name)?;
        Ok(info.base_address)
    }
}

pub(crate) fn get_process_id(process_name: &str) -> Result<u32, TAExternalError> {
    unsafe {
        let snap_handle = CreateToolhelp32Snapshot(TH32CS_SNAPPROCESS, 0);
        if snap_handle == INVALID_HANDLE_VALUE {
            return Err(TAExternalError::SnapshotFailed(
                SnapshotFailedDetail::InvalidHandle,
            ));
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
                        return Err(TAExternalError::SnapshotFailed(
                            SnapshotFailedDetail::NoMoreFiles,
                        ));
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
    Err(TAExternalError::ProcessNotFound)
}

#[inline]
pub(crate) fn get_process_handle(process_id: u32) -> HANDLE {
    unsafe { OpenProcess(PROCESS_ALL_ACCESS, FALSE, process_id as u32) }
}
