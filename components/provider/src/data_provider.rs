// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/master/LICENSE ).

use crate::error::Error;
use crate::resource::ResourceKey;
use crate::resource::ResourcePath;
use icu_locid::LanguageIdentifier;
use std::borrow::Cow;
use std::fmt;
use std::fmt::Debug;

/// A struct to request a certain piece of data from a data provider.
#[derive(Clone, Debug, PartialEq)]
pub struct DataRequest {
    pub resource_path: ResourcePath,
}

impl fmt::Display for DataRequest {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}/{}",
            self.resource_path.key, self.resource_path.options
        )
    }
}

/// Create a DataRequest to a particular ResourceKey with default options.
impl From<ResourceKey> for DataRequest {
    fn from(key: ResourceKey) -> Self {
        DataRequest {
            resource_path: ResourcePath {
                key,
                options: Default::default(),
            },
        }
    }
}

impl DataRequest {
    /// Returns the LanguageIdentifier for this DataRequest, or an error if it is not present.
    ///
    /// # Example
    ///
    /// ```
    /// use icu_provider::prelude::*;
    ///
    /// const FOO_BAR: ResourceKey = icu_provider::resource_key!(x, "foo", "bar", 1);
    ///
    /// let req_no_langid = DataRequest {
    ///     resource_path: ResourcePath {
    ///         key: FOO_BAR,
    ///         options: ResourceOptions::default(),
    ///     }
    /// };
    ///
    /// let req_with_langid = DataRequest {
    ///     resource_path: ResourcePath {
    ///         key: FOO_BAR,
    ///         options: ResourceOptions {
    ///             variant: None,
    ///             langid: Some(icu_locid_macros::langid!("ar-EG")),
    ///         },
    ///     }
    /// };
    ///
    /// assert!(matches!(req_no_langid.try_langid(), Err(DataError::NeedsLanguageIdentifier(_))));
    /// assert!(matches!(req_with_langid.try_langid(), Ok(_)));
    /// ```
    pub fn try_langid(&self) -> Result<&LanguageIdentifier, Error> {
        self.resource_path
            .options
            .langid
            .as_ref()
            .ok_or_else(|| Error::NeedsLanguageIdentifier(self.clone()))
    }
}

/// A response object containing metadata about the returned data.
#[derive(Debug, Clone, PartialEq, Default)]
pub struct DataResponseMetadata {
    /// The language of the returned data, or None if the resource key isn't localized.
    pub data_langid: Option<LanguageIdentifier>,
}

/// A response object containing an object as payload and metadata about it.
#[derive(Debug, Clone)]
pub struct DataResponse<'d, T>
where
    T: ToOwned + ?Sized,
    <T as ToOwned>::Owned: Debug,
{
    /// Metadata about the returned object.
    pub metadata: DataResponseMetadata,

    /// The object itself; None if it was not loaded.
    pub payload: Option<Cow<'d, T>>,
}

impl<'d, T> DataResponse<'d, T>
where
    T: ToOwned + ?Sized,
    <T as ToOwned>::Owned: Debug,
{
    /// Convenience method: borrows the underlying payload. Error if not present.
    pub fn borrow_payload(&self) -> Result<&T, Error> {
        use std::borrow::Borrow;
        self.payload
            .as_ref()
            .map(|cow| cow.borrow())
            .ok_or(Error::MissingPayload)
    }

    /// Convenience method: takes ownership of the underlying payload. Error if not present.
    pub fn take_payload(&mut self) -> Result<Cow<'d, T>, Error> {
        self.payload.take().ok_or(Error::MissingPayload)
    }
}

// TODO(#434): Implement DataProvider<ErasedDataStruct>

/// A generic data provider that loads a payload of a specific type.
pub trait DataProvider<'d, T>
where
    T: ToOwned + ?Sized,
    <T as ToOwned>::Owned: Debug,
{
    /// Query the provider for data, returning the result.
    ///
    /// Returns Ok if the request successfully loaded data. If data failed to load, returns an
    /// Error with more information.
    fn load_payload(&self, req: &DataRequest) -> Result<DataResponse<'d, T>, Error>;
}
