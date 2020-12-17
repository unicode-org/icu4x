// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/master/LICENSE ).

use crate::data_entry::DataEntry;
use crate::data_key::DataKey;
use crate::data_receiver::DataReceiver;
use crate::data_receiver::DataReceiverForType;
use icu_locid::LanguageIdentifier;
use std::any::Any;
use std::borrow::Cow;
use std::fmt;
use std::fmt::Debug;

// Re-export Error so it can be referenced by "data_provider::Error"
pub use crate::error::Error;

/// A struct to request a certain hunk of data from a data provider.
#[derive(PartialEq, Clone, Debug)]
pub struct DataRequest {
    pub data_key: DataKey,
    pub data_entry: DataEntry,
}

impl fmt::Display for DataRequest {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}/{}", self.data_key, self.data_entry)
    }
}

/// A response object containing metadata about the returned data.
#[derive(Debug, Clone)]
pub struct DataResponse {
    pub data_langid: LanguageIdentifier,
}

/// An abstract data provider that loads a payload given a request object.
pub trait DataProvider<'d> {
    /// Query the provider for data, loading it into a DataReceiver.
    ///
    /// Returns Ok if the request successfully loaded data. If data failed to load, returns an
    /// Error with more information.
    fn load_to_receiver(
        &self,
        req: &DataRequest,
        receiver: &mut dyn DataReceiver<'d, 'static>,
    ) -> Result<DataResponse, Error>;
}

/// A response object containing an object as payload and metadata about it.
pub struct DataResponseWithPayload<'d, T>
where
    T: Clone + Debug,
{
    /// Metadata about the returned object.
    pub response: DataResponse,

    /// The object itself; None if it was not loaded.
    pub payload: Option<Cow<'d, T>>,
}

impl<'d> dyn DataProvider<'d> + 'd {
    /// Query the provider for data, returning the result.
    ///
    /// Returns Ok if the request successfully loaded data. If data failed to load, returns an
    /// Error with more information.
    pub fn load_payload<T>(
        &self,
        req: &DataRequest,
    ) -> Result<DataResponseWithPayload<'d, T>, Error>
    where
        T: serde::Deserialize<'static> + erased_serde::Serialize + Any + Clone + Debug,
    {
        let mut receiver = DataReceiverForType::<T>::new();
        let response = self.load_to_receiver(req, &mut receiver)?;
        Ok(DataResponseWithPayload {
            response,
            payload: receiver.payload,
        })
    }
}
