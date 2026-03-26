// You can call this crate silly but I'm not duplicating code.

mod logging;
mod cell_helper;
mod win32_unwrap;
pub use win32_unwrap::*;

pub mod log {
    pub use crate::{info, warn, fatal};
}

pub mod cell {
    pub use crate::{init, call, util};
}

#[macro_export]
macro_rules! hookdef {
    ($original:ident $name:ident ($($params:tt)*) $(-> $ret:ty)? { $($body:tt)* }) => {
        thread_local! {
            #[allow(non_snake_case)]
            static $original: std::cell::OnceCell<unsafe extern "system" fn(
                $($params)*
            ) $(-> $ret)?> = std::cell::OnceCell::new();
        }

        #[unsafe(no_mangle)]
        #[allow(non_snake_case)]
        pub unsafe extern "system" fn $name($($params)*) $(-> $ret)? {
            $($body)*
        }
    };
}