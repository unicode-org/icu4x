// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use super::options::*;

mod validated_options;
pub use validated_options::{DurationFormatterOptionsError, ValidatedDurationFormatterOptions};

/// A formatter for [`Duration`](crate::duration::Duration)s.
#[derive(Clone)]
pub struct DurationFormatter {
    #[allow(dead_code)]
    /// Options for configuring the formatter.
    pub(crate) options: ValidatedDurationFormatterOptions,
}

impl DurationFormatter {
    /// Create a new [`DurationFormatter`] with the given options.
    pub fn new(options: DurationFormatterOptions) -> Result<Self, DurationFormatterOptionsError> {
        Ok(DurationFormatter {
            options: ValidatedDurationFormatterOptions::validate(options)?,
        })
    }
}
