// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

#[cfg(feature = "alloc")]
use alloc::boxed::Box;

/// Internal function: casts a box of `Inner` to a box of `Outer`, assuming
/// `Outer` is transparent over `Inner`.
///
/// # Safety
///
/// `Outer` is `repr(transparent)` and has one non-zero-sized field
/// of type `Inner`.
#[cfg(feature = "alloc")]
pub unsafe fn cast_transparent_box<Outer, Inner>(inner: Box<Inner>) -> Box<Outer> {
    // Safety:
    //
    // - Both boxes have the same allocator (the global allocator).
    // - Since `Outer` is transparent over `Inner`, they have the same layout.
    // - `Box::into_raw` returns a properly aligned, non-null `*mut Inner`.
    Box::from_raw(Box::into_raw(inner) as *mut Outer)
}

/// Implements [`ZeroFrom`](crate::ZeroFrom) on a transparent type
/// from a reference to the inner type.
///
/// Also supports creating concrete functions.
///
/// # Examples
///
/// ```
/// use crate::zerofrom::ZeroFrom;
///
/// zerofrom::transparent!(
///     #[repr(transparent)]
///     pub struct StrWrap(str);
///     impl ZeroFrom<&str> for &StrWrap;
/// );
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
			impl ZeroFrom<&$inner_zf:ty> for &$outer_zf:ident;
		)?
		$(impl {
			$(
				@ref
				$(#[$meta_ref:meta])*
				$vis_ref:vis fn $fn_ref:ident(&$inner_ref:ty) -> &Self;
			)?
			$(
				@slice
				$(#[$meta_slice:meta])*
				$vis_slice:vis fn $fn_slice:ident(&[$inner_slice:ty]) -> &[Self];
			)?
			$(
				@box
				$(#[$meta_box:meta])*
				$vis_box:vis fn $fn_box:ident(Box<$inner_box:ty>) -> Box<Self>;
			)?
			$(
				@rc
				$(#[$meta_rc:meta])*
				$vis_rc:vis fn $fn_rc:ident(Rc<$inner_rc:ty>) -> Rc<Self>;
			)?
		})?
	) => {
		#[repr(transparent)]
		$(#[$meta])*
		$vis struct $outer($inner);
		$(
			impl<'zf> $crate::ZeroFrom<'zf, $inner_zf> for &'zf $outer {
				fn zero_from(inner: &'zf $inner) -> Self {
					unsafe { core::mem::transmute(inner) }
				}
			}
		)?
		$(impl $outer {
			$(
				$(#[$meta_ref])*
				$vis_ref fn $fn_ref(inner: &$inner_ref) -> &Self {
					unsafe { core::mem::transmute(inner) }
				}
			)?
			$(
				$(#[$meta_slice])*
				$vis_slice fn $fn_slice(inner: &[$inner_slice]) -> &[Self] {
					unsafe { core::mem::transmute(inner) }
				}
			)?
			$(
				$(#[$meta_box])*
				$vis_box fn $fn_box(inner: $crate::internal::Box<$inner_box>) -> $crate::internal::Box<Self> {
					// Safety: $outer is repr(transparent) over $inner.
					// TODO: Enforce that $inner is the same as $inner_box
					unsafe { $crate::internal::cast_transparent_box(inner) }
				}
			)?
			$(
				$(#[$meta_rc])*
				$vis_rc fn $fn_rc(inner: $crate::internal::Rc<$inner_rc>) -> $crate::internal::Rc<Self> {
					unsafe { core::mem::transmute(inner) }
				}
			)?
		})?
	};
}
