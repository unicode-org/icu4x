// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use icu_locid::Locale as ICULocale;
use icu_plurals::{PluralCategory, PluralOperands, PluralRuleType, PluralRules};

use crate::provider::ICU4XDataProvider;

use std::ptr;

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

#[no_mangle]
/// FFI version of [`PluralRules::select()`]. See its docs for more details.
pub extern "C" fn icu4x_plural_rules_select(
    pr: &ICU4XPluralRules,
    op: &ICU4XPluralOperands,
) -> ICU4XPluralCategory {
    pr.select(*op).into()
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
#[derive(Copy, Clone)]
/// FFI version of [`PluralOperands`]. See its docs for more details.
pub struct ICU4XPluralOperands {
    pub i: u64,
    pub v: usize,
    pub w: usize,
    pub f: u64,
    pub t: u64,
    pub c: usize,
}

#[repr(C)]
/// FFI version of [`PluralRuleType`]. See its docs for more details.
pub enum ICU4XPluralRuleType {
    Cardinal,
    Ordinal,
}

#[repr(C)]
/// FFI version of [`PluralCategory`]. See its docs for more details.
pub enum ICU4XPluralCategory {
    Zero,
    One,
    Two,
    Few,
    Many,
    Other,
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

impl From<ICU4XPluralRuleType> for PluralRuleType {
    fn from(other: ICU4XPluralRuleType) -> Self {
        match other {
            ICU4XPluralRuleType::Cardinal => Self::Cardinal,
            ICU4XPluralRuleType::Ordinal => Self::Ordinal,
        }
    }
}
impl From<PluralCategory> for ICU4XPluralCategory {
    fn from(other: PluralCategory) -> Self {
        match other {
            PluralCategory::Zero => Self::Zero,
            PluralCategory::One => Self::One,
            PluralCategory::Two => Self::Two,
            PluralCategory::Few => Self::Few,
            PluralCategory::Many => Self::Many,
            PluralCategory::Other => Self::Other,
        }
    }
}
