// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

#[cfg(test)]
#[cfg(feature = "serialize")]
mod test;

use crate::buf::BufferMarker;
use crate::error::{DataError, DataErrorKind};
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
    /// const FOO_BAR: ResourceKey = icu_provider::resource_key!("foo/bar@1");
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
    /// assert!(matches!(
    ///     req_no_langid.try_langid(),
    ///     Err(DataError { kind: DataErrorKind::NeedsLocale, .. })
    /// ));
    /// assert!(matches!(
    ///     req_with_langid.try_langid(),
    ///     Ok(_)
    /// ));
    /// ```
    pub fn try_langid(&self) -> Result<&LanguageIdentifier, DataError> {
        self.resource_path
            .options
            .langid
            .as_ref()
            .ok_or_else(|| DataErrorKind::NeedsLocale.with_req(self))
    }
}

/// A response object containing metadata about the returned data.
#[derive(Debug, Clone, PartialEq, Default)]
#[non_exhaustive]
pub struct DataResponseMetadata {
    /// The language of the returned data, or None if the resource key isn't localized.
    pub data_langid: Option<LanguageIdentifier>,
    /// The format of the buffer for buffer-backed data, if known (for example, JSON).
    pub buffer_format: Option<crate::buf::BufferFormat>,
}

/// A container for data payloads returned from a [`DataProvider`].
///
/// [`DataPayload`] is built on top of the [`yoke`] framework, which allows for cheap, zero-copy
/// operations on data via the use of self-references. A [`DataPayload`] may be backed by one of
/// several data stores ("carts"):
///
/// 1. Fully-owned structured data ([`DataPayload::from_owned()`])
/// 2. A reference-counted byte buffer ([`DataPayload::try_from_rc_buffer()`])
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
pub struct DataPayload<M>
where
    M: DataMarker,
{
    pub(crate) yoke: Yoke<M::Yokeable, Option<Rc<[u8]>>>,
}

impl<M> Debug for DataPayload<M>
where
    M: DataMarker,
    for<'a> &'a <M::Yokeable as Yokeable<'a>>::Output: Debug,
{
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.get().fmt(f)
    }
}

/// Cloning a DataPayload is generally a cheap operation.
/// See notes in the `Clone` impl for [`Yoke`].
///
/// # Examples
///
/// ```no_run
/// use icu_provider::prelude::*;
/// use icu_provider::hello_world::*;
///
/// let resp1: DataPayload<HelloWorldV1Marker> = todo!();
/// let resp2 = resp1.clone();
/// ```
impl<M> Clone for DataPayload<M>
where
    M: DataMarker,
    for<'a> YokeTraitHack<<M::Yokeable as Yokeable<'a>>::Output>: Clone,
{
    fn clone(&self) -> Self {
        Self {
            yoke: self.yoke.clone(),
        }
    }
}

impl<M> PartialEq for DataPayload<M>
where
    M: DataMarker,
    for<'a> YokeTraitHack<<M::Yokeable as Yokeable<'a>>::Output>: PartialEq,
{
    fn eq(&self, other: &Self) -> bool {
        YokeTraitHack(self.get()).into_ref() == YokeTraitHack(other.get()).into_ref()
    }
}

impl<M> Eq for DataPayload<M>
where
    M: DataMarker,
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

