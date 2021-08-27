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
use crate::yoke::trait_hack::YokeTraitHack;
use crate::yoke::*;

use alloc::rc::Rc;

use core::convert::TryFrom;
use core::fmt;
use core::fmt::Debug;
use core::marker::PhantomData;
use icu_locid::LanguageIdentifier;

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

pub(crate) enum DataPayloadInner<'data, M>
where
    M: DataMarker<'data>,
{
    RcStruct(Yoke<M::Yokeable, Rc<M::Cart>>),
    Owned(Yoke<M::Yokeable, ()>),
    RcBuf(Yoke<M::Yokeable, Rc<[u8]>>),
}

/// A container for data payloads returned from a [`DataProvider`].
///
/// [`DataPayload`] is built on top of the [`yoke`] framework, which allows for cheap, zero-copy
/// operations on data via the use of self-references. A [`DataPayload`] may be backed by one of
/// several data stores ("carts"):
///
/// 1. Fully-owned structured data ([`DataPayload::from_owned()`])
/// 2. Partially-owned structured data in an [`Rc`] ([`DataPayload::from_partial_owned()`])
/// 3. A reference-counted byte buffer ([`DataPayload::try_from_rc_buffer()`])
///
/// The type of the data stored in [`DataPayload`], and the type of the structured data store
/// (cart), is determined by the [`DataMarker`] type parameter.
///
/// ## Accessing the data
///
/// To get a reference to the data inside [`DataPayload`], use [`DataPayload::get()`]. If you need
/// to store the data for later use, it is recommended to store the [`DataPayload`] itself, not
/// the ephemeral reference, since the reference results in a short-lived lifetime.
///
/// ## Mutating the data
///
/// To modify the data stored in a [`DataPayload`], use [`DataPayload::with_mut()`].
///
/// ## Transforming the data to a different type
///
/// To transform a [`DataPayload`] to a different type backed by the same data store (cart), use
/// [`DataPayload::map_project()`] or one of its sister methods.
///
/// ## Downcasting from a trait object
///
/// If you have a [`DataPayload`]`<`[`ErasedDataStructMarker`]`>`, use [`DataPayload::downcast()`]
/// to transform it into a payload of a concrete type.
///
/// # Examples
///
/// Basic usage, using the `CowStrMarker` marker:
///
/// ```
/// use icu_provider::prelude::*;
/// use icu_provider::marker::CowStrMarker;
/// use std::borrow::Cow;
///
/// let payload = DataPayload::<CowStrMarker>::from_owned(Cow::Borrowed("Demo"));
///
/// assert_eq!("Demo", payload.get());
/// ```
pub struct DataPayload<'data, M>
where
    M: DataMarker<'data>,
{
    pub(crate) inner: DataPayloadInner<'data, M>,
}

impl<'data, M> Debug for DataPayload<'data, M>
where
    M: DataMarker<'data>,
    for<'a> &'a <M::Yokeable as Yokeable<'a>>::Output: Debug,
{
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.get().fmt(f)
    }
}

/// Cloning a DataPayload is generally a cheap operation.
/// See notes in the `Clone` impl for [`Yoke`].
impl<'data, M> Clone for DataPayload<'data, M>
where
    M: DataMarker<'data>,
    for<'a> YokeTraitHack<<M::Yokeable as Yokeable<'a>>::Output>: Clone,
{
    fn clone(&self) -> Self {
        use DataPayloadInner::*;
        let new_inner = match &self.inner {
            RcStruct(yoke) => RcStruct(yoke.clone()),
            Owned(yoke) => Owned(yoke.clone()),
            RcBuf(yoke) => RcBuf(yoke.clone()),
        };
        Self { inner: new_inner }
    }
}

impl<'data, M> PartialEq for DataPayload<'data, M>
where
    M: DataMarker<'data>,
    for<'a> YokeTraitHack<<M::Yokeable as Yokeable<'a>>::Output>: PartialEq,
{
    fn eq(&self, other: &Self) -> bool {
        YokeTraitHack(self.get()).into_ref() == YokeTraitHack(other.get()).into_ref()
    }
}

impl<'data, M> Eq for DataPayload<'data, M>
where
    M: DataMarker<'data>,
    for<'a> YokeTraitHack<<M::Yokeable as Yokeable<'a>>::Output>: Eq,
{
}

#[test]
fn test_clone_eq() {
    use crate::marker::CowStrMarker;
    let p1 = DataPayload::<CowStrMarker>::from_static_str("Demo");
    let p2 = p1.clone();
    assert_eq!(p1, p2);
}

impl<'data, M> DataPayload<'data, M>
where
    M: DataMarker<'data>,
    M::Yokeable: ZeroCopyFrom<M::Cart>,
{
    /// Convert an [`Rc`]`<`[`Cart`]`>` into a [`DataPayload`].
    ///
    /// The data need not be fully owned; this constructor creates payloads bounded by `'data`.
    ///
    /// # Examples
    ///
    /// ```
    /// use icu_provider::prelude::*;
    /// use icu_provider::hello_world::*;
    /// use std::borrow::Cow;
    /// use std::rc::Rc;
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
}

