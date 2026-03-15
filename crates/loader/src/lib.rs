#[cfg(not(all(windows, target_arch = "x86")))]
compile_error!("compilation is only allowed on 32-bit windows");

use std::mem;
use std::ffi::c_void;
use windows::core::{BOOL, HRESULT, GUID, PCSTR, s};
use windows::Win32::Foundation::{HMODULE, HINSTANCE};
use windows::Win32::System::SystemServices::DLL_PROCESS_ATTACH;
use windows::Win32::System::SystemInformation::GetSystemDirectoryA;
use windows::Win32::System::LibraryLoader::{LoadLibraryA, GetProcAddress};

type DInput8CreateFn = extern "system" fn(
    HINSTANCE,
    u32,
    *const GUID,
    *mut *mut c_void,
    *mut c_void
) -> HRESULT;

static mut ORIGINAL_DLL: Option<HMODULE> = None;
static mut DINPUT_CREATE: Option<DInput8CreateFn> = None;

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub extern "system" fn DllMain(_module: HMODULE, reason: u32, _: *mut c_void) -> BOOL {
    match reason {
        DLL_PROCESS_ATTACH => {
            let dll_result = load_dinput();
            
            if dll_result != BOOL(1) {
                return dll_result;
            }
        
            /* We can probably just yolo this for now. */
            let _ = unsafe { LoadLibraryA(s!("boopass.dll")) };
        }
        _ => { /* Yay, do nothing! */ }
    }

    BOOL(1)
}

fn load_dinput() -> BOOL {
    unsafe {
        let mut path_buffer = [0u8; 260];
        let len = GetSystemDirectoryA(Some(&mut path_buffer));
        
        if len == 0 {
            return BOOL(0);
        }
        
        /* Do I love the unwrap here? No. Does it matter? Probably not. */
        let sys_dir = std::str::from_utf8(&path_buffer[..len as usize]).unwrap();
        let original_path = format!(r"{}\dinput8.dll", sys_dir);
        let raw_path = PCSTR::from_raw(original_path.as_ptr());
        
        if let Ok(dinput) = LoadLibraryA(raw_path) {
            ORIGINAL_DLL = Some(dinput);
        } else {
            return BOOL(0);
        }
        
        /* We already checked if it was valid, this unwrap should be fine? */
        if let Some(target_fn) = GetProcAddress(ORIGINAL_DLL.unwrap(), s!("DirectInput8Create")) {
            DINPUT_CREATE = Some(mem::transmute(target_fn));
        } else {
            return BOOL(0);
        }
    }
    
    BOOL(1)
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub extern "system" fn DirectInput8Create(
    hinst: HINSTANCE,
    dwVersion: u32,
    riidltf: *const GUID,
    ppvOut: *mut *mut c_void,
    punkOuter: *mut c_void
) -> HRESULT {
    unsafe {
        /* I think this unwrap is fine too. */
        (DINPUT_CREATE.unwrap())(hinst, dwVersion, riidltf, ppvOut, punkOuter)
    }
}
