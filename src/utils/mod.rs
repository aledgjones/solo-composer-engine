extern crate web_sys;

pub mod duration;
pub mod measurements;
pub mod pitch;
pub mod storage;
pub mod text;

use rand::distributions::Alphanumeric;
use rand::{thread_rng, Rng};

pub fn set_panic_hook() {
    #[cfg(feature = "console_error_panic_hook")]
    console_error_panic_hook::set_once();
}

// A macro to provide `println!(..)`-style syntax for `console.log` logging.
#[macro_export]
macro_rules! log {
    ( $( $t:tt )* ) => {
        web_sys::console::log_1(&format!( $( $t )* ).into());
    }
}

pub fn shortid() -> String {
    thread_rng().sample_iter(&Alphanumeric).take(14).collect()
}
