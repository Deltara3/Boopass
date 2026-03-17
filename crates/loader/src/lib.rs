#[cfg(not(all(windows, target_arch = "x86")))]
compile_error!("compilation is only allowed on 32-bit windows");

use std::mem;
use std::ffi::c_void;
use windows::core::{BOOL, HRESULT, GUID, PCSTR, s};
use windows::Win32::Foundation::{HMODULE, HINSTANCE, GetLastError};
use windows::Win32::System::SystemServices::DLL_PROCESS_ATTACH;
use windows::Win32::System::SystemInformation::GetSystemDirectoryA;
use windows::Win32::System::LibraryLoader::{LoadLibraryA, GetProcAddress};
use windows::Win32::UI::WindowsAndMessaging::{MessageBoxA, MB_OK, MB_ICONERROR, MB_ICONWARNING};

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
        
            match unsafe { LoadLibraryA(s!("boopass.dll")) } {
                Ok(_) => {},
                Err(error) => {
                    let msg = format!("Failed to load Boopass, game will run without it.\nReason: {}\0", error.message());
                    let _ = unsafe { MessageBoxA(None, PCSTR(msg.as_ptr()), s!("Uh-oh!"), MB_OK | MB_ICONWARNING) };
                }
            }
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
        
        match LoadLibraryA(raw_path) {
            Ok(dinput) => { ORIGINAL_DLL = Some(dinput); },
            Err(error) => {
                let msg = format!("Failed to load original DLL.\nReason: {}\0", error.message());
                let _ = MessageBoxA(None, PCSTR(msg.as_ptr()), s!("Uh-oh!"), MB_OK | MB_ICONERROR);
                return BOOL(0);
            }
        }
        
        /* We already checked if load was successful, this unwrap should be fine? */
        match GetProcAddress(ORIGINAL_DLL.unwrap(), s!("DirectInput8Create")) {
            Some(target) => { DINPUT_CREATE = Some(mem::transmute(target)); },
            None => {
                let error = GetLastError().to_hresult();
                let msg = format!("Failed to get address for proxied function.\nReason: {}\0", error.message());
                let _ = MessageBoxA(None, PCSTR(msg.as_ptr()), s!("Uh-oh!"), MB_OK | MB_ICONERROR);
                return BOOL(0);
            }
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
        /* I think this unwrap is fine too as the DLL should exit before this is run. */
        (DINPUT_CREATE.unwrap())(hinst, dwVersion, riidltf, ppvOut, punkOuter)
    }
}
