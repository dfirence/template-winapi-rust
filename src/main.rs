#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_must_use)]


#[cfg(target_os = "windows")]
use std::mem;

#[cfg(target_os = "windows")]
use std::ptr;

#[cfg(target_os = "windows")]
use std::ffi::{OsStr};

#[cfg(target_os = "windows")]
use std::os::windows::prelude::*;

// 3rd Party Modules
// Import: WinApi
#[cfg(target_os = "windows")]
extern crate winapi;

// WinApi Imports
#[cfg(target_os = "windows")]
use winapi::{
    shared::ntdef::{
        NULL
    },
    shared::minwindef::{
        TRUE,
        FALSE,
        LPVOID,
        DWORD
    },
    um::{
        errhandlingapi::{
            GetLastError
        },
        handleapi::{
            CloseHandle
        },
        processthreadsapi::{
            PROCESS_INFORMATION_CLASS,
            PROCESS_INFORMATION,
            STARTUPINFOA,
            STARTUPINFOW,
            CreateProcessA,
            CreateProcessW,
            GetCurrentProcessId,
            OpenProcess
        },
        psapi::{
            EnumProcessModules
        },
        synchapi::{
            WaitForSingleObject
        },
        winbase::{
            CREATE_NEW_CONSOLE,
            CREATE_NEW_PROCESS_GROUP,
            CREATE_NO_WINDOW,
            CREATE_SUSPENDED,
            CREATE_UNICODE_ENVIRONMENT,
            DETACHED_PROCESS,
            INFINITE
        },
        winnt::{
            HANDLE,
            PROCESS_QUERY_INFORMATION,
            PROCESS_VM_READ
        }
    }
};

#[cfg(target_os = "windows")]
fn main() -> Result<(), Box<dyn std::error::Error>>
{
    Ok(())
}