// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

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
		$vis:vis struct $name:ident($type:ty);
		$(
			impl ZeroFrom<&$type_zf:ty> for &$name_zf:ident;
		)?
		$(impl {
			$(
				@ref
				$(#[$meta_ref:meta])*
				$vis_ref:vis fn $fn_ref:ident(&$type_ref:ty) -> &Self;
			)?
			$(
				@slice
				$(#[$meta_slice:meta])*
				$vis_slice:vis fn $fn_slice:ident(&[$type_slice:ty]) -> &[Self];
			)?
			$(
				@box
				$(#[$meta_box:meta])*
				$vis_box:vis fn $fn_box:ident(Box<$type_box:ty>) -> Box<Self>;
			)?
			$(
				@rc
				$(#[$meta_rc:meta])*
				$vis_rc:vis fn $fn_rc:ident(Rc<$type_rc:ty>) -> Rc<Self>;
			)?
		})?
	) => {
		#[repr(transparent)]
		$(#[$meta])*
		$vis struct $name($type);
		$(
			impl<'zf> $crate::ZeroFrom<'zf, $type_zf> for &'zf $name {
				fn zero_from(inner: &'zf $type) -> Self {
					unsafe { core::mem::transmute(inner) }
				}
			}
		)?
		$(impl $name {
			$(
				$(#[$meta_ref])*
				$vis_ref fn $fn_ref(inner: &$type_ref) -> &Self {
					unsafe { core::mem::transmute(inner) }
				}
			)?
			$(
				$(#[$meta_slice])*
				$vis_slice fn $fn_slice(inner: &[$type_slice]) -> &[Self] {
					unsafe { core::mem::transmute(inner) }
				}
			)?
			$(
				$(#[$meta_box])*
				$vis_box fn $fn_box(inner: $crate::internal::Box<$type_box>) -> $crate::internal::Box<Self> {
					unsafe { core::mem::transmute(inner) }
				}
			)?
			$(
				$(#[$meta_rc])*
				$vis_rc fn $fn_rc(inner: $crate::internal::Rc<$type_rc>) -> $crate::internal::Rc<Self> {
					unsafe { core::mem::transmute(inner) }
				}
			)?
		})?
	};
}
