use windows::core::{s, PCSTR, Result};
use windows::Win32::{
    Foundation::GetLastError,
    System::Threading::ExitProcess,
    UI::WindowsAndMessaging::{MB_ICONERROR, MB_OK, MB_TOPMOST, MessageBoxA}
};

// This is strictly for error types returned by Win32 calls.
pub trait Win32Unwrap<T> {
    fn unwrap_or_die(self, msg: &'static str) -> T;
}

impl<T> Win32Unwrap<T> for Option<T> {
    fn unwrap_or_die(self, msg: &'static str) -> T {
        match self {
            Some(value) => value,
            None => {
                let error = unsafe { GetLastError() }.to_hresult();
                let text = format!("{}\nReason: {}\0", msg, error.message());

                unsafe {
                    let _ = MessageBoxA(
                        None,
                        PCSTR(text.as_ptr()),
                        s!("Uh-oh!"),
                        MB_OK | MB_ICONERROR | MB_TOPMOST
                    );
                }

                unsafe { ExitProcess(1) }
            }
        }
    }
}

impl<T> Win32Unwrap<T> for Result<T> {
    fn unwrap_or_die(self, msg: &'static str) -> T {
        match self {
            Ok(value) => value,
            Err(error) => {
                let text = format!("{}\nReason: {}\0", msg, error.message());

                unsafe {
                    let _ = MessageBoxA(
                        None,
                        PCSTR(text.as_ptr()),
                        s!("Uh-oh!"),
                        MB_OK | MB_ICONERROR | MB_TOPMOST
                    );
                }

                unsafe { ExitProcess(1) }
            }
        }
    }
}