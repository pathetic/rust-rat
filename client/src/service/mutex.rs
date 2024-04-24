extern crate winapi;

use winapi::um::synchapi::CreateMutexW;
use winapi::um::errhandlingapi::GetLastError;
use std::ptr;
use std::ffi::OsStr;
use std::os::windows::ffi::OsStrExt;
use winapi::um::handleapi::CloseHandle;
use std::process::exit;
use winapi::shared::winerror::ERROR_ALREADY_EXISTS;

pub fn mutex_lock() {
    let mutex_value = format!("Local\\{}", crate::settings::MUTEX);
    let mutex = OsStr::new(&mutex_value).encode_wide().chain(Some(0)).collect::<Vec<u16>>();

    unsafe {
        let mutex_handle = CreateMutexW(ptr::null_mut(), 1, mutex.as_ptr());

        if mutex_handle.is_null() {
            exit(0);
        }

        let last_error = GetLastError();
        if last_error == ERROR_ALREADY_EXISTS {
            CloseHandle(mutex_handle);
            exit(0);
        }

        CloseHandle(mutex_handle);
    }
}
