// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use icu_locid::Locale as ICULocale;
use icu_plurals::{PluralCategory, PluralOperands, PluralRuleType, PluralRules};

use crate::provider::ICU4XDataProvider;
use std::ptr;
use std::slice;
use std::str::{self, FromStr};

/// Opaque type for use behind a pointer, is [`PluralRules`]
///
/// Can be obtained via [`icu4x_plural_rules_create()`] and destroyed via [`icu4x_plural_rules_destroy()`]
pub type ICU4XPluralRules = PluralRules;

#[repr(C)]
/// This is the result returned by [`icu4x_plural_rules_create()`]
pub struct ICU4XCreatePluralRulesResult {
    /// Will be null if `success` is [`false`]
    pub rules: *mut ICU4XPluralRules,
    /// Currently just a boolean, but we might add a proper error enum as necessary
    pub success: bool,
}

#[no_mangle]
/// FFI version of [`PluralRules::try_new()`]. See its docs for more details.
///
/// # Safety
/// - `locale` should be constructed via [`icu4x_locale_create()`](crate::locale::icu4x_locale_create)
/// - `provider` should be constructed via one of the functions in [`crate::locale`](crate::locale)
/// - Only access `rules` in the result if `success` is [`true`].
pub extern "C" fn icu4x_plural_rules_create(
    locale: &ICULocale,
    provider: &ICU4XDataProvider,
    ty: ICU4XPluralRuleType,
) -> ICU4XCreatePluralRulesResult {
    // cheap as long as there are no variants
    let langid = locale.as_ref().clone();
    let provider = provider.as_dyn_ref();
    match ICU4XPluralRules::try_new(langid, provider, ty.into()) {
        Ok(pr) => {
            let pr = Box::new(pr);
            ICU4XCreatePluralRulesResult {
                rules: Box::into_raw(pr),
                success: true,
            }
        }
        Err(_) => ICU4XCreatePluralRulesResult {
            rules: ptr::null_mut(),
            success: false,
        },
    }
}

#[repr(C)]
/// This is the result returned by [`icu4x_plural_operands_create()`]
pub struct ICU4XCreatePluralOperandsResult {
    /// Will default initialized if `success` is [`false`]
    pub operands: ICU4XPluralOperands,
    /// Currently just a boolean, but we might add a proper error enum as necessary
    pub success: bool,
}

#[no_mangle]
/// FFI version of [`PluralOperands::from_str()`]. See its docs for more details.
///
/// # Safety
/// `number` and `len` should point to a valid ASCII string of length `len`.
/// The string `number` should be able to parse as a floating-point value.
///
/// It does not need to be be null terminated, and `len` should not include a null
/// terminator (this will just cause the function to panic, and is not a safety requirement).
pub unsafe extern "C" fn icu4x_plural_operands_create(
    number: *const u8,
    len: usize,
) -> ICU4XCreatePluralOperandsResult {
    str::from_utf8(slice::from_raw_parts(number, len))
        .ok()
        .and_then(|s| {
            PluralOperands::from_str(s)
                .ok()
                .map(|ops| ICU4XCreatePluralOperandsResult {
                    operands: ops.into(),
                    success: true,
                })
        })
        .unwrap_or(ICU4XCreatePluralOperandsResult {
            operands: ICU4XPluralOperands::default(),
            success: false,
        })
}

#[no_mangle]
/// FFI version of [`PluralRules::select()`]. See its docs for more details.
pub extern "C" fn icu4x_plural_rules_select(
    pr: &ICU4XPluralRules,
    op: &ICU4XPluralOperands,
) -> ICU4XPluralCategory {
    pr.select(*op).into()
}

#[no_mangle]
/// FFI version of [`PluralRules::categories()`]. See its docs for more details.
pub extern "C" fn icu4x_plural_rules_categories(pr: &ICU4XPluralRules) -> ICU4XPluralCategories {
    pr.categories().fold(
        ICU4XPluralCategories::default(),
        |mut categories, category| {
            match category {
                PluralCategory::Zero => categories.zero = true,
                PluralCategory::One => categories.one = true,
                PluralCategory::Two => categories.two = true,
                PluralCategory::Few => categories.few = true,
                PluralCategory::Many => categories.many = true,
                PluralCategory::Other => categories.other = true,
            };
            categories
        },
    )
}

#[no_mangle]
/// Destructor for [`ICU4XPluralRules`]
///
/// # Safety
/// `pr` must be a pointer to a valid [`ICU4XPluralRules`] constructed by
/// [`icu4x_plural_rules_create()`].
pub unsafe extern "C" fn icu4x_plural_rules_destroy(pr: *mut ICU4XPluralRules) {
    let _ = Box::from_raw(pr);
}

#[repr(C)]
#[derive(Copy, Clone, Default)]
/// FFI version of [`PluralOperands`]. See its docs for more details.
pub struct ICU4XPluralOperands {
    pub i: u64,
    pub v: usize,
    pub w: usize,
    pub f: u64,
    pub t: u64,
    pub c: usize,
}

c_enum! {
    /// FFI version of [`PluralRuleType`]. See its docs for more details.
    pub c_enum ICU4XPluralRuleType is PluralRuleType {
        Cardinal,
        Ordinal,
    }
}

#[repr(C)]
c_enum! {
    /// FFI version of [`PluralCategory`]. See its docs for more details.
    pub c_enum ICU4XPluralCategory is PluralCategory {
        Zero,
        One,
        Two,
        Few,
        Many,
        Other,
    }
}

#[repr(C)]
#[derive(Default)]
/// FFI version of [`PluralRules::categories()`] data. See its docs for more details.
pub struct ICU4XPluralCategories {
    zero: bool,
    one: bool,
    two: bool,
    few: bool,
    many: bool,
    other: bool,
}

impl From<PluralOperands> for ICU4XPluralOperands {
    fn from(other: PluralOperands) -> Self {
        Self {
            i: other.i,
            v: other.v,
            w: other.w,
            f: other.f,
            t: other.t,
            c: other.c,
        }
    }
}

impl From<ICU4XPluralOperands> for PluralOperands {
    fn from(other: ICU4XPluralOperands) -> Self {
        Self {
            i: other.i,
            v: other.v,
            w: other.w,
            f: other.f,
            t: other.t,
            c: other.c,
        }
    }
}
