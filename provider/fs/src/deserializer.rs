// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use crate::manifest::SyntaxOption;
use icu_provider::prelude::*;
use icu_provider::serde::*;
use icu_provider::yoke::trait_hack::YokeTraitHack;
use icu_provider::yoke::Yokeable;
use serde::Deserialize;
use std::path::Path;
use std::rc::Rc;
use thiserror::Error;

/// An Error type specifically for the [`Deserializer`](serde::Deserializer) that doesn't carry filenames
#[derive(Error, Debug)]
pub enum Error {
    #[error(transparent)]
    Json(#[from] serde_json::error::Error),
    #[cfg(feature = "bincode")]
    #[error(transparent)]
    Bincode(#[from] bincode::Error),
    #[error(transparent)]
    DataProvider(#[from] DataError),
    #[allow(dead_code)]
    #[error("Unknown syntax: {0:?}")]
    UnknownSyntax(SyntaxOption),
}

impl Error {
    pub fn into_resource_error<P: AsRef<Path>>(self, path: P) -> DataError {
        use crate::error::Error as CrateError;
        let crate_error = match self {
            Self::Json(err) => {
                CrateError::Deserializer(Box::new(err), Some(path.as_ref().to_path_buf()))
            }
            #[cfg(feature = "bincode")]
            Self::Bincode(err) => {
                CrateError::Deserializer(Box::new(err), Some(path.as_ref().to_path_buf()))
            }
            Self::DataProvider(err) => {
                CrateError::Deserializer(Box::new(err), Some(path.as_ref().to_path_buf()))
            }
            Self::UnknownSyntax(v) => CrateError::UnknownSyntax(v),
        };
        DataError::Resource(Box::new(crate_error))
    }
}

/// Get a JSON zero-copy deserializer. Implemeted as a macro because the return type is complex/private.
macro_rules! get_json_deserializer_zc {
    ($bytes:tt) => {
        serde_json::Deserializer::from_slice($bytes)
    };
}

/// Get a Bincode zero-copy Deserializer. Implemeted as a macro because the return type is complex/private.
#[cfg(feature = "bincode")]
macro_rules! get_bincode_deserializer_zc {
    ($bytes:tt) => {{
        use bincode::Options;
        let options = bincode::DefaultOptions::new()
            .with_fixint_encoding()
            .allow_trailing_bytes();
        bincode::de::Deserializer::from_slice($bytes, options)
    }};
}

/// Deserialize into a generic type ([`DataProvider`]). Covers all supported data formats.
#[allow(clippy::type_complexity)]
pub fn deserialize_zero_copy<'s, M>(
    syntax_option: &SyntaxOption,
) -> for<'de> fn(bytes: &'de [u8]) -> Result<<M::Yokeable as Yokeable<'de>>::Output, Error>
where
    M: DataMarker<'s>,
    // Actual bound:
    //     for<'de> <M::Yokeable as Yokeable<'de>>::Output: serde::de::Deserialize<'de>,
    // Necessary workaround bound (see `yoke::trait_hack` docs):
    for<'de> YokeTraitHack<<M::Yokeable as Yokeable<'de>>::Output>: serde::de::Deserialize<'de>,
{
    match syntax_option {
        SyntaxOption::Json => |bytes| {
            let mut d = get_json_deserializer_zc!(bytes);
            let data = YokeTraitHack::<<M::Yokeable as Yokeable>::Output>::deserialize(&mut d)?;
            Ok(data.0)
        },
        #[cfg(feature = "bincode")]
        SyntaxOption::Bincode => |bytes| {
            let mut d = get_bincode_deserializer_zc!(bytes);
            let data = YokeTraitHack::<<M::Yokeable as Yokeable>::Output>::deserialize(&mut d)?;
            Ok(data.0)
        },
        #[cfg(not(feature = "bincode"))]
        SyntaxOption::Bincode => |_| Err(Error::UnknownSyntax(SyntaxOption::Bincode)),
    }
}

/// Deserialize into a receiver used by [`SerdeDeDataProvider`](icu_provider::serde::SerdeDeDataProvider).
/// Covers all supported data formats.
pub fn deserialize_into_receiver(
    rc_buffer: Rc<[u8]>,
    syntax_option: &SyntaxOption,
    receiver: &mut dyn SerdeDeDataReceiver,
) -> Result<(), Error> {
    match syntax_option {
        SyntaxOption::Json => {
            receiver.receive_rc_buffer(rc_buffer, |bytes, f2| {
                let mut d = get_json_deserializer_zc!(bytes);
                f2(&mut <dyn erased_serde::Deserializer>::erase(&mut d))
            })?;
            Ok(())
        }
        #[cfg(feature = "bincode")]
        SyntaxOption::Bincode => {
            receiver.receive_rc_buffer(rc_buffer, |bytes, f2| {
                let mut d = get_bincode_deserializer_zc!(bytes);
                f2(&mut <dyn erased_serde::Deserializer>::erase(&mut d))
            })?;
            Ok(())
        }
        #[cfg(not(feature = "bincode"))]
        SyntaxOption::Bincode => Err(Error::UnknownSyntax(SyntaxOption::Bincode)),
    }
}
