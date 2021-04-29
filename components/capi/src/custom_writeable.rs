use std::ffi::c_void;
use std::{fmt, ptr};

#[repr(C)]
/// An object that can one can write UTF-8 strings to
///
/// This allows the C API to write to arbitrary kinds of objects, for example a
/// C++ std::string or a char buffer.
///
/// [`icu4x_simple_writeable()`] can be used to write to a fixed-size char buffer.
///
/// May be extended in the future to support further invariants
///
/// ICU4XCustomWriteable will not perform any cleanup on `context` or `buf`, these are logically
/// "borrows" from the FFI side.
///
/// # Safety invariants:
///  - `flush()` and `grow()` will be passed `context` and the value should be valid for that
///     `context` may be  null, however `flush()` and `grow()` must then be ready to receive it
///  - `buf` must be `cap` bytes long
///  - `grow()` must either return null or a valid buffer of at least the requested buffer size
///  - Rust code must call `ICU4XCustomWriteable::flush()` before releasing to C
pub struct ICU4XCustomWriteable {
    /// Pointer to the actual object. While we're writing, we will write
    /// directly to `buf` without updating `context`'s state, this pointer exists so that
    /// `grow()` and `flush()` can get access to the full object on the foreign side.
    ///
    /// This can be null if the
    context: *mut c_void,
    /// The buffer to write directly to
    buf: *mut u8,
    /// The current filled size of the buffer
    len: usize,
    /// The current capacity of the buffer
    cap: usize,
    /// Called by Rust code when it is done writing, updating `context`
    /// with the new length
    flush: extern "C" fn(*mut c_void, usize),
    /// Called by Rust when it needs more capacity, passing
    /// in the requested capacity. Returns the new buffer, with
    /// the existing data already copied.
    ///
    /// Should return `null` if the requested capacity could not be achieved
    ///
    /// The capacity value will be updated if the actually allocated capacity is larger.
    grow: extern "C" fn(*mut c_void, *mut usize) -> *mut u8,
}

impl ICU4XCustomWriteable {
    /// Call this function before releasing the buffer to C
    pub fn flush(&mut self) {
        (self.flush)(self.context, self.len);
    }
}
impl fmt::Write for ICU4XCustomWriteable {
    fn write_str(&mut self, s: &str) -> Result<(), fmt::Error> {
        let needed_len = self.len + s.len();
        if needed_len > self.cap {
            let mut new_cap = needed_len;
            let newbuf = (self.grow)(self.context, &mut new_cap);
            if newbuf.is_null() {
                return Err(fmt::Error);
            }
            self.cap = new_cap;
            self.buf = newbuf;
        }
        debug_assert!(needed_len <= self.cap);
        unsafe {
            ptr::copy_nonoverlapping(
                s.as_bytes().as_ptr(),
                self.buf.offset(self.len as isize),
                s.len(),
            );
        }
        self.len = needed_len;
        Ok(())
    }
}

/// Create an `ICU4XCustomWriteable` that can write to a fixed-length stack allocated `u8` buffer.
///
/// Once done, this will append a null terminator to the written string.
#[no_mangle]
pub unsafe extern "C" fn icu4x_simple_writeable(buf: *mut u8, len: usize) -> ICU4XCustomWriteable {
    extern "C" fn grow(_context: *mut c_void, _cap: *mut usize) -> *mut u8 {
        ptr::null_mut()
    }
    extern "C" fn flush(context: *mut c_void, size: usize) {
        unsafe {
            let buf = context as *mut u8;
            ptr::write(buf.offset(size as isize), 0)
        }
    }
    ICU4XCustomWriteable {
        context: buf as *mut c_void,
        buf,
        len: 0,
        // keep an extra byte in our pocket for the null terminator
        cap: len - 1,
        flush,
        grow,
    }
}
