#[cfg(not(all(windows, target_arch = "x86")))]
compile_error!("compilation is only allowed on 32-bit windows");

use std::ffi::c_void;
use windows::core::BOOL;
use windows::Win32::Foundation::HMODULE;
use windows::Win32::System::SystemServices::DLL_PROCESS_ATTACH;

unsafe extern "C" { fn AttachHook() -> BOOL; }

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub extern "system" fn DllMain(_module: HMODULE, reason: u32, _: *mut c_void) -> BOOL {
    match reason {
        DLL_PROCESS_ATTACH => {
            let attach_result = unsafe { AttachHook() };

            if attach_result != BOOL(1) {
                return attach_result;
            }
        },
        _ => { /* Yay, do nothing again! */ }
    };

    BOOL(1)
}