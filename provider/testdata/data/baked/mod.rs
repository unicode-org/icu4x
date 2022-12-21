// @generated
mod calendar;
mod collator;
mod compactdecimal;
mod core;
mod datetime;
mod decimal;
mod displaynames;
mod fallback;
mod list;
mod locid_transform;
mod normalizer;
mod plurals;
mod props;
mod relativetime;
mod segmenter;
mod time_zone;
use ::icu_provider::prelude::*;
/// Implement [`DataProvider<M>`] on the given struct using the data
/// hardcoded in this module. This allows the struct to be used with
/// `icu`'s `_unstable` constructors.
///
/// This macro can only be called from its definition-site, i.e. right
/// after `include!`-ing the generated module.
///
/// ```compile_fail
/// struct MyDataProvider;
/// include!("/path/to/generated/mod.rs");
/// impl_data_provider(MyDataProvider);
/// ```
#[allow(unused_macros)]
macro_rules! impl_data_provider {
    ($ provider : path) => {
        #[cfg(feature = "icu_calendar")]
        impl DataProvider<::icu_calendar::provider::JapaneseErasV1Marker> for $provider {
            fn load(&self, req: DataRequest) -> Result<DataResponse<::icu_calendar::provider::JapaneseErasV1Marker>, DataError> {
                calendar::japanese_v1::lookup(&req.locale)
                    .map(zerofrom::ZeroFrom::zero_from)
                    .map(DataPayload::from_owned)
                    .map(|payload| DataResponse {
                        metadata: Default::default(),
                        payload: Some(payload),
                    })
                    .ok_or_else(|| DataErrorKind::MissingLocale.with_req(::icu_calendar::provider::JapaneseErasV1Marker::KEY, req))
            }
        }
        #[cfg(feature = "icu_calendar")]
        impl DataProvider<::icu_calendar::provider::JapaneseExtendedErasV1Marker> for $provider {
            fn load(&self, req: DataRequest) -> Result<DataResponse<::icu_calendar::provider::JapaneseExtendedErasV1Marker>, DataError> {
                calendar::japanext_v1::lookup(&req.locale)
                    .map(zerofrom::ZeroFrom::zero_from)
                    .map(DataPayload::from_owned)
                    .map(|payload| DataResponse {
                        metadata: Default::default(),
                        payload: Some(payload),
                    })
                    .ok_or_else(|| DataErrorKind::MissingLocale.with_req(::icu_calendar::provider::JapaneseExtendedErasV1Marker::KEY, req))
            }
        }
        #[cfg(feature = "icu_calendar")]
        impl DataProvider<::icu_calendar::provider::WeekDataV1Marker> for $provider {
            fn load(&self, req: DataRequest) -> Result<DataResponse<::icu_calendar::provider::WeekDataV1Marker>, DataError> {
                datetime::week_data_v1::lookup(&req.locale)
                    .map(zerofrom::ZeroFrom::zero_from)
                    .map(DataPayload::from_owned)
                    .map(|payload| DataResponse {
                        metadata: Default::default(),
                        payload: Some(payload),
                    })
                    .ok_or_else(|| DataErrorKind::MissingLocale.with_req(::icu_calendar::provider::WeekDataV1Marker::KEY, req))
            }
        }
        #[cfg(feature = "icu_casemapping")]
        impl DataProvider<::icu_casemapping::provider::CaseMappingV1Marker> for $provider {
            fn load(&self, req: DataRequest) -> Result<DataResponse<::icu_casemapping::provider::CaseMappingV1Marker>, DataError> {
                props::casemap_v1::lookup(&req.locale)
                    .map(zerofrom::ZeroFrom::zero_from)
                    .map(DataPayload::from_owned)
                    .map(|payload| DataResponse {
                        metadata: Default::default(),
                        payload: Some(payload),
                    })
                    .ok_or_else(|| DataErrorKind::MissingLocale.with_req(::icu_casemapping::provider::CaseMappingV1Marker::KEY, req))
            }
        }
        #[cfg(feature = "icu_collator")]
        impl DataProvider<::icu_collator::provider::CollationDataV1Marker> for $provider {
            fn load(&self, req: DataRequest) -> Result<DataResponse<::icu_collator::provider::CollationDataV1Marker>, DataError> {
                collator::data_v1::lookup(&req.locale)
                    .map(zerofrom::ZeroFrom::zero_from)
                    .map(DataPayload::from_owned)
                    .map(|payload| DataResponse {
                        metadata: Default::default(),
                        payload: Some(payload),
                    })
                    .ok_or_else(|| DataErrorKind::MissingLocale.with_req(::icu_collator::provider::CollationDataV1Marker::KEY, req))
            }
        }
        #[cfg(feature = "icu_collator")]
        impl DataProvider<::icu_collator::provider::CollationDiacriticsV1Marker> for $provider {
            fn load(&self, req: DataRequest) -> Result<DataResponse<::icu_collator::provider::CollationDiacriticsV1Marker>, DataError> {
                collator::dia_v1::lookup(&req.locale)
                    .map(zerofrom::ZeroFrom::zero_from)
                    .map(DataPayload::from_owned)
                    .map(|payload| DataResponse {
                        metadata: Default::default(),
                        payload: Some(payload),
                    })
                    .ok_or_else(|| DataErrorKind::MissingLocale.with_req(::icu_collator::provider::CollationDiacriticsV1Marker::KEY, req))
            }
        }
        #[cfg(feature = "icu_collator")]
        impl DataProvider<::icu_collator::provider::CollationJamoV1Marker> for $provider {
            fn load(&self, req: DataRequest) -> Result<DataResponse<::icu_collator::provider::CollationJamoV1Marker>, DataError> {
                collator::jamo_v1::lookup(&req.locale)
                    .map(zerofrom::ZeroFrom::zero_from)
                    .map(DataPayload::from_owned)
                    .map(|payload| DataResponse {
                        metadata: Default::default(),
                        payload: Some(payload),
                    })
                    .ok_or_else(|| DataErrorKind::MissingLocale.with_req(::icu_collator::provider::CollationJamoV1Marker::KEY, req))
            }
        }
        #[cfg(feature = "icu_collator")]
        impl DataProvider<::icu_collator::provider::CollationMetadataV1Marker> for $provider {
            fn load(&self, req: DataRequest) -> Result<DataResponse<::icu_collator::provider::CollationMetadataV1Marker>, DataError> {
                collator::meta_v1::lookup(&req.locale)
                    .map(zerofrom::ZeroFrom::zero_from)
                    .map(DataPayload::from_owned)
                    .map(|payload| DataResponse {
                        metadata: Default::default(),
                        payload: Some(payload),
                    })
                    .ok_or_else(|| DataErrorKind::MissingLocale.with_req(::icu_collator::provider::CollationMetadataV1Marker::KEY, req))
            }
        }
        #[cfg(feature = "icu_collator")]
        impl DataProvider<::icu_collator::provider::CollationReorderingV1Marker> for $provider {
            fn load(&self, req: DataRequest) -> Result<DataResponse<::icu_collator::provider::CollationReorderingV1Marker>, DataError> {
                collator::reord_v1::lookup(&req.locale)
                    .map(zerofrom::ZeroFrom::zero_from)
                    .map(DataPayload::from_owned)
                    .map(|payload| DataResponse {
                        metadata: Default::default(),
                        payload: Some(payload),
                    })
                    .ok_or_else(|| DataErrorKind::MissingLocale.with_req(::icu_collator::provider::CollationReorderingV1Marker::KEY, req))
            }
        }
        #[cfg(feature = "icu_collator")]
        impl DataProvider<::icu_collator::provider::CollationSpecialPrimariesV1Marker> for $provider {
            fn load(&self, req: DataRequest) -> Result<DataResponse<::icu_collator::provider::CollationSpecialPrimariesV1Marker>, DataError> {
                collator::prim_v1::lookup(&req.locale)
                    .map(zerofrom::ZeroFrom::zero_from)
                    .map(DataPayload::from_owned)
                    .map(|payload| DataResponse {
                        metadata: Default::default(),
                        payload: Some(payload),
                    })
                    .ok_or_else(|| DataErrorKind::MissingLocale.with_req(::icu_collator::provider::CollationSpecialPrimariesV1Marker::KEY, req))
            }
        }
        #[cfg(feature = "icu_compactdecimal")]
        impl DataProvider<::icu_compactdecimal::provider::LongCompactDecimalFormatDataV1Marker> for $provider {
            fn load(
                &self,
                req: DataRequest,
            ) -> Result<DataResponse<::icu_compactdecimal::provider::LongCompactDecimalFormatDataV1Marker>, DataError> {
                compactdecimal::long_v1::lookup(&req.locale)
                    .map(zerofrom::ZeroFrom::zero_from)
                    .map(DataPayload::from_owned)
                    .map(|payload| DataResponse {
                        metadata: Default::default(),
                        payload: Some(payload),
                    })
                    .ok_or_else(|| {
                        DataErrorKind::MissingLocale.with_req(::icu_compactdecimal::provider::LongCompactDecimalFormatDataV1Marker::KEY, req)
                    })
            }
        }
        #[cfg(feature = "icu_compactdecimal")]
        impl DataProvider<::icu_compactdecimal::provider::ShortCompactDecimalFormatDataV1Marker> for $provider {
            fn load(
                &self,
                req: DataRequest,
            ) -> Result<DataResponse<::icu_compactdecimal::provider::ShortCompactDecimalFormatDataV1Marker>, DataError> {
                compactdecimal::short_v1::lookup(&req.locale)
                    .map(zerofrom::ZeroFrom::zero_from)
                    .map(DataPayload::from_owned)
                    .map(|payload| DataResponse {
                        metadata: Default::default(),
                        payload: Some(payload),
                    })
                    .ok_or_else(|| {
                        DataErrorKind::MissingLocale.with_req(::icu_compactdecimal::provider::ShortCompactDecimalFormatDataV1Marker::KEY, req)
                    })
            }
        }
        #[cfg(feature = "icu_datetime")]
        impl DataProvider<::icu_datetime::provider::calendar::BuddhistDateLengthsV1Marker> for $provider {
            fn load(&self, req: DataRequest) -> Result<DataResponse<::icu_datetime::provider::calendar::BuddhistDateLengthsV1Marker>, DataError> {
                datetime::buddhist::datelengths_v1::lookup(&req.locale)
                    .map(zerofrom::ZeroFrom::zero_from)
                    .map(DataPayload::from_owned)
                    .map(|payload| DataResponse {
                        metadata: Default::default(),
                        payload: Some(payload),
                    })
                    .ok_or_else(|| DataErrorKind::MissingLocale.with_req(::icu_datetime::provider::calendar::BuddhistDateLengthsV1Marker::KEY, req))
            }
        }
        #[cfg(feature = "icu_datetime")]
        impl DataProvider<::icu_datetime::provider::calendar::BuddhistDateSymbolsV1Marker> for $provider {
            fn load(&self, req: DataRequest) -> Result<DataResponse<::icu_datetime::provider::calendar::BuddhistDateSymbolsV1Marker>, DataError> {
                datetime::buddhist::datesymbols_v1::lookup(&req.locale)
                    .map(zerofrom::ZeroFrom::zero_from)
                    .map(DataPayload::from_owned)
                    .map(|payload| DataResponse {
                        metadata: Default::default(),
                        payload: Some(payload),
                    })
                    .ok_or_else(|| DataErrorKind::MissingLocale.with_req(::icu_datetime::provider::calendar::BuddhistDateSymbolsV1Marker::KEY, req))
            }
        }
        #[cfg(feature = "icu_datetime")]
        impl DataProvider<::icu_datetime::provider::calendar::CopticDateLengthsV1Marker> for $provider {
            fn load(&self, req: DataRequest) -> Result<DataResponse<::icu_datetime::provider::calendar::CopticDateLengthsV1Marker>, DataError> {
                datetime::coptic::datelengths_v1::lookup(&req.locale)
                    .map(zerofrom::ZeroFrom::zero_from)
                    .map(DataPayload::from_owned)
                    .map(|payload| DataResponse {
                        metadata: Default::default(),
                        payload: Some(payload),
                    })
                    .ok_or_else(|| DataErrorKind::MissingLocale.with_req(::icu_datetime::provider::calendar::CopticDateLengthsV1Marker::KEY, req))
            }
        }
        #[cfg(feature = "icu_datetime")]
        impl DataProvider<::icu_datetime::provider::calendar::CopticDateSymbolsV1Marker> for $provider {
            fn load(&self, req: DataRequest) -> Result<DataResponse<::icu_datetime::provider::calendar::CopticDateSymbolsV1Marker>, DataError> {
                datetime::coptic::datesymbols_v1::lookup(&req.locale)
                    .map(zerofrom::ZeroFrom::zero_from)
                    .map(DataPayload::from_owned)
                    .map(|payload| DataResponse {
                        metadata: Default::default(),
                        payload: Some(payload),
                    })
                    .ok_or_else(|| DataErrorKind::MissingLocale.with_req(::icu_datetime::provider::calendar::CopticDateSymbolsV1Marker::KEY, req))
            }
        }
        #[cfg(feature = "icu_datetime_experimental")]
        impl DataProvider<::icu_datetime::provider::calendar::DateSkeletonPatternsV1Marker> for $provider {
            fn load(&self, req: DataRequest) -> Result<DataResponse<::icu_datetime::provider::calendar::DateSkeletonPatternsV1Marker>, DataError> {
                datetime::skeletons_v1::lookup(&req.locale)
                    .map(zerofrom::ZeroFrom::zero_from)
                    .map(DataPayload::from_owned)
                    .map(|payload| DataResponse {
                        metadata: Default::default(),
                        payload: Some(payload),
                    })
                    .ok_or_else(|| DataErrorKind::MissingLocale.with_req(::icu_datetime::provider::calendar::DateSkeletonPatternsV1Marker::KEY, req))
            }
        }
        #[cfg(feature = "icu_datetime")]
        impl DataProvider<::icu_datetime::provider::calendar::EthiopianDateLengthsV1Marker> for $provider {
            fn load(&self, req: DataRequest) -> Result<DataResponse<::icu_datetime::provider::calendar::EthiopianDateLengthsV1Marker>, DataError> {
                datetime::ethiopic::datelengths_v1::lookup(&req.locale)
                    .map(zerofrom::ZeroFrom::zero_from)
                    .map(DataPayload::from_owned)
                    .map(|payload| DataResponse {
                        metadata: Default::default(),
                        payload: Some(payload),
                    })
                    .ok_or_else(|| DataErrorKind::MissingLocale.with_req(::icu_datetime::provider::calendar::EthiopianDateLengthsV1Marker::KEY, req))
            }
        }
        #[cfg(feature = "icu_datetime")]
        impl DataProvider<::icu_datetime::provider::calendar::EthiopianDateSymbolsV1Marker> for $provider {
            fn load(&self, req: DataRequest) -> Result<DataResponse<::icu_datetime::provider::calendar::EthiopianDateSymbolsV1Marker>, DataError> {
                datetime::ethiopic::datesymbols_v1::lookup(&req.locale)
                    .map(zerofrom::ZeroFrom::zero_from)
                    .map(DataPayload::from_owned)
                    .map(|payload| DataResponse {
                        metadata: Default::default(),
                        payload: Some(payload),
                    })
                    .ok_or_else(|| DataErrorKind::MissingLocale.with_req(::icu_datetime::provider::calendar::EthiopianDateSymbolsV1Marker::KEY, req))
            }
        }
        #[cfg(feature = "icu_datetime")]
        impl DataProvider<::icu_datetime::provider::calendar::GregorianDateLengthsV1Marker> for $provider {
            fn load(&self, req: DataRequest) -> Result<DataResponse<::icu_datetime::provider::calendar::GregorianDateLengthsV1Marker>, DataError> {
                datetime::gregory::datelengths_v1::lookup(&req.locale)
                    .map(zerofrom::ZeroFrom::zero_from)
                    .map(DataPayload::from_owned)
                    .map(|payload| DataResponse {
                        metadata: Default::default(),
                        payload: Some(payload),
                    })
                    .ok_or_else(|| DataErrorKind::MissingLocale.with_req(::icu_datetime::provider::calendar::GregorianDateLengthsV1Marker::KEY, req))
            }
        }
        #[cfg(feature = "icu_datetime")]
        impl DataProvider<::icu_datetime::provider::calendar::GregorianDateSymbolsV1Marker> for $provider {
            fn load(&self, req: DataRequest) -> Result<DataResponse<::icu_datetime::provider::calendar::GregorianDateSymbolsV1Marker>, DataError> {
                datetime::gregory::datesymbols_v1::lookup(&req.locale)
                    .map(zerofrom::ZeroFrom::zero_from)
                    .map(DataPayload::from_owned)
                    .map(|payload| DataResponse {
                        metadata: Default::default(),
                        payload: Some(payload),
                    })
                    .ok_or_else(|| DataErrorKind::MissingLocale.with_req(::icu_datetime::provider::calendar::GregorianDateSymbolsV1Marker::KEY, req))
            }
        }
        #[cfg(feature = "icu_datetime")]
        impl DataProvider<::icu_datetime::provider::calendar::IndianDateLengthsV1Marker> for $provider {
            fn load(&self, req: DataRequest) -> Result<DataResponse<::icu_datetime::provider::calendar::IndianDateLengthsV1Marker>, DataError> {
                datetime::indian::datelengths_v1::lookup(&req.locale)
                    .map(zerofrom::ZeroFrom::zero_from)
                    .map(DataPayload::from_owned)
                    .map(|payload| DataResponse {
                        metadata: Default::default(),
                        payload: Some(payload),
                    })
                    .ok_or_else(|| DataErrorKind::MissingLocale.with_req(::icu_datetime::provider::calendar::IndianDateLengthsV1Marker::KEY, req))
            }
        }
        #[cfg(feature = "icu_datetime")]
        impl DataProvider<::icu_datetime::provider::calendar::IndianDateSymbolsV1Marker> for $provider {
            fn load(&self, req: DataRequest) -> Result<DataResponse<::icu_datetime::provider::calendar::IndianDateSymbolsV1Marker>, DataError> {
                datetime::indian::datesymbols_v1::lookup(&req.locale)
                    .map(zerofrom::ZeroFrom::zero_from)
                    .map(DataPayload::from_owned)
                    .map(|payload| DataResponse {
                        metadata: Default::default(),
                        payload: Some(payload),
                    })
                    .ok_or_else(|| DataErrorKind::MissingLocale.with_req(::icu_datetime::provider::calendar::IndianDateSymbolsV1Marker::KEY, req))
            }
        }
        #[cfg(feature = "icu_datetime")]
        impl DataProvider<::icu_datetime::provider::calendar::JapaneseDateLengthsV1Marker> for $provider {
            fn load(&self, req: DataRequest) -> Result<DataResponse<::icu_datetime::provider::calendar::JapaneseDateLengthsV1Marker>, DataError> {
                datetime::japanese::datelengths_v1::lookup(&req.locale)
                    .map(zerofrom::ZeroFrom::zero_from)
                    .map(DataPayload::from_owned)
                    .map(|payload| DataResponse {
                        metadata: Default::default(),
                        payload: Some(payload),
                    })
                    .ok_or_else(|| DataErrorKind::MissingLocale.with_req(::icu_datetime::provider::calendar::JapaneseDateLengthsV1Marker::KEY, req))
            }
        }
        #[cfg(feature = "icu_datetime")]
        impl DataProvider<::icu_datetime::provider::calendar::JapaneseDateSymbolsV1Marker> for $provider {
            fn load(&self, req: DataRequest) -> Result<DataResponse<::icu_datetime::provider::calendar::JapaneseDateSymbolsV1Marker>, DataError> {
                datetime::japanese::datesymbols_v1::lookup(&req.locale)
                    .map(zerofrom::ZeroFrom::zero_from)
                    .map(DataPayload::from_owned)
                    .map(|payload| DataResponse {
                        metadata: Default::default(),
                        payload: Some(payload),
                    })
                    .ok_or_else(|| DataErrorKind::MissingLocale.with_req(::icu_datetime::provider::calendar::JapaneseDateSymbolsV1Marker::KEY, req))
            }
        }
        #[cfg(feature = "icu_datetime")]
        impl DataProvider<::icu_datetime::provider::calendar::JapaneseExtendedDateLengthsV1Marker> for $provider {
            fn load(
                &self,
                req: DataRequest,
            ) -> Result<DataResponse<::icu_datetime::provider::calendar::JapaneseExtendedDateLengthsV1Marker>, DataError> {
                datetime::japanext::datelengths_v1::lookup(&req.locale)
                    .map(zerofrom::ZeroFrom::zero_from)
                    .map(DataPayload::from_owned)
                    .map(|payload| DataResponse {
                        metadata: Default::default(),
                        payload: Some(payload),
                    })
                    .ok_or_else(|| {
                        DataErrorKind::MissingLocale.with_req(::icu_datetime::provider::calendar::JapaneseExtendedDateLengthsV1Marker::KEY, req)
                    })
            }
        }
        #[cfg(feature = "icu_datetime")]
        impl DataProvider<::icu_datetime::provider::calendar::JapaneseExtendedDateSymbolsV1Marker> for $provider {
            fn load(
                &self,
                req: DataRequest,
            ) -> Result<DataResponse<::icu_datetime::provider::calendar::JapaneseExtendedDateSymbolsV1Marker>, DataError> {
                datetime::japanext::datesymbols_v1::lookup(&req.locale)
                    .map(zerofrom::ZeroFrom::zero_from)
                    .map(DataPayload::from_owned)
                    .map(|payload| DataResponse {
                        metadata: Default::default(),
                        payload: Some(payload),
                    })
                    .ok_or_else(|| {
                        DataErrorKind::MissingLocale.with_req(::icu_datetime::provider::calendar::JapaneseExtendedDateSymbolsV1Marker::KEY, req)
                    })
            }
        }
        #[cfg(feature = "icu_datetime")]
        impl DataProvider<::icu_datetime::provider::calendar::TimeLengthsV1Marker> for $provider {
            fn load(&self, req: DataRequest) -> Result<DataResponse<::icu_datetime::provider::calendar::TimeLengthsV1Marker>, DataError> {
                datetime::timelengths_v1::lookup(&req.locale)
                    .map(zerofrom::ZeroFrom::zero_from)
                    .map(DataPayload::from_owned)
                    .map(|payload| DataResponse {
                        metadata: Default::default(),
                        payload: Some(payload),
                    })
                    .ok_or_else(|| DataErrorKind::MissingLocale.with_req(::icu_datetime::provider::calendar::TimeLengthsV1Marker::KEY, req))
            }
        }
        #[cfg(feature = "icu_datetime")]
        impl DataProvider<::icu_datetime::provider::calendar::TimeSymbolsV1Marker> for $provider {
            fn load(&self, req: DataRequest) -> Result<DataResponse<::icu_datetime::provider::calendar::TimeSymbolsV1Marker>, DataError> {
                datetime::timesymbols_v1::lookup(&req.locale)
                    .map(zerofrom::ZeroFrom::zero_from)
                    .map(DataPayload::from_owned)
                    .map(|payload| DataResponse {
                        metadata: Default::default(),
                        payload: Some(payload),
                    })
                    .ok_or_else(|| DataErrorKind::MissingLocale.with_req(::icu_datetime::provider::calendar::TimeSymbolsV1Marker::KEY, req))
            }
        }
        #[cfg(feature = "icu_datetime")]
        impl DataProvider<::icu_datetime::provider::time_zones::ExemplarCitiesV1Marker> for $provider {
            fn load(&self, req: DataRequest) -> Result<DataResponse<::icu_datetime::provider::time_zones::ExemplarCitiesV1Marker>, DataError> {
                time_zone::exemplar_cities_v1::lookup(&req.locale)
                    .map(zerofrom::ZeroFrom::zero_from)
                    .map(DataPayload::from_owned)
                    .map(|payload| DataResponse {
                        metadata: Default::default(),
                        payload: Some(payload),
                    })
                    .ok_or_else(|| DataErrorKind::MissingLocale.with_req(::icu_datetime::provider::time_zones::ExemplarCitiesV1Marker::KEY, req))
            }
        }
        #[cfg(feature = "icu_datetime")]
        impl DataProvider<::icu_datetime::provider::time_zones::MetazoneGenericNamesLongV1Marker> for $provider {
            fn load(
                &self,
                req: DataRequest,
            ) -> Result<DataResponse<::icu_datetime::provider::time_zones::MetazoneGenericNamesLongV1Marker>, DataError> {
                time_zone::generic_long_v1::lookup(&req.locale)
                    .map(zerofrom::ZeroFrom::zero_from)
                    .map(DataPayload::from_owned)
                    .map(|payload| DataResponse {
                        metadata: Default::default(),
                        payload: Some(payload),
                    })
                    .ok_or_else(|| {
                        DataErrorKind::MissingLocale.with_req(::icu_datetime::provider::time_zones::MetazoneGenericNamesLongV1Marker::KEY, req)
                    })
            }
        }
        #[cfg(feature = "icu_datetime")]
        impl DataProvider<::icu_datetime::provider::time_zones::MetazoneGenericNamesShortV1Marker> for $provider {
            fn load(
                &self,
                req: DataRequest,
            ) -> Result<DataResponse<::icu_datetime::provider::time_zones::MetazoneGenericNamesShortV1Marker>, DataError> {
                time_zone::generic_short_v1::lookup(&req.locale)
                    .map(zerofrom::ZeroFrom::zero_from)
                    .map(DataPayload::from_owned)
                    .map(|payload| DataResponse {
                        metadata: Default::default(),
                        payload: Some(payload),
                    })
                    .ok_or_else(|| {
                        DataErrorKind::MissingLocale.with_req(::icu_datetime::provider::time_zones::MetazoneGenericNamesShortV1Marker::KEY, req)
                    })
            }
        }
        #[cfg(feature = "icu_datetime")]
        impl DataProvider<::icu_datetime::provider::time_zones::MetazoneSpecificNamesLongV1Marker> for $provider {
            fn load(
                &self,
                req: DataRequest,
            ) -> Result<DataResponse<::icu_datetime::provider::time_zones::MetazoneSpecificNamesLongV1Marker>, DataError> {
                time_zone::specific_long_v1::lookup(&req.locale)
                    .map(zerofrom::ZeroFrom::zero_from)
                    .map(DataPayload::from_owned)
                    .map(|payload| DataResponse {
                        metadata: Default::default(),
                        payload: Some(payload),
                    })
                    .ok_or_else(|| {
                        DataErrorKind::MissingLocale.with_req(::icu_datetime::provider::time_zones::MetazoneSpecificNamesLongV1Marker::KEY, req)
                    })
            }
        }
        #[cfg(feature = "icu_datetime")]
        impl DataProvider<::icu_datetime::provider::time_zones::MetazoneSpecificNamesShortV1Marker> for $provider {
            fn load(
                &self,
                req: DataRequest,
            ) -> Result<DataResponse<::icu_datetime::provider::time_zones::MetazoneSpecificNamesShortV1Marker>, DataError> {
                time_zone::specific_short_v1::lookup(&req.locale)
                    .map(zerofrom::ZeroFrom::zero_from)
                    .map(DataPayload::from_owned)
                    .map(|payload| DataResponse {
                        metadata: Default::default(),
                        payload: Some(payload),
                    })
                    .ok_or_else(|| {
                        DataErrorKind::MissingLocale.with_req(::icu_datetime::provider::time_zones::MetazoneSpecificNamesShortV1Marker::KEY, req)
                    })
            }
        }
        #[cfg(feature = "icu_datetime")]
        impl DataProvider<::icu_datetime::provider::time_zones::TimeZoneFormatsV1Marker> for $provider {
            fn load(&self, req: DataRequest) -> Result<DataResponse<::icu_datetime::provider::time_zones::TimeZoneFormatsV1Marker>, DataError> {
                time_zone::formats_v1::lookup(&req.locale)
                    .map(zerofrom::ZeroFrom::zero_from)
                    .map(DataPayload::from_owned)
                    .map(|payload| DataResponse {
                        metadata: Default::default(),
                        payload: Some(payload),
                    })
                    .ok_or_else(|| DataErrorKind::MissingLocale.with_req(::icu_datetime::provider::time_zones::TimeZoneFormatsV1Marker::KEY, req))
            }
        }
        #[cfg(feature = "icu_decimal")]
        impl DataProvider<::icu_decimal::provider::DecimalSymbolsV1Marker> for $provider {
            fn load(&self, req: DataRequest) -> Result<DataResponse<::icu_decimal::provider::DecimalSymbolsV1Marker>, DataError> {
                decimal::symbols_v1::lookup(&req.locale)
                    .map(zerofrom::ZeroFrom::zero_from)
                    .map(DataPayload::from_owned)
                    .map(|payload| DataResponse {
                        metadata: Default::default(),
                        payload: Some(payload),
                    })
                    .ok_or_else(|| DataErrorKind::MissingLocale.with_req(::icu_decimal::provider::DecimalSymbolsV1Marker::KEY, req))
            }
        }
        #[cfg(feature = "icu_displaynames")]
        impl DataProvider<::icu_displaynames::provider::LanguageDisplayNamesV1Marker> for $provider {
            fn load(&self, req: DataRequest) -> Result<DataResponse<::icu_displaynames::provider::LanguageDisplayNamesV1Marker>, DataError> {
                displaynames::languages_v1::lookup(&req.locale)
                    .map(zerofrom::ZeroFrom::zero_from)
                    .map(DataPayload::from_owned)
                    .map(|payload| DataResponse {
                        metadata: Default::default(),
                        payload: Some(payload),
                    })
                    .ok_or_else(|| DataErrorKind::MissingLocale.with_req(::icu_displaynames::provider::LanguageDisplayNamesV1Marker::KEY, req))
            }
        }
        #[cfg(feature = "icu_displaynames")]
        impl DataProvider<::icu_displaynames::provider::TerritoryDisplayNamesV1Marker> for $provider {
            fn load(&self, req: DataRequest) -> Result<DataResponse<::icu_displaynames::provider::TerritoryDisplayNamesV1Marker>, DataError> {
                displaynames::territories_v1::lookup(&req.locale)
                    .map(zerofrom::ZeroFrom::zero_from)
                    .map(DataPayload::from_owned)
                    .map(|payload| DataResponse {
                        metadata: Default::default(),
                        payload: Some(payload),
                    })
                    .ok_or_else(|| DataErrorKind::MissingLocale.with_req(::icu_displaynames::provider::TerritoryDisplayNamesV1Marker::KEY, req))
            }
        }
        #[cfg(feature = "icu_list")]
        impl DataProvider<::icu_list::provider::AndListV1Marker> for $provider {
            fn load(&self, req: DataRequest) -> Result<DataResponse<::icu_list::provider::AndListV1Marker>, DataError> {
                list::and_v1::lookup(&req.locale)
                    .map(zerofrom::ZeroFrom::zero_from)
                    .map(DataPayload::from_owned)
                    .map(|payload| DataResponse {
                        metadata: Default::default(),
                        payload: Some(payload),
                    })
                    .ok_or_else(|| DataErrorKind::MissingLocale.with_req(::icu_list::provider::AndListV1Marker::KEY, req))
            }
        }
        #[cfg(feature = "icu_list")]
        impl DataProvider<::icu_list::provider::OrListV1Marker> for $provider {
            fn load(&self, req: DataRequest) -> Result<DataResponse<::icu_list::provider::OrListV1Marker>, DataError> {
                list::or_v1::lookup(&req.locale)
                    .map(zerofrom::ZeroFrom::zero_from)
                    .map(DataPayload::from_owned)
                    .map(|payload| DataResponse {
                        metadata: Default::default(),
                        payload: Some(payload),
                    })
                    .ok_or_else(|| DataErrorKind::MissingLocale.with_req(::icu_list::provider::OrListV1Marker::KEY, req))
            }
        }
        #[cfg(feature = "icu_list")]
        impl DataProvider<::icu_list::provider::UnitListV1Marker> for $provider {
            fn load(&self, req: DataRequest) -> Result<DataResponse<::icu_list::provider::UnitListV1Marker>, DataError> {
                list::unit_v1::lookup(&req.locale)
                    .map(zerofrom::ZeroFrom::zero_from)
                    .map(DataPayload::from_owned)
                    .map(|payload| DataResponse {
                        metadata: Default::default(),
                        payload: Some(payload),
                    })
                    .ok_or_else(|| DataErrorKind::MissingLocale.with_req(::icu_list::provider::UnitListV1Marker::KEY, req))
            }
        }
        #[cfg(feature = "icu_locid_transform")]
        impl DataProvider<::icu_locid_transform::provider::AliasesV1Marker> for $provider {
            fn load(&self, req: DataRequest) -> Result<DataResponse<::icu_locid_transform::provider::AliasesV1Marker>, DataError> {
                locid_transform::aliases_v1::lookup(&req.locale)
                    .map(zerofrom::ZeroFrom::zero_from)
                    .map(DataPayload::from_owned)
                    .map(|payload| DataResponse {
                        metadata: Default::default(),
                        payload: Some(payload),
                    })
                    .ok_or_else(|| DataErrorKind::MissingLocale.with_req(::icu_locid_transform::provider::AliasesV1Marker::KEY, req))
            }
        }
        #[cfg(feature = "icu_locid_transform")]
        impl DataProvider<::icu_locid_transform::provider::LikelySubtagsV1Marker> for $provider {
            fn load(&self, req: DataRequest) -> Result<DataResponse<::icu_locid_transform::provider::LikelySubtagsV1Marker>, DataError> {
                locid_transform::likelysubtags_v1::lookup(&req.locale)
                    .map(zerofrom::ZeroFrom::zero_from)
                    .map(DataPayload::from_owned)
                    .map(|payload| DataResponse {
                        metadata: Default::default(),
                        payload: Some(payload),
                    })
                    .ok_or_else(|| DataErrorKind::MissingLocale.with_req(::icu_locid_transform::provider::LikelySubtagsV1Marker::KEY, req))
            }
        }
        #[cfg(feature = "icu_normalizer")]
        impl DataProvider<::icu_normalizer::provider::CanonicalCompositionsV1Marker> for $provider {
            fn load(&self, req: DataRequest) -> Result<DataResponse<::icu_normalizer::provider::CanonicalCompositionsV1Marker>, DataError> {
                normalizer::comp_v1::lookup(&req.locale)
                    .map(zerofrom::ZeroFrom::zero_from)
                    .map(DataPayload::from_owned)
                    .map(|payload| DataResponse {
                        metadata: Default::default(),
                        payload: Some(payload),
                    })
                    .ok_or_else(|| DataErrorKind::MissingLocale.with_req(::icu_normalizer::provider::CanonicalCompositionsV1Marker::KEY, req))
            }
        }
        #[cfg(feature = "icu_normalizer")]
        impl DataProvider<::icu_normalizer::provider::CanonicalDecompositionDataV1Marker> for $provider {
            fn load(&self, req: DataRequest) -> Result<DataResponse<::icu_normalizer::provider::CanonicalDecompositionDataV1Marker>, DataError> {
                normalizer::nfd_v1::lookup(&req.locale)
                    .map(zerofrom::ZeroFrom::zero_from)
                    .map(DataPayload::from_owned)
                    .map(|payload| DataResponse {
                        metadata: Default::default(),
                        payload: Some(payload),
                    })
                    .ok_or_else(|| DataErrorKind::MissingLocale.with_req(::icu_normalizer::provider::CanonicalDecompositionDataV1Marker::KEY, req))
            }
        }
        #[cfg(feature = "icu_normalizer")]
        impl DataProvider<::icu_normalizer::provider::CanonicalDecompositionTablesV1Marker> for $provider {
            fn load(&self, req: DataRequest) -> Result<DataResponse<::icu_normalizer::provider::CanonicalDecompositionTablesV1Marker>, DataError> {
                normalizer::nfdex_v1::lookup(&req.locale)
                    .map(zerofrom::ZeroFrom::zero_from)
                    .map(DataPayload::from_owned)
                    .map(|payload| DataResponse {
                        metadata: Default::default(),
                        payload: Some(payload),
                    })
                    .ok_or_else(|| DataErrorKind::MissingLocale.with_req(::icu_normalizer::provider::CanonicalDecompositionTablesV1Marker::KEY, req))
            }
        }
        #[cfg(feature = "icu_normalizer")]
        impl DataProvider<::icu_normalizer::provider::CompatibilityDecompositionSupplementV1Marker> for $provider {
            fn load(
                &self,
                req: DataRequest,
            ) -> Result<DataResponse<::icu_normalizer::provider::CompatibilityDecompositionSupplementV1Marker>, DataError> {
                normalizer::nfkd_v1::lookup(&req.locale)
                    .map(zerofrom::ZeroFrom::zero_from)
                    .map(DataPayload::from_owned)
                    .map(|payload| DataResponse {
                        metadata: Default::default(),
                        payload: Some(payload),
                    })
                    .ok_or_else(|| {
                        DataErrorKind::MissingLocale.with_req(::icu_normalizer::provider::CompatibilityDecompositionSupplementV1Marker::KEY, req)
                    })
            }
        }
        #[cfg(feature = "icu_normalizer")]
        impl DataProvider<::icu_normalizer::provider::CompatibilityDecompositionTablesV1Marker> for $provider {
            fn load(
                &self,
                req: DataRequest,
            ) -> Result<DataResponse<::icu_normalizer::provider::CompatibilityDecompositionTablesV1Marker>, DataError> {
                normalizer::nfkdex_v1::lookup(&req.locale)
                    .map(zerofrom::ZeroFrom::zero_from)
                    .map(DataPayload::from_owned)
                    .map(|payload| DataResponse {
                        metadata: Default::default(),
                        payload: Some(payload),
                    })
                    .ok_or_else(|| {
                        DataErrorKind::MissingLocale.with_req(::icu_normalizer::provider::CompatibilityDecompositionTablesV1Marker::KEY, req)
                    })
            }
        }
        #[cfg(feature = "icu_normalizer")]
        impl DataProvider<::icu_normalizer::provider::NonRecursiveDecompositionSupplementV1Marker> for $provider {
            fn load(
                &self,
                req: DataRequest,
            ) -> Result<DataResponse<::icu_normalizer::provider::NonRecursiveDecompositionSupplementV1Marker>, DataError> {
                normalizer::decomp_v1::lookup(&req.locale)
                    .map(zerofrom::ZeroFrom::zero_from)
                    .map(DataPayload::from_owned)
                    .map(|payload| DataResponse {
                        metadata: Default::default(),
                        payload: Some(payload),
                    })
                    .ok_or_else(|| {
                        DataErrorKind::MissingLocale.with_req(::icu_normalizer::provider::NonRecursiveDecompositionSupplementV1Marker::KEY, req)
                    })
            }
        }
        #[cfg(feature = "icu_normalizer")]
        impl DataProvider<::icu_normalizer::provider::Uts46DecompositionSupplementV1Marker> for $provider {
            fn load(&self, req: DataRequest) -> Result<DataResponse<::icu_normalizer::provider::Uts46DecompositionSupplementV1Marker>, DataError> {
                normalizer::uts46d_v1::lookup(&req.locale)
                    .map(zerofrom::ZeroFrom::zero_from)
                    .map(DataPayload::from_owned)
                    .map(|payload| DataResponse {
                        metadata: Default::default(),
                        payload: Some(payload),
                    })
                    .ok_or_else(|| DataErrorKind::MissingLocale.with_req(::icu_normalizer::provider::Uts46DecompositionSupplementV1Marker::KEY, req))
            }
        }
        #[cfg(feature = "icu_plurals")]
        impl DataProvider<::icu_plurals::provider::CardinalV1Marker> for $provider {
            fn load(&self, req: DataRequest) -> Result<DataResponse<::icu_plurals::provider::CardinalV1Marker>, DataError> {
                plurals::cardinal_v1::lookup(&req.locale)
                    .map(zerofrom::ZeroFrom::zero_from)
                    .map(DataPayload::from_owned)
                    .map(|payload| DataResponse {
                        metadata: Default::default(),
                        payload: Some(payload),
                    })
                    .ok_or_else(|| DataErrorKind::MissingLocale.with_req(::icu_plurals::provider::CardinalV1Marker::KEY, req))
            }
        }
        #[cfg(feature = "icu_plurals")]
        impl DataProvider<::icu_plurals::provider::OrdinalV1Marker> for $provider {
            fn load(&self, req: DataRequest) -> Result<DataResponse<::icu_plurals::provider::OrdinalV1Marker>, DataError> {
                plurals::ordinal_v1::lookup(&req.locale)
                    .map(zerofrom::ZeroFrom::zero_from)
                    .map(DataPayload::from_owned)
                    .map(|payload| DataResponse {
                        metadata: Default::default(),
                        payload: Some(payload),
                    })
                    .ok_or_else(|| DataErrorKind::MissingLocale.with_req(::icu_plurals::provider::OrdinalV1Marker::KEY, req))
            }
        }
        #[cfg(feature = "icu_properties")]
        impl DataProvider<::icu_properties::provider::AlphabeticV1Marker> for $provider {
            fn load(&self, req: DataRequest) -> Result<DataResponse<::icu_properties::provider::AlphabeticV1Marker>, DataError> {
                props::alpha_v1::lookup(&req.locale)
                    .map(zerofrom::ZeroFrom::zero_from)
                    .map(DataPayload::from_owned)
                    .map(|payload| DataResponse {
                        metadata: Default::default(),
                        payload: Some(payload),
                    })
                    .ok_or_else(|| DataErrorKind::MissingLocale.with_req(::icu_properties::provider::AlphabeticV1Marker::KEY, req))
            }
        }
        #[cfg(feature = "icu_properties")]
        impl DataProvider<::icu_properties::provider::AsciiHexDigitV1Marker> for $provider {
            fn load(&self, req: DataRequest) -> Result<DataResponse<::icu_properties::provider::AsciiHexDigitV1Marker>, DataError> {
                props::ahex_v1::lookup(&req.locale)
                    .map(zerofrom::ZeroFrom::zero_from)
                    .map(DataPayload::from_owned)
                    .map(|payload| DataResponse {
                        metadata: Default::default(),
                        payload: Some(payload),
                    })
                    .ok_or_else(|| DataErrorKind::MissingLocale.with_req(::icu_properties::provider::AsciiHexDigitV1Marker::KEY, req))
            }
        }
        #[cfg(feature = "icu_properties")]
        impl DataProvider<::icu_properties::provider::BasicEmojiV1Marker> for $provider {
            fn load(&self, req: DataRequest) -> Result<DataResponse<::icu_properties::provider::BasicEmojiV1Marker>, DataError> {
                props::basic_emoji_v1::lookup(&req.locale)
                    .map(zerofrom::ZeroFrom::zero_from)
                    .map(DataPayload::from_owned)
                    .map(|payload| DataResponse {
                        metadata: Default::default(),
                        payload: Some(payload),
                    })
                    .ok_or_else(|| DataErrorKind::MissingLocale.with_req(::icu_properties::provider::BasicEmojiV1Marker::KEY, req))
            }
        }
        #[cfg(feature = "icu_properties")]
        impl DataProvider<::icu_properties::provider::BidiClassV1Marker> for $provider {
            fn load(&self, req: DataRequest) -> Result<DataResponse<::icu_properties::provider::BidiClassV1Marker>, DataError> {
                props::bc_v1::lookup(&req.locale)
                    .map(zerofrom::ZeroFrom::zero_from)
                    .map(DataPayload::from_owned)
                    .map(|payload| DataResponse {
                        metadata: Default::default(),
                        payload: Some(payload),
                    })
                    .ok_or_else(|| DataErrorKind::MissingLocale.with_req(::icu_properties::provider::BidiClassV1Marker::KEY, req))
            }
        }
        #[cfg(feature = "icu_properties")]
        impl DataProvider<::icu_properties::provider::BidiControlV1Marker> for $provider {
            fn load(&self, req: DataRequest) -> Result<DataResponse<::icu_properties::provider::BidiControlV1Marker>, DataError> {
                props::bidi_c_v1::lookup(&req.locale)
                    .map(zerofrom::ZeroFrom::zero_from)
                    .map(DataPayload::from_owned)
                    .map(|payload| DataResponse {
                        metadata: Default::default(),
                        payload: Some(payload),
                    })
                    .ok_or_else(|| DataErrorKind::MissingLocale.with_req(::icu_properties::provider::BidiControlV1Marker::KEY, req))
            }
        }
        #[cfg(feature = "icu_properties")]
        impl DataProvider<::icu_properties::provider::BidiMirroredV1Marker> for $provider {
            fn load(&self, req: DataRequest) -> Result<DataResponse<::icu_properties::provider::BidiMirroredV1Marker>, DataError> {
                props::bidi_m_v1::lookup(&req.locale)
                    .map(zerofrom::ZeroFrom::zero_from)
                    .map(DataPayload::from_owned)
                    .map(|payload| DataResponse {
                        metadata: Default::default(),
                        payload: Some(payload),
                    })
                    .ok_or_else(|| DataErrorKind::MissingLocale.with_req(::icu_properties::provider::BidiMirroredV1Marker::KEY, req))
            }
        }
        #[cfg(feature = "icu_properties")]
        impl DataProvider<::icu_properties::provider::CanonicalCombiningClassV1Marker> for $provider {
            fn load(&self, req: DataRequest) -> Result<DataResponse<::icu_properties::provider::CanonicalCombiningClassV1Marker>, DataError> {
                props::ccc_v1::lookup(&req.locale)
                    .map(zerofrom::ZeroFrom::zero_from)
                    .map(DataPayload::from_owned)
                    .map(|payload| DataResponse {
                        metadata: Default::default(),
                        payload: Some(payload),
                    })
                    .ok_or_else(|| DataErrorKind::MissingLocale.with_req(::icu_properties::provider::CanonicalCombiningClassV1Marker::KEY, req))
            }
        }
        #[cfg(feature = "icu_properties")]
        impl DataProvider<::icu_properties::provider::CaseIgnorableV1Marker> for $provider {
            fn load(&self, req: DataRequest) -> Result<DataResponse<::icu_properties::provider::CaseIgnorableV1Marker>, DataError> {
                props::ci_v1::lookup(&req.locale)
                    .map(zerofrom::ZeroFrom::zero_from)
                    .map(DataPayload::from_owned)
                    .map(|payload| DataResponse {
                        metadata: Default::default(),
                        payload: Some(payload),
                    })
                    .ok_or_else(|| DataErrorKind::MissingLocale.with_req(::icu_properties::provider::CaseIgnorableV1Marker::KEY, req))
            }
        }
        #[cfg(feature = "icu_properties")]
        impl DataProvider<::icu_properties::provider::CasedV1Marker> for $provider {
            fn load(&self, req: DataRequest) -> Result<DataResponse<::icu_properties::provider::CasedV1Marker>, DataError> {
                props::cased_v1::lookup(&req.locale)
                    .map(zerofrom::ZeroFrom::zero_from)
                    .map(DataPayload::from_owned)
                    .map(|payload| DataResponse {
                        metadata: Default::default(),
                        payload: Some(payload),
                    })
                    .ok_or_else(|| DataErrorKind::MissingLocale.with_req(::icu_properties::provider::CasedV1Marker::KEY, req))
            }
        }
        #[cfg(feature = "icu_properties")]
        impl DataProvider<::icu_properties::provider::ChangesWhenCasefoldedV1Marker> for $provider {
            fn load(&self, req: DataRequest) -> Result<DataResponse<::icu_properties::provider::ChangesWhenCasefoldedV1Marker>, DataError> {
                props::cwcf_v1::lookup(&req.locale)
                    .map(zerofrom::ZeroFrom::zero_from)
                    .map(DataPayload::from_owned)
                    .map(|payload| DataResponse {
                        metadata: Default::default(),
                        payload: Some(payload),
                    })
                    .ok_or_else(|| DataErrorKind::MissingLocale.with_req(::icu_properties::provider::ChangesWhenCasefoldedV1Marker::KEY, req))
            }
        }
        #[cfg(feature = "icu_properties")]
        impl DataProvider<::icu_properties::provider::ChangesWhenLowercasedV1Marker> for $provider {
            fn load(&self, req: DataRequest) -> Result<DataResponse<::icu_properties::provider::ChangesWhenLowercasedV1Marker>, DataError> {
                props::cwl_v1::lookup(&req.locale)
                    .map(zerofrom::ZeroFrom::zero_from)
                    .map(DataPayload::from_owned)
                    .map(|payload| DataResponse {
                        metadata: Default::default(),
                        payload: Some(payload),
                    })
                    .ok_or_else(|| DataErrorKind::MissingLocale.with_req(::icu_properties::provider::ChangesWhenLowercasedV1Marker::KEY, req))
            }
        }
        #[cfg(feature = "icu_properties")]
        impl DataProvider<::icu_properties::provider::ChangesWhenNfkcCasefoldedV1Marker> for $provider {
            fn load(&self, req: DataRequest) -> Result<DataResponse<::icu_properties::provider::ChangesWhenNfkcCasefoldedV1Marker>, DataError> {
                props::cwkcf_v1::lookup(&req.locale)
                    .map(zerofrom::ZeroFrom::zero_from)
                    .map(DataPayload::from_owned)
                    .map(|payload| DataResponse {
                        metadata: Default::default(),
                        payload: Some(payload),
                    })
                    .ok_or_else(|| DataErrorKind::MissingLocale.with_req(::icu_properties::provider::ChangesWhenNfkcCasefoldedV1Marker::KEY, req))
            }
        }
        #[cfg(feature = "icu_properties")]
        impl DataProvider<::icu_properties::provider::ChangesWhenTitlecasedV1Marker> for $provider {
            fn load(&self, req: DataRequest) -> Result<DataResponse<::icu_properties::provider::ChangesWhenTitlecasedV1Marker>, DataError> {
                props::cwt_v1::lookup(&req.locale)
                    .map(zerofrom::ZeroFrom::zero_from)
                    .map(DataPayload::from_owned)
                    .map(|payload| DataResponse {
                        metadata: Default::default(),
                        payload: Some(payload),
                    })
                    .ok_or_else(|| DataErrorKind::MissingLocale.with_req(::icu_properties::provider::ChangesWhenTitlecasedV1Marker::KEY, req))
            }
        }
        #[cfg(feature = "icu_properties")]
        impl DataProvider<::icu_properties::provider::ChangesWhenUppercasedV1Marker> for $provider {
            fn load(&self, req: DataRequest) -> Result<DataResponse<::icu_properties::provider::ChangesWhenUppercasedV1Marker>, DataError> {
                props::cwu_v1::lookup(&req.locale)
                    .map(zerofrom::ZeroFrom::zero_from)
                    .map(DataPayload::from_owned)
                    .map(|payload| DataResponse {
                        metadata: Default::default(),
                        payload: Some(payload),
                    })
                    .ok_or_else(|| DataErrorKind::MissingLocale.with_req(::icu_properties::provider::ChangesWhenUppercasedV1Marker::KEY, req))
            }
        }
        #[cfg(feature = "icu_properties")]
        impl DataProvider<::icu_properties::provider::DashV1Marker> for $provider {
            fn load(&self, req: DataRequest) -> Result<DataResponse<::icu_properties::provider::DashV1Marker>, DataError> {
                props::dash_v1::lookup(&req.locale)
                    .map(zerofrom::ZeroFrom::zero_from)
                    .map(DataPayload::from_owned)
                    .map(|payload| DataResponse {
                        metadata: Default::default(),
                        payload: Some(payload),
                    })
                    .ok_or_else(|| DataErrorKind::MissingLocale.with_req(::icu_properties::provider::DashV1Marker::KEY, req))
            }
        }
        #[cfg(feature = "icu_properties")]
        impl DataProvider<::icu_properties::provider::DefaultIgnorableCodePointV1Marker> for $provider {
            fn load(&self, req: DataRequest) -> Result<DataResponse<::icu_properties::provider::DefaultIgnorableCodePointV1Marker>, DataError> {
                props::di_v1::lookup(&req.locale)
                    .map(zerofrom::ZeroFrom::zero_from)
                    .map(DataPayload::from_owned)
                    .map(|payload| DataResponse {
                        metadata: Default::default(),
                        payload: Some(payload),
                    })
                    .ok_or_else(|| DataErrorKind::MissingLocale.with_req(::icu_properties::provider::DefaultIgnorableCodePointV1Marker::KEY, req))
            }
        }
        #[cfg(feature = "icu_properties")]
        impl DataProvider<::icu_properties::provider::DeprecatedV1Marker> for $provider {
            fn load(&self, req: DataRequest) -> Result<DataResponse<::icu_properties::provider::DeprecatedV1Marker>, DataError> {
                props::dep_v1::lookup(&req.locale)
                    .map(zerofrom::ZeroFrom::zero_from)
                    .map(DataPayload::from_owned)
                    .map(|payload| DataResponse {
                        metadata: Default::default(),
                        payload: Some(payload),
                    })
                    .ok_or_else(|| DataErrorKind::MissingLocale.with_req(::icu_properties::provider::DeprecatedV1Marker::KEY, req))
            }
        }
        #[cfg(feature = "icu_properties")]
        impl DataProvider<::icu_properties::provider::DiacriticV1Marker> for $provider {
            fn load(&self, req: DataRequest) -> Result<DataResponse<::icu_properties::provider::DiacriticV1Marker>, DataError> {
                props::dia_v1::lookup(&req.locale)
                    .map(zerofrom::ZeroFrom::zero_from)
                    .map(DataPayload::from_owned)
                    .map(|payload| DataResponse {
                        metadata: Default::default(),
                        payload: Some(payload),
                    })
                    .ok_or_else(|| DataErrorKind::MissingLocale.with_req(::icu_properties::provider::DiacriticV1Marker::KEY, req))
            }
        }
        #[cfg(feature = "icu_properties")]
        impl DataProvider<::icu_properties::provider::EastAsianWidthV1Marker> for $provider {
            fn load(&self, req: DataRequest) -> Result<DataResponse<::icu_properties::provider::EastAsianWidthV1Marker>, DataError> {
                props::ea_v1::lookup(&req.locale)
                    .map(zerofrom::ZeroFrom::zero_from)
                    .map(DataPayload::from_owned)
                    .map(|payload| DataResponse {
                        metadata: Default::default(),
                        payload: Some(payload),
                    })
                    .ok_or_else(|| DataErrorKind::MissingLocale.with_req(::icu_properties::provider::EastAsianWidthV1Marker::KEY, req))
            }
        }
        #[cfg(feature = "icu_properties")]
        impl DataProvider<::icu_properties::provider::EmojiComponentV1Marker> for $provider {
            fn load(&self, req: DataRequest) -> Result<DataResponse<::icu_properties::provider::EmojiComponentV1Marker>, DataError> {
                props::ecomp_v1::lookup(&req.locale)
                    .map(zerofrom::ZeroFrom::zero_from)
                    .map(DataPayload::from_owned)
                    .map(|payload| DataResponse {
                        metadata: Default::default(),
                        payload: Some(payload),
                    })
                    .ok_or_else(|| DataErrorKind::MissingLocale.with_req(::icu_properties::provider::EmojiComponentV1Marker::KEY, req))
            }
        }
        #[cfg(feature = "icu_properties")]
        impl DataProvider<::icu_properties::provider::EmojiModifierBaseV1Marker> for $provider {
            fn load(&self, req: DataRequest) -> Result<DataResponse<::icu_properties::provider::EmojiModifierBaseV1Marker>, DataError> {
                props::ebase_v1::lookup(&req.locale)
                    .map(zerofrom::ZeroFrom::zero_from)
                    .map(DataPayload::from_owned)
                    .map(|payload| DataResponse {
                        metadata: Default::default(),
                        payload: Some(payload),
                    })
                    .ok_or_else(|| DataErrorKind::MissingLocale.with_req(::icu_properties::provider::EmojiModifierBaseV1Marker::KEY, req))
            }
        }
        #[cfg(feature = "icu_properties")]
        impl DataProvider<::icu_properties::provider::EmojiModifierV1Marker> for $provider {
            fn load(&self, req: DataRequest) -> Result<DataResponse<::icu_properties::provider::EmojiModifierV1Marker>, DataError> {
                props::emod_v1::lookup(&req.locale)
                    .map(zerofrom::ZeroFrom::zero_from)
                    .map(DataPayload::from_owned)
                    .map(|payload| DataResponse {
                        metadata: Default::default(),
                        payload: Some(payload),
                    })
                    .ok_or_else(|| DataErrorKind::MissingLocale.with_req(::icu_properties::provider::EmojiModifierV1Marker::KEY, req))
            }
        }
        #[cfg(feature = "icu_properties")]
        impl DataProvider<::icu_properties::provider::EmojiPresentationV1Marker> for $provider {
            fn load(&self, req: DataRequest) -> Result<DataResponse<::icu_properties::provider::EmojiPresentationV1Marker>, DataError> {
                props::epres_v1::lookup(&req.locale)
                    .map(zerofrom::ZeroFrom::zero_from)
                    .map(DataPayload::from_owned)
                    .map(|payload| DataResponse {
                        metadata: Default::default(),
                        payload: Some(payload),
                    })
                    .ok_or_else(|| DataErrorKind::MissingLocale.with_req(::icu_properties::provider::EmojiPresentationV1Marker::KEY, req))
            }
        }
        #[cfg(feature = "icu_properties")]
        impl DataProvider<::icu_properties::provider::EmojiV1Marker> for $provider {
            fn load(&self, req: DataRequest) -> Result<DataResponse<::icu_properties::provider::EmojiV1Marker>, DataError> {
                props::emoji_v1::lookup(&req.locale)
                    .map(zerofrom::ZeroFrom::zero_from)
                    .map(DataPayload::from_owned)
                    .map(|payload| DataResponse {
                        metadata: Default::default(),
                        payload: Some(payload),
                    })
                    .ok_or_else(|| DataErrorKind::MissingLocale.with_req(::icu_properties::provider::EmojiV1Marker::KEY, req))
            }
        }
        #[cfg(feature = "icu_properties")]
        impl DataProvider<::icu_properties::provider::ExemplarCharactersAuxiliaryV1Marker> for $provider {
            fn load(&self, req: DataRequest) -> Result<DataResponse<::icu_properties::provider::ExemplarCharactersAuxiliaryV1Marker>, DataError> {
                props::exemplarchars::auxiliary_v1::lookup(&req.locale)
                    .map(zerofrom::ZeroFrom::zero_from)
                    .map(DataPayload::from_owned)
                    .map(|payload| DataResponse {
                        metadata: Default::default(),
                        payload: Some(payload),
                    })
                    .ok_or_else(|| DataErrorKind::MissingLocale.with_req(::icu_properties::provider::ExemplarCharactersAuxiliaryV1Marker::KEY, req))
            }
        }
        #[cfg(feature = "icu_properties")]
        impl DataProvider<::icu_properties::provider::ExemplarCharactersIndexV1Marker> for $provider {
            fn load(&self, req: DataRequest) -> Result<DataResponse<::icu_properties::provider::ExemplarCharactersIndexV1Marker>, DataError> {
                props::exemplarchars::index_v1::lookup(&req.locale)
                    .map(zerofrom::ZeroFrom::zero_from)
                    .map(DataPayload::from_owned)
                    .map(|payload| DataResponse {
                        metadata: Default::default(),
                        payload: Some(payload),
                    })
                    .ok_or_else(|| DataErrorKind::MissingLocale.with_req(::icu_properties::provider::ExemplarCharactersIndexV1Marker::KEY, req))
            }
        }
        #[cfg(feature = "icu_properties")]
        impl DataProvider<::icu_properties::provider::ExemplarCharactersMainV1Marker> for $provider {
            fn load(&self, req: DataRequest) -> Result<DataResponse<::icu_properties::provider::ExemplarCharactersMainV1Marker>, DataError> {
                props::exemplarchars::main_v1::lookup(&req.locale)
                    .map(zerofrom::ZeroFrom::zero_from)
                    .map(DataPayload::from_owned)
                    .map(|payload| DataResponse {
                        metadata: Default::default(),
                        payload: Some(payload),
                    })
                    .ok_or_else(|| DataErrorKind::MissingLocale.with_req(::icu_properties::provider::ExemplarCharactersMainV1Marker::KEY, req))
            }
        }
        #[cfg(feature = "icu_properties")]
        impl DataProvider<::icu_properties::provider::ExemplarCharactersNumbersV1Marker> for $provider {
            fn load(&self, req: DataRequest) -> Result<DataResponse<::icu_properties::provider::ExemplarCharactersNumbersV1Marker>, DataError> {
                props::exemplarchars::numbers_v1::lookup(&req.locale)
                    .map(zerofrom::ZeroFrom::zero_from)
                    .map(DataPayload::from_owned)
                    .map(|payload| DataResponse {
                        metadata: Default::default(),
                        payload: Some(payload),
                    })
                    .ok_or_else(|| DataErrorKind::MissingLocale.with_req(::icu_properties::provider::ExemplarCharactersNumbersV1Marker::KEY, req))
            }
        }
        #[cfg(feature = "icu_properties")]
        impl DataProvider<::icu_properties::provider::ExemplarCharactersPunctuationV1Marker> for $provider {
            fn load(&self, req: DataRequest) -> Result<DataResponse<::icu_properties::provider::ExemplarCharactersPunctuationV1Marker>, DataError> {
                props::exemplarchars::punctuation_v1::lookup(&req.locale)
                    .map(zerofrom::ZeroFrom::zero_from)
                    .map(DataPayload::from_owned)
                    .map(|payload| DataResponse {
                        metadata: Default::default(),
                        payload: Some(payload),
                    })
                    .ok_or_else(|| DataErrorKind::MissingLocale.with_req(::icu_properties::provider::ExemplarCharactersPunctuationV1Marker::KEY, req))
            }
        }
        #[cfg(feature = "icu_properties")]
        impl DataProvider<::icu_properties::provider::ExtendedPictographicV1Marker> for $provider {
            fn load(&self, req: DataRequest) -> Result<DataResponse<::icu_properties::provider::ExtendedPictographicV1Marker>, DataError> {
                props::extpict_v1::lookup(&req.locale)
                    .map(zerofrom::ZeroFrom::zero_from)
                    .map(DataPayload::from_owned)
                    .map(|payload| DataResponse {
                        metadata: Default::default(),
                        payload: Some(payload),
                    })
                    .ok_or_else(|| DataErrorKind::MissingLocale.with_req(::icu_properties::provider::ExtendedPictographicV1Marker::KEY, req))
            }
        }
        #[cfg(feature = "icu_properties")]
        impl DataProvider<::icu_properties::provider::ExtenderV1Marker> for $provider {
            fn load(&self, req: DataRequest) -> Result<DataResponse<::icu_properties::provider::ExtenderV1Marker>, DataError> {
                props::ext_v1::lookup(&req.locale)
                    .map(zerofrom::ZeroFrom::zero_from)
                    .map(DataPayload::from_owned)
                    .map(|payload| DataResponse {
                        metadata: Default::default(),
                        payload: Some(payload),
                    })
                    .ok_or_else(|| DataErrorKind::MissingLocale.with_req(::icu_properties::provider::ExtenderV1Marker::KEY, req))
            }
        }
        #[cfg(feature = "icu_properties")]
        impl DataProvider<::icu_properties::provider::GeneralCategoryV1Marker> for $provider {
            fn load(&self, req: DataRequest) -> Result<DataResponse<::icu_properties::provider::GeneralCategoryV1Marker>, DataError> {
                props::gc_v1::lookup(&req.locale)
                    .map(zerofrom::ZeroFrom::zero_from)
                    .map(DataPayload::from_owned)
                    .map(|payload| DataResponse {
                        metadata: Default::default(),
                        payload: Some(payload),
                    })
                    .ok_or_else(|| DataErrorKind::MissingLocale.with_req(::icu_properties::provider::GeneralCategoryV1Marker::KEY, req))
            }
        }
        #[cfg(feature = "icu_properties")]
        impl DataProvider<::icu_properties::provider::GraphemeBaseV1Marker> for $provider {
            fn load(&self, req: DataRequest) -> Result<DataResponse<::icu_properties::provider::GraphemeBaseV1Marker>, DataError> {
                props::gr_base_v1::lookup(&req.locale)
                    .map(zerofrom::ZeroFrom::zero_from)
                    .map(DataPayload::from_owned)
                    .map(|payload| DataResponse {
                        metadata: Default::default(),
                        payload: Some(payload),
                    })
                    .ok_or_else(|| DataErrorKind::MissingLocale.with_req(::icu_properties::provider::GraphemeBaseV1Marker::KEY, req))
            }
        }
        #[cfg(feature = "icu_properties")]
        impl DataProvider<::icu_properties::provider::GraphemeClusterBreakV1Marker> for $provider {
            fn load(&self, req: DataRequest) -> Result<DataResponse<::icu_properties::provider::GraphemeClusterBreakV1Marker>, DataError> {
                props::gcb_v1::lookup(&req.locale)
                    .map(zerofrom::ZeroFrom::zero_from)
                    .map(DataPayload::from_owned)
                    .map(|payload| DataResponse {
                        metadata: Default::default(),
                        payload: Some(payload),
                    })
                    .ok_or_else(|| DataErrorKind::MissingLocale.with_req(::icu_properties::provider::GraphemeClusterBreakV1Marker::KEY, req))
            }
        }
        #[cfg(feature = "icu_properties")]
        impl DataProvider<::icu_properties::provider::GraphemeExtendV1Marker> for $provider {
            fn load(&self, req: DataRequest) -> Result<DataResponse<::icu_properties::provider::GraphemeExtendV1Marker>, DataError> {
                props::gr_ext_v1::lookup(&req.locale)
                    .map(zerofrom::ZeroFrom::zero_from)
                    .map(DataPayload::from_owned)
                    .map(|payload| DataResponse {
                        metadata: Default::default(),
                        payload: Some(payload),
                    })
                    .ok_or_else(|| DataErrorKind::MissingLocale.with_req(::icu_properties::provider::GraphemeExtendV1Marker::KEY, req))
            }
        }
        #[cfg(feature = "icu_properties")]
        impl DataProvider<::icu_properties::provider::HexDigitV1Marker> for $provider {
            fn load(&self, req: DataRequest) -> Result<DataResponse<::icu_properties::provider::HexDigitV1Marker>, DataError> {
                props::hex_v1::lookup(&req.locale)
                    .map(zerofrom::ZeroFrom::zero_from)
                    .map(DataPayload::from_owned)
                    .map(|payload| DataResponse {
                        metadata: Default::default(),
                        payload: Some(payload),
                    })
                    .ok_or_else(|| DataErrorKind::MissingLocale.with_req(::icu_properties::provider::HexDigitV1Marker::KEY, req))
            }
        }
        #[cfg(feature = "icu_properties")]
        impl DataProvider<::icu_properties::provider::IdContinueV1Marker> for $provider {
            fn load(&self, req: DataRequest) -> Result<DataResponse<::icu_properties::provider::IdContinueV1Marker>, DataError> {
                props::idc_v1::lookup(&req.locale)
                    .map(zerofrom::ZeroFrom::zero_from)
                    .map(DataPayload::from_owned)
                    .map(|payload| DataResponse {
                        metadata: Default::default(),
                        payload: Some(payload),
                    })
                    .ok_or_else(|| DataErrorKind::MissingLocale.with_req(::icu_properties::provider::IdContinueV1Marker::KEY, req))
            }
        }
        #[cfg(feature = "icu_properties")]
        impl DataProvider<::icu_properties::provider::IdStartV1Marker> for $provider {
            fn load(&self, req: DataRequest) -> Result<DataResponse<::icu_properties::provider::IdStartV1Marker>, DataError> {
                props::ids_v1::lookup(&req.locale)
                    .map(zerofrom::ZeroFrom::zero_from)
                    .map(DataPayload::from_owned)
                    .map(|payload| DataResponse {
                        metadata: Default::default(),
                        payload: Some(payload),
                    })
                    .ok_or_else(|| DataErrorKind::MissingLocale.with_req(::icu_properties::provider::IdStartV1Marker::KEY, req))
            }
        }
        #[cfg(feature = "icu_properties")]
        impl DataProvider<::icu_properties::provider::IdeographicV1Marker> for $provider {
            fn load(&self, req: DataRequest) -> Result<DataResponse<::icu_properties::provider::IdeographicV1Marker>, DataError> {
                props::ideo_v1::lookup(&req.locale)
                    .map(zerofrom::ZeroFrom::zero_from)
                    .map(DataPayload::from_owned)
                    .map(|payload| DataResponse {
                        metadata: Default::default(),
                        payload: Some(payload),
                    })
                    .ok_or_else(|| DataErrorKind::MissingLocale.with_req(::icu_properties::provider::IdeographicV1Marker::KEY, req))
            }
        }
        #[cfg(feature = "icu_properties")]
        impl DataProvider<::icu_properties::provider::IdsBinaryOperatorV1Marker> for $provider {
            fn load(&self, req: DataRequest) -> Result<DataResponse<::icu_properties::provider::IdsBinaryOperatorV1Marker>, DataError> {
                props::idsb_v1::lookup(&req.locale)
                    .map(zerofrom::ZeroFrom::zero_from)
                    .map(DataPayload::from_owned)
                    .map(|payload| DataResponse {
                        metadata: Default::default(),
                        payload: Some(payload),
                    })
                    .ok_or_else(|| DataErrorKind::MissingLocale.with_req(::icu_properties::provider::IdsBinaryOperatorV1Marker::KEY, req))
            }
        }
        #[cfg(feature = "icu_properties")]
        impl DataProvider<::icu_properties::provider::IdsTrinaryOperatorV1Marker> for $provider {
            fn load(&self, req: DataRequest) -> Result<DataResponse<::icu_properties::provider::IdsTrinaryOperatorV1Marker>, DataError> {
                props::idst_v1::lookup(&req.locale)
                    .map(zerofrom::ZeroFrom::zero_from)
                    .map(DataPayload::from_owned)
                    .map(|payload| DataResponse {
                        metadata: Default::default(),
                        payload: Some(payload),
                    })
                    .ok_or_else(|| DataErrorKind::MissingLocale.with_req(::icu_properties::provider::IdsTrinaryOperatorV1Marker::KEY, req))
            }
        }
        #[cfg(feature = "icu_properties")]
        impl DataProvider<::icu_properties::provider::JoinControlV1Marker> for $provider {
            fn load(&self, req: DataRequest) -> Result<DataResponse<::icu_properties::provider::JoinControlV1Marker>, DataError> {
                props::join_c_v1::lookup(&req.locale)
                    .map(zerofrom::ZeroFrom::zero_from)
                    .map(DataPayload::from_owned)
                    .map(|payload| DataResponse {
                        metadata: Default::default(),
                        payload: Some(payload),
                    })
                    .ok_or_else(|| DataErrorKind::MissingLocale.with_req(::icu_properties::provider::JoinControlV1Marker::KEY, req))
            }
        }
        #[cfg(feature = "icu_properties")]
        impl DataProvider<::icu_properties::provider::LineBreakV1Marker> for $provider {
            fn load(&self, req: DataRequest) -> Result<DataResponse<::icu_properties::provider::LineBreakV1Marker>, DataError> {
                props::lb_v1::lookup(&req.locale)
                    .map(zerofrom::ZeroFrom::zero_from)
                    .map(DataPayload::from_owned)
                    .map(|payload| DataResponse {
                        metadata: Default::default(),
                        payload: Some(payload),
                    })
                    .ok_or_else(|| DataErrorKind::MissingLocale.with_req(::icu_properties::provider::LineBreakV1Marker::KEY, req))
            }
        }
        #[cfg(feature = "icu_properties")]
        impl DataProvider<::icu_properties::provider::LogicalOrderExceptionV1Marker> for $provider {
            fn load(&self, req: DataRequest) -> Result<DataResponse<::icu_properties::provider::LogicalOrderExceptionV1Marker>, DataError> {
                props::loe_v1::lookup(&req.locale)
                    .map(zerofrom::ZeroFrom::zero_from)
                    .map(DataPayload::from_owned)
                    .map(|payload| DataResponse {
                        metadata: Default::default(),
                        payload: Some(payload),
                    })
                    .ok_or_else(|| DataErrorKind::MissingLocale.with_req(::icu_properties::provider::LogicalOrderExceptionV1Marker::KEY, req))
            }
        }
        #[cfg(feature = "icu_properties")]
        impl DataProvider<::icu_properties::provider::LowercaseV1Marker> for $provider {
            fn load(&self, req: DataRequest) -> Result<DataResponse<::icu_properties::provider::LowercaseV1Marker>, DataError> {
                props::lower_v1::lookup(&req.locale)
                    .map(zerofrom::ZeroFrom::zero_from)
                    .map(DataPayload::from_owned)
                    .map(|payload| DataResponse {
                        metadata: Default::default(),
                        payload: Some(payload),
                    })
                    .ok_or_else(|| DataErrorKind::MissingLocale.with_req(::icu_properties::provider::LowercaseV1Marker::KEY, req))
            }
        }
        #[cfg(feature = "icu_properties")]
        impl DataProvider<::icu_properties::provider::MathV1Marker> for $provider {
            fn load(&self, req: DataRequest) -> Result<DataResponse<::icu_properties::provider::MathV1Marker>, DataError> {
                props::math_v1::lookup(&req.locale)
                    .map(zerofrom::ZeroFrom::zero_from)
                    .map(DataPayload::from_owned)
                    .map(|payload| DataResponse {
                        metadata: Default::default(),
                        payload: Some(payload),
                    })
                    .ok_or_else(|| DataErrorKind::MissingLocale.with_req(::icu_properties::provider::MathV1Marker::KEY, req))
            }
        }
        #[cfg(feature = "icu_properties")]
        impl DataProvider<::icu_properties::provider::NoncharacterCodePointV1Marker> for $provider {
            fn load(&self, req: DataRequest) -> Result<DataResponse<::icu_properties::provider::NoncharacterCodePointV1Marker>, DataError> {
                props::nchar_v1::lookup(&req.locale)
                    .map(zerofrom::ZeroFrom::zero_from)
                    .map(DataPayload::from_owned)
                    .map(|payload| DataResponse {
                        metadata: Default::default(),
                        payload: Some(payload),
                    })
                    .ok_or_else(|| DataErrorKind::MissingLocale.with_req(::icu_properties::provider::NoncharacterCodePointV1Marker::KEY, req))
            }
        }
        #[cfg(feature = "icu_properties")]
        impl DataProvider<::icu_properties::provider::PatternSyntaxV1Marker> for $provider {
            fn load(&self, req: DataRequest) -> Result<DataResponse<::icu_properties::provider::PatternSyntaxV1Marker>, DataError> {
                props::pat_syn_v1::lookup(&req.locale)
                    .map(zerofrom::ZeroFrom::zero_from)
                    .map(DataPayload::from_owned)
                    .map(|payload| DataResponse {
                        metadata: Default::default(),
                        payload: Some(payload),
                    })
                    .ok_or_else(|| DataErrorKind::MissingLocale.with_req(::icu_properties::provider::PatternSyntaxV1Marker::KEY, req))
            }
        }
        #[cfg(feature = "icu_properties")]
        impl DataProvider<::icu_properties::provider::PatternWhiteSpaceV1Marker> for $provider {
            fn load(&self, req: DataRequest) -> Result<DataResponse<::icu_properties::provider::PatternWhiteSpaceV1Marker>, DataError> {
                props::pat_ws_v1::lookup(&req.locale)
                    .map(zerofrom::ZeroFrom::zero_from)
                    .map(DataPayload::from_owned)
                    .map(|payload| DataResponse {
                        metadata: Default::default(),
                        payload: Some(payload),
                    })
                    .ok_or_else(|| DataErrorKind::MissingLocale.with_req(::icu_properties::provider::PatternWhiteSpaceV1Marker::KEY, req))
            }
        }
        #[cfg(feature = "icu_properties")]
        impl DataProvider<::icu_properties::provider::QuotationMarkV1Marker> for $provider {
            fn load(&self, req: DataRequest) -> Result<DataResponse<::icu_properties::provider::QuotationMarkV1Marker>, DataError> {
                props::qmark_v1::lookup(&req.locale)
                    .map(zerofrom::ZeroFrom::zero_from)
                    .map(DataPayload::from_owned)
                    .map(|payload| DataResponse {
                        metadata: Default::default(),
                        payload: Some(payload),
                    })
                    .ok_or_else(|| DataErrorKind::MissingLocale.with_req(::icu_properties::provider::QuotationMarkV1Marker::KEY, req))
            }
        }
        #[cfg(feature = "icu_properties")]
        impl DataProvider<::icu_properties::provider::RadicalV1Marker> for $provider {
            fn load(&self, req: DataRequest) -> Result<DataResponse<::icu_properties::provider::RadicalV1Marker>, DataError> {
                props::radical_v1::lookup(&req.locale)
                    .map(zerofrom::ZeroFrom::zero_from)
                    .map(DataPayload::from_owned)
                    .map(|payload| DataResponse {
                        metadata: Default::default(),
                        payload: Some(payload),
                    })
                    .ok_or_else(|| DataErrorKind::MissingLocale.with_req(::icu_properties::provider::RadicalV1Marker::KEY, req))
            }
        }
        #[cfg(feature = "icu_properties")]
        impl DataProvider<::icu_properties::provider::RegionalIndicatorV1Marker> for $provider {
            fn load(&self, req: DataRequest) -> Result<DataResponse<::icu_properties::provider::RegionalIndicatorV1Marker>, DataError> {
                props::ri_v1::lookup(&req.locale)
                    .map(zerofrom::ZeroFrom::zero_from)
                    .map(DataPayload::from_owned)
                    .map(|payload| DataResponse {
                        metadata: Default::default(),
                        payload: Some(payload),
                    })
                    .ok_or_else(|| DataErrorKind::MissingLocale.with_req(::icu_properties::provider::RegionalIndicatorV1Marker::KEY, req))
            }
        }
        #[cfg(feature = "icu_properties")]
        impl DataProvider<::icu_properties::provider::ScriptV1Marker> for $provider {
            fn load(&self, req: DataRequest) -> Result<DataResponse<::icu_properties::provider::ScriptV1Marker>, DataError> {
                props::sc_v1::lookup(&req.locale)
                    .map(zerofrom::ZeroFrom::zero_from)
                    .map(DataPayload::from_owned)
                    .map(|payload| DataResponse {
                        metadata: Default::default(),
                        payload: Some(payload),
                    })
                    .ok_or_else(|| DataErrorKind::MissingLocale.with_req(::icu_properties::provider::ScriptV1Marker::KEY, req))
            }
        }
        #[cfg(feature = "icu_properties")]
        impl DataProvider<::icu_properties::provider::ScriptWithExtensionsPropertyV1Marker> for $provider {
            fn load(&self, req: DataRequest) -> Result<DataResponse<::icu_properties::provider::ScriptWithExtensionsPropertyV1Marker>, DataError> {
                props::scx_v1::lookup(&req.locale)
                    .map(zerofrom::ZeroFrom::zero_from)
                    .map(DataPayload::from_owned)
                    .map(|payload| DataResponse {
                        metadata: Default::default(),
                        payload: Some(payload),
                    })
                    .ok_or_else(|| DataErrorKind::MissingLocale.with_req(::icu_properties::provider::ScriptWithExtensionsPropertyV1Marker::KEY, req))
            }
        }
        #[cfg(feature = "icu_properties")]
        impl DataProvider<::icu_properties::provider::SentenceBreakV1Marker> for $provider {
            fn load(&self, req: DataRequest) -> Result<DataResponse<::icu_properties::provider::SentenceBreakV1Marker>, DataError> {
                props::sb_v1::lookup(&req.locale)
                    .map(zerofrom::ZeroFrom::zero_from)
                    .map(DataPayload::from_owned)
                    .map(|payload| DataResponse {
                        metadata: Default::default(),
                        payload: Some(payload),
                    })
                    .ok_or_else(|| DataErrorKind::MissingLocale.with_req(::icu_properties::provider::SentenceBreakV1Marker::KEY, req))
            }
        }
        #[cfg(feature = "icu_properties")]
        impl DataProvider<::icu_properties::provider::SentenceTerminalV1Marker> for $provider {
            fn load(&self, req: DataRequest) -> Result<DataResponse<::icu_properties::provider::SentenceTerminalV1Marker>, DataError> {
                props::sterm_v1::lookup(&req.locale)
                    .map(zerofrom::ZeroFrom::zero_from)
                    .map(DataPayload::from_owned)
                    .map(|payload| DataResponse {
                        metadata: Default::default(),
                        payload: Some(payload),
                    })
                    .ok_or_else(|| DataErrorKind::MissingLocale.with_req(::icu_properties::provider::SentenceTerminalV1Marker::KEY, req))
            }
        }
        #[cfg(feature = "icu_properties")]
        impl DataProvider<::icu_properties::provider::SoftDottedV1Marker> for $provider {
            fn load(&self, req: DataRequest) -> Result<DataResponse<::icu_properties::provider::SoftDottedV1Marker>, DataError> {
                props::sd_v1::lookup(&req.locale)
                    .map(zerofrom::ZeroFrom::zero_from)
                    .map(DataPayload::from_owned)
                    .map(|payload| DataResponse {
                        metadata: Default::default(),
                        payload: Some(payload),
                    })
                    .ok_or_else(|| DataErrorKind::MissingLocale.with_req(::icu_properties::provider::SoftDottedV1Marker::KEY, req))
            }
        }
        #[cfg(feature = "icu_properties")]
        impl DataProvider<::icu_properties::provider::TerminalPunctuationV1Marker> for $provider {
            fn load(&self, req: DataRequest) -> Result<DataResponse<::icu_properties::provider::TerminalPunctuationV1Marker>, DataError> {
                props::term_v1::lookup(&req.locale)
                    .map(zerofrom::ZeroFrom::zero_from)
                    .map(DataPayload::from_owned)
                    .map(|payload| DataResponse {
                        metadata: Default::default(),
                        payload: Some(payload),
                    })
                    .ok_or_else(|| DataErrorKind::MissingLocale.with_req(::icu_properties::provider::TerminalPunctuationV1Marker::KEY, req))
            }
        }
        #[cfg(feature = "icu_properties")]
        impl DataProvider<::icu_properties::provider::UnifiedIdeographV1Marker> for $provider {
            fn load(&self, req: DataRequest) -> Result<DataResponse<::icu_properties::provider::UnifiedIdeographV1Marker>, DataError> {
                props::uideo_v1::lookup(&req.locale)
                    .map(zerofrom::ZeroFrom::zero_from)
                    .map(DataPayload::from_owned)
                    .map(|payload| DataResponse {
                        metadata: Default::default(),
                        payload: Some(payload),
                    })
                    .ok_or_else(|| DataErrorKind::MissingLocale.with_req(::icu_properties::provider::UnifiedIdeographV1Marker::KEY, req))
            }
        }
        #[cfg(feature = "icu_properties")]
        impl DataProvider<::icu_properties::provider::UppercaseV1Marker> for $provider {
            fn load(&self, req: DataRequest) -> Result<DataResponse<::icu_properties::provider::UppercaseV1Marker>, DataError> {
                props::upper_v1::lookup(&req.locale)
                    .map(zerofrom::ZeroFrom::zero_from)
                    .map(DataPayload::from_owned)
                    .map(|payload| DataResponse {
                        metadata: Default::default(),
                        payload: Some(payload),
                    })
                    .ok_or_else(|| DataErrorKind::MissingLocale.with_req(::icu_properties::provider::UppercaseV1Marker::KEY, req))
            }
        }
        #[cfg(feature = "icu_properties")]
        impl DataProvider<::icu_properties::provider::VariationSelectorV1Marker> for $provider {
            fn load(&self, req: DataRequest) -> Result<DataResponse<::icu_properties::provider::VariationSelectorV1Marker>, DataError> {
                props::vs_v1::lookup(&req.locale)
                    .map(zerofrom::ZeroFrom::zero_from)
                    .map(DataPayload::from_owned)
                    .map(|payload| DataResponse {
                        metadata: Default::default(),
                        payload: Some(payload),
                    })
                    .ok_or_else(|| DataErrorKind::MissingLocale.with_req(::icu_properties::provider::VariationSelectorV1Marker::KEY, req))
            }
        }
        #[cfg(feature = "icu_properties")]
        impl DataProvider<::icu_properties::provider::WhiteSpaceV1Marker> for $provider {
            fn load(&self, req: DataRequest) -> Result<DataResponse<::icu_properties::provider::WhiteSpaceV1Marker>, DataError> {
                props::wspace_v1::lookup(&req.locale)
                    .map(zerofrom::ZeroFrom::zero_from)
                    .map(DataPayload::from_owned)
                    .map(|payload| DataResponse {
                        metadata: Default::default(),
                        payload: Some(payload),
                    })
                    .ok_or_else(|| DataErrorKind::MissingLocale.with_req(::icu_properties::provider::WhiteSpaceV1Marker::KEY, req))
            }
        }
        #[cfg(feature = "icu_properties")]
        impl DataProvider<::icu_properties::provider::WordBreakV1Marker> for $provider {
            fn load(&self, req: DataRequest) -> Result<DataResponse<::icu_properties::provider::WordBreakV1Marker>, DataError> {
                props::wb_v1::lookup(&req.locale)
                    .map(zerofrom::ZeroFrom::zero_from)
                    .map(DataPayload::from_owned)
                    .map(|payload| DataResponse {
                        metadata: Default::default(),
                        payload: Some(payload),
                    })
                    .ok_or_else(|| DataErrorKind::MissingLocale.with_req(::icu_properties::provider::WordBreakV1Marker::KEY, req))
            }
        }
        #[cfg(feature = "icu_properties")]
        impl DataProvider<::icu_properties::provider::XidContinueV1Marker> for $provider {
            fn load(&self, req: DataRequest) -> Result<DataResponse<::icu_properties::provider::XidContinueV1Marker>, DataError> {
                props::xidc_v1::lookup(&req.locale)
                    .map(zerofrom::ZeroFrom::zero_from)
                    .map(DataPayload::from_owned)
                    .map(|payload| DataResponse {
                        metadata: Default::default(),
                        payload: Some(payload),
                    })
                    .ok_or_else(|| DataErrorKind::MissingLocale.with_req(::icu_properties::provider::XidContinueV1Marker::KEY, req))
            }
        }
        #[cfg(feature = "icu_properties")]
        impl DataProvider<::icu_properties::provider::XidStartV1Marker> for $provider {
            fn load(&self, req: DataRequest) -> Result<DataResponse<::icu_properties::provider::XidStartV1Marker>, DataError> {
                props::xids_v1::lookup(&req.locale)
                    .map(zerofrom::ZeroFrom::zero_from)
                    .map(DataPayload::from_owned)
                    .map(|payload| DataResponse {
                        metadata: Default::default(),
                        payload: Some(payload),
                    })
                    .ok_or_else(|| DataErrorKind::MissingLocale.with_req(::icu_properties::provider::XidStartV1Marker::KEY, req))
            }
        }
        impl DataProvider<::icu_provider::hello_world::HelloWorldV1Marker> for $provider {
            fn load(&self, req: DataRequest) -> Result<DataResponse<::icu_provider::hello_world::HelloWorldV1Marker>, DataError> {
                core::helloworld_v1::lookup(&req.locale)
                    .map(zerofrom::ZeroFrom::zero_from)
                    .map(DataPayload::from_owned)
                    .map(|payload| DataResponse {
                        metadata: Default::default(),
                        payload: Some(payload),
                    })
                    .ok_or_else(|| DataErrorKind::MissingLocale.with_req(::icu_provider::hello_world::HelloWorldV1Marker::KEY, req))
            }
        }
        impl DataProvider<::icu_provider_adapters::fallback::provider::CollationFallbackSupplementV1Marker> for $provider {
            fn load(
                &self,
                req: DataRequest,
            ) -> Result<DataResponse<::icu_provider_adapters::fallback::provider::CollationFallbackSupplementV1Marker>, DataError> {
                fallback::supplement::co_v1::lookup(&req.locale)
                    .map(zerofrom::ZeroFrom::zero_from)
                    .map(DataPayload::from_owned)
                    .map(|payload| DataResponse {
                        metadata: Default::default(),
                        payload: Some(payload),
                    })
                    .ok_or_else(|| {
                        DataErrorKind::MissingLocale.with_req(
                            ::icu_provider_adapters::fallback::provider::CollationFallbackSupplementV1Marker::KEY,
                            req,
                        )
                    })
            }
        }
        impl DataProvider<::icu_provider_adapters::fallback::provider::LocaleFallbackLikelySubtagsV1Marker> for $provider {
            fn load(
                &self,
                req: DataRequest,
            ) -> Result<DataResponse<::icu_provider_adapters::fallback::provider::LocaleFallbackLikelySubtagsV1Marker>, DataError> {
                fallback::likelysubtags_v1::lookup(&req.locale)
                    .map(zerofrom::ZeroFrom::zero_from)
                    .map(DataPayload::from_owned)
                    .map(|payload| DataResponse {
                        metadata: Default::default(),
                        payload: Some(payload),
                    })
                    .ok_or_else(|| {
                        DataErrorKind::MissingLocale.with_req(
                            ::icu_provider_adapters::fallback::provider::LocaleFallbackLikelySubtagsV1Marker::KEY,
                            req,
                        )
                    })
            }
        }
        impl DataProvider<::icu_provider_adapters::fallback::provider::LocaleFallbackParentsV1Marker> for $provider {
            fn load(
                &self,
                req: DataRequest,
            ) -> Result<DataResponse<::icu_provider_adapters::fallback::provider::LocaleFallbackParentsV1Marker>, DataError> {
                fallback::parents_v1::lookup(&req.locale)
                    .map(zerofrom::ZeroFrom::zero_from)
                    .map(DataPayload::from_owned)
                    .map(|payload| DataResponse {
                        metadata: Default::default(),
                        payload: Some(payload),
                    })
                    .ok_or_else(|| {
                        DataErrorKind::MissingLocale.with_req(::icu_provider_adapters::fallback::provider::LocaleFallbackParentsV1Marker::KEY, req)
                    })
            }
        }
        #[cfg(feature = "icu_relativetime")]
        impl DataProvider<::icu_relativetime::provider::LongDayRelativeTimeFormatDataV1Marker> for $provider {
            fn load(&self, req: DataRequest) -> Result<DataResponse<::icu_relativetime::provider::LongDayRelativeTimeFormatDataV1Marker>, DataError> {
                relativetime::long::day_v1::lookup(&req.locale)
                    .map(zerofrom::ZeroFrom::zero_from)
                    .map(DataPayload::from_owned)
                    .map(|payload| DataResponse {
                        metadata: Default::default(),
                        payload: Some(payload),
                    })
                    .ok_or_else(|| {
                        DataErrorKind::MissingLocale.with_req(::icu_relativetime::provider::LongDayRelativeTimeFormatDataV1Marker::KEY, req)
                    })
            }
        }
        #[cfg(feature = "icu_relativetime")]
        impl DataProvider<::icu_relativetime::provider::LongHourRelativeTimeFormatDataV1Marker> for $provider {
            fn load(
                &self,
                req: DataRequest,
            ) -> Result<DataResponse<::icu_relativetime::provider::LongHourRelativeTimeFormatDataV1Marker>, DataError> {
                relativetime::long::hour_v1::lookup(&req.locale)
                    .map(zerofrom::ZeroFrom::zero_from)
                    .map(DataPayload::from_owned)
                    .map(|payload| DataResponse {
                        metadata: Default::default(),
                        payload: Some(payload),
                    })
                    .ok_or_else(|| {
                        DataErrorKind::MissingLocale.with_req(::icu_relativetime::provider::LongHourRelativeTimeFormatDataV1Marker::KEY, req)
                    })
            }
        }
        #[cfg(feature = "icu_relativetime")]
        impl DataProvider<::icu_relativetime::provider::LongMinuteRelativeTimeFormatDataV1Marker> for $provider {
            fn load(
                &self,
                req: DataRequest,
            ) -> Result<DataResponse<::icu_relativetime::provider::LongMinuteRelativeTimeFormatDataV1Marker>, DataError> {
                relativetime::long::minute_v1::lookup(&req.locale)
                    .map(zerofrom::ZeroFrom::zero_from)
                    .map(DataPayload::from_owned)
                    .map(|payload| DataResponse {
                        metadata: Default::default(),
                        payload: Some(payload),
                    })
                    .ok_or_else(|| {
                        DataErrorKind::MissingLocale.with_req(::icu_relativetime::provider::LongMinuteRelativeTimeFormatDataV1Marker::KEY, req)
                    })
            }
        }
        #[cfg(feature = "icu_relativetime")]
        impl DataProvider<::icu_relativetime::provider::LongMonthRelativeTimeFormatDataV1Marker> for $provider {
            fn load(
                &self,
                req: DataRequest,
            ) -> Result<DataResponse<::icu_relativetime::provider::LongMonthRelativeTimeFormatDataV1Marker>, DataError> {
                relativetime::long::month_v1::lookup(&req.locale)
                    .map(zerofrom::ZeroFrom::zero_from)
                    .map(DataPayload::from_owned)
                    .map(|payload| DataResponse {
                        metadata: Default::default(),
                        payload: Some(payload),
                    })
                    .ok_or_else(|| {
                        DataErrorKind::MissingLocale.with_req(::icu_relativetime::provider::LongMonthRelativeTimeFormatDataV1Marker::KEY, req)
                    })
            }
        }
        #[cfg(feature = "icu_relativetime")]
        impl DataProvider<::icu_relativetime::provider::LongQuarterRelativeTimeFormatDataV1Marker> for $provider {
            fn load(
                &self,
                req: DataRequest,
            ) -> Result<DataResponse<::icu_relativetime::provider::LongQuarterRelativeTimeFormatDataV1Marker>, DataError> {
                relativetime::long::quarter_v1::lookup(&req.locale)
                    .map(zerofrom::ZeroFrom::zero_from)
                    .map(DataPayload::from_owned)
                    .map(|payload| DataResponse {
                        metadata: Default::default(),
                        payload: Some(payload),
                    })
                    .ok_or_else(|| {
                        DataErrorKind::MissingLocale.with_req(::icu_relativetime::provider::LongQuarterRelativeTimeFormatDataV1Marker::KEY, req)
                    })
            }
        }
        #[cfg(feature = "icu_relativetime")]
        impl DataProvider<::icu_relativetime::provider::LongSecondRelativeTimeFormatDataV1Marker> for $provider {
            fn load(
                &self,
                req: DataRequest,
            ) -> Result<DataResponse<::icu_relativetime::provider::LongSecondRelativeTimeFormatDataV1Marker>, DataError> {
                relativetime::long::second_v1::lookup(&req.locale)
                    .map(zerofrom::ZeroFrom::zero_from)
                    .map(DataPayload::from_owned)
                    .map(|payload| DataResponse {
                        metadata: Default::default(),
                        payload: Some(payload),
                    })
                    .ok_or_else(|| {
                        DataErrorKind::MissingLocale.with_req(::icu_relativetime::provider::LongSecondRelativeTimeFormatDataV1Marker::KEY, req)
                    })
            }
        }
        #[cfg(feature = "icu_relativetime")]
        impl DataProvider<::icu_relativetime::provider::LongWeekRelativeTimeFormatDataV1Marker> for $provider {
            fn load(
                &self,
                req: DataRequest,
            ) -> Result<DataResponse<::icu_relativetime::provider::LongWeekRelativeTimeFormatDataV1Marker>, DataError> {
                relativetime::long::week_v1::lookup(&req.locale)
                    .map(zerofrom::ZeroFrom::zero_from)
                    .map(DataPayload::from_owned)
                    .map(|payload| DataResponse {
                        metadata: Default::default(),
                        payload: Some(payload),
                    })
                    .ok_or_else(|| {
                        DataErrorKind::MissingLocale.with_req(::icu_relativetime::provider::LongWeekRelativeTimeFormatDataV1Marker::KEY, req)
                    })
            }
        }
        #[cfg(feature = "icu_relativetime")]
        impl DataProvider<::icu_relativetime::provider::LongYearRelativeTimeFormatDataV1Marker> for $provider {
            fn load(
                &self,
                req: DataRequest,
            ) -> Result<DataResponse<::icu_relativetime::provider::LongYearRelativeTimeFormatDataV1Marker>, DataError> {
                relativetime::long::year_v1::lookup(&req.locale)
                    .map(zerofrom::ZeroFrom::zero_from)
                    .map(DataPayload::from_owned)
                    .map(|payload| DataResponse {
                        metadata: Default::default(),
                        payload: Some(payload),
                    })
                    .ok_or_else(|| {
                        DataErrorKind::MissingLocale.with_req(::icu_relativetime::provider::LongYearRelativeTimeFormatDataV1Marker::KEY, req)
                    })
            }
        }
        #[cfg(feature = "icu_relativetime")]
        impl DataProvider<::icu_relativetime::provider::NarrowDayRelativeTimeFormatDataV1Marker> for $provider {
            fn load(
                &self,
                req: DataRequest,
            ) -> Result<DataResponse<::icu_relativetime::provider::NarrowDayRelativeTimeFormatDataV1Marker>, DataError> {
                relativetime::narrow::day_v1::lookup(&req.locale)
                    .map(zerofrom::ZeroFrom::zero_from)
                    .map(DataPayload::from_owned)
                    .map(|payload| DataResponse {
                        metadata: Default::default(),
                        payload: Some(payload),
                    })
                    .ok_or_else(|| {
                        DataErrorKind::MissingLocale.with_req(::icu_relativetime::provider::NarrowDayRelativeTimeFormatDataV1Marker::KEY, req)
                    })
            }
        }
        #[cfg(feature = "icu_relativetime")]
        impl DataProvider<::icu_relativetime::provider::NarrowHourRelativeTimeFormatDataV1Marker> for $provider {
            fn load(
                &self,
                req: DataRequest,
            ) -> Result<DataResponse<::icu_relativetime::provider::NarrowHourRelativeTimeFormatDataV1Marker>, DataError> {
                relativetime::narrow::hour_v1::lookup(&req.locale)
                    .map(zerofrom::ZeroFrom::zero_from)
                    .map(DataPayload::from_owned)
                    .map(|payload| DataResponse {
                        metadata: Default::default(),
                        payload: Some(payload),
                    })
                    .ok_or_else(|| {
                        DataErrorKind::MissingLocale.with_req(::icu_relativetime::provider::NarrowHourRelativeTimeFormatDataV1Marker::KEY, req)
                    })
            }
        }
        #[cfg(feature = "icu_relativetime")]
        impl DataProvider<::icu_relativetime::provider::NarrowMinuteRelativeTimeFormatDataV1Marker> for $provider {
            fn load(
                &self,
                req: DataRequest,
            ) -> Result<DataResponse<::icu_relativetime::provider::NarrowMinuteRelativeTimeFormatDataV1Marker>, DataError> {
                relativetime::narrow::minute_v1::lookup(&req.locale)
                    .map(zerofrom::ZeroFrom::zero_from)
                    .map(DataPayload::from_owned)
                    .map(|payload| DataResponse {
                        metadata: Default::default(),
                        payload: Some(payload),
                    })
                    .ok_or_else(|| {
                        DataErrorKind::MissingLocale.with_req(::icu_relativetime::provider::NarrowMinuteRelativeTimeFormatDataV1Marker::KEY, req)
                    })
            }
        }
        #[cfg(feature = "icu_relativetime")]
        impl DataProvider<::icu_relativetime::provider::NarrowMonthRelativeTimeFormatDataV1Marker> for $provider {
            fn load(
                &self,
                req: DataRequest,
            ) -> Result<DataResponse<::icu_relativetime::provider::NarrowMonthRelativeTimeFormatDataV1Marker>, DataError> {
                relativetime::narrow::month_v1::lookup(&req.locale)
                    .map(zerofrom::ZeroFrom::zero_from)
                    .map(DataPayload::from_owned)
                    .map(|payload| DataResponse {
                        metadata: Default::default(),
                        payload: Some(payload),
                    })
                    .ok_or_else(|| {
                        DataErrorKind::MissingLocale.with_req(::icu_relativetime::provider::NarrowMonthRelativeTimeFormatDataV1Marker::KEY, req)
                    })
            }
        }
        #[cfg(feature = "icu_relativetime")]
        impl DataProvider<::icu_relativetime::provider::NarrowQuarterRelativeTimeFormatDataV1Marker> for $provider {
            fn load(
                &self,
                req: DataRequest,
            ) -> Result<DataResponse<::icu_relativetime::provider::NarrowQuarterRelativeTimeFormatDataV1Marker>, DataError> {
                relativetime::narrow::quarter_v1::lookup(&req.locale)
                    .map(zerofrom::ZeroFrom::zero_from)
                    .map(DataPayload::from_owned)
                    .map(|payload| DataResponse {
                        metadata: Default::default(),
                        payload: Some(payload),
                    })
                    .ok_or_else(|| {
                        DataErrorKind::MissingLocale.with_req(::icu_relativetime::provider::NarrowQuarterRelativeTimeFormatDataV1Marker::KEY, req)
                    })
            }
        }
        #[cfg(feature = "icu_relativetime")]
        impl DataProvider<::icu_relativetime::provider::NarrowSecondRelativeTimeFormatDataV1Marker> for $provider {
            fn load(
                &self,
                req: DataRequest,
            ) -> Result<DataResponse<::icu_relativetime::provider::NarrowSecondRelativeTimeFormatDataV1Marker>, DataError> {
                relativetime::narrow::second_v1::lookup(&req.locale)
                    .map(zerofrom::ZeroFrom::zero_from)
                    .map(DataPayload::from_owned)
                    .map(|payload| DataResponse {
                        metadata: Default::default(),
                        payload: Some(payload),
                    })
                    .ok_or_else(|| {
                        DataErrorKind::MissingLocale.with_req(::icu_relativetime::provider::NarrowSecondRelativeTimeFormatDataV1Marker::KEY, req)
                    })
            }
        }
        #[cfg(feature = "icu_relativetime")]
        impl DataProvider<::icu_relativetime::provider::NarrowWeekRelativeTimeFormatDataV1Marker> for $provider {
            fn load(
                &self,
                req: DataRequest,
            ) -> Result<DataResponse<::icu_relativetime::provider::NarrowWeekRelativeTimeFormatDataV1Marker>, DataError> {
                relativetime::narrow::week_v1::lookup(&req.locale)
                    .map(zerofrom::ZeroFrom::zero_from)
                    .map(DataPayload::from_owned)
                    .map(|payload| DataResponse {
                        metadata: Default::default(),
                        payload: Some(payload),
                    })
                    .ok_or_else(|| {
                        DataErrorKind::MissingLocale.with_req(::icu_relativetime::provider::NarrowWeekRelativeTimeFormatDataV1Marker::KEY, req)
                    })
            }
        }
        #[cfg(feature = "icu_relativetime")]
        impl DataProvider<::icu_relativetime::provider::NarrowYearRelativeTimeFormatDataV1Marker> for $provider {
            fn load(
                &self,
                req: DataRequest,
            ) -> Result<DataResponse<::icu_relativetime::provider::NarrowYearRelativeTimeFormatDataV1Marker>, DataError> {
                relativetime::narrow::year_v1::lookup(&req.locale)
                    .map(zerofrom::ZeroFrom::zero_from)
                    .map(DataPayload::from_owned)
                    .map(|payload| DataResponse {
                        metadata: Default::default(),
                        payload: Some(payload),
                    })
                    .ok_or_else(|| {
                        DataErrorKind::MissingLocale.with_req(::icu_relativetime::provider::NarrowYearRelativeTimeFormatDataV1Marker::KEY, req)
                    })
            }
        }
        #[cfg(feature = "icu_relativetime")]
        impl DataProvider<::icu_relativetime::provider::ShortDayRelativeTimeFormatDataV1Marker> for $provider {
            fn load(
                &self,
                req: DataRequest,
            ) -> Result<DataResponse<::icu_relativetime::provider::ShortDayRelativeTimeFormatDataV1Marker>, DataError> {
                relativetime::short::day_v1::lookup(&req.locale)
                    .map(zerofrom::ZeroFrom::zero_from)
                    .map(DataPayload::from_owned)
                    .map(|payload| DataResponse {
                        metadata: Default::default(),
                        payload: Some(payload),
                    })
                    .ok_or_else(|| {
                        DataErrorKind::MissingLocale.with_req(::icu_relativetime::provider::ShortDayRelativeTimeFormatDataV1Marker::KEY, req)
                    })
            }
        }
        #[cfg(feature = "icu_relativetime")]
        impl DataProvider<::icu_relativetime::provider::ShortHourRelativeTimeFormatDataV1Marker> for $provider {
            fn load(
                &self,
                req: DataRequest,
            ) -> Result<DataResponse<::icu_relativetime::provider::ShortHourRelativeTimeFormatDataV1Marker>, DataError> {
                relativetime::short::hour_v1::lookup(&req.locale)
                    .map(zerofrom::ZeroFrom::zero_from)
                    .map(DataPayload::from_owned)
                    .map(|payload| DataResponse {
                        metadata: Default::default(),
                        payload: Some(payload),
                    })
                    .ok_or_else(|| {
                        DataErrorKind::MissingLocale.with_req(::icu_relativetime::provider::ShortHourRelativeTimeFormatDataV1Marker::KEY, req)
                    })
            }
        }
        #[cfg(feature = "icu_relativetime")]
        impl DataProvider<::icu_relativetime::provider::ShortMinuteRelativeTimeFormatDataV1Marker> for $provider {
            fn load(
                &self,
                req: DataRequest,
            ) -> Result<DataResponse<::icu_relativetime::provider::ShortMinuteRelativeTimeFormatDataV1Marker>, DataError> {
                relativetime::short::minute_v1::lookup(&req.locale)
                    .map(zerofrom::ZeroFrom::zero_from)
                    .map(DataPayload::from_owned)
                    .map(|payload| DataResponse {
                        metadata: Default::default(),
                        payload: Some(payload),
                    })
                    .ok_or_else(|| {
                        DataErrorKind::MissingLocale.with_req(::icu_relativetime::provider::ShortMinuteRelativeTimeFormatDataV1Marker::KEY, req)
                    })
            }
        }
        #[cfg(feature = "icu_relativetime")]
        impl DataProvider<::icu_relativetime::provider::ShortMonthRelativeTimeFormatDataV1Marker> for $provider {
            fn load(
                &self,
                req: DataRequest,
            ) -> Result<DataResponse<::icu_relativetime::provider::ShortMonthRelativeTimeFormatDataV1Marker>, DataError> {
                relativetime::short::month_v1::lookup(&req.locale)
                    .map(zerofrom::ZeroFrom::zero_from)
                    .map(DataPayload::from_owned)
                    .map(|payload| DataResponse {
                        metadata: Default::default(),
                        payload: Some(payload),
                    })
                    .ok_or_else(|| {
                        DataErrorKind::MissingLocale.with_req(::icu_relativetime::provider::ShortMonthRelativeTimeFormatDataV1Marker::KEY, req)
                    })
            }
        }
        #[cfg(feature = "icu_relativetime")]
        impl DataProvider<::icu_relativetime::provider::ShortQuarterRelativeTimeFormatDataV1Marker> for $provider {
            fn load(
                &self,
                req: DataRequest,
            ) -> Result<DataResponse<::icu_relativetime::provider::ShortQuarterRelativeTimeFormatDataV1Marker>, DataError> {
                relativetime::short::quarter_v1::lookup(&req.locale)
                    .map(zerofrom::ZeroFrom::zero_from)
                    .map(DataPayload::from_owned)
                    .map(|payload| DataResponse {
                        metadata: Default::default(),
                        payload: Some(payload),
                    })
                    .ok_or_else(|| {
                        DataErrorKind::MissingLocale.with_req(::icu_relativetime::provider::ShortQuarterRelativeTimeFormatDataV1Marker::KEY, req)
                    })
            }
        }
        #[cfg(feature = "icu_relativetime")]
        impl DataProvider<::icu_relativetime::provider::ShortSecondRelativeTimeFormatDataV1Marker> for $provider {
            fn load(
                &self,
                req: DataRequest,
            ) -> Result<DataResponse<::icu_relativetime::provider::ShortSecondRelativeTimeFormatDataV1Marker>, DataError> {
                relativetime::short::second_v1::lookup(&req.locale)
                    .map(zerofrom::ZeroFrom::zero_from)
                    .map(DataPayload::from_owned)
                    .map(|payload| DataResponse {
                        metadata: Default::default(),
                        payload: Some(payload),
                    })
                    .ok_or_else(|| {
                        DataErrorKind::MissingLocale.with_req(::icu_relativetime::provider::ShortSecondRelativeTimeFormatDataV1Marker::KEY, req)
                    })
            }
        }
        #[cfg(feature = "icu_relativetime")]
        impl DataProvider<::icu_relativetime::provider::ShortWeekRelativeTimeFormatDataV1Marker> for $provider {
            fn load(
                &self,
                req: DataRequest,
            ) -> Result<DataResponse<::icu_relativetime::provider::ShortWeekRelativeTimeFormatDataV1Marker>, DataError> {
                relativetime::short::week_v1::lookup(&req.locale)
                    .map(zerofrom::ZeroFrom::zero_from)
                    .map(DataPayload::from_owned)
                    .map(|payload| DataResponse {
                        metadata: Default::default(),
                        payload: Some(payload),
                    })
                    .ok_or_else(|| {
                        DataErrorKind::MissingLocale.with_req(::icu_relativetime::provider::ShortWeekRelativeTimeFormatDataV1Marker::KEY, req)
                    })
            }
        }
        #[cfg(feature = "icu_relativetime")]
        impl DataProvider<::icu_relativetime::provider::ShortYearRelativeTimeFormatDataV1Marker> for $provider {
            fn load(
                &self,
                req: DataRequest,
            ) -> Result<DataResponse<::icu_relativetime::provider::ShortYearRelativeTimeFormatDataV1Marker>, DataError> {
                relativetime::short::year_v1::lookup(&req.locale)
                    .map(zerofrom::ZeroFrom::zero_from)
                    .map(DataPayload::from_owned)
                    .map(|payload| DataResponse {
                        metadata: Default::default(),
                        payload: Some(payload),
                    })
                    .ok_or_else(|| {
                        DataErrorKind::MissingLocale.with_req(::icu_relativetime::provider::ShortYearRelativeTimeFormatDataV1Marker::KEY, req)
                    })
            }
        }
        #[cfg(feature = "icu_segmenter")]
        impl DataProvider<::icu_segmenter::provider::GraphemeClusterBreakDataV1Marker> for $provider {
            fn load(&self, req: DataRequest) -> Result<DataResponse<::icu_segmenter::provider::GraphemeClusterBreakDataV1Marker>, DataError> {
                segmenter::grapheme_v1::lookup(&req.locale)
                    .map(zerofrom::ZeroFrom::zero_from)
                    .map(DataPayload::from_owned)
                    .map(|payload| DataResponse {
                        metadata: Default::default(),
                        payload: Some(payload),
                    })
                    .ok_or_else(|| DataErrorKind::MissingLocale.with_req(::icu_segmenter::provider::GraphemeClusterBreakDataV1Marker::KEY, req))
            }
        }
        #[cfg(feature = "icu_segmenter")]
        impl DataProvider<::icu_segmenter::provider::LineBreakDataV1Marker> for $provider {
            fn load(&self, req: DataRequest) -> Result<DataResponse<::icu_segmenter::provider::LineBreakDataV1Marker>, DataError> {
                segmenter::line_v1::lookup(&req.locale)
                    .map(zerofrom::ZeroFrom::zero_from)
                    .map(DataPayload::from_owned)
                    .map(|payload| DataResponse {
                        metadata: Default::default(),
                        payload: Some(payload),
                    })
                    .ok_or_else(|| DataErrorKind::MissingLocale.with_req(::icu_segmenter::provider::LineBreakDataV1Marker::KEY, req))
            }
        }
        #[cfg(feature = "icu_segmenter")]
        impl DataProvider<::icu_segmenter::provider::LstmDataV1Marker> for $provider {
            fn load(&self, req: DataRequest) -> Result<DataResponse<::icu_segmenter::provider::LstmDataV1Marker>, DataError> {
                segmenter::lstm_v1::lookup(&req.locale)
                    .map(zerofrom::ZeroFrom::zero_from)
                    .map(DataPayload::from_owned)
                    .map(|payload| DataResponse {
                        metadata: Default::default(),
                        payload: Some(payload),
                    })
                    .ok_or_else(|| DataErrorKind::MissingLocale.with_req(::icu_segmenter::provider::LstmDataV1Marker::KEY, req))
            }
        }
        #[cfg(feature = "icu_segmenter")]
        impl DataProvider<::icu_segmenter::provider::SentenceBreakDataV1Marker> for $provider {
            fn load(&self, req: DataRequest) -> Result<DataResponse<::icu_segmenter::provider::SentenceBreakDataV1Marker>, DataError> {
                segmenter::sentence_v1::lookup(&req.locale)
                    .map(zerofrom::ZeroFrom::zero_from)
                    .map(DataPayload::from_owned)
                    .map(|payload| DataResponse {
                        metadata: Default::default(),
                        payload: Some(payload),
                    })
                    .ok_or_else(|| DataErrorKind::MissingLocale.with_req(::icu_segmenter::provider::SentenceBreakDataV1Marker::KEY, req))
            }
        }
        #[cfg(feature = "icu_segmenter")]
        impl DataProvider<::icu_segmenter::provider::UCharDictionaryBreakDataV1Marker> for $provider {
            fn load(&self, req: DataRequest) -> Result<DataResponse<::icu_segmenter::provider::UCharDictionaryBreakDataV1Marker>, DataError> {
                segmenter::dictionary_v1::lookup(&req.locale)
                    .map(zerofrom::ZeroFrom::zero_from)
                    .map(DataPayload::from_owned)
                    .map(|payload| DataResponse {
                        metadata: Default::default(),
                        payload: Some(payload),
                    })
                    .ok_or_else(|| DataErrorKind::MissingLocale.with_req(::icu_segmenter::provider::UCharDictionaryBreakDataV1Marker::KEY, req))
            }
        }
        #[cfg(feature = "icu_segmenter")]
        impl DataProvider<::icu_segmenter::provider::WordBreakDataV1Marker> for $provider {
            fn load(&self, req: DataRequest) -> Result<DataResponse<::icu_segmenter::provider::WordBreakDataV1Marker>, DataError> {
                segmenter::word_v1::lookup(&req.locale)
                    .map(zerofrom::ZeroFrom::zero_from)
                    .map(DataPayload::from_owned)
                    .map(|payload| DataResponse {
                        metadata: Default::default(),
                        payload: Some(payload),
                    })
                    .ok_or_else(|| DataErrorKind::MissingLocale.with_req(::icu_segmenter::provider::WordBreakDataV1Marker::KEY, req))
            }
        }
        #[cfg(feature = "icu_timezone")]
        impl DataProvider<::icu_timezone::provider::MetazonePeriodV1Marker> for $provider {
            fn load(&self, req: DataRequest) -> Result<DataResponse<::icu_timezone::provider::MetazonePeriodV1Marker>, DataError> {
                time_zone::metazone_period_v1::lookup(&req.locale)
                    .map(zerofrom::ZeroFrom::zero_from)
                    .map(DataPayload::from_owned)
                    .map(|payload| DataResponse {
                        metadata: Default::default(),
                        payload: Some(payload),
                    })
                    .ok_or_else(|| DataErrorKind::MissingLocale.with_req(::icu_timezone::provider::MetazonePeriodV1Marker::KEY, req))
            }
        }
    };
}
/// Implement [`AnyProvider`] on the given struct using the data
/// hardcoded in this module. This allows the struct to be used with
/// `icu`'s `_any` constructors.
///
/// This macro can only be called from its definition-site, i.e. right
/// after `include!`-ing the generated module.
///
/// ```compile_fail
/// struct MyAnyProvider;
/// include!("/path/to/generated/mod.rs");
/// impl_any_provider(MyAnyProvider);
/// ```
#[allow(unused_macros)]
macro_rules! impl_any_provider {
    ($ provider : path) => {
        impl AnyProvider for $provider {
            fn load_any(&self, key: DataKey, req: DataRequest) -> Result<AnyResponse, DataError> {
                #[cfg(feature = "icu_calendar")]
                const JAPANESEERASV1MARKER: ::icu_provider::DataKeyHash = ::icu_calendar::provider::JapaneseErasV1Marker::KEY.hashed();
                #[cfg(feature = "icu_calendar")]
                const JAPANESEEXTENDEDERASV1MARKER: ::icu_provider::DataKeyHash =
                    ::icu_calendar::provider::JapaneseExtendedErasV1Marker::KEY.hashed();
                #[cfg(feature = "icu_calendar")]
                const WEEKDATAV1MARKER: ::icu_provider::DataKeyHash = ::icu_calendar::provider::WeekDataV1Marker::KEY.hashed();
                #[cfg(feature = "icu_casemapping")]
                const CASEMAPPINGV1MARKER: ::icu_provider::DataKeyHash = ::icu_casemapping::provider::CaseMappingV1Marker::KEY.hashed();
                #[cfg(feature = "icu_collator")]
                const COLLATIONDATAV1MARKER: ::icu_provider::DataKeyHash = ::icu_collator::provider::CollationDataV1Marker::KEY.hashed();
                #[cfg(feature = "icu_collator")]
                const COLLATIONDIACRITICSV1MARKER: ::icu_provider::DataKeyHash = ::icu_collator::provider::CollationDiacriticsV1Marker::KEY.hashed();
                #[cfg(feature = "icu_collator")]
                const COLLATIONJAMOV1MARKER: ::icu_provider::DataKeyHash = ::icu_collator::provider::CollationJamoV1Marker::KEY.hashed();
                #[cfg(feature = "icu_collator")]
                const COLLATIONMETADATAV1MARKER: ::icu_provider::DataKeyHash = ::icu_collator::provider::CollationMetadataV1Marker::KEY.hashed();
                #[cfg(feature = "icu_collator")]
                const COLLATIONREORDERINGV1MARKER: ::icu_provider::DataKeyHash = ::icu_collator::provider::CollationReorderingV1Marker::KEY.hashed();
                #[cfg(feature = "icu_collator")]
                const COLLATIONSPECIALPRIMARIESV1MARKER: ::icu_provider::DataKeyHash =
                    ::icu_collator::provider::CollationSpecialPrimariesV1Marker::KEY.hashed();
                #[cfg(feature = "icu_compactdecimal")]
                const LONGCOMPACTDECIMALFORMATDATAV1MARKER: ::icu_provider::DataKeyHash =
                    ::icu_compactdecimal::provider::LongCompactDecimalFormatDataV1Marker::KEY.hashed();
                #[cfg(feature = "icu_compactdecimal")]
                const SHORTCOMPACTDECIMALFORMATDATAV1MARKER: ::icu_provider::DataKeyHash =
                    ::icu_compactdecimal::provider::ShortCompactDecimalFormatDataV1Marker::KEY.hashed();
                #[cfg(feature = "icu_datetime")]
                const BUDDHISTDATELENGTHSV1MARKER: ::icu_provider::DataKeyHash =
                    ::icu_datetime::provider::calendar::BuddhistDateLengthsV1Marker::KEY.hashed();
                #[cfg(feature = "icu_datetime")]
                const BUDDHISTDATESYMBOLSV1MARKER: ::icu_provider::DataKeyHash =
                    ::icu_datetime::provider::calendar::BuddhistDateSymbolsV1Marker::KEY.hashed();
                #[cfg(feature = "icu_datetime")]
                const COPTICDATELENGTHSV1MARKER: ::icu_provider::DataKeyHash =
                    ::icu_datetime::provider::calendar::CopticDateLengthsV1Marker::KEY.hashed();
                #[cfg(feature = "icu_datetime")]
                const COPTICDATESYMBOLSV1MARKER: ::icu_provider::DataKeyHash =
                    ::icu_datetime::provider::calendar::CopticDateSymbolsV1Marker::KEY.hashed();
                #[cfg(feature = "icu_datetime_experimental")]
                const DATESKELETONPATTERNSV1MARKER: ::icu_provider::DataKeyHash =
                    ::icu_datetime::provider::calendar::DateSkeletonPatternsV1Marker::KEY.hashed();
                #[cfg(feature = "icu_datetime")]
                const ETHIOPIANDATELENGTHSV1MARKER: ::icu_provider::DataKeyHash =
                    ::icu_datetime::provider::calendar::EthiopianDateLengthsV1Marker::KEY.hashed();
                #[cfg(feature = "icu_datetime")]
                const ETHIOPIANDATESYMBOLSV1MARKER: ::icu_provider::DataKeyHash =
                    ::icu_datetime::provider::calendar::EthiopianDateSymbolsV1Marker::KEY.hashed();
                #[cfg(feature = "icu_datetime")]
                const GREGORIANDATELENGTHSV1MARKER: ::icu_provider::DataKeyHash =
                    ::icu_datetime::provider::calendar::GregorianDateLengthsV1Marker::KEY.hashed();
                #[cfg(feature = "icu_datetime")]
                const GREGORIANDATESYMBOLSV1MARKER: ::icu_provider::DataKeyHash =
                    ::icu_datetime::provider::calendar::GregorianDateSymbolsV1Marker::KEY.hashed();
                #[cfg(feature = "icu_datetime")]
                const INDIANDATELENGTHSV1MARKER: ::icu_provider::DataKeyHash =
                    ::icu_datetime::provider::calendar::IndianDateLengthsV1Marker::KEY.hashed();
                #[cfg(feature = "icu_datetime")]
                const INDIANDATESYMBOLSV1MARKER: ::icu_provider::DataKeyHash =
                    ::icu_datetime::provider::calendar::IndianDateSymbolsV1Marker::KEY.hashed();
                #[cfg(feature = "icu_datetime")]
                const JAPANESEDATELENGTHSV1MARKER: ::icu_provider::DataKeyHash =
                    ::icu_datetime::provider::calendar::JapaneseDateLengthsV1Marker::KEY.hashed();
                #[cfg(feature = "icu_datetime")]
                const JAPANESEDATESYMBOLSV1MARKER: ::icu_provider::DataKeyHash =
                    ::icu_datetime::provider::calendar::JapaneseDateSymbolsV1Marker::KEY.hashed();
                #[cfg(feature = "icu_datetime")]
                const JAPANESEEXTENDEDDATELENGTHSV1MARKER: ::icu_provider::DataKeyHash =
                    ::icu_datetime::provider::calendar::JapaneseExtendedDateLengthsV1Marker::KEY.hashed();
                #[cfg(feature = "icu_datetime")]
                const JAPANESEEXTENDEDDATESYMBOLSV1MARKER: ::icu_provider::DataKeyHash =
                    ::icu_datetime::provider::calendar::JapaneseExtendedDateSymbolsV1Marker::KEY.hashed();
                #[cfg(feature = "icu_datetime")]
                const TIMELENGTHSV1MARKER: ::icu_provider::DataKeyHash = ::icu_datetime::provider::calendar::TimeLengthsV1Marker::KEY.hashed();
                #[cfg(feature = "icu_datetime")]
                const TIMESYMBOLSV1MARKER: ::icu_provider::DataKeyHash = ::icu_datetime::provider::calendar::TimeSymbolsV1Marker::KEY.hashed();
                #[cfg(feature = "icu_datetime")]
                const EXEMPLARCITIESV1MARKER: ::icu_provider::DataKeyHash =
                    ::icu_datetime::provider::time_zones::ExemplarCitiesV1Marker::KEY.hashed();
                #[cfg(feature = "icu_datetime")]
                const METAZONEGENERICNAMESLONGV1MARKER: ::icu_provider::DataKeyHash =
                    ::icu_datetime::provider::time_zones::MetazoneGenericNamesLongV1Marker::KEY.hashed();
                #[cfg(feature = "icu_datetime")]
                const METAZONEGENERICNAMESSHORTV1MARKER: ::icu_provider::DataKeyHash =
                    ::icu_datetime::provider::time_zones::MetazoneGenericNamesShortV1Marker::KEY.hashed();
                #[cfg(feature = "icu_datetime")]
                const METAZONESPECIFICNAMESLONGV1MARKER: ::icu_provider::DataKeyHash =
                    ::icu_datetime::provider::time_zones::MetazoneSpecificNamesLongV1Marker::KEY.hashed();
                #[cfg(feature = "icu_datetime")]
                const METAZONESPECIFICNAMESSHORTV1MARKER: ::icu_provider::DataKeyHash =
                    ::icu_datetime::provider::time_zones::MetazoneSpecificNamesShortV1Marker::KEY.hashed();
                #[cfg(feature = "icu_datetime")]
                const TIMEZONEFORMATSV1MARKER: ::icu_provider::DataKeyHash =
                    ::icu_datetime::provider::time_zones::TimeZoneFormatsV1Marker::KEY.hashed();
                #[cfg(feature = "icu_decimal")]
                const DECIMALSYMBOLSV1MARKER: ::icu_provider::DataKeyHash = ::icu_decimal::provider::DecimalSymbolsV1Marker::KEY.hashed();
                #[cfg(feature = "icu_displaynames")]
                const LANGUAGEDISPLAYNAMESV1MARKER: ::icu_provider::DataKeyHash =
                    ::icu_displaynames::provider::LanguageDisplayNamesV1Marker::KEY.hashed();
                #[cfg(feature = "icu_displaynames")]
                const TERRITORYDISPLAYNAMESV1MARKER: ::icu_provider::DataKeyHash =
                    ::icu_displaynames::provider::TerritoryDisplayNamesV1Marker::KEY.hashed();
                #[cfg(feature = "icu_list")]
                const ANDLISTV1MARKER: ::icu_provider::DataKeyHash = ::icu_list::provider::AndListV1Marker::KEY.hashed();
                #[cfg(feature = "icu_list")]
                const ORLISTV1MARKER: ::icu_provider::DataKeyHash = ::icu_list::provider::OrListV1Marker::KEY.hashed();
                #[cfg(feature = "icu_list")]
                const UNITLISTV1MARKER: ::icu_provider::DataKeyHash = ::icu_list::provider::UnitListV1Marker::KEY.hashed();
                #[cfg(feature = "icu_locid_transform")]
                const ALIASESV1MARKER: ::icu_provider::DataKeyHash = ::icu_locid_transform::provider::AliasesV1Marker::KEY.hashed();
                #[cfg(feature = "icu_locid_transform")]
                const LIKELYSUBTAGSV1MARKER: ::icu_provider::DataKeyHash = ::icu_locid_transform::provider::LikelySubtagsV1Marker::KEY.hashed();
                #[cfg(feature = "icu_normalizer")]
                const CANONICALCOMPOSITIONSV1MARKER: ::icu_provider::DataKeyHash =
                    ::icu_normalizer::provider::CanonicalCompositionsV1Marker::KEY.hashed();
                #[cfg(feature = "icu_normalizer")]
                const CANONICALDECOMPOSITIONDATAV1MARKER: ::icu_provider::DataKeyHash =
                    ::icu_normalizer::provider::CanonicalDecompositionDataV1Marker::KEY.hashed();
                #[cfg(feature = "icu_normalizer")]
                const CANONICALDECOMPOSITIONTABLESV1MARKER: ::icu_provider::DataKeyHash =
                    ::icu_normalizer::provider::CanonicalDecompositionTablesV1Marker::KEY.hashed();
                #[cfg(feature = "icu_normalizer")]
                const COMPATIBILITYDECOMPOSITIONSUPPLEMENTV1MARKER: ::icu_provider::DataKeyHash =
                    ::icu_normalizer::provider::CompatibilityDecompositionSupplementV1Marker::KEY.hashed();
                #[cfg(feature = "icu_normalizer")]
                const COMPATIBILITYDECOMPOSITIONTABLESV1MARKER: ::icu_provider::DataKeyHash =
                    ::icu_normalizer::provider::CompatibilityDecompositionTablesV1Marker::KEY.hashed();
                #[cfg(feature = "icu_normalizer")]
                const NONRECURSIVEDECOMPOSITIONSUPPLEMENTV1MARKER: ::icu_provider::DataKeyHash =
                    ::icu_normalizer::provider::NonRecursiveDecompositionSupplementV1Marker::KEY.hashed();
                #[cfg(feature = "icu_normalizer")]
                const UTS46DECOMPOSITIONSUPPLEMENTV1MARKER: ::icu_provider::DataKeyHash =
                    ::icu_normalizer::provider::Uts46DecompositionSupplementV1Marker::KEY.hashed();
                #[cfg(feature = "icu_plurals")]
                const CARDINALV1MARKER: ::icu_provider::DataKeyHash = ::icu_plurals::provider::CardinalV1Marker::KEY.hashed();
                #[cfg(feature = "icu_plurals")]
                const ORDINALV1MARKER: ::icu_provider::DataKeyHash = ::icu_plurals::provider::OrdinalV1Marker::KEY.hashed();
                #[cfg(feature = "icu_properties")]
                const ALPHABETICV1MARKER: ::icu_provider::DataKeyHash = ::icu_properties::provider::AlphabeticV1Marker::KEY.hashed();
                #[cfg(feature = "icu_properties")]
                const ASCIIHEXDIGITV1MARKER: ::icu_provider::DataKeyHash = ::icu_properties::provider::AsciiHexDigitV1Marker::KEY.hashed();
                #[cfg(feature = "icu_properties")]
                const BASICEMOJIV1MARKER: ::icu_provider::DataKeyHash = ::icu_properties::provider::BasicEmojiV1Marker::KEY.hashed();
                #[cfg(feature = "icu_properties")]
                const BIDICLASSV1MARKER: ::icu_provider::DataKeyHash = ::icu_properties::provider::BidiClassV1Marker::KEY.hashed();
                #[cfg(feature = "icu_properties")]
                const BIDICONTROLV1MARKER: ::icu_provider::DataKeyHash = ::icu_properties::provider::BidiControlV1Marker::KEY.hashed();
                #[cfg(feature = "icu_properties")]
                const BIDIMIRROREDV1MARKER: ::icu_provider::DataKeyHash = ::icu_properties::provider::BidiMirroredV1Marker::KEY.hashed();
                #[cfg(feature = "icu_properties")]
                const CANONICALCOMBININGCLASSV1MARKER: ::icu_provider::DataKeyHash =
                    ::icu_properties::provider::CanonicalCombiningClassV1Marker::KEY.hashed();
                #[cfg(feature = "icu_properties")]
                const CASEIGNORABLEV1MARKER: ::icu_provider::DataKeyHash = ::icu_properties::provider::CaseIgnorableV1Marker::KEY.hashed();
                #[cfg(feature = "icu_properties")]
                const CASEDV1MARKER: ::icu_provider::DataKeyHash = ::icu_properties::provider::CasedV1Marker::KEY.hashed();
                #[cfg(feature = "icu_properties")]
                const CHANGESWHENCASEFOLDEDV1MARKER: ::icu_provider::DataKeyHash =
                    ::icu_properties::provider::ChangesWhenCasefoldedV1Marker::KEY.hashed();
                #[cfg(feature = "icu_properties")]
                const CHANGESWHENLOWERCASEDV1MARKER: ::icu_provider::DataKeyHash =
                    ::icu_properties::provider::ChangesWhenLowercasedV1Marker::KEY.hashed();
                #[cfg(feature = "icu_properties")]
                const CHANGESWHENNFKCCASEFOLDEDV1MARKER: ::icu_provider::DataKeyHash =
                    ::icu_properties::provider::ChangesWhenNfkcCasefoldedV1Marker::KEY.hashed();
                #[cfg(feature = "icu_properties")]
                const CHANGESWHENTITLECASEDV1MARKER: ::icu_provider::DataKeyHash =
                    ::icu_properties::provider::ChangesWhenTitlecasedV1Marker::KEY.hashed();
                #[cfg(feature = "icu_properties")]
                const CHANGESWHENUPPERCASEDV1MARKER: ::icu_provider::DataKeyHash =
                    ::icu_properties::provider::ChangesWhenUppercasedV1Marker::KEY.hashed();
                #[cfg(feature = "icu_properties")]
                const DASHV1MARKER: ::icu_provider::DataKeyHash = ::icu_properties::provider::DashV1Marker::KEY.hashed();
                #[cfg(feature = "icu_properties")]
                const DEFAULTIGNORABLECODEPOINTV1MARKER: ::icu_provider::DataKeyHash =
                    ::icu_properties::provider::DefaultIgnorableCodePointV1Marker::KEY.hashed();
                #[cfg(feature = "icu_properties")]
                const DEPRECATEDV1MARKER: ::icu_provider::DataKeyHash = ::icu_properties::provider::DeprecatedV1Marker::KEY.hashed();
                #[cfg(feature = "icu_properties")]
                const DIACRITICV1MARKER: ::icu_provider::DataKeyHash = ::icu_properties::provider::DiacriticV1Marker::KEY.hashed();
                #[cfg(feature = "icu_properties")]
                const EASTASIANWIDTHV1MARKER: ::icu_provider::DataKeyHash = ::icu_properties::provider::EastAsianWidthV1Marker::KEY.hashed();
                #[cfg(feature = "icu_properties")]
                const EMOJICOMPONENTV1MARKER: ::icu_provider::DataKeyHash = ::icu_properties::provider::EmojiComponentV1Marker::KEY.hashed();
                #[cfg(feature = "icu_properties")]
                const EMOJIMODIFIERBASEV1MARKER: ::icu_provider::DataKeyHash = ::icu_properties::provider::EmojiModifierBaseV1Marker::KEY.hashed();
                #[cfg(feature = "icu_properties")]
                const EMOJIMODIFIERV1MARKER: ::icu_provider::DataKeyHash = ::icu_properties::provider::EmojiModifierV1Marker::KEY.hashed();
                #[cfg(feature = "icu_properties")]
                const EMOJIPRESENTATIONV1MARKER: ::icu_provider::DataKeyHash = ::icu_properties::provider::EmojiPresentationV1Marker::KEY.hashed();
                #[cfg(feature = "icu_properties")]
                const EMOJIV1MARKER: ::icu_provider::DataKeyHash = ::icu_properties::provider::EmojiV1Marker::KEY.hashed();
                #[cfg(feature = "icu_properties")]
                const EXEMPLARCHARACTERSAUXILIARYV1MARKER: ::icu_provider::DataKeyHash =
                    ::icu_properties::provider::ExemplarCharactersAuxiliaryV1Marker::KEY.hashed();
                #[cfg(feature = "icu_properties")]
                const EXEMPLARCHARACTERSINDEXV1MARKER: ::icu_provider::DataKeyHash =
                    ::icu_properties::provider::ExemplarCharactersIndexV1Marker::KEY.hashed();
                #[cfg(feature = "icu_properties")]
                const EXEMPLARCHARACTERSMAINV1MARKER: ::icu_provider::DataKeyHash =
                    ::icu_properties::provider::ExemplarCharactersMainV1Marker::KEY.hashed();
                #[cfg(feature = "icu_properties")]
                const EXEMPLARCHARACTERSNUMBERSV1MARKER: ::icu_provider::DataKeyHash =
                    ::icu_properties::provider::ExemplarCharactersNumbersV1Marker::KEY.hashed();
                #[cfg(feature = "icu_properties")]
                const EXEMPLARCHARACTERSPUNCTUATIONV1MARKER: ::icu_provider::DataKeyHash =
                    ::icu_properties::provider::ExemplarCharactersPunctuationV1Marker::KEY.hashed();
                #[cfg(feature = "icu_properties")]
                const EXTENDEDPICTOGRAPHICV1MARKER: ::icu_provider::DataKeyHash =
                    ::icu_properties::provider::ExtendedPictographicV1Marker::KEY.hashed();
                #[cfg(feature = "icu_properties")]
                const EXTENDERV1MARKER: ::icu_provider::DataKeyHash = ::icu_properties::provider::ExtenderV1Marker::KEY.hashed();
                #[cfg(feature = "icu_properties")]
                const GENERALCATEGORYV1MARKER: ::icu_provider::DataKeyHash = ::icu_properties::provider::GeneralCategoryV1Marker::KEY.hashed();
                #[cfg(feature = "icu_properties")]
                const GRAPHEMEBASEV1MARKER: ::icu_provider::DataKeyHash = ::icu_properties::provider::GraphemeBaseV1Marker::KEY.hashed();
                #[cfg(feature = "icu_properties")]
                const GRAPHEMECLUSTERBREAKV1MARKER: ::icu_provider::DataKeyHash =
                    ::icu_properties::provider::GraphemeClusterBreakV1Marker::KEY.hashed();
                #[cfg(feature = "icu_properties")]
                const GRAPHEMEEXTENDV1MARKER: ::icu_provider::DataKeyHash = ::icu_properties::provider::GraphemeExtendV1Marker::KEY.hashed();
                #[cfg(feature = "icu_properties")]
                const HEXDIGITV1MARKER: ::icu_provider::DataKeyHash = ::icu_properties::provider::HexDigitV1Marker::KEY.hashed();
                #[cfg(feature = "icu_properties")]
                const IDCONTINUEV1MARKER: ::icu_provider::DataKeyHash = ::icu_properties::provider::IdContinueV1Marker::KEY.hashed();
                #[cfg(feature = "icu_properties")]
                const IDSTARTV1MARKER: ::icu_provider::DataKeyHash = ::icu_properties::provider::IdStartV1Marker::KEY.hashed();
                #[cfg(feature = "icu_properties")]
                const IDEOGRAPHICV1MARKER: ::icu_provider::DataKeyHash = ::icu_properties::provider::IdeographicV1Marker::KEY.hashed();
                #[cfg(feature = "icu_properties")]
                const IDSBINARYOPERATORV1MARKER: ::icu_provider::DataKeyHash = ::icu_properties::provider::IdsBinaryOperatorV1Marker::KEY.hashed();
                #[cfg(feature = "icu_properties")]
                const IDSTRINARYOPERATORV1MARKER: ::icu_provider::DataKeyHash = ::icu_properties::provider::IdsTrinaryOperatorV1Marker::KEY.hashed();
                #[cfg(feature = "icu_properties")]
                const JOINCONTROLV1MARKER: ::icu_provider::DataKeyHash = ::icu_properties::provider::JoinControlV1Marker::KEY.hashed();
                #[cfg(feature = "icu_properties")]
                const LINEBREAKV1MARKER: ::icu_provider::DataKeyHash = ::icu_properties::provider::LineBreakV1Marker::KEY.hashed();
                #[cfg(feature = "icu_properties")]
                const LOGICALORDEREXCEPTIONV1MARKER: ::icu_provider::DataKeyHash =
                    ::icu_properties::provider::LogicalOrderExceptionV1Marker::KEY.hashed();
                #[cfg(feature = "icu_properties")]
                const LOWERCASEV1MARKER: ::icu_provider::DataKeyHash = ::icu_properties::provider::LowercaseV1Marker::KEY.hashed();
                #[cfg(feature = "icu_properties")]
                const MATHV1MARKER: ::icu_provider::DataKeyHash = ::icu_properties::provider::MathV1Marker::KEY.hashed();
                #[cfg(feature = "icu_properties")]
                const NONCHARACTERCODEPOINTV1MARKER: ::icu_provider::DataKeyHash =
                    ::icu_properties::provider::NoncharacterCodePointV1Marker::KEY.hashed();
                #[cfg(feature = "icu_properties")]
                const PATTERNSYNTAXV1MARKER: ::icu_provider::DataKeyHash = ::icu_properties::provider::PatternSyntaxV1Marker::KEY.hashed();
                #[cfg(feature = "icu_properties")]
                const PATTERNWHITESPACEV1MARKER: ::icu_provider::DataKeyHash = ::icu_properties::provider::PatternWhiteSpaceV1Marker::KEY.hashed();
                #[cfg(feature = "icu_properties")]
                const QUOTATIONMARKV1MARKER: ::icu_provider::DataKeyHash = ::icu_properties::provider::QuotationMarkV1Marker::KEY.hashed();
                #[cfg(feature = "icu_properties")]
                const RADICALV1MARKER: ::icu_provider::DataKeyHash = ::icu_properties::provider::RadicalV1Marker::KEY.hashed();
                #[cfg(feature = "icu_properties")]
                const REGIONALINDICATORV1MARKER: ::icu_provider::DataKeyHash = ::icu_properties::provider::RegionalIndicatorV1Marker::KEY.hashed();
                #[cfg(feature = "icu_properties")]
                const SCRIPTV1MARKER: ::icu_provider::DataKeyHash = ::icu_properties::provider::ScriptV1Marker::KEY.hashed();
                #[cfg(feature = "icu_properties")]
                const SCRIPTWITHEXTENSIONSPROPERTYV1MARKER: ::icu_provider::DataKeyHash =
                    ::icu_properties::provider::ScriptWithExtensionsPropertyV1Marker::KEY.hashed();
                #[cfg(feature = "icu_properties")]
                const SENTENCEBREAKV1MARKER: ::icu_provider::DataKeyHash = ::icu_properties::provider::SentenceBreakV1Marker::KEY.hashed();
                #[cfg(feature = "icu_properties")]
                const SENTENCETERMINALV1MARKER: ::icu_provider::DataKeyHash = ::icu_properties::provider::SentenceTerminalV1Marker::KEY.hashed();
                #[cfg(feature = "icu_properties")]
                const SOFTDOTTEDV1MARKER: ::icu_provider::DataKeyHash = ::icu_properties::provider::SoftDottedV1Marker::KEY.hashed();
                #[cfg(feature = "icu_properties")]
                const TERMINALPUNCTUATIONV1MARKER: ::icu_provider::DataKeyHash =
                    ::icu_properties::provider::TerminalPunctuationV1Marker::KEY.hashed();
                #[cfg(feature = "icu_properties")]
                const UNIFIEDIDEOGRAPHV1MARKER: ::icu_provider::DataKeyHash = ::icu_properties::provider::UnifiedIdeographV1Marker::KEY.hashed();
                #[cfg(feature = "icu_properties")]
                const UPPERCASEV1MARKER: ::icu_provider::DataKeyHash = ::icu_properties::provider::UppercaseV1Marker::KEY.hashed();
                #[cfg(feature = "icu_properties")]
                const VARIATIONSELECTORV1MARKER: ::icu_provider::DataKeyHash = ::icu_properties::provider::VariationSelectorV1Marker::KEY.hashed();
                #[cfg(feature = "icu_properties")]
                const WHITESPACEV1MARKER: ::icu_provider::DataKeyHash = ::icu_properties::provider::WhiteSpaceV1Marker::KEY.hashed();
                #[cfg(feature = "icu_properties")]
                const WORDBREAKV1MARKER: ::icu_provider::DataKeyHash = ::icu_properties::provider::WordBreakV1Marker::KEY.hashed();
                #[cfg(feature = "icu_properties")]
                const XIDCONTINUEV1MARKER: ::icu_provider::DataKeyHash = ::icu_properties::provider::XidContinueV1Marker::KEY.hashed();
                #[cfg(feature = "icu_properties")]
                const XIDSTARTV1MARKER: ::icu_provider::DataKeyHash = ::icu_properties::provider::XidStartV1Marker::KEY.hashed();
                const HELLOWORLDV1MARKER: ::icu_provider::DataKeyHash = ::icu_provider::hello_world::HelloWorldV1Marker::KEY.hashed();
                const COLLATIONFALLBACKSUPPLEMENTV1MARKER: ::icu_provider::DataKeyHash =
                    ::icu_provider_adapters::fallback::provider::CollationFallbackSupplementV1Marker::KEY.hashed();
                const LOCALEFALLBACKLIKELYSUBTAGSV1MARKER: ::icu_provider::DataKeyHash =
                    ::icu_provider_adapters::fallback::provider::LocaleFallbackLikelySubtagsV1Marker::KEY.hashed();
                const LOCALEFALLBACKPARENTSV1MARKER: ::icu_provider::DataKeyHash =
                    ::icu_provider_adapters::fallback::provider::LocaleFallbackParentsV1Marker::KEY.hashed();
                #[cfg(feature = "icu_relativetime")]
                const LONGDAYRELATIVETIMEFORMATDATAV1MARKER: ::icu_provider::DataKeyHash =
                    ::icu_relativetime::provider::LongDayRelativeTimeFormatDataV1Marker::KEY.hashed();
                #[cfg(feature = "icu_relativetime")]
                const LONGHOURRELATIVETIMEFORMATDATAV1MARKER: ::icu_provider::DataKeyHash =
                    ::icu_relativetime::provider::LongHourRelativeTimeFormatDataV1Marker::KEY.hashed();
                #[cfg(feature = "icu_relativetime")]
                const LONGMINUTERELATIVETIMEFORMATDATAV1MARKER: ::icu_provider::DataKeyHash =
                    ::icu_relativetime::provider::LongMinuteRelativeTimeFormatDataV1Marker::KEY.hashed();
                #[cfg(feature = "icu_relativetime")]
                const LONGMONTHRELATIVETIMEFORMATDATAV1MARKER: ::icu_provider::DataKeyHash =
                    ::icu_relativetime::provider::LongMonthRelativeTimeFormatDataV1Marker::KEY.hashed();
                #[cfg(feature = "icu_relativetime")]
                const LONGQUARTERRELATIVETIMEFORMATDATAV1MARKER: ::icu_provider::DataKeyHash =
                    ::icu_relativetime::provider::LongQuarterRelativeTimeFormatDataV1Marker::KEY.hashed();
                #[cfg(feature = "icu_relativetime")]
                const LONGSECONDRELATIVETIMEFORMATDATAV1MARKER: ::icu_provider::DataKeyHash =
                    ::icu_relativetime::provider::LongSecondRelativeTimeFormatDataV1Marker::KEY.hashed();
                #[cfg(feature = "icu_relativetime")]
                const LONGWEEKRELATIVETIMEFORMATDATAV1MARKER: ::icu_provider::DataKeyHash =
                    ::icu_relativetime::provider::LongWeekRelativeTimeFormatDataV1Marker::KEY.hashed();
                #[cfg(feature = "icu_relativetime")]
                const LONGYEARRELATIVETIMEFORMATDATAV1MARKER: ::icu_provider::DataKeyHash =
                    ::icu_relativetime::provider::LongYearRelativeTimeFormatDataV1Marker::KEY.hashed();
                #[cfg(feature = "icu_relativetime")]
                const NARROWDAYRELATIVETIMEFORMATDATAV1MARKER: ::icu_provider::DataKeyHash =
                    ::icu_relativetime::provider::NarrowDayRelativeTimeFormatDataV1Marker::KEY.hashed();
                #[cfg(feature = "icu_relativetime")]
                const NARROWHOURRELATIVETIMEFORMATDATAV1MARKER: ::icu_provider::DataKeyHash =
                    ::icu_relativetime::provider::NarrowHourRelativeTimeFormatDataV1Marker::KEY.hashed();
                #[cfg(feature = "icu_relativetime")]
                const NARROWMINUTERELATIVETIMEFORMATDATAV1MARKER: ::icu_provider::DataKeyHash =
                    ::icu_relativetime::provider::NarrowMinuteRelativeTimeFormatDataV1Marker::KEY.hashed();
                #[cfg(feature = "icu_relativetime")]
                const NARROWMONTHRELATIVETIMEFORMATDATAV1MARKER: ::icu_provider::DataKeyHash =
                    ::icu_relativetime::provider::NarrowMonthRelativeTimeFormatDataV1Marker::KEY.hashed();
                #[cfg(feature = "icu_relativetime")]
                const NARROWQUARTERRELATIVETIMEFORMATDATAV1MARKER: ::icu_provider::DataKeyHash =
                    ::icu_relativetime::provider::NarrowQuarterRelativeTimeFormatDataV1Marker::KEY.hashed();
                #[cfg(feature = "icu_relativetime")]
                const NARROWSECONDRELATIVETIMEFORMATDATAV1MARKER: ::icu_provider::DataKeyHash =
                    ::icu_relativetime::provider::NarrowSecondRelativeTimeFormatDataV1Marker::KEY.hashed();
                #[cfg(feature = "icu_relativetime")]
                const NARROWWEEKRELATIVETIMEFORMATDATAV1MARKER: ::icu_provider::DataKeyHash =
                    ::icu_relativetime::provider::NarrowWeekRelativeTimeFormatDataV1Marker::KEY.hashed();
                #[cfg(feature = "icu_relativetime")]
                const NARROWYEARRELATIVETIMEFORMATDATAV1MARKER: ::icu_provider::DataKeyHash =
                    ::icu_relativetime::provider::NarrowYearRelativeTimeFormatDataV1Marker::KEY.hashed();
                #[cfg(feature = "icu_relativetime")]
                const SHORTDAYRELATIVETIMEFORMATDATAV1MARKER: ::icu_provider::DataKeyHash =
                    ::icu_relativetime::provider::ShortDayRelativeTimeFormatDataV1Marker::KEY.hashed();
                #[cfg(feature = "icu_relativetime")]
                const SHORTHOURRELATIVETIMEFORMATDATAV1MARKER: ::icu_provider::DataKeyHash =
                    ::icu_relativetime::provider::ShortHourRelativeTimeFormatDataV1Marker::KEY.hashed();
                #[cfg(feature = "icu_relativetime")]
                const SHORTMINUTERELATIVETIMEFORMATDATAV1MARKER: ::icu_provider::DataKeyHash =
                    ::icu_relativetime::provider::ShortMinuteRelativeTimeFormatDataV1Marker::KEY.hashed();
                #[cfg(feature = "icu_relativetime")]
                const SHORTMONTHRELATIVETIMEFORMATDATAV1MARKER: ::icu_provider::DataKeyHash =
                    ::icu_relativetime::provider::ShortMonthRelativeTimeFormatDataV1Marker::KEY.hashed();
                #[cfg(feature = "icu_relativetime")]
                const SHORTQUARTERRELATIVETIMEFORMATDATAV1MARKER: ::icu_provider::DataKeyHash =
                    ::icu_relativetime::provider::ShortQuarterRelativeTimeFormatDataV1Marker::KEY.hashed();
                #[cfg(feature = "icu_relativetime")]
                const SHORTSECONDRELATIVETIMEFORMATDATAV1MARKER: ::icu_provider::DataKeyHash =
                    ::icu_relativetime::provider::ShortSecondRelativeTimeFormatDataV1Marker::KEY.hashed();
                #[cfg(feature = "icu_relativetime")]
                const SHORTWEEKRELATIVETIMEFORMATDATAV1MARKER: ::icu_provider::DataKeyHash =
                    ::icu_relativetime::provider::ShortWeekRelativeTimeFormatDataV1Marker::KEY.hashed();
                #[cfg(feature = "icu_relativetime")]
                const SHORTYEARRELATIVETIMEFORMATDATAV1MARKER: ::icu_provider::DataKeyHash =
                    ::icu_relativetime::provider::ShortYearRelativeTimeFormatDataV1Marker::KEY.hashed();
                #[cfg(feature = "icu_segmenter")]
                const GRAPHEMECLUSTERBREAKDATAV1MARKER: ::icu_provider::DataKeyHash =
                    ::icu_segmenter::provider::GraphemeClusterBreakDataV1Marker::KEY.hashed();
                #[cfg(feature = "icu_segmenter")]
                const LINEBREAKDATAV1MARKER: ::icu_provider::DataKeyHash = ::icu_segmenter::provider::LineBreakDataV1Marker::KEY.hashed();
                #[cfg(feature = "icu_segmenter")]
                const LSTMDATAV1MARKER: ::icu_provider::DataKeyHash = ::icu_segmenter::provider::LstmDataV1Marker::KEY.hashed();
                #[cfg(feature = "icu_segmenter")]
                const SENTENCEBREAKDATAV1MARKER: ::icu_provider::DataKeyHash = ::icu_segmenter::provider::SentenceBreakDataV1Marker::KEY.hashed();
                #[cfg(feature = "icu_segmenter")]
                const UCHARDICTIONARYBREAKDATAV1MARKER: ::icu_provider::DataKeyHash =
                    ::icu_segmenter::provider::UCharDictionaryBreakDataV1Marker::KEY.hashed();
                #[cfg(feature = "icu_segmenter")]
                const WORDBREAKDATAV1MARKER: ::icu_provider::DataKeyHash = ::icu_segmenter::provider::WordBreakDataV1Marker::KEY.hashed();
                #[cfg(feature = "icu_timezone")]
                const METAZONEPERIODV1MARKER: ::icu_provider::DataKeyHash = ::icu_timezone::provider::MetazonePeriodV1Marker::KEY.hashed();
                #[allow(clippy::match_single_binding)]
                match key.hashed() {
                    #[cfg(feature = "icu_calendar")]
                    JAPANESEERASV1MARKER => calendar::japanese_v1::lookup(&req.locale).map(AnyPayload::from_static_ref),
                    #[cfg(feature = "icu_calendar")]
                    JAPANESEEXTENDEDERASV1MARKER => calendar::japanext_v1::lookup(&req.locale).map(AnyPayload::from_static_ref),
                    #[cfg(feature = "icu_calendar")]
                    WEEKDATAV1MARKER => datetime::week_data_v1::lookup(&req.locale).map(AnyPayload::from_static_ref),
                    #[cfg(feature = "icu_casemapping")]
                    CASEMAPPINGV1MARKER => props::casemap_v1::lookup(&req.locale).map(AnyPayload::from_static_ref),
                    #[cfg(feature = "icu_collator")]
                    COLLATIONDATAV1MARKER => collator::data_v1::lookup(&req.locale).map(AnyPayload::from_static_ref),
                    #[cfg(feature = "icu_collator")]
                    COLLATIONDIACRITICSV1MARKER => collator::dia_v1::lookup(&req.locale).map(AnyPayload::from_static_ref),
                    #[cfg(feature = "icu_collator")]
                    COLLATIONJAMOV1MARKER => collator::jamo_v1::lookup(&req.locale).map(AnyPayload::from_static_ref),
                    #[cfg(feature = "icu_collator")]
                    COLLATIONMETADATAV1MARKER => collator::meta_v1::lookup(&req.locale).map(AnyPayload::from_static_ref),
                    #[cfg(feature = "icu_collator")]
                    COLLATIONREORDERINGV1MARKER => collator::reord_v1::lookup(&req.locale).map(AnyPayload::from_static_ref),
                    #[cfg(feature = "icu_collator")]
                    COLLATIONSPECIALPRIMARIESV1MARKER => collator::prim_v1::lookup(&req.locale).map(AnyPayload::from_static_ref),
                    #[cfg(feature = "icu_compactdecimal")]
                    LONGCOMPACTDECIMALFORMATDATAV1MARKER => compactdecimal::long_v1::lookup(&req.locale).map(AnyPayload::from_static_ref),
                    #[cfg(feature = "icu_compactdecimal")]
                    SHORTCOMPACTDECIMALFORMATDATAV1MARKER => compactdecimal::short_v1::lookup(&req.locale).map(AnyPayload::from_static_ref),
                    #[cfg(feature = "icu_datetime")]
                    BUDDHISTDATELENGTHSV1MARKER => datetime::buddhist::datelengths_v1::lookup(&req.locale).map(AnyPayload::from_static_ref),
                    #[cfg(feature = "icu_datetime")]
                    BUDDHISTDATESYMBOLSV1MARKER => datetime::buddhist::datesymbols_v1::lookup(&req.locale).map(AnyPayload::from_static_ref),
                    #[cfg(feature = "icu_datetime")]
                    COPTICDATELENGTHSV1MARKER => datetime::coptic::datelengths_v1::lookup(&req.locale).map(AnyPayload::from_static_ref),
                    #[cfg(feature = "icu_datetime")]
                    COPTICDATESYMBOLSV1MARKER => datetime::coptic::datesymbols_v1::lookup(&req.locale).map(AnyPayload::from_static_ref),
                    #[cfg(feature = "icu_datetime_experimental")]
                    DATESKELETONPATTERNSV1MARKER => datetime::skeletons_v1::lookup(&req.locale)
                        .map(zerofrom::ZeroFrom::zero_from)
                        .map(DataPayload::<::icu_datetime::provider::calendar::DateSkeletonPatternsV1Marker>::from_owned)
                        .map(DataPayload::wrap_into_any_payload),
                    #[cfg(feature = "icu_datetime")]
                    ETHIOPIANDATELENGTHSV1MARKER => datetime::ethiopic::datelengths_v1::lookup(&req.locale).map(AnyPayload::from_static_ref),
                    #[cfg(feature = "icu_datetime")]
                    ETHIOPIANDATESYMBOLSV1MARKER => datetime::ethiopic::datesymbols_v1::lookup(&req.locale).map(AnyPayload::from_static_ref),
                    #[cfg(feature = "icu_datetime")]
                    GREGORIANDATELENGTHSV1MARKER => datetime::gregory::datelengths_v1::lookup(&req.locale).map(AnyPayload::from_static_ref),
                    #[cfg(feature = "icu_datetime")]
                    GREGORIANDATESYMBOLSV1MARKER => datetime::gregory::datesymbols_v1::lookup(&req.locale).map(AnyPayload::from_static_ref),
                    #[cfg(feature = "icu_datetime")]
                    INDIANDATELENGTHSV1MARKER => datetime::indian::datelengths_v1::lookup(&req.locale).map(AnyPayload::from_static_ref),
                    #[cfg(feature = "icu_datetime")]
                    INDIANDATESYMBOLSV1MARKER => datetime::indian::datesymbols_v1::lookup(&req.locale).map(AnyPayload::from_static_ref),
                    #[cfg(feature = "icu_datetime")]
                    JAPANESEDATELENGTHSV1MARKER => datetime::japanese::datelengths_v1::lookup(&req.locale).map(AnyPayload::from_static_ref),
                    #[cfg(feature = "icu_datetime")]
                    JAPANESEDATESYMBOLSV1MARKER => datetime::japanese::datesymbols_v1::lookup(&req.locale).map(AnyPayload::from_static_ref),
                    #[cfg(feature = "icu_datetime")]
                    JAPANESEEXTENDEDDATELENGTHSV1MARKER => datetime::japanext::datelengths_v1::lookup(&req.locale).map(AnyPayload::from_static_ref),
                    #[cfg(feature = "icu_datetime")]
                    JAPANESEEXTENDEDDATESYMBOLSV1MARKER => datetime::japanext::datesymbols_v1::lookup(&req.locale).map(AnyPayload::from_static_ref),
                    #[cfg(feature = "icu_datetime")]
                    TIMELENGTHSV1MARKER => datetime::timelengths_v1::lookup(&req.locale).map(AnyPayload::from_static_ref),
                    #[cfg(feature = "icu_datetime")]
                    TIMESYMBOLSV1MARKER => datetime::timesymbols_v1::lookup(&req.locale).map(AnyPayload::from_static_ref),
                    #[cfg(feature = "icu_datetime")]
                    EXEMPLARCITIESV1MARKER => time_zone::exemplar_cities_v1::lookup(&req.locale).map(AnyPayload::from_static_ref),
                    #[cfg(feature = "icu_datetime")]
                    METAZONEGENERICNAMESLONGV1MARKER => time_zone::generic_long_v1::lookup(&req.locale).map(AnyPayload::from_static_ref),
                    #[cfg(feature = "icu_datetime")]
                    METAZONEGENERICNAMESSHORTV1MARKER => time_zone::generic_short_v1::lookup(&req.locale).map(AnyPayload::from_static_ref),
                    #[cfg(feature = "icu_datetime")]
                    METAZONESPECIFICNAMESLONGV1MARKER => time_zone::specific_long_v1::lookup(&req.locale).map(AnyPayload::from_static_ref),
                    #[cfg(feature = "icu_datetime")]
                    METAZONESPECIFICNAMESSHORTV1MARKER => time_zone::specific_short_v1::lookup(&req.locale).map(AnyPayload::from_static_ref),
                    #[cfg(feature = "icu_datetime")]
                    TIMEZONEFORMATSV1MARKER => time_zone::formats_v1::lookup(&req.locale).map(AnyPayload::from_static_ref),
                    #[cfg(feature = "icu_decimal")]
                    DECIMALSYMBOLSV1MARKER => decimal::symbols_v1::lookup(&req.locale).map(AnyPayload::from_static_ref),
                    #[cfg(feature = "icu_displaynames")]
                    LANGUAGEDISPLAYNAMESV1MARKER => displaynames::languages_v1::lookup(&req.locale).map(AnyPayload::from_static_ref),
                    #[cfg(feature = "icu_displaynames")]
                    TERRITORYDISPLAYNAMESV1MARKER => displaynames::territories_v1::lookup(&req.locale).map(AnyPayload::from_static_ref),
                    #[cfg(feature = "icu_list")]
                    ANDLISTV1MARKER => list::and_v1::lookup(&req.locale).map(AnyPayload::from_static_ref),
                    #[cfg(feature = "icu_list")]
                    ORLISTV1MARKER => list::or_v1::lookup(&req.locale).map(AnyPayload::from_static_ref),
                    #[cfg(feature = "icu_list")]
                    UNITLISTV1MARKER => list::unit_v1::lookup(&req.locale).map(AnyPayload::from_static_ref),
                    #[cfg(feature = "icu_locid_transform")]
                    ALIASESV1MARKER => locid_transform::aliases_v1::lookup(&req.locale).map(AnyPayload::from_static_ref),
                    #[cfg(feature = "icu_locid_transform")]
                    LIKELYSUBTAGSV1MARKER => locid_transform::likelysubtags_v1::lookup(&req.locale).map(AnyPayload::from_static_ref),
                    #[cfg(feature = "icu_normalizer")]
                    CANONICALCOMPOSITIONSV1MARKER => normalizer::comp_v1::lookup(&req.locale).map(AnyPayload::from_static_ref),
                    #[cfg(feature = "icu_normalizer")]
                    CANONICALDECOMPOSITIONDATAV1MARKER => normalizer::nfd_v1::lookup(&req.locale).map(AnyPayload::from_static_ref),
                    #[cfg(feature = "icu_normalizer")]
                    CANONICALDECOMPOSITIONTABLESV1MARKER => normalizer::nfdex_v1::lookup(&req.locale).map(AnyPayload::from_static_ref),
                    #[cfg(feature = "icu_normalizer")]
                    COMPATIBILITYDECOMPOSITIONSUPPLEMENTV1MARKER => normalizer::nfkd_v1::lookup(&req.locale).map(AnyPayload::from_static_ref),
                    #[cfg(feature = "icu_normalizer")]
                    COMPATIBILITYDECOMPOSITIONTABLESV1MARKER => normalizer::nfkdex_v1::lookup(&req.locale).map(AnyPayload::from_static_ref),
                    #[cfg(feature = "icu_normalizer")]
                    NONRECURSIVEDECOMPOSITIONSUPPLEMENTV1MARKER => normalizer::decomp_v1::lookup(&req.locale).map(AnyPayload::from_static_ref),
                    #[cfg(feature = "icu_normalizer")]
                    UTS46DECOMPOSITIONSUPPLEMENTV1MARKER => normalizer::uts46d_v1::lookup(&req.locale).map(AnyPayload::from_static_ref),
                    #[cfg(feature = "icu_plurals")]
                    CARDINALV1MARKER => plurals::cardinal_v1::lookup(&req.locale).map(AnyPayload::from_static_ref),
                    #[cfg(feature = "icu_plurals")]
                    ORDINALV1MARKER => plurals::ordinal_v1::lookup(&req.locale).map(AnyPayload::from_static_ref),
                    #[cfg(feature = "icu_properties")]
                    ALPHABETICV1MARKER => props::alpha_v1::lookup(&req.locale).map(AnyPayload::from_static_ref),
                    #[cfg(feature = "icu_properties")]
                    ASCIIHEXDIGITV1MARKER => props::ahex_v1::lookup(&req.locale).map(AnyPayload::from_static_ref),
                    #[cfg(feature = "icu_properties")]
                    BASICEMOJIV1MARKER => props::basic_emoji_v1::lookup(&req.locale).map(AnyPayload::from_static_ref),
                    #[cfg(feature = "icu_properties")]
                    BIDICLASSV1MARKER => props::bc_v1::lookup(&req.locale).map(AnyPayload::from_static_ref),
                    #[cfg(feature = "icu_properties")]
                    BIDICONTROLV1MARKER => props::bidi_c_v1::lookup(&req.locale).map(AnyPayload::from_static_ref),
                    #[cfg(feature = "icu_properties")]
                    BIDIMIRROREDV1MARKER => props::bidi_m_v1::lookup(&req.locale).map(AnyPayload::from_static_ref),
                    #[cfg(feature = "icu_properties")]
                    CANONICALCOMBININGCLASSV1MARKER => props::ccc_v1::lookup(&req.locale).map(AnyPayload::from_static_ref),
                    #[cfg(feature = "icu_properties")]
                    CASEIGNORABLEV1MARKER => props::ci_v1::lookup(&req.locale).map(AnyPayload::from_static_ref),
                    #[cfg(feature = "icu_properties")]
                    CASEDV1MARKER => props::cased_v1::lookup(&req.locale).map(AnyPayload::from_static_ref),
                    #[cfg(feature = "icu_properties")]
                    CHANGESWHENCASEFOLDEDV1MARKER => props::cwcf_v1::lookup(&req.locale).map(AnyPayload::from_static_ref),
                    #[cfg(feature = "icu_properties")]
                    CHANGESWHENLOWERCASEDV1MARKER => props::cwl_v1::lookup(&req.locale).map(AnyPayload::from_static_ref),
                    #[cfg(feature = "icu_properties")]
                    CHANGESWHENNFKCCASEFOLDEDV1MARKER => props::cwkcf_v1::lookup(&req.locale).map(AnyPayload::from_static_ref),
                    #[cfg(feature = "icu_properties")]
                    CHANGESWHENTITLECASEDV1MARKER => props::cwt_v1::lookup(&req.locale).map(AnyPayload::from_static_ref),
                    #[cfg(feature = "icu_properties")]
                    CHANGESWHENUPPERCASEDV1MARKER => props::cwu_v1::lookup(&req.locale).map(AnyPayload::from_static_ref),
                    #[cfg(feature = "icu_properties")]
                    DASHV1MARKER => props::dash_v1::lookup(&req.locale).map(AnyPayload::from_static_ref),
                    #[cfg(feature = "icu_properties")]
                    DEFAULTIGNORABLECODEPOINTV1MARKER => props::di_v1::lookup(&req.locale).map(AnyPayload::from_static_ref),
                    #[cfg(feature = "icu_properties")]
                    DEPRECATEDV1MARKER => props::dep_v1::lookup(&req.locale).map(AnyPayload::from_static_ref),
                    #[cfg(feature = "icu_properties")]
                    DIACRITICV1MARKER => props::dia_v1::lookup(&req.locale).map(AnyPayload::from_static_ref),
                    #[cfg(feature = "icu_properties")]
                    EASTASIANWIDTHV1MARKER => props::ea_v1::lookup(&req.locale).map(AnyPayload::from_static_ref),
                    #[cfg(feature = "icu_properties")]
                    EMOJICOMPONENTV1MARKER => props::ecomp_v1::lookup(&req.locale).map(AnyPayload::from_static_ref),
                    #[cfg(feature = "icu_properties")]
                    EMOJIMODIFIERBASEV1MARKER => props::ebase_v1::lookup(&req.locale).map(AnyPayload::from_static_ref),
                    #[cfg(feature = "icu_properties")]
                    EMOJIMODIFIERV1MARKER => props::emod_v1::lookup(&req.locale).map(AnyPayload::from_static_ref),
                    #[cfg(feature = "icu_properties")]
                    EMOJIPRESENTATIONV1MARKER => props::epres_v1::lookup(&req.locale).map(AnyPayload::from_static_ref),
                    #[cfg(feature = "icu_properties")]
                    EMOJIV1MARKER => props::emoji_v1::lookup(&req.locale).map(AnyPayload::from_static_ref),
                    #[cfg(feature = "icu_properties")]
                    EXEMPLARCHARACTERSAUXILIARYV1MARKER => props::exemplarchars::auxiliary_v1::lookup(&req.locale).map(AnyPayload::from_static_ref),
                    #[cfg(feature = "icu_properties")]
                    EXEMPLARCHARACTERSINDEXV1MARKER => props::exemplarchars::index_v1::lookup(&req.locale).map(AnyPayload::from_static_ref),
                    #[cfg(feature = "icu_properties")]
                    EXEMPLARCHARACTERSMAINV1MARKER => props::exemplarchars::main_v1::lookup(&req.locale).map(AnyPayload::from_static_ref),
                    #[cfg(feature = "icu_properties")]
                    EXEMPLARCHARACTERSNUMBERSV1MARKER => props::exemplarchars::numbers_v1::lookup(&req.locale).map(AnyPayload::from_static_ref),
                    #[cfg(feature = "icu_properties")]
                    EXEMPLARCHARACTERSPUNCTUATIONV1MARKER => {
                        props::exemplarchars::punctuation_v1::lookup(&req.locale).map(AnyPayload::from_static_ref)
                    }
                    #[cfg(feature = "icu_properties")]
                    EXTENDEDPICTOGRAPHICV1MARKER => props::extpict_v1::lookup(&req.locale).map(AnyPayload::from_static_ref),
                    #[cfg(feature = "icu_properties")]
                    EXTENDERV1MARKER => props::ext_v1::lookup(&req.locale).map(AnyPayload::from_static_ref),
                    #[cfg(feature = "icu_properties")]
                    GENERALCATEGORYV1MARKER => props::gc_v1::lookup(&req.locale).map(AnyPayload::from_static_ref),
                    #[cfg(feature = "icu_properties")]
                    GRAPHEMEBASEV1MARKER => props::gr_base_v1::lookup(&req.locale).map(AnyPayload::from_static_ref),
                    #[cfg(feature = "icu_properties")]
                    GRAPHEMECLUSTERBREAKV1MARKER => props::gcb_v1::lookup(&req.locale).map(AnyPayload::from_static_ref),
                    #[cfg(feature = "icu_properties")]
                    GRAPHEMEEXTENDV1MARKER => props::gr_ext_v1::lookup(&req.locale).map(AnyPayload::from_static_ref),
                    #[cfg(feature = "icu_properties")]
                    HEXDIGITV1MARKER => props::hex_v1::lookup(&req.locale).map(AnyPayload::from_static_ref),
                    #[cfg(feature = "icu_properties")]
                    IDCONTINUEV1MARKER => props::idc_v1::lookup(&req.locale).map(AnyPayload::from_static_ref),
                    #[cfg(feature = "icu_properties")]
                    IDSTARTV1MARKER => props::ids_v1::lookup(&req.locale).map(AnyPayload::from_static_ref),
                    #[cfg(feature = "icu_properties")]
                    IDEOGRAPHICV1MARKER => props::ideo_v1::lookup(&req.locale).map(AnyPayload::from_static_ref),
                    #[cfg(feature = "icu_properties")]
                    IDSBINARYOPERATORV1MARKER => props::idsb_v1::lookup(&req.locale).map(AnyPayload::from_static_ref),
                    #[cfg(feature = "icu_properties")]
                    IDSTRINARYOPERATORV1MARKER => props::idst_v1::lookup(&req.locale).map(AnyPayload::from_static_ref),
                    #[cfg(feature = "icu_properties")]
                    JOINCONTROLV1MARKER => props::join_c_v1::lookup(&req.locale).map(AnyPayload::from_static_ref),
                    #[cfg(feature = "icu_properties")]
                    LINEBREAKV1MARKER => props::lb_v1::lookup(&req.locale).map(AnyPayload::from_static_ref),
                    #[cfg(feature = "icu_properties")]
                    LOGICALORDEREXCEPTIONV1MARKER => props::loe_v1::lookup(&req.locale).map(AnyPayload::from_static_ref),
                    #[cfg(feature = "icu_properties")]
                    LOWERCASEV1MARKER => props::lower_v1::lookup(&req.locale).map(AnyPayload::from_static_ref),
                    #[cfg(feature = "icu_properties")]
                    MATHV1MARKER => props::math_v1::lookup(&req.locale).map(AnyPayload::from_static_ref),
                    #[cfg(feature = "icu_properties")]
                    NONCHARACTERCODEPOINTV1MARKER => props::nchar_v1::lookup(&req.locale).map(AnyPayload::from_static_ref),
                    #[cfg(feature = "icu_properties")]
                    PATTERNSYNTAXV1MARKER => props::pat_syn_v1::lookup(&req.locale).map(AnyPayload::from_static_ref),
                    #[cfg(feature = "icu_properties")]
                    PATTERNWHITESPACEV1MARKER => props::pat_ws_v1::lookup(&req.locale).map(AnyPayload::from_static_ref),
                    #[cfg(feature = "icu_properties")]
                    QUOTATIONMARKV1MARKER => props::qmark_v1::lookup(&req.locale).map(AnyPayload::from_static_ref),
                    #[cfg(feature = "icu_properties")]
                    RADICALV1MARKER => props::radical_v1::lookup(&req.locale).map(AnyPayload::from_static_ref),
                    #[cfg(feature = "icu_properties")]
                    REGIONALINDICATORV1MARKER => props::ri_v1::lookup(&req.locale).map(AnyPayload::from_static_ref),
                    #[cfg(feature = "icu_properties")]
                    SCRIPTV1MARKER => props::sc_v1::lookup(&req.locale).map(AnyPayload::from_static_ref),
                    #[cfg(feature = "icu_properties")]
                    SCRIPTWITHEXTENSIONSPROPERTYV1MARKER => props::scx_v1::lookup(&req.locale).map(AnyPayload::from_static_ref),
                    #[cfg(feature = "icu_properties")]
                    SENTENCEBREAKV1MARKER => props::sb_v1::lookup(&req.locale).map(AnyPayload::from_static_ref),
                    #[cfg(feature = "icu_properties")]
                    SENTENCETERMINALV1MARKER => props::sterm_v1::lookup(&req.locale).map(AnyPayload::from_static_ref),
                    #[cfg(feature = "icu_properties")]
                    SOFTDOTTEDV1MARKER => props::sd_v1::lookup(&req.locale).map(AnyPayload::from_static_ref),
                    #[cfg(feature = "icu_properties")]
                    TERMINALPUNCTUATIONV1MARKER => props::term_v1::lookup(&req.locale).map(AnyPayload::from_static_ref),
                    #[cfg(feature = "icu_properties")]
                    UNIFIEDIDEOGRAPHV1MARKER => props::uideo_v1::lookup(&req.locale).map(AnyPayload::from_static_ref),
                    #[cfg(feature = "icu_properties")]
                    UPPERCASEV1MARKER => props::upper_v1::lookup(&req.locale).map(AnyPayload::from_static_ref),
                    #[cfg(feature = "icu_properties")]
                    VARIATIONSELECTORV1MARKER => props::vs_v1::lookup(&req.locale).map(AnyPayload::from_static_ref),
                    #[cfg(feature = "icu_properties")]
                    WHITESPACEV1MARKER => props::wspace_v1::lookup(&req.locale).map(AnyPayload::from_static_ref),
                    #[cfg(feature = "icu_properties")]
                    WORDBREAKV1MARKER => props::wb_v1::lookup(&req.locale).map(AnyPayload::from_static_ref),
                    #[cfg(feature = "icu_properties")]
                    XIDCONTINUEV1MARKER => props::xidc_v1::lookup(&req.locale).map(AnyPayload::from_static_ref),
                    #[cfg(feature = "icu_properties")]
                    XIDSTARTV1MARKER => props::xids_v1::lookup(&req.locale).map(AnyPayload::from_static_ref),
                    HELLOWORLDV1MARKER => core::helloworld_v1::lookup(&req.locale).map(AnyPayload::from_static_ref),
                    COLLATIONFALLBACKSUPPLEMENTV1MARKER => fallback::supplement::co_v1::lookup(&req.locale).map(AnyPayload::from_static_ref),
                    LOCALEFALLBACKLIKELYSUBTAGSV1MARKER => fallback::likelysubtags_v1::lookup(&req.locale).map(AnyPayload::from_static_ref),
                    LOCALEFALLBACKPARENTSV1MARKER => fallback::parents_v1::lookup(&req.locale).map(AnyPayload::from_static_ref),
                    #[cfg(feature = "icu_relativetime")]
                    LONGDAYRELATIVETIMEFORMATDATAV1MARKER => relativetime::long::day_v1::lookup(&req.locale).map(AnyPayload::from_static_ref),
                    #[cfg(feature = "icu_relativetime")]
                    LONGHOURRELATIVETIMEFORMATDATAV1MARKER => relativetime::long::hour_v1::lookup(&req.locale).map(AnyPayload::from_static_ref),
                    #[cfg(feature = "icu_relativetime")]
                    LONGMINUTERELATIVETIMEFORMATDATAV1MARKER => relativetime::long::minute_v1::lookup(&req.locale).map(AnyPayload::from_static_ref),
                    #[cfg(feature = "icu_relativetime")]
                    LONGMONTHRELATIVETIMEFORMATDATAV1MARKER => relativetime::long::month_v1::lookup(&req.locale).map(AnyPayload::from_static_ref),
                    #[cfg(feature = "icu_relativetime")]
                    LONGQUARTERRELATIVETIMEFORMATDATAV1MARKER => relativetime::long::quarter_v1::lookup(&req.locale).map(AnyPayload::from_static_ref),
                    #[cfg(feature = "icu_relativetime")]
                    LONGSECONDRELATIVETIMEFORMATDATAV1MARKER => relativetime::long::second_v1::lookup(&req.locale).map(AnyPayload::from_static_ref),
                    #[cfg(feature = "icu_relativetime")]
                    LONGWEEKRELATIVETIMEFORMATDATAV1MARKER => relativetime::long::week_v1::lookup(&req.locale).map(AnyPayload::from_static_ref),
                    #[cfg(feature = "icu_relativetime")]
                    LONGYEARRELATIVETIMEFORMATDATAV1MARKER => relativetime::long::year_v1::lookup(&req.locale).map(AnyPayload::from_static_ref),
                    #[cfg(feature = "icu_relativetime")]
                    NARROWDAYRELATIVETIMEFORMATDATAV1MARKER => relativetime::narrow::day_v1::lookup(&req.locale).map(AnyPayload::from_static_ref),
                    #[cfg(feature = "icu_relativetime")]
                    NARROWHOURRELATIVETIMEFORMATDATAV1MARKER => relativetime::narrow::hour_v1::lookup(&req.locale).map(AnyPayload::from_static_ref),
                    #[cfg(feature = "icu_relativetime")]
                    NARROWMINUTERELATIVETIMEFORMATDATAV1MARKER => {
                        relativetime::narrow::minute_v1::lookup(&req.locale).map(AnyPayload::from_static_ref)
                    }
                    #[cfg(feature = "icu_relativetime")]
                    NARROWMONTHRELATIVETIMEFORMATDATAV1MARKER => relativetime::narrow::month_v1::lookup(&req.locale).map(AnyPayload::from_static_ref),
                    #[cfg(feature = "icu_relativetime")]
                    NARROWQUARTERRELATIVETIMEFORMATDATAV1MARKER => {
                        relativetime::narrow::quarter_v1::lookup(&req.locale).map(AnyPayload::from_static_ref)
                    }
                    #[cfg(feature = "icu_relativetime")]
                    NARROWSECONDRELATIVETIMEFORMATDATAV1MARKER => {
                        relativetime::narrow::second_v1::lookup(&req.locale).map(AnyPayload::from_static_ref)
                    }
                    #[cfg(feature = "icu_relativetime")]
                    NARROWWEEKRELATIVETIMEFORMATDATAV1MARKER => relativetime::narrow::week_v1::lookup(&req.locale).map(AnyPayload::from_static_ref),
                    #[cfg(feature = "icu_relativetime")]
                    NARROWYEARRELATIVETIMEFORMATDATAV1MARKER => relativetime::narrow::year_v1::lookup(&req.locale).map(AnyPayload::from_static_ref),
                    #[cfg(feature = "icu_relativetime")]
                    SHORTDAYRELATIVETIMEFORMATDATAV1MARKER => relativetime::short::day_v1::lookup(&req.locale).map(AnyPayload::from_static_ref),
                    #[cfg(feature = "icu_relativetime")]
                    SHORTHOURRELATIVETIMEFORMATDATAV1MARKER => relativetime::short::hour_v1::lookup(&req.locale).map(AnyPayload::from_static_ref),
                    #[cfg(feature = "icu_relativetime")]
                    SHORTMINUTERELATIVETIMEFORMATDATAV1MARKER => relativetime::short::minute_v1::lookup(&req.locale).map(AnyPayload::from_static_ref),
                    #[cfg(feature = "icu_relativetime")]
                    SHORTMONTHRELATIVETIMEFORMATDATAV1MARKER => relativetime::short::month_v1::lookup(&req.locale).map(AnyPayload::from_static_ref),
                    #[cfg(feature = "icu_relativetime")]
                    SHORTQUARTERRELATIVETIMEFORMATDATAV1MARKER => {
                        relativetime::short::quarter_v1::lookup(&req.locale).map(AnyPayload::from_static_ref)
                    }
                    #[cfg(feature = "icu_relativetime")]
                    SHORTSECONDRELATIVETIMEFORMATDATAV1MARKER => relativetime::short::second_v1::lookup(&req.locale).map(AnyPayload::from_static_ref),
                    #[cfg(feature = "icu_relativetime")]
                    SHORTWEEKRELATIVETIMEFORMATDATAV1MARKER => relativetime::short::week_v1::lookup(&req.locale).map(AnyPayload::from_static_ref),
                    #[cfg(feature = "icu_relativetime")]
                    SHORTYEARRELATIVETIMEFORMATDATAV1MARKER => relativetime::short::year_v1::lookup(&req.locale).map(AnyPayload::from_static_ref),
                    #[cfg(feature = "icu_segmenter")]
                    GRAPHEMECLUSTERBREAKDATAV1MARKER => segmenter::grapheme_v1::lookup(&req.locale).map(AnyPayload::from_static_ref),
                    #[cfg(feature = "icu_segmenter")]
                    LINEBREAKDATAV1MARKER => segmenter::line_v1::lookup(&req.locale).map(AnyPayload::from_static_ref),
                    #[cfg(feature = "icu_segmenter")]
                    LSTMDATAV1MARKER => segmenter::lstm_v1::lookup(&req.locale).map(AnyPayload::from_static_ref),
                    #[cfg(feature = "icu_segmenter")]
                    SENTENCEBREAKDATAV1MARKER => segmenter::sentence_v1::lookup(&req.locale).map(AnyPayload::from_static_ref),
                    #[cfg(feature = "icu_segmenter")]
                    UCHARDICTIONARYBREAKDATAV1MARKER => segmenter::dictionary_v1::lookup(&req.locale).map(AnyPayload::from_static_ref),
                    #[cfg(feature = "icu_segmenter")]
                    WORDBREAKDATAV1MARKER => segmenter::word_v1::lookup(&req.locale).map(AnyPayload::from_static_ref),
                    #[cfg(feature = "icu_timezone")]
                    METAZONEPERIODV1MARKER => time_zone::metazone_period_v1::lookup(&req.locale).map(AnyPayload::from_static_ref),
                    _ => return Err(DataErrorKind::MissingDataKey.with_req(key, req)),
                }
                .map(|payload| AnyResponse {
                    payload: Some(payload),
                    metadata: Default::default(),
                })
                .ok_or_else(|| DataErrorKind::MissingLocale.with_req(key, req))
            }
        }
    };
}
