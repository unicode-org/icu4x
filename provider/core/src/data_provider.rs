// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

#[cfg(test)]
#[cfg(feature = "serde")]
mod test;

use crate::error::Error;
use crate::resource::ResourceKey;
use crate::resource::ResourcePath;
use icu_locid::LanguageIdentifier;
use std::borrow::Cow;
use std::convert::TryFrom;
use std::fmt;
use std::fmt::Debug;
use std::rc::Rc;
use yoke::Yoke;
use yoke::Yokeable;
use crate::marker::DataMarker;

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

/// Create a [`DataRequest`] to a particular [`ResourceKey`] with default options.
impl From<ResourceKey> for DataRequest {
    fn from(key: ResourceKey) -> Self {
        Self {
            resource_path: ResourcePath {
                key,
                options: Default::default(),
            },
        }
    }
}

impl DataRequest {
    /// Returns the [`LanguageIdentifier`] for this [`DataRequest`], or an error if it is not present.
    ///
    /// # Examples
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

pub(crate) enum DataPayloadInner<'d, 's: 'd, M>
where
    M: DataMarker<'s>,
{
    Borrowed(Yoke<M::Yokeable, &'d M::Cart>),
    RcStruct(Yoke<M::Yokeable, Rc<M::Cart>>),
    Owned(Yoke<M::Yokeable, Option<&'static ()>>),
}

/// A wrapper around the payload returned in a [`DataResponse`].
///
/// # Examples
///
/// ```
/// use icu_provider::prelude::*;
///
/// let payload = DataPayload::from_borrowed("Demo");
///
/// assert_eq!("Demo", payload.get());
/// ```
pub struct DataPayload<'d, 's, M>
where
    M: DataMarker<'s>,
{
    pub(crate) inner: DataPayloadInner<'d, 's, M>,
}

// TODO
impl<'d, 's, M> Debug for DataPayload<'d, 's, M>
where
    M: DataMarker<'s>,
{
    fn fmt(&self, _f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}
impl<'d, 's, M> PartialEq for DataPayload<'d, 's, M>
where
    M: DataMarker<'s>,
{
    fn eq(&self, _other: &Self) -> bool {
        todo!()
    }
}
impl<'d, 's, M> Clone for DataPayload<'d, 's, M>
where
    M: DataMarker<'s>,
{
    fn clone(&self) -> Self {
        todo!()
    }
}

/// TODO: MOVE THIS TO THE YOKE CRATE ///

pub trait ZeroCopyClone<C: ?Sized>: for<'a> Yokeable<'a> {
    fn zcc<'b>(this: &'b C) -> <Self as Yokeable<'b>>::Output;
}

fn make_borrowed_yoke<'b, 's, C: ?Sized, Y: ZeroCopyClone<C> + for<'a> Yokeable<'a>>(
    cart: &'b C,
) -> Yoke<Y, &'b C> {
    Yoke::<Y, &'b C>::attach_to_cart_badly(cart, Y::zcc)
}

fn make_rc_yoke<'b, 's, C: ?Sized, Y: ZeroCopyClone<C> + for<'a> Yokeable<'a>>(
    cart: Rc<C>,
) -> Yoke<Y, Rc<C>> {
    Yoke::<Y, Rc<C>>::attach_to_cart_badly(cart, Y::zcc)
}

/// END ///

impl<'d, 's, M> DataPayload<'d, 's, M>
where
    M: DataMarker<'s>,
    M::Yokeable: ZeroCopyClone<M::Cart>,
{
    /// Convert a partially owned (`'d`) data struct into a DataPayload.
    #[inline]
    pub fn from_partial_owned(data: Rc<M::Cart>) -> Self {
        Self {
            inner: DataPayloadInner::RcStruct(make_rc_yoke(data)),
        }
    }

    /// Convert a borrowed data struct into a DataPayload.
    #[inline]
    pub fn from_borrowed(data: &'d M::Cart) -> Self {
        Self {
            inner: DataPayloadInner::Borrowed(make_borrowed_yoke(data)),
        }
    }
}

impl<'d, 's, M> DataPayload<'d, 's, M>
where
    M: DataMarker<'s>,
{
    /// Convert a fully owned (`'static`) data struct into a DataPayload.
    #[inline]
    pub fn from_owned(data: M::Yokeable) -> Self {
        Self {
            inner: DataPayloadInner::Owned(Yoke::new_owned(data)),
        }
    }

    /// Mutate the data contained in this DataPayload.
    ///
    /// For safety, all mutation operations must take place within a helper function that cannot
    /// borrow data from the surrounding context.
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// use icu_provider::prelude::*;
    ///
    /// let mut payload = DataPayload::<str>::from_owned("Hello".to_string());
    ///
    /// payload.with_mut(|s| s.push_str(" World"));
    ///
    /// assert_eq!("Hello World", payload.get());
    /// ```
    ///
    /// To transfer data from the context into the data struct, use the `move` keyword:
    ///
    /// ```
    /// use icu_provider::prelude::*;
    ///
    /// let initial_vector = vec!["Foo".to_string()];
    /// let mut payload: DataPayload<Vec<String>> = DataPayload::from_owned(initial_vector);
    ///
    /// let new_value = "Bar".to_string();
    /// payload.with_mut(move |v| v.push(new_value));
    ///
    /// assert_eq!("Foo", payload.get()[0]);
    /// assert_eq!("Bar", payload.get()[1]);
    /// ```
    pub fn with_mut<'a, F>(&'a mut self, f: F)
    where
        F: 'static + for<'b> FnOnce(&'b mut <M::Yokeable as Yokeable<'a>>::Output),
    {
        use DataPayloadInner::*;
        match &mut self.inner {
            Borrowed(yoke) => yoke.with_mut(f),
            RcStruct(yoke) => yoke.with_mut(f),
            Owned(yoke) => yoke.with_mut(f),
        }
    }

    /// Borrows the underlying data.
    ///
    /// This function should be used like `Deref` would normally be used. For more information on
    /// why DataPayload cannot implement `Deref`, see the `yoke` crate.
    ///
    /// # Examples
    ///
    /// ```
    /// use icu_provider::prelude::*;
    ///
    /// let payload = DataPayload::from_borrowed("Demo");
    ///
    /// assert_eq!("Demo", payload.get());
    /// ```
    pub fn get<'a>(&'a self) -> &'a <M::Yokeable as Yokeable<'a>>::Output {
        use DataPayloadInner::*;
        match &self.inner {
            Borrowed(yoke) => yoke.get(),
            RcStruct(yoke) => yoke.get(),
            Owned(yoke) => yoke.get(),
        }
    }
}

