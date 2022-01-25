mod gdtdb;
mod linker;

use std::{path::Path, ptr::null};

use log::LevelFilter;
use simplelog::{ColorChoice, Config, TermLogger, TerminalMode};
use winapi::{
    shared::minwindef::{BOOL, DWORD, HINSTANCE, LPVOID, MAX_PATH, TRUE},
    um::{
        libloaderapi::{GetModuleFileNameA, GetModuleHandleW},
        winnt::DLL_PROCESS_ATTACH,
    },
};

#[no_mangle]
#[allow(non_snake_case)]
pub unsafe extern "system" fn DllMain(
    _hins_dll: HINSTANCE,
    fdw_reason: DWORD,
    _lpv_reserved: LPVOID,
) -> BOOL {
    match fdw_reason {
        DLL_PROCESS_ATTACH => ext_entypoint(),
        _ => (),
    }

    TRUE
}

pub fn ext_entypoint() {
    better_panic::install();

    TermLogger::init(
        LevelFilter::Info,
        Config::default(),
        TerminalMode::Mixed,
        ColorChoice::Auto,
    )
    .expect("TermLogger::init failed");

    unsafe {
        let base = GetModuleHandleW(null());
        let mut file_name_b: Vec<i8> = vec![0; MAX_PATH];
        let name_len = GetModuleFileNameA(base, file_name_b.as_mut_ptr().cast(), MAX_PATH as u32);
        let file_name =
            String::from_raw_parts(file_name_b.as_mut_ptr().cast(), name_len as usize, MAX_PATH);
        let file_without_ext = Path::new(&file_name).file_name().unwrap().to_str();

        match file_without_ext {
            Some("linker_modtools.exe") => linker::patch_linker(),
            Some("gdtdb.exe") => gdtdb::patch_gdtdb(),
            _ => (),
        }
    }
}
