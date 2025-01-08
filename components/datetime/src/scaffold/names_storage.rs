// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use crate::error::ErrorField;
use crate::pattern::{YearNameLength, MonthNameLength, WeekdayNameLength, DayPeriodNameLength, PatternLoadError};
use crate::provider::neo::*;
use crate::provider::time_zones::tz;
use core::fmt;
use icu_provider::prelude::*;
use yoke::Yokeable;

use super::UnstableSealed;

/// Trait for a type that owns datetime names data, usually in the form of data payloads.
///
/// This trait allows for types that contain data for some but not all types of datetime names,
/// allowing for reduced stack size. For example, a type could contain year and month names but
/// not weekday, day period, or time zone names.
#[allow(missing_docs)]
pub trait DateTimeNamesMarker: UnstableSealed {
    type YearNames: NamesContainer<YearNamesV1Marker, YearNameLength>;
    type MonthNames: NamesContainer<MonthNamesV1Marker, MonthNameLength>;
    type WeekdayNames: NamesContainer<WeekdayNamesV1Marker, WeekdayNameLength>;
    type DayPeriodNames: NamesContainer<DayPeriodNamesV1Marker, DayPeriodNameLength>;
    type ZoneEssentials: NamesContainer<tz::EssentialsV1Marker, ()>;
    type ZoneLocations: NamesContainer<tz::LocationsV1Marker, ()>;
    type ZoneGenericLong: NamesContainer<tz::MzGenericLongV1Marker, ()>;
    type ZoneGenericShort: NamesContainer<tz::MzGenericShortV1Marker, ()>;
    type ZoneSpecificLong: NamesContainer<tz::MzSpecificLongV1Marker, ()>;
    type ZoneSpecificShort: NamesContainer<tz::MzSpecificShortV1Marker, ()>;
    type MetazoneLookup: NamesContainer<tz::MzPeriodV1Marker, ()>;
}

/// Trait that associates a container for a payload parameterized by the given variables.
#[allow(missing_docs)]
pub trait NamesContainer<M: DynamicDataMarker, Variables>: UnstableSealed
where
    Variables: PartialEq + Copy + fmt::Debug,
{
    type Container: MaybePayload<M, Variables> + fmt::Debug;
}

impl<M: DynamicDataMarker, Variables> NamesContainer<M, Variables> for ()
where
    Variables: PartialEq + Copy + fmt::Debug,
{
    type Container = ();
}

macro_rules! impl_holder_trait {
    ($marker:path) => {
        impl UnstableSealed for $marker {}
        impl<Variables> NamesContainer<$marker, Variables> for $marker
        where
            Variables: PartialEq + Copy + fmt::Debug,
        {
            type Container = DataPayloadWithVariables<$marker, Variables>;
        }
    };
}

impl_holder_trait!(YearNamesV1Marker);
impl_holder_trait!(MonthNamesV1Marker);
impl_holder_trait!(WeekdayNamesV1Marker);
impl_holder_trait!(DayPeriodNamesV1Marker);
impl_holder_trait!(tz::EssentialsV1Marker);
impl_holder_trait!(tz::LocationsV1Marker);
impl_holder_trait!(tz::MzGenericLongV1Marker);
impl_holder_trait!(tz::MzGenericShortV1Marker);
impl_holder_trait!(tz::MzSpecificLongV1Marker);
impl_holder_trait!(tz::MzSpecificShortV1Marker);
impl_holder_trait!(tz::MzPeriodV1Marker);

/// An error returned by [`MaybePayload`].
#[allow(missing_docs)]
#[derive(Debug)]
#[non_exhaustive]
pub enum MaybePayloadError {
    TypeTooSpecific,
    ConflictingField,
}

impl MaybePayloadError {
    pub(crate) fn into_load_error(self, error_field: ErrorField) -> PatternLoadError {
        match self {
            Self::TypeTooSpecific => PatternLoadError::TypeTooSpecific(error_field),
            Self::ConflictingField => PatternLoadError::ConflictingField(error_field),
        }
    }
}

