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
pub struct DataResponseV2 {
    pub data_langid: LanguageIdentifier,
}

/// An abstract data provider that loads a payload given a request object.
pub trait DataProviderV2<'d> {
    /// Query the provider for data, loading it into a DataReceiver.
    ///
    /// Returns Ok if the request successfully loaded data. If data failed to load, returns an
    /// Error with more information.
    fn load_v2(
        &self,
        req: &DataRequest,
        receiver: &mut dyn DataReceiver<'d, 'static>,
    ) -> Result<DataResponseV2, Error>;
}

pub struct DataResponseV2a<'d, T>
where
    T: Clone + Debug,
{
    pub response: DataResponseV2,
    pub payload: Option<Cow<'d, T>>,
}

impl<'d> dyn DataProviderV2<'d> + 'd {
    /// Query the provider for data, returning the result.
    ///
    /// Returns Ok if the request successfully loaded data. If data failed to load, returns an
    /// Error with more information.
    pub fn load_v2a<T>(&self, req: &DataRequest) -> Result<DataResponseV2a<'d, T>, Error>
    where
        T: serde::Deserialize<'static> + erased_serde::Serialize + Any + Clone + Debug,
    {
        let mut receiver = DataReceiverForType::<T>::new();
        let response = self.load_v2(req, &mut receiver)?;
        Ok(DataResponseV2a {
            response,
            payload: receiver.payload,
        })
    }
}
