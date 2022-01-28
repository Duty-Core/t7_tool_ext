#![feature(let_else)]
#![no_main]

#[macro_use]
mod macros;
mod gdtdb;
mod hook;
mod linker;

use core::{ffi::c_void, ptr::null_mut};
use core::mem::MaybeUninit;
use std::sync::Once;

use windows_sys::Win32::{
    Foundation::{BOOL, HINSTANCE, MAX_PATH},
    System::{
        LibraryLoader::GetModuleHandleA, ProcessStatus::K32GetModuleBaseNameA,
        SystemServices::DLL_PROCESS_ATTACH,
    },
};

static ENTRY_ONCE: Once = Once::new();

#[no_mangle]
#[allow(non_snake_case)]
pub unsafe extern "system" fn DllMain(
    _hins_dll: HINSTANCE,
    fdw_reason: u32,
    _lpv_reserved: *const c_void,
) -> BOOL {
    match fdw_reason {
        DLL_PROCESS_ATTACH => ENTRY_ONCE.call_once(ext_entypoint),
        _ => (),
    }

    1
}

pub fn ext_entypoint() {
    unsafe {
        let base = GetModuleHandleA(null_mut());
        let mut file_name = MaybeUninit::<[u8; MAX_PATH as usize]>::uninit();
        let file_name_len = K32GetModuleBaseNameA(-1 as _, base, file_name.as_mut_ptr().cast(), MAX_PATH) as usize;
        let file_name = file_name.assume_init();

        match &file_name[..file_name_len] {
            b"linker_modtools.exe" => linker::patch_linker(),
            b"gdtdb.exe" => gdtdb::patch_gdtdb(),
            _ => (),
        }
    }
}
