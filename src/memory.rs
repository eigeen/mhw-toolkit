use std::ffi::c_void;

use windows::Win32::System::Memory::{
    VirtualProtect, PAGE_EXECUTE_READWRITE, PAGE_PROTECTION_FLAGS,
};

pub unsafe fn patch(position: *const c_void, bytes: &[u8]) -> Result<(), String> {
    let patch_len = bytes.len();
    let dwsize = (patch_len / 4096 + 1) * 4096;
    let mut old_protection = PAGE_PROTECTION_FLAGS::default();
    VirtualProtect(
        position,
        dwsize,
        PAGE_EXECUTE_READWRITE,
        &mut old_protection,
    )
    .map_err(|e| e.to_string())?;

    let patch_ptr = position as *mut u8;
    std::ptr::copy_nonoverlapping(bytes.as_ptr(), patch_ptr, patch_len);

    let mut _temp = PAGE_PROTECTION_FLAGS::default();
    VirtualProtect(position, dwsize, old_protection, &mut _temp).map_err(|e| e.to_string())?;

    Ok(())
}

pub unsafe fn patch_nop(position: *const c_void, length: usize) -> Result<(), String> {
    let nop_bytes = vec![0x90; length];
    patch(position, &nop_bytes)?;

    Ok(())
}
