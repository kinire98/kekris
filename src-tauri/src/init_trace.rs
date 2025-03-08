use std::{env, sync::Once};

static INIT: Once = Once::new();

pub fn initialize() {
    INIT.call_once(|| {
        unsafe {
            env::set_var("RUST_BACKTRACE", "1");
        }
        color_eyre::install().unwrap();
    });
}
