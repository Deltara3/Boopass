// Rendering hook implementation.
// Works in conjunction with C++.

macro_rules! write_lock {
    ($addr: expr, $size: expr, $old: expr, $section: literal, $body: block) => {
        unsafe { VirtualProtect($addr, $size, PAGE_EXECUTE_READWRITE, $old) }
            .unwrap_or_die(concat!("Failed to enable writing for the ", $section, " patch, exiting."));

        $body

        if let Err(error) = unsafe { VirtualProtect($addr, $size, *$old, $old) } {
            let msg = concat!("Failed to disable writing for the ", $section, " patch.");
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
    };
}

pub fn install() {

}