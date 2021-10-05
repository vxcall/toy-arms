use std::str::Utf8Error;
use winapi::shared::minwindef::HMODULE;
use crate::{get_module_handle, read_null_terminated_string};
use crate::cast;

pub struct Memory<'a> {
    pub module_name: &'a str,
    pub module_handle: HMODULE,
}

impl<'a> Memory<'a> {
    pub fn from_module(module_name: &'a str) -> Self {
        Memory {
            module_name,
            module_handle: get_module_handle(module_name),
        }
    }

    pub fn read<T>(&self, address: i32) -> *const T {
        cast!(self.module_handle as usize + address as usize, T)
    }

    pub fn read_mut<T>(&self, address: i32) -> *mut T {
        cast!(mut self.module_handle as usize + address as usize, T)
    }

    pub fn read_string(&self, address: i32) -> Result<String, Utf8Error> {
        unsafe{
            Ok(read_null_terminated_string(self.module_handle as usize + address as usize)?)
        }

    }
}