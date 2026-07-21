// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

//! Types for loading a single display name at a time.
//!
//! This submodule is useful for applications that only need to display one or
//! two specific names, such as the name of the current region.
//!
//! ### Status
//!
//! Currently, this module has limited support. It supports regions, scripts,
//! and variants, but support for languages and locales is currently missing.
//! More features are on their way.
//!
//! If you have any feedback, please let us know at
//! <https://github.com/unicode-org/icu4x/issues/7825>.
//!
//! See [the parent module](mod@super) for a comparison of single and multi.

mod language;
mod region;
mod script;
mod variant;

// Re-export from submodules
pub use language::{
    LanguageIdentifierDisplayName, LanguageIdentifierDisplayNameOwned,
    LanguageIdentifierNameFallbackError,
};
pub use region::{RegionDisplayName, RegionDisplayNameOwned};
pub use script::{ScriptDisplayName, ScriptDisplayNameOwned};
pub use variant::{VariantDisplayName, VariantDisplayNameOwned};

use icu_provider::prelude::*;

fn load_one<M0, M1, P>(
    provider: &P,
    locale: &DataLocale,
    attrs: &DataMarkerAttributes,
) -> Result<Option<DataPayload<M1>>, DataError>
where
    M0: DataMarker,
    M1: DynamicDataMarker<DataStruct = M0::DataStruct>,
    P: DataProvider<M0> + ?Sized,
{
    let id = DataIdentifierBorrowed::for_marker_attributes_and_locale(attrs, locale);
    let mut metadata = DataRequestMetadata::default();
    metadata.silent = true;
    let response = provider
        .load(DataRequest { id, metadata })
        .allow_identifier_not_found()?;
    Ok(response.map(|r| r.payload.cast()))
}

macro_rules! impl_writeable_for_single_display_name_borrowed {
    ($borrowed:ident) => {
        impl<'a> writeable::Writeable for $borrowed<'a> {
            #[inline]
            fn write_to<W: core::fmt::Write + ?Sized>(&self, sink: &mut W) -> core::fmt::Result {
                sink.write_str(self.value)
            }

            #[inline]
            fn writeable_length_hint(&self) -> writeable::LengthHint {
                writeable::LengthHint::exact(self.value.len())
            }

            #[inline]
            fn writeable_borrow(&self) -> Option<&str> {
                Some(self.value)
            }
        }

        writeable::impl_display_with_writeable!($borrowed<'_>);
    };
}

macro_rules! impl_writeable_for_single_display_name_owned {
    ($owned:ident) => {
        impl writeable::Writeable for $owned {
            #[inline]
            fn write_to<W: core::fmt::Write + ?Sized>(&self, sink: &mut W) -> core::fmt::Result {
                self.as_borrowed().write_to(sink)
            }

            #[inline]
            fn write_to_parts<S: writeable::PartsWrite + ?Sized>(
                &self,
                sink: &mut S,
            ) -> core::fmt::Result {
                self.as_borrowed().write_to_parts(sink)
            }

            #[inline]
            fn writeable_length_hint(&self) -> writeable::LengthHint {
                self.as_borrowed().writeable_length_hint()
            }

            #[inline]
            fn writeable_borrow(&self) -> Option<&str> {
                Some(self.payload.get())
            }
        }

        writeable::impl_display_with_writeable!($owned);
    };
}

pub(crate) use impl_writeable_for_single_display_name_borrowed;
pub(crate) use impl_writeable_for_single_display_name_owned;
