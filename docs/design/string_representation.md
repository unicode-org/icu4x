# String representation on the library API boundary

(This document is scoped to string representation on the outer boundary of the library.)

## Introduction

ICU4X assumes that not all callers are Rust programs that follow the Rust practice of using guaranteed-valid UTF-8 for representing text. Browser engines and other large code bases dating from the 1990s also use (potentially-invalid) UTF-16 internally for many things. Furthermore, systems that have come to regret the commitment to UTF-16 often implement optimizations that store the code units without the high half if all the code units in a string have the high half zeroed. For example, SpiderMonkey, V8, optionally HotSpot, and text nodes in the Gecko DOM use this optimization. In this document, this representation is referred to as Latin1. (This is not the same as windows-1252.) Additionally, Go uses UTF-8 by convention, but doesn't guarantee the validity. In practice, C and C++ lack strong enough facilities for maintaining UTF-8 validity for sure, so it is prudent to assume that UTF-8 coming from C or C++ callers might be invalid. ICU4X should cater to these non-Rust callers in a way that ideally does not compromise performance.

In addition to catering to encoding forms other than guaranteed-valid UTF-8, the need not to compromise performance when serving non-Rust callers brings up the issue of transferring text over the FFI boundary in a performant way. For example, some Rust-only text processing APIs might make sense as iterator adaptors: iterators that yield `char` items after performing processing on `char` items obtained from the delegate iterator. Such an API relies heavily on the compiler being able to optimize across the consumer of the iterator adaptor, the iterator implementation, and the concrete implementation of the delegate iterator. This cannot be assumed to work across the FFI boundary, especially if the caller language is not being ahead-of-time compiled with LLVM and link-time optimized together with the Rust code.

Therefore, ICU4X needs to provide API surface other than a mere Rust `String` or iterators over `char`.

## Prefer working with caller-allocated memory

Returning string objects whose type is dictated by ICU4X would likely lead to callers having to copy data out of such strings into whatever is idiomatic for them and then to free the ICU4X-returned object. If the caller manages the location and ICU4X writes into that allocation, there is a chance to avoid a copy in some scenarios.

Specifically, these scenarios are considered:

* The caller uses an exact allocator, i.e. requesting an allocation of N bytes actually allocates N bytes.
* The caller uses a bucket allocator, e.g. requesting an location of N bytes actually allocates N rounded up to the next power up two. (Gecko uses a bucket allocator, so this case is particularly relevant to Mozilla.)
* The caller uses a bump allocator that at most can realloc the most recent allocation and otherwise can't free anything, i.e. memory is reclaimed by the process terminating.

Historically, it appears that a lot of APIs have been designed for the first case with the assumption that allocating the exact number of bytes is super-important and as if the allocation operation was substantially more expensive than figuring out what the exact location size is. A typical C API pattern is that if you pass `NULL` as the pointer to the output buffer, the API doesn't write anything but performs all the processing for computing how many code units would have been written so that the caller can then allocate a buffer exactly to that size and then call the API again with a pointer to that buffer. This doesn't make much sense if the worst-case size is within a reasonable factor of the input size and the processing to compute the exact buffer size is substantially larger than the cost of another allocation, `memcpy`, and freeing the temporary worst-case sized buffer.

If we assumed that in every case we'd first allocate for the worst case, write into that buffer (finding out the exact size needed in the process), and then `memcpy`ed into an exact-sized buffer, we might as well make ICU4X allocated the worst-case buffer, return it, have the caller `memcpy` the data from that buffer, and then ask ICU4X to free the worst-case sized buffer. However, this approach would end up copying more often than strictly necessary in the two other scenarios as well is in the scenario where an exact allocator can shrink an allocation without copying.

With a bucket allocator, if the worst case and the actual case rounded up to the same bucket size, there is no need to copy. In that case, it would be beneficial for the caller to be able to manage the buffer to begin with and not to have to deal with a buffer that has to be freed by ICU4X. If an exact allocator can shrink an allocation without copying, it is likewise beneficial for the caller not to have to deal with a buffer that has to be freed by ICU4X and to instead deal with the buffer that it itself can realloc down to the needed size.

