// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

#[cfg(test)]
#[cfg(feature = "serde")]
mod test;

use crate::error::Error;
use crate::marker::DataMarker;
use crate::resource::ResourceKey;
use crate::resource::ResourcePath;
use icu_locid::LanguageIdentifier;
use std::convert::TryFrom;
use std::fmt;
use std::fmt::Debug;
use std::rc::Rc;
use yoke::*;

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
    Owned(Yoke<M::Yokeable, ()>),
    RcBuf(Yoke<M::Yokeable, Rc<[u8]>>),
}

/// A wrapper around the payload returned in a [`DataResponse`].
///
/// Internally, the data is represented using the [`yoke`] crate, with several variations for
/// different ownership models.
///
/// `DataPayload` is closely coupled with [`DataMarker`].
///
/// # Examples
///
/// Basic usage, using the `CowStrMarker` marker:
///
/// ```
/// use icu_provider::prelude::*;
/// use icu_provider::marker::CowStrMarker;
///
/// let payload = DataPayload::<CowStrMarker>::from_borrowed("Demo");
///
/// assert_eq!("Demo", payload.get());
/// ```
pub struct DataPayload<'d, 's, M>
where
    M: DataMarker<'s>,
{
    pub(crate) inner: DataPayloadInner<'d, 's, M>,
}

impl<'d, 's, M> Debug for DataPayload<'d, 's, M>
where
    M: DataMarker<'s>,
    for<'a> &'a <M::Yokeable as Yokeable<'a>>::Output: Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.get().fmt(f)
    }
}

impl<'d, 's, M> Clone for DataPayload<'d, 's, M>
where
    M: DataMarker<'s>,
    for<'a> <M::Yokeable as Yokeable<'a>>::Output: Clone,
{
    /// Note: This function is currently inoperable. For more details, see
    /// https://github.com/unicode-org/icu4x/issues/753
    fn clone(&self) -> Self {
        use DataPayloadInner::*;
        let new_inner = match &self.inner {
            Borrowed(yoke) => Borrowed(yoke.clone()),
            RcStruct(yoke) => RcStruct(yoke.clone()),
            Owned(yoke) => Owned(yoke.clone()),
            RcBuf(yoke) => RcBuf(yoke.clone()),
        };
        Self { inner: new_inner }
    }
}

impl<'d, 's, M> DataPayload<'d, 's, M>
where
    M: DataMarker<'s>,
    M::Yokeable: ZeroCopyFrom<M::Cart>,
{
    /// Convert an [`Rc`]`<`[`Cart`]`>` into a [`DataPayload`]. The data need not be fully owned;
    /// it may be constrained by the `'s` lifetime.
    ///
    /// # Examples
    ///
    /// ```
    /// use icu_provider::prelude::*;
    /// use icu_provider::hello_world::*;
    /// use std::rc::Rc;
    /// use std::borrow::Cow;
    ///
    /// let local_data = "example".to_string();
    ///
    /// let rc_struct = Rc::from(HelloWorldV1 {
    ///     message: Cow::Borrowed(&local_data),
    /// });
    ///
    /// let payload = DataPayload::<HelloWorldV1Marker>::from_partial_owned(rc_struct.clone());
    ///
    /// assert_eq!(payload.get(), &*rc_struct);
    /// ```
    ///
    /// [`Cart`]: crate::marker::DataMarker::Cart
    #[inline]
    pub fn from_partial_owned(data: Rc<M::Cart>) -> Self {
        Self {
            inner: DataPayloadInner::RcStruct(Yoke::attach_to_rc_cart(data)),
        }
    }

    /// Convert an `&'d `[`Cart`] into a [`DataPayload`]. The data need not be fully owned;
    /// it may be constrained by the `'s` lifetime.
    ///
    /// # Examples
    ///
    /// ```
    /// use icu_provider::prelude::*;
    /// use icu_provider::hello_world::*;
    /// use std::borrow::Cow;
    ///
    /// let local_data = "example".to_string();
    ///
    /// let local_struct = HelloWorldV1 {
    ///     message: Cow::Borrowed(&local_data),
    /// };
    ///
    /// let payload = DataPayload::<HelloWorldV1Marker>::from_borrowed(&local_struct);
    ///
    /// assert_eq!(payload.get(), &local_struct);
    /// ```
    ///
    /// [`Cart`]: crate::marker::DataMarker::Cart
    #[inline]
    pub fn from_borrowed(data: &'d M::Cart) -> Self {
        Self {
            inner: DataPayloadInner::Borrowed(Yoke::attach_to_borrowed_cart(data)),
        }
    }
}

