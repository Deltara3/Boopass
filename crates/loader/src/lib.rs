#[cfg(not(all(windows, target_arch = "x86")))]
compile_error!("compilation is only allowed on 32-bit windows");

use std::ffi::{c_void, CString};
use std::mem;
use shared::log;
use windows::core::{s, BOOL};
use windows::Win32::{
    Foundation::{HMODULE, NTSTATUS},
    System::LibraryLoader::LoadLibraryA,
    System::SystemServices::DLL_PROCESS_ATTACH,
    System::Console::{AttachConsole, ATTACH_PARENT_PROCESS},
    System::LibraryLoader::{GetModuleHandleA, GetProcAddress},
    System::SystemInformation::OSVERSIONINFOW
};

mod dinput;

type RtlGetVersionFn = unsafe extern "system" fn(*mut OSVERSIONINFOW) -> NTSTATUS;
type WineGetVersionFn = unsafe extern "system" fn() -> *const i8;

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub extern "system" fn DllMain(_module: HMODULE, reason: u32, _: *mut c_void) -> BOOL {
    if reason == DLL_PROCESS_ATTACH {
        // Attach to OpenParrotLoader's console.
        let _ = unsafe { AttachConsole(ATTACH_PARENT_PROCESS) };

        log_os_info();
        dinput::load();

        if let Err(error) = unsafe { LoadLibraryA(s!("boopass.dll")) } {
            log::warn!("Loader", "Loading Boopass failed with code {}, proceeding without.", error.code());
        }
    }

    BOOL(1)
}

fn log_os_info() {
    unsafe {
        // NTDLL should always be loaded and such such this shouldn't fail.
        let ntdll = GetModuleHandleA(s!("ntdll.dll")).unwrap();

        // This function should also always be present and this shouldn't fail.
        let rtl_get_version_ptr = GetProcAddress(ntdll, s!("RtlGetVersion")).unwrap();
        let rtl_get_version: RtlGetVersionFn = mem::transmute(rtl_get_version_ptr);

        let mut winver = OSVERSIONINFOW::default();
        let _ = rtl_get_version(&mut winver);

        if let Some(wine_get_version_ptr) = GetProcAddress(ntdll, s!("wine_get_version")) {
            let wine_get_version: WineGetVersionFn = mem::transmute(wine_get_version_ptr);

            // I don't think the unwrap matters here.
            let winever = CString::from_raw(wine_get_version() as *mut i8).into_string().unwrap();
            
            log::info!("Loader",
                "Starting on Windows v{}.{}.{} in Wine v{}",
                winver.dwMajorVersion, winver.dwMinorVersion, winver.dwBuildNumber,
                winever
            );
        } else {
            log::info!("Loader",
                "Starting on Windows v{}.{}.{}.",
                winver.dwMajorVersion, winver.dwMinorVersion, winver.dwBuildNumber
            );
        }
    }
}