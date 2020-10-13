// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/master/LICENSE ).
use crate::error::Error;
use crate::prelude::*;

/// An object that exposes an iterable list of DataEntry instances for a certain DataKey.
pub trait DataEntryCollection {
    // Note: This trait could have an associated type for the Iterator, but associated types
    // prevent the trait from being used as a type object. Instead, we return a Boxed Iterator.
    fn iter_for_key(
        &self,
        data_key: &DataKey,
    ) -> Result<Box<dyn Iterator<Item = DataEntry>>, Error>;
}

/// Auto-implemented trait: A data provider that allows for iteration over DataEntry instances.
pub trait IterableDataProvider<'d>: DataProvider<'d> + DataEntryCollection {
    /// Dump all data in this data provider for the specified key into the sink.
    fn export_key(&self, data_key: &DataKey, sink: &mut dyn DataExporter) -> Result<(), Error>;
}

/// Trait for objects capable of persisting serialized data hunks.
pub trait DataExporter {
    /// Save `obj` corresponding to `req`.
    fn put(
        &mut self,
        req: &DataRequest,
        obj: &dyn erased_serde::Serialize,
    ) -> Result<(), Box<dyn std::error::Error>>;

    /// Whether to load and dump data for the given entry. This function enables the
    /// DataExporter to filter out certain data entries.
    fn includes(&self, data_entry: &DataEntry) -> bool;
}

impl<'d, T> IterableDataProvider<'d> for T
where
    T: DataProvider<'d> + DataEntryCollection,
{
    fn export_key(&self, data_key: &DataKey, sink: &mut dyn DataExporter) -> Result<(), Error> {
        for data_entry in self.iter_for_key(data_key)? {
            if !sink.includes(&data_entry) {
                continue;
            }
            let req = DataRequest {
                data_key: *data_key,
                data_entry,
            };
            let response = self.load(&req)?;
            let payload = response.borrow_as_serialize();
            sink.put(&req, &payload)?;
        }
        Ok(())
    }
}
