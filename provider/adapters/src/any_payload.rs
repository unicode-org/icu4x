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
/// [`AnyPayloadProvider`] implements [`AnyProvider`], so it can be used in
/// `*_with_any_provider` constructors across ICU4X.
///
/// # Examples
///
/// ```
/// use icu_provider::hello_world::*;
/// use icu_provider::prelude::*;
/// use icu_provider_adapters::any_payload::AnyPayloadProvider;
/// use std::borrow::Cow;
/// use writeable::assert_writeable_eq;
///
/// let provider =
///     AnyPayloadProvider::from_static::<HelloWorldV1Marker>(&HelloWorldV1 {
///         message: Cow::Borrowed("custom hello world"),
///     });
///
/// // Check that it works:
/// let formatter = HelloWorldFormatter::try_new_with_any_provider(
///     &provider,
///     &Default::default(),
/// )
/// .expect("marker matches");
/// assert_writeable_eq!(formatter.format(), "custom hello world");
///
/// # struct DummyMarker;
/// # impl DynamicDataMarker for DummyMarker {
/// #     type DataStruct = <HelloWorldV1Marker as DynamicDataMarker>::DataStruct;
/// # }
/// # impl DataMarker for DummyMarker {
/// #     const INFO: DataMarkerInfo = DataMarkerInfo::from_path(icu_provider::marker::data_marker_path!("dummy@1"));
/// # }
/// // Requests for invalid markers get MissingDataMarker
/// assert!(matches!(
///     provider.load_any(DummyMarker::INFO, Default::default()),
///     Err(DataError {
///         kind: DataErrorKind::MarkerNotFound,
///         ..
///     })
/// ))
/// ```
#[derive(Debug)]
#[allow(clippy::exhaustive_structs)] // this type is stable
pub struct AnyPayloadProvider {
    /// The [`DataMarkerInfo`] for which to provide data. All others will receive a
    /// [`DataErrorKind::MarkerNotFound`].
    marker: DataMarkerInfo,
    /// The [`AnyPayload`] to return on matching requests.
    data: AnyPayload,
}

impl AnyPayloadProvider {
    /// Creates an `AnyPayloadProvider` with an owned (allocated) payload of the given data.
    pub fn from_owned<M: DataMarker>(data: M::DataStruct) -> Self
    where
        M::DataStruct: icu_provider::any::MaybeSendSync,
    {
        Self::from_payload::<M>(DataPayload::from_owned(data))
    }

    /// Creates an `AnyPayloadProvider` with a statically borrowed payload of the given data.
    pub fn from_static<M: DataMarker>(data: &'static M::DataStruct) -> Self {
        AnyPayloadProvider {
            marker: M::INFO,
            data: AnyPayload::from_static_ref(data),
        }
    }

    /// Creates an `AnyPayloadProvider` from an existing [`DataPayload`].
    pub fn from_payload<M: DataMarker>(payload: DataPayload<M>) -> Self
    where
        M::DataStruct: icu_provider::any::MaybeSendSync,
    {
        AnyPayloadProvider {
            marker: M::INFO,
            data: payload.wrap_into_any_payload(),
        }
    }

    /// Creates an `AnyPayloadProvider` from an existing [`AnyPayload`].
    pub fn from_any_payload<M: DataMarker>(payload: AnyPayload) -> Self {
        AnyPayloadProvider {
            marker: M::INFO,
            data: payload,
        }
    }

    /// Creates an `AnyPayloadProvider` with the default (allocated) version of the data struct.
    pub fn new_default<M: DataMarker>() -> Self
    where
        M::DataStruct: Default,
        M::DataStruct: icu_provider::any::MaybeSendSync,
    {
        Self::from_owned::<M>(M::DataStruct::default())
    }
}

impl AnyProvider for AnyPayloadProvider {
    fn load_any(&self, marker: DataMarkerInfo, _: DataRequest) -> Result<AnyResponse, DataError> {
        marker.match_marker(self.marker)?;
        Ok(AnyResponse {
            metadata: DataResponseMetadata::default(),
            payload: self.data.clone(),
        })
    }
}

impl<M> DataProvider<M> for AnyPayloadProvider
where
    M: DataMarker,
    for<'a> YokeTraitHack<<M::DataStruct as Yokeable<'a>>::Output>: Clone,
    M::DataStruct: ZeroFrom<'static, M::DataStruct>,
    M::DataStruct: icu_provider::any::MaybeSendSync,
{
    fn load(&self, req: DataRequest) -> Result<DataResponse<M>, DataError> {
        self.as_downcasting().load(req)
    }
}