In the bump allocator case, if ICU4X didn't call malloc internally, realloc can shrink the buffer in place by adjusting the bump. Otherwise, the bump allocator is unable to reclaim memory and allocating a new smaller buffer is useless: it would just consume more memory. Either way, leaving the allocation policy to the caller makes sense.

Notably, allocating for the worst case and shrinking at most once result in fewer allocations than performing append operations to the sink that has to allocate backing buffer without knowing about future appends. Such a sink (for example Rust `String`) will likely end up both over-allocating and allocating and copying more than once.

For operation where the common case is the same as the input case or with a bucket allocator rounds to the same bucket, we can do even better with a restartable API. Then the wrapper can first allocate as many output code units as there are input code units, try the operation, and, if the full output didn't fit, reallocate the buffer to the worst case (copying the output so far), continue with processing the rest of the input, and then finally possibly shrink the buffer. This results in the worst case for number of allocations being three with the common case being one rather than two.

ICU4X operations therefore should, where possible taking into account the nature of the operation, provide API surface that:

* Takes input as slice
* Writes output to a slice
* Returns the number of code units read and the number of code units written such that if the former equals the length of the input, there's nothing more to be done, and otherwise it's OK to continue from the input subslice starting at the code unit index equaling to the number of code unit previously read. (That is, the implementation may have actually examined code units further. The number of code units read means the number of code units that correspond to the number of code units written.)
* Provides a quick worst-case estimator function that takes the number of code units of input and returns the worst case number of code units in output

It is expected that text transformation functions, such as case changes and normalization will fit into this category in terms of output mechanism.

## Supported encoding forms

The following encoding forms are reasonable candidates to be supported by a given ICU4X operation. Since Rust doesn't have function overloading based on argument type, what are logically overloads need to be signified by naming convention.

### Potentially-invalid UTF-8

This is what Go uses natively and what UTF-8-using C and C++ code bases can be trusted to use. For input, the Rust type is `&[u8]`. For output, it is `&mut [u8]`. The name annotation is `_utf8`. In the headers for C and C++, the slice pointer shall be of type `char8_t*` (with const and without, respectively). For versions of C and C++ that don't support `char8_t`, ICU4X shall provide a preprocessor polyfill that `typedef`s `char8_t` as `unsigned char`.

On the input side, ICU4X must iterate over the input such that ill-formed sequences are treated as the REPLACEMENT CHARACTER according to the WHATWG Encoding Standard. (Unicode 13.0 does not require this behavior but points the reader to the W3C snapshot of the Encoding Standard on this point. Older versions called this "best practice".) This is consistent with what the Rust standard library does if it is asked to convert from `&[u8]` to `String`. Unfortunately, this does not match the REPLACEMENT CHARACTER generation behavior built into Go's iterators, so a Go wrapper for ICU4X would behave inconsistently with Go's language behavior on this point. Documentation for future Go wrapper for ICU4X should acknowledge this but otherwise not to do anything special about it.

On the output side, ICU4X must guarantee that it actually writes valid UTF-8 into the output buffer even though the `&mut [u8]` type does not communicate this guarantee via the Rust type system. If the API is incremental, i.e. the logical output stream can be split across many API calls, a UTF-8 sequence must not be split across output slices even if it means not filling the earlier slice completely.

A given ICU4X operation must always provide a version that accepts potentially invalid UTF-8 and outputs guaranteed-valid UTF-8 into a byte slice.

### Guaranteed-valid UTF-8

This is what Rust uses natively. C and C++ programmers who want maximum performance and are confident in their code correctness in a particular place in their code could use this for specific things. For input, the type is `&str`, which is idiomatic Rust. For output, the type is `&mut str`, which isn't quite idiomatic Rust. (In particular, the language currently doesn't allow materializing a zero-initialized stack-allocated `&mut str` without `unsafe`, even though there is no fundamental reason why this operation could not be provided in the language without `unsafe`.) The pointer types in C and C++ header are the same as in the potentially-invalid case.