/// A type that may or may not be a [`DataPayload`] and may or may not contain
/// a value depending on the type parameter `Variables`.
///
/// Helper trait for [`DateTimeNamesMarker`].
#[allow(missing_docs)]
pub trait MaybePayload<M: DynamicDataMarker, Variables>: UnstableSealed {
    fn new_empty() -> Self;
    fn load_put<P>(
        &mut self,
        provider: &P,
        req: DataRequest,
        variables: Variables,
    ) -> Result<Result<(), DataError>, MaybePayloadError>
    where
        P: BoundDataProvider<M> + ?Sized,
        Self: Sized;
    fn get(&self) -> DataPayloadWithVariablesBorrowed<M, Variables>;
}

/// An implementation of [`MaybePayload`] that wraps an optional [`DataPayload`],
/// parameterized by `Variables`.
pub struct DataPayloadWithVariables<M: DynamicDataMarker, Variables> {
    inner: OptionalNames<Variables, DataPayload<M>>,
}

impl<M: DynamicDataMarker, Variables> UnstableSealed for DataPayloadWithVariables<M, Variables> {}

impl<M: DynamicDataMarker, Variables> fmt::Debug for DataPayloadWithVariables<M, Variables>
where
    Variables: fmt::Debug,
    DataPayload<M>: fmt::Debug,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.inner.fmt(f)
    }
}

// NOTE: This impl enables `with_fset` functions to work.
impl<M: DynamicDataMarker, Variables> From<()> for DataPayloadWithVariables<M, Variables> {
    #[inline]
    fn from(_: ()) -> Self {
        Self {
            inner: OptionalNames::None,
        }
    }
}

/// Borrowed version of [`DataPayloadWithVariables`].
#[allow(missing_docs)]
pub struct DataPayloadWithVariablesBorrowed<'data, M: DynamicDataMarker, Variables> {
    pub(crate) inner: OptionalNames<Variables, &'data <M::DataStruct as Yokeable<'data>>::Output>,
}

impl<'data, M: DynamicDataMarker, Variables> fmt::Debug
    for DataPayloadWithVariablesBorrowed<'data, M, Variables>
where
    <M::DataStruct as Yokeable<'data>>::Output: fmt::Debug,
    Variables: fmt::Debug,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct(core::any::type_name::<Self>())
            .field("inner", &self.inner)
            .finish()
    }
}

impl<M: DynamicDataMarker, Variables> MaybePayload<M, Variables>
    for DataPayloadWithVariables<M, Variables>
where
    Variables: PartialEq + Copy,
{
    #[inline]
    fn new_empty() -> Self {
        Self {
            inner: OptionalNames::None,
        }
    }
    fn load_put<P>(
        &mut self,
        provider: &P,
        req: DataRequest,
        variables: Variables,
    ) -> Result<Result<(), DataError>, MaybePayloadError>
    where
        P: BoundDataProvider<M> + ?Sized,
        Self: Sized,
    {
        let arg_variables = variables;
        match &self.inner {
            OptionalNames::SingleLength { variables, .. } if arg_variables == *variables => {
                return Ok(Ok(()));
            }
            OptionalNames::SingleLength { .. } => {
                return Err(MaybePayloadError::ConflictingField);
            }
            OptionalNames::None => (),
        };
        match provider.load_bound(req) {
            Ok(response) => {
                self.inner = OptionalNames::SingleLength {
                    payload: response.payload,
                    variables: arg_variables,
                };
                Ok(Ok(()))
            }
            Err(e) => Ok(Err(e)),
        }
    }
    #[inline]
    fn get(&self) -> DataPayloadWithVariablesBorrowed<M, Variables> {
        DataPayloadWithVariablesBorrowed {
            inner: self.inner.as_borrowed(),
        }
    }
}

impl<M: DynamicDataMarker, Variables> MaybePayload<M, Variables> for () {
    #[inline]
    fn new_empty() -> Self {}
    #[inline]
    fn load_put<P>(
        &mut self,
        _: &P,
        _: DataRequest,
        _: Variables,
    ) -> Result<Result<(), DataError>, MaybePayloadError>
    where
        P: BoundDataProvider<M> + ?Sized,
        Self: Sized,
    {
        Err(MaybePayloadError::TypeTooSpecific)
    }
    #[allow(clippy::needless_lifetimes)] // Yokeable is involved
    #[inline]
    fn get(&self) -> DataPayloadWithVariablesBorrowed<M, Variables> {
        DataPayloadWithVariablesBorrowed {
            inner: OptionalNames::None,
        }
    }
}

