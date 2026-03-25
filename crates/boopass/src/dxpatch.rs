// Rendering hook implementation.
// Works in conjunction with C++.

use std::{ptr, mem};
use std::ffi::c_void;
use shared::{Win32Unwrap, log};
use windows::Win32::System::Threading::ExitProcess;
use windows::core::{GUID, HRESULT, s};
use windows::Win32::{
    System::LibraryLoader::{GetModuleHandleA, GetProcAddress},
    System::Memory::{VirtualAlloc, VirtualProtect, PAGE_EXECUTE_READWRITE, MEM_COMMIT, MEM_RESERVE, PAGE_PROTECTION_FLAGS}
};

type CreateDXGIFactoryFn = unsafe extern "system" fn(*const GUID, *mut *mut c_void) -> HRESULT;

const PATCH_SIZE: usize = 5;
static mut CREATE_DXGI_FACTORY: Option<CreateDXGIFactoryFn> = None;

macro_rules! write_lock {
    ($section: literal, $addr: expr, $size: expr, $old: expr, $body: block) => {
        unsafe { VirtualProtect($addr, $size, PAGE_EXECUTE_READWRITE, $old) }.unwrap_or_die(|error| {
            log::fatal!("Core", 
                "Enabling writing for the {} patch failed with code {}, aborting.",
                $section,
                error.code()
            );
        });

        $body

        if let Err(error) = unsafe { VirtualProtect($addr, $size, *$old, $old) } {
            log::warn!("Core", 
                "Disabling writing for the {} patch failed with code {}.",
                $section,
                error.code()
            );
        }
    };
}

pub fn install() {
    let dxgi = unsafe { GetModuleHandleA(s!("dxgi.dll")) }.unwrap_or_die(|error| {
        log::fatal!("Core", "Retrieving handle for DXGI failed with code {}, aborting.", error.code());
    });

    log::info!("Core", "Retrieved DXGI with handle 0x{:08X}.", dxgi.0 as usize);

    let target = unsafe { GetProcAddress(dxgi, s!("CreateDXGIFactory")) }.unwrap_or_die(|error| {
        log::fatal!("Core", "Locating CreateDXGIFactory failed with code {}, aborting.", error.code());
    });

    log::info!("Core", "Found CreateDXGIFactory at address 0x{:08X}.", target as usize);

    let detour = unsafe {
        VirtualAlloc(
            None,
            PATCH_SIZE + 5,
            MEM_COMMIT | MEM_RESERVE,
            PAGE_EXECUTE_READWRITE
        )
    } as *mut u8;

    if detour.is_null() {
        log::fatal!("Core", "Failed to allocate memory for trampoline, aborting.");
        unsafe { ExitProcess(1) };
    }

    log::info!("Core", "Allocated memory for trampoline at address 0x{:08X}.", detour as usize);

    unsafe { ptr::copy_nonoverlapping(target as *const u8, detour, PATCH_SIZE) };

    let return_addr = unsafe { (target as *const c_void).add(PATCH_SIZE) };
    let relative_back = return_addr as isize - unsafe { detour.add(PATCH_SIZE) } as isize - 5;

    unsafe {
        *detour.add(PATCH_SIZE) = 0xE9;
        ptr::write_unaligned(detour.add(PATCH_SIZE + 1) as *mut u32, relative_back as u32);
    }

    log::info!("Core", "Wrote trampoline for address 0x{:08X}.", return_addr as usize + PATCH_SIZE);

    let mut old_protect = PAGE_PROTECTION_FLAGS(0);

    write_lock!("CreateDXGIFactory", target as *const c_void, PATCH_SIZE, &mut old_protect, {
        let hook_addr = hk_CreateDXGIFactory as *const c_void;
        let relative_to = hook_addr as isize - target as isize - 5;

        unsafe {
            let mut patch = [0u8; PATCH_SIZE];
            patch[0] = 0xE9;
            ptr::write_unaligned(patch.as_mut_ptr().add(1) as *mut u32, relative_to as u32);

            ptr::copy_nonoverlapping(patch.as_ptr(), target as *mut u8, PATCH_SIZE);
            log::info!("Core", "Wrote jump to 0x{:08X} at 0x{:08X}", hook_addr as usize, target as usize);
        }
    });

    unsafe { CREATE_DXGI_FACTORY = Some(mem::transmute(detour)) };
}

#[allow(non_snake_case)]
unsafe extern "system" fn hk_CreateDXGIFactory(riid: *const GUID, factory: *mut *mut c_void) -> HRESULT {
    unsafe { (CREATE_DXGI_FACTORY.unwrap())(riid, factory) }
}