In the Rust context, the name annotation is `_str`. In the FFI context of the input side, the main limitation is `_utf8_unsafe`. The point of the different annotations is that of the Rust side, thanks to the guarantees of Rust, calling these functions is safe. However, when called from the language that doesn't have Rust's guarantees as part of the language, it is up to the programmer to ensure that the input is valid UTF-8. Otherwise, Undefined Behavior ensues. Hence the "unsafe" designation in FFI, i.e. the C API. (This type is irrelevant in the FFI context on the output side.)

On the input side, `&str` differs from `&[u8]` holding potentially-invalid UTF-8 in performance. The former can be iterated over by Unicode scalar value without having to have branches that take into account invalid byte sequences. In this sense, the distinction between the function that takes `&str` and a function that takes UTF-8 `&[u8]` is one of performance. Correctness-wise, it is always okay for the function that takes `&str` to call `as_bytes()` on the argument and then delegate to the function that takes `&[u8]`. When performance of the operation is more important than minimizing binary size, the function that takes `&str` should make use of the knowledge that there are no ill-formed sequences.

On the outside, the function that takes `&mut str` always delegates to the version that takes `&mut [u8]` and trailing loop that zeros bytes after the last byte written by the delegate function until either the end of the slice or UTF a lead byte is reached. This is enough to uphold the invariant of `&mut str`.

A given ICU4X operation that outputs text must always provide a version, whose input type is `&str` and output type is `&mut [u8]` (this one gets exposed via FFI as `_unsafe_utf8`), the version whose input type is `&str` and output type is `&mut str`, and a version whose input type is `&str` and that returns a `String` instead of having an output argument. The last two or always implemented as a thin mechanical wrappers around the first one. If performance considerations permit, the first one may just delegate to the version that takes `&[u8]` input. The function that returns a `String` has no name annotation.

### Potentially-invalid UTF-16

This is what many systems whose compatibility constraints date back to the 1990s use. For input, the Rust type is `&[u16]`. For output, it is `&mut [u16]`. The name annotation is `_utf16`. In the headers for C and C++, the slice pointer shall be of type `char16_t*` (with const and without, respectively). 

On the input side, ICU4X must iterate over the input such that each unpaired surrogate is treated as the REPLACEMENT CHARACTER. On the output side, ICU4X must guarantee UTF-16 validity. If the API is incremental, i.e. the logical output stream can be split across many API calls, the surrogate pair not be split across output slices even if it means not filling the earlier slice completely.

A given ICU4X operation must provide a version that takes potentially-invalid UTF-16 and outputs guaranteed-valid UTF-16.

### Latin1

This is what SpiderMonkey and V8 use as an optimization for strings whose semantics are potentially-invalid UTF-16 but for whose each UTF-16 code unit would have zero bits and the higher half. For input, the Rust type is `&[u8]`. For output, it is `&mut [u8]`. The name annotation is `_latin1`. In the headers for C and C++, the slice pointer shall be of type `char*` (with const and without, respectively). (Note that while it is incorrect, due to sign extension issues on the ABI boundary, to declare `char` as non-pointer argument whose Rust type is Rust `u8`, it is OK to declare `const char*` for `*const u8` and `char*` for `*mut u8`.)

A given ICU4X operation may provide a version that takes Latin1 when it makes sense for performance reasons. If the operation is known to keep the domain of the output within the Latin1 range, the output slice should be Latin1 as well. Otherwise, the output slice should be UTF-16. The case where the output range is known to be within the Latin1 range is expected to be very rare or non-existent: i.e. the common case for Latin1 input is expected to be UTF-16 output.

### Encoding forms deliberately not included

Guaranteed-valid UTF-16 is not in use in practice. The use of UTF-32 is rare enough that it's not worth supporting.

## Non-resumable operations

If the model of returning from an operation early with number of code units read so far and the number of code units written is not suitable for a given operation, it is OK to make the operation non-resumable such that it doesn't return the number of code units read but instead requires the output buffer to be sized to the worst case and panics if the output buffer is too small. This, of course, requires the operation to be one with a reasonable worst-case estimate.

## Operations that are neither resumable nor have a reasonable worst-case estimate

