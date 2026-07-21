// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

//! Data provider struct definitions for the AdaBoost segmenter.

use icu_provider::prelude::*;
use zerovec::ZeroMap;

/// The data powering the Chinese AdaBoost segmentation model.
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
    /// The model bias, already expressed in doubled-score units.
    pub bias_x2: i32,
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

icu_provider::data_struct!(
    AdaboostData<'_>,
    #[cfg(feature = "datagen")]
);

icu_provider::data_marker!(
    /// Chinese AdaBoost segmentation model data.
    SegmenterAdaboostAutoV1,
    "segmenter/adaboost/auto/v1",
    AdaboostData<'static>,
    #[cfg(feature = "datagen")]
    attributes_domain = "segmenter"
);
