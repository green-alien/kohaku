
use std::sync::{Mutex, Once};
use std::borrow::BorrowMut;

/// engine settings and options
struct Config {
    debug: Option<bool>,
}

// fuckery
static mut CONF: Option<Mutex<Config>> = None;
static INIT: Once = Once::new();

/** config
 * get a reference to the struct representing the engine's configuation 
 * used internaly within this module only
 */
fn config<'a>() -> &'a Mutex<Config> {
    // call once black magic
    INIT.call_once(|| {
        unsafe {
            *CONF.borrow_mut() = Some(Mutex::new(
                Config {
                    debug: None,
                }
            ));
        }
    });

    unsafe { CONF.as_ref().unwrap() }
}
// https://www.sitepoint.com/rust-global-variables/#multithreadedglobalswithruntimeinitialization


/* DO NOT FUCK WITH "CONF" AFTER THIS POINT */

#[allow(dead_code)]
/** get debug */
pub fn get_debug() -> Option<bool> {
    config().lock().unwrap().debug
}

/** set debug */
pub fn set_debug(b: bool) -> () {
    config().lock().unwrap().debug = Some(b);
}