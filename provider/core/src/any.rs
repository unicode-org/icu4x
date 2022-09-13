// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

//! Traits for data providers that produce `Any` objects.

use crate::prelude::*;
use crate::response::RcWrap;
use crate::response::RcWrapBounds;
use core::any::Any;
use core::convert::TryFrom;
use core::convert::TryInto;
use yoke::trait_hack::YokeTraitHack;
use yoke::Yokeable;
use zerofrom::ZeroFrom;

/// Representations of the `Any` trait object.
///
/// **Important Note:** The types enclosed by `StructRef` and `PayloadRc` are NOT the same!
/// The first refers to the struct itself, whereas the second refers to a `DataPayload`.
#[derive(Debug, Clone)]
enum AnyPayloadInner {
    /// A reference to `M::Yokeable`
    StructRef(&'static dyn Any),
    /// A boxed `DataPayload<M>`.
    ///
    /// Note: This needs to be an `RcWrap`, not a `Box`, so that `AnyPayload` is cloneable.
    /// If an `AnyPayload` is cloned, the actual cloning of the data is delayed until
    /// `downcast()` is invoked (at which point we have the concrete type).

    #[cfg(not(feature = "sync"))]
    PayloadRc(RcWrap<dyn Any>),

    #[cfg(feature = "sync")]
    PayloadRc(RcWrap<dyn Any + Send + Sync>),
}

/// A type-erased data payload.
///
/// The only useful method on this type is [`AnyPayload::downcast()`], which transforms this into
/// a normal `DataPayload` which you can subsequently access or mutate.
///
/// As with `DataPayload`, cloning is designed to be cheap.
#[derive(Debug, Clone, Yokeable)]
pub struct AnyPayload {
    inner: AnyPayloadInner,
    type_name: &'static str,
}

/// The [`DataMarker`] marker type for [`AnyPayload`].
#[allow(clippy::exhaustive_structs)] // marker type
pub struct AnyMarker;

impl DataMarker for AnyMarker {
    type Yokeable = AnyPayload;
}

impl<M> crate::dynutil::UpcastDataPayload<M> for AnyMarker
where
    M: DataMarker + 'static,
    M::Yokeable: RcWrapBounds,
{
    #[inline]
    fn upcast(other: DataPayload<M>) -> DataPayload<AnyMarker> {
        DataPayload::from_owned(other.wrap_into_any_payload())
    }
}

impl AnyPayload {
    /// Transforms a type-erased `AnyPayload` into a concrete `DataPayload<M>`.
    ///
    /// Because it is expected that the call site knows the identity of the AnyPayload (e.g., from
    /// the data request), this function returns a `DataError` if the generic type does not match
    /// the type stored in the `AnyPayload`.
    pub fn downcast<M>(self) -> Result<DataPayload<M>, DataError>
    where
        M: DataMarker + 'static,
        // For the StructRef case:
        M::Yokeable: ZeroFrom<'static, M::Yokeable>,
        // For the PayloadRc case:
        M::Yokeable: RcWrapBounds,
        for<'a> YokeTraitHack<<M::Yokeable as Yokeable<'a>>::Output>: Clone,
    {
        use AnyPayloadInner::*;
        let type_name = self.type_name;
        match self.inner {
            StructRef(any_ref) => {
                let down_ref: &'static M::Yokeable = any_ref
                    .downcast_ref()
                    .ok_or_else(|| DataError::for_type::<M>().with_str_context(type_name))?;
                Ok(DataPayload::from_owned(M::Yokeable::zero_from(down_ref)))
            }
            PayloadRc(any_rc) => {
                let down_rc: RcWrap<DataPayload<M>> = any_rc
                    .downcast()
                    .map_err(|_| DataError::for_type::<M>().with_str_context(type_name))?;
                Ok(RcWrap::try_unwrap(down_rc).unwrap_or_else(|down_rc| (*down_rc).clone()))
            }
        }
    }

