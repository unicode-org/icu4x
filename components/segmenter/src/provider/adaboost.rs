// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

//! Data provider struct definitions for the `AdaBoost` segmenter.

use icu_provider::prelude::*;
use zerovec::ZeroMap;

/// The data powering the Chinese `AdaBoost` segmentation model.
///
/// <div class="stab unstable">
/// 🚧 This code is considered unstable; it may change at any time, in breaking or non-breaking ways,
/// including in SemVer minor releases. While the serde representation of data structs is guaranteed
/// to be stable, their Rust representation might not be. Use with caution.
/// </div>
#[derive(Debug, PartialEq, Clone, yoke::Yokeable, zerofrom::ZeroFrom)]
#[cfg_attr(feature = "datagen", derive(serde::Serialize, databake::Bake))]
#[cfg_attr(
    feature = "datagen",
    databake(path = icu_segmenter::provider::adaboost)
)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
#[yoke(prove_covariance_manually)]
pub struct AdaboostData<'data> {
    /// The model bias.
    pub bias: i32,
    /// Weights for the scalar two positions to the left of a candidate boundary.
    #[cfg_attr(feature = "serde", serde(borrow))]
    pub uw2: ZeroMap<'data, char, i16>,
    /// Weights for the scalar immediately to the left of a candidate boundary.
    #[cfg_attr(feature = "serde", serde(borrow))]
    pub uw3: ZeroMap<'data, char, i16>,
    /// Weights for the scalar immediately to the right of a candidate boundary.
    #[cfg_attr(feature = "serde", serde(borrow))]
    pub uw4: ZeroMap<'data, char, i16>,
    /// Weights for the scalar two positions to the right of a candidate boundary.
    #[cfg_attr(feature = "serde", serde(borrow))]
    pub uw5: ZeroMap<'data, char, i16>,
    /// Weights for the scalar pair surrounding a candidate boundary.
    #[cfg_attr(feature = "serde", serde(borrow))]
    pub bw2: ZeroMap<'data, (char, char), i16>,
    /// Weights for the radical pair surrounding a candidate boundary.
    #[cfg_attr(feature = "serde", serde(borrow))]
    pub rad: ZeroMap<'data, (u8, u8), i16>,
    /// Weights for the left radical and right scalar pair.
    #[cfg_attr(feature = "serde", serde(borrow))]
    pub lsrid: ZeroMap<'data, (u8, char), i16>,
    /// Weights for the left scalar and right radical pair.
    #[cfg_attr(feature = "serde", serde(borrow))]
    pub rsrid: ZeroMap<'data, (char, u8), i16>,
}

/// The data powering the Thai `AdaBoost` segmentation model.
///
/// <div class="stab unstable">
/// 🚧 This code is considered unstable; it may change at any time, in breaking or non-breaking ways,
/// including in SemVer minor releases. While the serde representation of data structs is guaranteed
/// to be stable, their Rust representation might not be. Use with caution.
/// </div>
#[derive(Debug, PartialEq, Clone, yoke::Yokeable, zerofrom::ZeroFrom)]
#[cfg_attr(feature = "datagen", derive(serde::Serialize, databake::Bake))]
#[cfg_attr(
    feature = "datagen",
    databake(path = icu_segmenter::provider::adaboost)
)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
#[yoke(prove_covariance_manually)]
pub struct ThaiAdaboostData<'data> {
    /// The doubled model bias.
    pub bias: i32,
    /// Unigram weights three positions to the left of a candidate boundary.
    #[cfg_attr(feature = "serde", serde(borrow))]
    pub uw1: ZeroMap<'data, char, i16>,
    /// Unigram weights two positions to the left of a candidate boundary.
    #[cfg_attr(feature = "serde", serde(borrow))]
    pub uw2: ZeroMap<'data, char, i16>,
    /// Unigram weights immediately to the left of a candidate boundary.
    #[cfg_attr(feature = "serde", serde(borrow))]
    pub uw3: ZeroMap<'data, char, i16>,
    /// Unigram weights immediately to the right of a candidate boundary.
    #[cfg_attr(feature = "serde", serde(borrow))]
    pub uw4: ZeroMap<'data, char, i16>,
    /// Unigram weights two positions to the right of a candidate boundary.
    #[cfg_attr(feature = "serde", serde(borrow))]
    pub uw5: ZeroMap<'data, char, i16>,
    /// Unigram weights three positions to the right of a candidate boundary.
    #[cfg_attr(feature = "serde", serde(borrow))]
    pub uw6: ZeroMap<'data, char, i16>,
    /// Bigram weights immediately to the left of a candidate boundary.
    #[cfg_attr(feature = "serde", serde(borrow))]
    pub bw1: ZeroMap<'data, (char, char), i16>,
    /// Bigram weights surrounding a candidate boundary.
    #[cfg_attr(feature = "serde", serde(borrow))]
    pub bw2: ZeroMap<'data, (char, char), i16>,
    /// Bigram weights immediately to the right of a candidate boundary.
    #[cfg_attr(feature = "serde", serde(borrow))]
    pub bw3: ZeroMap<'data, (char, char), i16>,
    /// Trigram weights to the left of a candidate boundary.
    ///
    /// These remain string-keyed to preserve the source model exactly.
    #[cfg_attr(feature = "serde", serde(borrow))]
    pub tw1: ZeroMap<'data, str, i16>,
    /// Trigram weights centered immediately to the left of a candidate boundary.
    #[cfg_attr(feature = "serde", serde(borrow))]
    pub tw2: ZeroMap<'data, str, i16>,
    /// Trigram weights centered immediately to the right of a candidate boundary.
    #[cfg_attr(feature = "serde", serde(borrow))]
    pub tw3: ZeroMap<'data, str, i16>,
    /// Trigram weights to the right of a candidate boundary.
    #[cfg_attr(feature = "serde", serde(borrow))]
    pub tw4: ZeroMap<'data, str, i16>,
}

