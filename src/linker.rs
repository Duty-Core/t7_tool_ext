use core::{mem::transmute, ptr::null_mut};
use std::{ffi::{CStr, OsStr}, path::Path};

use windows_sys::Win32::{System::LibraryLoader::GetModuleHandleA};

use crate::hook;

pub type FLoadAsset = unsafe extern "C" fn(*const i8) -> usize;

static mut S_LOADASSET: Option<FLoadAsset> = None;
static mut S_LOADSCRIPTASSET: Option<FLoadAsset> = None;

pub fn patch_linker() {
    pextln!("Initializing Linker modifications...");

    unsafe {
        let handle = GetModuleHandleA(null_mut()) as *mut u8;

        let Some(o_loadasset) = hook::hook(handle.add(0xC86438).cast(), load_asset as usize).map(|x| transmute(x)) else {
            pextln!("Failed to hook LoadAsset. Exiting.");
            return;
        };

        let Some(o_loadscriptasset) = hook::hook(handle.add(0xC86470).cast(), load_script_asset as usize).map(|x| transmute(x)) else {
            pextln!("Failed to hook LoadScriptAsset. Exiting.");
            return;
        };

        S_LOADASSET = Some(o_loadasset);
        S_LOADSCRIPTASSET = Some(o_loadscriptasset);

        // Patch linker dependency check
        *handle.add(0x322AE0).cast::<[u8; 5]>() = [0xE9, 0xB1, 0x00, 0x00, 0x00];

        pextln!("Linker Modifications initialized!");
    };
}

unsafe extern "C" fn load_asset(buffer: *mut i8) -> usize {
    let Ok(asset) = CStr::from_ptr(buffer).to_str() else {
        return (S_LOADASSET.unwrap())(buffer);
    };
    pextln!("Linking Asset {}...", asset);
    let time = std::time::Instant::now();
    match Path::new(asset).extension().and_then(OsStr::to_str) {
        Some(".luac") => (), // do compiled lua stuff
        Some(".lua") => (), // do lua stuff
        _ => (),
    }

    let r = (S_LOADASSET.unwrap())(buffer);
    println!("Linking {} took {:?}ms", asset, time.elapsed());

    r
}

unsafe extern "C" fn load_script_asset(buffer: *const i8) -> usize {
    let Ok(asset) = CStr::from_ptr(buffer).to_str() else {
        return (S_LOADASSET.unwrap())(buffer);
    };
    pextln!("Linking Script {}...", asset);
    let time = std::time::Instant::now();
    match Path::new(asset).extension().and_then(OsStr::to_str) {
        Some(".gscc" | ".cscc" | ".gshc") => (), // do compiled gsc stuff
        _ => (),
    }
    let r = (S_LOADSCRIPTASSET.unwrap())(buffer);
    println!("Linking {} took {:?}", asset, time.elapsed());

    r
}