impl<M> DataPayload<M>
where
    M: DataMarker,
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
        let yoke = Yoke::try_attach_to_cart(rc_buffer, f)?.wrap_cart_in_option();
        Ok(Self { yoke })
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
    /// # #[cfg(feature = "serde_json")] {
    /// use icu_provider::prelude::*;
    /// use icu_provider::hello_world::*;
    /// use std::rc::Rc;
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
    /// # } // feature = "serde_json"
    /// ```
    #[allow(clippy::type_complexity)]
    pub fn try_from_rc_buffer_badly<E>(
        rc_buffer: Rc<[u8]>,
        f: for<'de> fn(&'de [u8]) -> Result<<M::Yokeable as Yokeable<'de>>::Output, E>,
    ) -> Result<Self, E> {
        let yoke = Yoke::try_attach_to_cart(rc_buffer, f)?.wrap_cart_in_option();
        Ok(Self { yoke })
    }

    /// Convert a byte buffer into a [`DataPayload`]. A function must be provided to perform the
    /// conversion. This can often be a Serde deserialization operation.
    ///
    /// This function is similar to [`DataPayload::try_from_rc_buffer`], but it accepts a buffer
    /// that is already yoked to an Rc buffer cart.
    ///
    /// # Examples
    ///
    /// ```
    /// # #[cfg(feature = "serde_json")] {
    /// use icu_provider::prelude::*;
    /// use icu_provider::hello_world::*;
    /// use std::rc::Rc;
    /// use icu_provider::yoke::Yoke;
    ///
    /// let json_text = "{\"message\":\"Hello World\"}";
    /// let json_rc_buffer: Rc<[u8]> = json_text.as_bytes().into();
    ///
    /// let payload = DataPayload::<HelloWorldV1Marker>::try_from_yoked_buffer(
    ///     Yoke::attach_to_zero_copy_cart(json_rc_buffer),
    ///     (),
    ///     |bytes, _, _| {
    ///         serde_json::from_slice(bytes)
    ///     }
    /// )
    /// .expect("JSON is valid");
    ///
    /// assert_eq!("Hello World", payload.get().message);
    /// # } // feature = "serde_json"
    /// ```
    #[allow(clippy::type_complexity)]
    pub fn try_from_yoked_buffer<T, E>(
        yoked_buffer: Yoke<&'static [u8], Rc<[u8]>>,
        capture: T,
        f: for<'de> fn(
            <&'static [u8] as yoke::Yokeable<'de>>::Output,
            T,
            PhantomData<&'de ()>,
        ) -> Result<<M::Yokeable as Yokeable<'de>>::Output, E>,
    ) -> Result<Self, E> {
        let yoke = yoked_buffer
            .wrap_cart_in_option()
            .try_project_with_capture(capture, f)?;
        Ok(Self { yoke })
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
            yoke: Yoke::new_owned(data),
        }
    }

    /// Convert a DataPayload that was created via [`DataPayload::from_owned()`] back into the
    /// concrete type used to construct it.
    pub fn try_unwrap_owned(self) -> Result<M::Yokeable, DataError> {
        self.yoke
            .try_into_yokeable()
            .map_err(|_| DataErrorKind::InvalidState.with_str_context("try_unwrap_owned"))
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
        self.yoke.with_mut(f)
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
    #[inline]
    #[allow(clippy::needless_lifetimes)]
    pub fn get<'a>(&'a self) -> &'a <M::Yokeable as Yokeable<'a>>::Output {
        self.yoke.get()
    }

    /// Maps `DataPayload<M>` to `DataPayload<M2>` by projecting it with [`Yoke::project`].
    ///
    /// This is accomplished by a function that takes `M`'s data type and returns `M2`'s data
    /// type. The function takes a second argument which should be ignored. For more details,
    /// see [`Yoke::project()`].
    ///
    /// The standard [`DataPayload::map_project()`] function moves `self` and cannot capture any
    /// data from its context. Use one of the sister methods if you need these capabilities:
    ///
    /// - [`DataPayload::map_project_cloned()`] if you don't have ownership of `self`
    /// - [`DataPayload::map_project_with_capture()`] to pass context to the mapping function
    /// - [`DataPayload::map_project_cloned_with_capture()`] to do both of these things
    ///
    /// # Examples
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
    /// impl DataMarker for HelloWorldV1MessageMarker {
    ///     type Yokeable = Cow<'static, str>;
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
    /// // Note: at this point, p1 has been moved.
    /// assert_eq!("Hello World", p2.get());
    /// ```
    #[allow(clippy::type_complexity)]
    pub fn map_project<M2>(
        self,
        f: for<'a> fn(
            <M::Yokeable as Yokeable<'a>>::Output,
            PhantomData<&'a ()>,
        ) -> <M2::Yokeable as Yokeable<'a>>::Output,
    ) -> DataPayload<M2>
    where
        M2: DataMarker,
    {
        DataPayload {
            yoke: self.yoke.project(f),
        }
    }

    /// Version of [`DataPayload::map_project()`] that borrows `self` instead of moving `self`.
    ///
    /// # Examples
    ///
    /// Same example as above, but this time, do not move out of `p1`:
    ///
    /// ***[#1061](https://github.com/unicode-org/icu4x/issues/1061): The following example
    /// requires Rust 1.57.***
    ///
    /// ```ignore
    /// // Same imports and definitions as above
    /// # use icu_provider::hello_world::*;
    /// # use icu_provider::prelude::*;
    /// # use std::borrow::Cow;
    /// # struct HelloWorldV1MessageMarker;
    /// # impl DataMarker for HelloWorldV1MessageMarker {
    /// #     type Yokeable = Cow<'static, str>;
    /// # }
    ///
    /// let p1: DataPayload<HelloWorldV1Marker> = DataPayload::from_owned(HelloWorldV1 {
    ///     message: Cow::Borrowed("Hello World")
    /// });
    ///
    /// assert_eq!("Hello World", p1.get().message);
    ///
    /// let p2: DataPayload<HelloWorldV1MessageMarker> = p1.map_project_cloned(|obj, _| {
    ///     obj.message.clone()
    /// });
    ///
    /// // Note: p1 is still valid.
    /// assert_eq!(p1.get().message, *p2.get());
    /// ```
    #[allow(clippy::type_complexity)]
    pub fn map_project_cloned<'this, M2>(
        &'this self,
        f: for<'a> fn(
            &'this <M::Yokeable as Yokeable<'a>>::Output,
            PhantomData<&'a ()>,
        ) -> <M2::Yokeable as Yokeable<'a>>::Output,
    ) -> DataPayload<M2>
    where
        M2: DataMarker,
    {
        DataPayload {
            yoke: self.yoke.project_cloned(f),
        }
    }

    /// Version of [`DataPayload::map_project()`] that moves `self` and takes a `capture`
    /// parameter to pass additional data to `f`.
    ///
    /// # Examples
    ///
    /// Capture a string from the context and append it to the message:
    ///
    /// ***[#1061](https://github.com/unicode-org/icu4x/issues/1061): The following example
    /// requires Rust 1.57.***
    ///
    /// ```ignore
    /// // Same imports and definitions as above
    /// # use icu_provider::hello_world::*;
    /// # use icu_provider::prelude::*;
    /// # use std::borrow::Cow;
    /// # struct HelloWorldV1MessageMarker;
    /// # impl DataMarker for HelloWorldV1MessageMarker {
    /// #     type Yokeable = Cow<'static, str>;
    /// # }
    ///
    /// let p1: DataPayload<HelloWorldV1Marker> = DataPayload::from_owned(HelloWorldV1 {
    ///     message: Cow::Borrowed("Hello World")
    /// });
    ///
    /// assert_eq!("Hello World", p1.get().message);
    ///
    /// let p2: DataPayload<HelloWorldV1MessageMarker> = p1.map_project_with_capture(
    ///     "Extra",
    ///     |mut obj, capture, _| {
    ///         obj.message.to_mut().push_str(capture);
    ///         obj.message
    ///     });
    ///
    /// assert_eq!("Hello WorldExtra", p2.get());
    /// ```
    ///
    /// Prior to Rust 1.57, pass the capture by value instead of by reference:
    ///
    /// ```
    /// // Same imports and definitions as above
    /// # use icu_provider::hello_world::*;
    /// # use icu_provider::prelude::*;
    /// # use std::borrow::Cow;
    /// # struct HelloWorldV1MessageMarker;
    /// # impl DataMarker for HelloWorldV1MessageMarker {
    /// #     type Yokeable = Cow<'static, str>;
    /// # }
    ///
    /// let p1: DataPayload<HelloWorldV1Marker> = DataPayload::from_owned(HelloWorldV1 {
    ///     message: Cow::Borrowed("Hello World")
    /// });
    ///
    /// assert_eq!("Hello World", p1.get().message);
    ///
    /// let p2: DataPayload<HelloWorldV1MessageMarker> = p1.map_project_with_capture(
    ///     "Extra".to_string(),
    ///     |mut obj, capture, _| {
    ///         obj.message.to_mut().push_str(&capture);
    ///         obj.message
    ///     });
    ///
    /// assert_eq!("Hello WorldExtra", p2.get());
    /// ```
    #[allow(clippy::type_complexity)]
    pub fn map_project_with_capture<M2, T>(
        self,
        capture: T,
        f: for<'a> fn(
            <M::Yokeable as Yokeable<'a>>::Output,
            capture: T,
            PhantomData<&'a ()>,
        ) -> <M2::Yokeable as Yokeable<'a>>::Output,
    ) -> DataPayload<M2>
    where
        M2: DataMarker,
    {
        DataPayload {
            yoke: self.yoke.project_with_capture(capture, f),
        }
    }

    /// Version of [`DataPayload::map_project()`] that borrows `self` and takes a `capture`
    /// parameter to pass additional data to `f`.
    ///
    /// # Examples
    ///
    /// Same example as above, but this time, do not move out of `p1`:
    ///
    /// ***[#1061](https://github.com/unicode-org/icu4x/issues/1061): The following example
    /// requires Rust 1.57.***
    ///
    /// ```ignore
    /// // Same imports and definitions as above
    /// # use icu_provider::hello_world::*;
    /// # use icu_provider::prelude::*;
    /// # use std::borrow::Cow;
    /// # struct HelloWorldV1MessageMarker;
    /// # impl DataMarker for HelloWorldV1MessageMarker {
    /// #     type Yokeable = Cow<'static, str>;
    /// # }
    ///
    /// let p1: DataPayload<HelloWorldV1Marker> = DataPayload::from_owned(HelloWorldV1 {
    ///     message: Cow::Borrowed("Hello World")
    /// });
    ///
    /// assert_eq!("Hello World", p1.get().message);
    ///
    /// let p2: DataPayload<HelloWorldV1MessageMarker> = p1.map_project_cloned_with_capture(
    ///     "Extra",
    ///     |obj, capture, _| {
    ///         let mut message = obj.message.clone();
    ///         message.to_mut().push_str(capture);
    ///         message
    ///     });
    ///
    /// // Note: p1 is still valid, but the values no longer equal.
    /// assert_ne!(p1.get().message, *p2.get());
    /// assert_eq!("Hello WorldExtra", p2.get());
    /// ```
    #[allow(clippy::type_complexity)]
    pub fn map_project_cloned_with_capture<'this, M2, T>(
        &'this self,
        capture: T,
        f: for<'a> fn(
            &'this <M::Yokeable as Yokeable<'a>>::Output,
            capture: T,
            PhantomData<&'a ()>,
        ) -> <M2::Yokeable as Yokeable<'a>>::Output,
    ) -> DataPayload<M2>
    where
        M2: DataMarker,
    {
        DataPayload {
            yoke: self.yoke.project_cloned_with_capture(capture, f),
        }
    }

    /// Version of [`DataPayload::map_project()`] that moves `self`, takes a `capture`
    /// parameter to pass additional data to `f`, and bubbles up an error from `f`.
    ///
    /// # Examples
    ///
    /// Same example as above, but bubble up an error:
    ///
    /// ***[#1061](https://github.com/unicode-org/icu4x/issues/1061): The following example
    /// requires Rust 1.57.***
    ///
    /// ```ignore
    /// // Same imports and definitions as above
    /// # use icu_provider::hello_world::*;
    /// # use icu_provider::prelude::*;
    /// # use std::borrow::Cow;
    /// # struct HelloWorldV1MessageMarker;
    /// # impl DataMarker for HelloWorldV1MessageMarker {
    /// #     type Yokeable = Cow<'static, str>;
    /// # }
    ///
    /// let p1: DataPayload<HelloWorldV1Marker> = DataPayload::from_owned(HelloWorldV1 {
    ///     message: Cow::Borrowed("Hello World")
    /// });
    ///
    /// assert_eq!("Hello World", p1.get().message);
    ///
    /// let p2: DataPayload<HelloWorldV1MessageMarker> = p1.try_map_project_with_capture(
    ///     "Extra",
    ///     |mut obj, capture, _| {
    ///         if obj.message.is_empty() {
    ///             return Err("Example error");
    ///         }
    ///         obj.message.to_mut().push_str(&capture);
    ///         Ok(obj.message)
    ///     })?;
    ///
    /// assert_eq!("Hello WorldExtra", p2.get());
    /// # Ok::<(), &'static str>(())
    /// ```
    ///
    /// Prior to Rust 1.57, pass the capture by value instead of by reference:
    ///
    /// ```
    /// // Same imports and definitions as above
    /// # use icu_provider::hello_world::*;
    /// # use icu_provider::prelude::*;
    /// # use std::borrow::Cow;
    /// # struct HelloWorldV1MessageMarker;
    /// # impl DataMarker for HelloWorldV1MessageMarker {
    /// #     type Yokeable = Cow<'static, str>;
    /// # }
    ///
    /// let p1: DataPayload<HelloWorldV1Marker> = DataPayload::from_owned(HelloWorldV1 {
    ///     message: Cow::Borrowed("Hello World")
    /// });
    ///
    /// assert_eq!("Hello World", p1.get().message);
    ///
    /// let p2: DataPayload<HelloWorldV1MessageMarker> = p1.try_map_project_with_capture(
    ///     "Extra".to_string(),
    ///     |mut obj, capture, _| {
    ///         if obj.message.is_empty() {
    ///             return Err(())
    ///         }
    ///         obj.message.to_mut().push_str(&capture);
    ///         Ok(obj.message)
    ///     })?;
    ///
    /// assert_eq!("Hello WorldExtra", p2.get());
    /// # Ok::<(), ()>(())
    /// ```
    #[allow(clippy::type_complexity)]
    pub fn try_map_project_with_capture<M2, T, E>(
        self,
        capture: T,
        f: for<'a> fn(
            <M::Yokeable as Yokeable<'a>>::Output,
            capture: T,
            PhantomData<&'a ()>,
        ) -> Result<<M2::Yokeable as Yokeable<'a>>::Output, E>,
    ) -> Result<DataPayload<M2>, E>
    where
        M2: DataMarker,
    {
        Ok(DataPayload {
            yoke: self.yoke.try_project_with_capture(capture, f)?,
        })
    }

    /// Version of [`DataPayload::map_project()`] that borrows `self`, takes a `capture`
    /// parameter to pass additional data to `f`, and bubbles up an error from `f`.
    ///
    /// # Examples
    ///
    /// Same example as above, but bubble up an error:
    ///
    /// ***[#1061](https://github.com/unicode-org/icu4x/issues/1061): The following example
    /// requires Rust 1.57.***
    ///
    /// ```ignore
    /// // Same imports and definitions as above
    /// # use icu_provider::hello_world::*;
    /// # use icu_provider::prelude::*;
    /// # use std::borrow::Cow;
    /// # struct HelloWorldV1MessageMarker;
    /// # impl DataMarker for HelloWorldV1MessageMarker {
    /// #     type Yokeable = Cow<'static, str>;
    /// # }
    ///
    /// let p1: DataPayload<HelloWorldV1Marker> = DataPayload::from_owned(HelloWorldV1 {
    ///     message: Cow::Borrowed("Hello World")
    /// });
    ///
    /// assert_eq!("Hello World", p1.get().message);
    ///
    /// let p2: DataPayload<HelloWorldV1MessageMarker> = p1.try_map_project_cloned_with_capture(
    ///     "Extra",
    ///     |obj, capture, _| {
    ///         if obj.message.is_empty() {
    ///             return Err("Example error");
    ///         }
    ///         let mut message = obj.message.clone();
    ///         message.to_mut().push_str(capture);
    ///         Ok(message)
    ///     })?;
    ///
    /// // Note: p1 is still valid, but the values no longer equal.
    /// assert_ne!(p1.get().message, *p2.get());
    /// assert_eq!("Hello WorldExtra", p2.get());
    /// # Ok::<(), &'static str>(())
    /// ```
    #[allow(clippy::type_complexity)]
    pub fn try_map_project_cloned_with_capture<'this, M2, T, E>(
        &'this self,
        capture: T,
        f: for<'a> fn(
            &'this <M::Yokeable as Yokeable<'a>>::Output,
            capture: T,
            PhantomData<&'a ()>,
        ) -> Result<<M2::Yokeable as Yokeable<'a>>::Output, E>,
    ) -> Result<DataPayload<M2>, E>
    where
        M2: DataMarker,
    {
        Ok(DataPayload {
            yoke: self.yoke.try_project_cloned_with_capture(capture, f)?,
        })
    }
}