impl<'data, M> DataPayload<'data, M>
where
    M: DataMarker<'data>,
{
    /// Convert a byte buffer into a [`DataPayload`]. A function must be provided to perform the
    /// conversion. This can often be a Serde deserialization operation.
    ///
    /// This constructor creates `'static` payloads; borrowing is handled by [`Yoke`].
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
    /// This constructor creates `'static` payloads; borrowing is handled by [`Yoke`].
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

    /// Convert a fully owned (`'static`) data struct into a DataPayload.
    ///
    /// This constructor creates `'static` payloads.
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
    /// let mut payload = DataPayload::<CowStrMarker>::from_static_str("Hello");
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
    /// let mut payload = DataPayload::<CowStrMarker>::from_static_str("Hello");
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
    /// let payload = DataPayload::<CowStrMarker>::from_static_str("Demo");
    ///
    /// assert_eq!("Demo", payload.get());
    /// ```
    #[allow(clippy::needless_lifetimes)]
    pub fn get<'a>(&'a self) -> &'a <M::Yokeable as Yokeable<'a>>::Output {
        use DataPayloadInner::*;
        match &self.inner {
            RcStruct(yoke) => yoke.get(),
            Owned(yoke) => yoke.get(),
            RcBuf(yoke) => yoke.get(),
        }
    }

    /// Maps `DataPayload<M>` to `DataPayload<M2>` by projecting it with [`Yoke::project`].
    ///
    /// This is accomplished by a function that takes `M`'s data type and returns `M2`'s data
    /// type. The function takes a second argument which should be ignored. For more details,
    /// see [`Yoke::project()`].
    ///
    /// Both `M` and `M2` have the same [`DataMarker::Cart`]. This means that when using
    /// `map_project`, it is usually necessary to define a custom [`DataMarker`] type.
    ///
    /// The standard [`DataPayload::map_project()`] function moves `self` and cannot capture any
    /// data from its context. Use one of the sister methods if you need these capabilities:
    ///
    /// - [`DataPayload::map_project_cloned()`] if you don't have ownership of `self`
    /// - [`DataPayload::map_project_with_capture()`] to pass context to the mapping function
    /// - [`DataPayload::map_project_cloned_with_capture()`] to do both of these things
    ///
    /// # Example
    ///
    /// Map from `HelloWorldV1` to a `Cow<str>` containing just the message:
    ///
    /// ```
    /// use icu_provider::hello_world::*;
    /// use icu_provider::prelude::*;
    /// use std::borrow::Cow;
    ///
    /// // A custom marker type is required when using `map_project`. The Yokeable should be the
    /// // target type, and the Cart should correspond to the type being transformed.
    ///
    /// struct HelloWorldV1MessageMarker;
    /// impl<'data> DataMarker<'data> for HelloWorldV1MessageMarker {
    ///     type Yokeable = Cow<'static, str>;
    ///     type Cart = HelloWorldV1<'data>;
    /// }
    ///
    /// let p1: DataPayload<HelloWorldV1Marker> = DataPayload::from_owned(HelloWorldV1 {
    ///     message: Cow::Borrowed("Hello World")
    /// });
    ///
    /// assert_eq!("Hello World", p1.get().message);
    ///
    /// let p2: DataPayload<HelloWorldV1MessageMarker> = p1.map_project(|obj, _| {
    ///     obj.message
    /// });
    ///
    /// assert_eq!("Hello World", p2.get());
    /// ```
    pub fn map_project<M2>(
        self,
        f: for<'a> fn(
            <M::Yokeable as Yokeable<'a>>::Output,
            PhantomData<&'a ()>,
        ) -> <M2::Yokeable as Yokeable<'a>>::Output,
    ) -> DataPayload<'data, M2>
    where
        M2: DataMarker<'data, Cart = M::Cart>,
    {
        use DataPayloadInner::*;
        match self.inner {
            RcStruct(yoke) => DataPayload {
                inner: RcStruct(yoke.project(f)),
            },
            Owned(yoke) => DataPayload {
                inner: Owned(yoke.project(f)),
            },
            RcBuf(yoke) => DataPayload {
                inner: RcBuf(yoke.project(f)),
            },
        }
    }

    /// Version of [`DataPayload::map_project()`] that borrows `self` instead of moving `self`.
    pub fn map_project_cloned<'this, M2>(
        &'this self,
        f: for<'a> fn(
            &'this <M::Yokeable as Yokeable<'a>>::Output,
            PhantomData<&'a ()>,
        ) -> <M2::Yokeable as Yokeable<'a>>::Output,
    ) -> DataPayload<'data, M2>
    where
        M2: DataMarker<'data, Cart = M::Cart>,
    {
        use DataPayloadInner::*;
        match &self.inner {
            RcStruct(yoke) => DataPayload {
                inner: RcStruct(yoke.project_cloned(f)),
            },
            Owned(yoke) => DataPayload {
                inner: Owned(yoke.project_cloned(f)),
            },
            RcBuf(yoke) => DataPayload {
                inner: RcBuf(yoke.project_cloned(f)),
            },
        }
    }

    /// Version of [`DataPayload::map_project()`] that moves `self` and takes a `capture`
    /// parameter to pass additional data to `f`.
    pub fn map_project_with_capture<M2, T>(
        self,
        capture: T,
        f: for<'a> fn(
            <M::Yokeable as Yokeable<'a>>::Output,
            capture: T,
            PhantomData<&'a ()>,
        ) -> <M2::Yokeable as Yokeable<'a>>::Output,
    ) -> DataPayload<'data, M2>
    where
        M2: DataMarker<'data, Cart = M::Cart>,
    {
        use DataPayloadInner::*;
        match self.inner {
            RcStruct(yoke) => DataPayload {
                inner: RcStruct(yoke.project_with_capture(capture, f)),
            },
            Owned(yoke) => DataPayload {
                inner: Owned(yoke.project_with_capture(capture, f)),
            },
            RcBuf(yoke) => DataPayload {
                inner: RcBuf(yoke.project_with_capture(capture, f)),
            },
        }
    }

    /// Version of [`DataPayload::map_project()`] that borrows `self` and takes a `capture`
    /// parameter to pass additional data to `f`.
    pub fn map_project_cloned_with_capture<'this, M2, T>(
        &'this self,
        capture: T,
        f: for<'a> fn(
            &'this <M::Yokeable as Yokeable<'a>>::Output,
            capture: T,
            PhantomData<&'a ()>,
        ) -> <M2::Yokeable as Yokeable<'a>>::Output,
    ) -> DataPayload<'data, M2>
    where
        M2: DataMarker<'data, Cart = M::Cart>,
    {
        use DataPayloadInner::*;
        match &self.inner {
            RcStruct(yoke) => DataPayload {
                inner: RcStruct(yoke.project_cloned_with_capture(capture, f)),
            },
            Owned(yoke) => DataPayload {
                inner: Owned(yoke.project_cloned_with_capture(capture, f)),
            },
            RcBuf(yoke) => DataPayload {
                inner: RcBuf(yoke.project_cloned_with_capture(capture, f)),
            },
        }
    }
}