For operations that don't fit into either caller-allocated model, ICU4X should return `String` in the UTF-8 case and `Vec<u16>` in the UTF-16 case. (It's unlikely that there are operations whose output range is Latin1 but that don't have a reasonable worst-case number of code units estimate.)

FFI must then hand out the length, capacity, and the internal buffer of the `String`/`Vec`.

ICU4X must provide functions `icu_free_utf8` and `icu_free_utf16` for freeing these.

It is expected that text formatting functions will fit into this output category. That is, this output category, while the third option in priority, is expected to be very common.

## Iterators

For Rust callers, a given ICU4X operation may be provided as a version that takes input via an iterator that yields `char` and provides output by implementing an iterator that yields `char`. For some operations, this may be the sole internal implementation such that the slice versions are merely wrapper around this one. However, for operations that need lookahead, the slice operations may be implemented more efficiently by performing lookahead on the slice as opposed to having an iterator implementation that allocates internally in order to turn lookahead into buffering. The name annotation for the function that creates the iterator is `_iter`.

## Encoding over FFI

We tend to try and match the FFI ecosystem's approach to encoding where possible. So, in JS/WASM, Rust APIs that take in `&str` will instead consume a JS `String` and convert it to UTF-8 on the fly. On the other hand, in C we take in a `char*` and a length, and in C++ we take a `std::string_view`, and assume that they are UTF-8.

The `char8_t` and `std::u8string_view` types were considered but not chosen due to compatibility concerns. `u8"..."` style literals are recommended to be used if available as they will be compatible with `std::string_view`.


## Example: `to_lowercase` without locale-sensitive behavior

This example is instructive, because, due to final sigma, a slice-based implementation can work without heap allocation, but an iterator-based implementation potentially has to allocate on the heap, so specializing the implementation for slices instead of just delegating them to an iterator avoids heap allocation. Furthermore, when there are no locale-sensitive behaviors, the Latin1 case stays within range. (Not true for the uppercasing direction due to 'ÿ'. Also, not true for 'I' when lowercasing with Turkish locale.)

