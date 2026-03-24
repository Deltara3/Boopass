#[cfg(not(all(windows, target_arch = "x86")))]
compile_error!("compilation is only allowed on 32-bit windows");

use std::ffi::c_void;
use windows::core::{s, PCSTR, BOOL};
use windows::Win32::{
    Foundation::HMODULE,
    System::LibraryLoader::LoadLibraryA,
    System::SystemServices::DLL_PROCESS_ATTACH,
    UI::WindowsAndMessaging::{MessageBoxA, MB_OK, MB_ICONWARNING, MB_TOPMOST}
};

mod dinput;

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub extern "system" fn DllMain(_module: HMODULE, reason: u32, _: *mut c_void) -> BOOL {
    if reason == DLL_PROCESS_ATTACH {
        dinput::load();

        if let Err(error) = unsafe { LoadLibraryA(s!("boopass.dll")) } {
            let msg = "Failed to load Boopass, continuing without.";
            let text = format!("{}\nReason: {}\0", msg, error.message());

            unsafe { 
                let _ = MessageBoxA(
                    None,
                    PCSTR(text.as_ptr()),
                    s!("Uh-oh!"),
                    MB_OK | MB_ICONWARNING | MB_TOPMOST
                );
            }
        }
    }

    BOOL(1)
}