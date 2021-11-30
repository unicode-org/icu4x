// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use super::{BufferFormat, Error};
use crate::prelude::*;
use serde::de::Deserialize;
use yoke::trait_hack::YokeTraitHack;
use yoke::Yokeable;

/// Converts a buffer into a concrete type by deserializing from a supported buffer format.
#[allow(clippy::type_complexity)]
pub fn deserialize<M>(
    buffer_format: BufferFormat,
) -> for<'de> fn(
    bytes: &'de [u8],
    buffer_format: BufferFormat,
) -> Result<<M::Yokeable as Yokeable<'de>>::Output, Error>
where
    M: DataMarker,
    // Actual bound:
    //     for<'de> <M::Yokeable as Yokeable<'de>>::Output: Deserialize<'de>,
    // Necessary workaround bound (see `yoke::trait_hack` docs):
    for<'de> YokeTraitHack<<M::Yokeable as Yokeable<'de>>::Output>: Deserialize<'de>,
{
    match buffer_format {
        #[cfg(feature = "provider_json")]
        BufferFormat::Json => |bytes, _| {
            let mut d = serde_json::Deserializer::from_slice(bytes);
            let data = YokeTraitHack::<<M::Yokeable as Yokeable>::Output>::deserialize(&mut d)?;
            Ok(data.0)
        },

        #[cfg(feature = "provider_bincode1")]
        BufferFormat::Bincode1 => |bytes, _| {
            use bincode::Options;
            let options = bincode::DefaultOptions::new()
                .with_fixint_encoding()
                .allow_trailing_bytes();
            let mut d = bincode::de::Deserializer::from_slice(bytes, options);
            let data = YokeTraitHack::<<M::Yokeable as Yokeable>::Output>::deserialize(&mut d)?;
            Ok(data.0)
        },

        #[cfg(feature = "provider_postcard07")]
        BufferFormat::Postcard07 => |bytes, _| {
            let mut d = postcard::Deserializer::from_bytes(bytes);
            let data = YokeTraitHack::<<M::Yokeable as Yokeable>::Output>::deserialize(&mut d)?;
            Ok(data.0)
        },

        // Allowed for cases in which all features are enabled
        #[allow(unreachable_patterns)]
        _ => |_, buffer_format| Err(Error::UnavailableFormat(buffer_format)),
    }
}
