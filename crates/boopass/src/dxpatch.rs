// Rendering hook implementation.
// Works in conjunction with C++.

use std::{ptr, mem};
use std::ffi::c_void;
use shared::{Win32Unwrap, log};
use windows::core::{s, GUID, HRESULT, IUnknown};
use windows::Win32::{
    System::LibraryLoader::{GetModuleHandleA, GetProcAddress},
    System::Memory::{VirtualAlloc, VirtualProtect, PAGE_EXECUTE_READWRITE, MEM_COMMIT, MEM_RESERVE, PAGE_PROTECTION_FLAGS},
    System::Threading::ExitProcess,
    Graphics::Dxgi::{IDXGIFactory, IDXGISwapChain, DXGI_SWAP_CHAIN_DESC}
};

type CreateDXGIFactoryFn = unsafe extern "system" fn(
    *const GUID,
    *mut *mut c_void
) -> HRESULT;

type CreateSwapChainFn = unsafe extern "system" fn(
    *mut IDXGIFactory,
    *mut IUnknown,
    *mut DXGI_SWAP_CHAIN_DESC,
    *mut *mut IDXGISwapChain
) -> HRESULT;

const PATCH_SIZE: usize = 5;
static mut CREATE_DXGI_FACTORY: Option<CreateDXGIFactoryFn> = None;
static mut CREATE_SWAP_CHAIN: Option<CreateSwapChainFn> = None;

macro_rules! write_lock {
    ($section: literal, $addr: expr, $size: expr, $body: block) => {
        let mut old_protect = PAGE_PROTECTION_FLAGS(0);

        unsafe { VirtualProtect($addr, $size, PAGE_EXECUTE_READWRITE, &mut old_protect) }.unwrap_or_die(|error| {
            log::fatal!("Core", 
                "Enabling writing for the {} patch failed with code {}, aborting.",
                $section,
                error.code()
            );
        });

        $body

        if let Err(error) = unsafe { VirtualProtect($addr, $size, old_protect, &mut old_protect) } {
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

    write_lock!("CreateDXGIFactory", target as *const c_void, PATCH_SIZE, {
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
unsafe extern "system" fn hk_CreateDXGIFactory(
    riid: *const GUID, 
    factory: *mut *mut c_void
) -> HRESULT {
    // This is checked way before the hook gets called, unwrap should be fine.
    let hr = unsafe { (CREATE_DXGI_FACTORY.unwrap())(riid, factory) };

    if hr.is_ok() && !factory.is_null() && !unsafe { (*factory).is_null() } {
        let vtable = unsafe { *(*factory as *mut *mut *mut c_void) };
        let entry = unsafe { vtable.add(10) };

        log::info!("Core", "Found CreateSwapChain at entry address 0x{:08X}.", entry as usize);

        write_lock!("CreateSwapChain", entry as *const c_void, mem::size_of::<*mut c_void>(), {
            unsafe {
                let old_addr = *entry;
                let hook_addr = hk_CreateSwapChain as *mut c_void;

                CREATE_SWAP_CHAIN = Some(mem::transmute(old_addr));
                *entry = hook_addr;

                log::info!("Core", "Wrote hook address 0x{:08X} to 0x{:08X}", hook_addr as usize, old_addr as usize);
            }
        });
    } else {
        log::fatal!("Core", "CreateDXGIFactory failed with code 0x{:08X}.", hr.0);
    }

    return hr;
}

#[allow(non_snake_case)]
unsafe extern "system" fn hk_CreateSwapChain(
    factory: *mut IDXGIFactory,
    device: *mut IUnknown,
    desc: *mut DXGI_SWAP_CHAIN_DESC,
    swapchain: *mut *mut IDXGISwapChain
) -> HRESULT {
    unsafe {
        // Ditto of above, unwrap should be fine.
        (CREATE_SWAP_CHAIN.unwrap())(factory, device, desc, swapchain)
    }
}