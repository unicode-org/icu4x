// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use crate::manifest::SyntaxOption;
use displaydoc::Display;
use icu_provider::prelude::*;
use icu_provider::serde::*;
use icu_provider::yoke::trait_hack::YokeTraitHack;
use icu_provider::yoke::Yokeable;
use std::path::Path;
use std::rc::Rc;

// This import is unused if no deserializer features are enabled
#[allow(unused_imports)]
use serde::Deserialize;

/// An Error type specifically for the [`Deserializer`](serde::Deserializer) that doesn't carry filenames
#[derive(Display, Debug)]
pub enum Error {
    #[cfg(feature = "provider_json")]
    #[displaydoc("{0}")]
    Json(serde_json::error::Error),
    #[cfg(feature = "provider_bincode")]
    #[displaydoc("{0}")]
    Bincode(bincode::Error),
    #[displaydoc("{0}")]
    DataProvider(DataError),
    #[allow(dead_code)]
    #[displaydoc("Unknown syntax: {0:?}")]
    UnknownSyntax(SyntaxOption),
}

impl std::error::Error for Error {}

#[cfg(feature = "provider_json")]
impl From<serde_json::error::Error> for Error {
    fn from(e: serde_json::error::Error) -> Self {
        Error::Json(e)
    }
}

#[cfg(feature = "provider_bincode")]
impl From<bincode::Error> for Error {
    fn from(e: bincode::Error) -> Self {
        Error::Bincode(e)
    }
}

impl From<DataError> for Error {
    fn from(e: DataError) -> Self {
        Error::DataProvider(e)
    }
}

impl Error {
    pub fn into_resource_error<P: AsRef<Path>>(self, path: P) -> DataError {
        use crate::error::Error as CrateError;
        let crate_error = match self {
            #[cfg(feature = "provider_json")]
            Self::Json(err) => {
                CrateError::Deserializer(format!("{}", err), Some(path.as_ref().to_path_buf()))
            }
            #[cfg(feature = "provider_bincode")]
            Self::Bincode(err) => {
                CrateError::Deserializer(format!("{}", err), Some(path.as_ref().to_path_buf()))
            }
            Self::DataProvider(err) => {
                CrateError::Deserializer(format!("{}", err), Some(path.as_ref().to_path_buf()))
            }
            Self::UnknownSyntax(v) => CrateError::UnknownSyntax(v),
        };
        DataError::new_resc_error(crate_error)
    }
}

/// Get a JSON zero-copy deserializer. Implemeted as a macro because the return type is complex/private.
#[cfg(feature = "provider_json")]
macro_rules! get_json_deserializer_zc {
    ($bytes:tt) => {
        serde_json::Deserializer::from_slice($bytes)
    };
}

/// Get a Bincode zero-copy Deserializer. Implemeted as a macro because the return type is complex/private.
#[cfg(feature = "provider_bincode")]
macro_rules! get_bincode_deserializer_zc {
    ($bytes:tt) => {{
        use bincode::Options;
        let options = bincode::DefaultOptions::new()
            .with_fixint_encoding()
            .allow_trailing_bytes();
        bincode::de::Deserializer::from_slice($bytes, options)
    }};
}

/// Returns an error if the syntax option is not supported.
pub fn check_format_supported(syntax_option: &SyntaxOption) -> Result<(), crate::error::Error> {
    #[allow(unused_imports)]
    use crate::error::Error;
    match syntax_option {
        #[cfg(feature = "provider_json")]
        SyntaxOption::Json => Ok(()),
        #[cfg(not(feature = "provider_json"))]
        SyntaxOption::Json => Err(Error::UnknownSyntax(SyntaxOption::Json)),
        #[cfg(feature = "provider_bincode")]
        SyntaxOption::Bincode => Ok(()),
        #[cfg(not(feature = "provider_bincode"))]
        SyntaxOption::Bincode => Err(Error::UnknownSyntax(SyntaxOption::Bincode)),
    }
}

/// Deserialize into a generic type ([`DataProvider`]). Covers all supported data formats.
#[allow(clippy::type_complexity)]
pub fn deserialize_zero_copy<'data, M>(
    syntax_option: &SyntaxOption,
) -> for<'de> fn(bytes: &'de [u8]) -> Result<<M::Yokeable as Yokeable<'de>>::Output, Error>
where
    M: DataMarker,
    // Actual bound:
    //     for<'de> <M::Yokeable as Yokeable<'de>>::Output: serde::de::Deserialize<'de>,
    // Necessary workaround bound (see `yoke::trait_hack` docs):
    for<'de> YokeTraitHack<<M::Yokeable as Yokeable<'de>>::Output>: serde::de::Deserialize<'de>,
{
    match syntax_option {
        #[cfg(feature = "provider_json")]
        SyntaxOption::Json => |bytes| {
            let mut d = get_json_deserializer_zc!(bytes);
            let data = YokeTraitHack::<<M::Yokeable as Yokeable>::Output>::deserialize(&mut d)?;
            Ok(data.0)
        },
        #[cfg(not(feature = "provider_json"))]
        SyntaxOption::Json => |_| Err(Error::UnknownSyntax(SyntaxOption::Json)),
        #[cfg(feature = "provider_bincode")]
        SyntaxOption::Bincode => |bytes| {
            let mut d = get_bincode_deserializer_zc!(bytes);
            let data = YokeTraitHack::<<M::Yokeable as Yokeable>::Output>::deserialize(&mut d)?;
            Ok(data.0)
        },
        #[cfg(not(feature = "provider_bincode"))]
        SyntaxOption::Bincode => |_| Err(Error::UnknownSyntax(SyntaxOption::Bincode)),
    }
}

/// Deserialize into a receiver used by [`SerdeDeDataProvider`](icu_provider::serde::SerdeDeDataProvider).
/// Covers all supported data formats.
#[allow(unused_variables)]
pub fn deserialize_into_receiver(
    rc_buffer: Rc<[u8]>,
    syntax_option: &SyntaxOption,
    receiver: &mut dyn SerdeDeDataReceiver,
) -> Result<(), Error> {
    match syntax_option {
        #[cfg(feature = "provider_json")]
        SyntaxOption::Json => {
            receiver.receive_rc_buffer(rc_buffer, |bytes, f2| {
                let mut d = get_json_deserializer_zc!(bytes);
                f2(&mut <dyn erased_serde::Deserializer>::erase(&mut d))
            })?;
            Ok(())
        }
        #[cfg(not(feature = "provider_json"))]
        SyntaxOption::Json => Err(Error::UnknownSyntax(SyntaxOption::Json)),
        #[cfg(feature = "provider_bincode")]
        SyntaxOption::Bincode => {
            receiver.receive_rc_buffer(rc_buffer, |bytes, f2| {
                let mut d = get_bincode_deserializer_zc!(bytes);
                f2(&mut <dyn erased_serde::Deserializer>::erase(&mut d))
            })?;
            Ok(())
        }
        #[cfg(not(feature = "provider_bincode"))]
        SyntaxOption::Bincode => Err(Error::UnknownSyntax(SyntaxOption::Bincode)),
    }
}
