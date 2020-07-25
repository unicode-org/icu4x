use crate::data_entry::DataEntry;
use crate::data_key::DataKey;
use crate::data_provider::DataProvider;
use crate::error::ResponseError;

/// An object that exposes an iterable list of DataEntry instances.
pub trait DataEntryCollection {
    // Note: This trait could have an associated type for the Iterator, but associated types
    // prevent the trait from being used as a type object. Instead, we return a Boxed Iterator.
    fn iter_for_key(
        &self,
        data_key: &DataKey,
    ) -> Result<Box<dyn Iterator<Item = DataEntry>>, ResponseError>;
}

/// A data provider that also exposes an iterable list of DataEntry instances.
pub trait IterableDataProvider<'a, 'd>: DataProvider<'a, 'd> + DataEntryCollection {}

impl<'a, 'd, T> IterableDataProvider<'a, 'd> for T where
    T: DataProvider<'a, 'd> + DataEntryCollection
{
}