impl DataPayload<BufferMarker> {
    /// Converts a reference-counted byte buffer into a `DataPayload<BufferMarker>`.
    pub fn from_rc_buffer(buffer: Rc<[u8]>) -> Self {
        Self {
            yoke: Yoke::attach_to_zero_copy_cart(buffer).wrap_cart_in_option(),
        }
    }

    /// Converts a yoked byte buffer into a `DataPayload<BufferMarker>`.
    pub fn from_yoked_buffer(yoked_buffer: Yoke<&'static [u8], Rc<[u8]>>) -> Self {
        Self {
            yoke: yoked_buffer.wrap_cart_in_option(),
        }
    }

    /// Converts a static byte buffer into a `DataPayload<BufferMarker>`.
    pub fn from_static_buffer(buffer: &'static [u8]) -> Self {
        Self {
            yoke: Yoke::new_owned(buffer),
        }
    }
}

impl<M> Default for DataPayload<M>
where
    M: DataMarker,
    M::Yokeable: Default,
{
    fn default() -> Self {
        Self::from_owned(Default::default())
    }
}

/// A response object containing an object as payload and metadata about it.
pub struct DataResponse<M>
where
    M: DataMarker,
{
    /// Metadata about the returned object.
    pub metadata: DataResponseMetadata,

    /// The object itself; None if it was not loaded.
    pub payload: Option<DataPayload<M>>,
}

