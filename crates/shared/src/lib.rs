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