impl<'d, 's, M> DataPayload<'d, 's, M>
where
    M: DataMarker<'s>,
    <M::Yokeable as Yokeable<'s>>::Output: ToOwned,
{
    /// Converts the DataPayload into a Cow. May require cloning the data.
    ///
    /// # Examples
    ///
    /// ```
    /// use icu_provider::prelude::*;
    /// use std::borrow::Cow;
    ///
    /// let payload = DataPayload::from_borrowed("Demo");
    /// let data: Cow<str> = payload.into_cow();
    /// assert!(matches!(data, Cow::Borrowed(_)));
    ///
    /// let payload = DataPayload::<str>::from_owned("Demo".to_string());
    /// let data: Cow<str> = payload.into_cow();
    /// assert!(matches!(data, Cow::Owned(_)));
    /// ```
    #[inline]
    pub fn into_cow(self) -> Cow<'d, <M::Yokeable as Yokeable<'s>>::Output> {
        todo!()
    }
}

/// A response object containing an object as payload and metadata about it.
#[derive(Debug, Clone)]
pub struct DataResponse<'d, 's, M>
where
    M: DataMarker<'s>,
{
    /// Metadata about the returned object.
    pub metadata: DataResponseMetadata,

    /// The object itself; None if it was not loaded.
    pub payload: Option<DataPayload<'d, 's, M>>,
}

impl<'d, 's, M> DataResponse<'d, 's, M>
where
    M: DataMarker<'s>,
{
    /// Takes ownership of the underlying payload. Error if not present.
    #[inline]
    pub fn take_payload(self) -> Result<DataPayload<'d, 's, M>, Error> {
        self.payload.ok_or(Error::MissingPayload)
    }
}

impl<'d, 's, M> TryFrom<DataResponse<'d, 's, M>> for DataPayload<'d, 's, M>
where
    M: DataMarker<'s>,
{
    type Error = Error;

    fn try_from(response: DataResponse<'d, 's, M>) -> Result<Self, Self::Error> {
        response.take_payload()
    }
}

/// A generic data provider that loads a payload of a specific type.
///
/// See examples on some of the concrete implementations:
///
/// - [`HelloWorldProvider`](crate::hello_world::HelloWorldProvider)
/// - [`StructProvider`](crate::struct_provider::StructProvider)
/// - [`InvariantDataProvider`](crate::inv::InvariantDataProvider)
pub trait DataProvider<'d, 's, M>
where
    M: DataMarker<'s>,
{
    /// Query the provider for data, returning the result.
    ///
    /// Returns [`Ok`] if the request successfully loaded data. If data failed to load, returns an
    /// Error with more information.
    fn load_payload(&self, req: &DataRequest) -> Result<DataResponse<'d, 's, M>, Error>;
}