```rust
use iterlower::IterLowercase;

// Start functions that actually require thinking

/// Takes an iterator that yields Unicode scalar values and returns
/// an iterator adapter that yields a lower-cased stream of those
/// Unicode scalar values. Buffers internally on the heap when
/// looking ahead to see if a capital sigma is a final sigma.
pub fn to_lowercase_iter<I: Iterator<Item = char>>(iter: I) -> impl Iterator<Item = char> {
    // (see the iterlower crate for an example; calling that crate here
    // to make this example compile.)
    iter.to_lowercase(false)
}

/// Given UTF-8 input of `len` code units, returns the worst-case
/// number of UTF-8 code units in the output or `None` on overflow.
/// Overflow still relevant for 32-bit systems. Sigh.
pub fn to_lowercase_utf8_max(len: usize) -> Option<usize> {
    len.checked_mul(3) // likely not the real factor
}

/// Given UTF-16 input of `len` code units, returns the worst-case
/// number of UTF-16 code units in the output or `None` on overflow.
/// Overflow still relevant for 32-bit systems. Sigh.
pub fn to_lowercase_utf16_max(len: usize) -> Option<usize> {
    len.checked_mul(3) // possibly not the real factor
}

/// Given Latin1 input of `len` code units, returns the worst-case
/// number of Latin1 code units in the output or `None` on overflow.
/// Overflow still relevant for 32-bit systems. Sigh.
///
/// Known to be identity function but included for illustration
/// purposes.
pub fn to_lowercase_latin1_max(len: usize) -> Option<usize> {
    Some(len)
}

/// Reads potentially-invalid UTF-8 from `src` and writes corresponding
/// lower-cased text into `dst` as guaranteed-valid UTF-8. Returns the
/// number of code units read and the number of code units written. The
/// number of code units read is guaranteed to be `src.len()` if
/// `dst.len() >= to_lowercase_utf8_max(src.len()).unwrap()`.
pub fn to_lowercase_utf8(src: &[u8], dst: &mut [u8]) -> (usize, usize) {
    // Iterate over `src` taking into account ill-formed sequences.

    // Inside the loop, call by `char` into property lookup shared by all of these.
    // In case of capital sigma, scan forward in an inner loop.

    // Including placeholder code that compiles.
    (src.len(), dst.len())
}

/// Reads UTF-8 from `src` and writes corresponding lower-cased text
/// into `dst` as guaranteed-valid UTF-8. Returns the
/// number of code units read and the number of code units written. The
/// number of code units read is guaranteed to be `src.len()` if
/// `dst.len() >= to_lowercase_utf8_max(src.len()).unwrap()`.
pub fn to_lowercase_str_utf8(src: &str, dst: &mut [u8]) -> (usize, usize) {
    // EITHER for performance iterate over `src` _without_ taking into
    // account ill-formed sequences OR just call
    // to_lowercase_utf8(src.as_bytes(), dst) for smaller binary size.

    // Inside the loop, call by `char` into property lookup shared by all of these.
    // In case of capital sigma, scan forward in an inner loop.

    // Including placeholder code that compiles.
    (src.len(), dst.len())
}

/// Reads potentially-invalid UTF-16 from `src` and writes corresponding
/// lower-cased text into `dst` as guaranteed-valid UTF-16. Returns the
/// number of code units read and the number of code units written. The
/// number of code units read is guaranteed to be `src.len()` if
/// `dst.len() >= to_lowercase_utf16_max(src.len()).unwrap()`.
pub fn to_lowercase_utf16(src: &[u16], dst: &mut [u16]) -> (usize, usize) {
    // Iterate over `src` taking into account ill-formed sequences.

    // Inside the loop, call by `char` into property lookup shared by all of these.
    // In case of capital sigma, scan forward in an inner loop.

    // Including placeholder code that compiles.
    (src.len(), dst.len())
}

/// Reads Latin1 from `src` and writes corresponding
/// lower-cased text into `dst` as Latin1. Returns the
/// number of code units read and the number of code units written. The
/// number of code units read is guaranteed to be `src.len()` if
/// `dst.len() >= to_lowercase_latin1_max(src.len()).unwrap()`.
pub fn to_lowercase_latin1(src: &[u8], dst: &mut [u8]) -> (usize, usize) {
    // Iterate over `src`.

    // Inside the loop, index into a by-byte lookup table.

    // Including placeholder code that compiles.
    (src.len(), dst.len())
}

/// Some operation whose output size doesn't have a reasonable
/// estimator function.
pub fn inestimable(src: &str) -> String {
    // Placeholder implementation that compiles
    String::with_capacity(src.len())
}

/// Some operation whose output size doesn't have a reasonable
/// estimator function. (Version that accepts potentially-invalid
/// UTF-8.)
///
/// The returned `String` may have excess capacity. If you are holding onto
/// it for an extended period of time, consider calling `shrink_to_fit()`
/// on it.
pub fn inestimable_utf8(src: &[u8]) -> String {
    // Placeholder implementation that compiles
    String::with_capacity(src.len())
}

/// Some operation whose output size doesn't have a reasonable
/// estimator function. (Version that accepts potentially-invalid
/// UTF-16.)
///
/// The returned `Vec` may have excess capacity. If you are holding onto
/// it for an extended period of time, consider calling `shrink_to_fit()`
/// on it.
pub fn inestimable_utf16(src: &[u16]) -> Vec<u16> {
    // Placeholder implementation that compiles
    Vec::with_capacity(src.len())
}

// End functions that actually require thinking

// Start Rust-only boilerplate

/// Returns a `String` whose contents are `src` lower-cased.
///
/// The returned `String` may have excess capacity. If you are holding onto
/// it for an extended period of time, consider calling `shrink_to_fit()`
/// on it.
pub fn to_lowercase(src: &str) -> String {
    // This is the same boilerplate for all String`-returning wrappers
    // and should be generated using a macro.
    let mut vec = Vec::new();
    // First assume the same length
    vec.resize(src.len(), 0);
    let (read_first, written_first) = to_lowercase_str_utf8(src, &mut vec);
    let mut total_written = written_first;
    let input_left = src.len() - read_first;
    if input_left != 0 {
        vec.resize(
            written_first + to_lowercase_utf8_max(input_left).unwrap(),
            0,
        );
        let (read_second, written_second) =
            to_lowercase_str_utf8(&src[read_first..], &mut vec[written_first..]);
        debug_assert_eq!(read_first + read_second, src.len());
        total_written += written_second;
    }
    // Update to `shrink_to` once stable.
    vec.resize(total_written, 0);
    unsafe {
        // Unsafe OK, because we believe we can write valid
        // UTF-8 as promised.
        String::from_utf8_unchecked(vec)
    }
}

