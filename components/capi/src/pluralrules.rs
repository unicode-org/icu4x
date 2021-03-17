// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use icu_locid::Locale as ICULocale;
use icu_plurals::{PluralCategory, PluralOperands, PluralRuleType};


use crate::provider::ICU4XDataProvider;

use std::ptr;

// opaque type
pub use icu_plurals::PluralRules as ICU4XPluralRules;

#[repr(C)]
pub struct ICU4XCreatePluralRulesResult {
    rules: *mut ICU4XPluralRules,
    success: bool,
}

#[no_mangle]
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
pub extern "C" fn icu4x_plural_rules_select(pr: &ICU4XPluralRules, op: &ICU4XPluralOperands) -> ICU4XPluralCategory {
    pr.select(*op).into()
}

#[no_mangle]
pub unsafe extern "C" fn icu4x_plural_rules_destroy(pr: *mut ICU4XPluralRules) {
    let _ = Box::from_raw(pr);
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ICU4XPluralOperands {
    pub i: u64,
    pub v: usize,
    pub w: usize,
    pub f: u64,
    pub t: u64,
    pub c: usize,
}

#[repr(C)]
pub enum ICU4XPluralRuleType {
    Cardinal,
    Ordinal,
}

#[repr(C)]
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
    fn from(other: ICU4XPluralOperands) -> PluralOperands {
        PluralOperands {
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
            ICU4XPluralRuleType::Cardinal => PluralRuleType::Cardinal,
            ICU4XPluralRuleType::Ordinal => PluralRuleType::Ordinal,
        }
    }
}
impl From<PluralCategory> for ICU4XPluralCategory {
    fn from(other: PluralCategory) -> Self {
        match other {
            PluralCategory::Zero => ICU4XPluralCategory::Zero,
            PluralCategory::One => ICU4XPluralCategory::One,
            PluralCategory::Two => ICU4XPluralCategory::Two,
            PluralCategory::Few => ICU4XPluralCategory::Few,
            PluralCategory::Many => ICU4XPluralCategory::Many,
            PluralCategory::Other => ICU4XPluralCategory::Other,
        }
    }
}