impl<M> DataResponse<M>
where
    M: DataMarker,
{
    /// Takes ownership of the underlying payload. Error if not present.
    ///
    /// To take the metadata, too, use [`Self::take_metadata_and_payload()`].
    #[inline]
    pub fn take_payload(self) -> Result<DataPayload<M>, DataError> {
        Ok(self.take_metadata_and_payload()?.1)
    }

    /// Takes ownership of the underlying metadata and payload. Error if payload is not present.
    #[inline]
    pub fn take_metadata_and_payload(
        self,
    ) -> Result<(DataResponseMetadata, DataPayload<M>), DataError> {
        Ok((
            self.metadata,
            self.payload
                .ok_or_else(|| DataErrorKind::MissingPayload.with_type_context::<M>())?,
        ))
    }
}

impl<M> TryFrom<DataResponse<M>> for DataPayload<M>
where
    M: DataMarker,
{
    type Error = DataError;

    fn try_from(response: DataResponse<M>) -> Result<Self, Self::Error> {
        response.take_payload()
    }
}

impl<M> Debug for DataResponse<M>
where
    M: DataMarker,
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

/// Cloning a DataResponse is generally a cheap operation.
/// See notes in the `Clone` impl for [`Yoke`].
///
/// # Examples
///
/// ```no_run
/// use icu_provider::prelude::*;
/// use icu_provider::hello_world::*;
///
/// let resp1: DataResponse<HelloWorldV1Marker> = todo!();
/// let resp2 = resp1.clone();
/// ```
impl<M> Clone for DataResponse<M>
where
    M: DataMarker,
    for<'a> YokeTraitHack<<M::Yokeable as Yokeable<'a>>::Output>: Clone,
{
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
    assert_eq!("DataResponse { metadata: DataResponseMetadata { data_langid: None, buffer_format: None }, payload: Some(HelloWorldV1 { message: \"foo\" }) }", format!("{:?}", resp));
}

/// A generic data provider that loads a payload of a specific type.
///
/// See examples on some of the concrete implementations:
///
/// - [`HelloWorldProvider`](crate::hello_world::HelloWorldProvider)
/// - [`AnyPayloadProvider`](crate::struct_provider::AnyPayloadProvider)
/// - [`InvariantDataProvider`](crate::inv::InvariantDataProvider)
pub trait DataProvider<M>
where
    M: DataMarker,
{
    /// Query the provider for data, returning the result.
    ///
    /// Returns [`Ok`] if the request successfully loaded data. If data failed to load, returns an
    /// Error with more information.
    fn load_payload(&self, req: &DataRequest) -> Result<DataResponse<M>, DataError>;
}
