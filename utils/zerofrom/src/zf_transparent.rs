// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

#[cfg(feature = "alloc")]
use alloc::{boxed::Box, rc::Rc};


/// Internal function: casts a box of `Inner` to a box of `Outer`, assuming
/// `Outer` is transparent over `Inner`.
///
/// # Safety
///
/// `Outer` MUST be `repr(transparent)` and MUST have one non-zero-sized field,
/// which MUST be of type `Inner`.
///
/// Suggestion: explicitly setting the generic parameters to two types satisfying
/// this invariant makes the fn safe to call.
#[cfg(feature = "alloc")]
#[inline(always)]
pub unsafe fn cast_transparent_box<Inner, Outer>(inner: Box<Inner>) -> Box<Outer> {
    // Safety:
    //
    // - Both boxes have the same allocator (the global allocator).
    // - Since `Outer` is transparent over `Inner`, they have the same layout.
    // - `Box::into_raw` returns a properly aligned, non-null `*mut Inner`.
    Box::from_raw(Box::into_raw(inner) as *mut Outer)
}

/// Internal function: casts a Rc of `Inner` to a Rc of `Outer`, assuming
/// `Outer` is transparent over `Inner`.
///
/// # Safety
///
/// `Outer` MUST be `repr(transparent)` and MUST have one non-zero-sized field,
/// which MUST be of type `Inner`.
///
/// Suggestion: explicitly setting the generic parameters to two types satisfying
/// this invariant makes the fn safe to call.
#[cfg(feature = "alloc")]
#[inline(always)]
pub unsafe fn cast_transparent_rc<Inner, Outer>(inner: Rc<Inner>) -> Rc<Outer> {
    // Safety:
    //
    // - Both boxes have the same allocator (the global allocator).
    // - Since `Outer` is transparent over `Inner`, they have the same layout.
    // - `Rc::into_raw` returns a properly aligned, non-null `*mut Inner`.
    Rc::from_raw(Rc::into_raw(inner) as *mut Outer)
}

/// Implements functions that cast a reference of an inner type
/// to a reference of an outer type that is `repr(transparent)`
/// over the inner type.
///
/// These functions can be used to implement `ZeroFrom`.
///
/// # Examples
///
/// ```
/// use crate::zerofrom::ZeroFrom;
///
/// zerofrom::transparent!(
///     #[repr(transparent)]
///     pub struct StrWrap(str);
///     impl StrWrap {
///         fn zero_from_transparent_ref(&str) -> &Self;
///     }
/// );
///
/// impl<'zf> ZeroFrom<'zf, str> for &'zf StrWrap {
///     fn zero_from(other: &'zf str) -> &'zf StrWrap {
///         StrWrap::zero_from_transparent_ref(other)
///     }
/// }
///
/// let s = "hello";
/// let wrap = <&StrWrap>::zero_from(s);
///
/// assert_eq!(&wrap.0, "hello");
/// ```
#[macro_export]
macro_rules! transparent {
	(
		#[repr(transparent)]
		$(#[$meta:meta])*
		$vis:vis struct $outer:ident($inner:ty);
		$(
			$(#[$meta_impl:meta])*
			impl $outer_impl:ident {
				$(
					fn zero_from_transparent_ref(&$inner_ref:ty) -> &Self;
				)?
				$(
					fn zero_from_transparent_slice(&[$inner_slice:ty]) -> &[Self];
				)?
				$(
					fn zero_from_transparent_box(Box<$inner_box:ty>) -> Box<Self>;
				)?
				$(
					fn zero_from_transparent_rc(Rc<$inner_rc:ty>) -> Rc<Self>;
				)?
			}
		)+
	) => {
		#[repr(transparent)]
		$(#[$meta])*
		$vis struct $outer($inner);
		$(
			impl $outer {
				$(
					fn zero_from_transparent_ref(inner: &$inner_ref) -> &$outer_impl {
						// Safety: $outer is repr(transparent) over $inner.
						unsafe { core::mem::transmute::<&$inner, &$outer>(inner) }
					}
				)?
				$(
					fn zero_from_transparent_slice(inner: &[$inner_slice]) -> &[$outer_impl] {
						// Safety: $outer is repr(transparent) over $inner.
						unsafe { core::mem::transmute::<&[$inner], &[$outer]>(inner) }
					}
				)?
				$(
					fn zero_from_transparent_box(inner: $crate::internal::Box<$inner_box>) -> $crate::internal::Box<$outer_impl> {
						// Safety: $outer is repr(transparent) over $inner.
						unsafe { $crate::internal::cast_transparent_box::<$inner, $outer>(inner) }
					}
				)?
				$(
					fn zero_from_transparent_rc(inner: $crate::internal::Rc<$inner_rc>) -> $crate::internal::Rc<$outer_impl> {
						// Safety: $outer is repr(transparent) over $inner.
						unsafe { $crate::internal::cast_transparent_rc::<$inner, $outer>(inner) }
					}
				)?
			}
		)+
	};
}

/// Additional tests for failure modes.
///
/// ```compile_fail,E0308
/// zerofrom::transparent! {
///     #[repr(transparent)]
///     pub struct Foo(String);
///     impl Foo {
///         // should be &String
///         fn zero_from_transparent_ref(&Foo) -> &Self;
///     }
/// };
/// ```
///
/// ```compile_fail,E0308
/// zerofrom::transparent! {
///     #[repr(transparent)]
///     pub struct Foo(String);
///     impl Foo {
///         // should be &[String]
///         fn zero_from_transparent_slice(&[Foo]) -> &[Self];
///     }
/// };
/// ```
///
/// ```compile_fail,E0308
/// zerofrom::transparent! {
///     #[repr(transparent)]
///     pub struct Foo(String);
///     // should be Foo
///     impl String {
///         fn zero_from_transparent_slice(&[String]) -> &[Self];
///     }
/// };
/// ```
///
/// ```compile_fail,E0277
/// // Can't cast a slice of DSTs
/// zerofrom::transparent! {
///     #[repr(transparent)]
///     pub struct DST(str);
///     impl DST {
///         fn zero_from_transparent_slice(&[str]) -> &[Self];
///     }
/// };
/// ```
///
/// TODO: Rc
mod _tests {}
