use std::ffi::c_void;
use std::{fmt, ptr};

#[repr(C)]
/// An object that can one can write UTF-8 strings to
///
/// This allows the C API to write to arbitrary kinds of objects, for example a
/// C++ std::string or a char buffer.
///
/// The way to use this object is to fill out the `buf`, `len`, `cap` fields with
/// appropriate values for the buffer, its current length, and its current capacity,
/// and `flush` and `grow` with appropriate callbacks (using `context` to reference any
/// state they need). This object will be passed by mutable reference to the Rust side,
/// and Rust will write to it, calling `grow()` as necessary. Once done, it will call `flush()`
/// to update any state on `context` (e.g. adding a null terminator, updating the length).
/// The object on the foreign side will be directly usable after this, the foreign side
/// need not perform additional state updates after passing an [`ICU4XCustomWriteable`] to
/// a function.
///
/// [`icu4x_simple_writeable()`] can be used to write to a fixed-size char buffer.
///
/// May be extended in the future to support further invariants
///
/// ICU4XCustomWriteable will not perform any cleanup on `context` or `buf`, these are logically
/// "borrows" from the FFI side.
///
/// # Safety invariants:
///  - `flush()` and `grow()` will be passed `context` and it should always be safe to do so.
///     `context` may be  null, however `flush()` and `grow()` must then be ready to receive it as such.
///  - `buf` must be `cap` bytes long
///  - `grow()` must either return null or a valid buffer of at least the requested buffer size
///  - Rust code must call `ICU4XCustomWriteable::flush()` before releasing to C
pub struct ICU4XCustomWriteable {
    /// Context pointer passed to `grow()` and `flush()`. May be `null`.
    ///
    /// The pointer may reference structured data on the foreign side,
    /// such as C++ std::string, used to reallocate buf.
    context: *mut c_void,
    /// The raw string buffer, which will be mutated on the Rust side.
    buf: *mut u8,
    /// The current filled size of the buffer
    len: usize,
    /// The current capacity of the buffer
    cap: usize,
    /// Called by Rust to indicate that there is no more data to write.
    ///
    /// Arguments:
    /// - `context` (`*mut c_void`): The `context` field of this struct
    /// - `length` (`usize`): The final length of the string in `buf`
    flush: extern "C" fn(*mut c_void, usize),
    /// Called by Rust to request more capacity in the buffer. The implementation should allocate a new
    /// buffer and copy the contents of the old buffer into the new buffer.
    ///
    /// Arguments:
    /// - `context` (`*mut c_void`): The `context` field of this struct.
    /// - `capacity` (`*mut usize`): The requested capacity. Should be updated to reflect
    ///    the actual capacity if the allocated buffer is larger than was requested.
    ///
    /// Returns: the newly allocated buffer, or `null` if allocation failed.
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
pub unsafe extern "C" fn icu4x_simple_writeable(
    buf: *mut u8,
    buf_size: usize,
) -> ICU4XCustomWriteable {
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
        cap: buf_size - 1,
        flush,
        grow,
    }
}
