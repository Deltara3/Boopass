// Proxy implementation.
// We don't need anything else as the game just uses this function.

use std::ffi::c_void;
use shared::{Win32Unwrap, log};
use windows::core::{s, GUID, HRESULT, PCSTR};
use windows::Win32::{
    Foundation::{HINSTANCE, HMODULE, MAX_PATH},
    System::SystemInformation::GetSystemDirectoryA,
    System::LibraryLoader::{GetProcAddress, LoadLibraryA}
};

type DInput8CreateFn = unsafe extern "system" fn(
    HINSTANCE,
    u32,
    *const GUID,
    *mut *mut c_void,
    *mut c_void
) -> HRESULT;

static mut ORIGINAL_DLL: Option<HMODULE> = None;
static mut DINPUT_CREATE: Option<DInput8CreateFn> = None;

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
        
        ORIGINAL_DLL = Some(module);
        log::info!("Loader", "Loaded original DLL with handle 0x{:08X}.", module.0 as usize);

        let method = GetProcAddress(ORIGINAL_DLL.unwrap(), s!("DirectInput8Create")).unwrap_or_die(|error| {
            log::fatal!("Loader", "Retrieving original function failed with code {}, aborting.", error.code());
        });

        DINPUT_CREATE = Some(std::mem::transmute(method));
        log::info!("Loader", "Retrieved original function from address 0x{:08X}.", method as usize);
    }
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "system" fn DirectInput8Create(
    hinst: HINSTANCE,
    dwVersion: u32,
    riidltf: *const GUID,
    ppvOut: *mut *mut c_void,
    punkOuter: *mut c_void
) -> HRESULT {
    unsafe {
        // We crash if the function or module doesn't exist, should be fine.
        (DINPUT_CREATE.unwrap())(hinst, dwVersion, riidltf, ppvOut, punkOuter)
    }
}