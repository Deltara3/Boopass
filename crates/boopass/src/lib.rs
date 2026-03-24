#[cfg(not(all(windows, target_arch = "x86")))]
compile_error!("compilation is only allowed on 32-bit windows");

use std::ffi::c_void;
use windows::core::BOOL;
use windows::Win32::{
    Foundation::HMODULE,
    System::SystemServices::DLL_PROCESS_ATTACH
};

mod dxpatch;

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub extern "system" fn DllMain(_module: HMODULE, reason: u32, _: *mut c_void) -> BOOL {
    if reason == DLL_PROCESS_ATTACH {
        dxpatch::install();
    }

    BOOL(1)
}