    /// Creates an `AnyPayload` from a static reference to a data struct.
    ///
    /// # Examples
    ///
    /// ```
    /// use icu_provider::hello_world::*;
    /// use icu_provider::prelude::*;
    /// use std::borrow::Cow;
    ///
    /// const HELLO_DATA: HelloWorldV1<'static> = HelloWorldV1 {
    ///     message: Cow::Borrowed("Custom Hello World"),
    /// };
    ///
    /// let any_payload = AnyPayload::from_static_ref(&HELLO_DATA);
    ///
    /// let payload: DataPayload<HelloWorldV1Marker> = any_payload.downcast().expect("TypeId matches");
    /// assert_eq!("Custom Hello World", payload.get().message);
    /// ```
    pub fn from_static_ref<Y>(static_ref: &'static Y) -> Self
    where
        Y: for<'a> Yokeable<'a>,
    {
        AnyPayload {
            inner: AnyPayloadInner::StructRef(static_ref),
            // Note: This records the Yokeable type rather than the DataMarker type,
            // but that is okay since this is only for debugging
            type_name: core::any::type_name::<Y>(),
        }
    }

    /// Creates an `AnyPayload` from a [`DataPayload`] stored in an [`RcWrap`].
    ///
    /// # Examples
    ///
    /// ```
    /// use icu_provider::hello_world::*;
    /// use icu_provider::prelude::*;
    /// use icu_provider::RcWrap;
    /// use std::borrow::Cow;
    ///
    /// let payload: DataPayload<HelloWorldV1Marker> = DataPayload::from_owned(HelloWorldV1 {
    ///     message: Cow::Borrowed("Custom Hello World"),
    /// });
    /// let rc_payload = RcWrap::from(payload);
    ///
    /// let any_payload = AnyPayload::from_rcwrap_payload(rc_payload);
    ///
    /// let payload: DataPayload<HelloWorldV1Marker> = any_payload.downcast().expect("TypeId matches");
    /// assert_eq!("Custom Hello World", payload.get().message);
    /// ```
    pub fn from_rcwrap_payload<M>(rc_payload: RcWrap<DataPayload<M>>) -> Self
    where
        M: DataMarker + 'static,
        M::Yokeable: RcWrapBounds,
    {
        AnyPayload {
            inner: AnyPayloadInner::PayloadRc(rc_payload.into_dyn_any()),
            type_name: core::any::type_name::<M>(),
        }
    }
}

impl<M> DataPayload<M>
where
    M: DataMarker + 'static,
    M::Yokeable: RcWrapBounds,
{
    /// Wraps this DataPayload in an `Rc` and returns it as an `AnyPayload`.
    ///
    /// # Examples
    ///
    /// ```
    /// use icu_provider::hello_world::*;
    /// use icu_provider::prelude::*;
    /// use std::borrow::Cow;
    /// use std::rc::Rc;
    ///
    /// let payload: DataPayload<HelloWorldV1Marker> = DataPayload::from_owned(HelloWorldV1 {
    ///     message: Cow::Borrowed("Custom Hello World"),
    /// });
    ///
    /// let any_payload = payload.wrap_into_any_payload();
    ///
    /// let payload: DataPayload<HelloWorldV1Marker> = any_payload.downcast().expect("TypeId matches");
    /// assert_eq!("Custom Hello World", payload.get().message);
    /// ```
    pub fn wrap_into_any_payload(self) -> AnyPayload {
        AnyPayload {
            inner: AnyPayloadInner::PayloadRc(RcWrap::from(self).into_dyn_any()),
            type_name: core::any::type_name::<M>(),
        }
    }
}

impl DataPayload<AnyMarker> {
    /// Transforms a type-erased `DataPayload<AnyMarker>` into a concrete `DataPayload<M>`.
    #[inline]
    pub fn downcast<M>(self) -> Result<DataPayload<M>, DataError>
    where
        M: DataMarker + 'static,
        for<'a> YokeTraitHack<<M::Yokeable as Yokeable<'a>>::Output>: Clone,
        M::Yokeable: ZeroFrom<'static, M::Yokeable>,
        M::Yokeable: RcWrapBounds,
    {
        self.try_unwrap_owned()?.downcast()
    }
}

/// A [`DataResponse`] for type-erased values.
///
/// Convertible to and from `DataResponse<AnyMarker>`.
#[allow(clippy::exhaustive_structs)] // this type is stable (the metadata is allowed to grow)
pub struct AnyResponse {
    /// Metadata about the returned object.
    pub metadata: DataResponseMetadata,