/// Reads UTF-8 from `src` and writes corresponding lower-cased text
/// into `dst` as guaranteed-valid UTF-8. Returns the
/// number of code units read and the number of code units written. The
/// number of code units read is guaranteed to be `src.len()` if
/// `dst.len() >= to_lowercase_utf8_max(src.len()).unwrap()`.
pub fn to_lowercase_str(src: &str, dst: &mut str) -> (usize, usize) {
    // This is the same boilerplate for all `&mut str`-writing wrappers
    // and should be generated using a macro.

    // Unsafe OK, because we believe we can write valid
    // UTF-8 as promised.
    let dst_bytes = unsafe { dst.as_bytes_mut() };
    let (read, written) = to_lowercase_str_utf8(src, dst_bytes);
    // Now uphold the str invariant for the tail that was not written to.
    // This loop runs at most 3 iterations: The worst case is that the
    // last byte that the above call overwrote was the lead of a 4-byte
    // sequence.
    let mut trail = written;
    while trail < dst_bytes.len() && ((dst_bytes[trail] & 0xC0) == 0x80) {
        dst_bytes[trail] = 0;
        trail += 1;
    }
    (read, written)
}

// End Rust-only boilerplate

// Start FFI boilerplate

// Note that these will keep everything alive for dead code elimination purposes
// if exported from a shared library, so there should probably be
// cargo features for manually managing which ones of these to export.

/// This function can be replaced with `Vec::into_raw_parts()` once it
/// becomes available on the release channel.
fn vec_into_raw_parts<T>(vec: Vec<T>) -> (*mut T, usize, usize) {
    let mut vec = ::std::mem::ManuallyDrop::new(vec);
    let len = vec.len();
    let capacity = vec.capacity();
    let ptr = vec.as_mut_ptr();
    (ptr, len, capacity)
}

/// `usize` as `size_t` in .h.
/// `SIZE_MAX` means overflow, i.e. the operation is saturating.
#[no_mangle]
pub unsafe extern "C" fn icu_to_lowercase_utf8_max(len: usize) -> usize {
    to_lowercase_utf8_max(len).unwrap_or(::std::usize::MAX)
}

/// `usize` as `size_t` in .h.
/// `SIZE_MAX` means overflow, i.e. the operation is saturating.
#[no_mangle]
pub unsafe extern "C" fn icu_to_lowercase_utf16_max(len: usize) -> usize {
    to_lowercase_utf16_max(len).unwrap_or(::std::usize::MAX)
}

/// `usize` as `size_t` in .h.
/// `SIZE_MAX` means overflow, i.e. the operation is saturating.
#[no_mangle]
pub unsafe extern "C" fn icu_to_lowercase_latin1_max(len: usize) -> usize {
    to_lowercase_latin1_max(len).unwrap_or(::std::usize::MAX)
}

/// `usize` as `size_t` and `u8` as `char8_t` in .h.
/// `src_len` and `dsc_len` are inout params.
#[no_mangle]
pub unsafe extern "C" fn icu_to_lowercase_utf8(
    src: *const u8,
    src_len: *mut usize,
    dst: *mut u8,
    dst_len: *mut usize,
) {
    let src_slice = ::std::slice::from_raw_parts(src, *src_len);
    let dst_slice = ::std::slice::from_raw_parts_mut(dst, *dst_len);
    let (read, written) = to_lowercase_utf8(src_slice, dst_slice);
    *src_len = read;
    *dst_len = written;
}

