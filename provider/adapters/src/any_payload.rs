// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

//! Data provider always serving the same struct.

use icu_provider::prelude::*;
use yoke::trait_hack::YokeTraitHack;
use yoke::Yokeable;
use zerofrom::ZeroFrom;

/// A data provider that returns clones of a fixed type-erased payload.
///
/// # Examples
///
/// ```
/// use icu_provider::hello_world::*;
/// use icu_provider::prelude::*;
/// use icu_provider_adapters::any_payload::AnyPayloadProvider;
/// use std::borrow::Cow;
///
/// let provider =
///     AnyPayloadProvider::new_static::<HelloWorldV1Marker>(&HelloWorldV1 {
///         message: Cow::Borrowed("hello world"),
///     });
///
/// let payload: DataPayload<HelloWorldV1Marker> = provider
///     .load_any(HelloWorldV1Marker::KEY, Default::default())
///     .expect("Load should succeed")
///     .downcast()
///     .expect("Types should match")
///     .take_payload()
///     .expect("Data should be present");
///
/// assert_eq!(payload.get().message, "hello world");
/// ```
#[allow(clippy::exhaustive_structs)] // this type is stable
pub struct AnyPayloadProvider {
    pub key: DataKey,
    pub data: AnyPayload,
}

impl AnyPayloadProvider {
    /// Creates an `AnyPayloadProvider` with an owned (allocated) payload of the given data.
    pub fn new_owned<M: KeyedDataMarker + 'static>(data: M::Yokeable) -> Self {
        AnyPayloadProvider {
            key: M::KEY,
            data: AnyPayload::from_rc_payload::<M>(alloc::rc::Rc::from(DataPayload::from_owned(
                data,
            ))),
        }
    }

    /// Creates an `AnyPayloadProvider` with a statically borrowed payload of the given data.
    pub fn new_static<M: KeyedDataMarker>(data: &'static M::Yokeable) -> Self {
        AnyPayloadProvider {
            key: M::KEY,
            data: AnyPayload::from_static_ref(data),
        }
    }

    /// Creates an `AnyPayloadProvider` with the default (allocated) version of the data struct.
    pub fn new_default<M: KeyedDataMarker + 'static>() -> Self
    where
        M::Yokeable: Default,
    {
        Self::new_owned::<M>(M::Yokeable::default())
    }
}

impl AnyProvider for AnyPayloadProvider {
    fn load_any(&self, key: DataKey, _: DataRequest) -> Result<AnyResponse, DataError> {
        key.match_key(self.key)?;
        Ok(AnyResponse {
            metadata: DataResponseMetadata::default(),
            payload: Some(self.data.clone()),
        })
    }
}

impl<M> DataProvider<M> for AnyPayloadProvider
where
    M: KeyedDataMarker + 'static,
    for<'a> YokeTraitHack<<M::Yokeable as Yokeable<'a>>::Output>: Clone,
    M::Yokeable: ZeroFrom<'static, M::Yokeable>,
{
    fn load(&self, req: DataRequest) -> Result<DataResponse<M>, DataError> {
        self.as_downcasting().load(req)
    }
}