    /// The object itself; None if it was not loaded.
    pub payload: Option<AnyPayload>,
}

impl TryFrom<DataResponse<AnyMarker>> for AnyResponse {
    type Error = DataError;
    #[inline]
    fn try_from(other: DataResponse<AnyMarker>) -> Result<Self, Self::Error> {
        Ok(Self {
            metadata: other.metadata,
            payload: other.payload.map(|p| p.try_unwrap_owned()).transpose()?,
        })
    }
}

impl From<AnyResponse> for DataResponse<AnyMarker> {
    #[inline]
    fn from(other: AnyResponse) -> Self {
        Self {
            metadata: other.metadata,
            payload: other.payload.map(DataPayload::from_owned),
        }
    }
}

impl AnyResponse {
    /// Transforms a type-erased `DataResponse<AnyMarker>` into a concrete `DataResponse<M>`.
    #[inline]
    pub fn downcast<M>(self) -> Result<DataResponse<M>, DataError>
    where
        M: DataMarker + 'static,
        for<'a> YokeTraitHack<<M::Yokeable as Yokeable<'a>>::Output>: Clone,
        M::Yokeable: ZeroFrom<'static, M::Yokeable>,
        M::Yokeable: RcWrapBounds,
    {
        Ok(DataResponse {
            metadata: self.metadata,
            payload: self.payload.map(|p| p.downcast()).transpose()?,
        })
    }
}

/// An object-safe data provider that returns data structs cast to `dyn Any` trait objects.
///
/// # Examples
///
/// ```
/// use icu_provider::hello_world::*;
/// use icu_provider::prelude::*;
/// use std::borrow::Cow;
///
/// let any_response = HelloWorldProvider
///     .as_any_provider()
///     .load_any(
///         HelloWorldV1Marker::KEY,
///         DataRequest {
///             locale: &icu_locid::locale!("de").into(),
///             metadata: Default::default(),
///         },
///     )
///     .expect("Load should succeed");
///
/// // Downcast to something useful
/// let response: DataResponse<HelloWorldV1Marker> =
///     any_response.downcast().expect("Types match");
///
/// let payload = response.take_payload().expect("Data should be present");
///
/// assert_eq!(payload.get().message, "Hallo Welt");
/// ```
pub trait AnyProvider {
    /// Loads an [`AnyPayload`] according to the key and request.
    fn load_any(&self, key: DataKey, req: DataRequest) -> Result<AnyResponse, DataError>;
}

impl<T: AnyProvider + ?Sized> AnyProvider for alloc::boxed::Box<T> {
    fn load_any(&self, key: DataKey, req: DataRequest) -> Result<AnyResponse, DataError> {
        (**self).load_any(key, req)
    }
}

/// A wrapper over `DynamicDataProvider<AnyMarker>` that implements `AnyProvider`
#[allow(clippy::exhaustive_structs)] // newtype
pub struct DynamicDataProviderAnyMarkerWrap<'a, P: ?Sized>(pub &'a P);

/// Blanket-implemented trait adding the [`Self::as_any_provider()`] function.
pub trait AsDynamicDataProviderAnyMarkerWrap {
    /// Returns an object implementing `AnyProvider` when called on `DynamicDataProvider<AnyMarker>`
    fn as_any_provider(&self) -> DynamicDataProviderAnyMarkerWrap<Self>;
}

impl<P> AsDynamicDataProviderAnyMarkerWrap for P
where
    P: DynamicDataProvider<AnyMarker>,
{
    #[inline]
    fn as_any_provider(&self) -> DynamicDataProviderAnyMarkerWrap<P> {
        DynamicDataProviderAnyMarkerWrap(self)
    }
}

impl<P> AnyProvider for DynamicDataProviderAnyMarkerWrap<'_, P>
where
    P: DynamicDataProvider<AnyMarker> + ?Sized,
{
    #[inline]
    fn load_any(&self, key: DataKey, req: DataRequest) -> Result<AnyResponse, DataError> {
        self.0.load_data(key, req)?.try_into()
    }
}

/// A wrapper over `AnyProvider` that implements `DynamicDataProvider<M>` via downcasting
#[allow(clippy::exhaustive_structs)] // newtype
pub struct DowncastingAnyProvider<'a, P: ?Sized>(pub &'a P);

