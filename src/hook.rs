use core::ptr;

use windows_sys::Win32::System::Memory::{VirtualProtect, PAGE_EXECUTE_READWRITE};

pub unsafe fn hook<T>(old: *mut T, new: T) -> Option<T> {
    let mut old_protect: u32 = 0;
    if VirtualProtect(old.cast(), 8, PAGE_EXECUTE_READWRITE, &mut old_protect) != 0 {
        let original = ptr::replace(old, new);
        VirtualProtect(old.cast(), 8, old_protect, &mut old_protect);
        return Some(original);
    }

    None
}
