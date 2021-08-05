// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use alloc::format;
use std::ffi::CString;
use std::io;
use std::os::raw::c_char;

use log::{Level, LevelFilter, Metadata, Record};

// minimal WASM logger based on https://github.com/DeMille/wasm-glue
extern "C" {
    fn log_js(ptr: *const c_char);
    fn warn_js(ptr: *const c_char);
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

static LOGGER: ConsoleLogger = ConsoleLogger;

#[no_mangle]
pub unsafe extern "C" fn icu4x_init() {
    log::set_logger(&LOGGER)
        .map(|()| log::set_max_level(LevelFilter::Debug))
        .unwrap();
}
