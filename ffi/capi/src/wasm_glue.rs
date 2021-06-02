// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use std::ffi::CString;
use std::io;
use std::os::raw::c_char;

use log::{Level, LevelFilter, Metadata, Record};

#[cfg(debug_assertions)]
use std::panic;

// minimal WASM logger based on https://github.com/DeMille/wasm-glue
extern "C" {
    fn log_js(ptr: *const c_char);
    fn warn_js(ptr: *const c_char);

    #[allow(dead_code)] // we want a consistent set of externs
    fn trace_js(ptr: *const c_char);
}

fn _log(buf: &str) -> io::Result<()> {
    let cstring = CString::new(buf)?;

    unsafe {
        log_js(cstring.as_ptr());
    }

    Ok(())
}

fn _warn(buf: &str) -> io::Result<()> {
    let cstring = CString::new(buf)?;

    unsafe {
        warn_js(cstring.as_ptr());
    }

    Ok(())
}

struct ConsoleLogger;

impl log::Log for ConsoleLogger {
    fn enabled(&self, _: &Metadata) -> bool {
        true
    }

    fn log(&self, record: &Record) {
        if record.level() <= Level::Warn {
            _warn(format!("[{}] {}", record.level(), record.args()).as_str()).unwrap();
        } else {
            _log(format!("[{}] {}", record.level(), record.args()).as_str()).unwrap();
        }
    }

    fn flush(&self) {}
}

/// Sets a custom panic hook, uses your JavaScript `trace` function
#[cfg(debug_assertions)]
fn set_panic_hook() {
    panic::set_hook(Box::new(|info| {
        let file = info.location().unwrap().file();
        let line = info.location().unwrap().line();
        let col = info.location().unwrap().column();

        let msg = match info.payload().downcast_ref::<&'static str>() {
            Some(s) => *s,
            None => match info.payload().downcast_ref::<String>() {
                Some(s) => &s[..],
                None => "Box<Any>",
            },
        };

        let err_info = format!("Panicked at '{}', {}:{}:{}", msg, file, line, col);
        let cstring = CString::new(err_info).unwrap();

        unsafe {
            trace_js(cstring.as_ptr());
        }
    }));
}

static LOGGER: ConsoleLogger = ConsoleLogger;

#[no_mangle]
pub unsafe extern "C" fn icu4x_init() {
    #[cfg(debug_assertions)]
    set_panic_hook();

    log::set_logger(&LOGGER)
        .map(|()| log::set_max_level(LevelFilter::Debug))
        .unwrap();
}

#[no_mangle]
/// Allocates a buffer of a given size in Rust's memory.
///
/// # Safety
/// - The allocated buffer must be freed with [`icu4x_free()`].
pub unsafe extern "C" fn icu4x_alloc(size: usize) -> *mut u8 {
    let mut vec = Vec::<u8>::with_capacity(size);
    let ret = vec.as_mut_ptr();
    std::mem::forget(vec);
    ret
}

#[no_mangle]
/// Frees a buffer that was allocated in Rust's memory.
/// # Safety
/// - `ptr` must be a pointer to a valid buffer allocated by [`icu4x_alloc()`].
pub unsafe extern "C" fn icu4x_free(ptr: *mut u8, size: usize) {
    let vec = Vec::from_raw_parts(ptr, size, size);
    drop(vec);
}