#[test]
fn test_map_project_docs() {
    use crate::hello_world::*;
    use crate::prelude::*;
    use std::borrow::Cow;

    struct HelloWorldV1MessageMarker;
    impl<'data> DataMarker<'data> for HelloWorldV1MessageMarker {
        type Yokeable = Cow<'static, str>;
        type Cart = HelloWorldV1<'data>;
    }
    
    let p1: DataPayload<HelloWorldV1Marker> = DataPayload::from_owned(HelloWorldV1 {
        message: Cow::Borrowed("Hello World")
    });
    
    assert_eq!("Hello World", p1.get().message);
    
    let p2: DataPayload<HelloWorldV1MessageMarker> = p1.map_project(|obj, _| {
        obj.message
    });
    
    assert_eq!("Hello World", p2.get());
}

/// A response object containing an object as payload and metadata about it.
pub struct DataResponse<'data, M>
where
    M: DataMarker<'data>,
{
    /// Metadata about the returned object.
    pub metadata: DataResponseMetadata,

    /// The object itself; None if it was not loaded.
    pub payload: Option<DataPayload<'data, M>>,
}

impl<'data, M> DataResponse<'data, M>
where
    M: DataMarker<'data>,
{
    /// Takes ownership of the underlying payload. Error if not present.
    #[inline]
    pub fn take_payload(self) -> Result<DataPayload<'data, M>, Error> {
        self.payload.ok_or(Error::MissingPayload)
    }
}

impl<'data, M> TryFrom<DataResponse<'data, M>> for DataPayload<'data, M>
where
    M: DataMarker<'data>,
{
    type Error = Error;

    fn try_from(response: DataResponse<'data, M>) -> Result<Self, Self::Error> {
        response.take_payload()
    }
}

impl<'data, M> Debug for DataResponse<'data, M>
where
    M: DataMarker<'data>,
    for<'a> &'a <M::Yokeable as Yokeable<'a>>::Output: Debug,
{
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(
            f,
            "DataResponse {{ metadata: {:?}, payload: {:?} }}",
            self.metadata, self.payload
        )
    }
}

impl<'data, M> Clone for DataResponse<'data, M>
where
    M: DataMarker<'data>,
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
    use alloc::borrow::Cow;
    let resp = DataResponse::<HelloWorldV1Marker> {
        metadata: Default::default(),
        payload: Some(DataPayload::from_owned(HelloWorldV1 {
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
pub trait DataProvider<'data, M>
where
    M: DataMarker<'data>,
{
    /// Query the provider for data, returning the result.
    ///
    /// Returns [`Ok`] if the request successfully loaded data. If data failed to load, returns an
    /// Error with more information.
    fn load_payload(&self, req: &DataRequest) -> Result<DataResponse<'data, M>, Error>;
}