/// This can be extended in the future to support multiple lengths.
/// For now, this type wraps a symbols object tagged with a single length. See #4337
#[derive(Debug, Copy, Clone)]
pub(crate) enum OptionalNames<Variables, Payload> {
    None,
    SingleLength {
        variables: Variables,
        payload: Payload,
    },
}

impl<Variables, Payload> OptionalNames<Variables, Payload>
where
    Variables: Copy + PartialEq,
    Payload: Copy,
{
    pub(crate) fn get_with_variables(&self, arg_variables: Variables) -> Option<Payload> {
        match self {
            Self::None => None,
            Self::SingleLength { variables, payload } if arg_variables == *variables => {
                Some(*payload)
            }
            _ => None,
        }
    }
}

impl<Payload> OptionalNames<(), Payload>
where
    Payload: Copy,
{
    pub(crate) fn get_option(&self) -> Option<Payload> {
        match self {
            Self::SingleLength {
                variables: (),
                payload,
            } => Some(*payload),
            _ => None,
        }
    }
}

impl<M: DynamicDataMarker, Variables> OptionalNames<Variables, DataPayload<M>>
where
    Variables: Copy,
{
    #[allow(clippy::needless_lifetimes)] // Yokeable is involved
    #[inline]
    pub(crate) fn as_borrowed<'a>(
        &'a self,
    ) -> OptionalNames<Variables, &'a <M::DataStruct as Yokeable<'a>>::Output> {
        match self {
            Self::None => OptionalNames::None,
            Self::SingleLength { variables, payload } => OptionalNames::SingleLength {
                variables: *variables,
                payload: payload.get(),
            },
        }
    }
}

/// A trait for a [`DateTimeNamesMarker`] that can be created from a more specific one, `M`.
///
/// This trait is blanket-implemented on all [field sets](crate::fieldsets) that are more general than `M`.
///
/// # Examples
///
/// Example pairs of field sets where the trait is implemented:
///
/// ```
/// use icu::datetime::fieldsets::T;
/// use icu::datetime::fieldsets::YMD;
/// use icu::datetime::fieldsets::enums::DateFieldSet;
/// use icu::datetime::fieldsets::enums::TimeFieldSet;
/// use icu::datetime::fieldsets::enums::CompositeDateTimeFieldSet;
/// use icu::datetime::fieldsets::enums::CompositeFieldSet;
/// use icu::datetime::scaffold::DateTimeNamesFrom;
/// use icu::datetime::scaffold::DateTimeNamesMarker;
///
/// fn is_trait_implemented<Source, Target>()
/// where
///     Source: DateTimeNamesMarker,
///     Target: DateTimeNamesFrom<Source>
/// {}
///
/// is_trait_implemented::<YMD, DateFieldSet>();
/// is_trait_implemented::<YMD, CompositeDateTimeFieldSet>();
/// is_trait_implemented::<YMD, CompositeFieldSet>();
/// is_trait_implemented::<T, TimeFieldSet>();
/// is_trait_implemented::<T, CompositeDateTimeFieldSet>();
/// is_trait_implemented::<T, CompositeFieldSet>();
/// is_trait_implemented::<DateFieldSet, CompositeDateTimeFieldSet>();
/// is_trait_implemented::<DateFieldSet, CompositeFieldSet>();
/// is_trait_implemented::<TimeFieldSet, CompositeDateTimeFieldSet>();
/// is_trait_implemented::<TimeFieldSet, CompositeFieldSet>();
/// ```
#[allow(missing_docs)]
pub trait DateTimeNamesFrom<M: DateTimeNamesMarker>: DateTimeNamesMarker {
    fn map_year_names(
        other: <M::YearNames as NamesContainer<YearNamesV1Marker, YearNameLength>>::Container,
    ) -> <Self::YearNames as NamesContainer<YearNamesV1Marker, YearNameLength>>::Container;
    fn map_month_names(
        other: <M::MonthNames as NamesContainer<MonthNamesV1Marker, MonthNameLength>>::Container
    ) -> <Self::MonthNames as NamesContainer<MonthNamesV1Marker, MonthNameLength>>::Container;
    fn map_weekday_names(
        other: <M::WeekdayNames as NamesContainer<WeekdayNamesV1Marker, WeekdayNameLength>>::Container,
    ) -> <Self::WeekdayNames as NamesContainer<WeekdayNamesV1Marker,WeekdayNameLength>>::Container;
    fn map_day_period_names(
        other: <M::DayPeriodNames as NamesContainer<DayPeriodNamesV1Marker, DayPeriodNameLength>>::Container,
    ) -> <Self::DayPeriodNames as NamesContainer<DayPeriodNamesV1Marker, DayPeriodNameLength>>::Container;
    fn map_zone_essentials(
        other: <M::ZoneEssentials as NamesContainer<tz::EssentialsV1Marker, ()>>::Container,
    ) -> <Self::ZoneEssentials as NamesContainer<tz::EssentialsV1Marker, ()>>::Container;
    fn map_zone_locations(
        other: <M::ZoneLocations as NamesContainer<tz::LocationsV1Marker, ()>>::Container,
    ) -> <Self::ZoneLocations as NamesContainer<tz::LocationsV1Marker, ()>>::Container;
    fn map_zone_generic_long(
        other: <M::ZoneGenericLong as NamesContainer<tz::MzGenericLongV1Marker, ()>>::Container,
    ) -> <Self::ZoneGenericLong as NamesContainer<tz::MzGenericLongV1Marker, ()>>::Container;
    fn map_zone_generic_short(
        other: <M::ZoneGenericShort as NamesContainer<tz::MzGenericShortV1Marker, ()>>::Container,
    ) -> <Self::ZoneGenericShort as NamesContainer<tz::MzGenericShortV1Marker, ()>>::Container;
    fn map_zone_specific_long(
        other: <M::ZoneSpecificLong as NamesContainer<tz::MzSpecificLongV1Marker, ()>>::Container,
    ) -> <Self::ZoneSpecificLong as NamesContainer<tz::MzSpecificLongV1Marker, ()>>::Container;
    fn map_zone_specific_short(
        other: <M::ZoneSpecificShort as NamesContainer<tz::MzSpecificShortV1Marker, ()>>::Container,
    ) -> <Self::ZoneSpecificShort as NamesContainer<tz::MzSpecificShortV1Marker, ()>>::Container;
    fn map_metazone_lookup(
        other: <M::MetazoneLookup as NamesContainer<tz::MzPeriodV1Marker, ()>>::Container,
    ) -> <Self::MetazoneLookup as NamesContainer<tz::MzPeriodV1Marker, ()>>::Container;
}

