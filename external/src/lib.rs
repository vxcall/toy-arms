pub mod error;

pub mod module;

pub mod pattern_scan;

pub mod process;

pub trait GameObject {
    unsafe fn from_raw(address: *const usize) -> Option<*mut Self>;
}
use std::{mem::size_of, ptr::null_mut};

use error::{ReadWriteMemoryFailedDetail, TAExternalError};
use utils::pattern_scan::is_page_readable;
use winapi::shared::minwindef::DWORD;
use winapi::um::memoryapi::{VirtualProtectEx, VirtualQueryEx};
use winapi::um::winnt::{MEMORY_BASIC_INFORMATION, PAGE_READWRITE};
use winapi::{
    shared::{
        basetsd::SIZE_T,
        minwindef::{FALSE, LPCVOID, LPVOID},
    },
    um::{
        errhandlingapi::GetLastError,
        memoryapi::{ReadProcessMemory, WriteProcessMemory},
        winnt::HANDLE,
    },
};

/// read fetches the value that given address is holding.
/// * `process_handle` - handle of the process that module belongs to.
/// * `base_address` - the address that is supposed to have the value you want
/// * `buffer` - the buffer to be filled with read value. must have identical type as T.
pub fn read<T>(
    process_handle: &HANDLE,
    base_address: usize,
    size: usize,
    buffer: *mut T,
) -> Result<(), TAExternalError> {
    unsafe {
        let mut memory_info: MEMORY_BASIC_INFORMATION = MEMORY_BASIC_INFORMATION::default();
        VirtualQueryEx(
            *process_handle,
            base_address as LPCVOID,
            &mut memory_info,
            std::mem::size_of::<MEMORY_BASIC_INFORMATION>(),
        );
        let is_readable = is_page_readable(&memory_info);
        let mut old_protect = PAGE_READWRITE;
        let mut new_protect = PAGE_READWRITE;
        if !is_readable {
            VirtualProtectEx(
                *process_handle,
                base_address as LPVOID,
                size_of::<LPVOID>(),
                new_protect,
                &mut old_protect as *mut DWORD,
            );
        }

        let ok = ReadProcessMemory(
            *process_handle,
            base_address as LPCVOID,
            buffer as *mut T as LPVOID,
            size as SIZE_T,
            null_mut::<SIZE_T>(),
        );

        if !is_readable {
            VirtualProtectEx(
                *process_handle,
                base_address as LPVOID,
                size_of::<LPVOID>(),
                old_protect,
                &mut new_protect as *mut DWORD,
            );
        }

        if ok == FALSE {
            let error_code = GetLastError();
            return match error_code {
                6 => Err(TAExternalError::ReadMemoryFailed(
                    ReadWriteMemoryFailedDetail::ErrorInvalidHandle,
                )),
                299 => Err(TAExternalError::ReadMemoryFailed(
                    ReadWriteMemoryFailedDetail::ErrorPartialCopy,
                )),
                487 => Err(TAExternalError::ReadMemoryFailed(
                    ReadWriteMemoryFailedDetail::ErrorInvalidAddress,
                )),
                _ => Err(TAExternalError::ReadMemoryFailed(
                    ReadWriteMemoryFailedDetail::UnknownError { error_code },
                )),
            };
        }
        Ok(())
    }
}

/// write overwrites the value that given base_address is holding.
/// * `base_address` - the address that is supposed have the value you want to tamper with.
/// * `value` - new value you wanna overwrite
pub fn write<T>(
    process_handle: &HANDLE,
    base_address: usize,
    value: &mut T,
) -> Result<(), TAExternalError> {
    unsafe {
        let ok = WriteProcessMemory(
            *process_handle,
            base_address as LPVOID,
            value as *mut T as LPCVOID,
            size_of::<T>() as SIZE_T,
            null_mut::<SIZE_T>(),
        );
        if ok == FALSE {
            let error_code = GetLastError();
            return match error_code {
                6 => Err(TAExternalError::ReadMemoryFailed(
                    ReadWriteMemoryFailedDetail::ErrorInvalidHandle,
                )),
                299 => Err(TAExternalError::WriteMemoryFailed(
                    ReadWriteMemoryFailedDetail::ErrorPartialCopy,
                )),
                487 => Err(TAExternalError::WriteMemoryFailed(
                    ReadWriteMemoryFailedDetail::ErrorInvalidAddress,
                )),
                _ => Err(TAExternalError::WriteMemoryFailed(
                    ReadWriteMemoryFailedDetail::UnknownError { error_code },
                )),
            };
        }
    }
    Ok(())
}

#[test]
#[ignore]
fn test_get_process_id() {
    let process_name = "csgo.exe";
    assert_ne!(0, process::get_process_id(process_name).unwrap());
}

#[test]
#[ignore]
fn test_get_process_handle() {
    let process_name = "csgo.exe";
    let process_id = process::get_process_id(process_name).unwrap();
    assert_ne!(0x0, process::get_process_handle(process_id) as i32);
}

#[test]
#[ignore]
fn test_get_module_info() {
    let memex = process::Process::from_process_name("csgo.exe")
        .expect("Error at Process::from_process_name");
    let module_info = memex.get_module_info("client.dll").unwrap();
    assert_ne!(module_info.name, "client.dll");
}