/// Blanket-implemented trait adding the [`Self::as_downcasting()`] function.
pub trait AsDowncastingAnyProvider {
    /// Returns an object implementing `DynamicDataProvider<M>` when called on `AnyProvider`
    fn as_downcasting(&self) -> DowncastingAnyProvider<Self>;
}

impl<P> AsDowncastingAnyProvider for P
where
    P: AnyProvider + ?Sized,
{
    #[inline]
    fn as_downcasting(&self) -> DowncastingAnyProvider<P> {
        DowncastingAnyProvider(self)
    }
}

impl<M, P> DataProvider<M> for DowncastingAnyProvider<'_, P>
where
    P: AnyProvider + ?Sized,
    M: KeyedDataMarker + 'static,
    for<'a> YokeTraitHack<<M::Yokeable as Yokeable<'a>>::Output>: Clone,
    M::Yokeable: ZeroFrom<'static, M::Yokeable>,
    M::Yokeable: RcWrapBounds,
{
    #[inline]
    fn load(&self, req: DataRequest) -> Result<DataResponse<M>, DataError> {
        self.0.load_any(M::KEY, req)?.downcast()
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::hello_world::*;
    use alloc::borrow::Cow;

    const CONST_DATA: HelloWorldV1<'static> = HelloWorldV1 {
        message: Cow::Borrowed("Custom Hello World"),
    };

    #[test]
    fn test_debug() {
        let payload: DataPayload<HelloWorldV1Marker> = DataPayload::from_owned(HelloWorldV1 {
            message: Cow::Borrowed("Custom Hello World"),
        });

        let any_payload = payload.wrap_into_any_payload();
        assert_eq!(
            "AnyPayload { inner: PayloadRc(RcWrap(Any { .. })), type_name: \"icu_provider::hello_world::HelloWorldV1Marker\" }",
            format!("{:?}", any_payload)
        );

        struct WrongMarker;

        impl DataMarker for WrongMarker {
            type Yokeable = u8;
        }

        let err = any_payload.downcast::<WrongMarker>().unwrap_err();
        assert_eq!(
            "ICU4X data error: Mismatched types: tried to downcast with icu_provider::any::test::test_debug::WrongMarker, but actual type is different: icu_provider::hello_world::HelloWorldV1Marker",
            format!("{}", err)
        );
    }

    #[test]
    fn test_non_owned_any_marker() {
        // This test demonstrates a code path that can trigger the InvalidState error kind.
        let payload_result: Result<DataPayload<AnyMarker>, core::convert::Infallible> =
            DataPayload::try_from_rc_buffer_badly((&[] as &[u8]).into(), |_| {
                Ok(AnyPayload::from_static_ref(&CONST_DATA))
            });
        let err = payload_result
            .expect("infallible")
            .downcast::<HelloWorldV1Marker>()
            .unwrap_err();
        assert!(matches!(
            err,
            DataError {
                kind: DataErrorKind::InvalidState,
                ..
            }
        ));
    }
}
