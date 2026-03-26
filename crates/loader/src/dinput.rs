// Proxy implementation.
// We don't need anything else as the game just uses this function.

use std::mem;
use std::ffi::c_void;
use std::cell::OnceCell;
use shared::{Win32Unwrap, log, cell, hookdef};
use windows::core::{s, GUID, HRESULT, PCSTR};
use windows::Win32::{
    Foundation::{HINSTANCE, HMODULE, MAX_PATH},
    System::SystemInformation::GetSystemDirectoryA,
    System::LibraryLoader::{GetProcAddress, LoadLibraryA}
};

thread_local! {
    static ORIGINAL_DLL: OnceCell<HMODULE> = OnceCell::new();
}

pub fn load() {
    unsafe {
        // Unless someone's computer is messed up so bad, I don't see this failing.
        let mut path_buf = [0u8; MAX_PATH as usize];
        let path_len = GetSystemDirectoryA(Some(&mut path_buf));

        // I honestly don't think this unwrap matters either.
        let sys_dir = std::str::from_utf8(&path_buf[..path_len as usize]).unwrap();
        let dll_path = format!(r"{}\dinput8.dll", sys_dir);
        let c_path = PCSTR::from_raw(dll_path.as_ptr());

        let module = LoadLibraryA(c_path).unwrap_or_die(|error| {
            log::fatal!("Loader", "Loading original DLL failed with code {}, aborting.", error.code());
        });
        
        ORIGINAL_DLL.with(|dll| {
            let _ = dll.set(module);

            log::info!("Loader", "Loaded original DLL with handle 0x{:08X}.", module.0 as usize);

            let method = GetProcAddress(*dll.get().unwrap(), s!("DirectInput8Create")).unwrap_or_die(|error| {
                log::fatal!("Loader", "Retrieving original function failed with code {}, aborting.", error.code());
            });

            cell::init!(DINPUT_CREATE, mem::transmute(method));
            log::info!("Loader", "Retrieved original function from address 0x{:08X}.", method as usize);
        });
    }
}

hookdef! {
    DINPUT_CREATE DirectInput8Create(
        hinst: HINSTANCE,
        dwVersion: u32,
        riidltf: *const GUID,
        ppvOut: *mut *mut c_void,
        punkOuter: *mut c_void
    ) -> HRESULT {
        unsafe {
            // We crash if the function or module doesn't exist, should be fine.
            cell::call!(DINPUT_CREATE, hinst, dwVersion, riidltf, ppvOut, punkOuter)
        }
    }
}