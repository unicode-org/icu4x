use crate::cloneable_any::CloneableAny;
use crate::data_entry::DataEntry;
use crate::data_key::DataKey;
use crate::error::PayloadError;
use crate::error::ResponseError;
use icu_locale::LanguageIdentifier;
use std::any::Any;
use std::any::TypeId;
use std::borrow::Borrow;
use std::borrow::BorrowMut;
use std::borrow::Cow;
use std::fmt;

/// A struct to request a certain hunk of data from a data provider.
#[derive(PartialEq, Clone, Debug)]
pub struct Request {
    pub data_key: DataKey,
    pub data_entry: DataEntry,
}

/// A response object containing a data hunk ("payload").
#[derive(Debug, Clone)]
pub struct Response<'d> {
    pub data_langid: LanguageIdentifier,
    payload: Cow<'d, dyn CloneableAny>,
    // source: Cow<'static, str>,
}

// TODO: Should this be an implemention of std::borrow::Borrow?
// TODO: Should the error types be &dyn Any, like for Box<dyn Any>::downcast?
impl<'d> Response<'d> {
    /// Get an immutable reference to the payload in a Response object.
    /// The payload may or may not be owned by the Response.
    pub fn borrow_payload<T: 'static>(&self) -> Result<&T, PayloadError> {
        let borrowed: &dyn CloneableAny = self.payload.borrow();
        borrowed
            .as_any()
            .downcast_ref::<T>()
            .ok_or_else(|| PayloadError::from(borrowed.as_any().type_id()))
    }

    /// Get a mutable reference to the payload in a Response object.
    /// The payload may or may not be owned by the Response.
    pub fn borrow_payload_mut<T: 'static>(&mut self) -> Result<&mut T, PayloadError> {
        let borrowed_mut: &mut dyn CloneableAny = self.payload.to_mut().borrow_mut();
        // TODO: If I move this into the lambda, I get E0502. Why?
        let type_id = borrowed_mut.as_any().type_id();
        borrowed_mut
            .as_any_mut()
            .downcast_mut::<T>()
            .ok_or_else(|| PayloadError::from(type_id))
    }

    /// Take ownership of the payload from a Response object. Consumes the Response object.
    pub fn take_payload<T: 'static + Clone>(self) -> Result<Cow<'d, T>, PayloadError> {
        match self.payload {
            Cow::Borrowed(borrowed) => match borrowed.as_any().downcast_ref::<T>() {
                Some(v) => Ok(Cow::Borrowed(v)),
                None => Err(PayloadError::from(borrowed.as_any().type_id())),
            },
            Cow::Owned(boxed) => match boxed.into_any().downcast::<T>() {
                Ok(boxed_t) => Ok(Cow::Owned(*boxed_t)),
                Err(boxed_any) => Err(PayloadError::from(boxed_any.type_id())),
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
pub struct ResponseBuilder {
    pub data_langid: LanguageIdentifier,
}

impl ResponseBuilder {
    /// Construct a Response from the builder, with owned data.
    /// Consumes both the builder and the data.
    /// Returns the 'static lifetime since there is no borrowed data.
    pub fn with_owned_payload<T: 'static + Clone + fmt::Debug>(self, t: T) -> Response<'static> {
        Response {
            data_langid: self.data_langid,
            payload: Cow::Owned(Box::new(t) as Box<dyn CloneableAny>),
        }
    }

    /// Construct a Response from the builder, with borrowed data.
    /// Consumes the builder, but not the data.
    #[allow(clippy::needless_lifetimes)]
    pub fn with_borrowed_payload<'d, T: 'static + Clone + fmt::Debug>(
        self,
        t: &'d T,
    ) -> Response<'d> {
        Response {
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
pub trait DataProvider<'a, 'd> {
    fn load(&'a self, req: &Request) -> Result<Response<'d>, ResponseError>;
}
