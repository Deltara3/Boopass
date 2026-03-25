#[cfg(not(all(windows, target_arch = "x86")))]
compile_error!("compilation is only allowed on 32-bit windows");

use std::ffi::c_void;
use shared::warn;
use windows::core::{s, BOOL};
use windows::Win32::{
    Foundation::HMODULE,
    System::LibraryLoader::LoadLibraryA,
    System::SystemServices::DLL_PROCESS_ATTACH,
    System::Console::{AttachConsole, ATTACH_PARENT_PROCESS}
};

mod dinput;

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub extern "system" fn DllMain(_module: HMODULE, reason: u32, _: *mut c_void) -> BOOL {
    if reason == DLL_PROCESS_ATTACH {
        // Attach to OpenParrotLoader's console.
        let _ = unsafe { AttachConsole(ATTACH_PARENT_PROCESS) };

        dinput::load();

        if let Err(error) = unsafe { LoadLibraryA(s!("boopass.dll")) } {
            warn!("Loader", "Loading Boopass failed with code {}, proceeding without.", error.code());
        }
    }

    BOOL(1)
}