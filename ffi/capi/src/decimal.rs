// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use crate::custom_writeable::ICU4XWriteable;
use crate::fixed_decimal::ICU4XFixedDecimal;
use crate::locale::ICU4XLocale;
use crate::provider::ICU4XDataProvider;
use icu_decimal::options::{FixedDecimalFormatOptions, GroupingStrategy, SignDisplay};
use icu_decimal::FixedDecimalFormat;
use std::ptr;

/// Opaque type for use behind a pointer, is [`FixedDecimalFormat`]
///
/// Can be obtained via [`icu4x_fixed_decimal_format_create()`] and destroyed via [`icu4x_fixed_decimal_format_destroy()`]
pub type ICU4XFixedDecimalFormat<'d> = FixedDecimalFormat<'d, 'static>;

#[repr(C)]
/// This is the result returned by [`icu4x_fixed_decimal_format_create()`]
pub struct ICU4XCreateFixedDecimalFormatResult<'d> {
    /// Will be null if `success` is [`false`]
    pub fdf: *mut ICU4XFixedDecimalFormat<'d>,
    /// Currently just a boolean, but we might add a proper error enum as necessary
    pub success: bool,
}

#[no_mangle]
/// FFI version of [`FixedDecimalFormat::try_new()`]. See its docs for more details.
///
/// # Safety
/// - `locale` should be constructed via [`icu4x_locale_create()`](crate::locale::icu4x_locale_create)
/// - `provider` should be constructed via one of the functions in [`crate::locale`](crate::locale)
/// - Only access `fdf` in the result if `success` is [`true`].
pub extern "C" fn icu4x_fixed_decimal_format_create<'d>(
    locale: &ICU4XLocale,
    provider: &'d ICU4XDataProvider,
    options: ICU4XFixedDecimalFormatOptions,
) -> ICU4XCreateFixedDecimalFormatResult<'d> {
    // cheap as long as there are no variants
    let langid = locale.as_ref().clone();
    let provider = provider.as_dyn_ref();
    match FixedDecimalFormat::try_new(langid, provider, options.into()) {
        Ok(fdf) => {
            let fdf = Box::new(fdf);
            ICU4XCreateFixedDecimalFormatResult {
                fdf: Box::into_raw(fdf),
                success: true,
            }
        }
        Err(_) => ICU4XCreateFixedDecimalFormatResult {
            fdf: ptr::null_mut(),
            success: false,
        },
    }
}

#[no_mangle]
/// FFI version of [`FixedDecimalFormat::format()`]. See its docs for more details.
///
/// Returns `false` when there were errors writing to `write`
pub extern "C" fn icu4x_fixed_decimal_format_write(
    fdf: &ICU4XFixedDecimalFormat<'_>,
    value: &ICU4XFixedDecimal,
    write: &mut ICU4XWriteable,
) -> bool {
    use writeable::Writeable;

    let formatted = fdf.format(value);
    let result = formatted.write_to(write).is_ok();
    write.flush();
    result
}

#[no_mangle]
/// Destructor for [`ICU4XFixedDecimalFormat`]
///
/// # Safety
/// `fdf` must be a pointer to a valid [`ICU4XFixedDecimalFormat`] constructed by
/// [`icu4x_fixed_decimal_format_create()`].
pub unsafe extern "C" fn icu4x_fixed_decimal_format_destroy(fdf: *mut ICU4XFixedDecimalFormat<'_>) {
    let _ = Box::from_raw(fdf);
}

#[repr(C)]
pub struct ICU4XFixedDecimalFormatOptions {
    grouping_strategy: ICU4XGroupingStrategy,
    sign_display: ICU4XSignDisplay,
}

c_enum! {
    /// FFI version of [`GroupingStrategy`]. See its docs for more details.
    pub c_enum ICU4XGroupingStrategy is #[non_exhaustive] GroupingStrategy {
        Auto,
        Never,
        Always,
        Min2,
    }
}

c_enum! {
    /// FFI version of [`SignDisplay`]. See its docs for more details.
    pub c_enum ICU4XSignDisplay is #[non_exhaustive] SignDisplay {
        Auto,
        Never,
        Always,
        ExceptZero,
        Negative,
    }
}

impl From<ICU4XFixedDecimalFormatOptions> for FixedDecimalFormatOptions {
    fn from(c: ICU4XFixedDecimalFormatOptions) -> Self {
        Self {
            grouping_strategy: c.grouping_strategy.into(),
            sign_display: c.sign_display.into(),
        }
    }
}
