// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/master/LICENSE ).

use crate::data_receiver::DataReceiver;
use crate::data_receiver::DataReceiverForType;
use crate::resource::ResourceKey;
use crate::resource::ResourcePath;
use icu_locid::LanguageIdentifier;
use std::any::Any;
use std::borrow::Cow;
use std::fmt;
use std::fmt::Debug;

// Re-export Error so it can be referenced by "data_provider::Error"
pub use crate::error::Error;

/// A struct to request a certain hunk of data from a data provider.
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
    T: Clone + Debug + 'd,
{
    /// Metadata about the returned object.
    pub metadata: DataResponseMetadata,

    /// The object itself; None if it was not loaded.
    pub payload: Option<Cow<'d, T>>,
}

/// A generic data provider that loads a payload of a specific type.
pub trait DataProvider<'d, T>
where
    T: Clone + Debug + 'd,
{
    /// Query the provider for data, returning the result.
    ///
    /// Returns Ok if the request successfully loaded data. If data failed to load, returns an
    /// Error with more information.
    fn load_payload(&self, req: &DataRequest) -> Result<DataResponse<'d, T>, Error>;
}

/// A type-erased data provider that loads a payload of any type.
pub trait ErasedDataProvider<'d> {
    /// Query the provider for data, loading it into a DataReceiver.
    ///
    /// Returns Ok if the request successfully loaded data. If data failed to load, returns an
    /// Error with more information.
    fn load_to_receiver(
        &self,
        req: &DataRequest,
        receiver: &mut dyn DataReceiver<'d>,
    ) -> Result<DataResponseMetadata, Error>;
}

/// Helper macro to implement ErasedDataProvider on an object implementing DataProvider for a
/// single type. Calls to `self.load_to_receiver` delegate to `self.load_payload`.
#[macro_export]
macro_rules! impl_erased {
    ($type:ty, $lifetime:tt) => {
        impl<$lifetime> $crate::prelude::ErasedDataProvider<$lifetime> for $type {
            fn load_to_receiver(
                &self,
                req: &$crate::prelude::DataRequest,
                receiver: &mut dyn $crate::prelude::DataReceiver<$lifetime>,
            ) -> Result<$crate::prelude::DataResponseMetadata, $crate::prelude::DataError> {
                println!("{:?}", req);
                let result = self.load_payload(req)?;
                println!("{:?}", result);
                receiver.receive_optional_cow(result.payload)?;
                Ok(result.metadata)
            }
        }
    };
}

/// Convenience implementation of DataProvider<T> given an ErasedDataProvider trait object.
impl<'a, 'd, T> DataProvider<'d, T> for dyn ErasedDataProvider<'d> + 'a
where
    T: serde::Deserialize<'d> + serde::Serialize + Any + Clone + Debug,
{
    fn load_payload(&self, req: &DataRequest) -> Result<DataResponse<'d, T>, Error> {
        let mut receiver = DataReceiverForType::<T>::new();
        let metadata = self.load_to_receiver(req, &mut receiver)?;
        Ok(DataResponse {
            metadata,
            payload: receiver.payload,
        })
    }
}