/// `usize` as `size_t` and `u8` as `char8_t` in .h.
/// `src_len` and `dsc_len` are inout params.
///
/// It is UNDEFINED BEHAVIOR for `src` and `*src_len` to designate
/// a buffer that is not valid UTF-8!
#[no_mangle]
pub unsafe extern "C" fn icu_to_lowercase_utf8_unsafe(
    src: *const u8,
    src_len: *mut usize,
    dst: *mut u8,
    dst_len: *mut usize,
) {
    let src_slice = ::std::slice::from_raw_parts(src, *src_len);
    let dst_slice = ::std::slice::from_raw_parts_mut(dst, *dst_len);
    let str_slice = ::std::str::from_utf8_unchecked(src_slice);
    let (read, written) = to_lowercase_str_utf8(str_slice, dst_slice);
    *src_len = read;
    *dst_len = written;
}

/// `usize` as `size_t` and `u16` as `char16_t` in .h.
/// `src_len` and `dsc_len` are inout params.
#[no_mangle]
pub unsafe extern "C" fn icu_to_lowercase_utf16(
    src: *const u16,
    src_len: *mut usize,
    dst: *mut u16,
    dst_len: *mut usize,
) {
    let src_slice = ::std::slice::from_raw_parts(src, *src_len);
    let dst_slice = ::std::slice::from_raw_parts_mut(dst, *dst_len);
    let (read, written) = to_lowercase_utf16(src_slice, dst_slice);
    *src_len = read;
    *dst_len = written;
}

/// `usize` as `size_t` and `u8` as `char` in .h.
/// `src_len` and `dsc_len` are inout params.
#[no_mangle]
pub unsafe extern "C" fn icu_to_lowercase_latin1(
    src: *const u8,
    src_len: *mut usize,
    dst: *mut u8,
    dst_len: *mut usize,
) {
    let src_slice = ::std::slice::from_raw_parts(src, *src_len);
    let dst_slice = ::std::slice::from_raw_parts_mut(dst, *dst_len);
    let (read, written) = to_lowercase_latin1(src_slice, dst_slice);
    *src_len = read;
    *dst_len = written;
}

/// `usize` as `size_t` and `u8` as `char8_t` in .h.
/// `len` is an inout param and `capacity is an out params; after return
/// `*len` is the logical (filled) length of the returned buffer and
/// `*capacity` is the allocated capacity of the returned buffer. The
/// returned buffer is not U+0000-terminated and the caller takes its
/// ownership and is responsible for freeing it. Use `icu_free_utf8`
/// to free it. Alternatively, if ICUX4 has been built with the same
/// allocator as the caller code, it is OK to use the corresponding C
/// freeing function. For example, if ICU4X has been built with
/// `alloc = system`, it is OK to use C `free()`.
#[no_mangle]
pub unsafe extern "C" fn icu_inestimable_utf8(
    src: *const u8,
    len: *mut usize,
    capacity: *mut usize,
) -> *mut u8 {
    let src_slice = ::std::slice::from_raw_parts(src, *len);
    let string = inestimable_utf8(src_slice);
    let (ptr, length, cap) = vec_into_raw_parts(string.into_bytes());
    *len = length;
    *capacity = cap;
    ptr
}

/// `usize` as `size_t` and `u8` as `char8_t` in .h.
/// `len` is an inout param and `capacity is an out params; after return
/// `*len` is the logical (filled) length of the returned buffer and
/// `*capacity` is the allocated capacity of the returned buffer. The
/// returned buffer is not U+0000-terminated and the caller takes its
/// ownership and is responsible for freeing it. Use `icu_free_utf8`
/// to free it. Alternatively, if ICUX4 has been built with the same
/// allocator as the caller code, it is OK to use the corresponding C
/// freeing function. For example, if ICU4X has been built with
/// `alloc = system`, it is OK to use C `free()`.
#[no_mangle]
pub unsafe extern "C" fn icu_inestimable_utf8_unsafe(
    src: *const u8,
    len: *mut usize,
    capacity: *mut usize,
) -> *mut u8 {
    let src_slice = ::std::slice::from_raw_parts(src, *len);
    let str_slice = ::std::str::from_utf8_unchecked(src_slice);
    let string = inestimable(str_slice);
    let (ptr, length, cap) = vec_into_raw_parts(string.into_bytes());
    *len = length;
    *capacity = cap;
    ptr
}