impl<M: DateTimeNamesMarker, T: DateTimeNamesMarker> DateTimeNamesFrom<M> for T
where
    <Self::YearNames as NamesContainer<YearNamesV1Marker, YearNameLength>>::Container: From<<M::YearNames as NamesContainer<YearNamesV1Marker, YearNameLength>>::Container>,
    <Self::MonthNames as NamesContainer<MonthNamesV1Marker, MonthNameLength>>::Container: From<<M::MonthNames as NamesContainer<MonthNamesV1Marker, MonthNameLength>>::Container>,
    <Self::WeekdayNames as NamesContainer<WeekdayNamesV1Marker, WeekdayNameLength>>::Container: From<<M::WeekdayNames as NamesContainer<WeekdayNamesV1Marker, WeekdayNameLength>>::Container>,
    <Self::DayPeriodNames as NamesContainer<DayPeriodNamesV1Marker, DayPeriodNameLength>>::Container: From<<M::DayPeriodNames as NamesContainer<DayPeriodNamesV1Marker, DayPeriodNameLength>>::Container>,
    <Self::ZoneEssentials as NamesContainer<tz::EssentialsV1Marker, ()>>::Container: From<<M::ZoneEssentials as NamesContainer<tz::EssentialsV1Marker, ()>>::Container>,
    <Self::ZoneLocations as NamesContainer<tz::LocationsV1Marker, ()>>::Container: From<<M::ZoneLocations as NamesContainer<tz::LocationsV1Marker, ()>>::Container>,
    <Self::ZoneGenericLong as NamesContainer<tz::MzGenericLongV1Marker, ()>>::Container: From<<M::ZoneGenericLong as NamesContainer<tz::MzGenericLongV1Marker, ()>>::Container>,
    <Self::ZoneGenericShort as NamesContainer<tz::MzGenericShortV1Marker, ()>>::Container: From<<M::ZoneGenericShort as NamesContainer<tz::MzGenericShortV1Marker, ()>>::Container>,
    <Self::ZoneSpecificLong as NamesContainer<tz::MzSpecificLongV1Marker, ()>>::Container: From<<M::ZoneSpecificLong as NamesContainer<tz::MzSpecificLongV1Marker, ()>>::Container>,
    <Self::ZoneSpecificShort as NamesContainer<tz::MzSpecificShortV1Marker, ()>>::Container: From<<M::ZoneSpecificShort as NamesContainer<tz::MzSpecificShortV1Marker, ()>>::Container>,
    <Self::MetazoneLookup as NamesContainer<tz::MzPeriodV1Marker, ()>>::Container: From<<M::MetazoneLookup as NamesContainer<tz::MzPeriodV1Marker, ()>>::Container>,
{
    #[inline]
    fn map_year_names(other: <M::YearNames as NamesContainer<YearNamesV1Marker, YearNameLength>>::Container) -> <Self::YearNames as NamesContainer<YearNamesV1Marker, YearNameLength>>::Container {
        other.into()
    }
    #[inline]
    fn map_month_names(other: <M::MonthNames as NamesContainer<MonthNamesV1Marker, MonthNameLength>>::Container) -> <Self::MonthNames as NamesContainer<MonthNamesV1Marker, MonthNameLength>>::Container {
        other.into()
    }
    #[inline]
    fn map_weekday_names(other: <M::WeekdayNames as NamesContainer<WeekdayNamesV1Marker, WeekdayNameLength>>::Container) -> <Self::WeekdayNames as NamesContainer<WeekdayNamesV1Marker, WeekdayNameLength>>::Container {
        other.into()
    }
    #[inline]
    fn map_day_period_names(other: <M::DayPeriodNames as NamesContainer<DayPeriodNamesV1Marker, DayPeriodNameLength>>::Container) -> <Self::DayPeriodNames as NamesContainer<DayPeriodNamesV1Marker, DayPeriodNameLength>>::Container {
        other.into()
    }
    #[inline]
    fn map_zone_essentials(other: <M::ZoneEssentials as NamesContainer<tz::EssentialsV1Marker, ()>>::Container) -> <Self::ZoneEssentials as NamesContainer<tz::EssentialsV1Marker, ()>>::Container {
        other.into()
    }
    #[inline]
    fn map_zone_locations(other: <M::ZoneLocations as NamesContainer<tz::LocationsV1Marker, ()>>::Container) -> <Self::ZoneLocations as NamesContainer<tz::LocationsV1Marker, ()>>::Container {
        other.into()
    }
    #[inline]
    fn map_zone_generic_long(other: <M::ZoneGenericLong as NamesContainer<tz::MzGenericLongV1Marker, ()>>::Container) -> <Self::ZoneGenericLong as NamesContainer<tz::MzGenericLongV1Marker, ()>>::Container {
        other.into()
    }
    #[inline]
    fn map_zone_generic_short(other: <M::ZoneGenericShort as NamesContainer<tz::MzGenericShortV1Marker, ()>>::Container) -> <Self::ZoneGenericShort as NamesContainer<tz::MzGenericShortV1Marker, ()>>::Container {
        other.into()
    }
    #[inline]
    fn map_zone_specific_long(other: <M::ZoneSpecificLong as NamesContainer<tz::MzSpecificLongV1Marker, ()>>::Container) -> <Self::ZoneSpecificLong as NamesContainer<tz::MzSpecificLongV1Marker, ()>>::Container {
        other.into()
    }
    #[inline]
    fn map_zone_specific_short(other: <M::ZoneSpecificShort as NamesContainer<tz::MzSpecificShortV1Marker, ()>>::Container) -> <Self::ZoneSpecificShort as NamesContainer<tz::MzSpecificShortV1Marker, ()>>::Container {
        other.into()
    }
    #[inline]
    fn map_metazone_lookup(other: <M::MetazoneLookup as NamesContainer<tz::MzPeriodV1Marker, ()>>::Container) -> <Self::MetazoneLookup as NamesContainer<tz::MzPeriodV1Marker, ()>>::Container {
        other.into()
    }
}
