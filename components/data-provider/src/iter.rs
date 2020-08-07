use crate::error::Error;
use crate::prelude::*;
use erased_serde;

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
    fn put(
        &mut self,
        req: &data_provider::Request,
        obj: &dyn erased_serde::Serialize,
    ) -> Result<(), Box<dyn std::error::Error>>;
}

impl<'d, T> IterableDataProvider<'d> for T
where
    T: DataProvider<'d> + DataEntryCollection,
{
    fn export_key(&self, data_key: &DataKey, sink: &mut dyn DataExporter) -> Result<(), Error> {
        for data_entry in self.iter_for_key(data_key)? {
            let req = data_provider::Request {
                data_key: *data_key,
                data_entry: data_entry.clone(),
            };
            let response = self.load(&req)?;
            let payload = response.borrow_as_serialize();
            sink.put(&req, &payload)?;
        }
        Ok(())
    }
}
