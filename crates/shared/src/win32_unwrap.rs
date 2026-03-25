use windows::core::{Error, HRESULT, Result, s};
use windows::Win32::{
    System::Threading::ExitProcess,
    UI::WindowsAndMessaging::{MB_ICONERROR, MB_OK, MB_TOPMOST, MessageBoxA}
};

// This is strictly for error types returned by Win32 calls.
pub trait Win32Unwrap<T> {
    fn unwrap_or_die<F: FnOnce(Error)>(self, closure: F) -> T;
}

impl<T> Win32Unwrap<T> for Option<T> {
    fn unwrap_or_die<F: FnOnce(Error)>(self, closure: F) -> T {
        match self {
            Some(value) => value,
            None => {
                closure(HRESULT::from_thread().into());

                unsafe {
                    let _ = MessageBoxA(
                        None,
                        s!("A fatal error occurred. Please check the console for more information."),
                        s!("Uh-oh!"),
                        MB_OK | MB_ICONERROR | MB_TOPMOST
                    );

                    ExitProcess(1);
                }
            }
        }
    }
}

impl<T> Win32Unwrap<T> for Result<T> {
    fn unwrap_or_die<F: FnOnce(Error)>(self, closure: F) -> T {
        match self {
            Ok(value) => value,
            Err(error) => {
                closure(error);

                unsafe {
                    let _ = MessageBoxA(
                        None,
                        s!("A fatal error occurred. Please check the console for more information."),
                        s!("Uh-oh!"),
                        MB_OK | MB_ICONERROR | MB_TOPMOST
                    );
                }

                unsafe { ExitProcess(1) }
            }
        }
    }
}