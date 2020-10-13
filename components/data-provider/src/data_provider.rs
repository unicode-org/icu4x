// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/master/LICENSE ).
use crate::cloneable_any::CloneableAny;
use crate::data_entry::DataEntry;
use crate::data_key::DataKey;
use icu_locale::LanguageIdentifier;
use std::any::Any;
use std::any::TypeId;
use std::borrow::Borrow;
use std::borrow::BorrowMut;
use std::borrow::Cow;
use std::fmt;

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

/// A response object containing a data hunk ("payload").
#[derive(Debug, Clone)]
pub struct DataResponse<'d> {
    pub data_langid: LanguageIdentifier,
    payload: Cow<'d, dyn CloneableAny>,
    // source: Cow<'static, str>,
}

impl<'d> DataResponse<'d> {
    /// Get an immutable reference to the payload in a Response object.
    /// The payload may or may not be owned by the Response.
    pub fn borrow_payload<T: 'static>(&self) -> Result<&T, Error> {
        let borrowed: &dyn CloneableAny = self.payload.borrow();
        borrowed
            .as_any()
            .downcast_ref::<T>()
            .ok_or_else(|| Error::MismatchedType {
                actual: borrowed.as_any().type_id(),
                generic: Some(TypeId::of::<T>()),
            })
    }

    /// Get an immutable reference to the payload as an erased_serde::Serialize trait object.
    pub fn borrow_as_serialize(&self) -> &dyn erased_serde::Serialize {
        let borrowed: &dyn CloneableAny = self.payload.borrow();
        borrowed.as_serialize()
    }

    /// Get a mutable reference to the payload in a Response object.
    /// The payload may or may not be owned by the Response.
    pub fn borrow_payload_mut<T: 'static>(&mut self) -> Result<&mut T, Error> {
        let borrowed_mut: &mut dyn CloneableAny = self.payload.to_mut().borrow_mut();
        // TODO: If I move this into the lambda, I get E0502. Why?
        let type_id = borrowed_mut.as_any().type_id();
        borrowed_mut
            .as_any_mut()
            .downcast_mut::<T>()
            .ok_or_else(|| Error::MismatchedType {
                actual: type_id,
                generic: Some(TypeId::of::<T>()),
            })
    }

    /// Take ownership of the payload from a Response object. Consumes the Response object.
    pub fn take_payload<T: 'static + Clone>(self) -> Result<Cow<'d, T>, Error> {
        match self.payload {
            Cow::Borrowed(borrowed) => match borrowed.as_any().downcast_ref::<T>() {
                Some(v) => Ok(Cow::Borrowed(v)),
                None => Err(Error::MismatchedType {
                    actual: borrowed.as_any().type_id(),
                    generic: Some(TypeId::of::<T>()),
                }),
            },
            Cow::Owned(boxed) => match boxed.into_any().downcast::<T>() {
                Ok(boxed_t) => Ok(Cow::Owned(*boxed_t)),
                Err(boxed_any) => Err(Error::MismatchedType {
                    actual: boxed_any.type_id(),
                    generic: Some(TypeId::of::<T>()),
                }),
            },
        }
    }

    /// Get the TypeId of the payload.
    pub fn get_payload_type_id(&self) -> TypeId {
        let borrowed: &dyn CloneableAny = self.payload.borrow();
        borrowed.as_any().type_id()
    }
}

/// Builder class used to construct a Response.
pub struct DataResponseBuilder {
    pub data_langid: LanguageIdentifier,
}

impl DataResponseBuilder {
    /// Construct a DataResponse from the builder, with owned data.
    /// Consumes both the builder and the data.
    /// Returns the 'static lifetime since there is no borrowed data.
    pub fn with_owned_payload<T: 'static + Clone + erased_serde::Serialize + fmt::Debug>(
        self,
        t: T,
    ) -> DataResponse<'static> {
        DataResponse {
            data_langid: self.data_langid,
            payload: Cow::Owned(Box::new(t) as Box<dyn CloneableAny>),
        }
    }

    /// Construct a DataResponse from the builder, with borrowed data.
    /// Consumes the builder, but not the data.
    #[allow(clippy::needless_lifetimes)]
    pub fn with_borrowed_payload<'d, T: 'static + Clone + erased_serde::Serialize + fmt::Debug>(
        self,
        t: &'d T,
    ) -> DataResponse<'d> {
        DataResponse {
            data_langid: self.data_langid,
            payload: Cow::Borrowed(t),
        }
    }
}

/// An abstract data provider that takes a request object and returns a response with a payload.
/// Lifetimes:
/// - 'a = lifetime of the DataProvider object
/// - 'd = lifetime of the borrowed payload
/// Note: 'd and 'a can be the same, but they do not need to be. For example, 'd = 'static if:
/// 1. The provider always returns data that lives in static memory
/// 2. The provider always returns owned data, not borrowed data
// TODO: Make this async
// #[async_trait]
pub trait DataProvider<'d> {
    /// Query the provider for data. Returns Ok if the request successfully loaded data. If data
    /// failed to load, returns an Error with more information.
    fn load<'a>(&'a self, req: &DataRequest) -> Result<DataResponse<'d>, Error>;
}

impl<'d> dyn DataProvider<'d> + 'd {
    /// Query the provider for data. Returns Ok(Some) if the request successfully loaded data. If
    /// data failed to load due to the provider not supporting the requested category or data key,
    /// returns Ok(None). Otherwise, returns an Error.
    pub fn load_graceful(&self, req: &DataRequest) -> Result<Option<DataResponse<'d>>, Error> {
        match self.load(req) {
            Ok(response) => Ok(Some(response)),
            Err(err) => match err {
                Error::UnsupportedCategory(_) => Ok(None),
                Error::UnsupportedDataKey(_) => Ok(None),
                _ => Err(err),
            },
        }
    }
}