/// `usize` as `size_t` and `u16` as `char16_t` in .h.
/// `len` is an inout param and `capacity is an out params; after return
/// `*len` is the logical (filled) length of the returned buffer and
/// `*capacity` is the allocated capacity of the returned buffer. The
/// returned buffer is not U+0000-terminated and the caller takes its
/// ownership and is responsible for freeing it. Use `icu_free_utf16`
/// to free it. Alternatively, if ICUX4 has been built with the same
/// allocator as the caller code, it is OK to use the corresponding C
/// freeing function. For example, if ICU4X has been built with
/// `alloc = system`, it is OK to use C `free()`.
#[no_mangle]
pub unsafe extern "C" fn icu_inestimable_utf16(
    src: *const u16,
    len: *mut usize,
    capacity: *mut usize,
) -> *mut u16 {
    let src_slice = ::std::slice::from_raw_parts(src, *len);
    let vec = inestimable_utf16(src_slice);
    let (ptr, length, cap) = vec_into_raw_parts(vec);
    *len = length;
    *capacity = cap;
    ptr
}

/// Frees a UTF-8 buffer previously obtained from ICU4X. `buf` must
/// be a pointer obtained from ICU4X and `capacity` must be the capacity
/// that was originally obtained alongside `buf`.
#[no_mangle]
pub unsafe extern "C" fn icu_free_utf8(buf: *mut u8, capacity: usize) {
    let _ = Vec::from_raw_parts(buf, 0, capacity);
}

/// Frees a UTF-16 buffer previously obtained from ICU4X. `buf` must
/// be a pointer obtained from ICU4X and `capacity` must be the capacity
/// that was originally obtained alongside `buf`.
#[no_mangle]
pub unsafe extern "C" fn icu_free_utf16(buf: *mut u16, capacity: usize) {
    let _ = Vec::from_raw_parts(buf, 0, capacity);
}

// End FFI boilerplate
```

C++ wrapper for a subset of the above FFI using C++20 standard library types.


```c++
extern "C" {
    void icu_to_lowercase_utf8(const char8_t* src, size_t* src_len, char8_t* dst, size_t* dst_len);
    size_t icu_to_lowercase_utf8_max(size_t len);
    char8_t* icu_inestimable_utf8(const char8_t* src, size_t* len, size_t* capacity);
    void icu_free_utf8(char8_t* buf, size_t capacity);
}

namespace icu {
    std::tuple<size_t, size_t> to_lowercase_utf8(std::u8string_view src, std::span<char8_t> dst) {
        // This is the same boilerplate for all operations
        size_t src_len = src.size();
        size_t dst_len = dst.size();
        icu_to_lowercase_str_utf8(src.data(), &src_len, dst.data(), &dst_len);
        return {src_len, dsc_len};
    }

    std::optional<size_t> to_lowercase_utf8_max(size_t len) {
        size_t max_len = icu_to_lowercase_utf8_max(len);
        if (max_len == SIZE_MAX) {
            return std::nullopt;
        }
        return max_len;   
    }

    std::u8string to_lowercase(std::u8string_view src) {
        // This is the same boilerplate for all operations

        // First assume the same length
        std::u8string string(src.size(), char8_t(0));
        auto [read_first, written_first] = to_lowercase_utf8(src, string);
        size_t total_written = written_first;
        size_t input_left = src.size() - read_first;
        if (input_left) {
            string.append(to_lowercase_utf8_max(input_left).value(), char8_t(0));
            std::span<char8_t> string_as_span{string};
            src.remove_prefix(read_first);
            auto [read_second, written_second] = to_lowercase_utf8(src, string_as_span.subspan(written_first));
            assert(read_first + read_second == src.size());
            total_written += written_second;
        }
        string.resize(total_written);
        string.shrink_to_fit();
        return string;
    }

    std::u8string inestimable(std::u8string_view src) {
        size_t len = src.size();
        size_t capacity;
        char8_t* buf = icu_inestimable_utf8(src.data(), &len, &capacity);
        std::u8string string(buf, len);
        icu_free_utf8(buf, capacity);
        return string;
    }

}; // namespace icu

```