icu_provider::data_struct!(
    AdaboostData<'_>,
    #[cfg(feature = "datagen")]
);

icu_provider::data_marker!(
    /// Chinese `AdaBoost` segmentation model data.
    SegmenterChineseAutoV1,
    "segmenter/chinese/auto/v1",
    AdaboostData<'static>,
    #[cfg(feature = "datagen")]
    attributes_domain = "segmenter"
);

icu_provider::data_struct!(
    ThaiAdaboostData<'_>,
    #[cfg(feature = "datagen")]
);

icu_provider::data_marker!(
    /// Thai `AdaBoost` segmentation model data.
    SegmenterThaiAutoV1,
    "segmenter/thai/auto/v1",
    ThaiAdaboostData<'static>,
    #[cfg(feature = "datagen")]
    attributes_domain = "segmenter"
);

#[cfg(all(test, feature = "compiled_data"))]
mod tests {
    use super::*;
    use crate::provider::Baked;

    const CHINESE_ADABOOST: &DataMarkerAttributes =
        DataMarkerAttributes::from_str_or_panic("Chinese_adaboost");
    const THAI_ADABOOST: &DataMarkerAttributes =
        DataMarkerAttributes::from_str_or_panic("Thai_adaboost");
    const UNKNOWN: &DataMarkerAttributes = DataMarkerAttributes::from_str_or_panic("unknown");

    #[test]
    fn baked_chinese_model() {
        let response: DataResponse<SegmenterChineseAutoV1> = Baked
            .load(DataRequest {
                id: DataIdentifierBorrowed::for_marker_attributes(CHINESE_ADABOOST),
                ..Default::default()
            })
            .expect("the baked Chinese AdaBoost model should load");
        let data = response.payload.get();

        assert_eq!(data.bias, 4);
        assert_eq!(
            data.uw2.len()
                + data.uw3.len()
                + data.uw4.len()
                + data.uw5.len()
                + data.bw2.len()
                + data.rad.len()
                + data.lsrid.len()
                + data.rsrid.len(),
            5291
        );
    }

    #[test]
    fn baked_thai_model() {
        let response: DataResponse<SegmenterThaiAutoV1> = Baked
            .load(DataRequest {
                id: DataIdentifierBorrowed::for_marker_attributes(THAI_ADABOOST),
                ..Default::default()
            })
            .expect("the baked Thai AdaBoost model should load");
        let data = response.payload.get();

        assert_eq!(data.bias, -3755);
        assert_eq!(
            data.uw1.len()
                + data.uw2.len()
                + data.uw3.len()
                + data.uw4.len()
                + data.uw5.len()
                + data.uw6.len()
                + data.bw1.len()
                + data.bw2.len()
                + data.bw3.len()
                + data.tw1.len()
                + data.tw2.len()
                + data.tw3.len()
                + data.tw4.len(),
            3901
        );
        assert_eq!(data.tw4.get_copied("ละ"), Some(870));
    }

    #[test]
    fn baked_chinese_model_rejects_unknown_attributes() {
        let error = <Baked as DataProvider<SegmenterChineseAutoV1>>::load(
            &Baked,
            DataRequest {
                id: DataIdentifierBorrowed::for_marker_attributes(UNKNOWN),
                ..Default::default()
            },
        )
        .expect_err("an unknown model attribute should not load");

        assert_eq!(error.kind, DataErrorKind::IdentifierNotFound);
    }

    #[test]
    fn baked_thai_model_rejects_unknown_attributes() {
        let error = <Baked as DataProvider<SegmenterThaiAutoV1>>::load(
            &Baked,
            DataRequest {
                id: DataIdentifierBorrowed::for_marker_attributes(UNKNOWN),
                ..Default::default()
            },
        )
        .expect_err("an unknown model attribute should not load");

        assert_eq!(error.kind, DataErrorKind::IdentifierNotFound);
    }
}