impl<'d, 's, M> DataPayload<'d, 's, M>
where
    M: DataMarker<'s>,
{
    /// Convert a byte buffer into a [`DataPayload`]. A function must be provided to perform the
    /// conversion. This can often be a Serde deserialization operation.
    ///
    /// Due to [compiler bug #84937](https://github.com/rust-lang/rust/issues/84937), call sites
    /// for this function may not compile; if this happens, use
    /// [`try_from_rc_buffer_badly()`](Self::try_from_rc_buffer_badly) instead.
    #[inline]
    pub fn try_from_rc_buffer<E>(
        rc_buffer: Rc<[u8]>,
        f: impl for<'de> FnOnce(&'de [u8]) -> Result<<M::Yokeable as Yokeable<'de>>::Output, E>,
    ) -> Result<Self, E> {
        let yoke = Yoke::try_attach_to_cart(rc_buffer, f)?;
        Ok(Self {
            inner: DataPayloadInner::RcBuf(yoke),
        })
    }

    /// Convert a byte buffer into a [`DataPayload`]. A function must be provided to perform the
    /// conversion. This can often be a Serde deserialization operation.
    ///
    /// For a version of this function that takes a `FnOnce` instead of a raw function pointer,
    /// see [`try_from_rc_buffer()`](Self::try_from_rc_buffer).
    ///
    /// # Examples
    ///
    /// ```
    /// # #[cfg(feature = "provider_serde")] {
    /// use icu_provider::prelude::*;
    /// use icu_provider::hello_world::*;
    /// use std::rc::Rc;
    /// use icu_provider::yoke::Yokeable;
    ///
    /// let json_text = "{\"message\":\"Hello World\"}";
    /// let json_rc_buffer: Rc<[u8]> = json_text.as_bytes().into();
    ///
    /// let payload = DataPayload::<HelloWorldV1Marker>::try_from_rc_buffer_badly(
    ///     json_rc_buffer.clone(),
    ///     |bytes| {
    ///         serde_json::from_slice(bytes)
    ///     }
    /// )
    /// .expect("JSON is valid");
    ///
    /// assert_eq!("Hello World", payload.get().message);
    /// # } // feature = "provider_serde"
    /// ```
    #[allow(clippy::type_complexity)]
    pub fn try_from_rc_buffer_badly<E>(
        rc_buffer: Rc<[u8]>,
        f: for<'de> fn(&'de [u8]) -> Result<<M::Yokeable as Yokeable<'de>>::Output, E>,
    ) -> Result<Self, E> {
        let yoke = Yoke::try_attach_to_cart_badly(rc_buffer, f)?;
        Ok(Self {
            inner: DataPayloadInner::RcBuf(yoke),
        })
    }
}

impl<'d, 's, M> DataPayload<'d, 's, M>
where
    M: DataMarker<'s>,
{
    /// Convert a fully owned (`'static`) data struct into a DataPayload.
    ///
    /// # Examples
    ///
    /// ```
    /// use icu_provider::prelude::*;
    /// use icu_provider::hello_world::*;
    /// use std::borrow::Cow;
    ///
    /// let local_struct = HelloWorldV1 {
    ///     message: Cow::Owned("example".to_string()),
    /// };
    ///
    /// let payload = DataPayload::<HelloWorldV1Marker>::from_owned(local_struct.clone());
    ///
    /// assert_eq!(payload.get(), &local_struct);
    /// ```
    #[inline]
    pub fn from_owned(data: M::Yokeable) -> Self {
        Self {
            inner: DataPayloadInner::Owned(Yoke::new_always_owned(data)),
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
    /// use icu_provider::marker::CowStrMarker;
    ///
    /// let mut payload = DataPayload::<CowStrMarker>::from_borrowed("Hello");
    ///
    /// payload.with_mut(|s| s.to_mut().push_str(" World"));
    ///
    /// assert_eq!("Hello World", payload.get());
    /// ```
    ///
    /// To transfer data from the context into the data struct, use the `move` keyword:
    ///
    /// ```
    /// use icu_provider::prelude::*;
    /// use icu_provider::marker::CowStrMarker;
    ///
    /// let mut payload = DataPayload::<CowStrMarker>::from_borrowed("Hello");
    ///
    /// let suffix = " World".to_string();
    /// payload.with_mut(move |s| s.to_mut().push_str(&suffix));
    ///
    /// assert_eq!("Hello World", payload.get());
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
            RcBuf(yoke) => yoke.with_mut(f),
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
    /// use icu_provider::marker::CowStrMarker;
    ///
    /// let payload = DataPayload::<CowStrMarker>::from_borrowed("Demo");
    ///
    /// assert_eq!("Demo", payload.get());
    /// ```
    #[allow(clippy::needless_lifetimes)]
    pub fn get<'a>(&'a self) -> &'a <M::Yokeable as Yokeable<'a>>::Output {
        use DataPayloadInner::*;
        match &self.inner {
            Borrowed(yoke) => yoke.get(),
            RcStruct(yoke) => yoke.get(),
            Owned(yoke) => yoke.get(),
            RcBuf(yoke) => yoke.get(),
        }
    }
}

/// A response object containing an object as payload and metadata about it.
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

impl<'d, 's, M> Debug for DataResponse<'d, 's, M>
where
    M: DataMarker<'s>,
    for<'a> &'a <M::Yokeable as Yokeable<'a>>::Output: Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "DataResponse {{ metadata: {:?}, payload: {:?} }}",
            self.metadata, self.payload
        )
    }
}

impl<'d, 's, M> Clone for DataResponse<'d, 's, M>
where
    M: DataMarker<'s>,
    for<'a> <M::Yokeable as Yokeable<'a>>::Output: Clone,
{
    /// Note: This function is currently inoperable. For more details, see
    /// https://github.com/unicode-org/icu4x/issues/753
    fn clone(&self) -> Self {
        Self {
            metadata: self.metadata.clone(),
            payload: self.payload.clone(),
        }
    }
}

#[test]
fn test_debug() {
    use crate::hello_world::*;
    use std::borrow::Cow;
    let resp = DataResponse::<HelloWorldV1Marker> {
        metadata: Default::default(),
        payload: Some(DataPayload::from_borrowed(&HelloWorldV1 {
            message: Cow::Borrowed("foo"),
        })),
    };
    assert_eq!("DataResponse { metadata: DataResponseMetadata { data_langid: None }, payload: Some(HelloWorldV1 { message: \"foo\" }) }", format!("{:?}", resp));
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
