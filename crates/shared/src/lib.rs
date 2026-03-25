// You can call this crate silly but I'm not duplicating code.

mod logging;
mod win32_unwrap;
pub use win32_unwrap::*;

pub mod log {
    pub use crate::{info, warn, fatal};
}